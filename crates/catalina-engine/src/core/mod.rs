//! Provides common types, traits, etc. that are used throughout the entire engine.

use crate::prelude::*;

use core::ops::Mul;

use float_eq::float_eq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Frequency in hertz, wraps an f32 with sufficiant 0.0001 precision for musical use.
///
/// Note that I made this frequency implementaiton a lot harder by not
/// just rounding to the nearest whole number, but I felt limiting the
/// whole system to not supporting a wider range of atonal sounds was
/// a crime.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone)]
pub struct Hertz(pub f32);

impl Hertz {
    /// Builds a frequency from hertz.
    pub fn from_hertz(value: f32) -> Self {
        Self(value)
    }

    /// Returns the frequency in hertz.
    pub fn hertz(&self) -> f32 {
        self.0
    }
}

impl From<f32> for Hertz {
    fn from(value: f32) -> Self {
        Hertz(value)
    }
}

impl From<Hertz> for f32 {
    fn from(value: Hertz) -> Self {
        value.0
    }
}

impl PartialEq for Hertz {
    fn eq(&self, other: &Self) -> bool {
        // For music, we only really care about hertz resolution down to 0.0001
        float_eq!(self.0, other.0, abs <= 0.000_1)
    }
}

// We consider the accurancy afforded by our PartialEq
// implementation "good enough" for music use, so allow Eq.
impl Eq for Hertz {}

/// Allows for directly multiplying with other frequencies.
impl Mul for Hertz {
    type Output = Hertz;

    fn mul(self, rhs: Self) -> Self::Output {
        Hertz(self.0 * rhs.0)
    }
}

/// Allows for direct multiplication with floats.
impl Mul<f32> for Hertz {
    type Output = Hertz;

    fn mul(self, rhs: f32) -> Self::Output {
        Hertz(self.0 * rhs)
    }
}

/// Allows us to properly use frequencies as keys in hashmaps.
impl Hash for Hertz {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let bits = if self.0.is_nan() {
            0x7fc00000
        } else {
            // "-0.0 + 0.0 == +0.0 under IEEE754 roundTiesToEven rounding mode,
            // which Rust guarantees. Thus by adding a positive zero we
            // canonicalize signed zero without any branches in one instruction."
            // https://github.com/reem/rust-ordered-float/blob/25da208e3e6cca1a1f9b1fcfeaec9e53f6497fa0/src/lib.rs#L2177
            (self.0 + 0.0).to_bits()
        };

        bits.hash(hasher);
    }
}
