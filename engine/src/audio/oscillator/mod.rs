//! Module implementing a base set of oscillator types for use in audio chains.
//!
//! Use [`RuntimeOscillator`] on devices where you have low memory constraints
//! and calculating the waveform samples on the fly at runtime is an acceptable
//! tradeoff.
//!
//! Use [`LookupOscillator`] with an oscillator pool on devices where you have
//! lots of available memory for oscillator lookup tables. Using an appropriate
//! oscillator pool allocator means the lookup tables can be shared across
//! oscillators of the same parameters to avoid memory duplication.

use dasp::sample::{FromSample, Sample};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Generates a sample of a sine wave given the provided
/// time index, sample rate, frequency, and amplitude.
pub fn sample_sine<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: f32,
) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains.

    let time = index as f32 / sample_rate as f32;
    ((2.0 * PI * frequency * time).sin()).to_sample()
}

/// Generates a sample of a saw wave given the provided
/// time index, sample rate, frequency, and amplitude.
pub fn sample_saw<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: f32,
) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains.

    (1.0 - ((index as f32 / sample_rate as f32 * frequency) % 1.0) * 2.0).to_sample()
}

/// Generates a sample of a triangle wave given the
/// provided time index, sample rate, and frequency.
pub fn sample_triangle<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: f32,
) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains.

    let slope = (index as f32 / sample_rate as f32 * frequency) % 1.0 * 2.0;
    if slope < 1.0 {
        (-1.0 + slope * 2.0).to_sample()
    } else {
        (3.0 - slope * 2.0).to_sample()
    }
}

/// Generates a sample of a square wave given the
/// provided time index, sample rate, and frequency.
pub fn sample_square<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: f32,
    duty_cycle: f32,
) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains.

    if (index as f32 / sample_rate as f32 * frequency) % 1.0 < duty_cycle {
        (1.0).to_sample()
    } else {
        (-1.0).to_sample()
    }
}

/// Defines the type of an oscillator.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OscillatorType {
    /// A classic continuous tone at a specific frequency.
    ///
    /// Sine waves have a single carrier and no harmonics.
    Sine,

    /// A buzzy strong sound that's signature to supersaw synths.
    ///
    /// Saw waves contain both even and odd harmonics of
    /// the fundamental frequency
    Saw,

    /// A fairly smooth tonal sound, close to a sine but
    /// with some more character due to the added harmonics.
    ///
    /// Saw waves have odd harmonics, with the higher harmonics
    /// rolling off much faster than in a square wave.
    Triangle,

    /// Very buzzy and strong sounding,
    ///
    /// Square waves have odd harmonics, with the higher harmonics
    /// rolling off much slower than in a triangle wave.
    Square,
}

impl OscillatorType {
    /// Samples an oscillator waveform depending on the selected type.
    pub fn sample<S: Sample + FromSample<f32>>(
        &self,
        index: usize,
        sample_rate: usize,
        frequency: f32,
        duty_cycle: f32,
    ) -> S {
        match self {
            OscillatorType::Sine => sample_sine(index, sample_rate, frequency),
            OscillatorType::Saw => sample_saw(index, sample_rate, frequency),
            OscillatorType::Triangle => sample_triangle(index, sample_rate, frequency),
            OscillatorType::Square => sample_square(index, sample_rate, frequency, duty_cycle),
        }
    }
}

/// Base trait for implementing oscillator methods with different
/// functionality (i.e. lookup-table based vs runtime).
pub trait Oscillator<S: Sample + FromSample<f32>> {
    /// Samples the oscillator for the provided sample index.
    fn sample(&self, index: usize) -> S;
}

/// Provides an oscillator that oscillates in a sine, saw, triangle,
/// or square wave by generating the waveform at runtime.
///
/// The advantage to using this implementation is that it requires
/// significantly less memory as it has no lookup table, the downside
/// is that it takes significantly more computation time per sample.
pub struct RuntimeOscillator {
    /// Specifies the type of the oscillator, used to
    /// determine which algorithm to use at runtime.
    osc_type: OscillatorType,

    sample_rate: usize,
    frequency: f32,

    /// Fractional duty cycle for square waves.
    duty_cycle: f32,
}

impl RuntimeOscillator {
    /// Construct a new runtime oscillator.
    pub fn new(osc_type: OscillatorType, sample_rate: usize, frequency: f32) -> Self {
        Self {
            osc_type,
            sample_rate,
            frequency,
            duty_cycle: 0.5,
        }
    }
}

impl<S: Sample + FromSample<f32>> Oscillator<S> for RuntimeOscillator {
    /// Sample from the oscillator at the provided sample index.
    fn sample(&self, index: usize) -> S {
        self.osc_type
            .sample(index, self.sample_rate, self.frequency, self.duty_cycle)
    }
}

/// Provides an oscillator that oscillates in a sine, saw, triangle,
/// or square wave by sampling from a pre-generated lookup table.
///
/// TODO: should have some sort of support for a global lookup table
///  so that oscillators using the same parameters aren't needlessly
///  duplicating memory.
// TODO: ideally the table sample type would be typed so the table could be
//  cached in a different/lower sample type without requiring conversion.
pub struct LookupOscillator<'a, LookupSample: Sample + FromSample<f32>> {
    /// The table is implemented as a reference to allow a shared oscillator
    /// allocator to handle a pool of waveform lookup tables.
    ///
    /// This allows oscillators with the same parameters (type, freq, sample
    /// rate) to share the same lookup table to avoid duplicating memory.
    table: &'a [LookupSample],
}

impl<'a, LookupSample: Sample + FromSample<f32>> LookupOscillator<'a, LookupSample> {
    /// Constructs a new lookup table-based oscillator from the provided table.
    pub fn new_from_table(table: &'a [LookupSample]) -> Self {
        Self { table }
    }
}

impl<'a, LookupSample: Sample + FromSample<f32>> Oscillator<LookupSample>
    for LookupOscillator<'a, LookupSample>
{
    /// Take a sample at the specified sample index from the oscillator.
    fn sample(&self, index: usize) -> LookupSample {
        // Modulo ensures that the sample index is wrapped
        // within the sample rate of the oscillator table.
        self.table[index % self.table.len()]
    }
}
