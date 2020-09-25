use stm32l4xx_hal::pac::{OCTOSPI2, RCC};

use crate::hal::prelude::*;
use stm32l4xx_hal::gpio::{
    // gpiog::{PG10, PG12, PG15},
    // gpioh::{PH10, PH8, PH9},
    // gpioi::{PI10, PI11, PI6, PI9},
    Speed,
};

use core::ptr;

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

pub struct OSpi;

impl OSpi {
    pub fn new() -> Self {
        OSpi
    }

    pub fn init(&mut self) {
        let ospi = unsafe { &*OCTOSPI2::ptr() };
        let rcc = unsafe { &*RCC::ptr() };
        let p = unsafe { hal::stm32::Peripherals::steal() };
        //let mut rcc = p.RCC.constrain();

        //Включаю тактирование octospi2
        rcc.ahb3enr.write(|w| w.ospi2en().set_bit());

        let mut rcc = p.RCC.constrain();
        let mut gpioi = p.GPIOI.split(&mut rcc.ahb2);
        let mut gpiog = p.GPIOG.split(&mut rcc.ahb2);
        let mut gpioh = p.GPIOH.split(&mut rcc.ahb2);

        let _clk = gpioi.pi6.into_af5(&mut gpioi.moder, &mut gpioi.afrl).set_speed(Speed::VeryHigh);
        let _ncs = gpiog.pg12.into_af5(&mut gpiog.moder, &mut gpiog.afrh).set_speed(Speed::VeryHigh);
        let _dqs = gpiog.pg15.into_af5(&mut gpiog.moder, &mut gpiog.afrh).set_speed(Speed::VeryHigh);

        let _io0 = gpioi.pi11.into_af5(&mut gpioi.moder, &mut gpioi.afrh).set_speed(Speed::VeryHigh);
        let _io1 = gpioi.pi10.into_af5(&mut gpioi.moder, &mut gpioi.afrh).set_speed(Speed::VeryHigh);
        let _io2 = gpioi.pi9.into_af5(&mut gpioi.moder, &mut gpioi.afrh).set_speed(Speed::VeryHigh);
        let _io3 = gpioh.ph8.into_af5(&mut gpioh.moder, &mut gpioh.afrh).set_speed(Speed::VeryHigh);
        let _io4 = gpioh.ph9.into_af5(&mut gpioh.moder, &mut gpioh.afrh).set_speed(Speed::VeryHigh);
        let _io5 = gpioh.ph10.into_af5(&mut gpioh.moder, &mut gpioh.afrh).set_speed(Speed::VeryHigh);
        let _io6 = gpiog.pg9.into_af5(&mut gpiog.moder, &mut gpiog.afrh).set_speed(Speed::VeryHigh);
        let _io7 = gpiog.pg10.into_af5(&mut gpiog.moder, &mut gpiog.afrh).set_speed(Speed::VeryHigh);



        ospi.cr.modify(|_, w| w.en().clear_bit());

        ospi.fcr.write(|w| {
            w.ctof()
                .set_bit()
                .csmf()
                .set_bit()
                .ctcf()
                .set_bit()
                .ctef()
                .set_bit()
        });

        // Octospi config
        unsafe { ospi.cr.modify(|_, w| w.fthres().bits(2u8 - 1u8)) };
        while ospi.sr.read().busy().bit_is_set() {}
        ospi.cr.modify(|_, w| w.teie().set_bit());
        while ospi.sr.read().busy().bit_is_set() {}
        unsafe { ospi.cr.modify(|_, w| w.fmode().bits(1u8)) };
        while ospi.sr.read().busy().bit_is_set() {}
        ospi.cr.modify(|_, w| w.en().set_bit());
        while ospi.sr.read().busy().bit_is_set() {}
        ospi.cr.modify(|_, w| w.apms().set_bit());
        while ospi.sr.read().busy().bit_is_set() {}
        unsafe { ospi.dcr1.modify(|_, w| w.csht().bits(1u8)) };
        while ospi.sr.read().busy().bit_is_set() {}
        unsafe { ospi.dcr1.modify(|_, w| w.devsize().bits(25u8)) };
        while ospi.sr.read().busy().bit_is_set() {}
        unsafe { ospi.dcr2.modify(|_, w| w.prescaler().bits(1u8)) };
        while ospi.sr.read().busy().bit_is_set() {}
        ospi.sr.modify(|_, w| w.ftf().set_bit());
        while ospi.sr.read().busy().bit_is_set() {}

        //ospi.fcr.write(|w| w.ctcf().set_bit());
        ospi.dlr.write(|w| unsafe { w.dl().bits(3u32 - 1) });
        //ospi.cr.modify(|_, w| w.sshift().bit(false));

        // Write CCR register with instruction etc.
        ospi.ccr.modify(|_, w| unsafe {
            w.admode()
                .bits(1)
                .adsize()
                .bits(0b11)
                .abmode()
                .bits(0b0)
                .absize()
                .bits(0b0)
                .dmode()
                .bits(0b1)
                .imode()
                .bits(0b1)
                .ddtr()
                .clear_bit()
        });

        ospi.tcr.modify(|_, w| unsafe { w.dcyc().bits(0b0) });
        ospi.ir.modify(|_, w| unsafe { w.instruction().bits(0x9f) });

        if ospi.sr.read().tef().bit_is_set() {
            println!("Transfer error");
        }

        let buffer: &mut [u8] = &mut [0u8; 3];
        let mut b = buffer.iter_mut();
        while ospi.sr.read().tcf().bit_is_clear() {
            if ospi.sr.read().ftf().bit_is_set() {
                if let Some(v) = b.next() {
                    unsafe {
                        *v = ptr::read_volatile(&ospi.dr as *const _ as *const u8);
                    }
                } else {
                    println!("Error");
                    // OVERFLOW
                }
            }
        }
        // When transfer complete, empty fifo buffer
        while ospi.sr.read().flevel().bits() > 0 {
            if let Some(v) = b.next() {
                unsafe {
                    *v = ptr::read_volatile(&ospi.dr as *const _ as *const u8);
                }
            } else {
                // OVERFLOW
            }
        }

        println!("Finally!! Congratulations: {}", buffer[0]);
    }
}
