use clap_sys::entry::clap_plugin_entry;
use clap_sys::factory::plugin_factory::{clap_plugin_factory, CLAP_PLUGIN_FACTORY_ID};
use clap_sys::host::clap_host;
use clap_sys::version::CLAP_VERSION;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libloading::{Library, Symbol};
use std::ffi::CString;

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
    name: b"NanoDAW\0".as_ptr() as *const i8,
    vendor: b"Independent\0".as_ptr() as *const i8,
    url: b"https://example.com\0".as_ptr() as *const i8,
    version: b"0.1.0\0".as_ptr() as *const i8,
    get_extension: Some(host_get_extension),
    request_restart: Some(host_request_restart),
    request_process: Some(host_request_process),
    request_callback: Some(host_request_callback),
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("<START>");

    let path_to_vital = r"C:\Program Files\Common Files\CLAP\Vital.clap";
    let lib = unsafe { Library::new(path_to_vital)? };

    let entry: Symbol<*const clap_plugin_entry> = unsafe { lib.get(b"clap_entry\0")? };
    let entry = unsafe { &**entry };

    unsafe { (entry.init.unwrap())(path_to_vital.as_ptr() as *const i8) };

    let factory_ptr =
        unsafe { entry.get_factory.unwrap()(CLAP_PLUGIN_FACTORY_ID.as_ptr() as *const i8) };

    println!(
        "Successfully initialized CLAP factory at: {:?}",
        factory_ptr
    );

    let factory = unsafe { &*(factory_ptr as *const clap_plugin_factory) };

    let vital_id = CString::new("audio.vital.vital").unwrap();

    let plugin_ptr =
        unsafe { (factory.create_plugin.unwrap())(factory, &MY_HOST, vital_id.as_ptr()) };

    if plugin_ptr.is_null() {
        panic!("Vital failed to initialize! Check if the ID is correct.");
    }

    let plugin = unsafe { &*plugin_ptr };
    println!("Vital Instance Created! Ready to Activate.");

    unsafe {
        (plugin.init.unwrap())(plugin);
        (plugin.activate.unwrap())(plugin, 44100.0, 1, 512);
    }

    println!("<START ASIO ENGINE>");

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
        println!("Config: {:?}", config);

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )?;

        stream.play()?;
        println!("(Stream playing)");

        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        println!("7a273791 ASIO host not found.");
    }

    unsafe { entry.deinit.unwrap()() };
    println!("<END>");

    Ok(())
}
