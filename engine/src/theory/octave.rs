//! A module for the [`Octave`] enum.

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
pub use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A trait for types that have an octave property.
pub trait HasOctave {
    /// Returns the octave of the type.
    fn octave(&self) -> Octave;
}

/// An enum representing a musical octave.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default, Ord, PartialOrd)]
#[repr(u8)]
pub enum Octave {
    /// Octave number 0.
    Zero,
    /// Octave number 1.
    One,
    /// Octave number 2.
    Two,
    /// Octave number 3.
    Three,
    /// Octave number 4.
    #[default]
    Four,
    /// Octave number 5.
    Five,
    /// Octave number 6.
    Six,
    /// Octave number 7.
    Seven,
    /// Octave number 8.
    Eight,
    /// Octave number 9.
    Nine,
    /// Octave number 10.
    Ten,
    /// Octave number 11.
    Eleven,
    /// Octave number 12.
    Twelve,
    /// Octave number 13.
    Thirteen,
    /// Octave number 14.
    Fourteen,
    /// Octave number 15.
    Fifteen,
}

impl Octave {
    #[inline]
    fn static_name(&self) -> &'static str {
        match self {
            Octave::Zero => "0",
            Octave::One => "1",
            Octave::Two => "2",
            Octave::Three => "3",
            Octave::Four => "4",
            Octave::Five => "5",
            Octave::Six => "6",
            Octave::Seven => "7",
            Octave::Eight => "8",
            Octave::Nine => "9",
            Octave::Ten => "10",
            Octave::Eleven => "11",
            Octave::Twelve => "12",
            Octave::Thirteen => "13",
            Octave::Fourteen => "14",
            Octave::Fifteen => "15",
        }
    }
}

impl Add for Octave {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let new_octave = self as u8 + rhs as u8;

        assert!(new_octave <= 15, "Octave overflow");

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { mem::transmute(new_octave) }
    }
}

impl Sub for Octave {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_octave = (self as u8)
            .checked_sub(rhs as u8)
            .expect("Octave underflow.");

        assert!(new_octave <= 15, "Octave overflow");

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { mem::transmute(new_octave) }
    }
}

impl TryFrom<u8> for Octave {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 15 {
            Err("Octave overflow.")
        } else {
            // SAFETY: The new octave is guaranteed to be less than or equal to 15.
            Ok(unsafe { mem::transmute::<u8, Octave>(value) })
        }
    }
}

impl Add<i8> for Octave {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        let new_octave = self as i8 + rhs;

        if new_octave > 15 {
            panic!("Octave overflow.");
        } else if new_octave < 0 {
            panic!("Octave underflow.");
        }

        // SAFETY: The new octave is guaranteed to be less than or equal to 15.
        unsafe { mem::transmute(new_octave) }
    }
}

impl Sub<i8> for Octave {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self + (-rhs)
    }
}

impl AddAssign for Octave {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<i8> for Octave {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl SubAssign<i8> for Octave {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl HasOctave for Octave {
    fn octave(&self) -> Octave {
        *self
    }
}

// Statics.

/// An array of all octaves.
pub static ALL_OCTAVES: [Octave; 16] = [
    Octave::Zero,
    Octave::One,
    Octave::Two,
    Octave::Three,
    Octave::Four,
    Octave::Five,
    Octave::Six,
    Octave::Seven,
    Octave::Eight,
    Octave::Nine,
    Octave::Ten,
    Octave::Eleven,
    Octave::Twelve,
    Octave::Thirteen,
    Octave::Fourteen,
    Octave::Fifteen,
];

// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theory::octave::HasOctave;
    use pretty_assertions::assert_eq;

    #[test]
    #[should_panic]
    fn test_self_overflow() {
        let _ = Octave::Fifteen + Octave::One;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_overflow() {
        let _ = Octave::Fifteen + 1;
    }

    #[test]
    #[should_panic]
    fn test_i8_add_underflow() {
        let _ = Octave::Zero + -1;
    }

    #[test]
    #[should_panic]
    fn test_i8_sub_overflow() {
        let _ = Octave::Fifteen - -1;
    }

    #[test]
    #[should_panic]
    fn test_i8_sub_underflow() {
        let _ = Octave::Zero - 1;
    }

    #[test]
    fn test_add_assign_self() {
        let mut a = Octave::Four;
        a += Octave::One;
        self::assert_eq!(a, Octave::Five);
    }

    #[test]
    fn test_add_assign_i8() {
        let mut a = Octave::Four;
        a += 1;
        self::assert_eq!(a, Octave::Five);
    }

    #[test]
    fn test_sub_assign_i8() {
        let mut a = Octave::Four;
        a -= 1;
        self::assert_eq!(a, Octave::Three);
    }

    #[test]
    fn test_properties() {
        self::assert_eq!(Octave::Four.octave(), Octave::Four);
        self::assert_eq!(Octave::default(), Octave::Four);
    }

    #[test]
    fn test_names() {
        self::assert_eq!(
            ALL_OCTAVES.map(|o| o.static_name()).join(" "),
            "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15"
        );
    }
}
