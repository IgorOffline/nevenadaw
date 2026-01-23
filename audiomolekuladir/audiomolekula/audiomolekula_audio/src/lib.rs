use clap_sys::audio_buffer::clap_audio_buffer;
use clap_sys::entry::clap_plugin_entry;
use clap_sys::events::{
    clap_event_header, clap_event_note, clap_input_events, clap_output_events,
    CLAP_CORE_EVENT_SPACE_ID, CLAP_EVENT_NOTE_ON,
};
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::plugin::clap_plugin;
use clap_sys::process::clap_process;
use clap_sys::version::CLAP_VERSION;
use cpal::traits::{DeviceTrait, HostTrait};
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};

#[allow(dead_code)]
struct MockNoteEvent {
    note: clap_event_note,
}

unsafe extern "C" fn event_list_size(list: *const clap_input_events) -> u32 {
    unsafe { if (*list).ctx.is_null() { 0 } else { 1 } }
}

unsafe extern "C" fn event_list_get(
    list: *const clap_input_events,
    _index: u32,
) -> *const clap_event_header {
    unsafe { (*list).ctx as *const clap_event_header }
}

unsafe extern "C" fn output_events_push(
    _list: *const clap_output_events,
    _event: *const clap_event_header,
) -> bool {
    true
}

unsafe extern "C" fn host_get_extension(
    _host: *const clap_host,
    _extension_id: *const i8,
) -> *const c_void {
    std::ptr::null()
}

unsafe extern "C" fn host_request_restart(_host: *const clap_host) {}
unsafe extern "C" fn host_request_process(_host: *const clap_host) {}
unsafe extern "C" fn host_request_callback(_host: *const clap_host) {}

static AUDIOMOLEKULA_HOST: clap_host = clap_host {
    clap_version: CLAP_VERSION,
    host_data: std::ptr::null_mut(),
    name: b"Audiomolekula\0".as_ptr() as *const i8,
    vendor: b"Independent\0".as_ptr() as *const i8,
    url: b"https://igordurbek.com\0".as_ptr() as *const i8,
    version: b"0.1.0\0".as_ptr() as *const i8,
    get_extension: Some(host_get_extension),
    request_restart: Some(host_request_restart),
    request_process: Some(host_request_process),
    request_callback: Some(host_request_callback),
};

struct PluginCleanup<'a> {
    entry: &'a clap_plugin_entry,
    entry_inited: bool,
    plugin: *const clap_plugin,
    activated: bool,
}

impl<'a> PluginCleanup<'a> {
    fn new(entry: &'a clap_plugin_entry) -> Self {
        Self {
            entry,
            entry_inited: false,
            plugin: std::ptr::null(),
            activated: false,
        }
    }
}

impl Drop for PluginCleanup<'_> {
    fn drop(&mut self) {
        unsafe {
            if !self.plugin.is_null() {
                let plugin = &*self.plugin;
                if self.activated {
                    if let Some(deactivate) = plugin.deactivate {
                        deactivate(plugin);
                    }
                }
                if let Some(destroy) = plugin.destroy {
                    destroy(plugin);
                }
            }

            if self.entry_inited {
                if let Some(deinit) = self.entry.deinit {
                    deinit();
                }
            }
        }
    }
}

unsafe fn load_entry(lib: &Library) -> Result<&clap_plugin_entry, String> {
    let entry_symbol: Symbol<*const clap_plugin_entry> = unsafe {
        lib.get(b"clap_entry\0")
            .map_err(|err| format!("Failed to get clap_entry: {}", err))?
    };
    let entry_ptr = *entry_symbol;
    if entry_ptr.is_null() {
        return Err("clap_entry symbol is null".to_string());
    }

    Ok(unsafe { &*entry_ptr })
}

unsafe fn get_factory(entry: &clap_plugin_entry) -> Result<&clap_plugin_factory, String> {
    let get_factory = entry
        .get_factory
        .ok_or_else(|| "get_factory missing".to_string())?;
    let factory_ptr = unsafe { get_factory(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8) };
    if factory_ptr.is_null() {
        return Err("Plugin factory is null".to_string());
    }

    Ok(unsafe { &*(factory_ptr as *const clap_plugin_factory) })
}

pub fn setup_audio_system() {
    let plugin_path = r"C:\Program Files\Common Files\CLAP\Vital.clap";
    let plugin_name = "Vital";

    if !std::path::Path::new(plugin_path).exists() {
        println!("{} not found at {}", plugin_name, plugin_path);
        return;
    }

    let lib = match unsafe { Library::new(plugin_path) } {
        Ok(lib) => lib,
        Err(err) => {
            println!("Failed to load {}: {}", plugin_name, err);
            return;
        }
    };

    let entry = match unsafe { load_entry(&lib) } {
        Ok(entry) => entry,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let plugin_path_cstring = match CString::new(plugin_path) {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid plugin path: {}", err);
            return;
        }
    };

    let entry_init = match entry.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return;
        }
    };

    if unsafe { !(entry_init)(plugin_path_cstring.as_ptr()) } {
        println!("Plugin entry init failed for {}", plugin_name);
        return;
    }
    let mut cleanup = PluginCleanup::new(entry);
    cleanup.entry_inited = true;

    let factory = match unsafe { get_factory(entry) } {
        Ok(factory) => factory,
        Err(msg) => {
            println!("{} for {}", msg, plugin_name);
            return;
        }
    };

    let get_plugin_count = match factory.get_plugin_count {
        Some(get_plugin_count) => get_plugin_count,
        None => {
            println!("get_plugin_count missing for {}", plugin_name);
            return;
        }
    };

    let count = unsafe { get_plugin_count(factory) };
    if count == 0 {
        println!("No plugins found in {}", plugin_name);
        return;
    }

    let get_plugin_descriptor = match factory.get_plugin_descriptor {
        Some(get_plugin_descriptor) => get_plugin_descriptor,
        None => {
            println!("get_plugin_descriptor missing for {}", plugin_name);
            return;
        }
    };
    let descriptor = unsafe { get_plugin_descriptor(factory, 0) };
    if descriptor.is_null() {
        println!("Plugin descriptor is null for {}", plugin_name);
        return;
    }

    let plugin_id = unsafe { (*descriptor).id };
    if plugin_id.is_null() {
        println!("Plugin descriptor id is null for {}", plugin_name);
        return;
    }

    let create_plugin = match factory.create_plugin {
        Some(create_plugin) => create_plugin,
        None => {
            println!("create_plugin missing for {}", plugin_name);
            return;
        }
    };
    let plugin_ptr = unsafe { create_plugin(factory, &AUDIOMOLEKULA_HOST, plugin_id) };

    if plugin_ptr.is_null() {
        println!("{} failed to initialize", plugin_name);
        return;
    }

    let plugin = unsafe { &*plugin_ptr };
    cleanup.plugin = plugin_ptr;
    let plugin_init = match plugin.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return;
        }
    };
    if unsafe { !(plugin_init)(plugin) } {
        println!("Error: Plugin init failed");
        return;
    }

    let mut activated = false;
    if let Some(activate) = plugin.activate {
        if unsafe { !(activate)(plugin, 44100.0, 1, 4096) } {
            println!("Error: Plugin activation failed");
            return;
        }
        activated = true;
    }
    cleanup.activated = activated;

    let plugin_ptr_usize = plugin_ptr as usize;

    let asio_host = cpal::host_from_id(cpal::HostId::Asio).unwrap_or_else(|_| cpal::default_host());
    let device = asio_host
        .output_devices()
        .unwrap()
        .find(|d| {
            d.description()
                .map(|desc| desc.name().contains("FlexASIO"))
                .unwrap_or(false)
        })
        .or_else(|| asio_host.default_output_device())
        .expect("No output device found.");

    let config = device
        .default_output_config()
        .expect("Failed to get default output config");

    let note_frames = config.sample_rate().0.saturating_mul(2);
    let mut left_out = vec![0.0f32; 4096];
    let mut right_out = vec![0.0f32; 4096];
    let mut frames_remaining = 0u32;
    let mut note_on_pending = true;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let plugin_ptr = plugin_ptr_usize as *const clap_plugin;
                let plugin = unsafe { &*plugin_ptr };

                let total_samples = data.len();
                if total_samples % 2 != 0 {
                    return;
                }
                let num_frames = (total_samples / 2) as u32;

                if num_frames > 4096 {
                    data.fill(0.0);
                    return;
                }

                left_out.fill(0.0);
                right_out.fill(0.0);

                if note_on_pending {
                    note_on_pending = false;
                    frames_remaining = note_frames;
                }

                if frames_remaining == 0 {
                    data.fill(0.0);
                    return;
                }

                let mut my_note = if frames_remaining == note_frames {
                    Some(MockNoteEvent {
                        note: clap_event_note {
                            header: clap_event_header {
                                size: size_of::<clap_event_note>() as u32,
                                time: 0,
                                space_id: CLAP_CORE_EVENT_SPACE_ID,
                                type_: CLAP_EVENT_NOTE_ON,
                                flags: 0,
                            },
                            note_id: -1,
                            port_index: 0,
                            channel: 0,
                            key: 60,
                            velocity: 1.0,
                        },
                    })
                } else {
                    None
                };

                let input_events = clap_input_events {
                    ctx: if let Some(ref mut note) = my_note {
                        note as *mut _ as *mut c_void
                    } else {
                        std::ptr::null_mut()
                    },
                    size: Some(event_list_size),
                    get: Some(event_list_get),
                };

                let mut out_events = clap_output_events {
                    ctx: std::ptr::null_mut(),
                    try_push: Some(output_events_push),
                };

                let mut channel_ptrs = [left_out.as_mut_ptr(), right_out.as_mut_ptr()];
                let mut output_buffer = clap_audio_buffer {
                    data32: channel_ptrs.as_mut_ptr(),
                    data64: std::ptr::null_mut(),
                    channel_count: 2,
                    latency: 0,
                    constant_mask: 0,
                };

                let process_data = clap_process {
                    steady_time: -1,
                    frames_count: num_frames,
                    transport: std::ptr::null(),
                    audio_inputs: std::ptr::null(),
                    audio_outputs: &mut output_buffer,
                    audio_inputs_count: 0,
                    audio_outputs_count: 1,
                    in_events: &input_events,
                    out_events: &mut out_events,
                };

                unsafe {
                    if let Some(process_fn) = plugin.process {
                        (process_fn)(plugin, &process_data);
                    }
                }

                let frames_to_copy = frames_remaining.min(num_frames) as usize;
                for i in 0..frames_to_copy {
                    data[i * 2] = left_out[i] * 0.2;
                    data[i * 2 + 1] = right_out[i] * 0.2;
                }
                for i in frames_to_copy..num_frames as usize {
                    data[i * 2] = 0.0;
                    data[i * 2 + 1] = 0.0;
                }
                frames_remaining = frames_remaining.saturating_sub(num_frames);
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .expect("Failed to build output stream");

    unsafe {
        if let Some(start_proc) = plugin.start_processing {
            (start_proc)(plugin);
        }
    }

    //stream.play().expect("Failed to play stream");

    println!("(Audio system setup completed)");
}
