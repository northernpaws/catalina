use crate::Pattern;

/// A project encapsulates a set of patterns and a resource pool.
pub struct Project<
    const MAX_PATTERNS: usize,
    const MAX_TRACKS: usize,
    const MAX_STEPS: usize,
    const MAX_TICK: usize,
> {
    /// Patterns contained within a project.
    patterns: [Option<Pattern<MAX_TRACKS, MAX_STEPS, MAX_TICK>>; MAX_PATTERNS],

    current_pattern: Option<usize>,
    next_pattern: Option<usize>,
}

/// Sequencer stepping methods.
///
/// See [Steppable] for explaination.
impl<
    const MAX_PATTERNS: usize,
    const MAX_TRACKS: usize,
    const MAX_STEPS: usize,
    const MAX_TICK: usize,
> Project<MAX_PATTERNS, MAX_TRACKS, MAX_STEPS, MAX_TICK>
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
