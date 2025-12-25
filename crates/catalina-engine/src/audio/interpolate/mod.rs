//! An abstraction for sample/frame rate interpolation.
//!
//! The [**Interpolator**](./trait.Interpolator.html) trait provides an abstraction over different
//! types of rate interpolation.
//!
//! See the `signal` module crate (or `catalina::engine::audio::signal` module) **Converter** type for a convenient way
//! to interpolate the rate of arbitrary signals.
//!
//! The interoplation types where adapted from [dasp](https://github.com/RustAudio/dasp/tree/master)
//! under the MIT license due to it's unmaintained status leaving the published
//! crates in an unusable state for embbeded use. The uses of core_intrinsics where
//! also ported to libm to remove the nightly toolchain requirement.

use crate::audio::frame::Frame;

pub mod floor;
pub mod linear;
pub mod sinc;

/// Types that can interpolate between two values.
///
/// Implementations should keep track of the necessary data both before and after the current
/// frame.
pub trait Interpolator {
    /// The type of frame over which the interpolate may operate.
    type Frame: Frame;

    /// Given a distance between [0.0 and 1.0) toward the following sample, return the interpolated
    /// value.
    fn interpolate(&self, x: f64) -> Self::Frame;

    /// To be called whenever the Interpolator value steps passed 1.0.
    fn next_source_frame(&mut self, source_frame: Self::Frame);

    /// Resets the state of the interpolator.
    ///
    /// Call this when there's a break in the continuity of the input data stream.
    fn reset(&mut self);
}
