///Reader of register AFRL
pub type R = crate::R<u32, super::AFRL>;
///Writer for register AFRL
pub type W = crate::W<u32, super::AFRL>;
///Register AFRL `reset()`'s with value 0
impl crate::ResetValue for super::AFRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Alternate function selection for port x bit y (y = 0..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFRL7_A {
    ///0: AF0
    AF0 = 0,
    ///1: AF1
    AF1 = 1,
    ///2: AF2
    AF2 = 2,
    ///3: AF3
    AF3 = 3,
    ///4: AF4
    AF4 = 4,
    ///5: AF5
    AF5 = 5,
    ///6: AF6
    AF6 = 6,
    ///7: AF7
    AF7 = 7,
    ///8: AF8
    AF8 = 8,
    ///9: AF9
    AF9 = 9,
    ///10: AF10
    AF10 = 10,
    ///11: AF11
    AF11 = 11,
    ///12: AF12
    AF12 = 12,
    ///13: AF13
    AF13 = 13,
    ///14: AF14
    AF14 = 14,
    ///15: AF15
    AF15 = 15,
}
impl From<AFRL7_A> for u8 {
    #[inline(always)]
    fn from(variant: AFRL7_A) -> Self {
        variant as _
    }
}
///Reader of field `AFRL7`
pub type AFRL7_R = crate::R<u8, AFRL7_A>;
impl AFRL7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFRL7_A {
        match self.bits {
            0 => AFRL7_A::AF0,
            1 => AFRL7_A::AF1,
            2 => AFRL7_A::AF2,
            3 => AFRL7_A::AF3,
            4 => AFRL7_A::AF4,
            5 => AFRL7_A::AF5,
            6 => AFRL7_A::AF6,
            7 => AFRL7_A::AF7,
            8 => AFRL7_A::AF8,
            9 => AFRL7_A::AF9,
            10 => AFRL7_A::AF10,
            11 => AFRL7_A::AF11,
            12 => AFRL7_A::AF12,
            13 => AFRL7_A::AF13,
            14 => AFRL7_A::AF14,
            15 => AFRL7_A::AF15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `AF0`
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFRL7_A::AF0
    }
    ///Checks if the value of the field is `AF1`
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFRL7_A::AF1
    }
    ///Checks if the value of the field is `AF2`
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFRL7_A::AF2
    }
    ///Checks if the value of the field is `AF3`
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFRL7_A::AF3
    }
    ///Checks if the value of the field is `AF4`
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFRL7_A::AF4
    }
    ///Checks if the value of the field is `AF5`
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFRL7_A::AF5
    }
    ///Checks if the value of the field is `AF6`
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFRL7_A::AF6
    }
    ///Checks if the value of the field is `AF7`
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFRL7_A::AF7
    }
    ///Checks if the value of the field is `AF8`
    #[inline(always)]
    pub fn is_af8(&self) -> bool {
        *self == AFRL7_A::AF8
    }
    ///Checks if the value of the field is `AF9`
    #[inline(always)]
    pub fn is_af9(&self) -> bool {
        *self == AFRL7_A::AF9
    }
    ///Checks if the value of the field is `AF10`
    #[inline(always)]
    pub fn is_af10(&self) -> bool {
        *self == AFRL7_A::AF10
    }
    ///Checks if the value of the field is `AF11`
    #[inline(always)]
    pub fn is_af11(&self) -> bool {
        *self == AFRL7_A::AF11
    }
    ///Checks if the value of the field is `AF12`
    #[inline(always)]
    pub fn is_af12(&self) -> bool {
        *self == AFRL7_A::AF12
    }
    ///Checks if the value of the field is `AF13`
    #[inline(always)]
    pub fn is_af13(&self) -> bool {
        *self == AFRL7_A::AF13
    }
    ///Checks if the value of the field is `AF14`
    #[inline(always)]
    pub fn is_af14(&self) -> bool {
        *self == AFRL7_A::AF14
    }
    ///Checks if the value of the field is `AF15`
    #[inline(always)]
    pub fn is_af15(&self) -> bool {
        *self == AFRL7_A::AF15
    }
}
///Write proxy for field `AFRL7`
pub struct AFRL7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL6_A = AFRL7_A;
///Reader of field `AFRL6`
pub type AFRL6_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL6`
pub struct AFRL6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL5_A = AFRL7_A;
///Reader of field `AFRL5`
pub type AFRL5_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL5`
pub struct AFRL5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL4_A = AFRL7_A;
///Reader of field `AFRL4`
pub type AFRL4_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL4`
pub struct AFRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL3_A = AFRL7_A;
///Reader of field `AFRL3`
pub type AFRL3_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL3`
pub struct AFRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL2_A = AFRL7_A;
///Reader of field `AFRL2`
pub type AFRL2_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL2`
pub struct AFRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL1_A = AFRL7_A;
///Reader of field `AFRL1`
pub type AFRL1_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL1`
pub struct AFRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Alternate function selection for port x bit y (y = 0..7)
pub type AFRL0_A = AFRL7_A;
///Reader of field `AFRL0`
pub type AFRL0_R = crate::R<u8, AFRL7_A>;
///Write proxy for field `AFRL0`
pub struct AFRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFRL0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFRL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFRL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFRL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFRL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFRL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFRL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFRL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFRL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFRL7_A::AF7)
    }
    ///AF8
    #[inline(always)]
    pub fn af8(self) -> &'a mut W {
        self.variant(AFRL7_A::AF8)
    }
    ///AF9
    #[inline(always)]
    pub fn af9(self) -> &'a mut W {
        self.variant(AFRL7_A::AF9)
    }
    ///AF10
    #[inline(always)]
    pub fn af10(self) -> &'a mut W {
        self.variant(AFRL7_A::AF10)
    }
    ///AF11
    #[inline(always)]
    pub fn af11(self) -> &'a mut W {
        self.variant(AFRL7_A::AF11)
    }
    ///AF12
    #[inline(always)]
    pub fn af12(self) -> &'a mut W {
        self.variant(AFRL7_A::AF12)
    }
    ///AF13
    #[inline(always)]
    pub fn af13(self) -> &'a mut W {
        self.variant(AFRL7_A::AF13)
    }
    ///AF14
    #[inline(always)]
    pub fn af14(self) -> &'a mut W {
        self.variant(AFRL7_A::AF14)
    }
    ///AF15
    #[inline(always)]
    pub fn af15(self) -> &'a mut W {
        self.variant(AFRL7_A::AF15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl7(&mut self) -> AFRL7_W {
        AFRL7_W { w: self }
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl6(&mut self) -> AFRL6_W {
        AFRL6_W { w: self }
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl5(&mut self) -> AFRL5_W {
        AFRL5_W { w: self }
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl4(&mut self) -> AFRL4_W {
        AFRL4_W { w: self }
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl3(&mut self) -> AFRL3_W {
        AFRL3_W { w: self }
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl2(&mut self) -> AFRL2_W {
        AFRL2_W { w: self }
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl1(&mut self) -> AFRL1_W {
        AFRL1_W { w: self }
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afrl0(&mut self) -> AFRL0_W {
        AFRL0_W { w: self }
    }
}
