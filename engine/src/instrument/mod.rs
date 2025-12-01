use crate::{audio::AudioSource, theory::note::Note};

pub trait Instrument<T>: AudioSource<T> {
    /// Initializes the instrument for use.
    fn init(&mut self);

    // TODO: parameters

    /// Signals to the instrument that a note has been pressed.
    fn note_on(&mut self, note: Note, velocity: u8);

    /// Signals to the instrument that a note has been released.
    fn note_off(&mut self, note: Note);
}
