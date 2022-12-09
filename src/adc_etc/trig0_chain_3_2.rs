#[doc = "Register `TRIG0_CHAIN_3_2` reader"]
pub struct R(crate::R<TRIG0_CHAIN_3_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG0_CHAIN_3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG0_CHAIN_3_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG0_CHAIN_3_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG0_CHAIN_3_2` writer"]
pub struct W(crate::W<TRIG0_CHAIN_3_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG0_CHAIN_3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TRIG0_CHAIN_3_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG0_CHAIN_3_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL2` reader - ADC channel selection"]
pub type CSEL2_R = crate::FieldReader<u8, CSEL2_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL2_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL2_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL2_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL2_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL2_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL2_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL2_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL2_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL2_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL2_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL2_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL2_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL2_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL2_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL2_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL2_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL2_15 = 15,
}
impl From<CSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL2_A) -> Self {
        variant as _
    }
}
impl CSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL2_A {
        match self.bits {
            0 => CSEL2_A::CSEL2_0,
            1 => CSEL2_A::CSEL2_1,
            2 => CSEL2_A::CSEL2_2,
            3 => CSEL2_A::CSEL2_3,
            4 => CSEL2_A::CSEL2_4,
            5 => CSEL2_A::CSEL2_5,
            6 => CSEL2_A::CSEL2_6,
            7 => CSEL2_A::CSEL2_7,
            8 => CSEL2_A::CSEL2_8,
            9 => CSEL2_A::CSEL2_9,
            10 => CSEL2_A::CSEL2_10,
            11 => CSEL2_A::CSEL2_11,
            12 => CSEL2_A::CSEL2_12,
            13 => CSEL2_A::CSEL2_13,
            14 => CSEL2_A::CSEL2_14,
            15 => CSEL2_A::CSEL2_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL2_0`"]
    #[inline(always)]
    pub fn is_csel2_0(&self) -> bool {
        *self == CSEL2_A::CSEL2_0
    }
    #[doc = "Checks if the value of the field is `CSEL2_1`"]
    #[inline(always)]
    pub fn is_csel2_1(&self) -> bool {
        *self == CSEL2_A::CSEL2_1
    }
    #[doc = "Checks if the value of the field is `CSEL2_2`"]
    #[inline(always)]
    pub fn is_csel2_2(&self) -> bool {
        *self == CSEL2_A::CSEL2_2
    }
    #[doc = "Checks if the value of the field is `CSEL2_3`"]
    #[inline(always)]
    pub fn is_csel2_3(&self) -> bool {
        *self == CSEL2_A::CSEL2_3
    }
    #[doc = "Checks if the value of the field is `CSEL2_4`"]
    #[inline(always)]
    pub fn is_csel2_4(&self) -> bool {
        *self == CSEL2_A::CSEL2_4
    }
    #[doc = "Checks if the value of the field is `CSEL2_5`"]
    #[inline(always)]
    pub fn is_csel2_5(&self) -> bool {
        *self == CSEL2_A::CSEL2_5
    }
    #[doc = "Checks if the value of the field is `CSEL2_6`"]
    #[inline(always)]
    pub fn is_csel2_6(&self) -> bool {
        *self == CSEL2_A::CSEL2_6
    }
    #[doc = "Checks if the value of the field is `CSEL2_7`"]
    #[inline(always)]
    pub fn is_csel2_7(&self) -> bool {
        *self == CSEL2_A::CSEL2_7
    }
    #[doc = "Checks if the value of the field is `CSEL2_8`"]
    #[inline(always)]
    pub fn is_csel2_8(&self) -> bool {
        *self == CSEL2_A::CSEL2_8
    }
    #[doc = "Checks if the value of the field is `CSEL2_9`"]
    #[inline(always)]
    pub fn is_csel2_9(&self) -> bool {
        *self == CSEL2_A::CSEL2_9
    }
    #[doc = "Checks if the value of the field is `CSEL2_10`"]
    #[inline(always)]
    pub fn is_csel2_10(&self) -> bool {
        *self == CSEL2_A::CSEL2_10
    }
    #[doc = "Checks if the value of the field is `CSEL2_11`"]
    #[inline(always)]
    pub fn is_csel2_11(&self) -> bool {
        *self == CSEL2_A::CSEL2_11
    }
    #[doc = "Checks if the value of the field is `CSEL2_12`"]
    #[inline(always)]
    pub fn is_csel2_12(&self) -> bool {
        *self == CSEL2_A::CSEL2_12
    }
    #[doc = "Checks if the value of the field is `CSEL2_13`"]
    #[inline(always)]
    pub fn is_csel2_13(&self) -> bool {
        *self == CSEL2_A::CSEL2_13
    }
    #[doc = "Checks if the value of the field is `CSEL2_14`"]
    #[inline(always)]
    pub fn is_csel2_14(&self) -> bool {
        *self == CSEL2_A::CSEL2_14
    }
    #[doc = "Checks if the value of the field is `CSEL2_15`"]
    #[inline(always)]
    pub fn is_csel2_15(&self) -> bool {
        *self == CSEL2_A::CSEL2_15
    }
}
#[doc = "Field `CSEL2` writer - ADC channel selection"]
pub type CSEL2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, CSEL2_A, 4, O>;
impl<'a, const O: u8> CSEL2_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel2_0(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel2_1(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel2_2(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel2_3(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel2_4(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel2_5(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel2_6(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel2_7(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel2_8(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel2_9(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel2_10(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel2_11(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel2_12(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel2_13(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel2_14(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel2_15(self) -> &'a mut W {
        self.variant(CSEL2_A::CSEL2_15)
    }
}
#[doc = "Field `HWTS2` reader - Segment 2 HWTS ADC hardware trigger selection"]
pub type HWTS2_R = crate::FieldReader<u8, HWTS2_A>;
#[doc = "Segment 2 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS2_A {
    #[doc = "0: no trigger selected"]
    HWTS2_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS2_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS2_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS2_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS2_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS2_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS2_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS2_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS2_128 = 128,
}
impl From<HWTS2_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS2_A) -> Self {
        variant as _
    }
}
impl HWTS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS2_A> {
        match self.bits {
            0 => Some(HWTS2_A::HWTS2_0),
            1 => Some(HWTS2_A::HWTS2_1),
            2 => Some(HWTS2_A::HWTS2_2),
            4 => Some(HWTS2_A::HWTS2_4),
            8 => Some(HWTS2_A::HWTS2_8),
            16 => Some(HWTS2_A::HWTS2_16),
            32 => Some(HWTS2_A::HWTS2_32),
            64 => Some(HWTS2_A::HWTS2_64),
            128 => Some(HWTS2_A::HWTS2_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS2_0`"]
    #[inline(always)]
    pub fn is_hwts2_0(&self) -> bool {
        *self == HWTS2_A::HWTS2_0
    }
    #[doc = "Checks if the value of the field is `HWTS2_1`"]
    #[inline(always)]
    pub fn is_hwts2_1(&self) -> bool {
        *self == HWTS2_A::HWTS2_1
    }
    #[doc = "Checks if the value of the field is `HWTS2_2`"]
    #[inline(always)]
    pub fn is_hwts2_2(&self) -> bool {
        *self == HWTS2_A::HWTS2_2
    }
    #[doc = "Checks if the value of the field is `HWTS2_4`"]
    #[inline(always)]
    pub fn is_hwts2_4(&self) -> bool {
        *self == HWTS2_A::HWTS2_4
    }
    #[doc = "Checks if the value of the field is `HWTS2_8`"]
    #[inline(always)]
    pub fn is_hwts2_8(&self) -> bool {
        *self == HWTS2_A::HWTS2_8
    }
    #[doc = "Checks if the value of the field is `HWTS2_16`"]
    #[inline(always)]
    pub fn is_hwts2_16(&self) -> bool {
        *self == HWTS2_A::HWTS2_16
    }
    #[doc = "Checks if the value of the field is `HWTS2_32`"]
    #[inline(always)]
    pub fn is_hwts2_32(&self) -> bool {
        *self == HWTS2_A::HWTS2_32
    }
    #[doc = "Checks if the value of the field is `HWTS2_64`"]
    #[inline(always)]
    pub fn is_hwts2_64(&self) -> bool {
        *self == HWTS2_A::HWTS2_64
    }
    #[doc = "Checks if the value of the field is `HWTS2_128`"]
    #[inline(always)]
    pub fn is_hwts2_128(&self) -> bool {
        *self == HWTS2_A::HWTS2_128
    }
}
#[doc = "Field `HWTS2` writer - Segment 2 HWTS ADC hardware trigger selection"]
pub type HWTS2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, HWTS2_A, 8, O>;
impl<'a, const O: u8> HWTS2_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts2_0(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts2_1(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts2_2(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts2_4(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts2_8(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts2_16(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts2_32(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts2_64(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts2_128(self) -> &'a mut W {
        self.variant(HWTS2_A::HWTS2_128)
    }
}
#[doc = "Field `B2B2` reader - Segment 2 B2B"]
pub type B2B2_R = crate::BitReader<B2B2_A>;
#[doc = "Segment 2 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B2_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B2_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B2_1 = 1,
}
impl From<B2B2_A> for bool {
    #[inline(always)]
    fn from(variant: B2B2_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B2_A {
        match self.bits {
            false => B2B2_A::B2B2_0,
            true => B2B2_A::B2B2_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B2_0`"]
    #[inline(always)]
    pub fn is_b2b2_0(&self) -> bool {
        *self == B2B2_A::B2B2_0
    }
    #[doc = "Checks if the value of the field is `B2B2_1`"]
    #[inline(always)]
    pub fn is_b2b2_1(&self) -> bool {
        *self == B2B2_A::B2B2_1
    }
}
#[doc = "Field `B2B2` writer - Segment 2 B2B"]
pub type B2B2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG0_CHAIN_3_2_SPEC, B2B2_A, O>;
impl<'a, const O: u8> B2B2_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG2_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b2_0(self) -> &'a mut W {
        self.variant(B2B2_A::B2B2_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b2_1(self) -> &'a mut W {
        self.variant(B2B2_A::B2B2_1)
    }
}
#[doc = "Field `IE2` reader - Segment 2 done interrupt selection"]
pub type IE2_R = crate::FieldReader<u8, IE2_A>;
#[doc = "Segment 2 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE2_A {
    #[doc = "0: No interrupt when finished"]
    IE2_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 2 finish."]
    IE2_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 2 finish."]
    IE2_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 2 finish."]
    IE2_3 = 3,
}
impl From<IE2_A> for u8 {
    #[inline(always)]
    fn from(variant: IE2_A) -> Self {
        variant as _
    }
}
impl IE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE2_A {
        match self.bits {
            0 => IE2_A::IE2_0,
            1 => IE2_A::IE2_1,
            2 => IE2_A::IE2_2,
            3 => IE2_A::IE2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE2_0`"]
    #[inline(always)]
    pub fn is_ie2_0(&self) -> bool {
        *self == IE2_A::IE2_0
    }
    #[doc = "Checks if the value of the field is `IE2_1`"]
    #[inline(always)]
    pub fn is_ie2_1(&self) -> bool {
        *self == IE2_A::IE2_1
    }
    #[doc = "Checks if the value of the field is `IE2_2`"]
    #[inline(always)]
    pub fn is_ie2_2(&self) -> bool {
        *self == IE2_A::IE2_2
    }
    #[doc = "Checks if the value of the field is `IE2_3`"]
    #[inline(always)]
    pub fn is_ie2_3(&self) -> bool {
        *self == IE2_A::IE2_3
    }
}
#[doc = "Field `IE2` writer - Segment 2 done interrupt selection"]
pub type IE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, IE2_A, 2, O>;
impl<'a, const O: u8> IE2_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie2_0(self) -> &'a mut W {
        self.variant(IE2_A::IE2_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 2 finish."]
    #[inline(always)]
    pub fn ie2_1(self) -> &'a mut W {
        self.variant(IE2_A::IE2_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 2 finish."]
    #[inline(always)]
    pub fn ie2_2(self) -> &'a mut W {
        self.variant(IE2_A::IE2_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 2 finish."]
    #[inline(always)]
    pub fn ie2_3(self) -> &'a mut W {
        self.variant(IE2_A::IE2_3)
    }
}
#[doc = "Field `CSEL3` reader - ADC channel selection"]
pub type CSEL3_R = crate::FieldReader<u8, CSEL3_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL3_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL3_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL3_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL3_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL3_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL3_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL3_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL3_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL3_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL3_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL3_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL3_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL3_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL3_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL3_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL3_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL3_15 = 15,
}
impl From<CSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL3_A) -> Self {
        variant as _
    }
}
impl CSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL3_A {
        match self.bits {
            0 => CSEL3_A::CSEL3_0,
            1 => CSEL3_A::CSEL3_1,
            2 => CSEL3_A::CSEL3_2,
            3 => CSEL3_A::CSEL3_3,
            4 => CSEL3_A::CSEL3_4,
            5 => CSEL3_A::CSEL3_5,
            6 => CSEL3_A::CSEL3_6,
            7 => CSEL3_A::CSEL3_7,
            8 => CSEL3_A::CSEL3_8,
            9 => CSEL3_A::CSEL3_9,
            10 => CSEL3_A::CSEL3_10,
            11 => CSEL3_A::CSEL3_11,
            12 => CSEL3_A::CSEL3_12,
            13 => CSEL3_A::CSEL3_13,
            14 => CSEL3_A::CSEL3_14,
            15 => CSEL3_A::CSEL3_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL3_0`"]
    #[inline(always)]
    pub fn is_csel3_0(&self) -> bool {
        *self == CSEL3_A::CSEL3_0
    }
    #[doc = "Checks if the value of the field is `CSEL3_1`"]
    #[inline(always)]
    pub fn is_csel3_1(&self) -> bool {
        *self == CSEL3_A::CSEL3_1
    }
    #[doc = "Checks if the value of the field is `CSEL3_2`"]
    #[inline(always)]
    pub fn is_csel3_2(&self) -> bool {
        *self == CSEL3_A::CSEL3_2
    }
    #[doc = "Checks if the value of the field is `CSEL3_3`"]
    #[inline(always)]
    pub fn is_csel3_3(&self) -> bool {
        *self == CSEL3_A::CSEL3_3
    }
    #[doc = "Checks if the value of the field is `CSEL3_4`"]
    #[inline(always)]
    pub fn is_csel3_4(&self) -> bool {
        *self == CSEL3_A::CSEL3_4
    }
    #[doc = "Checks if the value of the field is `CSEL3_5`"]
    #[inline(always)]
    pub fn is_csel3_5(&self) -> bool {
        *self == CSEL3_A::CSEL3_5
    }
    #[doc = "Checks if the value of the field is `CSEL3_6`"]
    #[inline(always)]
    pub fn is_csel3_6(&self) -> bool {
        *self == CSEL3_A::CSEL3_6
    }
    #[doc = "Checks if the value of the field is `CSEL3_7`"]
    #[inline(always)]
    pub fn is_csel3_7(&self) -> bool {
        *self == CSEL3_A::CSEL3_7
    }
    #[doc = "Checks if the value of the field is `CSEL3_8`"]
    #[inline(always)]
    pub fn is_csel3_8(&self) -> bool {
        *self == CSEL3_A::CSEL3_8
    }
    #[doc = "Checks if the value of the field is `CSEL3_9`"]
    #[inline(always)]
    pub fn is_csel3_9(&self) -> bool {
        *self == CSEL3_A::CSEL3_9
    }
    #[doc = "Checks if the value of the field is `CSEL3_10`"]
    #[inline(always)]
    pub fn is_csel3_10(&self) -> bool {
        *self == CSEL3_A::CSEL3_10
    }
    #[doc = "Checks if the value of the field is `CSEL3_11`"]
    #[inline(always)]
    pub fn is_csel3_11(&self) -> bool {
        *self == CSEL3_A::CSEL3_11
    }
    #[doc = "Checks if the value of the field is `CSEL3_12`"]
    #[inline(always)]
    pub fn is_csel3_12(&self) -> bool {
        *self == CSEL3_A::CSEL3_12
    }
    #[doc = "Checks if the value of the field is `CSEL3_13`"]
    #[inline(always)]
    pub fn is_csel3_13(&self) -> bool {
        *self == CSEL3_A::CSEL3_13
    }
    #[doc = "Checks if the value of the field is `CSEL3_14`"]
    #[inline(always)]
    pub fn is_csel3_14(&self) -> bool {
        *self == CSEL3_A::CSEL3_14
    }
    #[doc = "Checks if the value of the field is `CSEL3_15`"]
    #[inline(always)]
    pub fn is_csel3_15(&self) -> bool {
        *self == CSEL3_A::CSEL3_15
    }
}
#[doc = "Field `CSEL3` writer - ADC channel selection"]
pub type CSEL3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, CSEL3_A, 4, O>;
impl<'a, const O: u8> CSEL3_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel3_0(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel3_1(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel3_2(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel3_3(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel3_4(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel3_5(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel3_6(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel3_7(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel3_8(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel3_9(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel3_10(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel3_11(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel3_12(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel3_13(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel3_14(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel3_15(self) -> &'a mut W {
        self.variant(CSEL3_A::CSEL3_15)
    }
}
#[doc = "Field `HWTS3` reader - Segment 3 HWTS ADC hardware trigger selection"]
pub type HWTS3_R = crate::FieldReader<u8, HWTS3_A>;
#[doc = "Segment 3 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS3_A {
    #[doc = "0: no trigger selected"]
    HWTS3_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS3_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS3_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS3_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS3_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS3_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS3_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS3_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS3_128 = 128,
}
impl From<HWTS3_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS3_A) -> Self {
        variant as _
    }
}
impl HWTS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS3_A> {
        match self.bits {
            0 => Some(HWTS3_A::HWTS3_0),
            1 => Some(HWTS3_A::HWTS3_1),
            2 => Some(HWTS3_A::HWTS3_2),
            4 => Some(HWTS3_A::HWTS3_4),
            8 => Some(HWTS3_A::HWTS3_8),
            16 => Some(HWTS3_A::HWTS3_16),
            32 => Some(HWTS3_A::HWTS3_32),
            64 => Some(HWTS3_A::HWTS3_64),
            128 => Some(HWTS3_A::HWTS3_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS3_0`"]
    #[inline(always)]
    pub fn is_hwts3_0(&self) -> bool {
        *self == HWTS3_A::HWTS3_0
    }
    #[doc = "Checks if the value of the field is `HWTS3_1`"]
    #[inline(always)]
    pub fn is_hwts3_1(&self) -> bool {
        *self == HWTS3_A::HWTS3_1
    }
    #[doc = "Checks if the value of the field is `HWTS3_2`"]
    #[inline(always)]
    pub fn is_hwts3_2(&self) -> bool {
        *self == HWTS3_A::HWTS3_2
    }
    #[doc = "Checks if the value of the field is `HWTS3_4`"]
    #[inline(always)]
    pub fn is_hwts3_4(&self) -> bool {
        *self == HWTS3_A::HWTS3_4
    }
    #[doc = "Checks if the value of the field is `HWTS3_8`"]
    #[inline(always)]
    pub fn is_hwts3_8(&self) -> bool {
        *self == HWTS3_A::HWTS3_8
    }
    #[doc = "Checks if the value of the field is `HWTS3_16`"]
    #[inline(always)]
    pub fn is_hwts3_16(&self) -> bool {
        *self == HWTS3_A::HWTS3_16
    }
    #[doc = "Checks if the value of the field is `HWTS3_32`"]
    #[inline(always)]
    pub fn is_hwts3_32(&self) -> bool {
        *self == HWTS3_A::HWTS3_32
    }
    #[doc = "Checks if the value of the field is `HWTS3_64`"]
    #[inline(always)]
    pub fn is_hwts3_64(&self) -> bool {
        *self == HWTS3_A::HWTS3_64
    }
    #[doc = "Checks if the value of the field is `HWTS3_128`"]
    #[inline(always)]
    pub fn is_hwts3_128(&self) -> bool {
        *self == HWTS3_A::HWTS3_128
    }
}
#[doc = "Field `HWTS3` writer - Segment 3 HWTS ADC hardware trigger selection"]
pub type HWTS3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, HWTS3_A, 8, O>;
impl<'a, const O: u8> HWTS3_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts3_0(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts3_1(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts3_2(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts3_4(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts3_8(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts3_16(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts3_32(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts3_64(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts3_128(self) -> &'a mut W {
        self.variant(HWTS3_A::HWTS3_128)
    }
}
#[doc = "Field `B2B3` reader - Segment 3 B2B"]
pub type B2B3_R = crate::BitReader<B2B3_A>;
#[doc = "Segment 3 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B3_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B3_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B3_1 = 1,
}
impl From<B2B3_A> for bool {
    #[inline(always)]
    fn from(variant: B2B3_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B3_A {
        match self.bits {
            false => B2B3_A::B2B3_0,
            true => B2B3_A::B2B3_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B3_0`"]
    #[inline(always)]
    pub fn is_b2b3_0(&self) -> bool {
        *self == B2B3_A::B2B3_0
    }
    #[doc = "Checks if the value of the field is `B2B3_1`"]
    #[inline(always)]
    pub fn is_b2b3_1(&self) -> bool {
        *self == B2B3_A::B2B3_1
    }
}
#[doc = "Field `B2B3` writer - Segment 3 B2B"]
pub type B2B3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG0_CHAIN_3_2_SPEC, B2B3_A, O>;
impl<'a, const O: u8> B2B3_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG3_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b3_0(self) -> &'a mut W {
        self.variant(B2B3_A::B2B3_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b3_1(self) -> &'a mut W {
        self.variant(B2B3_A::B2B3_1)
    }
}
#[doc = "Field `IE3` reader - Segment 3 done interrupt selection"]
pub type IE3_R = crate::FieldReader<u8, IE3_A>;
#[doc = "Segment 3 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE3_A {
    #[doc = "0: No interrupt when finished"]
    IE3_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 3 finish."]
    IE3_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 3 finish."]
    IE3_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 3 finish."]
    IE3_3 = 3,
}
impl From<IE3_A> for u8 {
    #[inline(always)]
    fn from(variant: IE3_A) -> Self {
        variant as _
    }
}
impl IE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE3_A {
        match self.bits {
            0 => IE3_A::IE3_0,
            1 => IE3_A::IE3_1,
            2 => IE3_A::IE3_2,
            3 => IE3_A::IE3_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE3_0`"]
    #[inline(always)]
    pub fn is_ie3_0(&self) -> bool {
        *self == IE3_A::IE3_0
    }
    #[doc = "Checks if the value of the field is `IE3_1`"]
    #[inline(always)]
    pub fn is_ie3_1(&self) -> bool {
        *self == IE3_A::IE3_1
    }
    #[doc = "Checks if the value of the field is `IE3_2`"]
    #[inline(always)]
    pub fn is_ie3_2(&self) -> bool {
        *self == IE3_A::IE3_2
    }
    #[doc = "Checks if the value of the field is `IE3_3`"]
    #[inline(always)]
    pub fn is_ie3_3(&self) -> bool {
        *self == IE3_A::IE3_3
    }
}
#[doc = "Field `IE3` writer - Segment 3 done interrupt selection"]
pub type IE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG0_CHAIN_3_2_SPEC, u8, IE3_A, 2, O>;
impl<'a, const O: u8> IE3_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie3_0(self) -> &'a mut W {
        self.variant(IE3_A::IE3_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 3 finish."]
    #[inline(always)]
    pub fn ie3_1(self) -> &'a mut W {
        self.variant(IE3_A::IE3_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 3 finish."]
    #[inline(always)]
    pub fn ie3_2(self) -> &'a mut W {
        self.variant(IE3_A::IE3_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 3 finish."]
    #[inline(always)]
    pub fn ie3_3(self) -> &'a mut W {
        self.variant(IE3_A::IE3_3)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    pub fn csel2(&self) -> CSEL2_R {
        CSEL2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts2(&self) -> HWTS2_R {
        HWTS2_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Segment 2 B2B"]
    #[inline(always)]
    pub fn b2b2(&self) -> B2B2_R {
        B2B2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Segment 2 done interrupt selection"]
    #[inline(always)]
    pub fn ie2(&self) -> IE2_R {
        IE2_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    pub fn csel3(&self) -> CSEL3_R {
        CSEL3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts3(&self) -> HWTS3_R {
        HWTS3_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Segment 3 B2B"]
    #[inline(always)]
    pub fn b2b3(&self) -> B2B3_R {
        B2B3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Segment 3 done interrupt selection"]
    #[inline(always)]
    pub fn ie3(&self) -> IE3_R {
        IE3_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel2(&mut self) -> CSEL2_W<0> {
        CSEL2_W::new(self)
    }
    #[doc = "Bits 4:11 - Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts2(&mut self) -> HWTS2_W<4> {
        HWTS2_W::new(self)
    }
    #[doc = "Bit 12 - Segment 2 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b2(&mut self) -> B2B2_W<12> {
        B2B2_W::new(self)
    }
    #[doc = "Bits 13:14 - Segment 2 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie2(&mut self) -> IE2_W<13> {
        IE2_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel3(&mut self) -> CSEL3_W<16> {
        CSEL3_W::new(self)
    }
    #[doc = "Bits 20:27 - Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts3(&mut self) -> HWTS3_W<20> {
        HWTS3_W::new(self)
    }
    #[doc = "Bit 28 - Segment 3 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b3(&mut self) -> B2B3_W<28> {
        B2B3_W::new(self)
    }
    #[doc = "Bits 29:30 - Segment 3 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie3(&mut self) -> IE3_W<29> {
        IE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig0_chain_3_2](index.html) module"]
pub struct TRIG0_CHAIN_3_2_SPEC;
impl crate::RegisterSpec for TRIG0_CHAIN_3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig0_chain_3_2::R](R) reader structure"]
impl crate::Readable for TRIG0_CHAIN_3_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig0_chain_3_2::W](W) writer structure"]
impl crate::Writable for TRIG0_CHAIN_3_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG0_CHAIN_3_2 to value 0"]
impl crate::Resettable for TRIG0_CHAIN_3_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
