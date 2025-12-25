//! A module for the [`Pitch`] enum.

use crate::core::Hertz;

/**
   MIT License

   Copyright (c) 2022 Aaron Roney
   Copyright (c) 2025 Kat Mitchell

   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:

   The above copyright notice and this permission notice shall be included in all
   copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/
// Traits.
use super::helpers::mel;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A trait for types that have a pitch property.
pub trait HasPitch {
    /// Returns the pitch of the type (usually a [`Note`]).
    fn pitch(&self) -> Pitch;
}

/// A trait for types that have a base frequency property.
pub trait HasBaseFrequency {
    /// Returns the base frequency of the type (usually a [`Pitch`]).
    fn base_frequency(&self) -> f32;
}

/// A trait for types that have a frequency property.
pub trait HasFrequency {
    /// Returns the frequency of the type (usually a [`Note`]).
    fn frequency(&self) -> f32;

    /// Returns the frequency range of the type (usually a [`Note`]).
    /// Essentially, mid way between the frequency and the next frequency on either side.
    fn frequency_range(&self) -> (f32, f32) {
        let frequency = self.frequency();

        (
            frequency * (1.0 - 1.0 / 17.462 / 2.0),
            frequency * (1.0 + 1.0 / 16.8196 / 2.0),
        )
    }

    /// Returns the tight frequency range of the type (usually a [`Note`]).
    /// Essentially, 1/8 the way between the frequency and the next frequency on either side.
    fn tight_frequency_range(&self) -> (f32, f32) {
        let frequency = self.frequency();

        (
            frequency * (1.0 - 1.0 / 17.462 / 8.0),
            frequency * (1.0 + 1.0 / 16.8196 / 8.0),
        )
    }
}

/// A trait for types that have a mel property.
pub trait HasMel: HasFrequency {
    /// Returns the mel of the type (usually a [`Note`]).
    fn mel(&self) -> f32 {
        mel(self.frequency())
    }
}

// Enum.

/// An enum representing the pitch of a note.
///
/// The frequencies of the pitches are based on the [A4 frequency](https://en.wikipedia.org/wiki/A4_(pitch_standard)).
/// There is no enharmonic representation here, so all of the sharps are represented.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Pitch {
    /// The pitch C.
    C,
    /// The pitch C♯.
    DFlat,
    /// The pitch D.
    D,
    /// The pitch D♯.
    EFlat,
    /// The pitch E.
    E,
    /// The pitch F.
    F,
    /// The pitch F♯.
    GFlat,
    /// The pitch G.
    G,
    /// The pitch G♯.
    AFlat,
    /// The pitch A.
    A,
    /// The pitch A♯.
    BFlat,
    /// The pitch B.
    B,
}

// Pitch impls.

impl Pitch {
    pub const fn base_frequency(&self) -> Hertz {
        Hertz(match self {
            Pitch::C => 16.35,
            Pitch::DFlat => 17.32,
            Pitch::D => 18.35,
            Pitch::EFlat => 19.45,
            Pitch::E => 20.60,
            Pitch::F => 21.83,
            Pitch::GFlat => 23.12,
            Pitch::G => 24.50,
            Pitch::AFlat => 25.96,
            Pitch::A => 27.50,
            Pitch::BFlat => 29.14,
            Pitch::B => 30.87,
        })
    }
}

// impl HasBaseFrequency for Pitch {
//     fn base_frequency(&self) -> f32 {
//         match self {
//             Pitch::C => 16.35,
//             Pitch::DFlat => 17.32,
//             Pitch::D => 18.35,
//             Pitch::EFlat => 19.45,
//             Pitch::E => 20.60,
//             Pitch::F => 21.83,
//             Pitch::GFlat => 23.12,
//             Pitch::G => 24.50,
//             Pitch::AFlat => 25.96,
//             Pitch::A => 27.50,
//             Pitch::BFlat => 29.14,
//             Pitch::B => 30.87,
//         }
//     }
// }

impl HasPitch for Pitch {
    fn pitch(&self) -> Pitch {
        *self
    }
}

impl TryFrom<u8> for Pitch {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Pitch::C),
            1 => Ok(Pitch::DFlat),
            2 => Ok(Pitch::D),
            3 => Ok(Pitch::EFlat),
            4 => Ok(Pitch::E),
            5 => Ok(Pitch::F),
            6 => Ok(Pitch::GFlat),
            7 => Ok(Pitch::G),
            8 => Ok(Pitch::AFlat),
            9 => Ok(Pitch::A),
            10 => Ok(Pitch::BFlat),
            11 => Ok(Pitch::B),
            _ => Err("Invalid pitch"),
        }
    }
}

// Statics.

/// An array of all the pitches.
pub static ALL_PITCHES: [Pitch; 12] = [
    Pitch::C,
    Pitch::DFlat,
    Pitch::D,
    Pitch::EFlat,
    Pitch::E,
    Pitch::F,
    Pitch::GFlat,
    Pitch::G,
    Pitch::AFlat,
    Pitch::A,
    Pitch::BFlat,
    Pitch::B,
];

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_properties() {
        self::assert_eq!(Pitch::G.pitch(), Pitch::G);
        self::assert_eq!(Pitch::G.base_frequency().hertz(), 24.50);
    }
}
