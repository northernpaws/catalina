use dasp::Frame;

use crate::{audio::AudioSource, music::note::Note};

#[derive(Debug)]
pub enum NoteError {
    NoVoices,
}

pub trait Instrument: AudioSource {
    /// Initializes the instrument for use.
    fn init(&mut self);

    // TODO: parameters

    /// Signals to the instrument that a note has been pressed.
    fn note_on(&mut self, note: Note, velocity: u8) -> Result<(), NoteError>;

    /// Signals to the instrument that a note has been released.
    fn note_off(&mut self, note: Note);
}
