use heapless::index_map::FnvIndexMap;

use catalina::engine::{
    audio::{
        AudioSource, FromSample, Sample,
        oscillator::{Oscillator, OscillatorType, RuntimeOscillator},
        signal::Signal,
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
}

impl Voice {
    pub fn new(osc: RuntimeOscillator) -> Self {
        Self { osc }
    }

    /// Takes the next sample from the oscillator and increments the voice time base.
    fn next_sample<S: Sample + FromSample<f32>>(&mut self) -> S {
        let sample = self.osc.sample();

        sample
    }
}

/// Example instrument implementation with 8 polyphonic sine oscillator voices.
pub struct SineInstrument {
    sample_rate: f32,

    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Since we're a basic sine synth, we use one
    /// sine wave oscillator as each synth voice.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl SineInstrument {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            voices: FnvIndexMap::new(),
        }
    }
}

/// AudioSource provides the implementations for rendering
/// the instrument's sounds out as audio.
///
/// Note that this implementation uses f32 as the frame type,
/// which is equivalent to single-sample (aka mono) frames.
impl AudioSource for SineInstrument {
    type Frame = f32;

    /// Render out to a mono audio buffer.
    fn render(&mut self, buffer: &'_ mut [f32]) {
        for i in 0..buffer.len() {
            let mut sample = 0.0;

            // Loop through each active voice and sum them for the frame.
            for (_, voice) in self.voices.iter_mut() {
                sample = sample + voice.next_sample::<f32>();
            }

            // Note that the resulting buffer will be clipped on playback
            // depending on the voice count and frequencies.
            //
            // It's on the receiving end of the rendered buffer to apply
            // amplitude scaling to bring the audio samples down to an
            // acceptable level for playback.
            buffer[i] = sample;
        }
    }
}

impl Signal for SineInstrument {
    type Frame = f32;

    fn next(&mut self) -> Self::Frame {
        let mut sample = 0.0;

        // Loop through each active voice and sum them for the frame.
        for (_, voice) in self.voices.iter_mut() {
            sample = sample + voice.next_sample::<f32>();
        }

        // Note that the resulting buffer will be clipped on playback
        // depending on the voice count and frequencies.
        //
        // It's on the receiving end of the rendered buffer to apply
        // amplitude scaling to bring the audio samples down to an
        // acceptable level for playback.
        sample
    }
}

/// Provides the instrument-related control methods.
impl Instrument for SineInstrument {
    fn init(&mut self) {}

    fn note_on(&mut self, note: Note, _velocity: u8) -> Result<(), NoteError> {
        // Get the frequency of the note in hertz.
        //
        // We use this as the frequency of our voice oscillator so
        // that the oscillator plays in-key with the triggered note.
        let freq = note.frequency();

        println!(
            "adding note {:?} freq={} sample_rate={}",
            note, freq.0, self.sample_rate
        );

        // Attempt to add a voice.
        //
        // .insert() will return an error if the voices map is full.
        self.voices
            .insert(
                note, // This is the note we're adding a voice for
                Voice::new(RuntimeOscillator::new(
                    OscillatorType::Sine,
                    self.sample_rate,
                    freq,
                )), // This is the oscillator for the voice.
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
