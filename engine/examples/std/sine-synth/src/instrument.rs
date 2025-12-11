use heapless::index_map::FnvIndexMap;

use rythm_engine::{
    audio::{
        AudioSource, Buffer, Frame, FromSample, Sample,
        oscillator::{Oscillator, RuntimeOscillator},
    },
    core::Frequency,
    instrument::{Instrument, NoteError},
    theory::note::Note,
};

struct Voice {
    /// The sine oscillator used to render the voice.
    pub osc: RuntimeOscillator,

    /// A per-voice timebase for the oscillator index to allow each voice
    /// to oscillate relative to when the trigger key was pressed.
    time: usize,
}

impl Voice {
    pub fn new(osc: RuntimeOscillator) -> Self {
        Self { osc, time: 0 }
    }

    fn next_sample<S: Sample + FromSample<f32>>(&mut self) -> S {
        let sample = self.osc.sample(self.time);

        // Make sure to increment the sine time index so the oscillator.. oscillates
        self.time = (self.time + 1) & self.osc.get_sample_rate();

        sample
    }
}

/// Example instrument implementation that just plays a sine wave ocillator.
pub struct SineInstrument {
    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Since we're a basic sine synth, we use one
    /// sine wave oscillator as each synth voice.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl SineInstrument {
    pub fn new() -> Self {
        Self {
            voices: FnvIndexMap::new(),
        }
    }
}

impl<F: Frame> AudioSource<F> for SineInstrument {
    fn render(&mut self, buffer: &'_ mut Buffer<F>) {
        for i in 0..buffer.len() {
            let mut frame: [f32; 8] = [0_f32; 8];

            // Loop through each active voice and sum it to the output buffer.
            let mut j = 0;
            for (_, voice) in self.voices.iter_mut() {
                frame[j] = voice.next_sample();
                j += 1;
            }
        }
    }
}

impl<F: Frame> Instrument<F> for SineInstrument {
    fn init(&mut self) {}

    fn note_on(&mut self, note: Note, _velocity: u8) -> Result<(), NoteError> {
        // Get the frequency of the note in hertz.
        let freq = note.frequency();

        // Attempt to add a voice.
        //
        // .insert() will return an error if the voices map is full.
        self.voices
            .insert(
                note,
                Voice::new(RuntimeOscillator::new(
                    rythm_engine::audio::oscillator::OscillatorType::Sine,
                    44100,
                    freq,
                )),
            )
            .map_err(|_| NoteError::NoVoices)?;

        // There should ideally be some logic here to prempt
        // voices, but that's an exercise for later.

        Ok(())
    }

    fn note_off(&mut self, note: Note) {
        // Remove the voice for the note when the note is released.
        self.voices.remove(&note);
    }
}
