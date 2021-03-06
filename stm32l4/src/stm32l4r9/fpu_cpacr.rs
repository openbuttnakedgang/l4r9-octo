///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Coprocessor access control register
    pub cpacr: CPACR,
}
///Coprocessor access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpacr](cpacr) module
pub type CPACR = crate::Reg<u32, _CPACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPACR;
///`read()` method returns [cpacr::R](cpacr::R) reader structure
impl crate::Readable for CPACR {}
///`write(|w| ..)` method takes [cpacr::W](cpacr::W) writer structure
impl crate::Writable for CPACR {}
///Coprocessor access control register
pub mod cpacr;
