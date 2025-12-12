pub mod oscillator;

// Re-export `dasp` so the version used by the engine
// can be transparently imported by other crates.
pub use dasp::{
    Frame, Sample,
    frame::{self, Mono, Stereo},
    sample::FromSample,
    slice, *,
};

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}
