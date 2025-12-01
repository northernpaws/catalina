use crate::{audio::AudioSource, core::Frequency};

pub struct SineOscillator {
    frequency: Frequency,
}

impl SineOscillator {
    pub fn new(frequency: Frequency) -> Self {
        Self { frequency }
    }
}

/// Implementing [`AudioSource`] for the oscillator allows the
/// oscillator to be used directly as a source in an audio chain.
impl<T> AudioSource<T> for SineOscillator {
    fn render(&mut self, buffer: &'_ mut crate::audio::Buffer<T>) {
        todo!()
    }
}
