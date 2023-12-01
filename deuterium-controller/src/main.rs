//! Deuterium Battery Controller
//!
//! # Task Priorities
//! | Prio | Use        |
//! | ---- | ---------- |
//! | 0    | Background |
//! | 1    | General    |
//! | 2    | Comms      |
//! | 3    | Critical   |
//!

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod config;
mod io;
mod state;

use defmt_rtt as _;
use panic_probe as _;
use stm32l4xx_hal as hal;

use bxcan::{filter::Mask32, Interrupts};
use hal::prelude::*;
use hal::{
    can::Can,
    rtc::{Rtc, RtcClockSource, RtcConfig},
    watchdog::IndependentWatchdog,
};
use rtic_monotonics::{systick::Systick, Monotonic};
use state::State;

#[rtic::app(device = hal::pac, dispatchers = [SAI1, SWPMI1, QUADSPI])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        can1: bxcan::Can<io::Can1>,
        rtc: Rtc,
        state: State,
    }

    #[local]
    struct Local {
        watchdog: IndependentWatchdog,
        led_status: io::LedStatus,
        led_error: io::LedError,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::trace!("task: init");

        // peripheral access
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut pwr = cx.device.PWR.constrain(&mut rcc.apb1r1);
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb2);

        // configure system clock
        let clocks = rcc
            .cfgr
            .sysclk(80.MHz())
            .pclk1(80.MHz())
            .pclk2(80.MHz())
            .freeze(&mut flash.acr, &mut pwr);

        let mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, clocks.sysclk().to_Hz(), mono_token);

        let can1 = {
            let rx =
                gpioa
                    .pa11
                    .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
            let tx =
                gpioa
                    .pa12
                    .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

            let mut can = bxcan::Can::builder(Can::new(&mut rcc.apb1r1, cx.device.CAN1, (tx, rx)))
                .set_bit_timing(0x001c_0009)
                .enable();

            can.modify_filters().enable_bank(0, Mask32::accept_all());

            can.enable_interrupts(
                Interrupts::TRANSMIT_MAILBOX_EMPTY
                    | Interrupts::FIFO0_MESSAGE_PENDING
                    | Interrupts::FIFO1_MESSAGE_PENDING,
            );

            can.enable_non_blocking().unwrap();

            can
        };

        let rtc = Rtc::rtc(
            cx.device.RTC,
            &mut rcc.apb1r1,
            &mut rcc.bdcr,
            &mut pwr.cr1,
            RtcConfig::default().clock_config(RtcClockSource::LSE),
        );

        let led_status = gpiob
            .pb13
            .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);

        let led_error = gpiob
            .pb14
            .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);

        let watchdog = {
            let mut wd = IndependentWatchdog::new(cx.device.IWDG);
            wd.stop_on_debug(&cx.device.DBGMCU, true);
            wd.start(100.millis());
            wd
        };

        let state = State::default();

        (
            Shared { can1, rtc, state },
            Local {
                watchdog,
                led_status,
                led_error,
            },
        )
    }

    #[task(priority = 1, local = [watchdog])]
    async fn feed_watchdog(cx: feed_watchdog::Context) {
        loop {
            Systick::delay(50.millis()).await;

            defmt::trace!("task: feed_watchdog");

            cx.local.watchdog.feed();
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::trace!("task: idle");

        loop {
            cortex_m::asm::nop();
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

defmt::timestamp!("{=u32}ms", {
    // 1 tick = 1 millisecond
    Systick::now().duration_since_epoch().to_millis()
});
