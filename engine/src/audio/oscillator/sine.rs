use crate::{
    audio::{AudioSource, Sample},
    core::Frequency,
    prelude::*,
};

// TODO: add feature to work off a lookup table like https://github.com/tversteeg/usfx/blob/eb407eb200ea71e88a192f06b424b5f408635a7e/src/oscillator.rs#L46

/// An oscillator that produces a sine wave.
///
/// ref: https://blog.paramako.com/rust-audio-programming-oscillator-build-a-sine-wave-part-1
pub struct SineOscillator {
    amplitude: f32,

    /// The frequncy of the sine wave.
    ///
    /// Middle C is 261.63Hz
    frequency: Frequency,

    /// The sample rate the oscillator will be sampled at.
    ///
    /// In most cases this should match the audio stream
    /// sample rate.
    sample_rate: usize,
}

impl SineOscillator {
    pub fn new(frequency: Frequency, sample_rate: usize) -> Self {
        Self {
            amplitude: i16::MAX as f32,
            frequency,
            sample_rate,
        }
    }
}

impl SineOscillator {
    pub fn render<T: From<f32>>(&self, index: usize) -> T {
        let time = index as f32 / self.sample_rate as f32;
        (self.amplitude * (2.0 * PI * self.frequency * time).sin()).into()
    }
}

/// Implementing [`AudioSource`] for the oscillator allows the
/// oscillator to be used directly as a source in an audio chain.
impl<T: Sample> AudioSource<T> for SineOscillator {
    fn render(&mut self, buffer: &'_ mut crate::audio::Buffer<T>) {
        todo!()
    }
}
