use catalina_engine::music::note::{CFour, Note};

use crate::{Events, ParameterID, STEP_SUBSTEPS, TICKS_PER_BAR};

/// Specifies a conditional rule used to decide if the trigger should play.
#[derive(Default, PartialEq)]
pub enum TriggerCondition {
    /// Indicates there is no trigger condition.
    #[default]
    Always,

    /// This trigger plays if the most recently evaluated
    /// trigger on the same track was true.
    ///
    /// [Previous] and [NotPrevious] conditions on the prior
    /// are ignored.
    Previous,
    /// This trigger plays if the most recently evaluated
    /// trigger on the same track was NOT true.
    ///
    /// [Previous] and [NotPrevious] conditions on the prior
    /// are ignored.
    NotPrevious,

    /// This trigger plays if the most recent evaluated
    /// trigger on the neighboring track before this
    /// track was true.
    ///
    /// [Previous] and [NotPrevious] conditions on the prior
    /// trigger evaluation on the neighbour track are ignored.
    Neighbour,
    /// This trigger plays if the most recent evaluated
    /// trigger on the neighboring track before this
    /// track was NOT true.
    ///
    /// [Previous] and [NotPrevious] conditions on the prior
    /// trigger evaluation on the neighbour track are ignored.
    NotNeighbour,

    /// This trigger plays if this is the first time the
    /// pattern has played.
    First,
    /// This trigger plays if this is NOT first time the
    /// pattern has played.
    NotFirst,

    /// This trigger plays if this is the last play of the
    /// pattern before changing to a different pattern.
    Last,
    /// This trigger plays if this is NOT the last play of the
    /// pattern before changing to a different pattern.
    NotLast,

    /// This trigger plays if how many times the track plays
    /// before this trigger condition is true.
    ///
    /// The counter is reset every specified count of track plays.
    Cycle {
        /// When in the cycle count the trigger is true.
        index: u8,
        /// How many plays of the track or pattern
        /// occur before the condition is reset.
        count: u8,
    },
}

impl TriggerCondition {
    /// Evaluates if the trigger condition is met.
    ///
    /// last_trig_eval indicates if the previously
    /// evaluated trigger on this track was true.
    ///
    /// last_neighbour_trig_eval indicates if the
    /// previously evaluated trigger on the neighboring
    /// track was true.
    ///
    /// Both prior evaluation checks skip any [Previous]
    /// or [NotPrevious] prior triggers, effectivly evaluating
    /// against the last non-[Previous] and non-[NotPrevious]
    /// trigger.
    #[must_use = "eval is useless without result check"]
    pub fn evaluate(
        &self,
        last_trig_eval: bool,
        last_neighbour_trig_eval: bool,
        pattern_change_queued: bool,
        repeats: usize,
    ) -> bool {
        match self {
            // No trigger condition means the trigger always plays.
            TriggerCondition::Always => true,
            // True if the previous trigger eval was true.
            TriggerCondition::Previous => last_trig_eval,
            // True if the previous trigger eval was false.
            TriggerCondition::NotPrevious => !last_trig_eval,
            // True if the previous trigger eval on the neighboring track was true.
            TriggerCondition::Neighbour => last_neighbour_trig_eval,
            // True is the previous trigger eval on the neighboring track was false.
            TriggerCondition::NotNeighbour => !last_neighbour_trig_eval,
            // True if this is the first play of the pattern.
            TriggerCondition::First => repeats == 0,
            // True if this is not the first play of the pattern.
            TriggerCondition::NotFirst => repeats != 0,
            // True if this is the last play of the pattern before a pattern change.
            TriggerCondition::Last => pattern_change_queued,
            // True if this is the not last play of the pattern before a pattern change.
            TriggerCondition::NotLast => !pattern_change_queued,
            // This trigger plays if how many times the track plays
            // before this trigger condition is true.
            TriggerCondition::Cycle { index, count } => {
                if *index == 0 || *count == 0 {
                    return false;
                }

                if *index != 1 && repeats == 0 {
                    return false;
                }

                if index == count {
                    return ((repeats + 1) % *count as usize) == 0;
                }

                *index as usize == ((repeats + 1) % *count as usize)
            }
        }
    }
}

/// Specifies a value for a parameter that's
/// changed by a given step triggering.
pub struct ParameterLock {
    /// The parameter that the lock exists for.
    parameter: ParameterID,
}

/// An event that can be emitted from a trigger.
///
/// These are used to plumb other systems into
/// the sequencer on each tick.
#[derive(Clone, PartialEq)]
pub enum TriggerEvent {
    /// Indicates that the trigger played a note,
    /// and what that note is.
    PlayNote {
        note: Note,
        velocity: u8,
        length: u8,
    },

    /// Indicates that the trigger changes a
    /// parameter, and what that change is.
    ParameterChange {},
}

/// Microtiming allows for specifying an offset on a trigger
/// that places it earlier or later in the sequencer grid
/// then the actual step it's on. This takes advantage of the
/// sequencer being divided into sub-steps of the BPM, typically
/// as 24 sub-steps per step.
pub struct TriggerMicrotiming(i8);

impl TriggerMicrotiming {
    /// For some reason, Elektron sequencers, which use 24 subset
    /// resolution, display microtiming as fractionals of 384.
    ///
    /// At 24 substep resolution, we get 4 steps in every quarter note.
    /// At 96 ppqn resolution, multiplying that by 4 gives us 384 sequencece
    /// ticks per bar. So these microtiming divisions are show as fractions
    /// of the offset in the entire bar.
    ///
    /// These are calculated by dividing the microtiming number by 384,
    /// but then Elektron also simplifies the fractions where possible.
    ///
    /// This method performs that calculation and returns the numberator
    /// and denominator for display.
    pub fn as_384s(&self) -> (u16, u16) {
        // If we're not using 384 ticks per bar, then this ins't valid.
        assert!(TICKS_PER_BAR == 384);

        match self.0 {
            0 => (1, 1),

            1 => (1, 384),
            2 => (1, 192),
            3 => (1, 128),
            4 => (1, 96),
            5 => (5, 384),
            6 => (1, 64),
            7 => (7, 384),
            8 => (1, 48),
            9 => (3, 128),
            10 => (5, 192),
            11 => (11, 384),
            12 => (1, 32),
            13 => (13, 384),
            14 => (7, 192),
            15 => (5, 128),
            16 => (1, 24),
            17 => (17, 384),
            18 => (3, 64),
            19 => (9, 384),
            20 => (5, 96),
            21 => (7, 128),
            22 => (11, 192),
            23 => (23, 284),
            24 => (1, 16),

            _ => {
                // NOTE: conversion to u16 is needed
                // for comparison with 384 value.
                let numerator: u16 = if self.0 < 0 {
                    (-self.0) as u16
                } else {
                    self.0 as u16
                };

                // Simply the fraction.
                let gdc = num_integer::gcd(numerator, 384);
                (numerator / gdc, 382 / gdc)
            }
        }
    }

    /// For some reason, Elektron sequencers, which use 24 subset
    /// resolution, display microtiming as fractionals of 384, where
    /// the 384 is derrived from 384 seq clicks per bar, from 96 ppqn.
    ///
    /// These are calculated by dividing the microtiming number by 384,
    /// but then Elektron also simplifies the fractions where possible.
    ///
    /// See [as_384s] for more details
    ///
    /// This converts to a known string table of string representations
    /// of the microtiming division where known for 24-substep resolution.
    pub fn to_384_str(&self) -> &str {
        // If we're not using 384 ticks per bar, then this ins't valid.
        assert!(TICKS_PER_BAR == 384);

        match self.0 {
            0 => "",

            1 => "1/384",
            2 => "1/192",
            3 => "1/128",
            4 => "1/96",
            5 => "5/384",
            6 => "1/64",
            7 => "7/384",
            8 => "1/48",
            9 => "3/128",
            10 => "5/192",
            11 => "11/384",
            12 => "1/32",
            13 => "13/384",
            14 => "7/192",
            15 => "5/128",
            16 => "1/24",
            17 => "17/384",
            18 => "3/64",
            19 => "9/384",
            20 => "5/96",
            21 => "7/128 ",
            22 => "11/192",
            23 => "23/284",
            24 => "1/16",

            _ => "",
        }
    }
}

/// A trigger placed on a step in a track.
pub struct Trigger {
    /// Specifies an offset in substeps of the BMP that will
    /// play this trigger earlier or later then the step boundary.
    ///
    /// This is defined by the PPQN resolution, which is divided by
    /// 4 to get the substep resolution. Typically this is 96 PPQN,
    /// resulting in ±24 substep resolution for microtiming.
    pub(crate) microtiming: i8,

    /// Specifies the root note played when this trigger is hit.
    ///
    /// NOTE: This is specifically disambiguated as "root" note
    ///  and note just "note" because it may be used as the root
    ///  for polyphony in some setups.
    root_note: Note,

    /// Specifies the velocity of the note played by the trigger.
    velocity: u8, // (0-127)

    /// Specifies the length of the note in steps.
    ///
    /// This defines how long it takes for a note
    /// release event to occur for the trigger.
    length: u8,

    /// Percentage of probability as 0-100.
    ///
    /// Over 100 is counted as 100.
    probability: u8,

    /// Condition that decide if this trigger
    /// is actually triggered when hit or not.
    pub(crate) condition: TriggerCondition,

    /// Per-step parameters changes programmed with triggers.
    ///
    /// These change parameters related to the track sequencing,
    /// instruments, etc. in response to this trigger being hit.
    parameter: bool,
}

/// Creates a trigger with sane defaults.
impl Default for Trigger {
    fn default() -> Self {
        Self {
            microtiming: 0,
            root_note: CFour, // Use C4(60) as the default root note
            velocity: 80,     // mid-range velocity (0-127)
            length: 1,        // one step default
            probability: 100, // default 100% chance to trigger
            condition: TriggerCondition::Always, // always trigger by default
            parameter: Default::default(),
        }
    }
}

impl Trigger {
    /// Attempt to trigger the trigger.
    ///
    /// This returns if the trigger should actually be
    /// triggered, depending on trigger conditions
    /// such as probability.
    #[must_use = "result should always be checked"]
    pub fn evaluate(
        &mut self,
        last_trig_eval: bool,
        last_neighbour_trig_eval: bool,
        pattern_change_queued: bool,
        repeats: usize,
    ) -> bool {
        // Evaluate if the condition for the trigger is met.
        let condition = self.condition.evaluate(
            last_trig_eval,
            last_neighbour_trig_eval,
            pattern_change_queued,
            repeats,
        );

        // With the trig conditions evaluated to true,
        // we can now check the probability factor.
        if self.probability < 100 {
            // Pass the probabilty to the `rand`
            // crate as a percentage to calculate.
            //
            // SAFETY: Because of the above <100 check, and it
            //  being an unsigned int, we know a /100.0 will
            //  always be in the required 0.0-1.0 range for `rand`.
            rand::random_bool(self.probability as f64 / 100.0)
        } else {
            condition
        }
    }

    /// Resets and populates the provided events buffer with events for the trigger.
    pub fn reset_and_populate_events(&self, events: &mut Events<TriggerEvent>) {
        events.reset();

        todo!();
    }

    /// Sets the microtiming for the trigger.
    ///
    /// This allows a trigger to be triggered earlier or later the the step boundary
    /// it's actually placed on, thanks to fractional BMP-derrived ticks.
    ///
    /// If the specified BPM exceeds the step resolution,
    /// then it's capped to the max resolution.
    pub fn set_microtiming(&mut self, microtiming: i8) {
        if microtiming > 0 && microtiming as u8 >= STEP_SUBSTEPS - 1 {
            // SAFETY: While technically possible for STEP_SUBSTEPS to be
            //  set high enough to overflow i8, it never should be.
            self.microtiming = STEP_SUBSTEPS as i8 - 1;
        } else if microtiming < 0 && (microtiming < -(STEP_SUBSTEPS as i8 - 1)) {
            // SAFETY: While technically possible for STEP_SUBSTEPS to be
            //  set high enough to overflow i8, it never should be.
            self.microtiming = -(STEP_SUBSTEPS as i8 - 1);
        } else {
            self.microtiming = microtiming;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trig_condition_always() {
        assert_eq!(
            TriggerCondition::Always.evaluate(false, false, false, 0),
            true
        );
    }

    #[test]
    fn test_trig_condition_previous() {
        assert_eq!(
            TriggerCondition::Previous.evaluate(true, false, false, 0),
            true
        );

        assert_eq!(
            TriggerCondition::Previous.evaluate(false, false, false, 0),
            false
        );
    }

    #[test]
    fn test_trig_condition_not_previous() {
        assert_eq!(
            TriggerCondition::NotPrevious.evaluate(true, false, false, 0),
            false
        );

        assert_eq!(
            TriggerCondition::NotPrevious.evaluate(false, false, false, 0),
            true
        );
    }

    #[test]
    fn test_trig_condition_neighbour() {
        assert_eq!(
            TriggerCondition::Neighbour.evaluate(false, true, false, 0),
            true
        );

        assert_eq!(
            TriggerCondition::Neighbour.evaluate(false, false, false, 0),
            false
        );
    }

    #[test]
    fn test_trig_condition_not_neighbour() {
        assert_eq!(
            TriggerCondition::NotNeighbour.evaluate(false, true, false, 0),
            false
        );

        assert_eq!(
            TriggerCondition::NotNeighbour.evaluate(false, false, false, 0),
            true
        );
    }

    #[test]
    fn test_trig_condition_first() {
        assert_eq!(
            TriggerCondition::First.evaluate(false, false, false, 0),
            true
        );

        assert_eq!(
            TriggerCondition::First.evaluate(false, false, false, 1),
            false
        );
    }

    #[test]
    fn test_trig_condition_not_first() {
        assert_eq!(
            TriggerCondition::NotFirst.evaluate(false, false, false, 0),
            false
        );

        assert_eq!(
            TriggerCondition::NotFirst.evaluate(false, false, false, 1),
            true
        );
    }

    #[test]
    fn test_trig_condition_last() {
        assert_eq!(TriggerCondition::Last.evaluate(false, false, true, 0), true);

        assert_eq!(
            TriggerCondition::Last.evaluate(false, false, false, 1),
            false
        );
    }

    #[test]
    fn test_trig_condition_not_last() {
        assert_eq!(
            TriggerCondition::NotLast.evaluate(false, false, true, 0),
            false
        );

        assert_eq!(
            TriggerCondition::NotLast.evaluate(false, false, false, 1),
            true
        );
    }

    #[test]
    fn test_trig_condition_cycle_1_1() {
        let one_one = TriggerCondition::Cycle { index: 1, count: 1 };

        // Should play the every time the pattern plays.
        assert_eq!(one_one.evaluate(false, false, false, 0), true);
        assert_eq!(one_one.evaluate(false, false, false, 1), true);
        assert_eq!(one_one.evaluate(false, false, false, 2), true);
        assert_eq!(one_one.evaluate(false, false, false, 3), true);
        assert_eq!(one_one.evaluate(false, false, false, 4), true);
        assert_eq!(one_one.evaluate(false, false, false, 5), true);
    }

    #[test]
    fn test_trig_condition_cycle_1_2() {
        let one_two = TriggerCondition::Cycle { index: 1, count: 2 };

        // Should play the first time the pattern plays.
        assert_eq!(one_two.evaluate(false, false, false, 0), true);
        assert_eq!(one_two.evaluate(false, false, false, 1), false);

        // .. and the third
        assert_eq!(one_two.evaluate(false, false, false, 2), true);
        assert_eq!(one_two.evaluate(false, false, false, 3), false);

        // .. and the fifth
        assert_eq!(one_two.evaluate(false, false, false, 4), true);
        assert_eq!(one_two.evaluate(false, false, false, 5), false);
    }

    #[test]
    fn test_trig_condition_cycle_2_2() {
        let two_two = TriggerCondition::Cycle { index: 2, count: 2 };

        // Should play the second time the pattern plays.
        assert_eq!(two_two.evaluate(false, false, false, 0), false);
        assert_eq!(two_two.evaluate(false, false, false, 1), true);

        // .. and the fourth
        assert_eq!(two_two.evaluate(false, false, false, 2), false);
        assert_eq!(two_two.evaluate(false, false, false, 3), true);

        // .. and the sixth
        assert_eq!(two_two.evaluate(false, false, false, 4), false);
        assert_eq!(two_two.evaluate(false, false, false, 5), true);
    }

    #[test]
    fn test_trig_condition_cycle_two_four() {
        let two_four = TriggerCondition::Cycle { index: 2, count: 4 };

        // Should play the second time the pattern plays.
        assert_eq!(two_four.evaluate(false, false, false, 0), false);
        assert_eq!(two_four.evaluate(false, false, false, 1), true);
        assert_eq!(two_four.evaluate(false, false, false, 2), false);
        assert_eq!(two_four.evaluate(false, false, false, 3), false);

        // .. and the sixth.
        assert_eq!(two_four.evaluate(false, false, false, 4), false);
        assert_eq!(two_four.evaluate(false, false, false, 5), true);
        assert_eq!(two_four.evaluate(false, false, false, 6), false);
        assert_eq!(two_four.evaluate(false, false, false, 7), false);

        // .. and the tenth.
        assert_eq!(two_four.evaluate(false, false, false, 8), false);
        assert_eq!(two_four.evaluate(false, false, false, 9), true);
        assert_eq!(two_four.evaluate(false, false, false, 10), false);
        assert_eq!(two_four.evaluate(false, false, false, 11), false);
    }

    #[test]
    fn test_trig_condition_cycle_four_seven() {
        let four_seven = TriggerCondition::Cycle { index: 4, count: 7 };

        // Should play the fourth time the pattern plays.
        assert_eq!(four_seven.evaluate(false, false, false, 0), false);
        assert_eq!(four_seven.evaluate(false, false, false, 1), false);
        assert_eq!(four_seven.evaluate(false, false, false, 2), false);
        assert_eq!(four_seven.evaluate(false, false, false, 3), true);
        assert_eq!(four_seven.evaluate(false, false, false, 4), false);
        assert_eq!(four_seven.evaluate(false, false, false, 5), false);
        assert_eq!(four_seven.evaluate(false, false, false, 6), false);

        // .. and the eleventh
        assert_eq!(four_seven.evaluate(false, false, false, 7), false);
        assert_eq!(four_seven.evaluate(false, false, false, 8), false);
        assert_eq!(four_seven.evaluate(false, false, false, 9), false);
        assert_eq!(four_seven.evaluate(false, false, false, 10), true);
        assert_eq!(four_seven.evaluate(false, false, false, 11), false);
        assert_eq!(four_seven.evaluate(false, false, false, 12), false);
        assert_eq!(four_seven.evaluate(false, false, false, 13), false);

        // .. and the eighteenth
        assert_eq!(four_seven.evaluate(false, false, false, 14), false);
        assert_eq!(four_seven.evaluate(false, false, false, 15), false);
        assert_eq!(four_seven.evaluate(false, false, false, 16), false);
        assert_eq!(four_seven.evaluate(false, false, false, 17), true);
        assert_eq!(four_seven.evaluate(false, false, false, 18), false);
        assert_eq!(four_seven.evaluate(false, false, false, 19), false);
        assert_eq!(four_seven.evaluate(false, false, false, 20), false);
        assert_eq!(four_seven.evaluate(false, false, false, 21), false);
    }
}
