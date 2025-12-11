use heapless::index_map::FnvIndexMap;

use rythm_engine::{
    audio::{
        AudioSource, FromSample, Sample,
        oscillator::{Oscillator, OscillatorType, RuntimeOscillator},
    },
    instrument::{Instrument, NoteError},
    music::note::Note,
};

/// A voice is one of multiple simultaneous sounds in a polyphonic synthesizer.
///
/// When a key/note on the synth is pressed it allocates a "voice" for the sound
/// that key makes. In this example, the sound of the voice is a sine oscillator.
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

    /// Takes the next sample from the oscillator and increments the voice time base.
    fn next_sample<S: Sample + FromSample<f32>>(&mut self) -> S {
        let sample = self.osc.sample(self.time);

        // Make sure to increment the sine time index so the oscillator.. oscillates
        self.time = (self.time + 1) % self.osc.get_sample_rate();

        sample
    }
}

/// Example instrument implementation with 8 polyphonic sine oscillator voices.
pub struct SineInstrument {
    sample_rate: usize,

    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Since we're a basic sine synth, we use one
    /// sine wave oscillator as each synth voice.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl SineInstrument {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,
            voices: FnvIndexMap::new(),
        }
    }
}

/// AudioSource provides the implementations for rendering
/// the instrument's sounds out as audio.
impl AudioSource for SineInstrument {
    type Frame = f32;

    fn render(&mut self, buffer: &'_ mut [f32]) {
        for i in 0..buffer.len() {
            let mut sample = 0.0;

            // Loop through each active voice and sum it to the output buffer.
            for (_, voice) in self.voices.iter_mut() {
                sample = Sample::add_amp(sample, voice.next_sample());
            }

            buffer[i] = sample;
        }
    }
}

/// Provides the instrument-related control methods.
impl Instrument for SineInstrument {
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
                    OscillatorType::Sine,
                    self.sample_rate,
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
