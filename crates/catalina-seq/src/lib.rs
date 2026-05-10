mod timing;
pub use timing::*;

mod track;
pub use track::*;

mod pattern;
pub use pattern::*;

mod project;
pub use project::*;

/// The root type of the sequencer that initiates and
/// manages the rest of the sequencer components.
pub struct Sequencer<
    const MAX_PATTERNS: usize,
    const MAX_TRACKS: usize,
    const MAX_STEPS: usize,
    const MAX_TICK: usize,
> {
    /// The currently loaded project.
    project: Project<MAX_PATTERNS, MAX_TRACKS, MAX_STEPS, MAX_TICK>,
}

/// Sequencer stepping methods.
///
/// See [Steppable] for explaination.
impl<
    const MAX_PATTERNS: usize,
    const MAX_TRACKS: usize,
    const MAX_STEPS: usize,
    const MAX_TICK: usize,
> Sequencer<MAX_PATTERNS, MAX_TRACKS, MAX_STEPS, MAX_TICK>
{
    fn tick(&mut self) {
        self.project.tick();
    }
}
