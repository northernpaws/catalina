use rythm_engine::sequence;

// To easily defined the limits of our step sequencer
// we can create a few constants and type aliases.
const PATTERNS: usize = 8;
const TRACKS: usize = 4;
const STEPS: usize = 16;
type Pattern = sequence::pattern::Pattern<TRACKS, STEPS>;
type Project = sequence::Project<PATTERNS, TRACKS, STEPS>;

fn main() {
    println!("Hello, world!");

    // Initialize a new project using the pattern,
    // track and step limits we've configured.
    let mut project = Project::new();

    let mut pattern_1 = Pattern::new();
}
