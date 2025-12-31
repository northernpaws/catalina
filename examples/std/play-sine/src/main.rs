use cpal::{
    FromSample, SizedSample,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use catalina::engine::{
    audio::oscillator::{self, Oscillator},
    core::Hertz,
};

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
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as usize;
    let channels = config.channels as usize;

    // Create a sine oscillator with a frequency of 261.63 (middle C)
    let mut osc = oscillator::RuntimeOscillator::new(
        oscillator::OscillatorType::Sine,
        sample_rate, // 48000
        Hertz::from_hertz(261.63),
    );

    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let sample = osc.sample();
                let value: T = T::from_sample(sample);
                // println!("{}, {:?}", sample_clock, sample);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
