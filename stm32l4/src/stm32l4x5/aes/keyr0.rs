///Reader of register KEYR0
pub type R = crate::R<u32, super::KEYR0>;
///Writer for register KEYR0
pub type W = crate::W<u32, super::KEYR0>;
///Register KEYR0 `reset()`'s with value 0
impl crate::ResetValue for super::KEYR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AES_KEYR0`
pub type AES_KEYR0_R = crate::R<u32, u32>;
///Write proxy for field `AES_KEYR0`
pub struct AES_KEYR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Data Output Register (LSB key \[31:0\])
    #[inline(always)]
    pub fn aes_keyr0(&self) -> AES_KEYR0_R {
        AES_KEYR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Data Output Register (LSB key \[31:0\])
    #[inline(always)]
    pub fn aes_keyr0(&mut self) -> AES_KEYR0_W {
        AES_KEYR0_W { w: self }
    }
}
