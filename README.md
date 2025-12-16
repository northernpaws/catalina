# Rythm

> ⚠️ This is still a large work-in-progress! A semi-working version (hardware and software) is estimated to be ready around March 2026.

Rythm is an embedded-first Rust ecosystem for building music and audio processing devices, with a primary focus on ARM Cortex MCUs as the end-target.

The Rythm ecosystem is comprised of:
 * A Rust crate providing a DSP and audio engine designed for embedded devices (heapless by default, small footprint).
   * Various composable interfaces for quickly prototyping new instruments.
   * Drivers for common audio codecs, SDRAM, etc.
   * Windows, MacOS, and Linux support for developing audio chains before flashing to hardware.
 * Two base hardware modules designed for the engine:
   * **Rythm Mini** - small footprint with audio processing.
   * **Rythm Card** - larger footprint with audio processing, CV and sync.
 * Several module "host" boards:
   * Breadboard-compatible carrier with headers.
   * Protoboard-style carrier with an integrated module.

## TODO

  * [ ] Finish Mini module revision 1.
  * [ ] Finish Card module revision 2.
  * [ ] Stable minor release of `engine`, with audio and synth basic building blocks.

# Engine

`rythm-engine` is an embedded-first DSP and audio engine crate that can be quickly integrated into embedded projects as a dependency, or used standalone with the std library for audio application.

`rythm-engine` is designed to be as modular as possible, so i.e. if you just need a couple oscillators, you can use the oscillator module without needed to depend on any other parts of the engine. This composability is designed to make building audio effects and instruments as intuitive as possible.

The engine supports both the `core` embedded and `std` runtimes, including support for common embedded loggers (`defmt`, `log`), embedded and std library `tracing`, `std` and `alloc` feature sets from underlying libraries, and more via crate feature flags.

Our goal is to make the usage of the engine as seamless as possible across both embedded and non-embedded platforms, making prototyping designs on PC and in simulators, and then later transferring them to embedded device runtimes as easy as possible.

# Modules

> ⚠️ The modules and their associated boards are a large work-in-progress! Several designs are only half-complete or still in the planning stages.

The modules provides a set of small STM32-based boards that can be embedded into custom PCB designs, or used with a set of associated carrier boards. These carrier boards include basic breadboard header breakouts, some module-integrated protoboard designs, and some options for carriers with a selection of [Molex PicoBlade Connectors](https://www.molex.com/en-us/products/connectors/wire-to-board-connectors/picoblade-connectors) for modular plug-and-play prototyping.

See the kits section below for more types of carrier boards that are designed as self-assembled soldering kits that let you quickly get started creating guitar/effects pedals, and small synths and sequencers using the Rythm toolkit.

> ⚠️ The ecosystem's modules are **NOT** able to be used standalone, they only have fine-pitch [Hirose DF40C board-to-board connectors](https://www.hirose.com/en/product/document?clcode=&productname=&series=DF40&documenttype=Catalog&lang=en&documentid=en_DF40_CAT) - a carrier board is required to interface with them!
>
> See our upcoming **Mini Discovery** and **Mini Proto** carriers below for examples, or one of our upcoming soldering kits.

## Mini

| ![Layout](assets/images/mini/layout.png)  | ![Front](assets/images/mini/front.png) | ![Back](assets/images/mini/back.png) |
|:---:|:---:|:---:|

**[Schematic Viewer](https://kicanvas.org/?github=https%3A%2F%2Fgithub.com%2Fnorthernpaws%2Frythm%2Ftree%2Fmain%2Fhardware%2Fboards%2Fmini%2Fv0.2)**

  * 480MHz ARM Cortex-M7 processor (STM32H750XBHx)
  * 32MB of 16-bit SDRAM for audio buffers
  * 16MB QSPI Flash for programs and settings
  * 24-bit Stereo Audio Codec with internal mixer
  * SDMMC 4-bit interface
  * 1 Hardware MIDI Endpoint (RX and TX)

The Mini is a 20mm x 40mm module, designed to have a similar footprint to an Arduino Nano to make it easy to fit in a variaty of small audio and music devices.

The mini is ideal for applications that don't need a large amount of audio RAM and need to fit in a small footprint - such as a guitar/effects pedal, Altoids tin, or 2HP Eurorack modules similar to [Erica Synths' Pico series](https://www.ericasynths.lv/shop/eurorack-modules/by-series/pico-series/).

### TODO

  * ~~[ ] In Revision 3, see if we can switch to SDMMC2 to enable both QSPI 1 and 2 simultaneously to expose one over the connector.~~
  * [x] *Or connect PG6 (`QUADSPI_BK1_NCS`) to connector, connect QUADSPI1 to connector, and provide a bootloader toggle that allows the carrier to indicate it should use carrier select pin.*

### Notes

 * Mini was originally designed with the BGA201 package, but the pitch was too fine for economic fabrication capabilities. Upgraded to BGA265 in revision 2 to fix routing constraints. 
 * The Mini's original design concept was using 16mx32 SDRAM, with 32-bit SDRAM being ideal for handling encoding of stereo 16-bit audio samples (one 32-bit byte per stereo sample). Ultimately there was not enough room for routing x32 SDRAM with STM32's non-standard memory interface ball out, so a compromise to use 16-bit instead was made to keep the mini at in our 20mm wide constraint. 

## Card

> ⚠️ The card module is only in the planning stage.

Everything from the Mini:

  * 480MHz ARM Cortex-M7 processor
  * QSPI Flash
  * Stereo Audio Codec
  * SDMMC 4-bit interface

Plus:

  * 64MB of 32-bit SDRAM (4 times the amount on the Mini!)
  * Eurorack CV inputs (amount TBD)
  * Eurorack CV outputs (amount TBD)
  * Eurorack Sync In/Out (amount TBD)

The Card module is larger then the Mini, about the size of a credit card. It also features a different board-to-board connector from the Mini, supporting much higher board-to-board distances allowing it to seat over top of higher components (i.e. 3.5mm audio sockets) then the Mini can.

The Card is designed for audio applications that need significantly more memory, and benefit from having 32-bit memory (such as faster access for stereo 16-bit samples), like hardware samplers. The Card also features an onboard bank of dynamic CV sink, input, and output drivers that allow it to function as a Eurorack module off-the-shelf, or with synths and sequencers that have control voltage sockets.

### TODO

  * [ ] Ensure we can route a full 16mx32 SDRAM in the Card footprint.
  * [ ] Research the best CV circuits we can use to support a large range of biasing and voltages (i.e. -12v to +12v).
  * [ ] After SDRAM routing, can we break the parallel interface out to a B2B connector? Could be useful for driving displays, or memory expension with additional chips.
  * [ ] Aim to break out one of the QSPI interfaces so carriers can add additional flash. One ideas is for Card carriers to have their own carrier-specific flash that the module can load when attached, so programs don't need to be reflashed if sharing a module across carriers.

# Carriers

Carriers are breakout boards for the core modules that make them easy to prototype and work with, without needing to make a custom board for their dense board-to-board connectors. 

Some of the planned carriers are:

  * "Discovery" line that breaks the module's IO's out to standard breadboard compatible pin headers. Includes a USB-C port for device and host mode, contains an ARM Cortex programming header keyed for an IDC cable, and possibly an SD card. Includes the required electrolytic capacitors for the integrated headphone driver on the modules.
  * "Proto" line that has a protoboard layout with a socket for the module in the center, making it easy to not only prototype more stable circuits directly with the module, but also to unsocket and swap the module between multiple proto boards so you don't need to buy several for different projects.

And a couple other ideas that need to be planned out:
  * A carrier that breaks out the audio and MIDI lines as 3.5mm sockets, and IO on [Molex PicoBlade connectors](https://www.google.com/search?client=safari&rls=en&q=picoblade+connector&ie=UTF-8&oe=UTF-8) for rapid modular prototyping (maybe use the "Discovery" name for this one instead?). 

# Kits

Rythm's hardware ecosystem will also include several soldering kits that are currently in various stages of development:
 * **Rythm Mini**
   * **Guitar/Effects Pedal Kit** - PCB with several connectors for knobs and foot switches, includes some knob and footswitch options, a Mini module, and 
   * **Mini Synth Kit** - A carrier similar to the Korg Volca or Behringer Pro series with a capacitive keyboard, several knobs and buttons, and 3.5mm MIDI minijack. Comes with a few printed faceplates and a 3D printed case. Carrier sockets could be solderable in kit form.
   * **Mini Step Sequencer Kit** - A small step or drum sequencer design, similar to the Roland AIRA Compact or Elektron Model lineup. Buttons and knobs solderable for kit form.
   * **Mini Groovebox Kit** - Something similar to an Elektron Model or Novation Circuit series device. A 4x8 velocity pad grid (velostat?), several knobs, and some menu buttons.
 * **Rythm Card**
   * **Eurorack Module Kit** - A 4-8 HP (to be determined) Eurorack carrier sporting all the Card CV inputs and outputs as standard 3.5mm sockets, USB, 3.5mm MIDI Minijack and an SD card.
   * **Pro Synth Kit** - Similar to the Midi Synth Kit, but also sporting a display, more control, and 3.5mm CV sockets.
   * **Pro Step Sequencer Kit** - Something similar to an Elektron Digitakt, SEQTRAK, or Korg SQ-1 with 16-step buttons, a display, and some knobs.

Each of these kits will come with a board support package via a feature flag in the Rust crate. These board support packages will come with all the scaffolding necessary to quickly build a project around the boards, as well as several example projects. 

## TODO
  * FUSB303BTMX on discovery for USB-C support
  * Amphenol_12401610E4-2A or HRO_TYPE-C-31-M-12?