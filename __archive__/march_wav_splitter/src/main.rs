use aubio::{Onset, OnsetMode};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::env;
use std::error::Error;
use uuid::Uuid;

enum AudioSamples {
    Int { samples: Vec<i32>, scale: f32 },
    Float { samples: Vec<f32> },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_wav_file>", args[0]);
        return Ok(());
    }

    let input_path = &args[1];
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    println!("Input file: {}", input_path);
    println!(
        "Format: {:?} {}Hz {} channels {} bits",
        spec.sample_format, spec.sample_rate, spec.channels, spec.bits_per_sample
    );

    let samples = match spec.sample_format {
        SampleFormat::Int => {
            let samples: Vec<i32> = reader.samples::<i32>().collect::<Result<_, _>>()?;
            let bits = spec.bits_per_sample;
            let scale = if bits == 0 {
                1.0
            } else {
                (1_i64 << (bits - 1)) as f32
            };
            AudioSamples::Int { samples, scale }
        }
        SampleFormat::Float => {
            let samples: Vec<f32> = reader.samples::<f32>().collect::<Result<_, _>>()?;
            AudioSamples::Float { samples }
        }
    };

    process_and_split(samples, spec)?;

    Ok(())
}

fn process_and_split(samples: AudioSamples, spec: WavSpec) -> Result<(), Box<dyn Error>> {
    let channels = spec.channels as usize;

    let mono: Vec<f32> = match &samples {
        AudioSamples::Int { samples, scale } => samples
            .chunks_exact(channels)
            .map(|frame| frame.iter().map(|&s| s as f32 / scale).sum::<f32>() / channels as f32)
            .collect(),
        AudioSamples::Float { samples } => samples
            .chunks_exact(channels)
            .map(|frame| frame.iter().sum::<f32>() / channels as f32)
            .collect(),
    };

    let total_frames = mono.len();

    let onsets = detect_onsets_with_aubio(&mono, spec.sample_rate)?;
    let split_points = build_split_points(total_frames, &onsets);

    let hardcoded_notes = [
        "a2", "a#2", "b2", "c3", "c#3", "d3", "d#3", "e3", "f3", "f#3", "g3", "g#3", "a3", "a#3",
        "b3", "c4", "c#4", "d4", "d#4",
    ];

    for (i, &(start_frame, end_frame)) in split_points.iter().enumerate() {
        let note = hardcoded_notes.get(i).unwrap_or(&"unknown");
        let filename = format!("chunk_{:03}_{}_{}.wav", i, note, Uuid::new_v4());

        let mut writer = WavWriter::create(&filename, spec)?;
        let start_sample = start_frame * channels;
        let end_sample = end_frame * channels;

        match &samples {
            AudioSamples::Int { samples, .. } => {
                for &sample in &samples[start_sample..end_sample] {
                    writer.write_sample(sample)?;
                }
            }
            AudioSamples::Float { samples } => {
                for &sample in &samples[start_sample..end_sample] {
                    writer.write_sample(sample)?;
                }
            }
        }

        writer.finalize()?;
        println!("[{}/{}] Wrote {}", i + 1, split_points.len(), filename);
    }

    Ok(())
}

fn detect_onsets_with_aubio(mono: &[f32], sample_rate: u32) -> Result<Vec<usize>, Box<dyn Error>> {
    let buf_size = 1024usize;
    let hop_size = 512usize;

    let mut onset = Onset::new(OnsetMode::SpecFlux, buf_size, hop_size, sample_rate)?
        .with_silence(-40.0)
        .with_threshold(0.20)
        .with_minioi_s(0.35);

    let mut detected_frames = Vec::new();
    let mut block = vec![0.0f32; hop_size];

    for chunk in mono.chunks(hop_size) {
        block.fill(0.0);
        block[..chunk.len()].copy_from_slice(chunk);

        if onset.do_result(&block)? > 0.0 {
            let frame = onset.get_last();
            if frame < mono.len() {
                let is_new = detected_frames
                    .last()
                    .map(|&last| frame > last + (sample_rate as usize / 3))
                    .unwrap_or(true);

                if is_new {
                    detected_frames.push(frame);
                }
            }
        }
    }

    let edge_guard = (sample_rate as f32 * 0.03) as usize;
    detected_frames.retain(|&f| f > edge_guard && f + edge_guard < mono.len());
    detected_frames.sort_unstable();
    detected_frames.dedup();

    Ok(detected_frames)
}

fn build_split_points(total_frames: usize, onsets: &[usize]) -> Vec<(usize, usize)> {
    if total_frames == 0 {
        return Vec::new();
    }
    if onsets.is_empty() {
        return vec![(0, total_frames)];
    }

    let mut boundaries = Vec::with_capacity(onsets.len() + 2);
    boundaries.push(0);
    for pair in onsets.windows(2) {
        boundaries.push((pair[0] + pair[1]) / 2);
    }
    boundaries.push(total_frames);

    boundaries
        .windows(2)
        .filter_map(|w| {
            if w[1] > w[0] {
                Some((w[0], w[1]))
            } else {
                None
            }
        })
        .collect()
}
