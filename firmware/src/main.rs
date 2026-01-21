#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::spi::{Config, Spi};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

const CMD_READ: u8 = 0x03;
const ID_REV_REG: u16 = 0x50; // Offset for Chip ID and Revision

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = Config::default();
    config.frequency = 80_000_000;

    let mut spi = Spi::new(
        p.SPI0, p.PIN_2, // SCK
        p.PIN_3, // MOSI
        p.PIN_0, // MISO
        p.DMA_CH0, p.DMA_CH1, config,
    );
    //chipselect
    let mut cs = Output::new(p.PIN_1, Level::High);
    //reset
    let mut rst = Output::new(p.PIN_9, Level::High);

    defmt::info!("Starting");

    //start reset
    rst.set_low();
    Timer::after(Duration::from_millis(50)).await;
    rst.set_high();
    Timer::after(Duration::from_millis(100)).await;

    loop {
        let mut tx_buf = [0u8; 7];
        let mut rx_buf = [0u8; 7];

        tx_buf[0] = CMD_READ;
        //split bytes
        tx_buf[1] = (ID_REV_REG >> 8) as u8;
        tx_buf[2] = (ID_REV_REG & 0xFF) as u8;

        //start transc
        cs.set_low();
        let transfer_result = spi.transfer(&mut rx_buf, &tx_buf).await;
        cs.set_high();

        match transfer_result {
            Ok(_) => {
                //last 4 bytes only usefull
                let id_word = u32::from_le_bytes([rx_buf[3], rx_buf[4], rx_buf[5], rx_buf[6]]);

                defmt::info!("Read ID: {:#010x}", id_word);
            }
            Err(_) => defmt::error!("SPI Error"),
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}
