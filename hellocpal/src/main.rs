use clap_sys::audio_buffer::clap_audio_buffer;
use clap_sys::entry::clap_plugin_entry;
use clap_sys::events::{clap_event_header, clap_input_events, clap_output_events};
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::process::clap_process;
use clap_sys::version::CLAP_VERSION;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libloading::{Library, Symbol};
use std::ffi::CString;

unsafe extern "C" fn input_events_size(_list: *const clap_input_events) -> u32 {
    0
}

unsafe extern "C" fn input_events_get(
    _list: *const clap_input_events,
    _index: u32,
) -> *const clap_event_header {
    std::ptr::null()
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

static MY_HOST: clap_host = clap_host {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("<START>");

    let path_to_vital = r"C:\Program Files\Common Files\CLAP\Vital.clap";
    if !std::path::Path::new(path_to_vital).exists() {
        println!("0a30ca11 Vital.clap not found");

        return Ok(());
    }
    let lib = unsafe { Library::new(path_to_vital)? };

    let entry: Symbol<*const clap_plugin_entry> = unsafe { lib.get(b"clap_entry\0")? };
    let entry = unsafe { &**entry };

    unsafe { (entry.init.unwrap())(path_to_vital.as_ptr() as *const i8) };

    let factory_ptr =
        unsafe { entry.get_factory.unwrap()(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8) };

    println!(
        "d6c37660 Successfully initialized CLAP factory at: {:?}",
        factory_ptr
    );

    let factory = unsafe { &*(factory_ptr as *const clap_plugin_factory) };

    let vital_id = CString::new("audio.vital.synth").unwrap();

    let plugin_ptr =
        unsafe { (factory.create_plugin.unwrap())(factory, &MY_HOST, vital_id.as_ptr()) };

    if plugin_ptr.is_null() {
        println!("507516be Vital failed to initialize");

        return Ok(());
    }

    let plugin = unsafe { &*plugin_ptr };
    println!("ea484186 Vital instance created");

    unsafe {
        if let Some(init) = plugin.init {
            (init)(plugin);
        }
        if let Some(activate) = plugin.activate {
            (activate)(plugin, 44100.0, 1, 4096);
        }
    }

    let plugin_ptr_to_share = plugin_ptr as usize;

    let asio_host = cpal::host_from_id(cpal::HostId::Asio).ok();

    if let Some(host) = asio_host {
        println!("f092f58f ASIO host found.");

        let device = host
            .output_devices()?
            .find(|d| {
                d.description()
                    .map(|desc| desc.name().contains("FlexASIO"))
                    .unwrap_or(false)
            })
            .or_else(|| host.default_output_device())
            .expect("5d892dc5 No ASIO output device found.");

        let device_name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| "Unknown Device".to_string());
        println!("31a0243a Using Device for playback: {}", device_name);

        let config = device.default_output_config()?;
        println!("5e561a96 Config: {:?}", config);

        let mut left_out = vec![0.0f32; 4096];
        let mut right_out = vec![0.0f32; 4096];

        let in_events = clap_input_events {
            ctx: std::ptr::null_mut(),
            size: Some(input_events_size),
            get: Some(input_events_get),
        };

        let mut out_events = clap_output_events {
            ctx: std::ptr::null_mut(),
            try_push: Some(output_events_push),
        };

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let plugin_ptr = plugin_ptr_to_share as *const clap_sys::plugin::clap_plugin;
                let plugin = unsafe { &*plugin_ptr };

                let total_samples = data.len();
                if total_samples % 2 != 0 {
                    return;
                }
                let num_frames = (total_samples / 2) as u32;

                if num_frames > 4096 {
                    return;
                }

                let mut channel_ptrs = [left_out.as_mut_ptr(), right_out.as_mut_ptr()];
                let channel_ptrs_ptr = channel_ptrs.as_mut_ptr();

                let mut output_buffer = clap_audio_buffer {
                    data32: channel_ptrs_ptr,
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
                    in_events: &in_events,
                    out_events: &mut out_events,
                };

                unsafe {
                    if let Some(process_fn) = plugin.process {
                        (process_fn)(plugin, &process_data);
                    }
                }

                for i in 0..num_frames as usize {
                    data[i * 2] = left_out[i];
                    data[i * 2 + 1] = right_out[i];
                }
            },
            |err| eprintln!("d5358cdc Stream error: {}", err),
            None,
        )?;

        unsafe {
            if let Some(start_proc) = plugin.start_processing {
                (start_proc)(plugin);
                println!("75af0fc9 Plugin processing started");
            }
        }

        stream.play()?;
        println!("4c1dab84 (Stream playing)");

        std::thread::sleep(std::time::Duration::from_secs(2));

        unsafe {
            if let Some(stop_proc) = plugin.stop_processing {
                (stop_proc)(plugin);
                println!("3bdf6187 Plugin processing stopped.");
            }
            (plugin.deactivate.unwrap())(plugin);
        }
    } else {
        println!("7a273791 ASIO host not found.");
    }

    unsafe { entry.deinit.unwrap()() };
    println!("<END>");

    Ok(())
}
