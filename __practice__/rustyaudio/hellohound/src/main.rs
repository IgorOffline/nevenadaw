use hound::{WavSpec, WavWriter};
use uuid::Uuid;

struct Frequency {
    frequency: f64,
    name: String,
}

fn main() {
    let uuid = Uuid::new_v4();
    println!("<START {}>", uuid);
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let freq_cs3 = Frequency {
        frequency: 138.59,
        name: "cs3".to_string(),
    };
    let freq_d3 = Frequency {
        frequency: 146.83,
        name: "d3".to_string(),
    };
    let freq_e3 = Frequency {
        frequency: 164.81,
        name: "e3".to_string(),
    };
    let freq_fs3 = Frequency {
        frequency: 184.99,
        name: "fs3".to_string(),
    };
    let freq_g3 = Frequency {
        frequency: 196.00,
        name: "g3".to_string(),
    };
    let freq_a3 = Frequency {
        frequency: 220.00,
        name: "a3".to_string(),
    };
    let freq_b3 = Frequency {
        frequency: 246.94,
        name: "b3".to_string(),
    };
    let frequencies = [
        freq_cs3, freq_d3, freq_e3, freq_fs3, freq_g3, freq_a3, freq_b3,
    ];

    frequencies.iter().for_each(|frequency| {
        let filename = format!("saw_{}_v2.wav", frequency.name);
        let mut writer = WavWriter::create(filename, spec).expect("<EXPECT-1>");

        let sample_rate = 44100.0;
        let mut phase = 0.0;

        let multiplier = 44100f32 * 0.125;
        for _ in 0..((2f32 * multiplier) as i32) {
            let sample = 2.0 * phase - 1.0;
            writer
                .write_sample((sample * 32767.0) as i16)
                .expect("EXPECT-2");
            phase = (phase + frequency.frequency / sample_rate) % 1.0;
        }

        writer.finalize().expect("<EXPECT-3>");
    });

    println!("<END {}>", uuid);
}
