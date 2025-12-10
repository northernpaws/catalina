use dasp::sample::FromSample;

use crate::{
    audio::{AudioSource, Sample},
    theory::note::Note,
};

pub enum NoteError {
    NoVoices,
}

pub trait Instrument<T: Sample + FromSample<f32>>: AudioSource<T> {
    /// Initializes the instrument for use.
    fn init(&mut self);

    // TODO: parameters

    /// Signals to the instrument that a note has been pressed.
    fn note_on(&mut self, note: Note, velocity: u8) -> Result<(), NoteError>;

    /// Signals to the instrument that a note has been released.
    fn note_off(&mut self, note: Note);
}
