//! Module implementing various common oscillators for use in audio chains.

use dasp_sample::{FromSample, Sample};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

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

/// Base trait for implementing oscillator methods with different
/// functionality (i.e. lookup-table based vs runtime).
pub trait Oscillator {
    /// Samples the oscillator for the provided sample index.
    fn sample<S: Sample + FromSample<f32>>(&self, index: usize) -> S;
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
    pub fn new(osc_type: OscillatorType, sample_rate: usize, frequency: f32) -> Self {
        Self {
            osc_type,
            sample_rate,
            frequency,
            duty_cycle: 0.5,
        }
    }
}

impl Oscillator for RuntimeOscillator {
    fn sample<S: Sample + FromSample<f32>>(&self, index: usize) -> S {
        match self.osc_type {
            OscillatorType::Sine => sample_sine(index, self.sample_rate, self.frequency),
            OscillatorType::Saw => sample_saw(index, self.sample_rate, self.frequency),
            OscillatorType::Triangle => sample_triangle(index, self.sample_rate, self.frequency),
            OscillatorType::Square => {
                sample_square(index, self.sample_rate, self.frequency, self.duty_cycle)
            }
        }
    }
}

/*
/// Provides aa oscillator that oscillates in a sine, saw, triangle,
/// or square wave by sampling from a pre-generated lookup table.
///
/// TODO: should have some sort of support for a global lookup table
///  so that oscillators using the same parameters aren't needlessly
///  duplicating memory.
pub struct LookupOscillator<const SAMPLE_RATE: usize, S: Sample> {
    table: [S; SAMPLE_RATE * 2],
}

impl<const SAMPLE_RATE: usize> Oscillator for LookupOscillator<SAMPLE_RATE> {}
*/
