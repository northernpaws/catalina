#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a note in a sequence that has a pitch, length, velocity, etc.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Note {
    /// The length of the note in steps.
    length: u32,

    /// The velocity to press the note with
    ///
    /// This is used as the velocity MIDI parameter,
    /// and fed to instruments as the note on velocity.
    velocity: u8,
}

/// A single step in a pattern containing notes and/or automation parameters.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Step {
    /// The nodes triggered by the pattern step.
    notes: [Option<Note>; 8],
}

pub struct Track<const STEPS: usize> {
    /// The steps in the pattern.
    steps: [Option<Step>; STEPS],

    /// The total length of the pattern.
    length: u8,
}

/// A pattern provides a list of [`Step`]s thats are
/// sequenced to play an instrument or create MIDI data.
pub struct Pattern<const TRACKS: usize, const STEPS: usize> {
    /// The steps in the pattern.
    tracks: [Option<Track<STEPS>>; TRACKS],
}

impl<const TRACKS: usize, const STEPS: usize> Pattern<TRACKS, STEPS> {
    pub fn new() -> Self {
        Self {
            tracks: [const { None::<Track<STEPS>> }; TRACKS],
        }
    }
}
