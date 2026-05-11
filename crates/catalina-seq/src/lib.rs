//! The catalina-seq crate provides an Elektron-like step sequencer.
//!
//! This crate provides the mechanisms by which to create a step
//! sequencer with multiple patterns, tracks, etc. and to trigger
//! events and instruments/machines from those steps.
//!
//! This crate does not provide the actual means of playing back
//! instruments or samples from the steps, instead it's assumed
//! you'll connect into the sequencing APIs to trigger machines
//! as required from the main application loop in response to
//! triggers.

mod event;
pub use event::*;

mod timing;
pub use timing::*;

mod parameter;
pub use parameter::*;

mod trigger;
pub use trigger::*;

mod track;
pub use track::*;

mod pattern;
pub use pattern::*;

/// The root type of the sequencer that initiates and
/// manages the rest of the sequencer components.
pub struct Sequencer<const MAX_PATTERNS: usize, const MAX_TRACKS: usize, const MAX_STEPS: usize> {
    /// Sequence-wide BPM-based timing.
    timing: SequencerTiming,

    /// Patterns contained within a project.
    patterns: [Option<Pattern<MAX_TRACKS, MAX_STEPS>>; MAX_PATTERNS],

    current_pattern: Option<usize>,
    next_pattern: Option<usize>,
}

/// Sequencer stepping methods.
impl<const MAX_PATTERNS: usize, const MAX_TRACKS: usize, const MAX_STEPS: usize>
    Sequencer<MAX_PATTERNS, MAX_TRACKS, MAX_STEPS>
{
    /// Internal method that instantly change the pattern,
    /// usually called from pattern queuing checks.
    ///
    /// This makes sure that the pattern indexes are updated
    /// correctly to reflect the new pattern, checks that
    /// the pattern is valid, and resets it to start the play.
    fn change_pattern_now(&mut self, pattern_index: usize) {
        self.current_pattern = Some(pattern_index);

        // Reset if there was a queued pattern to play next.
        self.next_pattern = None;

        // Check that the pattern is valid, and reset it if so.
        let Some(pattern) = &mut self.patterns[pattern_index] else {
            // Reset the pattern index since we don't have a valid one.
            self.current_pattern = None;

            return;
        };

        // Reset the pattern timing and counters for a fresh play.
        //
        // This resets all the timing parameters used for playback
        // and trigger conditions to ensure they start properly.
        pattern.reset();
    }

    #[must_use = "project events need to be processed"]
    pub fn tick(&mut self) {
        // Tick the global sequencer timing.
        self.timing.tick();

        // Shortcut if there is no pattern selected somehow.
        let Some(current_pattern) = self.current_pattern else {
            // If there is no current pattern, but a next pattern
            // has been queued, then we start it next tick.
            if let Some(next_pattern) = self.next_pattern {
                self.change_pattern_now(next_pattern);
            }

            return;
        };

        // Shortcut if there is no valid pattern in the specified slot.
        let Some(pattern) = &mut self.patterns[current_pattern] else {
            // Reset the pattern index since we don't have a valid one.
            self.current_pattern = None;

            return;
        };

        // Check if a pattern change has been queued.
        let pattern_change_queued = self.next_pattern.is_some();

        // Tick the pattern and check if anything of note happened.
        match pattern.tick(pattern_change_queued) {
            crate::PatternTickResult::Tick => {}
            crate::PatternTickResult::PatternStart => {}
            crate::PatternTickResult::PatternEnd => {
                // If we've reached the end of the pattern and there's a
                // pattern change queued, then switch to the next pattern.
                if let Some(next_pattern) = self.next_pattern {
                    self.change_pattern_now(next_pattern);
                }
            }
        }
    }
}
