use rythm_engine::audio::oscillator;

fn main() {
    // Set the specification for the wave file we're going to create.
    let spec = hound::WavSpec {
        channels: 1,         // mono
        sample_rate: 44100,  // samples per second
        bits_per_sample: 32, // bit depth
        sample_format: hound::SampleFormat::Float,
    };

    // Create a WAV writer using the specification
    let mut writer = hound::WavWriter::create("sine.wav", spec).expect("Failed to create WAV file");

    // Create a sine oscillator with a frequency of 261.63 (middle C)
    let mut osc = oscillator::SineOscillator::new(261.63, spec.sample_rate as usize);

    // 1.0 for float-based sample formats.
    osc.set_amplitude(1.0);

    let duration_secs = 2.0; // 2 seconds
    let sample_rate = spec.sample_rate as f32;
    let total_samples = (sample_rate * duration_secs) as usize;

    for t in 0..total_samples {
        let sample = osc.render::<f32>(t);
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Sine wave written to 'sine.wav'");
}
