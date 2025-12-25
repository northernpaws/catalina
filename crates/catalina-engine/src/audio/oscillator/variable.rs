//! This module implements a variable shape oscillator.
//!
//! This is an oscillator that can morph it's shape at
//! runtime, in addition to it's frequency and amplitude.
//!
//! Ported from Emilie Gillet's [implementation in Mutable Instrument's Plaits](https://github.com/pichenettes/eurorack/blob/master/plaits/dsp/oscillator/variable_shape_oscillator.h) from 2016.

use crate::core::Hertz;

fn compute_naive_sample(
    phase: f32,
    pw: f32,
    slope_up: f32,
    slope_down: f32,
    triangle_amount: f32,
    square_amount: f32,
) -> f32 {
    let mut saw = phase;
    let square = if phase < pw { 0.0 } else { 1.0 };
    let triangle = if phase < pw {
        phase * slope_up
    } else {
        1.0 - (phase - pw) * slope_down
    };
    saw += (square - saw) * square_amount;
    saw += (triangle - saw) * triangle_amount;
    return saw;
}

/// Ported from https://github.com/pichenettes/stmlib/blob/d18def816c51d1da0c108236928b2bbd25c17481/dsp/polyblep.h#L41
pub fn this_blep_sample(t: f32) -> f32 {
    return 0.5 * t * t;
}

/// Ported from https://github.com/pichenettes/stmlib/blob/d18def816c51d1da0c108236928b2bbd25c17481/dsp/polyblep.h#L41
pub const fn next_blep_sample(mut t: f32) -> f32 {
    t = 1.0 - t;
    return -0.5 * t * t;
}

/// Ported from https://github.com/pichenettes/stmlib/blob/d18def816c51d1da0c108236928b2bbd25c17481/dsp/polyblep.h#L41
pub const fn next_integrated_blep_sample(t: f32) -> f32 {
    let t1 = 0.5 * t;
    let t2 = t1 * t1;
    let t4 = t2 * t2;
    return 0.1875 - t1 + 1.5 * t2 - t4;
}

/// Ported from https://github.com/pichenettes/stmlib/blob/d18def816c51d1da0c108236928b2bbd25c17481/dsp/polyblep.h#L41
pub fn this_integrated_blep_sample(t: f32) -> f32 {
    return next_integrated_blep_sample(1.0 - t);
}

/// Implements an oscillator that's waveform shape can be morphed and changed.
///
/// Ported from [Mutable Instrument's Plaits](https://github.com/pichenettes/eurorack/blob/master/plaits/dsp/oscillator/variable_shape_oscillator.h) originally written by Emilie Gillet in 2023.
pub struct VariableShapeOscillator {
    /// The sample rate of the audio engine.
    sample_rate: usize,

    enable_sync: bool,

    // Oscillator state.
    master_phase: f32,
    slave_phase: f32,
    next_sample: f32,
    previous_pw: f32,
    high: bool,

    // For interpolation of parameters.
    master_frequency: f32,
    slave_frequency: f32,
    pulse_width: f32,
    waveshape: f32,
}

impl VariableShapeOscillator {
    pub fn new(sample_rate: usize) -> Self {
        let mut osc = Self {
            sample_rate,

            enable_sync: false,

            master_phase: 0.0,
            slave_phase: 0.0,
            next_sample: 0.0,
            previous_pw: 0.5,
            high: false,

            master_frequency: 0.0,
            slave_frequency: 0.1,
            pulse_width: 0.5,
            waveshape: 0.0,
        };

        osc.set_frequency(440.0.into());
        osc.set_waveshape(0.0);
        osc.set_pulse_width(0.0);
        osc.set_sync(false);
        osc.set_sync_frequency(220.0.into());

        osc
    }

    /// Sets the frequency of the oscillator.
    pub fn set_frequency(&mut self, frequency: Hertz) {
        let freq: f32 = frequency.hertz() / self.sample_rate as f32;
        self.master_frequency = if freq >= 0.25 { 0.25 } else { freq };
    }

    /// Sets the pulse width for square waves or saw, ramp, triangle waves otherwise.
    pub fn set_pulse_width(&mut self, pw: f32) {
        if self.slave_frequency >= 0.25 {
            self.pulse_width = 0.5;
        } else {
            self.pulse_width =
                pw.clamp(self.slave_frequency * 2.0, 1.0 - 2.0 * self.slave_frequency);
        }
    }

    /// Sets the waveshape of the oscillator from saw/ramp/triangle to square.
    ///
    /// 0 is saw/ramp/triangle wave, 1 is square.
    pub fn set_waveshape(&mut self, waveshape: f32) {
        self.waveshape = waveshape;
    }

    /// Enables the sync oscillator.
    pub fn set_sync(&mut self, sync: bool) {
        self.enable_sync = sync;
    }

    /// Sets the frequency of the sync oscillator.
    pub fn set_sync_frequency(&mut self, frequency: Hertz) {
        let freq = frequency.hertz() / self.sample_rate as f32;
        self.pulse_width = if freq >= 0.25 { 0.5 } else { self.pulse_width };
        self.slave_frequency = if freq >= 0.25 { 0.25 } else { freq };
    }

    /// Reads the next sample from the oscillator.
    pub fn sample(&mut self) -> f32 {
        let mut next_sample: f32 = self.next_sample;

        let mut reset = false;
        let mut transition_during_reset = false;
        let mut reset_time: f32 = 0.0;

        let mut this_sample: f32 = next_sample;
        next_sample = 0.0;

        // TODO could calc these when setting the wavespave and pw..
        let square_amount: f32 = libm::fmaxf(self.waveshape - 0.5, 0.0) * 2.0;
        let triangle_amount: f32 = libm::fmaxf(1.0 - self.waveshape * 2.0, 0.0);
        let slope_up: f32 = 1.0 / (self.pulse_width);
        let slope_down: f32 = 1.0 / (1.0 - self.pulse_width);

        if self.enable_sync {
            self.master_phase += self.master_frequency;
            if self.master_phase >= 1.0 {
                self.master_phase -= 1.0;
                reset_time = self.master_phase / self.master_frequency;

                let mut slave_phase_at_reset: f32 =
                    self.slave_phase + (1.0 - reset_time) * self.slave_frequency;
                reset = true;
                if slave_phase_at_reset >= 1.0 {
                    slave_phase_at_reset -= 1.0;
                    transition_during_reset = true;
                }

                if !self.high && slave_phase_at_reset >= self.pulse_width {
                    transition_during_reset = true;
                }

                let value: f32 = compute_naive_sample(
                    slave_phase_at_reset,
                    self.pulse_width,
                    slope_up,
                    slope_down,
                    triangle_amount,
                    square_amount,
                );
                this_sample -= value * this_blep_sample(reset_time);
                next_sample -= value * next_blep_sample(reset_time);
            }
        }

        self.slave_phase += self.slave_frequency;
        while transition_during_reset || !reset {
            if !self.high {
                if self.slave_phase < self.pulse_width {
                    break;
                }

                let t: f32 = (self.slave_phase - self.pulse_width)
                    / (self.previous_pw - self.pulse_width + self.slave_frequency);
                let mut triangle_step: f32 = (slope_up + slope_down) * self.slave_frequency;
                triangle_step *= triangle_amount;

                this_sample += square_amount * this_blep_sample(t);
                next_sample += square_amount * next_blep_sample(t);
                this_sample -= triangle_step * this_integrated_blep_sample(t);
                next_sample -= triangle_step * next_integrated_blep_sample(t);
                self.high = true;
            }

            if self.high {
                if self.slave_phase < 1.0 {
                    break;
                }

                self.slave_phase -= 1.0;
                let t: f32 = self.slave_phase / self.slave_frequency;
                let mut triangle_step: f32 = (slope_up + slope_down) * self.slave_frequency;
                triangle_step *= triangle_amount;

                this_sample -= (1.0 - triangle_amount) * this_blep_sample(t);
                next_sample -= (1.0 - triangle_amount) * next_blep_sample(t);
                this_sample += triangle_step * this_integrated_blep_sample(t);
                next_sample += triangle_step * next_integrated_blep_sample(t);
                self.high = false;
            }
        }

        if self.enable_sync && reset {
            self.slave_phase = reset_time * self.slave_frequency;
            self.high = false;
        }

        next_sample += compute_naive_sample(
            self.slave_phase,
            self.pulse_width,
            slope_up,
            slope_down,
            triangle_amount,
            square_amount,
        );
        self.previous_pw = self.pulse_width;

        self.next_sample = next_sample;

        return 2.0 * this_sample - 1.0;
    }
}
