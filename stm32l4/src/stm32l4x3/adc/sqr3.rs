///Reader of register SQR3
pub type R = crate::R<u32, super::SQR3>;
///Writer for register SQR3
pub type W = crate::W<u32, super::SQR3>;
///Register SQR3 `reset()`'s with value 0
impl crate::ResetValue for super::SQR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SQ14`
pub type SQ14_R = crate::R<u8, u8>;
///Write proxy for field `SQ14`
pub struct SQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ14_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
///Reader of field `SQ13`
pub type SQ13_R = crate::R<u8, u8>;
///Write proxy for field `SQ13`
pub struct SQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ13_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
///Reader of field `SQ12`
pub type SQ12_R = crate::R<u8, u8>;
///Write proxy for field `SQ12`
pub struct SQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
///Reader of field `SQ11`
pub type SQ11_R = crate::R<u8, u8>;
///Write proxy for field `SQ11`
pub struct SQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ11_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
///Reader of field `SQ10`
pub type SQ10_R = crate::R<u8, u8>;
///Write proxy for field `SQ10`
pub struct SQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ10_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 24:28 - SQ14
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bits 18:22 - SQ13
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 12:16 - SQ12
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 6:10 - SQ11
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 0:4 - SQ10
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 24:28 - SQ14
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W {
        SQ14_W { w: self }
    }
    ///Bits 18:22 - SQ13
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W {
        SQ13_W { w: self }
    }
    ///Bits 12:16 - SQ12
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W {
        SQ12_W { w: self }
    }
    ///Bits 6:10 - SQ11
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W {
        SQ11_W { w: self }
    }
    ///Bits 0:4 - SQ10
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W {
        SQ10_W { w: self }
    }
}
