use heapless::Vec;

use crate::sequence::pattern::Pattern;

pub mod pattern;

pub enum PatternError {
    PatternsFull,
}

/// A project provide a collection of patterns.
pub struct Project<const PATTERNS: usize, const TRACKS: usize, const STEPS: usize> {
    /// The list of patterns in the track.
    patterns: Vec<Option<Pattern<TRACKS, STEPS>>, PATTERNS>,
}

impl<const PATTERNS: usize, const TRACKS: usize, const STEPS: usize>
    Project<PATTERNS, TRACKS, STEPS>
{
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    /// Retrieves a reference to a pattern in the track.
    pub fn get_pattern(&mut self, index: usize) -> Option<&Pattern<TRACKS, STEPS>> {
        if index > self.patterns.len() {
            return None;
        }

        let Some(pattern) = &self.patterns[index] else {
            return None;
        };

        Some(pattern)
    }

    /// Retrieves a reference to a pattern in the track.
    pub fn get_pattern_mut(&mut self, index: usize) -> Option<&mut Pattern<TRACKS, STEPS>> {
        if index > self.patterns.len() {
            return None;
        }

        let Some(pattern) = &mut self.patterns[index] else {
            return None;
        };

        Some(pattern)
    }
}
