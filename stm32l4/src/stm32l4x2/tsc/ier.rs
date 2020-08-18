///Reader of register IER
pub type R = crate::R<u32, super::IER>;
///Writer for register IER
pub type W = crate::W<u32, super::IER>;
///Register IER `reset()`'s with value 0
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MCEIE`
pub type MCEIE_R = crate::R<bool, bool>;
///Write proxy for field `MCEIE`
pub struct MCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEIE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `EOAIE`
pub type EOAIE_R = crate::R<bool, bool>;
///Write proxy for field `EOAIE`
pub struct EOAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOAIE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&mut self) -> MCEIE_W {
        MCEIE_W { w: self }
    }
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&mut self) -> EOAIE_W {
        EOAIE_W { w: self }
    }
}
