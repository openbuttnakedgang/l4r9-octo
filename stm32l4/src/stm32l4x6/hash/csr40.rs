///Reader of register CSR40
pub type R = crate::R<u32, super::CSR40>;
///Writer for register CSR40
pub type W = crate::W<u32, super::CSR40>;
///Register CSR40 `reset()`'s with value 0
impl crate::ResetValue for super::CSR40 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR40`
pub type CSR40_R = crate::R<u32, u32>;
///Write proxy for field `CSR40`
pub struct CSR40_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR40_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR40
    #[inline(always)]
    pub fn csr40(&self) -> CSR40_R {
        CSR40_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR40
    #[inline(always)]
    pub fn csr40(&mut self) -> CSR40_W {
        CSR40_W { w: self }
    }
}
