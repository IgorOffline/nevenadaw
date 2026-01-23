use clap_sys::entry::clap_plugin_entry;
use clap_sys::factory::plugin_factory::CLAP_PLUGIN_FACTORY_ID;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use libloading::{Library, Symbol};

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
