use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use clap_sys::audio_buffer::clap_audio_buffer;
use clap_sys::entry::clap_plugin_entry;
use clap_sys::events::{
    clap_event_header, clap_event_note, clap_input_events, clap_output_events,
    CLAP_CORE_EVENT_SPACE_ID, CLAP_EVENT_NOTE_OFF, CLAP_EVENT_NOTE_ON,
};
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::process::clap_process;
use clap_sys::version::CLAP_VERSION;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libloading::{Library, Symbol};
use std::ffi::CString;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Resource)]
struct AudioState {
    _library: Library,
    _stream: cpal::Stream,
    is_pressed: Arc<AtomicBool>,
}

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
) -> *const std::ffi::c_void {
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640, 360).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_camera_system, setup_audio_system))
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_audio_system(mut commands: Commands) {
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
        return;
    }

    let lib = unsafe {
        Library::new(plugin_path).unwrap_or_else(|_| panic!("Failed to load {}", plugin_name))
    };

    let entry_symbol: Symbol<*const clap_plugin_entry> =
        unsafe { lib.get(b"clap_entry\0").expect("Failed to get clap_entry") };
    let entry = unsafe { &**entry_symbol };

    let plugin_path_cstring = CString::new(plugin_path).unwrap();
    unsafe { (entry.init.expect("Plugin init missing"))(plugin_path_cstring.as_ptr()) };

    let factory_ptr = unsafe {
        entry.get_factory.expect("get_factory missing")(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8)
    };
    let factory = unsafe { &*(factory_ptr as *const clap_plugin_factory) };

    let plugin_ptr = unsafe {
        let count = (factory.get_plugin_count.expect("get_plugin_count missing"))(factory);
        if count == 0 {
            println!("No plugins found in {}", plugin_name);
            return;
        }
        let descriptor = (factory
            .get_plugin_descriptor
            .expect("get_plugin_descriptor missing"))(factory, 0);

        (factory.create_plugin.expect("create_plugin missing"))(
            factory,
            &HELLO_HOST,
            (*descriptor).id,
        )
    };

    if plugin_ptr.is_null() {
        println!("{} failed to initialize", plugin_name);
        return;
    }

    let plugin = unsafe { &*plugin_ptr };
    unsafe {
        if let Some(init) = plugin.init {
            if !(init)(plugin) {
                println!("Error: Plugin init failed");
                return;
            }
        }
        if let Some(activate) = plugin.activate {
            if !(activate)(plugin, 44100.0, 1, 4096) {
                println!("Error: Plugin activation failed");
                return;
            }
        }
    }

    let is_pressed = Arc::new(AtomicBool::new(false));
    let is_pressed_clone = is_pressed.clone();
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

    let mut left_out = vec![0.0f32; 4096];
    let mut right_out = vec![0.0f32; 4096];
    let mut last_pressed = false;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let plugin_ptr = plugin_ptr_usize as *const clap_sys::plugin::clap_plugin;
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

                let current_pressed = is_pressed_clone.load(Ordering::Relaxed);

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
                        note as *mut _ as *mut std::ffi::c_void
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

                for i in 0..num_frames as usize {
                    data[i * 2] = left_out[i] * 0.2;
                    data[i * 2 + 1] = right_out[i] * 0.2;
                }
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

    stream.play().expect("Failed to play stream");

    commands.insert_resource(AudioState {
        _library: lib,
        _stream: stream,
        is_pressed,
    });
}

fn ui_example_system(mut contexts: EguiContexts, audio_state: Option<Res<AudioState>>) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("(Soundhold)");
        let button_response = ui.button("Play Sound");
        if let Some(audio_state) = audio_state {
            if button_response.is_pointer_button_down_on() {
                audio_state.is_pressed.store(true, Ordering::Relaxed);
            } else {
                audio_state.is_pressed.store(false, Ordering::Relaxed);
            }
        }
    });

    Ok(())
}
