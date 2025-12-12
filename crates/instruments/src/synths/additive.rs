use heapless::index_map::FnvIndexMap;

use rythm_engine::{
    audio::{
        AudioSource, FromSample, Sample,
        oscillator::{self, Oscillator, OscillatorType, RuntimeOscillator},
        sample,
    },
    instrument::{Instrument, NoteError},
    music::note::{self, Note},
};

/// Implements the oscillators for the additive synth, including parameters
/// for per-oscillator phase shifting and frequency adjustments.
pub struct AdditiveOscillator {
    sample_rate: usize,

    /// Whole octave frequency offsets.
    frequency_corse: u8,
    /// Finer atonal frequency offsets.
    frequency_fine: u8,
}

impl AdditiveOscillator {
    /// Sample the oscillator for the provided note.
    pub fn sample<S: Sample + FromSample<f32>>(&self, phase: usize, note: Note) -> S {
        // Get the frequency of the note in hertz.
        //
        // We use this as the base frequency of our oscillators so
        // that the oscillator plays in-key with the triggered note.
        let note_freq = note.frequency();

        // Sample a sine wave using the provided voice phase, note frequency,
        // and the configured per-oscillator frequency offsets.
        oscillator::sample_sine(phase, self.sample_rate, note_freq + ())
    }
}

/// A type of synthesizer that adds multiple oscillators together, typically sine
/// waves, at different frequencies, amplitudes and phases to build harmonics.
pub struct AdditiveSynth {
    sample_rate: usize,

    // We have a bank of 4 oscillators that are
    // rendered and added for each voice.
    oscillators: [Option<AdditiveOscillator>; 4],

    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Each voice pair tracks the phase data for that note.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl AdditiveSynth {
    /// Construct a new instance of the additive synth.
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,

            // By default we're only populating the first oscillator.
            oscillators: [
                Some(RuntimeOscillator::new(
                    OscillatorType::Sine,
                    sample_rate,
                    note::CFour.frequency(),
                )),
                None,
                None,
                None,
            ],

            voices: FnvIndexMap::new(),
        }
    }
}

/// The interfaces for controlling the instrument from the framework.
impl Instrument for AdditiveSynth {
    fn init(&mut self) {}

    /// Called when a note is pressed.
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
                note,         // This is the note we're adding a voice for
                Voice::new(), // This holds the data for the voice.
            )
            .map_err(|_| NoteError::NoVoices)?;

        // There should ideally be some logic here to prempt
        // voices, but that's an exercise for later.

        Ok(())
    }

    /// Called when a note is released.
    fn note_off(&mut self, note: Note) {
        // Remove the voice for the note when the note is released.
        self.voices.remove(&note);
    }
}

/// The interfaces for rendering the audio output from the synth.
///
/// This is a mono implementation.
impl AudioSource for SineInstrument {
    /// Single frame type = mono.
    type Frame = f32;

    /// Render out to a mono audio buffer.
    fn render(&mut self, buffer: &'_ mut [f32]) {
        for i in 0..buffer.len() {
            let mut sample = 0.0;

            // Loop through each active voice and sum them for the frame.
            for (note, voice) in self.voices.iter_mut() {
                let mut voice_sample = 0.0;

                // Process each configured oscillator for each voice.
                for optional_osc in oscillators {
                    let Some(osc) = optional_osc else {
                        continue;
                    };

                    // Sample each configured oscillator and add them together.
                    voice_sample = voice_sample + osc.sample(voice.phase, note);

                    // Shift the base oscillator phase of the voice
                    // so that the voices oscillate independently.
                    voice.phase = (voice.phase + 1) % self.sample_rate;
                }

                sample = sample + voice_sample;
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

/// A voice renders the output sound from the synth.
///
/// In a monophonic synth there is a single voice that
/// is usually taken over by the most recent keypress.
///
/// In polyphonic synths there are several voices that
/// can play sounds from multiple keys at once.
struct Voice {
    /// Phase of the voice to be fed to the oscillators.
    ///
    /// Increments each sample, and loops back
    /// to 0 when exceeding the sample rate.
    phase: usize,
}

impl Voice {
    /// Constructs a new voice for the additive synth.
    pub fn new() -> Self {
        Self { phase: 0 }
    }
}
