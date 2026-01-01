#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

/// The engine crate provides the primary DSP audio processing
/// framework, transport and sequencing logic, and instrument
/// system that create the composable audio chains framework.
///
/// Re-exports the `catalina-engine` crate.
#[cfg(feature = "engine")]
pub mod engine {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use catalina_engine::*;
}

/// Board support packages for Catalina native hardware components.
///
/// Re-exports the `catalina-bsp` crate.
#[cfg(feature = "bsp")]
#[doc(inline)]
pub mod bsp {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use catalina_bsp::*;
}

/// Basic instruments to quickly get started with on Catalina hardware,
/// also serves as references and samples for the instrument framework.
///
/// Re-exports the `catalina-instruments` crate.
#[cfg(feature = "instruments")]
#[doc(inline)]
pub mod instruments {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use catalina_instruments::*;
}
