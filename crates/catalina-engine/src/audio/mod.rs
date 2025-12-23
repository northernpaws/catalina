pub mod oscillator;

// Re-export `dasp` so the version used by the engine
// can be transparently imported by other crates.
pub use dasp::{
    Frame,
    frame::{self, Mono, Stereo},
    slice,
};

pub mod sample;
pub use sample::{FromSample, Sample};

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}
