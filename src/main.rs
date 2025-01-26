#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use ch32_hal as hal;
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::gpio::{Level, Output};
use hal::println;

#[embassy_executor::task]
async fn blink(mut led: Output<'static>, interval_ms: u64) {
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
    let config = hal::Config {
        rcc: hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSE,
        ..Default::default()
    };
    let p = hal::init(config);
    Timer::after_millis(1000).await;

    println!("CHIP signature => {}", hal::signature::chip_id().name());
    Timer::after_millis(10).await;
    println!("Clocks {:?}", hal::rcc::clocks());

    spawner
        .spawn(blink(
            Output::new(p.PD3, Level::High, Default::default()),
            100,
        ))
        .unwrap();

    loop {
        println!("tick");
        Timer::after_millis(1000).await;
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hal::println!("\n\n\n{}", info);

    loop {}
}
