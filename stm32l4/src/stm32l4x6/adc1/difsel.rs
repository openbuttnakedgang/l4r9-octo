///Reader of register DIFSEL
pub type R = crate::R<u32, super::DIFSEL>;
///Writer for register DIFSEL
pub type W = crate::W<u32, super::DIFSEL>;
///Register DIFSEL `reset()`'s with value 0
impl crate::ResetValue for super::DIFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DIFSEL_0`
pub type DIFSEL_0_R = crate::R<bool, bool>;
///Reader of field `DIFSEL_1_15`
pub type DIFSEL_1_15_R = crate::R<u16, u16>;
///Write proxy for field `DIFSEL_1_15`
pub struct DIFSEL_1_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_1_15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | (((value as u32) & 0x7fff) << 1);
        self.w
    }
}
///Reader of field `DIFSEL_16_18`
pub type DIFSEL_16_18_R = crate::R<u8, u8>;
impl R {
    ///Bit 0 - Differential mode for channels 0
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:15 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    ///Bits 16:18 - Differential mode for channels 18 to 16
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bits 1:15 - Differential mode for channels 15 to 1
    #[inline(always)]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W {
        DIFSEL_1_15_W { w: self }
    }
}
