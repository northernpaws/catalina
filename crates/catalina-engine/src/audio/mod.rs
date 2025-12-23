pub mod oscillator;

pub mod sample;
pub use sample::{FromSample, Sample};

pub mod frame;
pub use frame::{Frame, Mono, Stereo};

pub mod slice;

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}
