#![no_std]
#![no_main]

#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate jlink_rtt;

use cortex_m_rt::{entry, exception};
use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;

use stm32l4xx_hal::{
    prelude::*,
    i2c, pac,
    prelude::*,
    rtc::Rtc,
    stm32,
    stm32::interrupt,
    stm32::Interrupt,
    datetime::{Date, Time},
    timer::{Event, Timer},
    pwr::{Pwr, PwrExt},
    delay::Delay,
    pac::NVIC,
    gpio,
};


#[macro_use]
mod macros;
mod sys;
#[cfg(not(feature = "prod"))]
mod emblog;

// ------ ------
// SINGLETONS
// ------ ------

static mut G_RTC: Option<Rtc> = None;


#[entry]
fn main() -> ! {
    #[cfg(not(feature = "prod"))]
    emblog::init().unwrap();

    info!("Loader startup!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);


    let clocks = rcc.cfgr
        .sysclk(64.mhz())
        .lsi(true)
        .hsi48(true)
        .freeze(&mut flash.acr, &mut pwr);

    let mut timer = Delay::new(cp.SYST, clocks);
    let rtc = Rtc::rtc(dp.RTC, &mut rcc.apb1r1, &mut rcc.bdcr, &mut pwr.cr1, clocks);

    let mut time = Time::new(21.hours(), 57.minutes(), 32.seconds(), false);
    let mut date = Date::new(1.day(), 24.date(), 4.month(), 2018.year());

    rtc.set_time(&time);
    rtc.set_date(&date);

    //timer.delay_ms(1000_u32);
    //timer.delay_ms(1000_u32);
    //timer.delay_ms(1000_u32);
    //delay(64_000_000);
    //delay(64_000_000);
    //delay(64_000_000);

    time = rtc.get_time();
    date = rtc.get_date();

    unsafe { G_RTC = Some(rtc); }
    unsafe { NVIC::unmask(Interrupt::RTC_WKUP) };

    info!("Time: {:?}", time);
    info!("Date: {:?}", date);
    info!("Good bye!");

    loop {

        core::sync::atomic::fence(
            core::sync::atomic::Ordering::SeqCst
        );
    }
}


#[interrupt]
fn RTC_WKUP() {
    
    let rtc = unsafe { G_RTC.as_mut().unwrap() };

    let time = rtc.get_time();
    let date = rtc.get_date();

    info!("Time: {:?}", time);
    info!("Date: {:?}", date);
    info!("Good bye!");
}

#[exception]
fn SysTick() {
    //let rtc = unsafe { G_RTC.as_mut().unwrap() };

    //let time = rtc.get_time();
    //let date = rtc.get_date();

    //info!("Time: {:?}", time);
    //info!("Date: {:?}", date);
    //info!("Good bye!");
}
