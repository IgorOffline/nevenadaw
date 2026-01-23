use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("<START>");

    let asio_host = cpal::host_from_id(cpal::HostId::Asio).ok();

    if let Some(host) = asio_host {
        println!("f092f58f ASIO host found.");
        let devices = host.output_devices().expect("failed to get output devices");
        println!("2fc39432 Available ASIO output devices:");
        for (i, device) in devices.enumerate() {
            let name = device
                .description()
                .map(|d| d.name().to_string())
                .unwrap_or_else(|_| "Unknown Device".to_string());
            println!("  {}. {}", i, name);
        }

        match host.default_output_device() {
            Some(device) => {
                let name = device
                    .description()
                    .map(|d| d.name().to_string())
                    .unwrap_or_else(|_| "Unknown Device".to_string());
                println!("09c79102 Default ASIO output device: {}", name);
            }
            None => {
                println!("776e005b No default ASIO output device found.");
            }
        }

        let device = host
            .output_devices()?
            .find(|d| {
                d.description()
                    .map(|desc| desc.name().contains("FlexASIO"))
                    .unwrap_or(false)
            })
            .or_else(|| host.default_output_device()) // Fallback to default if Flex not found
            .expect("5d892dc5 No ASIO output device found.");

        let device_name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| "Unknown Device".to_string());
        println!("31a0243a Using Device for playback: {}", device_name);

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate() as u32 as f32;
        let channels = config.channels() as usize;

        println!("Config: {:?}", config);

        let mut sample_clock = 0f32;
        let frequency = 440.0;

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value = (sample_clock * frequency * 2.0 * PI / sample_rate).sin();
                    for sample in frame.iter_mut() {
                        *sample = value * 0.2;
                    }
                    sample_clock += 1.0;
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )?;

        stream.play()?;
        println!("(Beep)");

        std::thread::sleep(std::time::Duration::from_secs(1));
    } else {
        println!("7a273791 ASIO host not found.");
    }

    let default_host = cpal::default_host();
    println!("3ee38461 Default host: {:?}", default_host.id());

    if let Some(device) = default_host.default_output_device() {
        let name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| "Unknown Device".to_string());
        println!("9539af33 Default output device on default host: {}", name);
    } else {
        println!("fcbedb0c No default output device found on default host.");
    }

    println!("<END>");
    Ok(())
}
