///Reader of register CSR26
pub type R = crate::R<u32, super::CSR26>;
///Writer for register CSR26
pub type W = crate::W<u32, super::CSR26>;
///Register CSR26 `reset()`'s with value 0
impl crate::ResetValue for super::CSR26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR26`
pub type CSR26_R = crate::R<u32, u32>;
///Write proxy for field `CSR26`
pub struct CSR26_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR26_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR26
    #[inline(always)]
    pub fn csr26(&self) -> CSR26_R {
        CSR26_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR26
    #[inline(always)]
    pub fn csr26(&mut self) -> CSR26_W {
        CSR26_W { w: self }
    }
}
