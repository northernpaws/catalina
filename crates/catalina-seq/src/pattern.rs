use crate::{STEP_SUBSTEPS, PatternTiming, Track, TrackEvents};

/// Concrete type for the name of patterns.
#[cfg(not(feature = "std"))]
pub type PatternName = heapless::String<64>;
#[cfg(feature = "std")]
pub type PatternName = String;

pub enum PatternTickResult {
    /// Indicates that nothing of note happened.
    Tick,
    /// Indicates that this tick is the start of the pattern.
    PatternStart,
    /// Indicates that this tick has reached the end of the pattern.
    PatternEnd,
}

/// An event raised from a pattern.
///
/// This encapsulates both pattern-specific events,
/// and events from tracks within that pattern.
pub enum PatternEvent {
    /// Indicates that a track raised some
    /// event(s), and what track it was.
    Track(u8, TrackEvents),

    /// Indicates that this tick is the start of the pattern.
    PatternStart,
    /// Indicates that this tick has reached the end of the pattern.
    PatternEnd,
}

/// A pattern sequences a set of tracks.
pub struct Pattern<const MAX_TRACKS: usize, const MAX_STEPS: usize> {
    /// Display name of the pattern.
    name: PatternName,

    /// Tracks for the pattern.
    tracks: [Track<MAX_STEPS>; MAX_TRACKS],

    /// Configures the timing for the pattern.
    timing: PatternTiming,
}

/// Generic platform methods.
impl<const MAX_TRACKS: usize, const MAX_STEPS: usize> Pattern<MAX_TRACKS, MAX_STEPS> {
    /// Sets the name of the pattern.
    pub fn set_name(&mut self, name: PatternName) {
        self.name = name;
    }

    /// Resets the pattern's timing and tracks for a fresh play.
    pub fn reset(&mut self) {
        // Loop through and reset the tracks.
        for track in &mut self.tracks {
            track.reset();
        }
    }

    /// Tick the pattern.
    #[must_use = "pattern events need to be processed"]
    pub fn tick(&mut self, pattern_change_queued: bool) -> PatternTickResult {
        // Tick the pattern-wide timing.
        self.timing.advance();

        // Track the last track that was ticked and evaluated.
        //
        // This is used for neighboring track conditional logic on triggers.
        let mut last_track: Option<&mut Track<MAX_STEPS>> = None;

        // Loop through and tick the tracks.
        for track in &mut self.tracks {
            // If there was a neighboring prior track,
            // get the last trigger eval state.
            //
            // This is used by some of the conditional trigger logic.
            let last_track_eval = match last_track {
                Some(track) => track.get_last_trig_eval(),
                None => false,
            };

            // Tick the track.
            track.tick(&self.timing, last_track_eval, pattern_change_queued);

            last_track = Some(track);
        }

        if self.timing.get_tick() == 0 {
            PatternTickResult::PatternStart
        } else if self.timing.get_tick() == STEP_SUBSTEPS - 1 {
            PatternTickResult::PatternEnd
        } else {
            PatternTickResult::Tick
        }
    }
}
