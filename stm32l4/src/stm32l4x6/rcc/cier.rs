///Reader of register CIER
pub type R = crate::R<u32, super::CIER>;
///Writer for register CIER
pub type W = crate::W<u32, super::CIER>;
///Register CIER `reset()`'s with value 0
impl crate::ResetValue for super::CIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LSECSSIE`
pub type LSECSSIE_R = crate::R<bool, bool>;
///Write proxy for field `LSECSSIE`
pub struct LSECSSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `PLLSAI2RDYIE`
pub type PLLSAI2RDYIE_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI2RDYIE`
pub struct PLLSAI2RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2RDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Reader of field `PLLSAI1RDYIE`
pub type PLLSAI1RDYIE_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI1RDYIE`
pub struct PLLSAI1RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1RDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `PLLRDYIE`
pub type PLLRDYIE_R = crate::R<bool, bool>;
///Write proxy for field `PLLRDYIE`
pub struct PLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `HSERDYIE`
pub type HSERDYIE_R = crate::R<bool, bool>;
///Write proxy for field `HSERDYIE`
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `HSIRDYIE`
pub type HSIRDYIE_R = crate::R<bool, bool>;
///Write proxy for field `HSIRDYIE`
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `MSIRDYIE`
pub type MSIRDYIE_R = crate::R<bool, bool>;
///Write proxy for field `MSIRDYIE`
pub struct MSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRDYIE_W<'a> {
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
///Reader of field `LSERDYIE`
pub type LSERDYIE_R = crate::R<bool, bool>;
///Write proxy for field `LSERDYIE`
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
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
///Reader of field `LSIRDYIE`
pub type LSIRDYIE_R = crate::R<bool, bool>;
///Write proxy for field `LSIRDYIE`
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
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
///Reader of field `HSI48RDYIE`
pub type HSI48RDYIE_R = crate::R<bool, bool>;
///Write proxy for field `HSI48RDYIE`
pub struct HSI48RDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48RDYIE_W<'a> {
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
impl R {
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 7 - PLLSAI2 ready interrupt enable
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W {
        LSECSSIE_W { w: self }
    }
    ///Bit 7 - PLLSAI2 ready interrupt enable
    #[inline(always)]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W {
        PLLSAI2RDYIE_W { w: self }
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W {
        PLLSAI1RDYIE_W { w: self }
    }
    ///Bit 5 - PLL ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W { w: self }
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W {
        MSIRDYIE_W { w: self }
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    ///Bit 0 - LSI ready interrupt enable
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W {
        HSI48RDYIE_W { w: self }
    }
}
