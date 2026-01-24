use audiomolekula_shared::AudioState;
use clap_sys::audio_buffer::clap_audio_buffer;
use clap_sys::entry::clap_plugin_entry;
use clap_sys::events::{
    clap_event_header, clap_event_note, clap_input_events, clap_output_events,
    CLAP_CORE_EVENT_SPACE_ID, CLAP_EVENT_NOTE_OFF, CLAP_EVENT_NOTE_ON,
};
use clap_sys::ext::gui::{clap_plugin_gui, CLAP_EXT_GUI};
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::plugin::clap_plugin;
use clap_sys::process::clap_process;
use clap_sys::version::CLAP_VERSION;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, SizedSample, SupportedBufferSize};
use libloading::{Library, Symbol};
use std::ffi::{c_void, CString};
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

static HELLO_HOST: clap_host = clap_host {
    clap_version: CLAP_VERSION,
    host_data: std::ptr::null_mut(),
    name: b"HelloCPAL\0".as_ptr() as *const i8,
    vendor: b"Independent\0".as_ptr() as *const i8,
    url: b"https://igordurbek.com\0".as_ptr() as *const i8,
    version: b"0.1.0\0".as_ptr() as *const i8,
    get_extension: Some(host_get_extension),
    request_restart: Some(host_request_restart),
    request_process: Some(host_request_process),
    request_callback: Some(host_request_callback),
};

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

fn query_gui_extension(plugin: &clap_plugin, plugin_name: &str) -> Option<*const clap_plugin_gui> {
    let get_extension = match plugin.get_extension {
        Some(get_extension) => get_extension,
        None => {
            println!("get_extension missing for {}", plugin_name);
            return None;
        }
    };

    let gui_ext_ptr = unsafe { get_extension(plugin, CLAP_EXT_GUI.as_ptr()) };
    if gui_ext_ptr.is_null() {
        println!("{} does not support CLAP_EXT_GUI", plugin_name);
        return None;
    }

    Some(gui_ext_ptr as *const clap_plugin_gui)
}

fn select_output_device(host: &cpal::Host, preferred_name: &str) -> Option<cpal::Device> {
    host.output_devices()
        .ok()
        .and_then(|mut devices| {
            devices.find(|device| {
                device
                    .description()
                    .map(|desc| desc.name().contains(preferred_name))
                    .unwrap_or(false)
            })
        })
        .or_else(|| host.default_output_device())
}

fn buffer_bounds(config: &cpal::SupportedStreamConfig) -> (u32, u32) {
    match config.buffer_size() {
        SupportedBufferSize::Range { min, max } => (*min, *max),
        SupportedBufferSize::Unknown => (1, 4096),
    }
}

fn build_stream_for_format<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    plugin_ptr_usize: usize,
    is_pressed: Arc<AtomicBool>,
    max_frames: usize,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: SizedSample + FromSample<f32> + Send + 'static,
{
    let channels = config.channels as usize;
    let mut left_out = vec![0.0f32; max_frames];
    let mut right_out = vec![0.0f32; max_frames];
    let mut last_pressed = false;

    device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let plugin_ptr = plugin_ptr_usize as *const clap_plugin;
            let plugin = unsafe { &*plugin_ptr };

            if channels == 0 {
                data.fill(T::EQUILIBRIUM);
                return;
            }

            let total_samples = data.len();
            if total_samples % channels != 0 {
                data.fill(T::EQUILIBRIUM);
                return;
            }

            let num_frames = total_samples / channels;
            if num_frames > max_frames {
                data.fill(T::EQUILIBRIUM);
                return;
            }

            left_out.fill(0.0);
            right_out.fill(0.0);

            let current_pressed = is_pressed.load(Ordering::Relaxed);

            let mut my_note = if current_pressed != last_pressed {
                let event_type = if current_pressed {
                    CLAP_EVENT_NOTE_ON
                } else {
                    CLAP_EVENT_NOTE_OFF
                };
                last_pressed = current_pressed;
                Some(MockNoteEvent {
                    note: clap_event_note {
                        header: clap_event_header {
                            size: mem::size_of::<clap_event_note>() as u32,
                            time: 0,
                            space_id: CLAP_CORE_EVENT_SPACE_ID,
                            type_: event_type,
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
                frames_count: num_frames as u32,
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

            for i in 0..num_frames {
                let left = left_out[i] * 0.2;
                let right = right_out[i] * 0.2;
                if channels == 1 {
                    let mixed = (left + right) * 0.5;
                    data[i] = T::from_sample(mixed);
                } else {
                    let base = i * channels;
                    data[base] = T::from_sample(left);
                    data[base + 1] = T::from_sample(right);
                    for channel in 2..channels {
                        data[base + channel] = T::EQUILIBRIUM;
                    }
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )
}

pub fn setup_audio_system() -> Option<AudioState> {
    let coin_toss = rand::random_range(1..=2);
    let (plugin_path, plugin_name) = if coin_toss == 1 {
        (r"C:\Program Files\Common Files\CLAP\Vital.clap", "Vital")
    } else {
        (
            r"C:\Program Files\Common Files\CLAP\Surge Synth Team\Surge XT.clap",
            "Surge XT",
        )
    };
    println!("Coin toss result: {}", plugin_name);

    if !std::path::Path::new(plugin_path).exists() {
        println!("{} not found at {}", plugin_name, plugin_path);
        return None;
    }

    let lib = match unsafe { Library::new(plugin_path) } {
        Ok(lib) => lib,
        Err(err) => {
            println!("Failed to load {}: {}", plugin_name, err);
            return None;
        }
    };

    let entry = match unsafe { load_entry(&lib) } {
        Ok(entry) => entry,
        Err(msg) => {
            println!("{}", msg);
            return None;
        }
    };

    let plugin_path_cstring = match CString::new(plugin_path) {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid plugin path: {}", err);
            return None;
        }
    };

    let entry_init = match entry.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return None;
        }
    };

    if unsafe { !(entry_init)(plugin_path_cstring.as_ptr()) } {
        println!("Plugin entry init failed for {}", plugin_name);
        return None;
    }

    let factory = match unsafe { get_factory(entry) } {
        Ok(factory) => factory,
        Err(msg) => {
            println!("{} for {}", msg, plugin_name);
            return None;
        }
    };

    let get_plugin_count = match factory.get_plugin_count {
        Some(get_plugin_count) => get_plugin_count,
        None => {
            println!("get_plugin_count missing for {}", plugin_name);
            return None;
        }
    };

    let count = unsafe { get_plugin_count(factory) };
    if count == 0 {
        println!("No plugins found in {}", plugin_name);
        return None;
    }

    let get_plugin_descriptor = match factory.get_plugin_descriptor {
        Some(get_plugin_descriptor) => get_plugin_descriptor,
        None => {
            println!("get_plugin_descriptor missing for {}", plugin_name);
            return None;
        }
    };
    let descriptor = unsafe { get_plugin_descriptor(factory, 0) };
    if descriptor.is_null() {
        println!("Plugin descriptor is null for {}", plugin_name);
        return None;
    }

    let plugin_id = unsafe { (*descriptor).id };
    if plugin_id.is_null() {
        println!("Plugin descriptor id is null for {}", plugin_name);
        return None;
    }

    let create_plugin = match factory.create_plugin {
        Some(create_plugin) => create_plugin,
        None => {
            println!("create_plugin missing for {}", plugin_name);
            return None;
        }
    };
    let plugin_ptr = unsafe { create_plugin(factory, &HELLO_HOST, plugin_id) };

    if plugin_ptr.is_null() {
        println!("{} failed to initialize", plugin_name);
        return None;
    }

    let plugin = unsafe { &*plugin_ptr };
    let plugin_init = match plugin.init {
        Some(init) => init,
        None => {
            println!("Plugin init missing for {}", plugin_name);
            return None;
        }
    };
    if unsafe { !(plugin_init)(plugin) } {
        println!("Error: Plugin init failed");
        return None;
    }

    let _ = query_gui_extension(plugin, plugin_name);

    let is_pressed = Arc::new(AtomicBool::new(false));
    let plugin_ptr_usize = plugin_ptr as usize;

    let (host_label, device) = if let Ok(asio_host) = cpal::host_from_id(cpal::HostId::Asio) {
        if let Some(device) = select_output_device(&asio_host, "FlexASIO") {
            ("asio", device)
        } else {
            let host = cpal::default_host();
            (
                "default",
                select_output_device(&host, "FlexASIO").expect("No output device found."),
            )
        }
    } else {
        let host = cpal::default_host();
        (
            "default",
            select_output_device(&host, "FlexASIO").expect("No output device found."),
        )
    };

    let device_desc = device
        .description()
        .map(|desc| desc.to_string())
        .unwrap_or_else(|_| "Unknown device".to_string());
    println!("Output device ({}): {}", host_label, device_desc);

    let supported_config = device
        .default_output_config()
        .expect("Failed to get default output config");
    let sample_rate = supported_config.sample_rate().0 as f64;
    let sample_format = supported_config.sample_format();
    let (min_frames, max_frames) = buffer_bounds(&supported_config);
    let min_frames = min_frames.max(1);
    let max_frames = max_frames.max(min_frames);
    let stream_config = supported_config.config();

    println!(
        "Output config: {} Hz, {} ch, format {}",
        stream_config.sample_rate.0, stream_config.channels, sample_format
    );

    if let Some(activate) = plugin.activate {
        if unsafe { !(activate)(plugin, sample_rate, min_frames, max_frames) } {
            println!("Error: Plugin activation failed");
            return None;
        }
    }

    let stream = match sample_format {
        SampleFormat::F32 => build_stream_for_format::<f32>(
            &device,
            &stream_config,
            plugin_ptr_usize,
            Arc::clone(&is_pressed),
            max_frames as usize,
        ),
        SampleFormat::I16 => build_stream_for_format::<i16>(
            &device,
            &stream_config,
            plugin_ptr_usize,
            Arc::clone(&is_pressed),
            max_frames as usize,
        ),
        SampleFormat::U16 => build_stream_for_format::<u16>(
            &device,
            &stream_config,
            plugin_ptr_usize,
            Arc::clone(&is_pressed),
            max_frames as usize,
        ),
        _ => {
            println!("Unsupported sample format: {}", sample_format);
            return None;
        }
    };

    let stream = match stream {
        Ok(stream) => stream,
        Err(err) => {
            println!("Failed to build output stream: {}", err);
            return None;
        }
    };

    unsafe {
        if let Some(start_proc) = plugin.start_processing {
            (start_proc)(plugin);
        }
    }

    if let Err(err) = stream.play() {
        println!("Failed to play stream: {}", err);
        return None;
    }

    Some(AudioState::new(lib, stream, is_pressed))
}
