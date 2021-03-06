///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - channel configuration y register
    pub chcfg0r1: CHCFG0R1,
    ///0x04 - channel configuration y register
    pub chcfg0r2: CHCFG0R2,
    ///0x08 - analog watchdog and short-circuit detector register
    pub awscd0r: AWSCD0R,
    ///0x0c - channel watchdog filter data register
    pub chwdat0r: CHWDAT0R,
    ///0x10 - channel data input register
    pub chdatin0r: CHDATIN0R,
    _reserved5: [u8; 12usize],
    ///0x20 - CHCFG1R1
    pub chcfg1r1: CHCFG1R1,
    ///0x24 - CHCFG1R2
    pub chcfg1r2: CHCFG1R2,
    ///0x28 - AWSCD1R
    pub awscd1r: AWSCD1R,
    ///0x2c - CHWDAT1R
    pub chwdat1r: CHWDAT1R,
    ///0x30 - CHDATIN1R
    pub chdatin1r: CHDATIN1R,
    _reserved10: [u8; 12usize],
    ///0x40 - CHCFG2R1
    pub chcfg2r1: CHCFG2R1,
    ///0x44 - CHCFG2R2
    pub chcfg2r2: CHCFG2R2,
    ///0x48 - AWSCD2R
    pub awscd2r: AWSCD2R,
    ///0x4c - CHWDAT2R
    pub chwdat2r: CHWDAT2R,
    ///0x50 - CHDATIN2R
    pub chdatin2r: CHDATIN2R,
    _reserved15: [u8; 12usize],
    ///0x60 - CHCFG3R1
    pub chcfg3r1: CHCFG3R1,
    ///0x64 - CHCFG3R2
    pub chcfg3r2: CHCFG3R2,
    ///0x68 - AWSCD3R
    pub awscd3r: AWSCD3R,
    ///0x6c - CHWDAT3R
    pub chwdat3r: CHWDAT3R,
    ///0x70 - CHDATIN3R
    pub chdatin3r: CHDATIN3R,
    _reserved20: [u8; 12usize],
    ///0x80 - CHCFG4R1
    pub chcfg4r1: CHCFG4R1,
    ///0x84 - CHCFG4R2
    pub chcfg4r2: CHCFG4R2,
    ///0x88 - AWSCD4R
    pub awscd4r: AWSCD4R,
    ///0x8c - CHWDAT4R
    pub chwdat4r: CHWDAT4R,
    ///0x90 - CHDATIN4R
    pub chdatin4r: CHDATIN4R,
    _reserved25: [u8; 12usize],
    ///0xa0 - CHCFG5R1
    pub chcfg5r1: CHCFG5R1,
    ///0xa4 - CHCFG5R2
    pub chcfg5r2: CHCFG5R2,
    ///0xa8 - AWSCD5R
    pub awscd5r: AWSCD5R,
    ///0xac - CHWDAT5R
    pub chwdat5r: CHWDAT5R,
    ///0xb0 - CHDATIN5R
    pub chdatin5r: CHDATIN5R,
    _reserved30: [u8; 12usize],
    ///0xc0 - CHCFG6R1
    pub chcfg6r1: CHCFG6R1,
    ///0xc4 - CHCFG6R2
    pub chcfg6r2: CHCFG6R2,
    ///0xc8 - AWSCD6R
    pub awscd6r: AWSCD6R,
    ///0xcc - CHWDAT6R
    pub chwdat6r: CHWDAT6R,
    ///0xd0 - CHDATIN6R
    pub chdatin6r: CHDATIN6R,
    _reserved35: [u8; 12usize],
    ///0xe0 - CHCFG7R1
    pub chcfg7r1: CHCFG7R1,
    ///0xe4 - CHCFG7R2
    pub chcfg7r2: CHCFG7R2,
    ///0xe8 - AWSCD7R
    pub awscd7r: AWSCD7R,
    ///0xec - CHWDAT7R
    pub chwdat7r: CHWDAT7R,
    ///0xf0 - CHDATIN7R
    pub chdatin7r: CHDATIN7R,
    _reserved40: [u8; 12usize],
    ///0x100 - control register 1
    pub dfsdm0_cr1: DFSDM0_CR1,
    ///0x104 - control register 2
    pub dfsdm0_cr2: DFSDM0_CR2,
    ///0x108 - interrupt and status register
    pub dfsdm0_isr: DFSDM0_ISR,
    ///0x10c - interrupt flag clear register
    pub dfsdm0_icr: DFSDM0_ICR,
    ///0x110 - injected channel group selection register
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    ///0x114 - filter control register
    pub dfsdm0_fcr: DFSDM0_FCR,
    ///0x118 - data register for injected group
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    ///0x11c - data register for the regular channel
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    ///0x120 - analog watchdog high threshold register
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    ///0x124 - analog watchdog low threshold register
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    ///0x128 - analog watchdog status register
    pub dfsdm0_awsr: DFSDM0_AWSR,
    ///0x12c - analog watchdog clear flag register
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    ///0x130 - Extremes detector maximum register
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    ///0x134 - Extremes detector minimum register
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    ///0x138 - conversion timer register
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved55: [u8; 196usize],
    ///0x200 - control register 1
    pub dfsdm1_cr1: DFSDM1_CR1,
    ///0x204 - control register 2
    pub dfsdm1_cr2: DFSDM1_CR2,
    ///0x208 - interrupt and status register
    pub dfsdm1_isr: DFSDM1_ISR,
    ///0x20c - interrupt flag clear register
    pub dfsdm1_icr: DFSDM1_ICR,
    ///0x210 - injected channel group selection register
    pub dfsdm1_jchgr: DFSDM1_JCHGR,
    ///0x214 - filter control register
    pub dfsdm1_fcr: DFSDM1_FCR,
    ///0x218 - data register for injected group
    pub dfsdm1_jdatar: DFSDM1_JDATAR,
    ///0x21c - data register for the regular channel
    pub dfsdm1_rdatar: DFSDM1_RDATAR,
    ///0x220 - analog watchdog high threshold register
    pub dfsdm1_awhtr: DFSDM1_AWHTR,
    ///0x224 - analog watchdog low threshold register
    pub dfsdm1_awltr: DFSDM1_AWLTR,
    ///0x228 - analog watchdog status register
    pub dfsdm1_awsr: DFSDM1_AWSR,
    ///0x22c - analog watchdog clear flag register
    pub dfsdm1_awcfr: DFSDM1_AWCFR,
    ///0x230 - Extremes detector maximum register
    pub dfsdm1_exmax: DFSDM1_EXMAX,
    ///0x234 - Extremes detector minimum register
    pub dfsdm1_exmin: DFSDM1_EXMIN,
    ///0x238 - conversion timer register
    pub dfsdm1_cnvtimr: DFSDM1_CNVTIMR,
    _reserved70: [u8; 196usize],
    ///0x300 - control register 1
    pub dfsdm2_cr1: DFSDM2_CR1,
    ///0x304 - control register 2
    pub dfsdm2_cr2: DFSDM2_CR2,
    ///0x308 - interrupt and status register
    pub dfsdm2_isr: DFSDM2_ISR,
    ///0x30c - interrupt flag clear register
    pub dfsdm2_icr: DFSDM2_ICR,
    ///0x310 - injected channel group selection register
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    ///0x314 - filter control register
    pub dfsdm2_fcr: DFSDM2_FCR,
    ///0x318 - data register for injected group
    pub dfsdm2_jdatar: DFSDM2_JDATAR,
    ///0x31c - data register for the regular channel
    pub dfsdm2_rdatar: DFSDM2_RDATAR,
    ///0x320 - analog watchdog high threshold register
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    ///0x324 - analog watchdog low threshold register
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    ///0x328 - analog watchdog status register
    pub dfsdm2_awsr: DFSDM2_AWSR,
    ///0x32c - analog watchdog clear flag register
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    ///0x330 - Extremes detector maximum register
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    ///0x334 - Extremes detector minimum register
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    ///0x338 - conversion timer register
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved85: [u8; 196usize],
    ///0x400 - control register 1
    pub dfsdm3_cr1: DFSDM3_CR1,
    ///0x404 - control register 2
    pub dfsdm3_cr2: DFSDM3_CR2,
    ///0x408 - interrupt and status register
    pub dfsdm3_isr: DFSDM3_ISR,
    ///0x40c - interrupt flag clear register
    pub dfsdm3_icr: DFSDM3_ICR,
    ///0x410 - injected channel group selection register
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    ///0x414 - filter control register
    pub dfsdm3_fcr: DFSDM3_FCR,
    ///0x418 - data register for injected group
    pub dfsdm3_jdatar: DFSDM3_JDATAR,
    ///0x41c - data register for the regular channel
    pub dfsdm3_rdatar: DFSDM3_RDATAR,
    ///0x420 - analog watchdog high threshold register
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    ///0x424 - analog watchdog low threshold register
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    ///0x428 - analog watchdog status register
    pub dfsdm3_awsr: DFSDM3_AWSR,
    ///0x42c - analog watchdog clear flag register
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    ///0x430 - Extremes detector maximum register
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    ///0x434 - Extremes detector minimum register
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    ///0x438 - conversion timer register
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
}
///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg0r1](chcfg0r1) module
pub type CHCFG0R1 = crate::Reg<u32, _CHCFG0R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG0R1;
///`read()` method returns [chcfg0r1::R](chcfg0r1::R) reader structure
impl crate::Readable for CHCFG0R1 {}
///`write(|w| ..)` method takes [chcfg0r1::W](chcfg0r1::W) writer structure
impl crate::Writable for CHCFG0R1 {}
///channel configuration y register
pub mod chcfg0r1;
///channel configuration y register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg0r2](chcfg0r2) module
pub type CHCFG0R2 = crate::Reg<u32, _CHCFG0R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG0R2;
///`read()` method returns [chcfg0r2::R](chcfg0r2::R) reader structure
impl crate::Readable for CHCFG0R2 {}
///`write(|w| ..)` method takes [chcfg0r2::W](chcfg0r2::W) writer structure
impl crate::Writable for CHCFG0R2 {}
///channel configuration y register
pub mod chcfg0r2;
///analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd0r](awscd0r) module
pub type AWSCD0R = crate::Reg<u32, _AWSCD0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD0R;
///`read()` method returns [awscd0r::R](awscd0r::R) reader structure
impl crate::Readable for AWSCD0R {}
///`write(|w| ..)` method takes [awscd0r::W](awscd0r::W) writer structure
impl crate::Writable for AWSCD0R {}
///analog watchdog and short-circuit detector register
pub mod awscd0r;
///channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat0r](chwdat0r) module
pub type CHWDAT0R = crate::Reg<u32, _CHWDAT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT0R;
///`read()` method returns [chwdat0r::R](chwdat0r::R) reader structure
impl crate::Readable for CHWDAT0R {}
///`write(|w| ..)` method takes [chwdat0r::W](chwdat0r::W) writer structure
impl crate::Writable for CHWDAT0R {}
///channel watchdog filter data register
pub mod chwdat0r;
///channel data input register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin0r](chdatin0r) module
pub type CHDATIN0R = crate::Reg<u32, _CHDATIN0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN0R;
///`read()` method returns [chdatin0r::R](chdatin0r::R) reader structure
impl crate::Readable for CHDATIN0R {}
///`write(|w| ..)` method takes [chdatin0r::W](chdatin0r::W) writer structure
impl crate::Writable for CHDATIN0R {}
///channel data input register
pub mod chdatin0r;
///CHCFG1R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg1r1](chcfg1r1) module
pub type CHCFG1R1 = crate::Reg<u32, _CHCFG1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG1R1;
///`read()` method returns [chcfg1r1::R](chcfg1r1::R) reader structure
impl crate::Readable for CHCFG1R1 {}
///`write(|w| ..)` method takes [chcfg1r1::W](chcfg1r1::W) writer structure
impl crate::Writable for CHCFG1R1 {}
///CHCFG1R1
pub mod chcfg1r1;
///CHCFG1R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg1r2](chcfg1r2) module
pub type CHCFG1R2 = crate::Reg<u32, _CHCFG1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG1R2;
///`read()` method returns [chcfg1r2::R](chcfg1r2::R) reader structure
impl crate::Readable for CHCFG1R2 {}
///`write(|w| ..)` method takes [chcfg1r2::W](chcfg1r2::W) writer structure
impl crate::Writable for CHCFG1R2 {}
///CHCFG1R2
pub mod chcfg1r2;
///AWSCD1R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd1r](awscd1r) module
pub type AWSCD1R = crate::Reg<u32, _AWSCD1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD1R;
///`read()` method returns [awscd1r::R](awscd1r::R) reader structure
impl crate::Readable for AWSCD1R {}
///`write(|w| ..)` method takes [awscd1r::W](awscd1r::W) writer structure
impl crate::Writable for AWSCD1R {}
///AWSCD1R
pub mod awscd1r;
///CHWDAT1R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat1r](chwdat1r) module
pub type CHWDAT1R = crate::Reg<u32, _CHWDAT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT1R;
///`read()` method returns [chwdat1r::R](chwdat1r::R) reader structure
impl crate::Readable for CHWDAT1R {}
///`write(|w| ..)` method takes [chwdat1r::W](chwdat1r::W) writer structure
impl crate::Writable for CHWDAT1R {}
///CHWDAT1R
pub mod chwdat1r;
///CHDATIN1R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin1r](chdatin1r) module
pub type CHDATIN1R = crate::Reg<u32, _CHDATIN1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN1R;
///`read()` method returns [chdatin1r::R](chdatin1r::R) reader structure
impl crate::Readable for CHDATIN1R {}
///`write(|w| ..)` method takes [chdatin1r::W](chdatin1r::W) writer structure
impl crate::Writable for CHDATIN1R {}
///CHDATIN1R
pub mod chdatin1r;
///CHCFG2R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg2r1](chcfg2r1) module
pub type CHCFG2R1 = crate::Reg<u32, _CHCFG2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG2R1;
///`read()` method returns [chcfg2r1::R](chcfg2r1::R) reader structure
impl crate::Readable for CHCFG2R1 {}
///`write(|w| ..)` method takes [chcfg2r1::W](chcfg2r1::W) writer structure
impl crate::Writable for CHCFG2R1 {}
///CHCFG2R1
pub mod chcfg2r1;
///CHCFG2R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg2r2](chcfg2r2) module
pub type CHCFG2R2 = crate::Reg<u32, _CHCFG2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG2R2;
///`read()` method returns [chcfg2r2::R](chcfg2r2::R) reader structure
impl crate::Readable for CHCFG2R2 {}
///`write(|w| ..)` method takes [chcfg2r2::W](chcfg2r2::W) writer structure
impl crate::Writable for CHCFG2R2 {}
///CHCFG2R2
pub mod chcfg2r2;
///AWSCD2R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd2r](awscd2r) module
pub type AWSCD2R = crate::Reg<u32, _AWSCD2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD2R;
///`read()` method returns [awscd2r::R](awscd2r::R) reader structure
impl crate::Readable for AWSCD2R {}
///`write(|w| ..)` method takes [awscd2r::W](awscd2r::W) writer structure
impl crate::Writable for AWSCD2R {}
///AWSCD2R
pub mod awscd2r;
///CHWDAT2R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat2r](chwdat2r) module
pub type CHWDAT2R = crate::Reg<u32, _CHWDAT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT2R;
///`read()` method returns [chwdat2r::R](chwdat2r::R) reader structure
impl crate::Readable for CHWDAT2R {}
///`write(|w| ..)` method takes [chwdat2r::W](chwdat2r::W) writer structure
impl crate::Writable for CHWDAT2R {}
///CHWDAT2R
pub mod chwdat2r;
///CHDATIN2R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin2r](chdatin2r) module
pub type CHDATIN2R = crate::Reg<u32, _CHDATIN2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN2R;
///`read()` method returns [chdatin2r::R](chdatin2r::R) reader structure
impl crate::Readable for CHDATIN2R {}
///`write(|w| ..)` method takes [chdatin2r::W](chdatin2r::W) writer structure
impl crate::Writable for CHDATIN2R {}
///CHDATIN2R
pub mod chdatin2r;
///CHCFG3R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg3r1](chcfg3r1) module
pub type CHCFG3R1 = crate::Reg<u32, _CHCFG3R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG3R1;
///`read()` method returns [chcfg3r1::R](chcfg3r1::R) reader structure
impl crate::Readable for CHCFG3R1 {}
///`write(|w| ..)` method takes [chcfg3r1::W](chcfg3r1::W) writer structure
impl crate::Writable for CHCFG3R1 {}
///CHCFG3R1
pub mod chcfg3r1;
///CHCFG3R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg3r2](chcfg3r2) module
pub type CHCFG3R2 = crate::Reg<u32, _CHCFG3R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG3R2;
///`read()` method returns [chcfg3r2::R](chcfg3r2::R) reader structure
impl crate::Readable for CHCFG3R2 {}
///`write(|w| ..)` method takes [chcfg3r2::W](chcfg3r2::W) writer structure
impl crate::Writable for CHCFG3R2 {}
///CHCFG3R2
pub mod chcfg3r2;
///AWSCD3R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd3r](awscd3r) module
pub type AWSCD3R = crate::Reg<u32, _AWSCD3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD3R;
///`read()` method returns [awscd3r::R](awscd3r::R) reader structure
impl crate::Readable for AWSCD3R {}
///`write(|w| ..)` method takes [awscd3r::W](awscd3r::W) writer structure
impl crate::Writable for AWSCD3R {}
///AWSCD3R
pub mod awscd3r;
///CHWDAT3R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat3r](chwdat3r) module
pub type CHWDAT3R = crate::Reg<u32, _CHWDAT3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT3R;
///`read()` method returns [chwdat3r::R](chwdat3r::R) reader structure
impl crate::Readable for CHWDAT3R {}
///`write(|w| ..)` method takes [chwdat3r::W](chwdat3r::W) writer structure
impl crate::Writable for CHWDAT3R {}
///CHWDAT3R
pub mod chwdat3r;
///CHDATIN3R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin3r](chdatin3r) module
pub type CHDATIN3R = crate::Reg<u32, _CHDATIN3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN3R;
///`read()` method returns [chdatin3r::R](chdatin3r::R) reader structure
impl crate::Readable for CHDATIN3R {}
///`write(|w| ..)` method takes [chdatin3r::W](chdatin3r::W) writer structure
impl crate::Writable for CHDATIN3R {}
///CHDATIN3R
pub mod chdatin3r;
///CHCFG4R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg4r1](chcfg4r1) module
pub type CHCFG4R1 = crate::Reg<u32, _CHCFG4R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG4R1;
///`read()` method returns [chcfg4r1::R](chcfg4r1::R) reader structure
impl crate::Readable for CHCFG4R1 {}
///`write(|w| ..)` method takes [chcfg4r1::W](chcfg4r1::W) writer structure
impl crate::Writable for CHCFG4R1 {}
///CHCFG4R1
pub mod chcfg4r1;
///CHCFG4R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg4r2](chcfg4r2) module
pub type CHCFG4R2 = crate::Reg<u32, _CHCFG4R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG4R2;
///`read()` method returns [chcfg4r2::R](chcfg4r2::R) reader structure
impl crate::Readable for CHCFG4R2 {}
///`write(|w| ..)` method takes [chcfg4r2::W](chcfg4r2::W) writer structure
impl crate::Writable for CHCFG4R2 {}
///CHCFG4R2
pub mod chcfg4r2;
///AWSCD4R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd4r](awscd4r) module
pub type AWSCD4R = crate::Reg<u32, _AWSCD4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD4R;
///`read()` method returns [awscd4r::R](awscd4r::R) reader structure
impl crate::Readable for AWSCD4R {}
///`write(|w| ..)` method takes [awscd4r::W](awscd4r::W) writer structure
impl crate::Writable for AWSCD4R {}
///AWSCD4R
pub mod awscd4r;
///CHWDAT4R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat4r](chwdat4r) module
pub type CHWDAT4R = crate::Reg<u32, _CHWDAT4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT4R;
///`read()` method returns [chwdat4r::R](chwdat4r::R) reader structure
impl crate::Readable for CHWDAT4R {}
///`write(|w| ..)` method takes [chwdat4r::W](chwdat4r::W) writer structure
impl crate::Writable for CHWDAT4R {}
///CHWDAT4R
pub mod chwdat4r;
///CHDATIN4R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin4r](chdatin4r) module
pub type CHDATIN4R = crate::Reg<u32, _CHDATIN4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN4R;
///`read()` method returns [chdatin4r::R](chdatin4r::R) reader structure
impl crate::Readable for CHDATIN4R {}
///`write(|w| ..)` method takes [chdatin4r::W](chdatin4r::W) writer structure
impl crate::Writable for CHDATIN4R {}
///CHDATIN4R
pub mod chdatin4r;
///CHCFG5R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg5r1](chcfg5r1) module
pub type CHCFG5R1 = crate::Reg<u32, _CHCFG5R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG5R1;
///`read()` method returns [chcfg5r1::R](chcfg5r1::R) reader structure
impl crate::Readable for CHCFG5R1 {}
///`write(|w| ..)` method takes [chcfg5r1::W](chcfg5r1::W) writer structure
impl crate::Writable for CHCFG5R1 {}
///CHCFG5R1
pub mod chcfg5r1;
///CHCFG5R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg5r2](chcfg5r2) module
pub type CHCFG5R2 = crate::Reg<u32, _CHCFG5R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG5R2;
///`read()` method returns [chcfg5r2::R](chcfg5r2::R) reader structure
impl crate::Readable for CHCFG5R2 {}
///`write(|w| ..)` method takes [chcfg5r2::W](chcfg5r2::W) writer structure
impl crate::Writable for CHCFG5R2 {}
///CHCFG5R2
pub mod chcfg5r2;
///AWSCD5R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd5r](awscd5r) module
pub type AWSCD5R = crate::Reg<u32, _AWSCD5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD5R;
///`read()` method returns [awscd5r::R](awscd5r::R) reader structure
impl crate::Readable for AWSCD5R {}
///`write(|w| ..)` method takes [awscd5r::W](awscd5r::W) writer structure
impl crate::Writable for AWSCD5R {}
///AWSCD5R
pub mod awscd5r;
///CHWDAT5R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat5r](chwdat5r) module
pub type CHWDAT5R = crate::Reg<u32, _CHWDAT5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT5R;
///`read()` method returns [chwdat5r::R](chwdat5r::R) reader structure
impl crate::Readable for CHWDAT5R {}
///`write(|w| ..)` method takes [chwdat5r::W](chwdat5r::W) writer structure
impl crate::Writable for CHWDAT5R {}
///CHWDAT5R
pub mod chwdat5r;
///CHDATIN5R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin5r](chdatin5r) module
pub type CHDATIN5R = crate::Reg<u32, _CHDATIN5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN5R;
///`read()` method returns [chdatin5r::R](chdatin5r::R) reader structure
impl crate::Readable for CHDATIN5R {}
///`write(|w| ..)` method takes [chdatin5r::W](chdatin5r::W) writer structure
impl crate::Writable for CHDATIN5R {}
///CHDATIN5R
pub mod chdatin5r;
///CHCFG6R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg6r1](chcfg6r1) module
pub type CHCFG6R1 = crate::Reg<u32, _CHCFG6R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG6R1;
///`read()` method returns [chcfg6r1::R](chcfg6r1::R) reader structure
impl crate::Readable for CHCFG6R1 {}
///`write(|w| ..)` method takes [chcfg6r1::W](chcfg6r1::W) writer structure
impl crate::Writable for CHCFG6R1 {}
///CHCFG6R1
pub mod chcfg6r1;
///CHCFG6R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg6r2](chcfg6r2) module
pub type CHCFG6R2 = crate::Reg<u32, _CHCFG6R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG6R2;
///`read()` method returns [chcfg6r2::R](chcfg6r2::R) reader structure
impl crate::Readable for CHCFG6R2 {}
///`write(|w| ..)` method takes [chcfg6r2::W](chcfg6r2::W) writer structure
impl crate::Writable for CHCFG6R2 {}
///CHCFG6R2
pub mod chcfg6r2;
///AWSCD6R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd6r](awscd6r) module
pub type AWSCD6R = crate::Reg<u32, _AWSCD6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD6R;
///`read()` method returns [awscd6r::R](awscd6r::R) reader structure
impl crate::Readable for AWSCD6R {}
///`write(|w| ..)` method takes [awscd6r::W](awscd6r::W) writer structure
impl crate::Writable for AWSCD6R {}
///AWSCD6R
pub mod awscd6r;
///CHWDAT6R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat6r](chwdat6r) module
pub type CHWDAT6R = crate::Reg<u32, _CHWDAT6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT6R;
///`read()` method returns [chwdat6r::R](chwdat6r::R) reader structure
impl crate::Readable for CHWDAT6R {}
///`write(|w| ..)` method takes [chwdat6r::W](chwdat6r::W) writer structure
impl crate::Writable for CHWDAT6R {}
///CHWDAT6R
pub mod chwdat6r;
///CHDATIN6R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin6r](chdatin6r) module
pub type CHDATIN6R = crate::Reg<u32, _CHDATIN6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN6R;
///`read()` method returns [chdatin6r::R](chdatin6r::R) reader structure
impl crate::Readable for CHDATIN6R {}
///`write(|w| ..)` method takes [chdatin6r::W](chdatin6r::W) writer structure
impl crate::Writable for CHDATIN6R {}
///CHDATIN6R
pub mod chdatin6r;
///CHCFG7R1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg7r1](chcfg7r1) module
pub type CHCFG7R1 = crate::Reg<u32, _CHCFG7R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG7R1;
///`read()` method returns [chcfg7r1::R](chcfg7r1::R) reader structure
impl crate::Readable for CHCFG7R1 {}
///`write(|w| ..)` method takes [chcfg7r1::W](chcfg7r1::W) writer structure
impl crate::Writable for CHCFG7R1 {}
///CHCFG7R1
pub mod chcfg7r1;
///CHCFG7R2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chcfg7r2](chcfg7r2) module
pub type CHCFG7R2 = crate::Reg<u32, _CHCFG7R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCFG7R2;
///`read()` method returns [chcfg7r2::R](chcfg7r2::R) reader structure
impl crate::Readable for CHCFG7R2 {}
///`write(|w| ..)` method takes [chcfg7r2::W](chcfg7r2::W) writer structure
impl crate::Writable for CHCFG7R2 {}
///CHCFG7R2
pub mod chcfg7r2;
///AWSCD7R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awscd7r](awscd7r) module
pub type AWSCD7R = crate::Reg<u32, _AWSCD7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCD7R;
///`read()` method returns [awscd7r::R](awscd7r::R) reader structure
impl crate::Readable for AWSCD7R {}
///`write(|w| ..)` method takes [awscd7r::W](awscd7r::W) writer structure
impl crate::Writable for AWSCD7R {}
///AWSCD7R
pub mod awscd7r;
///CHWDAT7R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chwdat7r](chwdat7r) module
pub type CHWDAT7R = crate::Reg<u32, _CHWDAT7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHWDAT7R;
///`read()` method returns [chwdat7r::R](chwdat7r::R) reader structure
impl crate::Readable for CHWDAT7R {}
///`write(|w| ..)` method takes [chwdat7r::W](chwdat7r::W) writer structure
impl crate::Writable for CHWDAT7R {}
///CHWDAT7R
pub mod chwdat7r;
///CHDATIN7R
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chdatin7r](chdatin7r) module
pub type CHDATIN7R = crate::Reg<u32, _CHDATIN7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDATIN7R;
///`read()` method returns [chdatin7r::R](chdatin7r::R) reader structure
impl crate::Readable for CHDATIN7R {}
///`write(|w| ..)` method takes [chdatin7r::W](chdatin7r::W) writer structure
impl crate::Writable for CHDATIN7R {}
///CHDATIN7R
pub mod chdatin7r;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cr1](dfsdm0_cr1) module
pub type DFSDM0_CR1 = crate::Reg<u32, _DFSDM0_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR1;
///`read()` method returns [dfsdm0_cr1::R](dfsdm0_cr1::R) reader structure
impl crate::Readable for DFSDM0_CR1 {}
///`write(|w| ..)` method takes [dfsdm0_cr1::W](dfsdm0_cr1::W) writer structure
impl crate::Writable for DFSDM0_CR1 {}
///control register 1
pub mod dfsdm0_cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cr2](dfsdm0_cr2) module
pub type DFSDM0_CR2 = crate::Reg<u32, _DFSDM0_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR2;
///`read()` method returns [dfsdm0_cr2::R](dfsdm0_cr2::R) reader structure
impl crate::Readable for DFSDM0_CR2 {}
///`write(|w| ..)` method takes [dfsdm0_cr2::W](dfsdm0_cr2::W) writer structure
impl crate::Writable for DFSDM0_CR2 {}
///control register 2
pub mod dfsdm0_cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_isr](dfsdm0_isr) module
pub type DFSDM0_ISR = crate::Reg<u32, _DFSDM0_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ISR;
///`read()` method returns [dfsdm0_isr::R](dfsdm0_isr::R) reader structure
impl crate::Readable for DFSDM0_ISR {}
///interrupt and status register
pub mod dfsdm0_isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_icr](dfsdm0_icr) module
pub type DFSDM0_ICR = crate::Reg<u32, _DFSDM0_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_ICR;
///`read()` method returns [dfsdm0_icr::R](dfsdm0_icr::R) reader structure
impl crate::Readable for DFSDM0_ICR {}
///`write(|w| ..)` method takes [dfsdm0_icr::W](dfsdm0_icr::W) writer structure
impl crate::Writable for DFSDM0_ICR {}
///interrupt flag clear register
pub mod dfsdm0_icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_jchgr](dfsdm0_jchgr) module
pub type DFSDM0_JCHGR = crate::Reg<u32, _DFSDM0_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JCHGR;
///`read()` method returns [dfsdm0_jchgr::R](dfsdm0_jchgr::R) reader structure
impl crate::Readable for DFSDM0_JCHGR {}
///`write(|w| ..)` method takes [dfsdm0_jchgr::W](dfsdm0_jchgr::W) writer structure
impl crate::Writable for DFSDM0_JCHGR {}
///injected channel group selection register
pub mod dfsdm0_jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_fcr](dfsdm0_fcr) module
pub type DFSDM0_FCR = crate::Reg<u32, _DFSDM0_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_FCR;
///`read()` method returns [dfsdm0_fcr::R](dfsdm0_fcr::R) reader structure
impl crate::Readable for DFSDM0_FCR {}
///`write(|w| ..)` method takes [dfsdm0_fcr::W](dfsdm0_fcr::W) writer structure
impl crate::Writable for DFSDM0_FCR {}
///filter control register
pub mod dfsdm0_fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_jdatar](dfsdm0_jdatar) module
pub type DFSDM0_JDATAR = crate::Reg<u32, _DFSDM0_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_JDATAR;
///`read()` method returns [dfsdm0_jdatar::R](dfsdm0_jdatar::R) reader structure
impl crate::Readable for DFSDM0_JDATAR {}
///data register for injected group
pub mod dfsdm0_jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_rdatar](dfsdm0_rdatar) module
pub type DFSDM0_RDATAR = crate::Reg<u32, _DFSDM0_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_RDATAR;
///`read()` method returns [dfsdm0_rdatar::R](dfsdm0_rdatar::R) reader structure
impl crate::Readable for DFSDM0_RDATAR {}
///data register for the regular channel
pub mod dfsdm0_rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awhtr](dfsdm0_awhtr) module
pub type DFSDM0_AWHTR = crate::Reg<u32, _DFSDM0_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWHTR;
///`read()` method returns [dfsdm0_awhtr::R](dfsdm0_awhtr::R) reader structure
impl crate::Readable for DFSDM0_AWHTR {}
///`write(|w| ..)` method takes [dfsdm0_awhtr::W](dfsdm0_awhtr::W) writer structure
impl crate::Writable for DFSDM0_AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm0_awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awltr](dfsdm0_awltr) module
pub type DFSDM0_AWLTR = crate::Reg<u32, _DFSDM0_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWLTR;
///`read()` method returns [dfsdm0_awltr::R](dfsdm0_awltr::R) reader structure
impl crate::Readable for DFSDM0_AWLTR {}
///`write(|w| ..)` method takes [dfsdm0_awltr::W](dfsdm0_awltr::W) writer structure
impl crate::Writable for DFSDM0_AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm0_awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awsr](dfsdm0_awsr) module
pub type DFSDM0_AWSR = crate::Reg<u32, _DFSDM0_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWSR;
///`read()` method returns [dfsdm0_awsr::R](dfsdm0_awsr::R) reader structure
impl crate::Readable for DFSDM0_AWSR {}
///analog watchdog status register
pub mod dfsdm0_awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_awcfr](dfsdm0_awcfr) module
pub type DFSDM0_AWCFR = crate::Reg<u32, _DFSDM0_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_AWCFR;
///`read()` method returns [dfsdm0_awcfr::R](dfsdm0_awcfr::R) reader structure
impl crate::Readable for DFSDM0_AWCFR {}
///`write(|w| ..)` method takes [dfsdm0_awcfr::W](dfsdm0_awcfr::W) writer structure
impl crate::Writable for DFSDM0_AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm0_awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_exmax](dfsdm0_exmax) module
pub type DFSDM0_EXMAX = crate::Reg<u32, _DFSDM0_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMAX;
///`read()` method returns [dfsdm0_exmax::R](dfsdm0_exmax::R) reader structure
impl crate::Readable for DFSDM0_EXMAX {}
///Extremes detector maximum register
pub mod dfsdm0_exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_exmin](dfsdm0_exmin) module
pub type DFSDM0_EXMIN = crate::Reg<u32, _DFSDM0_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_EXMIN;
///`read()` method returns [dfsdm0_exmin::R](dfsdm0_exmin::R) reader structure
impl crate::Readable for DFSDM0_EXMIN {}
///Extremes detector minimum register
pub mod dfsdm0_exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm0_cnvtimr](dfsdm0_cnvtimr) module
pub type DFSDM0_CNVTIMR = crate::Reg<u32, _DFSDM0_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CNVTIMR;
///`read()` method returns [dfsdm0_cnvtimr::R](dfsdm0_cnvtimr::R) reader structure
impl crate::Readable for DFSDM0_CNVTIMR {}
///conversion timer register
pub mod dfsdm0_cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cr1](dfsdm1_cr1) module
pub type DFSDM1_CR1 = crate::Reg<u32, _DFSDM1_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR1;
///`read()` method returns [dfsdm1_cr1::R](dfsdm1_cr1::R) reader structure
impl crate::Readable for DFSDM1_CR1 {}
///`write(|w| ..)` method takes [dfsdm1_cr1::W](dfsdm1_cr1::W) writer structure
impl crate::Writable for DFSDM1_CR1 {}
///control register 1
pub mod dfsdm1_cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cr2](dfsdm1_cr2) module
pub type DFSDM1_CR2 = crate::Reg<u32, _DFSDM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR2;
///`read()` method returns [dfsdm1_cr2::R](dfsdm1_cr2::R) reader structure
impl crate::Readable for DFSDM1_CR2 {}
///`write(|w| ..)` method takes [dfsdm1_cr2::W](dfsdm1_cr2::W) writer structure
impl crate::Writable for DFSDM1_CR2 {}
///control register 2
pub mod dfsdm1_cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_isr](dfsdm1_isr) module
pub type DFSDM1_ISR = crate::Reg<u32, _DFSDM1_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ISR;
///`read()` method returns [dfsdm1_isr::R](dfsdm1_isr::R) reader structure
impl crate::Readable for DFSDM1_ISR {}
///interrupt and status register
pub mod dfsdm1_isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_icr](dfsdm1_icr) module
pub type DFSDM1_ICR = crate::Reg<u32, _DFSDM1_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_ICR;
///`read()` method returns [dfsdm1_icr::R](dfsdm1_icr::R) reader structure
impl crate::Readable for DFSDM1_ICR {}
///`write(|w| ..)` method takes [dfsdm1_icr::W](dfsdm1_icr::W) writer structure
impl crate::Writable for DFSDM1_ICR {}
///interrupt flag clear register
pub mod dfsdm1_icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_jchgr](dfsdm1_jchgr) module
pub type DFSDM1_JCHGR = crate::Reg<u32, _DFSDM1_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JCHGR;
///`read()` method returns [dfsdm1_jchgr::R](dfsdm1_jchgr::R) reader structure
impl crate::Readable for DFSDM1_JCHGR {}
///`write(|w| ..)` method takes [dfsdm1_jchgr::W](dfsdm1_jchgr::W) writer structure
impl crate::Writable for DFSDM1_JCHGR {}
///injected channel group selection register
pub mod dfsdm1_jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_fcr](dfsdm1_fcr) module
pub type DFSDM1_FCR = crate::Reg<u32, _DFSDM1_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_FCR;
///`read()` method returns [dfsdm1_fcr::R](dfsdm1_fcr::R) reader structure
impl crate::Readable for DFSDM1_FCR {}
///`write(|w| ..)` method takes [dfsdm1_fcr::W](dfsdm1_fcr::W) writer structure
impl crate::Writable for DFSDM1_FCR {}
///filter control register
pub mod dfsdm1_fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_jdatar](dfsdm1_jdatar) module
pub type DFSDM1_JDATAR = crate::Reg<u32, _DFSDM1_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_JDATAR;
///`read()` method returns [dfsdm1_jdatar::R](dfsdm1_jdatar::R) reader structure
impl crate::Readable for DFSDM1_JDATAR {}
///data register for injected group
pub mod dfsdm1_jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_rdatar](dfsdm1_rdatar) module
pub type DFSDM1_RDATAR = crate::Reg<u32, _DFSDM1_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_RDATAR;
///`read()` method returns [dfsdm1_rdatar::R](dfsdm1_rdatar::R) reader structure
impl crate::Readable for DFSDM1_RDATAR {}
///data register for the regular channel
pub mod dfsdm1_rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awhtr](dfsdm1_awhtr) module
pub type DFSDM1_AWHTR = crate::Reg<u32, _DFSDM1_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWHTR;
///`read()` method returns [dfsdm1_awhtr::R](dfsdm1_awhtr::R) reader structure
impl crate::Readable for DFSDM1_AWHTR {}
///`write(|w| ..)` method takes [dfsdm1_awhtr::W](dfsdm1_awhtr::W) writer structure
impl crate::Writable for DFSDM1_AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm1_awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awltr](dfsdm1_awltr) module
pub type DFSDM1_AWLTR = crate::Reg<u32, _DFSDM1_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWLTR;
///`read()` method returns [dfsdm1_awltr::R](dfsdm1_awltr::R) reader structure
impl crate::Readable for DFSDM1_AWLTR {}
///`write(|w| ..)` method takes [dfsdm1_awltr::W](dfsdm1_awltr::W) writer structure
impl crate::Writable for DFSDM1_AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm1_awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awsr](dfsdm1_awsr) module
pub type DFSDM1_AWSR = crate::Reg<u32, _DFSDM1_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWSR;
///`read()` method returns [dfsdm1_awsr::R](dfsdm1_awsr::R) reader structure
impl crate::Readable for DFSDM1_AWSR {}
///analog watchdog status register
pub mod dfsdm1_awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_awcfr](dfsdm1_awcfr) module
pub type DFSDM1_AWCFR = crate::Reg<u32, _DFSDM1_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_AWCFR;
///`read()` method returns [dfsdm1_awcfr::R](dfsdm1_awcfr::R) reader structure
impl crate::Readable for DFSDM1_AWCFR {}
///`write(|w| ..)` method takes [dfsdm1_awcfr::W](dfsdm1_awcfr::W) writer structure
impl crate::Writable for DFSDM1_AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm1_awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_exmax](dfsdm1_exmax) module
pub type DFSDM1_EXMAX = crate::Reg<u32, _DFSDM1_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMAX;
///`read()` method returns [dfsdm1_exmax::R](dfsdm1_exmax::R) reader structure
impl crate::Readable for DFSDM1_EXMAX {}
///Extremes detector maximum register
pub mod dfsdm1_exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_exmin](dfsdm1_exmin) module
pub type DFSDM1_EXMIN = crate::Reg<u32, _DFSDM1_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_EXMIN;
///`read()` method returns [dfsdm1_exmin::R](dfsdm1_exmin::R) reader structure
impl crate::Readable for DFSDM1_EXMIN {}
///Extremes detector minimum register
pub mod dfsdm1_exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm1_cnvtimr](dfsdm1_cnvtimr) module
pub type DFSDM1_CNVTIMR = crate::Reg<u32, _DFSDM1_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CNVTIMR;
///`read()` method returns [dfsdm1_cnvtimr::R](dfsdm1_cnvtimr::R) reader structure
impl crate::Readable for DFSDM1_CNVTIMR {}
///conversion timer register
pub mod dfsdm1_cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cr1](dfsdm2_cr1) module
pub type DFSDM2_CR1 = crate::Reg<u32, _DFSDM2_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR1;
///`read()` method returns [dfsdm2_cr1::R](dfsdm2_cr1::R) reader structure
impl crate::Readable for DFSDM2_CR1 {}
///`write(|w| ..)` method takes [dfsdm2_cr1::W](dfsdm2_cr1::W) writer structure
impl crate::Writable for DFSDM2_CR1 {}
///control register 1
pub mod dfsdm2_cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cr2](dfsdm2_cr2) module
pub type DFSDM2_CR2 = crate::Reg<u32, _DFSDM2_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR2;
///`read()` method returns [dfsdm2_cr2::R](dfsdm2_cr2::R) reader structure
impl crate::Readable for DFSDM2_CR2 {}
///`write(|w| ..)` method takes [dfsdm2_cr2::W](dfsdm2_cr2::W) writer structure
impl crate::Writable for DFSDM2_CR2 {}
///control register 2
pub mod dfsdm2_cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_isr](dfsdm2_isr) module
pub type DFSDM2_ISR = crate::Reg<u32, _DFSDM2_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ISR;
///`read()` method returns [dfsdm2_isr::R](dfsdm2_isr::R) reader structure
impl crate::Readable for DFSDM2_ISR {}
///interrupt and status register
pub mod dfsdm2_isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_icr](dfsdm2_icr) module
pub type DFSDM2_ICR = crate::Reg<u32, _DFSDM2_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_ICR;
///`read()` method returns [dfsdm2_icr::R](dfsdm2_icr::R) reader structure
impl crate::Readable for DFSDM2_ICR {}
///`write(|w| ..)` method takes [dfsdm2_icr::W](dfsdm2_icr::W) writer structure
impl crate::Writable for DFSDM2_ICR {}
///interrupt flag clear register
pub mod dfsdm2_icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_jchgr](dfsdm2_jchgr) module
pub type DFSDM2_JCHGR = crate::Reg<u32, _DFSDM2_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JCHGR;
///`read()` method returns [dfsdm2_jchgr::R](dfsdm2_jchgr::R) reader structure
impl crate::Readable for DFSDM2_JCHGR {}
///`write(|w| ..)` method takes [dfsdm2_jchgr::W](dfsdm2_jchgr::W) writer structure
impl crate::Writable for DFSDM2_JCHGR {}
///injected channel group selection register
pub mod dfsdm2_jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_fcr](dfsdm2_fcr) module
pub type DFSDM2_FCR = crate::Reg<u32, _DFSDM2_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_FCR;
///`read()` method returns [dfsdm2_fcr::R](dfsdm2_fcr::R) reader structure
impl crate::Readable for DFSDM2_FCR {}
///`write(|w| ..)` method takes [dfsdm2_fcr::W](dfsdm2_fcr::W) writer structure
impl crate::Writable for DFSDM2_FCR {}
///filter control register
pub mod dfsdm2_fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_jdatar](dfsdm2_jdatar) module
pub type DFSDM2_JDATAR = crate::Reg<u32, _DFSDM2_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_JDATAR;
///`read()` method returns [dfsdm2_jdatar::R](dfsdm2_jdatar::R) reader structure
impl crate::Readable for DFSDM2_JDATAR {}
///data register for injected group
pub mod dfsdm2_jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_rdatar](dfsdm2_rdatar) module
pub type DFSDM2_RDATAR = crate::Reg<u32, _DFSDM2_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_RDATAR;
///`read()` method returns [dfsdm2_rdatar::R](dfsdm2_rdatar::R) reader structure
impl crate::Readable for DFSDM2_RDATAR {}
///data register for the regular channel
pub mod dfsdm2_rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awhtr](dfsdm2_awhtr) module
pub type DFSDM2_AWHTR = crate::Reg<u32, _DFSDM2_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWHTR;
///`read()` method returns [dfsdm2_awhtr::R](dfsdm2_awhtr::R) reader structure
impl crate::Readable for DFSDM2_AWHTR {}
///`write(|w| ..)` method takes [dfsdm2_awhtr::W](dfsdm2_awhtr::W) writer structure
impl crate::Writable for DFSDM2_AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm2_awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awltr](dfsdm2_awltr) module
pub type DFSDM2_AWLTR = crate::Reg<u32, _DFSDM2_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWLTR;
///`read()` method returns [dfsdm2_awltr::R](dfsdm2_awltr::R) reader structure
impl crate::Readable for DFSDM2_AWLTR {}
///`write(|w| ..)` method takes [dfsdm2_awltr::W](dfsdm2_awltr::W) writer structure
impl crate::Writable for DFSDM2_AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm2_awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awsr](dfsdm2_awsr) module
pub type DFSDM2_AWSR = crate::Reg<u32, _DFSDM2_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWSR;
///`read()` method returns [dfsdm2_awsr::R](dfsdm2_awsr::R) reader structure
impl crate::Readable for DFSDM2_AWSR {}
///analog watchdog status register
pub mod dfsdm2_awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awcfr](dfsdm2_awcfr) module
pub type DFSDM2_AWCFR = crate::Reg<u32, _DFSDM2_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_AWCFR;
///`read()` method returns [dfsdm2_awcfr::R](dfsdm2_awcfr::R) reader structure
impl crate::Readable for DFSDM2_AWCFR {}
///`write(|w| ..)` method takes [dfsdm2_awcfr::W](dfsdm2_awcfr::W) writer structure
impl crate::Writable for DFSDM2_AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm2_awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_exmax](dfsdm2_exmax) module
pub type DFSDM2_EXMAX = crate::Reg<u32, _DFSDM2_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMAX;
///`read()` method returns [dfsdm2_exmax::R](dfsdm2_exmax::R) reader structure
impl crate::Readable for DFSDM2_EXMAX {}
///Extremes detector maximum register
pub mod dfsdm2_exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_exmin](dfsdm2_exmin) module
pub type DFSDM2_EXMIN = crate::Reg<u32, _DFSDM2_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_EXMIN;
///`read()` method returns [dfsdm2_exmin::R](dfsdm2_exmin::R) reader structure
impl crate::Readable for DFSDM2_EXMIN {}
///Extremes detector minimum register
pub mod dfsdm2_exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_cnvtimr](dfsdm2_cnvtimr) module
pub type DFSDM2_CNVTIMR = crate::Reg<u32, _DFSDM2_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CNVTIMR;
///`read()` method returns [dfsdm2_cnvtimr::R](dfsdm2_cnvtimr::R) reader structure
impl crate::Readable for DFSDM2_CNVTIMR {}
///conversion timer register
pub mod dfsdm2_cnvtimr;
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cr1](dfsdm3_cr1) module
pub type DFSDM3_CR1 = crate::Reg<u32, _DFSDM3_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR1;
///`read()` method returns [dfsdm3_cr1::R](dfsdm3_cr1::R) reader structure
impl crate::Readable for DFSDM3_CR1 {}
///`write(|w| ..)` method takes [dfsdm3_cr1::W](dfsdm3_cr1::W) writer structure
impl crate::Writable for DFSDM3_CR1 {}
///control register 1
pub mod dfsdm3_cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cr2](dfsdm3_cr2) module
pub type DFSDM3_CR2 = crate::Reg<u32, _DFSDM3_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR2;
///`read()` method returns [dfsdm3_cr2::R](dfsdm3_cr2::R) reader structure
impl crate::Readable for DFSDM3_CR2 {}
///`write(|w| ..)` method takes [dfsdm3_cr2::W](dfsdm3_cr2::W) writer structure
impl crate::Writable for DFSDM3_CR2 {}
///control register 2
pub mod dfsdm3_cr2;
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_isr](dfsdm3_isr) module
pub type DFSDM3_ISR = crate::Reg<u32, _DFSDM3_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ISR;
///`read()` method returns [dfsdm3_isr::R](dfsdm3_isr::R) reader structure
impl crate::Readable for DFSDM3_ISR {}
///interrupt and status register
pub mod dfsdm3_isr;
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_icr](dfsdm3_icr) module
pub type DFSDM3_ICR = crate::Reg<u32, _DFSDM3_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_ICR;
///`read()` method returns [dfsdm3_icr::R](dfsdm3_icr::R) reader structure
impl crate::Readable for DFSDM3_ICR {}
///`write(|w| ..)` method takes [dfsdm3_icr::W](dfsdm3_icr::W) writer structure
impl crate::Writable for DFSDM3_ICR {}
///interrupt flag clear register
pub mod dfsdm3_icr;
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_jchgr](dfsdm3_jchgr) module
pub type DFSDM3_JCHGR = crate::Reg<u32, _DFSDM3_JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JCHGR;
///`read()` method returns [dfsdm3_jchgr::R](dfsdm3_jchgr::R) reader structure
impl crate::Readable for DFSDM3_JCHGR {}
///`write(|w| ..)` method takes [dfsdm3_jchgr::W](dfsdm3_jchgr::W) writer structure
impl crate::Writable for DFSDM3_JCHGR {}
///injected channel group selection register
pub mod dfsdm3_jchgr;
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_fcr](dfsdm3_fcr) module
pub type DFSDM3_FCR = crate::Reg<u32, _DFSDM3_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_FCR;
///`read()` method returns [dfsdm3_fcr::R](dfsdm3_fcr::R) reader structure
impl crate::Readable for DFSDM3_FCR {}
///`write(|w| ..)` method takes [dfsdm3_fcr::W](dfsdm3_fcr::W) writer structure
impl crate::Writable for DFSDM3_FCR {}
///filter control register
pub mod dfsdm3_fcr;
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_jdatar](dfsdm3_jdatar) module
pub type DFSDM3_JDATAR = crate::Reg<u32, _DFSDM3_JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_JDATAR;
///`read()` method returns [dfsdm3_jdatar::R](dfsdm3_jdatar::R) reader structure
impl crate::Readable for DFSDM3_JDATAR {}
///data register for injected group
pub mod dfsdm3_jdatar;
///data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_rdatar](dfsdm3_rdatar) module
pub type DFSDM3_RDATAR = crate::Reg<u32, _DFSDM3_RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_RDATAR;
///`read()` method returns [dfsdm3_rdatar::R](dfsdm3_rdatar::R) reader structure
impl crate::Readable for DFSDM3_RDATAR {}
///data register for the regular channel
pub mod dfsdm3_rdatar;
///analog watchdog high threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awhtr](dfsdm3_awhtr) module
pub type DFSDM3_AWHTR = crate::Reg<u32, _DFSDM3_AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWHTR;
///`read()` method returns [dfsdm3_awhtr::R](dfsdm3_awhtr::R) reader structure
impl crate::Readable for DFSDM3_AWHTR {}
///`write(|w| ..)` method takes [dfsdm3_awhtr::W](dfsdm3_awhtr::W) writer structure
impl crate::Writable for DFSDM3_AWHTR {}
///analog watchdog high threshold register
pub mod dfsdm3_awhtr;
///analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awltr](dfsdm3_awltr) module
pub type DFSDM3_AWLTR = crate::Reg<u32, _DFSDM3_AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWLTR;
///`read()` method returns [dfsdm3_awltr::R](dfsdm3_awltr::R) reader structure
impl crate::Readable for DFSDM3_AWLTR {}
///`write(|w| ..)` method takes [dfsdm3_awltr::W](dfsdm3_awltr::W) writer structure
impl crate::Writable for DFSDM3_AWLTR {}
///analog watchdog low threshold register
pub mod dfsdm3_awltr;
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awsr](dfsdm3_awsr) module
pub type DFSDM3_AWSR = crate::Reg<u32, _DFSDM3_AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWSR;
///`read()` method returns [dfsdm3_awsr::R](dfsdm3_awsr::R) reader structure
impl crate::Readable for DFSDM3_AWSR {}
///analog watchdog status register
pub mod dfsdm3_awsr;
///analog watchdog clear flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_awcfr](dfsdm3_awcfr) module
pub type DFSDM3_AWCFR = crate::Reg<u32, _DFSDM3_AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_AWCFR;
///`read()` method returns [dfsdm3_awcfr::R](dfsdm3_awcfr::R) reader structure
impl crate::Readable for DFSDM3_AWCFR {}
///`write(|w| ..)` method takes [dfsdm3_awcfr::W](dfsdm3_awcfr::W) writer structure
impl crate::Writable for DFSDM3_AWCFR {}
///analog watchdog clear flag register
pub mod dfsdm3_awcfr;
///Extremes detector maximum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_exmax](dfsdm3_exmax) module
pub type DFSDM3_EXMAX = crate::Reg<u32, _DFSDM3_EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMAX;
///`read()` method returns [dfsdm3_exmax::R](dfsdm3_exmax::R) reader structure
impl crate::Readable for DFSDM3_EXMAX {}
///Extremes detector maximum register
pub mod dfsdm3_exmax;
///Extremes detector minimum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_exmin](dfsdm3_exmin) module
pub type DFSDM3_EXMIN = crate::Reg<u32, _DFSDM3_EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_EXMIN;
///`read()` method returns [dfsdm3_exmin::R](dfsdm3_exmin::R) reader structure
impl crate::Readable for DFSDM3_EXMIN {}
///Extremes detector minimum register
pub mod dfsdm3_exmin;
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm3_cnvtimr](dfsdm3_cnvtimr) module
pub type DFSDM3_CNVTIMR = crate::Reg<u32, _DFSDM3_CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CNVTIMR;
///`read()` method returns [dfsdm3_cnvtimr::R](dfsdm3_cnvtimr::R) reader structure
impl crate::Readable for DFSDM3_CNVTIMR {}
///conversion timer register
pub mod dfsdm3_cnvtimr;
