///Reader of register SCSR
pub type R = crate::R<u32, super::SCSR>;
///Writer for register SCSR
pub type W = crate::W<u32, super::SCSR>;
///Register SCSR `reset()`'s with value 0
impl crate::ResetValue for super::SCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SRAM2BSY`
pub type SRAM2BSY_R = crate::R<bool, bool>;
///Reader of field `SRAM2ER`
pub type SRAM2ER_R = crate::R<bool, bool>;
///Write proxy for field `SRAM2ER`
pub struct SRAM2ER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2ER_W<'a> {
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
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W {
        SRAM2ER_W { w: self }
    }
}
