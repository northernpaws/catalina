#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

/// Re-export the engine crate under the root crate.
#[cfg(feature = "engine")]
#[doc(inline)]
pub use catalina_engine as engine;

/// Re-export the BSP crate under the root crate.
#[cfg(feature = "bsp")]
#[doc(inline)]
pub use catalina_bsp as bsp;

/// Re-export the BSP crate under the root crate.
#[cfg(feature = "instruments")]
#[doc(inline)]
pub use catalina_instruments as instruments;
