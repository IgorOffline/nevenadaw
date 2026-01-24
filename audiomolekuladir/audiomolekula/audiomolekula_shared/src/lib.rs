use bevy::prelude::Resource;
use clap_sys::ext::gui::{clap_plugin_gui, clap_window, clap_window_handle, CLAP_WINDOW_API_WIN32};
use clap_sys::host::clap_host;
use clap_sys::plugin::clap_plugin;
use libloading::Library;
use std::ffi::c_void;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct AudioState {
    _stream: cpal::Stream,
    _library: Library,
    _host: Box<clap_host>,
    _host_gui_state: Box<HostGuiState>,
    plugin: *const clap_plugin,
    gui: Option<*const clap_plugin_gui>,
    is_pressed: Arc<AtomicBool>,
    gui_state: Mutex<PluginGuiState>,
}

pub struct PluginGuiRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub scale: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuiRequests {
    pub requested_resize: Option<(u32, u32)>,
    pub requested_show: bool,
    pub requested_hide: bool,
    pub closed: bool,
    pub closed_was_destroyed: bool,
    pub resize_hints_changed: bool,
}

pub struct HostGuiState {
    requests: Mutex<GuiRequests>,
}

impl HostGuiState {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(GuiRequests::default()),
        }
    }

    pub fn request_resize(&self, width: u32, height: u32) -> bool {
        let mut requests = self.requests.lock().expect("c09a3710 gui requests lock");
        requests.requested_resize = Some((width, height));
        true
    }

    pub fn request_show(&self) -> bool {
        let mut requests = self.requests.lock().expect("a23a211b gui requests lock");
        requests.requested_show = true;
        true
    }

    pub fn request_hide(&self) -> bool {
        let mut requests = self.requests.lock().expect("6f64bfb2 gui requests lock");
        requests.requested_hide = true;
        true
    }

    pub fn resize_hints_changed(&self) {
        let mut requests = self.requests.lock().expect("5c22798f gui requests lock");
        requests.resize_hints_changed = true;
    }

    pub fn closed(&self, was_destroyed: bool) {
        let mut requests = self.requests.lock().expect("420fd7c2 gui requests lock");
        requests.closed = true;
        requests.closed_was_destroyed = was_destroyed;
    }

    pub fn take_requests(&self) -> GuiRequests {
        let mut requests = self.requests.lock().expect("6d2b87ee gui requests lock");
        mem::take(&mut *requests)
    }
}

#[derive(Default)]
struct PluginGuiState {
    created: bool,
    parent_hwnd: Option<isize>,
    parent_set: bool,
    last_size: Option<(u32, u32)>,
    last_scale: Option<f64>,
    last_visible: Option<bool>,
    checked_api: bool,
    api_supported: bool,
}

impl AudioState {
    pub fn new(
        library: Library,
        stream: cpal::Stream,
        is_pressed: Arc<AtomicBool>,
        plugin: *const clap_plugin,
        gui: Option<*const clap_plugin_gui>,
        host: Box<clap_host>,
        host_gui_state: Box<HostGuiState>,
    ) -> Self {
        Self {
            _stream: stream,
            _library: library,
            _host: host,
            _host_gui_state: host_gui_state,
            plugin,
            gui,
            is_pressed,
            gui_state: Mutex::new(PluginGuiState::default()),
        }
    }

    pub fn set_pressed(&self, pressed: bool) {
        self.is_pressed.store(pressed, Ordering::Relaxed);
    }

    pub fn set_parent_window(&self, hwnd: isize) {
        let Some(gui_ptr) = self.gui else {
            println!("CLAP GUI: extension missing, cannot set parent");
            return;
        };

        let mut gui_state = self
            .gui_state
            .lock()
            .expect("125edb30 plugin gui state lock");
        if gui_state.parent_hwnd == Some(hwnd) {
            return;
        }
        gui_state.parent_hwnd = Some(hwnd);

        let gui = unsafe { &*gui_ptr };
        if !self.ensure_gui_created(gui, &mut gui_state) {
            return;
        }

        if gui_state.parent_set {
            return;
        }

        let window = clap_window {
            api: CLAP_WINDOW_API_WIN32.as_ptr(),
            specific: clap_window_handle {
                win32: hwnd as *mut c_void,
            },
        };

        match gui.set_parent {
            Some(set_parent) => {
                let ok = unsafe { (set_parent)(self.plugin, &window) };
                println!("CLAP GUI set_parent win32 -> {}", ok);
                if ok {
                    gui_state.parent_set = true;
                }
            }
            None => println!("CLAP GUI set_parent missing"),
        }
    }

    pub fn update_gui_layout(&self, rect: PluginGuiRect) {
        let Some(gui_ptr) = self.gui else {
            return;
        };

        let gui = unsafe { &*gui_ptr };
        let mut gui_state = self
            .gui_state
            .lock()
            .expect("72c1fafa plugin gui state lock");

        if !self.ensure_gui_created(gui, &mut gui_state) {
            return;
        }

        if !gui_state.parent_set {
            if let Some(hwnd) = gui_state.parent_hwnd {
                let window = clap_window {
                    api: CLAP_WINDOW_API_WIN32.as_ptr(),
                    specific: clap_window_handle {
                        win32: hwnd as *mut c_void,
                    },
                };
                match gui.set_parent {
                    Some(set_parent) => {
                        let ok = unsafe { (set_parent)(self.plugin, &window) };
                        println!("CLAP GUI set_parent win32 -> {}", ok);
                        if ok {
                            gui_state.parent_set = true;
                        }
                    }
                    None => println!("CLAP GUI set_parent missing"),
                }
            } else {
                return;
            }
        }

        if let Some(set_scale) = gui.set_scale {
            let needs_scale = gui_state
                .last_scale
                .map(|last| (last - rect.scale).abs() > f64::EPSILON)
                .unwrap_or(true);
            if needs_scale {
                let ok = unsafe { (set_scale)(self.plugin, rect.scale) };
                println!("CLAP GUI set_scale {} -> {}", rect.scale, ok);
                if ok {
                    gui_state.last_scale = Some(rect.scale);
                }
            }
        }

        if rect.width > 0 && rect.height > 0 {
            let size = (rect.width, rect.height);
            if gui_state.last_size != Some(size) {
                match gui.set_size {
                    Some(set_size) => {
                        let ok = unsafe { (set_size)(self.plugin, rect.width, rect.height) };
                        println!("CLAP GUI set_size {}x{} -> {}", rect.width, rect.height, ok);
                        if ok {
                            gui_state.last_size = Some(size);
                        }
                    }
                    None => println!("CLAP GUI set_size missing"),
                }
            }
        }

        if gui_state.last_visible != Some(rect.visible) {
            if rect.visible {
                match gui.show {
                    Some(show) => {
                        let ok = unsafe { (show)(self.plugin) };
                        println!("CLAP GUI show -> {}", ok);
                        if ok {
                            gui_state.last_visible = Some(true);
                        }
                    }
                    None => {
                        println!("CLAP GUI show missing");
                        gui_state.last_visible = Some(true);
                    }
                }
            } else {
                match gui.hide {
                    Some(hide) => {
                        let ok = unsafe { (hide)(self.plugin) };
                        println!("CLAP GUI hide -> {}", ok);
                        if ok {
                            gui_state.last_visible = Some(false);
                        }
                    }
                    None => {
                        println!("CLAP GUI hide missing");
                        gui_state.last_visible = Some(false);
                    }
                }
            }
        }
    }

    pub fn take_gui_requests(&self) -> GuiRequests {
        self._host_gui_state.take_requests()
    }

    fn ensure_gui_created(&self, gui: &clap_plugin_gui, state: &mut PluginGuiState) -> bool {
        if !state.checked_api {
            state.checked_api = true;
            state.api_supported = match gui.is_api_supported {
                Some(is_api_supported) => unsafe {
                    (is_api_supported)(self.plugin, CLAP_WINDOW_API_WIN32.as_ptr(), false)
                },
                None => {
                    println!("CLAP GUI is_api_supported missing");
                    false
                }
            };
            println!(
                "CLAP GUI is_api_supported(win32): {}",
                state.api_supported
            );
        }

        if !state.api_supported {
            return false;
        }

        if !state.created {
            state.created = match gui.create {
                Some(create) => unsafe { (create)(self.plugin, CLAP_WINDOW_API_WIN32.as_ptr(), false) },
                None => {
                    println!("CLAP GUI create missing");
                    false
                }
            };
            println!("CLAP GUI create(win32): {}", state.created);
        }

        state.created
    }
}
