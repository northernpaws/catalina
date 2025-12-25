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

// TODO: cpal has an interesting oscillator algo that we might be able to adapt..
//  https://github.com/RustAudio/cpal/blob/da923a2d5a01dd7f841f648ec26aeb6c1eabfa3e/examples/synth_tones.rs#L59

use core::array;

use heapless::index_map::FnvIndexMap;

use crate::audio::sample::{FromSample, Sample};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{core::Hertz, prelude::*};

pub mod variable;

const PI2: f32 = PI * 2.0;

/// Generates a sample of a sine wave given the provided
/// phase, sample rate, frequency, and amplitude.
///
/// Phase can be calculated as (sample_index % sample_rate) / sample_rate.
pub fn sine<S: Sample + FromSample<f32>>(phase: f32) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains - for f32 it's a no-op.

    // TODO: replace 2.0*PI with TAU?
    ((2.0 * PI * phase).sin()).to_sample()
}

/// Generates a sample of a sine wave given the provided
/// time index, sample rate, frequency, and amplitude.
pub fn sample_sine<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: Hertz,
) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains - for f32 it's a no-op.

    sine(frequency.0 * index as f32 / sample_rate as f32)
}

/// Generates a sample of a saw wave given the provided
/// phase, sample rate, frequency, and amplitude.
///
/// Phase can be calculated as (sample_index % sample_rate) / sample_rate.
pub fn saw<S: Sample + FromSample<f32>>(phase: f32) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains - for f32 it's a no-op.

    (1.0 - (phase % 1.0) * 2.0).to_sample()
}

/// Generates a sample of a saw wave given the provided
/// time index, sample rate, frequency, and amplitude.
pub fn sample_saw<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: Hertz,
) -> S {
    saw(index as f32 / sample_rate as f32 * frequency.0)
}

/// Generates a sample of a triangle wave given the
/// provided phase, sample rate, and frequency.
///
/// Phase can be calculated as (sample_index % sample_rate) / sample_rate.
pub fn triangle<S: Sample + FromSample<f32>>(phase: f32) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains - for f32 it's a no-op.

    let slope = phase % 1.0 * 2.0;
    if slope < 1.0 {
        (-1.0 + slope * 2.0).to_sample()
    } else {
        (3.0 - slope * 2.0).to_sample()
    }
}

/// Generates a sample of a triangle wave given the
/// provided time index, sample rate, and frequency.
pub fn sample_triangle<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: Hertz,
) -> S {
    triangle(index as f32 / sample_rate as f32 * frequency.0)
}

/// Generates a sample of a square wave given the
/// provided phase, sample rate, and frequency.
///
/// Phase can be calculated as (sample_index % sample_rate) / sample_rate.
pub fn square<S: Sample + FromSample<f32>>(phase: f32, duty_cycle: DutyCycle) -> S {
    // Note that to_sample() handles the convertion of
    // the float-based waveform into other bit depth
    // domains - for f32 it's a no-op.

    if phase % 1.0 < duty_cycle.to_fractional() {
        (1.0).to_sample()
    } else {
        (-1.0).to_sample()
    }
}

/// Generates a sample of a square wave given the
/// provided time index, sample rate, and frequency.
pub fn sample_square<S: Sample + FromSample<f32>>(
    index: usize,
    sample_rate: usize,
    frequency: Hertz,
    duty_cycle: DutyCycle,
) -> S {
    square(index as f32 / sample_rate as f32 * frequency.0, duty_cycle)
}

/// Temporary solution to specifying an Eq compatile duty cycle.
///
/// Needs future work to allow a larger range of square wave cycles.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum DutyCycle {
    /// A duty cycle of 12.5%.
    Eight,
    /// A duty cycle of 25%.
    Quarter,
    /// A duty cycle of 33%.
    Third,
    /// A duty cycle of 50%.
    Half,
}

impl DutyCycle {
    /// Convert the duty cycle to an f32 fractional
    /// we can feed to algorithms.
    pub fn to_fractional(self) -> f32 {
        match self {
            DutyCycle::Eight => 0.125,
            DutyCycle::Quarter => 0.25,
            DutyCycle::Third => 0.33,
            DutyCycle::Half => 0.5,
        }
    }
}

impl Default for DutyCycle {
    /// The default cycle is half.
    fn default() -> Self {
        DutyCycle::Half
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

/// An error returned from building a lookup table for an oscillator.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum TableError {
    IncorrectSize { expected: usize, actual: usize },
    TableFull,
}

impl OscillatorType {
    /// Samples an oscillator waveform depending on the selected type.
    pub fn sample<S: Sample + FromSample<f32>>(&self, phase: f32, duty_cycle: DutyCycle) -> S {
        match self {
            OscillatorType::Sine => sine(phase),
            OscillatorType::Saw => saw(phase),
            OscillatorType::Triangle => triangle(phase),
            OscillatorType::Square => square(phase, duty_cycle),
        }
    }

    /// Samples an oscillator waveform depending on the selected type.
    pub fn sample_index<S: Sample + FromSample<f32>>(
        &self,
        index: usize,
        sample_rate: usize,
        frequency: Hertz,
        duty_cycle: DutyCycle,
    ) -> S {
        match self {
            OscillatorType::Sine => sample_sine(index, sample_rate, frequency),
            OscillatorType::Saw => sample_saw(index, sample_rate, frequency),
            OscillatorType::Triangle => sample_triangle(index, sample_rate, frequency),
            OscillatorType::Square => sample_square(index, sample_rate, frequency, duty_cycle),
        }
    }

    /// Fills a provided buffer with with a lookup table (also called a LUT)
    /// with the oscillator waveform for the provided sampling rate.
    pub fn build_table<S: Sample + FromSample<f32>>(
        &self,
        table: &'_ mut [S],
        sample_rate: usize,
        frequency: Hertz,
        duty_cycle: DutyCycle,
    ) -> Result<(), TableError> {
        // For this lookup we expect the table size
        // to match the provided sample rate.
        if table.len() != sample_rate {
            return Err(TableError::IncorrectSize {
                expected: sample_rate,
                actual: table.len(),
            });
        }

        match self {
            OscillatorType::Sine => {
                let mult: f32 = frequency.0 * PI2 / sample_rate as f32;

                // Note that we don't use the sample_sine function from above - there are a
                // few math optimizations we can do for sine to speed up building the table.
                for (index, row) in table.iter_mut().enumerate() {
                    *row = ((index as f32 * mult).sin()).to_sample()
                }
            }

            _ => {
                for (index, row) in table.iter_mut().enumerate() {
                    *row = self.sample_index(index, sample_rate, frequency, duty_cycle);
                }
            }
        }

        Ok(())
    }
}

/// Base trait for implementing oscillator methods with different
/// functionality (i.e. lookup-table based vs runtime).
///
/// Use this trait as a parameter typing to accept an oscillator
/// regardless of what the backing implementation is.
///
/// See [`RuntimeOscillator`] and [`LookupOscillator`] for implementations.
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct RuntimeOscillator {
    /// Specifies the type of the oscillator, used to
    /// determine which algorithm to use at runtime.
    osc_type: OscillatorType,

    sample_rate: usize,
    frequency: Hertz,

    /// Fractional duty cycle for square waves.
    duty_cycle: DutyCycle,
}

impl RuntimeOscillator {
    /// Construct a new runtime oscillator.
    pub fn new(osc_type: OscillatorType, sample_rate: usize, frequency: Hertz) -> Self {
        Self {
            osc_type,
            sample_rate,
            frequency,
            duty_cycle: DutyCycle::Half,
        }
    }

    #[inline]
    pub const fn get_sample_rate(&self) -> usize {
        self.sample_rate
    }

    /// Sample from the oscillator at the provided sample index/phase, with the provided frequency.
    ///
    /// This is unique to the RuntimeOscillator, because it calcualates the
    /// samples on-the-fly we can specify a different frequency each time.
    pub fn sample_with_frequency<S: Sample + FromSample<f32>>(
        &self,
        phase: usize,
        freq: Hertz,
    ) -> S {
        self.osc_type
            .sample_index(phase, self.sample_rate, freq, self.duty_cycle)
    }
}

impl<S: Sample + FromSample<f32>> Oscillator<S> for RuntimeOscillator {
    /// Sample from the oscillator at the provided sample index.
    fn sample(&self, index: usize) -> S {
        self.osc_type
            .sample_index(index, self.sample_rate, self.frequency, self.duty_cycle)
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

pub struct OscillatorAllocator<
    LookupSample: Sample + FromSample<f32>,
    const SAMPLE_RATE: usize,
    const MAX_TABLES: usize,
> {
    /// A hashmap for allocating the lookup tables for oscillators.
    ///
    /// Keyed by the oscillator type, frequency, and duty cycle.
    lookup: FnvIndexMap<
        (OscillatorType, Hertz, DutyCycle),
        RefCell<[LookupSample; SAMPLE_RATE]>,
        MAX_TABLES,
    >,
}

impl<LookupSample: Sample + FromSample<f32>, const SAMPLE_RATE: usize, const MAX_TABLES: usize>
    OscillatorAllocator<LookupSample, SAMPLE_RATE, MAX_TABLES>
{
    /// Get an oscillator either using an existing waveform lookup table, or by generating a new one.

    /// Tries to find an existing oscillator table with the specified
    /// oscillator waveform, generating a new one if required.
    pub fn lookup_or_allocate(
        &mut self,
        osc: OscillatorType,
        frequency: Hertz,
        duty_cycle: DutyCycle,
    ) -> Result<RefCell<[LookupSample; SAMPLE_RATE]>, TableError> {
        let table = match self
            .lookup
            .iter()
            .find(|entry| entry.0.0 == osc && entry.0.1 == frequency && entry.0.2 == duty_cycle)
        {
            Some(table) => RefCell::clone(table.1),
            None => {
                // If there was no cached lookup table, then we need to generate it.

                // TODO: this will create the table on stack which will be too big for most MCUs
                let mut table: [LookupSample; SAMPLE_RATE] = array::from_fn(|_| 0.0.to_sample());
                osc.build_table(&mut table, SAMPLE_RATE, frequency, duty_cycle)?;

                let cell = RefCell::new(table);

                // Clone the ref cell so we can return it after insert.
                let clone = RefCell::clone(&cell);

                self.lookup
                    .insert((osc, frequency, duty_cycle), cell)
                    .map_err(|_| TableError::TableFull)?;

                clone
            }
        };

        Ok(table)
    }
}
