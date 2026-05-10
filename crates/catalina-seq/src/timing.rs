/// A type for managing timing for all tracks in a
/// pattern that don't have their own timing.
pub struct PatternTiming<const MAX_TICK: usize> {
    /// Specifies how many steps the track or pattern has.
    steps: usize,

    /// The current tick counter.
    ///
    /// This is incremented at a multiple
    /// of steps to support microtiming.
    ///
    /// Ticks and incremented at a multiple
    /// of the BMP set for the pattern.
    tick: usize,

    /// The current rounded sequencer step, devoid of microtiming.
    step: usize,

    /// Indicates if the last tick caused a step.
    did_step: bool,
}

/// Generic pattern timing methods.
impl<const MAX_TICK: usize> PatternTiming<MAX_TICK> {
    pub fn new() -> Self {
        Self {
            steps: 16,

            // Counters are started at 0.
            tick: 0,
            step: 0,
            did_step: false,
        }
    }

    /// Returns the amount of steps divided by
    /// 16 steps to represent timing as pages.
    pub fn pages(&self) -> u8 {
        // Maximum steps to fix within a 255 u8 max.
        assert!(self.steps < 4080);

        // SAFETY: we should never reasonably hit an overflow.
        (self.steps / 16usize) as u8
    }

    /// Sets the maximum steps in the track or sequence.
    pub fn set_steps(&mut self, steps: usize) {
        // Maximum steps to fix within a 255 u8 max.
        assert!(self.steps < 4080);

        self.steps = steps;
    }

    /// Advances the timing.
    ///
    /// Returns [true] if max ticks have been
    /// reached and a step should occur.
    pub fn tick(&mut self) -> bool {
        self.did_step = false;
        self.tick = self.tick + 1;

        if self.tick >= MAX_TICK {
            self.tick = 0;

            self.step = self.step + 1;
            if self.step > self.steps {
                self.step = 0;
            }
            self.did_step = true;

            return true;
        }

        return false;
    }

    /// Returns if the last tick caused a sequence step.
    pub fn get_did_step(&self) -> bool {
        self.did_step
    }

    /// Returns the current step.
    pub fn get_step(&self) -> usize {
        self.step
    }

    /// Returns the next step in the sequence, wrapping to 0 if at the end.
    pub fn get_next_step(&self) -> usize {
        if self.step + 1 == self.steps {
            return 0;
        }

        self.step
    }

    /// Returns the current tick within the step.
    pub fn get_tick(&self) -> usize {
        self.tick
    }
}
