pub mod oscillator;

pub use dasp::{Frame, Sample, sample::FromSample};

/// dasp-sample already provides a robust set of types for sample
/// managment, so we wrap those in a local crate trait.
// pub trait Sample: dasp::sample::Sample + dasp::sample::FromSample<f32> {}

pub struct Buffer<'a, F: Frame> {
    data: &'a mut [F],
}

impl<'a, F: Frame> Buffer<'a, F> {
    /// Returns how many channels are in the buffer.
    pub fn channels(&self) -> usize {
        F::CHANNELS
    }

    /// Returns the count of frames in the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub trait AudioSource<F: Frame> {
    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut Buffer<F>);
}
