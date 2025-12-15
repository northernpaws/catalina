# Rythm

> ⚠️ This is still a large work-in-progress! A semi-working version (hardware and software) is estimated to be ready around March 2026.

Rythm is an embedded-first Rust ecosystem for buiding music and audio processing devices.

The Rythm ecosystem is comprised of:
 * A Rust crate providing a DSP and audio engine designed for embedded devices (heapless by default, small footprint).
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

## Mini

| ![Layout](assets/images/mini/layout.png)  | ![Front](assets/images/mini/front.png) | ![Back](assets/images/mini/back.png) |
|:---:|:---:|:---:|

**[Schematic Viewer](https://kicanvas.org/?github=https%3A%2F%2Fgithub.com%2Fnorthernpaws%2Frythm%2Ftree%2Fmain%2Fhardware%2Fboards%2Fmini%2Fv0.2)**

  * 480MHz ARM Cortex-M processor (STM32H750XBHx)
  * 64MB of 16-bit SDRAM for audio buffers
  * 16MB QSPI Flash for programs and settings
  * 24-bit Stereo Audio Codec with internal mixer
  * SDMMC 4-bit interface



### Notes

 * Mini was originally designed with the BGA201 package, but the pitch was too fine for economic fabrication capabilities. Upgraded to BGA265 in revision 2 to fix routing constraints. 

## Card

Everything from the Mini:

  * 480MHz ARM Cortex-M processor
  * 64MB of SDRAM
  * QSPI Flash
  * Stereo Audio Codec
  * SDMMC 4-bit interface
  * Stereo Audio Codec (2 in, 2 out)

Plus:

  * 1 Hardware MIDI Endpoint (RX and TX)
  * Eurorack x CV inputs
  * Eurorack x CV outputs
  * Eurorack Sync In/Out

# Kits

Rythm's hardware ecosystem will also include several soldering kits that are currently in various stages of development:
 * **Rythm Mini**
   * Guitar/Effects pedal kit.
   * Mini synth kit.
   * Mini step sequencer kit.
 * **Rythm Card**
   * Eurorack module kit.
   * Pro synth kit.
   * Pro step sequencer kit.

Each of these kits will come with a board support package via a feature flag in the Rust crate. These board support packages will come with all the scaffolding necessary to quickly build a project around the boards, as well as several example projects. 

## TODO
  * FUSB303BTMX on discovery for USB-C support
  * Amphenol_12401610E4-2A or HRO_TYPE-C-31-M-12?