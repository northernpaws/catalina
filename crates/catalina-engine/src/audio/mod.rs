//! The audio crate provides the DSP fundamentals for building audio and synthesis chains.
//!
//! In this crate you'll find most of the building blocks you need to perform most common DSP
//! tasks. Combining these blocks together can be used to make a larger audio processing change
//! or for more complex sound synthesis.

// Traits for working with audio samples.
// Ported from dasp.
pub mod sample;
pub use sample::{FromSample, Sample};

// Traits for working with audio frames, one or
// more samples based on the sampling rate.
// Ported from dasp.
pub mod frame;
pub use frame::{Frame, Mono, Stereo};

// Traits and functions for working with slices of samples and frames.
// Ported from dasp.
pub mod slice;

// Ported from dasp.
pub mod window;

// Ported from dasp.
pub mod rms;

// Ported from dasp.
pub mod peak;

// Provides functions for ample/frame rate interpolation.
// Ported from dasp.
pub mod interpolate;

// Traits and functions working with audio signals.
// Ported from dasp.
pub mod signal;

// Traits and implementations for working with oscillators.
pub mod oscillator;

pub mod envelope;

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}
