use catalina::engine::{
    audio::{AudioSource, Frame},
    instrument::Instrument,
    music::note,
};
use cpal::{
    FromSample, Sample, SizedSample,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::instrument::SineInstrument;

// See this crate for the actual instrument implementation.
mod instrument;

fn main() -> anyhow::Result<()> {
    // Retrieve the default audio engine host for the target compilation platform.
    let default_host = cpal::default_host();

    // Retrieve the default audio output device for the current device.
    let default_out = default_host
        .default_output_device()
        .expect("failed to find output device");

    println!(
        "  Default Output Device:\n    {}",
        default_out
            .name()
            .expect("failed to read output device name")
    );

    println!("Supported Sample Formats:");
    for config in default_out.supported_output_configs().unwrap() {
        println!("\t{}", config.sample_format());
    }

    // Retrieve the audio config for the device output.
    let out_config = default_out.default_output_config().unwrap();

    println!("  Default Sample Format: {}", out_config.sample_format());
    println!("  Sample Rate: {}", out_config.sample_rate().0);
    println!("  Channels: {}", out_config.channels());

    // Run the example using the appropriate encoding given
    // the sample format expected by the output device.
    match out_config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&default_out, &out_config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&default_out, &out_config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&default_out, &out_config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&default_out, &out_config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&default_out, &out_config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&default_out, &out_config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&default_out, &out_config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&default_out, &out_config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&default_out, &out_config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&default_out, &out_config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32> + Frame,
    <T as Frame>::Sample: FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as usize;
    let channels = config.channels as usize;

    // Create an instance of the example instrument.
    let mut inst = SineInstrument::new(sample_rate as f32);

    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    let time_at_start = std::time::Instant::now();
    let mut step = 0;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Use the time since start to automatically change notes.
            let time_since_start = std::time::Instant::now()
                .duration_since(time_at_start)
                .as_secs_f32();

            // Simulate some note on and off events for now.
            if step < 1 {
                inst.note_on(note::CFour, 255).unwrap();
                step = 1;
            } else if time_since_start > 1.0 && step < 2 {
                inst.note_on(note::DFour, 255).unwrap();
                step = 2;
            } else if time_since_start > 2.0 && step < 3 {
                inst.note_on(note::GFour, 255).unwrap();
                step = 3;
            } else if time_since_start > 3.0 && step < 4 {
                inst.note_off(note::CFour);
                step = 4;
            } else if time_since_start > 4.0 && step < 5 {
                inst.note_off(note::DFour);
                step = 5;
            } else if time_since_start > 5.0 && step < 6 {
                inst.note_off(note::GFour);
                step = 6;
            }

            for frame in data.chunks_mut(channels) {
                // Render a single sample from the instrument.
                //
                // Note: This isn't terribly efficiant, should
                //  look at options for passing an entire slice
                //  in with dasp_slice with slice::to_frame_slice(
                let mut f: [f32; 1] = [0_f32; 1];
                inst.render(&mut f);

                // Write the sample to the left, and if present, the right channel.
                for sample in frame.iter_mut() {
                    // Note that we scale the sample down to avoid
                    // clipping when introducing other voices.
                    *sample = f[0].scale_amp(0.25).to_sample();
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(5000));

    Ok(())
}
