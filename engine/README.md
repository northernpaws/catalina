# `rythm-engine`

The engine crate is the core of the entire Rythm ecosystem. It provides the concrete music theory types, audio processing framework, transport and sequencing logic, and instrument systems that create the composable Rythm framework. 

## Modules

 * `theory` - Music theory concepts as concrete types, such as enums for common octaves and pitches.
 * `sequence` - Elektron-style step sequencing framework.

## Generics

Parts of the engine that rely on known compile-time constants for maximum allocation values, maximum slice sizes, etc. are heavily paramatarized with generics. This allows devices with different requirements, such as a synth that supports 4 voice polyphony vs a seqeuncer that supports 8, to be implemented with the same framework without needing a large array of custom types or trait implementations on the user's end.
