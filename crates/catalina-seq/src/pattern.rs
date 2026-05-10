use crate::{PatternTiming, Track};

/// Concrete type for the name of patterns.
#[cfg(not(feature = "std"))]
pub type PatternName = heapless::String<64>;
#[cfg(feature = "std")]
pub type PatternName = String;

/// A pattern sequences a set of tracks.
pub struct Pattern<const MAX_TRACKS: usize, const MAX_STEPS: usize, const MAX_TICK: usize> {
    /// Display name of the pattern.
    name: PatternName,

    /// Tracks for the pattern.
    tracks: [Track<MAX_STEPS, MAX_TICK>; MAX_TRACKS],

    /// Configures the timing for the pattern.
    timing: PatternTiming<MAX_TICK>,
}

/// Generic platform methods.
impl<const MAX_TRACKS: usize, const MAX_STEPS: usize, const MAX_TICK: usize>
    Pattern<MAX_TRACKS, MAX_STEPS, MAX_TICK>
{
    /// Sets the name of the pattern.
    pub fn set_name(&mut self, name: PatternName) {
        self.name = name;
    }

    /// Tick the pattern.
    pub fn tick(&mut self) {
        // Tick the pattern-wide timing.
        self.timing.tick();

        // Loop through and tick the tracks.
        for track in &mut self.tracks {
            track.tick(&self.timing);
        }
    }
}
