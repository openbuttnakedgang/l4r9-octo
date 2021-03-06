///Reader of register PUCRH
pub type R = crate::R<u32, super::PUCRH>;
///Writer for register PUCRH
pub type W = crate::W<u32, super::PUCRH>;
///Register PUCRH `reset()`'s with value 0
impl crate::ResetValue for super::PUCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PU1`
pub type PU1_R = crate::R<bool, bool>;
///Write proxy for field `PU1`
pub struct PU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PU1_W<'a> {
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
///Reader of field `PU0`
pub type PU0_R = crate::R<bool, bool>;
///Write proxy for field `PU0`
pub struct PU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PU0_W<'a> {
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
    ///Bit 1 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W {
        PU1_W { w: self }
    }
    ///Bit 0 - Port H pull-up bit y (y=0..1)
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W {
        PU0_W { w: self }
    }
}
