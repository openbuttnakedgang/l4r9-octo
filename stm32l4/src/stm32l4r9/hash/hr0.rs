///Reader of register HR0
pub type R = crate::R<u32, super::HR0>;
///Reader of field `H0`
pub type H0_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - H0
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
