#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod usb_serial;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut sp = usb_serial::init(&spawner, p.USB);

    // NOTE: info!() only goes to a debug probe, sp.print() goes out USB
    info!("Hello World!");
    sp.print("Hello World!\n").await;

    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        info!("high");
        sp.print("high\n").await;
        led.set_high();
        Timer::after_millis(1000).await;

        info!("low");
        sp.print("low\n").await;
        led.set_low();
        Timer::after_millis(1000).await;
    }
}
