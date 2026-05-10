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

mod timing;
pub use timing::*;

mod parameter;
pub use parameter::*;

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
