#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal as hal;
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::gpio::{AnyPin, Level, Output, Pin};
use hal::println;

#[embassy_executor::task(pool_size = 4)]
async fn blink(pin: AnyPin, interval_ms: u64) {
    let mut led = Output::new(pin, Level::Low, Default::default());

    loop {
        led.set_high();
        Timer::after_millis(interval_ms).await;
        led.set_low();
        Timer::after_millis(interval_ms).await;
    }
}

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let p = hal::init(config);

    println!("CHIP signature => {}", hal::signature::chip_id().name());
    Timer::after_millis(10).await;
    println!("Clocks {:?}", hal::rcc::clocks());

    let mut led = Output::new(p.PD3, Level::High, Default::default());
    loop {
        println!("tick");

        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = hal::println!("\n\n\n{}", info);

    loop {}
}
