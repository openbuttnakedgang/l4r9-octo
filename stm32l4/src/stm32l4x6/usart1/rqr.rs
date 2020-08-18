///Writer for register RQR
pub type W = crate::W<u32, super::RQR>;
///Register RQR `reset()`'s with value 0
impl crate::ResetValue for super::RQR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Transmit data flush request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRQ_AW {
    ///1: Set the TXE flags. This allows to discard the transmit data
    DISCARD = 1,
}
impl From<TXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `TXFRQ`
pub struct TXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFRQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Set the TXE flags. This allows to discard the transmit data
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQ_AW::DISCARD)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Receive data flush request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_AW {
    ///1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    DISCARD = 1,
}
impl From<RXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `RXFRQ`
pub struct RXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFRQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_AW::DISCARD)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Mute mode request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_AW {
    ///1: Puts the USART in mute mode and sets the RWU flag
    MUTE = 1,
}
impl From<MMRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `MMRQ`
pub struct MMRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MMRQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Puts the USART in mute mode and sets the RWU flag
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_AW::MUTE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Send break request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_AW {
    ///1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    BREAK = 1,
}
impl From<SBKRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `SBKRQ`
pub struct SBKRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SBKRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SBKRQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_AW::BREAK)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Auto baud rate request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQ_AW {
    ///1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame
    REQUEST = 1,
}
impl From<ABRRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ABRRQ`
pub struct ABRRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ABRRQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQ_AW::REQUEST)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    ///Bit 4 - Transmit data flush request
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W {
        TXFRQ_W { w: self }
    }
    ///Bit 3 - Receive data flush request
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W {
        RXFRQ_W { w: self }
    }
    ///Bit 2 - Mute mode request
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W {
        MMRQ_W { w: self }
    }
    ///Bit 1 - Send break request
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W {
        SBKRQ_W { w: self }
    }
    ///Bit 0 - Auto baud rate request
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W {
        ABRRQ_W { w: self }
    }
}
