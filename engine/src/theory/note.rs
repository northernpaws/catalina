use crate::{
    core::Frequency,
    theory::{
        named_pitch::NamedPitch,
        pitch::{HasPitch, Pitch},
    },
};

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
    pitch: NamedPitch,
    octave: Octave,
}

impl Note {
    /// Returns the octave of the note.
    pub fn octave(&self) -> Octave {
        self.octave
    }

    /// Return the pitch of the note;
    pub fn pitch(&self) -> Pitch {
        self.pitch.pitch()
    }

    /// Returns the frequency of the note in hertz.
    pub fn frequency(&self) -> Frequency {
        let mut octave = self.octave();
        let base_frequency = self.pitch().base_frequency();

        match self.pitch {
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

        base_frequency * 2.0_f32.powf(octave as u8 as f32)
    }
}
