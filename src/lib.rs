#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

/// Re-export the engine crate under the root crate.
pub mod engine {
    pub use rythm_engine::*;
}

/// Re-export the BSP crate under the root crate.
pub mod bsp {
    pub use rythm_bsp::*;
}

/// Re-export the BSP crate under the root crate.
pub mod instruments {
    pub use rythm_instruments::*;
}
