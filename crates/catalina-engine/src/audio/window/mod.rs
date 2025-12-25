//! Module for windowing over a batch of Frames. Includes default Hann and Rectangle window
//! types.
//!
//! The frame types where adapted from [dasp](https://github.com/RustAudio/dasp/tree/master)
//! under the MIT license due to it's unmaintained status leaving the published
//! crates in an unusable state for embbeded use. The uses of core_intrinsics where
//! also ported to libm to remove the nightly toolchain requirement.

pub use hann::Hann;
pub use rectangle::Rectangle;

mod hann;
mod rectangle;

/// An abstraction supporting different types of `Window` functions.
///
/// The type `S` represents the phase of the window, while the `Output` represents the window
/// amplitude.
pub trait Window<S> {
    /// The type used to represent the window amplitude.
    type Output;
    /// Returns the amplitude for the given phase, given as some `Sample` type.
    fn window(phase: S) -> Self::Output;
}
