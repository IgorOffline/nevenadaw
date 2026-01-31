use crate::windows_utils;
use anyhow::Result;
use clap_sys::entry::clap_plugin_entry;
use clap_sys::ext::gui::{clap_plugin_gui, clap_window, CLAP_EXT_GUI, CLAP_WINDOW_API_WIN32};
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::plugin::clap_plugin;
use clap_sys::version::CLAP_VERSION;
use std::ffi::{c_void, CStr, CString};
use std::ptr;
use windows::Win32::Foundation::{HMODULE, HWND};

pub struct ClapPluginHost {
    dll_handle: Option<HMODULE>,
    entry: Option<*const clap_plugin_entry>,
    plugin: Option<*const clap_plugin>,
    gui_hwnd: Option<HWND>,
}

impl ClapPluginHost {
    pub fn new() -> Self {
        Self {
            dll_handle: None,
            entry: None,
            plugin: None,
            gui_hwnd: None,
        }
    }

    pub fn load_and_create_gui(
        &mut self,
        plugin_path: &str,
        parent_hwnd: Option<HWND>,
    ) -> Result<HWND> {
        log::info!("Loading plugin DLL: {}", plugin_path);
        let dll_handle = windows_utils::load_library(plugin_path)?;
        self.dll_handle = Some(dll_handle);

        let entry: *const clap_plugin_entry = {
            let ptr: *const clap_plugin_entry =
                windows_utils::get_function(dll_handle, "clap_entry")?;
            if ptr.is_null() {
                anyhow::bail!("clap_entry is null");
            }
            ptr
        };
        self.entry = Some(entry);

        let path_c = CString::new(plugin_path)?;
        if unsafe { !((*entry).init.expect("init function missing"))(path_c.as_ptr()) } {
            anyhow::bail!("Plugin initialization failed");
        }

        let factory = unsafe {
            ((*entry).get_factory.expect("get_factory function missing"))(
                CLAP_PLUGIN_FACTORY_ID.as_ptr(),
            )
        } as *const clap_plugin_factory;

        if factory.is_null() {
            anyhow::bail!("No plugin factory available");
        }

        let plugin_count = unsafe {
            ((*factory)
                .get_plugin_count
                .expect("get_plugin_count missing"))(factory)
        };
        if plugin_count == 0 {
            anyhow::bail!("No plugins in factory");
        }

        let descriptor = unsafe {
            ((*factory)
                .get_plugin_descriptor
                .expect("get_plugin_descriptor missing"))(factory, 0)
        };
        if descriptor.is_null() {
            anyhow::bail!("Failed to get plugin descriptor");
        }

        let plugin_id = unsafe { (*descriptor).id };
        log::info!("Creating plugin instance for: {:?}", unsafe {
            CStr::from_ptr(plugin_id)
        });

        let plugin = unsafe {
            ((*factory).create_plugin.expect("create_plugin missing"))(factory, &HOST, plugin_id)
        };
        if plugin.is_null() {
            anyhow::bail!("Failed to create plugin instance");
        }
        self.plugin = Some(plugin);

        if unsafe { !((*plugin).init.expect("plugin init missing"))(plugin) } {
            anyhow::bail!("Failed to initialize plugin instance");
        }

        let gui_ext = unsafe {
            ((*plugin).get_extension.expect("get_extension missing"))(plugin, CLAP_EXT_GUI.as_ptr())
        } as *const clap_plugin_gui;

        if !gui_ext.is_null() {
            let is_api_supported = unsafe {
                ((*gui_ext)
                    .is_api_supported
                    .expect("is_api_supported missing"))(
                    plugin,
                    CLAP_WINDOW_API_WIN32.as_ptr(),
                    false,
                )
            };
            if is_api_supported {
                log::info!("Plugin supports Win32 GUI API");
                unsafe {
                    if ((*gui_ext).create.expect("gui create missing"))(
                        plugin,
                        CLAP_WINDOW_API_WIN32.as_ptr(),
                        false,
                    ) {
                        if let Some(parent) = parent_hwnd {
                            let mut window: clap_window = std::mem::zeroed();
                            window.api = CLAP_WINDOW_API_WIN32.as_ptr();
                            window.specific.win32 = parent.0 as *mut c_void;
                            ((*gui_ext).set_parent.expect("set_parent missing"))(plugin, &window);
                        }
                        ((*gui_ext).show.expect("gui show missing"))(plugin);
                    } else {
                        log::warn!("Failed to create plugin GUI");
                    }
                }
            } else {
                log::warn!("Plugin does not support Win32 GUI API");
            }
        } else {
            log::warn!("Plugin does not support GUI extension");
        }

        Ok(self.gui_hwnd.unwrap_or(HWND(ptr::null_mut())))
    }

    #[allow(dead_code)]
    pub fn get_gui_hwnd(&self) -> Option<HWND> {
        self.gui_hwnd
    }
}

impl Drop for ClapPluginHost {
    fn drop(&mut self) {
        if let Some(plugin) = self.plugin {
            unsafe {
                if let Some(destroy) = (*plugin).destroy {
                    destroy(plugin);
                }
            }
        }
        if let Some(entry) = self.entry {
            unsafe {
                if let Some(deinit) = (*entry).deinit {
                    deinit();
                }
            }
        }
        if let Some(dll_handle) = self.dll_handle {
            unsafe {
                let _ = windows::Win32::Foundation::FreeLibrary(dll_handle);
            }
        }
    }
}

static HOST: clap_host = clap_host {
    clap_version: CLAP_VERSION,
    host_data: ptr::null_mut(),
    name: b"JanuarySound\0".as_ptr() as *const i8,
    vendor: b"Igor\0".as_ptr() as *const i8,
    url: b"https://igordurbek.com\0".as_ptr() as *const i8,
    version: b"0.1.0\0".as_ptr() as *const i8,
    get_extension: Some(host_get_extension),
    request_restart: Some(host_request_restart),
    request_process: Some(host_request_process),
    request_callback: Some(host_request_callback),
};

unsafe extern "C" fn host_get_extension(
    _host: *const clap_host,
    _extension_id: *const i8,
) -> *const c_void {
    ptr::null()
}

unsafe extern "C" fn host_request_restart(_host: *const clap_host) {}
unsafe extern "C" fn host_request_process(_host: *const clap_host) {}
unsafe extern "C" fn host_request_callback(_host: *const clap_host) {}
