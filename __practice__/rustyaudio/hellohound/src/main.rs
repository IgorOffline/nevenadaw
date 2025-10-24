use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    println!("<START {}>", uuid);
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let filename = uuid.to_string() + ".wav";
    let mut writer = WavWriter::create(filename, spec)?;

    let duration = 0.5;
    let frequency_cs3 = 138.59;
    let amplitude = 0.3;

    generate_beep(&mut writer, duration, frequency_cs3, amplitude)?;

    writer.finalize()?;
    println!("Beep generated");
    Ok(())
}

fn generate_beep(
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    duration: f32,
    frequency: f32,
    amplitude: f32,
) -> Result<(), hound::Error> {
    let sample_rate = writer.spec().sample_rate as f32;
    let num_samples = (duration * sample_rate) as usize;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample = (t * frequency * 2.0 * PI).sin();

        let sample_value = (sample * amplitude * i16::MAX as f32) as i16;
        writer.write_sample(sample_value)?;
    }

    Ok(())
}
