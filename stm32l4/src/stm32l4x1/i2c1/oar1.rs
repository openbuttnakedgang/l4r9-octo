///Reader of register OAR1
pub type R = crate::R<u32, super::OAR1>;
///Writer for register OAR1
pub type W = crate::W<u32, super::OAR1>;
///Register OAR1 `reset()`'s with value 0
impl crate::ResetValue for super::OAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OA1`
pub type OA1_R = crate::R<u16, u16>;
///Write proxy for field `OA1`
pub struct OA1_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
///Own Address 1 10-bit mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1MODE_A {
    ///0: Own address 1 is a 7-bit address
    BIT7 = 0,
    ///1: Own address 1 is a 10-bit address
    BIT10 = 1,
}
impl From<OA1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OA1MODE`
pub type OA1MODE_R = crate::R<bool, OA1MODE_A>;
impl OA1MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OA1MODE_A {
        match self.bits {
            false => OA1MODE_A::BIT7,
            true => OA1MODE_A::BIT10,
        }
    }
    ///Checks if the value of the field is `BIT7`
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == OA1MODE_A::BIT7
    }
    ///Checks if the value of the field is `BIT10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == OA1MODE_A::BIT10
    }
}
///Write proxy for field `OA1MODE`
pub struct OA1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1MODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OA1MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Own address 1 is a 7-bit address
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(OA1MODE_A::BIT7)
    }
    ///Own address 1 is a 10-bit address
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(OA1MODE_A::BIT10)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Own Address 1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1EN_A {
    ///0: Own address 1 disabled. The received slave address OA1 is NACKed
    DISABLED = 0,
    ///1: Own address 1 enabled. The received slave address OA1 is ACKed
    ENABLED = 1,
}
impl From<OA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA1EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OA1EN`
pub type OA1EN_R = crate::R<bool, OA1EN_A>;
impl OA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OA1EN_A {
        match self.bits {
            false => OA1EN_A::DISABLED,
            true => OA1EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA1EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA1EN_A::ENABLED
    }
}
///Write proxy for field `OA1EN`
pub struct OA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OA1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Own address 1 disabled. The received slave address OA1 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA1EN_A::DISABLED)
    }
    ///Own address 1 enabled. The received slave address OA1 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA1EN_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W {
        OA1_W { w: self }
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W {
        OA1MODE_W { w: self }
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W {
        OA1EN_W { w: self }
    }
}
