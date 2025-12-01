use crate::theory::pitch::Pitch;

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
    pitch: Pitch,
    octave: Octave,
}
