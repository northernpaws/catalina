#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

// Needs to be first module in list.
mod fmt;

pub mod prelude;

pub mod core;
pub mod music;

pub mod audio;
pub mod instrument;
pub mod sequence;
