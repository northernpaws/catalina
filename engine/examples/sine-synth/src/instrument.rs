use rythm_engine::{
    audio::{AudioSource, Buffer},
    instrument::Instrument,
    theory::note::Note,
};

pub struct SineInstrument {}

impl<T> AudioSource<T> for SineInstrument {
    fn render(&mut self, buffer: &'_ mut Buffer<T>) {
        todo!()
    }
}

impl<T> Instrument<T> for SineInstrument {
    fn init(&mut self) {
        todo!()
    }

    fn note_on(&mut self, note: Note, velocity: u8) {
        todo!()
    }

    fn note_off(&mut self, note: Note) {
        todo!()
    }
}
