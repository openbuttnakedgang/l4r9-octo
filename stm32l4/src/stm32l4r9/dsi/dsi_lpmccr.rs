///Reader of register DSI_LPMCCR
pub type R = crate::R<u32, super::DSI_LPMCCR>;
///Reader of field `VLPSIZE`
pub type VLPSIZE_R = crate::R<u8, u8>;
///Reader of field `LPSIZE`
pub type LPSIZE_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - VACT Largest Packet Size
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Largest Packet Size
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
