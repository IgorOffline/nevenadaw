use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    println!("<START>");

    let asio_host = cpal::host_from_id(cpal::HostId::Asio).ok();

    if let Some(host) = asio_host {
        println!("ASIO host found.");
        let devices = host.output_devices().expect("failed to get output devices");
        println!("Available ASIO output devices:");
        for (i, device) in devices.enumerate() {
            println!(
                "  {}. {}",
                i,
                device
                    .description()
                    .expect("bfe931cc failed to get output device")
                    .name()
            );
        }

        match host.default_output_device() {
            Some(device) => {
                println!(
                    "Default ASIO output device: {}",
                    device
                        .description()
                        .expect("fdefd3bc failed to get output device")
                        .name()
                );
            }
            None => {
                println!("No default ASIO output device found.");
            }
        }
    } else {
        println!("ASIO host not found.");
    }

    let default_host = cpal::default_host();
    println!("Default host: {:?}", default_host.id());

    if let Some(device) = default_host.default_output_device() {
        println!(
            "Default output device on default host: {}",
            device
                .description()
                .expect("d68c786e failed to get output device")
                .name()
        );
    } else {
        println!("No default output device found on default host.");
    }

    println!("<END>");
}
