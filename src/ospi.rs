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

        // Включаю тактирование octospi2
        rcc.ahb3enr.write(|w| w.ospi2en().set_bit());

        let mut rcc = p.RCC.constrain();
        let mut gpioi = p.GPIOI.split(&mut rcc.ahb2);
        let mut gpiog = p.GPIOG.split(&mut rcc.ahb2);
        let mut gpioh = p.GPIOH.split(&mut rcc.ahb2);

        // Включение нужных пинов
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




        // Очищение регистров
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

        // Заполнение control register
        ospi.cr.modify(|_, w| unsafe {
            w.en()
                .set_bit()
                .apms()
                .set_bit()
                .fthres()
                .bits(3u8 - 1u8)
        });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение device configuration register 1
        unsafe { ospi.dcr1.modify(|_, w| 
            w.csht()
                .bits(1u8)
                .devsize()
                .bits(25u8)
        )};
        while ospi.sr.read().busy().bit_is_set() {}
        // Заполнение device configuration register 2
        unsafe { ospi.dcr2.modify(|_, w| w.prescaler().bits(2u8)) };
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение data length register
        ospi.dlr.write(|w| unsafe { w.dl().bits(3u32 - 1) });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение communication configuration register параметрами команды
        ospi.ccr.modify(|_, w| unsafe {
            w.admode()
                .bits(0)
                .adsize()
                .bits(0)
                .abmode()
                .bits(0)
                .absize()
                .bits(0)
                .dmode()
                .bits(1u8)
                .imode()
                .bits(1u8)
                .ddtr()
                .clear_bit()
                .isize()
                .bits(0u8)
        });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение timing configuration register
        ospi.tcr.modify(|_, w| unsafe { w.dcyc().bits(0b0) });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение instruction register кодом команды
        ospi.ir.modify(|_, w| unsafe { w.instruction().bits(0x06F9) });
        while ospi.sr.read().busy().bit_is_set() {}

        // Указание режима работы ostospi, обязательно последним
        ospi.cr.modify(|_, w| unsafe {w.fmode().bits(0u8)});
        while ospi.sr.read().busy().bit_is_set() {}
        info!("{:x?}", &ospi.dr as *const _ as *const u8);
        // Заполнение регистра адреса, чтобы стриггерить отправку команды
        //ospi.ar.modify(|_, w| unsafe { w.address().bits(0x0) });
        ospi.ir.modify(|_, w| unsafe { w.instruction().bits(0x06F9) });

        if ospi.sr.read().tef().bit_is_set() {
            println!("Transfer error");
        }

        // Заполнение communication configuration register параметрами команды
        ospi.ccr.modify(|_, w| unsafe {
            w.admode()
                .bits(1)
                .adsize()
                .bits(3)
                .abmode()
                .bits(0)
                .absize()
                .bits(0)
                .dmode()
                .bits(0u8)
                .imode()
                .bits(1u8)
                .ddtr()
                .clear_bit()
                .isize()
                .bits(1u8)
        });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение timing configuration register
        ospi.tcr.modify(|_, w| unsafe { w.dcyc().bits(0b0) });
        while ospi.sr.read().busy().bit_is_set() {}

        // Заполнение instruction register кодом команды
        ospi.ir.modify(|_, w| unsafe { w.instruction().bits(0xDC0) });
        while ospi.sr.read().busy().bit_is_set() {}

        // Указание режима работы ostospi, обязательно последним
        ospi.cr.modify(|_, w| unsafe {w.fmode().bits(0u8)});
        while ospi.sr.read().busy().bit_is_set() {}

        ospi.ar.modify(|_, w| unsafe { w.address().bits(0x0) });

        if ospi.sr.read().tef().bit_is_set() {
            println!("Transfer error");
        }

        // let buffer: &mut [u8] = &mut [0u8; 3];
        // let mut b = buffer.iter_mut();
        // while ospi.sr.read().tcf().bit_is_clear() {
        //     if ospi.sr.read().ftf().bit_is_set() {
        //         if let Some(v) = b.next() {
        //             unsafe {
        //                 *v = ptr::read_volatile(&ospi.dr as *const _ as *const u8);
        //             }
        //         } else {
        //             println!("Error");
        //             // OVERFLOW
        //         }
        //     }
        // }
        // // When transfer complete, empty fifo buffer
        // while ospi.sr.read().flevel().bits() > 0 {
        //     if let Some(v) = b.next() {
        //         unsafe {
        //             *v = ptr::read_volatile(&ospi.dr as *const _ as *const u8);
        //         }
        //     } else {
        //         // OVERFLOW
        //     }
        // }

        println!("Finally!! Congratulations");
    }
}
