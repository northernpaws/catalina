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
    pub fn tick(&mut self) {
        // Loop over and tick the patterns.
        for pattern in &mut self.patterns {
            let Some(pattern) = pattern else {
                continue;
            };

            pattern.tick();
        }
    }
}
