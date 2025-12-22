# Catalina

Catalina is an audio and music processing ecosystem designed for embedded ARM Cortex devices. This includes the core DSP and instrument framework, example instruments, and a board support framework for the Catalina hardware modules, carriers and kits.

## Features

Firstly, there are some core features that enable shadowing of the other `catalina-` crates from the mono-repo into this base crate:

 - `engine` - The core DSP and instrument framework build building audio processing chains, sequencers, synths, etc. Enabled by default.
 - `instruments` - Basic instruments to serve as the base or examples for building custom instruments.
 - `bsp` - The board support packages for Catalina hardware. Requires other board-specific features such as `mini-v1` to enable board support for a specific hardware platform.

> These become available as shadowed imports via `catalina::<name>`.

One of these base features needs to be enabled for the `catalina` crate to provide anything.

And then there are several fairly standard features that are common across embedded Rust packages:

  - `std` - Disabled by default, enables `std` library support instead of no_std `core`.
  - `alloc` - Enables use of datastructures that require allocation. When disabled, `heapless` datastructures are used instead.
  - `defmt` or `log` to support either as the logging framework for instrumentation. Also adds the `defmt::Format` macro to structs for logging.
  - `serde` - Enables serde serializing and deserializing on structs and enums.
