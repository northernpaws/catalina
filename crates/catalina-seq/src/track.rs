use catalina_engine::music::note::{CFour, Note};

use crate::{Events, ParameterID, PatternTiming, Trigger, TriggerCondition, TriggerEvent};

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

/// Events emitted from a track tick.
///
/// These are used to plumb other systems into the sequencer.
#[derive(Clone, PartialEq)]
pub enum TrackEvent {
    /// A trigger was hit and caused events, and what step it was on.
    Trigger(usize, Events<TriggerEvent>),

    /// If the track has it's own timing, this
    /// indicates the start of a track sequence.
    TrackStart,
    /// If the track has it's own timing, this
    /// indicates the end of a track sequence.
    TrackEnd,
}

/// We know how many track events can be emitted at once, so this defines that.
///
/// A tick can have:
/// - either a start/end event, or none
/// - a single trigger event
pub type TrackEvents = Events<TrackEvent, 2>;

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

    /// Pre-initialized container for tick events.
    events: TrackEvents,
    /// Pre-initialized container for tick trigger events.
    trigger_events: Events<TriggerEvent>,
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
    #[must_use]
    pub fn has_track_timing(&self) -> bool {
        self.timing.is_some()
    }

    /// Returns what the most recently evaluated trigger resolved to.
    #[must_use]
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
    #[must_use = "track events need to be processed"]
    pub fn tick(
        &mut self,
        pattern_timing: &PatternTiming<MAX_TICK>,
        last_neighbour_trig_eval: bool,
        pattern_change_queued: bool,
    ) -> TrackEvents {
        // Reset the event container.
        self.events.reset();

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

        // If we're on the last step and last tick of
        // the step, then append a track end event.
        if timing.is_last_step() && timing.is_last_tick() {
            self.events.append(TrackEvent::TrackEnd);
        }

        // The step to trigger, if there is one.
        let mut to_trigger: Option<usize> = None;

        // If this tick advanced the step, we need to check if we're triggering a step.
        if timing.get_did_step() {
            // If this is a first or last step, then emit the corrosponding events.
            if timing.is_first_step() {
                self.events.append(TrackEvent::TrackStart);
            }

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
            // NOTE: negative microtiming on the FIRST step of a track won't be triggered on first
            // play, it will only be considered on the LAST step of the sequence as the next step.
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

                // If the trig eval was true, then we handle trig events.
                if eval {
                    // Populate the events for the evalauted trig.
                    step.reset_and_populate_events(&mut self.trigger_events);

                    // Append the trig events to the track events list.
                    self.events
                        .append(TrackEvent::Trigger(step_index, self.trigger_events.clone()));
                }
            }
        }

        self.events.clone()
    }
}
