use cpal::traits::{DeviceTrait, HostTrait};

#[link(name = "advapi32")]
unsafe extern "C" {}

fn main() {
    println!("<START>");
    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .expect("failed to get default output device");
    let id = output_device
        .id()
        .expect("failed to get default output device ID");
    println!("Output device: {}", id);
    println!("<END>");
}
