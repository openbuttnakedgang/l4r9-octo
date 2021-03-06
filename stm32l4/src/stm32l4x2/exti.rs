///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt mask register
    pub imr1: IMR1,
    ///0x04 - Event mask register
    pub emr1: EMR1,
    ///0x08 - Rising Trigger selection register
    pub rtsr1: RTSR1,
    ///0x0c - Falling Trigger selection register
    pub ftsr1: FTSR1,
    ///0x10 - Software interrupt event register
    pub swier1: SWIER1,
    ///0x14 - Pending register
    pub pr1: PR1,
    _reserved6: [u8; 8usize],
    ///0x20 - Interrupt mask register
    pub imr2: IMR2,
    ///0x24 - Event mask register
    pub emr2: EMR2,
    ///0x28 - Rising Trigger selection register
    pub rtsr2: RTSR2,
    ///0x2c - Falling Trigger selection register
    pub ftsr2: FTSR2,
    ///0x30 - Software interrupt event register
    pub swier2: SWIER2,
    ///0x34 - Pending register
    pub pr2: PR2,
}
///Interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr1](imr1) module
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
///`read()` method returns [imr1::R](imr1::R) reader structure
impl crate::Readable for IMR1 {}
///`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure
impl crate::Writable for IMR1 {}
///Interrupt mask register
pub mod imr1;
///Event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr1](emr1) module
pub type EMR1 = crate::Reg<u32, _EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR1;
///`read()` method returns [emr1::R](emr1::R) reader structure
impl crate::Readable for EMR1 {}
///`write(|w| ..)` method takes [emr1::W](emr1::W) writer structure
impl crate::Writable for EMR1 {}
///Event mask register
pub mod emr1;
///Rising Trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr1](rtsr1) module
pub type RTSR1 = crate::Reg<u32, _RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR1;
///`read()` method returns [rtsr1::R](rtsr1::R) reader structure
impl crate::Readable for RTSR1 {}
///`write(|w| ..)` method takes [rtsr1::W](rtsr1::W) writer structure
impl crate::Writable for RTSR1 {}
///Rising Trigger selection register
pub mod rtsr1;
///Falling Trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr1](ftsr1) module
pub type FTSR1 = crate::Reg<u32, _FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR1;
///`read()` method returns [ftsr1::R](ftsr1::R) reader structure
impl crate::Readable for FTSR1 {}
///`write(|w| ..)` method takes [ftsr1::W](ftsr1::W) writer structure
impl crate::Writable for FTSR1 {}
///Falling Trigger selection register
pub mod ftsr1;
///Software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier1](swier1) module
pub type SWIER1 = crate::Reg<u32, _SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER1;
///`read()` method returns [swier1::R](swier1::R) reader structure
impl crate::Readable for SWIER1 {}
///`write(|w| ..)` method takes [swier1::W](swier1::W) writer structure
impl crate::Writable for SWIER1 {}
///Software interrupt event register
pub mod swier1;
///Pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr1](pr1) module
pub type PR1 = crate::Reg<u32, _PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1;
///`read()` method returns [pr1::R](pr1::R) reader structure
impl crate::Readable for PR1 {}
///`write(|w| ..)` method takes [pr1::W](pr1::W) writer structure
impl crate::Writable for PR1 {}
///Pending register
pub mod pr1;
///Interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr2](imr2) module
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
///`read()` method returns [imr2::R](imr2::R) reader structure
impl crate::Readable for IMR2 {}
///`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure
impl crate::Writable for IMR2 {}
///Interrupt mask register
pub mod imr2;
///Event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr2](emr2) module
pub type EMR2 = crate::Reg<u32, _EMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR2;
///`read()` method returns [emr2::R](emr2::R) reader structure
impl crate::Readable for EMR2 {}
///`write(|w| ..)` method takes [emr2::W](emr2::W) writer structure
impl crate::Writable for EMR2 {}
///Event mask register
pub mod emr2;
///Rising Trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr2](rtsr2) module
pub type RTSR2 = crate::Reg<u32, _RTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR2;
///`read()` method returns [rtsr2::R](rtsr2::R) reader structure
impl crate::Readable for RTSR2 {}
///`write(|w| ..)` method takes [rtsr2::W](rtsr2::W) writer structure
impl crate::Writable for RTSR2 {}
///Rising Trigger selection register
pub mod rtsr2;
///Falling Trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr2](ftsr2) module
pub type FTSR2 = crate::Reg<u32, _FTSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR2;
///`read()` method returns [ftsr2::R](ftsr2::R) reader structure
impl crate::Readable for FTSR2 {}
///`write(|w| ..)` method takes [ftsr2::W](ftsr2::W) writer structure
impl crate::Writable for FTSR2 {}
///Falling Trigger selection register
pub mod ftsr2;
///Software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier2](swier2) module
pub type SWIER2 = crate::Reg<u32, _SWIER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER2;
///`read()` method returns [swier2::R](swier2::R) reader structure
impl crate::Readable for SWIER2 {}
///`write(|w| ..)` method takes [swier2::W](swier2::W) writer structure
impl crate::Writable for SWIER2 {}
///Software interrupt event register
pub mod swier2;
///Pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr2](pr2) module
pub type PR2 = crate::Reg<u32, _PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2;
///`read()` method returns [pr2::R](pr2::R) reader structure
impl crate::Readable for PR2 {}
///`write(|w| ..)` method takes [pr2::W](pr2::W) writer structure
impl crate::Writable for PR2 {}
///Pending register
pub mod pr2;
