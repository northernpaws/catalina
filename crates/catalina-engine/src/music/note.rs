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
use crate::{
    core::Hertz,
    music::{
        named_pitch::NamedPitch,
        octave::ALL_OCTAVES,
        pitch::{ALL_PITCHES, HasPitch, Pitch},
    },
};

use paste::paste;

use super::octave::Octave;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A note type.
///
/// This is a pitch with an octave.
///
/// This type allows for correctly attributing octave changes
/// across an interval from one [`Note`] to another.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Note {
    named_pitch: NamedPitch,
    octave: Octave,
}

impl Note {
    /// Returns the octave of the note.
    pub const fn octave(&self) -> Octave {
        self.octave
    }

    /// Return the pitch of the note;
    pub fn pitch(&self) -> Pitch {
        self.named_pitch.pitch()
    }

    /// Returns the frequency of the note in hertz.
    pub fn frequency(&self) -> Hertz {
        let mut octave = self.octave();
        let base_frequency = self.pitch().base_frequency();

        match self.named_pitch {
            NamedPitch::ATripleSharp
            | NamedPitch::BTripleSharp
            | NamedPitch::BDoubleSharp
            | NamedPitch::BSharp => {
                octave += 1;
            }
            NamedPitch::DTripleFlat
            | NamedPitch::CTripleFlat
            | NamedPitch::CDoubleFlat
            | NamedPitch::CFlat => {
                octave -= 1;
            }
            _ => {}
        }

        // Not sure why we need the +1.0 on the end, but without it all the tuning was 1 octave off.
        base_frequency * 2.0_f32.powf(octave as u8 as f32)
    }
}

/// Defines a note from a [`NamedPitch`].
#[rustfmt::skip]
macro_rules! define_note {
    ( $name:ident, $named_pitch:expr, $octave_num:ident, $octave:expr) => {
        paste! {
            /// The note [<$name$octave_num>].
            pub const [<$name$octave_num>]: Note = Note {
                named_pitch: $named_pitch,
                octave: $octave,
            };
        }
    };
}

/// Defines an octave of notes.
macro_rules! define_octave {
    ($octave_num:ident, $octave:expr) => {
        define_note!(FTripleFlat, NamedPitch::FTripleFlat, $octave_num, $octave);
        define_note!(CTripleFlat, NamedPitch::CTripleFlat, $octave_num, $octave);
        define_note!(GTripleFlat, NamedPitch::GTripleFlat, $octave_num, $octave);
        define_note!(DTripleFlat, NamedPitch::DTripleFlat, $octave_num, $octave);
        define_note!(ATripleFlat, NamedPitch::ATripleFlat, $octave_num, $octave);
        define_note!(ETripleFlat, NamedPitch::ETripleFlat, $octave_num, $octave);
        define_note!(BTripleFlat, NamedPitch::BTripleFlat, $octave_num, $octave);

        define_note!(FDoubleFlat, NamedPitch::FDoubleFlat, $octave_num, $octave);
        define_note!(CDoubleFlat, NamedPitch::CDoubleFlat, $octave_num, $octave);
        define_note!(GDoubleFlat, NamedPitch::GDoubleFlat, $octave_num, $octave);
        define_note!(DDoubleFlat, NamedPitch::DDoubleFlat, $octave_num, $octave);
        define_note!(ADoubleFlat, NamedPitch::ADoubleFlat, $octave_num, $octave);
        define_note!(EDoubleFlat, NamedPitch::EDoubleFlat, $octave_num, $octave);
        define_note!(BDoubleFlat, NamedPitch::BDoubleFlat, $octave_num, $octave);

        define_note!(FFlat, NamedPitch::FFlat, $octave_num, $octave);
        define_note!(CFlat, NamedPitch::CFlat, $octave_num, $octave);
        define_note!(GFlat, NamedPitch::GFlat, $octave_num, $octave);
        define_note!(DFlat, NamedPitch::DFlat, $octave_num, $octave);
        define_note!(AFlat, NamedPitch::AFlat, $octave_num, $octave);
        define_note!(EFlat, NamedPitch::EFlat, $octave_num, $octave);
        define_note!(BFlat, NamedPitch::BFlat, $octave_num, $octave);

        define_note!(F, NamedPitch::F, $octave_num, $octave);
        define_note!(C, NamedPitch::C, $octave_num, $octave);
        define_note!(G, NamedPitch::G, $octave_num, $octave);
        define_note!(D, NamedPitch::D, $octave_num, $octave);
        define_note!(A, NamedPitch::A, $octave_num, $octave);
        define_note!(E, NamedPitch::E, $octave_num, $octave);
        define_note!(B, NamedPitch::B, $octave_num, $octave);

        define_note!(FSharp, NamedPitch::FSharp, $octave_num, $octave);
        define_note!(CSharp, NamedPitch::CSharp, $octave_num, $octave);
        define_note!(GSharp, NamedPitch::GSharp, $octave_num, $octave);
        define_note!(DSharp, NamedPitch::DSharp, $octave_num, $octave);
        define_note!(ASharp, NamedPitch::ASharp, $octave_num, $octave);
        define_note!(ESharp, NamedPitch::ESharp, $octave_num, $octave);
        define_note!(BSharp, NamedPitch::BSharp, $octave_num, $octave);

        define_note!(FDoubleSharp, NamedPitch::FDoubleSharp, $octave_num, $octave);
        define_note!(CDoubleSharp, NamedPitch::CDoubleSharp, $octave_num, $octave);
        define_note!(GDoubleSharp, NamedPitch::GDoubleSharp, $octave_num, $octave);
        define_note!(DDoubleSharp, NamedPitch::DDoubleSharp, $octave_num, $octave);
        define_note!(ADoubleSharp, NamedPitch::ADoubleSharp, $octave_num, $octave);
        define_note!(EDoubleSharp, NamedPitch::EDoubleSharp, $octave_num, $octave);
        define_note!(BDoubleSharp, NamedPitch::BDoubleSharp, $octave_num, $octave);

        define_note!(FTripleSharp, NamedPitch::FTripleSharp, $octave_num, $octave);
        define_note!(CTripleSharp, NamedPitch::CTripleSharp, $octave_num, $octave);
        define_note!(GTripleSharp, NamedPitch::GTripleSharp, $octave_num, $octave);
        define_note!(DTripleSharp, NamedPitch::DTripleSharp, $octave_num, $octave);
        define_note!(ATripleSharp, NamedPitch::ATripleSharp, $octave_num, $octave);
        define_note!(ETripleSharp, NamedPitch::ETripleSharp, $octave_num, $octave);
        define_note!(BTripleSharp, NamedPitch::BTripleSharp, $octave_num, $octave);
    };
}

// Define octaves.

define_octave!(Zero, Octave::Zero);
define_octave!(One, Octave::One);
define_octave!(Two, Octave::Two);
define_octave!(Three, Octave::Three);
define_octave!(Four, Octave::Four);
define_octave!(Five, Octave::Five);
define_octave!(Six, Octave::Six);
define_octave!(Seven, Octave::Seven);
define_octave!(Eight, Octave::Eight);
define_octave!(Nine, Octave::Nine);
define_octave!(Ten, Octave::Ten);

// Define notes.

/// The default F triple flat (in the fourth octave).
pub const FTripleFlat: Note = FTripleFlatFour;
/// The default C triple flat (in the fourth octave).
pub const CTripleFlat: Note = CTripleFlatFour;
/// The default G triple flat (in the fourth octave).
pub const GTripleFlat: Note = GTripleFlatFour;
/// The default D triple flat (in the fourth octave).
pub const DTripleFlat: Note = DTripleFlatFour;
/// The default A triple flat (in the fourth octave).
pub const ATripleFlat: Note = ATripleFlatFour;
/// The default E triple flat (in the fourth octave).
pub const ETripleFlat: Note = ETripleFlatFour;
/// The default B triple flat (in the fourth octave).
pub const BTripleFlat: Note = BTripleFlatFour;

/// The default F double flat (in the fourth octave).
pub const FDoubleFlat: Note = FDoubleFlatFour;
/// The default C double flat (in the fourth octave).
pub const CDoubleFlat: Note = CDoubleFlatFour;
/// The default G double flat (in the fourth octave).
pub const GDoubleFlat: Note = GDoubleFlatFour;
/// The default D double flat (in the fourth octave).
pub const DDoubleFlat: Note = DDoubleFlatFour;
/// The default A double flat (in the fourth octave).
pub const ADoubleFlat: Note = ADoubleFlatFour;
/// The default E double flat (in the fourth octave).
pub const EDoubleFlat: Note = EDoubleFlatFour;
/// The default B double flat (in the fourth octave).
pub const BDoubleFlat: Note = BDoubleFlatFour;

/// The default F flat (in the fourth octave).
pub const FFlat: Note = FFlatFour;
/// The default C flat (in the fourth octave).
pub const CFlat: Note = CFlatFour;
/// The default G flat (in the fourth octave).
pub const GFlat: Note = GFlatFour;
/// The default D flat (in the fourth octave).
pub const DFlat: Note = DFlatFour;
/// The default A flat (in the fourth octave).
pub const AFlat: Note = AFlatFour;
/// The default E flat (in the fourth octave).
pub const EFlat: Note = EFlatFour;
/// The default B flat (in the fourth octave).
pub const BFlat: Note = BFlatFour;

/// The default F (in the fourth octave).
pub const F: Note = FFour;
/// The default C (in the fourth octave).
pub const C: Note = CFour;
/// The default G (in the fourth octave).
pub const G: Note = GFour;
/// The default D (in the fourth octave).
pub const D: Note = DFour;
/// The default A (in the fourth octave).
pub const A: Note = AFour;
/// The default E (in the fourth octave).
pub const E: Note = EFour;
/// The default B (in the fourth octave).
pub const B: Note = BFour;

/// The default F sharp (in the fourth octave).
pub const FSharp: Note = FSharpFour;
/// The default C sharp (in the fourth octave).
pub const CSharp: Note = CSharpFour;
/// The default G sharp (in the fourth octave).
pub const GSharp: Note = GSharpFour;
/// The default D sharp (in the fourth octave).
pub const DSharp: Note = DSharpFour;
/// The default A sharp (in the fourth octave).
pub const ASharp: Note = ASharpFour;
/// The default E sharp (in the fourth octave).
pub const ESharp: Note = ESharpFour;
/// The default B sharp (in the fourth octave).
pub const BSharp: Note = BSharpFour;

/// The default F double sharp (in the fourth octave).
pub const FDoubleSharp: Note = FDoubleSharpFour;
/// The default C double sharp (in the fourth octave).
pub const CDoubleSharp: Note = CDoubleSharpFour;
/// The default G double sharp (in the fourth octave).
pub const GDoubleSharp: Note = GDoubleSharpFour;
/// The default D double sharp (in the fourth octave).
pub const DDoubleSharp: Note = DDoubleSharpFour;
/// The default A double sharp (in the fourth octave).
pub const ADoubleSharp: Note = ADoubleSharpFour;
/// The default E double sharp (in the fourth octave).
pub const EDoubleSharp: Note = EDoubleSharpFour;
/// The default B double sharp (in the fourth octave).
pub const BDoubleSharp: Note = BDoubleSharpFour;

/// The default F triple sharp (in the fourth octave).
pub const FTripleSharp: Note = FTripleSharpFour;
/// The default C triple sharp (in the fourth octave).
pub const CTripleSharp: Note = CTripleSharpFour;
/// The default G triple sharp (in the fourth octave).
pub const GTripleSharp: Note = GTripleSharpFour;
/// The default D triple sharp (in the fourth octave).
pub const DTripleSharp: Note = DTripleSharpFour;
/// The default A triple sharp (in the fourth octave).
pub const ATripleSharp: Note = ATripleSharpFour;
/// The default E triple sharp (in the fourth octave).
pub const ETripleSharp: Note = ETripleSharpFour;
/// The default B triple sharp (in the fourth octave).
pub const BTripleSharp: Note = BTripleSharpFour;

// Statics.

#[cfg(feature = "std")]
use std::sync::LazyLock;

/// All the notes in all octaves.
#[cfg(feature = "std")]
pub static ALL_PITCH_NOTES: LazyLock<[Note; 192]> = LazyLock::new(|| {
    let mut all_notes = Vec::with_capacity(132);

    for octave in ALL_OCTAVES.iter() {
        for pitch in ALL_PITCHES.iter() {
            all_notes.push(Note {
                octave: *octave,
                named_pitch: pitch.into(),
            });
        }
    }

    all_notes.try_into().unwrap()
});

// All the notes in all octaves with their frequency.
// #[cfg(feature = "std")]
// pub static ALL_PITCH_NOTES_WITH_FREQUENCY: LazyLock<[(Note, f32); 192]> = LazyLock::new(|| {
//     let mut all_notes = Vec::with_capacity(132);

//     for note in ALL_PITCH_NOTES.iter() {
//         all_notes.push((*note, note.frequency()));
//     }

//     all_notes.try_into().unwrap()
// });
