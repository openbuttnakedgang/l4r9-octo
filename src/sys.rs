use core::panic::PanicInfo;

use cortex_m::asm::{self, bkpt, delay, wfi};
use cortex_m::interrupt::{disable as int_disable, enable as int_enable};
use cortex_m_rt::{exception, ExceptionFrame};
//use stm32f1xx_hal::backup_domain::BackupDomain;

pub fn reset() -> ! {
    cortex_m::interrupt::disable();
    cortex_m::peripheral::SCB::sys_reset()
}

//pub fn goto_bootloader(backup_domain: &mut BackupDomain) -> ! {
//    // static const uint32_t CMD_BOOT = 0x544F4F42UL;
//    // BOOT in ACII
//    const BOOT: [u16;2] = [0x4F42, 0x544F];
//
//    backup_domain.write_data_register_low(0, BOOT[0]);
//    backup_domain.write_data_register_low(1, BOOT[1]);
//
//    cortex_m::interrupt::disable();
//    cortex_m::peripheral::SCB::sys_reset()
//}

#[cfg(not(feature = "prod"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    int_disable();
    error!("Panic handler! Reseting...");
    error!("Panic info : {}", info);
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(feature = "prod")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(not(feature = "prod"))]
#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    int_disable();
    error!("HardFault handler!");
    error!("{:?}", &ef);
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(feature = "prod")]
#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
