///Reader of register CSR
pub type R = crate::R<u32, super::CSR>;
///Writer for register CSR
pub type W = crate::W<u32, super::CSR>;
///Register CSR `reset()`'s with value 0x02
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
///Reader of field `ENVR`
pub type ENVR_R = crate::R<bool, bool>;
///Write proxy for field `ENVR`
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
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
///Reader of field `HIZ`
pub type HIZ_R = crate::R<bool, bool>;
///Write proxy for field `HIZ`
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
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
///Reader of field `VRS`
pub type VRS_R = crate::R<bool, bool>;
///Write proxy for field `VRS`
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `VRR`
pub type VRR_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Voltage reference buffer enable
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Voltage reference buffer ready
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Voltage reference buffer enable
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
}
