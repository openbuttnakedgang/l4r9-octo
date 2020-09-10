//! Interfacing the on-board L3GD20 (gyroscope)
//#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use]
extern crate log;
extern crate jlink_rtt;

#[macro_use]
mod macros;
mod sys;
#[cfg(not(feature = "prod"))]
mod emblog;
mod ospi;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate embedded_hal as ehal;
//extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;
//use crate::hal::qspi::{Qspi, QspiConfig, QspiMode, QspiReadCommand};

use crate::ehal::spi::{Mode, Phase, Polarity};
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::rt::ExceptionFrame;
use cortex_m::asm;
use stm32l4xx_hal::pac;


/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

//spi clock pin - i6
//spi io 0(SI) pin - i11
//spi io 1(SO) pin - i10
//spi cs pin - g12

// AF5:
// OCTOSPIM_P2_NCS - pg12
// OCTOSPIM_P2_DQS - pg15
// OCTOSPIM_P2_CLK - pi6

// OCTOSPIM_P2_IO6 - pg9
// OCTOSPIM_P2_IO7 - pg10
// OCTOSPIM_P2_IO3 - ph8
// OCTOSPIM_P2_IO4 - ph9
// OCTOSPIM_P2_IO5 - ph10
// OCTOSPIM_P2_IO2 - pi9
// OCTOSPIM_P2_IO1 - pi10
// OCTOSPIM_P2_IO0 - pi11

//TODO: запустить пример из куба в kale

#[entry]
fn main() -> ! {
    emblog::init().unwrap();

    let p = hal::stm32::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);

    info!("Hello");

    // TRY the other clock configuration
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb2);
    let mut gpioi = p.GPIOI.split(&mut rcc.ahb2);
    let mut gpiog = p.GPIOG.split(&mut rcc.ahb2);
    let rcc1 = unsafe { &*pac::RCC::ptr()};
    rcc1.ahb2enr.modify(|_, w| {
        w.gpioien().set_bit();
        w
    });

    // let mut pinout = gpioi.pi6.into_push_pull_output(&mut gpioi.moder, &mut gpioi.otyper);

    // loop {
    //     pinout.set_low();
    //     info!("Pin");
    //     pinout.set_high();  
    // }


    
    // // The `L3gd20` abstraction exposed by the `f3` crate requires a specific pin configuration to
    // // be used and won't accept any configuration other than the one used here. Trying to use a
    // // different pin configuration will result in a compiler error.
    // let sck = gpioi.pi6.into_af5(&mut gpioi.moder, &mut gpioi.afrl);
    // let miso = gpioi.pi11.into_af5(&mut gpioi.moder, &mut gpioi.afrh);
    // let mosi = gpioi.pi10.into_af5(&mut gpioi.moder, &mut gpioi.afrh);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let sck2 = gpiob.pb13.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
    let miso2 = gpiob.pb14.into_af5(&mut gpiob.moder, &mut gpiob.afrh);
    let mosi2 = gpiob.pb15.into_af5(&mut gpiob.moder, &mut gpiob.afrh);

    // // nss.set_high();
    // dc.set_low();

    let mut spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        MODE,
        // 1.mhz(),
        100.khz(),
        clocks,
        &mut rcc.apb2,
    );

    // // nss.set_low();
    let data = [0x3C];
    spi.write(&data).unwrap();
    spi.write(&data).unwrap();
    spi.write(&data).unwrap();
    info!("Buy");
    // // nss.set_high();

    // // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // // gyroscope and the temperature sensor readings
    asm::bkpt();

    loop {}
}

// #[exception]
// fn HardFault(ef: &ExceptionFrame) -> ! {
//     panic!("{:#?}", ef);
// }