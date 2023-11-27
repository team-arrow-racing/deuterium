#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use pac25_hal::pac25140 as hal;
use panic_probe as _;

use hal::prelude::*;
use rtic_monotonics::systick::Systick;
use rtic_monotonics::Monotonic;

const SYSCLK: u32 = 150_000_000;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("Init");

        let mono_token = rtic_monotonics::create_systick_token!();
        let mono = Systick::start(cx.core.SYST, SYSCLK, mono_token);

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("Idling...");

        loop {
            cortex_m::asm::nop();
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

defmt::timestamp!("time={=u32}ms", {
    // 1 tick = 1 millisecond
    Systick::now().ticks()
});
