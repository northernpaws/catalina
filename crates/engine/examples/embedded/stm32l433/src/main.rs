#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::i2s::{Config, Format, I2S};
use embassy_stm32::time::Hertz;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the microscontroller (MCU).
    let p = embassy_stm32::init(Default::default());

    // Configure the I2S bus on the MCU for
    // playing back sound over a codec.
    let mut dma_buffer = [0u16; 2400];
    let mut i2s_config = Config::default();
    i2s_config.format = Format::Data16Channel32;
    i2s_config.master_clock = false;
    let mut i2s = I2S::new_txonly_nomck(
        p.SPI3,
        p.PB5,  // sd
        p.PA15, // ws
        p.PB3,  // ck
        p.DMA1_CH7,
        &mut dma_buffer,
        i2s_config,
    );
    i2s.start();

    loop {
        i2s.write(&wavetable).await.ok();
    }
}
