use heapless::{Vec, index_map::FnvIndexMap};

use rythm_engine::{
    audio::{AudioSource, Buffer, oscillator::SineOscillator},
    instrument::{Instrument, NoteError},
    theory::note::Note,
};

/// Example instrument implementation that just plays a sine wave ocillator.
pub struct SineInstrument {
    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Since we're a basic sine synth, we use one
    /// sine wave oscillator as each synth voice.
    voices: FnvIndexMap<Note, SineOscillator, 8>,
}

impl SineInstrument {
    pub fn new() -> Self {
        Self {
            voices: FnvIndexMap::new(),
        }
    }
}

impl<T> AudioSource<T> for SineInstrument {
    fn render(&mut self, buffer: &'_ mut Buffer<T>) {
        // Loop through each active voice and sum it to the output buffer.
        for (_, voice) in self.voices.iter() {
            let voice_buffer = buffer.new();
            voice.render(voice_buffer);
        }
    }
}

impl<T> Instrument<T> for SineInstrument {
    fn init(&mut self) {}

    fn note_on(&mut self, note: Note, velocity: u8) -> Result<(), NoteError> {
        // Get the frequency of the note in hertz.
        let freq = note.frequency();

        // Feed the note frequency to a sine oscillator.
        let osc = SineOscillator::new(freq);

        // Attempt to add a voice.
        self.voices
            .insert(note, osc)
            .map_err(|_| NoteError::NoVoices);

        Ok(())
    }

    fn note_off(&mut self, note: Note) {
        // Remove the voice for the note when the note is released.
        self.voices.remove(&note);
    }
}
