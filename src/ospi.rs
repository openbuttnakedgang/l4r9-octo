use stm32l4xx_hal::pac::{OCTOSPI2, RCC};

use stm32l4xx_hal::gpio::{
    gpioi::{PI6, PI9, PI10, PI11},
    gpiog::{PG10, PG12, PG15},
    gpioh::{PH8, PH9, PH10},
};

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

struct OSpi;

impl OSpi {
    pub fn new() -> Self {
        OSpi
    }

    pub fn init(&mut self) {
        let ospi = unsafe { &*OCTOSPI2::ptr() };
        let rcc = unsafe { &*RCC::ptr() };
        let p = unsafe { hal::stm32::Peripherals::steal() };

        //Включаю тактирование octospi2
        rcc.ahb3enr.write(|w| w.ospi2en().set_bit());
        let mut gpioi = p.GPIOI.split(&mut rcc.ahb2);
        let mut gpiog = p.GPIOG.split(&mut rcc.ahb2);
        let mut gpioh = p.GPIOH.split(&mut rcc.ahb2);
    }
}