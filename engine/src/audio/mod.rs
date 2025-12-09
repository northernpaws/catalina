pub use dasp_sample::{FromSample, Sample};

pub mod oscillator;

#[repr(u8)]
pub enum Channels {
    Mono = 1,
    Stereo = 2,
}

/// dasp-sample already provides a robust set of types for sample
/// managment, so we wrap those in a local crate trait.
// pub trait Sample: dasp_sample::Sample + dasp_sample::FromSample<f32> {}

pub struct Buffer<'a, T: Sample> {
    data: &'a mut [T],
}

impl<'a, T: Sample> Buffer<'a, T> {
    /// Returns how many channels are in the buffer.
    pub fn channels(&self) -> Channels {
        Channels::Stereo
    }

    /// Returns the length of the buffer.
    pub fn frames(&self) -> usize {
        match self.channels() {
            Channels::Mono => self.data.len(),
            Channels::Stereo => self.data.len() / 2,
        }
    }

    /// Writes a sample to all audio channels at the specified frame index.
    pub fn write_mono(&mut self, frame: usize, sample: T) {
        match self.channels() {
            Channels::Mono => todo!(),
            Channels::Stereo => todo!(),
        }
    }
}

pub trait AudioSource<T: Sample + FromSample<f32>> {
    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut Buffer<T>);
}
