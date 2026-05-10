use crate::{ParameterID, PatternTiming};

pub struct MicrotimingStep {
    /// The tick to trigger the step on.
    tick: usize,

    /// Index of the step to trigger.
    ///
    /// We don't store the step directly because
    /// it could have been deleted between when
    /// microtiming was calculated, and when it
    /// actually got executed.
    step: usize,
}

/// Indicates if an event was triggered by a track trick.
pub enum TrackTickResult {
    /// Nothing of note happened in the tick.
    Ticked,

    /// Indicates that a trigger was evaluated on the
    /// tick, and the result of that evaluation.
    TriggerEvaluated(bool),
}

/// A track contains an assortment of steps that
/// trigger a machine associated with a track.
pub struct Track<const MAX_STEPS: usize, const MAX_TICK: usize> {
    steps: [Option<Trigger<MAX_TICK>>; MAX_STEPS],

    /// Configures the timing for the track.
    ///
    /// When [None] then the parent pattern timing is used.
    timing: Option<PatternTiming<MAX_TICK>>,

    this_step: Option<MicrotimingStep>,
    next_step: Option<MicrotimingStep>,

    /// Indicates if the most recently evaluated trigger in the track was true.
    ///
    /// Used for conditional trigger sequencing.
    last_trig_eval: bool,
}

impl<const MAX_STEPS: usize, const MAX_TICK: usize> Track<MAX_STEPS, MAX_TICK> {
    /// Sets the maximum steps in the track.
    pub fn set_steps(&mut self, steps: usize) {
        // Enable per-track timing if required.
        let timing = match &mut self.timing {
            Some(timing) => timing,
            None => {
                self.timing = Some(PatternTiming::new());
                self.timing
                    .as_mut()
                    .expect("timing should have been populated")
            }
        };

        timing.set_steps(steps);

        // Clear out any steps populated after the new maximum.
        for step in &mut self.steps[steps..] {
            *step = None;
        }
    }

    /// Returns if this track has per-track timing.
    pub fn has_track_timing(&self) -> bool {
        self.timing.is_some()
    }

    /// Returns what the most recently evaluated trigger resolved to.
    pub fn get_last_trig_eval(&self) -> bool {
        self.last_trig_eval
    }

    /// Resets timing parameters and tracking when the track is queued.
    pub fn reset(&mut self) {
        self.last_trig_eval = false;

        // Restart the timing to zero out the counters.
        if let Some(timing) = &mut self.timing {
            timing.reset();
        }
    }

    /// Tick the track in the sequence.
    ///
    /// Receives the pattern timing information which is used
    /// if no tracks-specific timing information is present.
    pub fn tick(
        &mut self,
        pattern_timing: &PatternTiming<MAX_TICK>,
        last_neighbour_trig_eval: bool,
        pattern_change_queued: bool,
    ) -> TrackTickResult {
        // Determine whether to use a pattern or track specific timing.
        let timing = match &mut self.timing {
            Some(timing) => {
                // Tick the timing if we're using track-specific timing.
                //
                // Not requred when using pattern timing as it'll have
                // already been ticked before the track tick is called.
                timing.tick();

                // SAFETY: we explicity remove the mutability and don't
                //  take pattern_timing as mut to ensure we're not
                //  acidentally modifing pattern timing per-track.
                timing as &_
            }
            None => pattern_timing,
        };

        // NOTE: negative microtiming on the FIRST step of a track won't be triggered on first
        // play, it will only be considered on the LAST step of the sequence as the next step.

        let mut to_trigger: Option<usize> = None;

        // If this tick advanced the step, we need to check if we're triggering a step.
        if timing.get_did_step() {
            // Check if there's a trig on the step.
            if let Some(step) = &mut self.steps[timing.get_step()] {
                // If there's no microtiming set, then
                // we can immediately trigger the step.
                if step.microtiming == 0 {
                    to_trigger = Some(timing.get_step());
                } else {
                    // Otherwise, we need to calculate when to trigger
                    // the next step based on the microtiming.

                    // If the microtiming is in the future, then we just
                    // delay the trigger until the required amount of ticks.
                    if step.microtiming > 0 {
                        // Prime the step for triggering on the next microtimed offset tick.
                        self.this_step = Some(MicrotimingStep {
                            // A positive microtiming is an offset from
                            // the start of the step boundary.
                            //
                            // SAFETY: since we've checked >0 above, we can assume
                            //  it's castable from signed into a usize.
                            tick: step.microtiming as usize,
                            step: timing.get_step(),
                        })
                    }
                }
            }

            // Now, we also need to account for **negative** microtiming on the
            // next step, which plays it **earlier** then the upcoming step boundary.
            //
            // Checking this on the prior step boundary is fairly cheap compared
            // to if we did it every tick, and don't really loose us realtime
            // editing responsivness to trig changes, so we do it here.
            //
            // NOTE: get_next_step() wraps to 0 at the maximum step boundary,
            //  allowing us to always check the "next" step.
            if let Some(step) = &mut self.steps[timing.get_next_step()] {
                // If the microtiming is smaller then 0, then
                // we need to calculate when to trigger it.
                if step.microtiming < 0 {
                    // Prime the step for triggering on the specified tick.
                    self.next_step = Some(MicrotimingStep {
                        // Since it's relative to the NEXT step, we need to
                        // subtract the microtiming from the step tick maximum.
                        //
                        // SAFETY: Since we've checked <0 above we know it's
                        //  negative, so inverting and subtracking from steps
                        //  will be positive.
                        tick: MAX_TICK - (-step.microtiming as usize),
                        step: timing.get_step(),
                    })
                }
            }
        } else {
            // If there was a microtiming step coming up,
            // we need to check if it's time to trigger it.
            //
            // NOTE: We calculate and check when a microtiming
            //  trigger is coming up in the code above via the
            //  step boundary to avoid needing to repeatedly
            //  calculate it every tick, which would be somewhat
            //  expensive to process.

            // This checks for if the CURRENT step is +microtimed.
            if let Some(this_step) = &self.this_step
                && this_step.tick == timing.get_tick()
            {
                // Queue the trigger to be triggered this step.
                to_trigger = Some(this_step.step);
            }

            // This checks for if the NEXT step is -microtimed.
            if let Some(next_step) = &self.next_step
                && next_step.tick == timing.get_tick()
            {
                // Queue the trigger to be triggered this step.
                to_trigger = Some(next_step.step);
            }
        }

        // If we're triggering a trigger this tick, either because we're on a step
        // boundary or because a microtiming condition is met, then we do it here.
        //
        // This prevents repeating code across both step trigger conditions.
        if let Some(step_index) = to_trigger {
            // Retrieve the step.
            //
            // SAFETY: This may be None if the user removed a step between when
            //  a microtiming trigger was calculated and actually executed.
            if let Some(step) = &mut self.steps[step_index] {
                // Attempt to trigger the trigger.
                //
                // This evaluates if the conditions for this trigger are met.
                //
                // This takes in some parameters about the last trig eval in
                // the track, and the neighboring tracks's last trig eval to
                // drive some of the possible condition logic.
                let eval = step.evaluate(
                    self.last_trig_eval,
                    last_neighbour_trig_eval,
                    pattern_change_queued,
                    timing.get_repeats(),
                );

                // Previous and NotPrevious conditions are ignored for trigger conditions.
                //
                // If the most recently evaluated trigger is a Previous or NotPrevious, then
                // effectively the trigger BEFORE that one is evaluated for the condition.
                if step.condition != TriggerCondition::Previous
                    && step.condition != TriggerCondition::NotPrevious
                {
                    self.last_trig_eval = eval;
                }

                if eval {
                    // TODO: may want to dispatch some kind of event.
                }
            }
        }

        // Some conditional logic depends on what the track before it
        // did, this is used to help track those conditional events.
        if to_trigger.is_some() {
            TrackTickResult::TriggerEvaluated(self.last_trig_eval)
        } else {
            TrackTickResult::Ticked
        }
    }
}

/// Specifies a value for a parameter that's
/// changed by a given step triggering.
pub struct ParameterLock {
    /// The parameter that the lock exists for.
    parameter: ParameterID,
}

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

/// A trigger placed on a step in a track.
pub struct Trigger<const MAX_TICK: usize> {
    /// Specifies an offset in divisions of the BMP (calculated by BMP/MAX_TICK)
    /// that will play this trigger earlier or later then the step boundary.
    ///
    /// Microtiming can be ±(MAX_TICK/2), this is to prevent the next step
    /// from acidentally overlapping with the previous step.
    microtiming: i8,

    /// Condition that decide if this trigger
    /// is actually triggered when hit or not.
    condition: TriggerCondition,

    /// Per-step parameters changes programmed with triggers.
    ///
    /// These change parameters related to the track sequencing,
    /// instruments, etc. in response to this trigger being hit.
    parameter: bool,
}

impl<const MAX_TICK: usize> Trigger<MAX_TICK> {
    /// Attempt to trigger the trigger.
    ///
    /// This returns if the trigger should actually be
    /// triggered, depending on trigger conditions
    /// such as probability.
    pub fn evaluate(
        &mut self,
        last_trig_eval: bool,
        last_neighbour_trig_eval: bool,
        pattern_change_queued: bool,
        repeats: usize,
    ) -> bool {
        // TODO: trigger conditions

        // Evaluate if the condition for the trigger is met.
        self.condition.evaluate(
            last_trig_eval,
            last_neighbour_trig_eval,
            pattern_change_queued,
            repeats,
        )
    }

    /// Sets the microtiming for the trigger.
    ///
    /// This allows a trigger to be triggered earlier or later the the step boundary
    /// it's actually placed on, thanks to fractional BMP-derrived ticks.
    pub fn set_microtiming(&mut self, microtiming: i8) {
        let abs = if microtiming < 0 {
            -microtiming
        } else {
            microtiming
        };

        // Check that the microtiming doesn't
        // exceed the half-step boundary.
        if abs as usize > MAX_TICK / 2 {
            // TODO: return an error, or cap?
            return;
        }

        self.microtiming = microtiming;
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
