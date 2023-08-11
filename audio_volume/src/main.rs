use std::fs::File;

use anyhow::Result;
use symphonia::core::{
    audio::{SampleBuffer, SignalSpec},
    errors::Error,
    io::MediaSourceStream,
    probe::Hint,
};

fn main() -> Result<()> {
    // let file = r"V:\Music\Ayesha Erotica\Music\2018 - horny.4u\Vacation Bible School.mp3";
    let file = r"V:\Music\Ayesha Erotica\Music\Covers\Hold Me Like a Microphone.mp3";
    let file = File::open(file)?;

    let (samples, spec) = load_audio(file)?;

    println!(
        "loaded {} samples across {} channels",
        samples.len(),
        spec.channels.count()
    );
    println!("RMS: {}", rms(samples.samples()));

    Ok(())
}

fn load_audio(file: File) -> Result<(SampleBuffer<f32>, SignalSpec)> {
    let media_source = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3"); // TODO:

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            media_source,
            &Default::default(),
            &Default::default(),
        )
        .unwrap();

    let mut format = probed.format;
    let track = format.default_track().unwrap();
    let track_id = track.id;

    let mut spec = None;
    let mut sample_buf = None;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &Default::default())
        .unwrap();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if sample_buf.is_none() {
                    let this_spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, this_spec));
                    spec = Some(this_spec);
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    Ok((sample_buf.unwrap(), spec.unwrap()))
}

fn rms(samples: &[f32]) -> f32 {
    let mut sum = 0.0;
    for sample in samples {
        sum += sample * sample;
    }
    (sum / samples.len() as f32).sqrt()
}
