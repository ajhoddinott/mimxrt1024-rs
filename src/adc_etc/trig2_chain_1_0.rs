#[doc = "Register `TRIG2_CHAIN_1_0` reader"]
pub struct R(crate::R<TRIG2_CHAIN_1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG2_CHAIN_1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG2_CHAIN_1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG2_CHAIN_1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG2_CHAIN_1_0` writer"]
pub struct W(crate::W<TRIG2_CHAIN_1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG2_CHAIN_1_0_SPEC>;
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
impl From<crate::W<TRIG2_CHAIN_1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG2_CHAIN_1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL0` reader - ADC channel selection"]
pub type CSEL0_R = crate::FieldReader<u8, CSEL0_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL0_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL0_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL0_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL0_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL0_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL0_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL0_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL0_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL0_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL0_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL0_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL0_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL0_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL0_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL0_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL0_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL0_15 = 15,
}
impl From<CSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL0_A) -> Self {
        variant as _
    }
}
impl CSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL0_A {
        match self.bits {
            0 => CSEL0_A::CSEL0_0,
            1 => CSEL0_A::CSEL0_1,
            2 => CSEL0_A::CSEL0_2,
            3 => CSEL0_A::CSEL0_3,
            4 => CSEL0_A::CSEL0_4,
            5 => CSEL0_A::CSEL0_5,
            6 => CSEL0_A::CSEL0_6,
            7 => CSEL0_A::CSEL0_7,
            8 => CSEL0_A::CSEL0_8,
            9 => CSEL0_A::CSEL0_9,
            10 => CSEL0_A::CSEL0_10,
            11 => CSEL0_A::CSEL0_11,
            12 => CSEL0_A::CSEL0_12,
            13 => CSEL0_A::CSEL0_13,
            14 => CSEL0_A::CSEL0_14,
            15 => CSEL0_A::CSEL0_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL0_0`"]
    #[inline(always)]
    pub fn is_csel0_0(&self) -> bool {
        *self == CSEL0_A::CSEL0_0
    }
    #[doc = "Checks if the value of the field is `CSEL0_1`"]
    #[inline(always)]
    pub fn is_csel0_1(&self) -> bool {
        *self == CSEL0_A::CSEL0_1
    }
    #[doc = "Checks if the value of the field is `CSEL0_2`"]
    #[inline(always)]
    pub fn is_csel0_2(&self) -> bool {
        *self == CSEL0_A::CSEL0_2
    }
    #[doc = "Checks if the value of the field is `CSEL0_3`"]
    #[inline(always)]
    pub fn is_csel0_3(&self) -> bool {
        *self == CSEL0_A::CSEL0_3
    }
    #[doc = "Checks if the value of the field is `CSEL0_4`"]
    #[inline(always)]
    pub fn is_csel0_4(&self) -> bool {
        *self == CSEL0_A::CSEL0_4
    }
    #[doc = "Checks if the value of the field is `CSEL0_5`"]
    #[inline(always)]
    pub fn is_csel0_5(&self) -> bool {
        *self == CSEL0_A::CSEL0_5
    }
    #[doc = "Checks if the value of the field is `CSEL0_6`"]
    #[inline(always)]
    pub fn is_csel0_6(&self) -> bool {
        *self == CSEL0_A::CSEL0_6
    }
    #[doc = "Checks if the value of the field is `CSEL0_7`"]
    #[inline(always)]
    pub fn is_csel0_7(&self) -> bool {
        *self == CSEL0_A::CSEL0_7
    }
    #[doc = "Checks if the value of the field is `CSEL0_8`"]
    #[inline(always)]
    pub fn is_csel0_8(&self) -> bool {
        *self == CSEL0_A::CSEL0_8
    }
    #[doc = "Checks if the value of the field is `CSEL0_9`"]
    #[inline(always)]
    pub fn is_csel0_9(&self) -> bool {
        *self == CSEL0_A::CSEL0_9
    }
    #[doc = "Checks if the value of the field is `CSEL0_10`"]
    #[inline(always)]
    pub fn is_csel0_10(&self) -> bool {
        *self == CSEL0_A::CSEL0_10
    }
    #[doc = "Checks if the value of the field is `CSEL0_11`"]
    #[inline(always)]
    pub fn is_csel0_11(&self) -> bool {
        *self == CSEL0_A::CSEL0_11
    }
    #[doc = "Checks if the value of the field is `CSEL0_12`"]
    #[inline(always)]
    pub fn is_csel0_12(&self) -> bool {
        *self == CSEL0_A::CSEL0_12
    }
    #[doc = "Checks if the value of the field is `CSEL0_13`"]
    #[inline(always)]
    pub fn is_csel0_13(&self) -> bool {
        *self == CSEL0_A::CSEL0_13
    }
    #[doc = "Checks if the value of the field is `CSEL0_14`"]
    #[inline(always)]
    pub fn is_csel0_14(&self) -> bool {
        *self == CSEL0_A::CSEL0_14
    }
    #[doc = "Checks if the value of the field is `CSEL0_15`"]
    #[inline(always)]
    pub fn is_csel0_15(&self) -> bool {
        *self == CSEL0_A::CSEL0_15
    }
}
#[doc = "Field `CSEL0` writer - ADC channel selection"]
pub type CSEL0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, CSEL0_A, 4, O>;
impl<'a, const O: u8> CSEL0_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel0_0(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel0_1(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel0_2(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel0_3(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel0_4(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel0_5(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel0_6(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel0_7(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel0_8(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel0_9(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel0_10(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel0_11(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel0_12(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel0_13(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel0_14(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel0_15(self) -> &'a mut W {
        self.variant(CSEL0_A::CSEL0_15)
    }
}
#[doc = "Field `HWTS0` reader - Segment 0 HWTS ADC hardware trigger selection"]
pub type HWTS0_R = crate::FieldReader<u8, HWTS0_A>;
#[doc = "Segment 0 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS0_A {
    #[doc = "0: no trigger selected"]
    HWTS0_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS0_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS0_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS0_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS0_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS0_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS0_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS0_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS0_128 = 128,
}
impl From<HWTS0_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS0_A) -> Self {
        variant as _
    }
}
impl HWTS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS0_A> {
        match self.bits {
            0 => Some(HWTS0_A::HWTS0_0),
            1 => Some(HWTS0_A::HWTS0_1),
            2 => Some(HWTS0_A::HWTS0_2),
            4 => Some(HWTS0_A::HWTS0_4),
            8 => Some(HWTS0_A::HWTS0_8),
            16 => Some(HWTS0_A::HWTS0_16),
            32 => Some(HWTS0_A::HWTS0_32),
            64 => Some(HWTS0_A::HWTS0_64),
            128 => Some(HWTS0_A::HWTS0_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS0_0`"]
    #[inline(always)]
    pub fn is_hwts0_0(&self) -> bool {
        *self == HWTS0_A::HWTS0_0
    }
    #[doc = "Checks if the value of the field is `HWTS0_1`"]
    #[inline(always)]
    pub fn is_hwts0_1(&self) -> bool {
        *self == HWTS0_A::HWTS0_1
    }
    #[doc = "Checks if the value of the field is `HWTS0_2`"]
    #[inline(always)]
    pub fn is_hwts0_2(&self) -> bool {
        *self == HWTS0_A::HWTS0_2
    }
    #[doc = "Checks if the value of the field is `HWTS0_4`"]
    #[inline(always)]
    pub fn is_hwts0_4(&self) -> bool {
        *self == HWTS0_A::HWTS0_4
    }
    #[doc = "Checks if the value of the field is `HWTS0_8`"]
    #[inline(always)]
    pub fn is_hwts0_8(&self) -> bool {
        *self == HWTS0_A::HWTS0_8
    }
    #[doc = "Checks if the value of the field is `HWTS0_16`"]
    #[inline(always)]
    pub fn is_hwts0_16(&self) -> bool {
        *self == HWTS0_A::HWTS0_16
    }
    #[doc = "Checks if the value of the field is `HWTS0_32`"]
    #[inline(always)]
    pub fn is_hwts0_32(&self) -> bool {
        *self == HWTS0_A::HWTS0_32
    }
    #[doc = "Checks if the value of the field is `HWTS0_64`"]
    #[inline(always)]
    pub fn is_hwts0_64(&self) -> bool {
        *self == HWTS0_A::HWTS0_64
    }
    #[doc = "Checks if the value of the field is `HWTS0_128`"]
    #[inline(always)]
    pub fn is_hwts0_128(&self) -> bool {
        *self == HWTS0_A::HWTS0_128
    }
}
#[doc = "Field `HWTS0` writer - Segment 0 HWTS ADC hardware trigger selection"]
pub type HWTS0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, HWTS0_A, 8, O>;
impl<'a, const O: u8> HWTS0_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts0_0(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts0_1(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts0_2(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts0_4(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts0_8(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts0_16(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts0_32(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts0_64(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts0_128(self) -> &'a mut W {
        self.variant(HWTS0_A::HWTS0_128)
    }
}
#[doc = "Field `B2B0` reader - Segment 0 B2B"]
pub type B2B0_R = crate::BitReader<B2B0_A>;
#[doc = "Segment 0 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B0_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B0_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B0_1 = 1,
}
impl From<B2B0_A> for bool {
    #[inline(always)]
    fn from(variant: B2B0_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B0_A {
        match self.bits {
            false => B2B0_A::B2B0_0,
            true => B2B0_A::B2B0_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B0_0`"]
    #[inline(always)]
    pub fn is_b2b0_0(&self) -> bool {
        *self == B2B0_A::B2B0_0
    }
    #[doc = "Checks if the value of the field is `B2B0_1`"]
    #[inline(always)]
    pub fn is_b2b0_1(&self) -> bool {
        *self == B2B0_A::B2B0_1
    }
}
#[doc = "Field `B2B0` writer - Segment 0 B2B"]
pub type B2B0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG2_CHAIN_1_0_SPEC, B2B0_A, O>;
impl<'a, const O: u8> B2B0_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG0_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b0_0(self) -> &'a mut W {
        self.variant(B2B0_A::B2B0_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b0_1(self) -> &'a mut W {
        self.variant(B2B0_A::B2B0_1)
    }
}
#[doc = "Field `IE0` reader - Segment 0 done interrupt selection"]
pub type IE0_R = crate::FieldReader<u8, IE0_A>;
#[doc = "Segment 0 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE0_A {
    #[doc = "0: No interrupt when finished"]
    IE0_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 0 finish."]
    IE0_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 0 finish."]
    IE0_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 0 finish."]
    IE0_3 = 3,
}
impl From<IE0_A> for u8 {
    #[inline(always)]
    fn from(variant: IE0_A) -> Self {
        variant as _
    }
}
impl IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE0_A {
        match self.bits {
            0 => IE0_A::IE0_0,
            1 => IE0_A::IE0_1,
            2 => IE0_A::IE0_2,
            3 => IE0_A::IE0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE0_0`"]
    #[inline(always)]
    pub fn is_ie0_0(&self) -> bool {
        *self == IE0_A::IE0_0
    }
    #[doc = "Checks if the value of the field is `IE0_1`"]
    #[inline(always)]
    pub fn is_ie0_1(&self) -> bool {
        *self == IE0_A::IE0_1
    }
    #[doc = "Checks if the value of the field is `IE0_2`"]
    #[inline(always)]
    pub fn is_ie0_2(&self) -> bool {
        *self == IE0_A::IE0_2
    }
    #[doc = "Checks if the value of the field is `IE0_3`"]
    #[inline(always)]
    pub fn is_ie0_3(&self) -> bool {
        *self == IE0_A::IE0_3
    }
}
#[doc = "Field `IE0` writer - Segment 0 done interrupt selection"]
pub type IE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, IE0_A, 2, O>;
impl<'a, const O: u8> IE0_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie0_0(self) -> &'a mut W {
        self.variant(IE0_A::IE0_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 0 finish."]
    #[inline(always)]
    pub fn ie0_1(self) -> &'a mut W {
        self.variant(IE0_A::IE0_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 0 finish."]
    #[inline(always)]
    pub fn ie0_2(self) -> &'a mut W {
        self.variant(IE0_A::IE0_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 0 finish."]
    #[inline(always)]
    pub fn ie0_3(self) -> &'a mut W {
        self.variant(IE0_A::IE0_3)
    }
}
#[doc = "Field `CSEL1` reader - ADC channel selection"]
pub type CSEL1_R = crate::FieldReader<u8, CSEL1_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL1_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL1_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL1_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL1_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL1_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL1_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL1_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL1_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL1_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL1_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL1_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL1_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL1_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL1_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL1_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL1_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL1_15 = 15,
}
impl From<CSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL1_A) -> Self {
        variant as _
    }
}
impl CSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL1_A {
        match self.bits {
            0 => CSEL1_A::CSEL1_0,
            1 => CSEL1_A::CSEL1_1,
            2 => CSEL1_A::CSEL1_2,
            3 => CSEL1_A::CSEL1_3,
            4 => CSEL1_A::CSEL1_4,
            5 => CSEL1_A::CSEL1_5,
            6 => CSEL1_A::CSEL1_6,
            7 => CSEL1_A::CSEL1_7,
            8 => CSEL1_A::CSEL1_8,
            9 => CSEL1_A::CSEL1_9,
            10 => CSEL1_A::CSEL1_10,
            11 => CSEL1_A::CSEL1_11,
            12 => CSEL1_A::CSEL1_12,
            13 => CSEL1_A::CSEL1_13,
            14 => CSEL1_A::CSEL1_14,
            15 => CSEL1_A::CSEL1_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL1_0`"]
    #[inline(always)]
    pub fn is_csel1_0(&self) -> bool {
        *self == CSEL1_A::CSEL1_0
    }
    #[doc = "Checks if the value of the field is `CSEL1_1`"]
    #[inline(always)]
    pub fn is_csel1_1(&self) -> bool {
        *self == CSEL1_A::CSEL1_1
    }
    #[doc = "Checks if the value of the field is `CSEL1_2`"]
    #[inline(always)]
    pub fn is_csel1_2(&self) -> bool {
        *self == CSEL1_A::CSEL1_2
    }
    #[doc = "Checks if the value of the field is `CSEL1_3`"]
    #[inline(always)]
    pub fn is_csel1_3(&self) -> bool {
        *self == CSEL1_A::CSEL1_3
    }
    #[doc = "Checks if the value of the field is `CSEL1_4`"]
    #[inline(always)]
    pub fn is_csel1_4(&self) -> bool {
        *self == CSEL1_A::CSEL1_4
    }
    #[doc = "Checks if the value of the field is `CSEL1_5`"]
    #[inline(always)]
    pub fn is_csel1_5(&self) -> bool {
        *self == CSEL1_A::CSEL1_5
    }
    #[doc = "Checks if the value of the field is `CSEL1_6`"]
    #[inline(always)]
    pub fn is_csel1_6(&self) -> bool {
        *self == CSEL1_A::CSEL1_6
    }
    #[doc = "Checks if the value of the field is `CSEL1_7`"]
    #[inline(always)]
    pub fn is_csel1_7(&self) -> bool {
        *self == CSEL1_A::CSEL1_7
    }
    #[doc = "Checks if the value of the field is `CSEL1_8`"]
    #[inline(always)]
    pub fn is_csel1_8(&self) -> bool {
        *self == CSEL1_A::CSEL1_8
    }
    #[doc = "Checks if the value of the field is `CSEL1_9`"]
    #[inline(always)]
    pub fn is_csel1_9(&self) -> bool {
        *self == CSEL1_A::CSEL1_9
    }
    #[doc = "Checks if the value of the field is `CSEL1_10`"]
    #[inline(always)]
    pub fn is_csel1_10(&self) -> bool {
        *self == CSEL1_A::CSEL1_10
    }
    #[doc = "Checks if the value of the field is `CSEL1_11`"]
    #[inline(always)]
    pub fn is_csel1_11(&self) -> bool {
        *self == CSEL1_A::CSEL1_11
    }
    #[doc = "Checks if the value of the field is `CSEL1_12`"]
    #[inline(always)]
    pub fn is_csel1_12(&self) -> bool {
        *self == CSEL1_A::CSEL1_12
    }
    #[doc = "Checks if the value of the field is `CSEL1_13`"]
    #[inline(always)]
    pub fn is_csel1_13(&self) -> bool {
        *self == CSEL1_A::CSEL1_13
    }
    #[doc = "Checks if the value of the field is `CSEL1_14`"]
    #[inline(always)]
    pub fn is_csel1_14(&self) -> bool {
        *self == CSEL1_A::CSEL1_14
    }
    #[doc = "Checks if the value of the field is `CSEL1_15`"]
    #[inline(always)]
    pub fn is_csel1_15(&self) -> bool {
        *self == CSEL1_A::CSEL1_15
    }
}
#[doc = "Field `CSEL1` writer - ADC channel selection"]
pub type CSEL1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, CSEL1_A, 4, O>;
impl<'a, const O: u8> CSEL1_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel1_0(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel1_1(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel1_2(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel1_3(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel1_4(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel1_5(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel1_6(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel1_7(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel1_8(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel1_9(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel1_10(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel1_11(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel1_12(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel1_13(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel1_14(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel1_15(self) -> &'a mut W {
        self.variant(CSEL1_A::CSEL1_15)
    }
}
#[doc = "Field `HWTS1` reader - Segment 1 HWTS ADC hardware trigger selection"]
pub type HWTS1_R = crate::FieldReader<u8, HWTS1_A>;
#[doc = "Segment 1 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS1_A {
    #[doc = "0: no trigger selected"]
    HWTS1_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS1_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS1_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS1_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS1_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS1_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS1_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS1_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS1_128 = 128,
}
impl From<HWTS1_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS1_A) -> Self {
        variant as _
    }
}
impl HWTS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS1_A> {
        match self.bits {
            0 => Some(HWTS1_A::HWTS1_0),
            1 => Some(HWTS1_A::HWTS1_1),
            2 => Some(HWTS1_A::HWTS1_2),
            4 => Some(HWTS1_A::HWTS1_4),
            8 => Some(HWTS1_A::HWTS1_8),
            16 => Some(HWTS1_A::HWTS1_16),
            32 => Some(HWTS1_A::HWTS1_32),
            64 => Some(HWTS1_A::HWTS1_64),
            128 => Some(HWTS1_A::HWTS1_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS1_0`"]
    #[inline(always)]
    pub fn is_hwts1_0(&self) -> bool {
        *self == HWTS1_A::HWTS1_0
    }
    #[doc = "Checks if the value of the field is `HWTS1_1`"]
    #[inline(always)]
    pub fn is_hwts1_1(&self) -> bool {
        *self == HWTS1_A::HWTS1_1
    }
    #[doc = "Checks if the value of the field is `HWTS1_2`"]
    #[inline(always)]
    pub fn is_hwts1_2(&self) -> bool {
        *self == HWTS1_A::HWTS1_2
    }
    #[doc = "Checks if the value of the field is `HWTS1_4`"]
    #[inline(always)]
    pub fn is_hwts1_4(&self) -> bool {
        *self == HWTS1_A::HWTS1_4
    }
    #[doc = "Checks if the value of the field is `HWTS1_8`"]
    #[inline(always)]
    pub fn is_hwts1_8(&self) -> bool {
        *self == HWTS1_A::HWTS1_8
    }
    #[doc = "Checks if the value of the field is `HWTS1_16`"]
    #[inline(always)]
    pub fn is_hwts1_16(&self) -> bool {
        *self == HWTS1_A::HWTS1_16
    }
    #[doc = "Checks if the value of the field is `HWTS1_32`"]
    #[inline(always)]
    pub fn is_hwts1_32(&self) -> bool {
        *self == HWTS1_A::HWTS1_32
    }
    #[doc = "Checks if the value of the field is `HWTS1_64`"]
    #[inline(always)]
    pub fn is_hwts1_64(&self) -> bool {
        *self == HWTS1_A::HWTS1_64
    }
    #[doc = "Checks if the value of the field is `HWTS1_128`"]
    #[inline(always)]
    pub fn is_hwts1_128(&self) -> bool {
        *self == HWTS1_A::HWTS1_128
    }
}
#[doc = "Field `HWTS1` writer - Segment 1 HWTS ADC hardware trigger selection"]
pub type HWTS1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, HWTS1_A, 8, O>;
impl<'a, const O: u8> HWTS1_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts1_0(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts1_1(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts1_2(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts1_4(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts1_8(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts1_16(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts1_32(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts1_64(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts1_128(self) -> &'a mut W {
        self.variant(HWTS1_A::HWTS1_128)
    }
}
#[doc = "Field `B2B1` reader - Segment 1 B2B"]
pub type B2B1_R = crate::BitReader<B2B1_A>;
#[doc = "Segment 1 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B1_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B1_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B1_1 = 1,
}
impl From<B2B1_A> for bool {
    #[inline(always)]
    fn from(variant: B2B1_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B1_A {
        match self.bits {
            false => B2B1_A::B2B1_0,
            true => B2B1_A::B2B1_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B1_0`"]
    #[inline(always)]
    pub fn is_b2b1_0(&self) -> bool {
        *self == B2B1_A::B2B1_0
    }
    #[doc = "Checks if the value of the field is `B2B1_1`"]
    #[inline(always)]
    pub fn is_b2b1_1(&self) -> bool {
        *self == B2B1_A::B2B1_1
    }
}
#[doc = "Field `B2B1` writer - Segment 1 B2B"]
pub type B2B1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG2_CHAIN_1_0_SPEC, B2B1_A, O>;
impl<'a, const O: u8> B2B1_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG1_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b1_0(self) -> &'a mut W {
        self.variant(B2B1_A::B2B1_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b1_1(self) -> &'a mut W {
        self.variant(B2B1_A::B2B1_1)
    }
}
#[doc = "Field `IE1` reader - Segment 1 done interrupt selection"]
pub type IE1_R = crate::FieldReader<u8, IE1_A>;
#[doc = "Segment 1 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE1_A {
    #[doc = "0: No interrupt when finished"]
    IE1_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when Segment 1 finish."]
    IE1_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when Segment 1 finish."]
    IE1_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when Segment 1 finish."]
    IE1_3 = 3,
}
impl From<IE1_A> for u8 {
    #[inline(always)]
    fn from(variant: IE1_A) -> Self {
        variant as _
    }
}
impl IE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE1_A {
        match self.bits {
            0 => IE1_A::IE1_0,
            1 => IE1_A::IE1_1,
            2 => IE1_A::IE1_2,
            3 => IE1_A::IE1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE1_0`"]
    #[inline(always)]
    pub fn is_ie1_0(&self) -> bool {
        *self == IE1_A::IE1_0
    }
    #[doc = "Checks if the value of the field is `IE1_1`"]
    #[inline(always)]
    pub fn is_ie1_1(&self) -> bool {
        *self == IE1_A::IE1_1
    }
    #[doc = "Checks if the value of the field is `IE1_2`"]
    #[inline(always)]
    pub fn is_ie1_2(&self) -> bool {
        *self == IE1_A::IE1_2
    }
    #[doc = "Checks if the value of the field is `IE1_3`"]
    #[inline(always)]
    pub fn is_ie1_3(&self) -> bool {
        *self == IE1_A::IE1_3
    }
}
#[doc = "Field `IE1` writer - Segment 1 done interrupt selection"]
pub type IE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_1_0_SPEC, u8, IE1_A, 2, O>;
impl<'a, const O: u8> IE1_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie1_0(self) -> &'a mut W {
        self.variant(IE1_A::IE1_0)
    }
    #[doc = "Generate interrupt on Done0 when Segment 1 finish."]
    #[inline(always)]
    pub fn ie1_1(self) -> &'a mut W {
        self.variant(IE1_A::IE1_1)
    }
    #[doc = "Generate interrupt on Done1 when Segment 1 finish."]
    #[inline(always)]
    pub fn ie1_2(self) -> &'a mut W {
        self.variant(IE1_A::IE1_2)
    }
    #[doc = "Generate interrupt on Done2 when Segment 1 finish."]
    #[inline(always)]
    pub fn ie1_3(self) -> &'a mut W {
        self.variant(IE1_A::IE1_3)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    pub fn csel0(&self) -> CSEL0_R {
        CSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts0(&self) -> HWTS0_R {
        HWTS0_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Segment 0 B2B"]
    #[inline(always)]
    pub fn b2b0(&self) -> B2B0_R {
        B2B0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Segment 0 done interrupt selection"]
    #[inline(always)]
    pub fn ie0(&self) -> IE0_R {
        IE0_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    pub fn csel1(&self) -> CSEL1_R {
        CSEL1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts1(&self) -> HWTS1_R {
        HWTS1_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Segment 1 B2B"]
    #[inline(always)]
    pub fn b2b1(&self) -> B2B1_R {
        B2B1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Segment 1 done interrupt selection"]
    #[inline(always)]
    pub fn ie1(&self) -> IE1_R {
        IE1_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel0(&mut self) -> CSEL0_W<0> {
        CSEL0_W::new(self)
    }
    #[doc = "Bits 4:11 - Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts0(&mut self) -> HWTS0_W<4> {
        HWTS0_W::new(self)
    }
    #[doc = "Bit 12 - Segment 0 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b0(&mut self) -> B2B0_W<12> {
        B2B0_W::new(self)
    }
    #[doc = "Bits 13:14 - Segment 0 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie0(&mut self) -> IE0_W<13> {
        IE0_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel1(&mut self) -> CSEL1_W<16> {
        CSEL1_W::new(self)
    }
    #[doc = "Bits 20:27 - Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts1(&mut self) -> HWTS1_W<20> {
        HWTS1_W::new(self)
    }
    #[doc = "Bit 28 - Segment 1 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b1(&mut self) -> B2B1_W<28> {
        B2B1_W::new(self)
    }
    #[doc = "Bits 29:30 - Segment 1 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie1(&mut self) -> IE1_W<29> {
        IE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_1_0](index.html) module"]
pub struct TRIG2_CHAIN_1_0_SPEC;
impl crate::RegisterSpec for TRIG2_CHAIN_1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig2_chain_1_0::R](R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig2_chain_1_0::W](W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_1_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG2_CHAIN_1_0 to value 0"]
impl crate::Resettable for TRIG2_CHAIN_1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
