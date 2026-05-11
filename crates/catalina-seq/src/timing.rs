/// Pulses-per-quarter-note.
///
/// Typically MIDI terminology for how the MIDI clock operates.
///
// In the sequencer, we correlate one quarter-note to 4 steps in
// the sequencer. That means the step resolution is PPQM/4.
pub const SEQUENCER_PPQM: u8 = 96;

/// Each step is divided into 24 sub-steps.
///
/// A tick is created for each sub-step.
///
/// This corrosponds to Elektron sequencer's
/// 96 pulse-per-quarter-note resolution where
/// every 4 steps is 96 "ticks".
pub const STEP_SUBSTEPS: u8 = SEQUENCER_PPQM / 4;

/// A bar on the sequencer is 4 quarter notes.
///
/// This is typically 384.
pub const TICKS_PER_BAR: u16 = SEQUENCER_PPQM as u16 * 4;

pub struct SequencerTiming {
    /// Specifies the beats-per-minute for the project.
    ///
    /// Each beat corresponds to a step advance in a sequence,
    /// but doesn't necessarily mean a sequence step is triggered,
    /// depending on conditions and microtiming adjustments.
    bpm: u8,

    /// Internal counter tracking ticks.
    ///
    /// Ticks are counted in divisions of the BMP.
    tick: u8,
}

impl SequencerTiming {
    pub fn new() -> Self {
        Self {
            bpm: 120, // sane default

            tick: 0, // counter
        }
    }

    /// Advances the project timing by a tick.
    pub fn tick(&mut self) {
        self.tick = self.tick + 1;

        if self.tick == STEP_SUBSTEPS {
            self.tick = 0;
        }
    }

    /// Returns true if the current tick is on a beat of the BMP.
    pub fn is_beat(&self) -> bool {
        self.tick == 0
    }
}

/// Allows a pattern to run in multiples of the project BMP.
#[derive(Default)]
#[repr(u8)]
pub enum TimingSpeed {
    OneEighth = 0,    // 1/8
    OneFourth = 1,    // 1/4
    Half = 2,         // 1/2
    ThreeFourths = 3, // 3/4
    #[default]
    Normal = 4, // 1
    ThreeTwos = 5,    // 3/2
    Double = 6,       // 2
}

// impl TimingSpeed {
//     pub fn should_tick(&self, tick: usize) -> bool {
//         match self {
//             TimingSpeed::OneEighth => tick % 8 == 0,
//             TimingSpeed::OneFourth => tick % 4 == 0,
//             TimingSpeed::Half => tick % 2 == 0,
//             TimingSpeed::ThreeFourths => todo!(),
//             TimingSpeed::Normal => true,
//             TimingSpeed::ThreeTwos => todo!(),
//             TimingSpeed::Double => ,
//         }
//     }
// }

/// A type for managing timing for all tracks in a
/// pattern that don't have their own timing.
pub struct PatternTiming {
    /// Specifies how many steps the track or pattern has.
    steps: usize,

    /// The current tick counter.
    ///
    /// This is incremented at a multiple
    /// of steps to support microtiming.
    ///
    /// Ticks and incremented at a multiple
    /// of the BMP set for the pattern.
    tick: u8,

    /// The current rounded sequencer step, devoid of microtiming.
    step: usize,

    /// Indicates if the last tick caused a step.
    did_step: bool,

    /// Indicates how many times the pattern has looped.
    repeats: usize,

    /// Specifies how fast the pattern or track is played relative to the BMP.
    speed: TimingSpeed,
}

pub enum TimingTickResult {
    /// Nothing of note happened.
    Tick,
    /// Indicates that this tick advanced
    /// the steps to the specified step.
    Step(usize),
    /// Indicates that this tick advanced
    /// the steps to the specified step,
    /// and looped the pattern.
    ///
    /// Second parameter contains the pattern loop counter.
    StepAndRepeat(usize, usize),
}

/// Generic pattern timing methods.
impl PatternTiming {
    pub fn new() -> Self {
        Self {
            steps: 16,

            // Counters are started at 0.
            tick: 0,
            step: 0,
            did_step: false,
            repeats: 0,
            speed: TimingSpeed::Normal,
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
    pub fn advance(&mut self) -> TimingTickResult {
        self.did_step = false;
        self.tick = self.tick + 1;

        if self.tick >= STEP_SUBSTEPS {
            self.tick = 0;

            self.did_step = true;
            self.step = self.step + 1;
            if self.step > self.steps {
                self.step = 0;
                self.repeats = self.repeats + 1;

                return TimingTickResult::StepAndRepeat(self.step, self.repeats);
            }

            return TimingTickResult::Step(self.step);
        }

        return TimingTickResult::Tick;
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
    pub fn get_tick(&self) -> u8 {
        self.tick
    }

    /// Returns how many times the sequence has repeated.
    pub fn get_repeats(&self) -> usize {
        self.repeats
    }

    /// Zeros-out the timing tracking variables for a fresh play.
    pub fn reset(&mut self) {
        self.did_step = false;
        self.tick = 0;
        self.step = 0;
        self.repeats = 0;
    }

    /// Checks if the current step is 0.
    pub fn is_first_step(&self) -> bool {
        self.step == 0
    }

    /// Checks if the current step is the last
    pub fn is_last_step(&self) -> bool {
        self.step == self.steps
    }

    /// Returns if the current tick is the last in the timing.
    pub fn is_last_tick(&self) -> bool {
        self.tick - 1 == STEP_SUBSTEPS
    }
}
