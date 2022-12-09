#[doc = "Register `TRIG2_CHAIN_5_4` reader"]
pub struct R(crate::R<TRIG2_CHAIN_5_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG2_CHAIN_5_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG2_CHAIN_5_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG2_CHAIN_5_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG2_CHAIN_5_4` writer"]
pub struct W(crate::W<TRIG2_CHAIN_5_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG2_CHAIN_5_4_SPEC>;
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
impl From<crate::W<TRIG2_CHAIN_5_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG2_CHAIN_5_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL4` reader - ADC channel selection"]
pub type CSEL4_R = crate::FieldReader<u8, CSEL4_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL4_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL4_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL4_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL4_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL4_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL4_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL4_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL4_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL4_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL4_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL4_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL4_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL4_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL4_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL4_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL4_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL4_15 = 15,
}
impl From<CSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL4_A) -> Self {
        variant as _
    }
}
impl CSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL4_A {
        match self.bits {
            0 => CSEL4_A::CSEL4_0,
            1 => CSEL4_A::CSEL4_1,
            2 => CSEL4_A::CSEL4_2,
            3 => CSEL4_A::CSEL4_3,
            4 => CSEL4_A::CSEL4_4,
            5 => CSEL4_A::CSEL4_5,
            6 => CSEL4_A::CSEL4_6,
            7 => CSEL4_A::CSEL4_7,
            8 => CSEL4_A::CSEL4_8,
            9 => CSEL4_A::CSEL4_9,
            10 => CSEL4_A::CSEL4_10,
            11 => CSEL4_A::CSEL4_11,
            12 => CSEL4_A::CSEL4_12,
            13 => CSEL4_A::CSEL4_13,
            14 => CSEL4_A::CSEL4_14,
            15 => CSEL4_A::CSEL4_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL4_0`"]
    #[inline(always)]
    pub fn is_csel4_0(&self) -> bool {
        *self == CSEL4_A::CSEL4_0
    }
    #[doc = "Checks if the value of the field is `CSEL4_1`"]
    #[inline(always)]
    pub fn is_csel4_1(&self) -> bool {
        *self == CSEL4_A::CSEL4_1
    }
    #[doc = "Checks if the value of the field is `CSEL4_2`"]
    #[inline(always)]
    pub fn is_csel4_2(&self) -> bool {
        *self == CSEL4_A::CSEL4_2
    }
    #[doc = "Checks if the value of the field is `CSEL4_3`"]
    #[inline(always)]
    pub fn is_csel4_3(&self) -> bool {
        *self == CSEL4_A::CSEL4_3
    }
    #[doc = "Checks if the value of the field is `CSEL4_4`"]
    #[inline(always)]
    pub fn is_csel4_4(&self) -> bool {
        *self == CSEL4_A::CSEL4_4
    }
    #[doc = "Checks if the value of the field is `CSEL4_5`"]
    #[inline(always)]
    pub fn is_csel4_5(&self) -> bool {
        *self == CSEL4_A::CSEL4_5
    }
    #[doc = "Checks if the value of the field is `CSEL4_6`"]
    #[inline(always)]
    pub fn is_csel4_6(&self) -> bool {
        *self == CSEL4_A::CSEL4_6
    }
    #[doc = "Checks if the value of the field is `CSEL4_7`"]
    #[inline(always)]
    pub fn is_csel4_7(&self) -> bool {
        *self == CSEL4_A::CSEL4_7
    }
    #[doc = "Checks if the value of the field is `CSEL4_8`"]
    #[inline(always)]
    pub fn is_csel4_8(&self) -> bool {
        *self == CSEL4_A::CSEL4_8
    }
    #[doc = "Checks if the value of the field is `CSEL4_9`"]
    #[inline(always)]
    pub fn is_csel4_9(&self) -> bool {
        *self == CSEL4_A::CSEL4_9
    }
    #[doc = "Checks if the value of the field is `CSEL4_10`"]
    #[inline(always)]
    pub fn is_csel4_10(&self) -> bool {
        *self == CSEL4_A::CSEL4_10
    }
    #[doc = "Checks if the value of the field is `CSEL4_11`"]
    #[inline(always)]
    pub fn is_csel4_11(&self) -> bool {
        *self == CSEL4_A::CSEL4_11
    }
    #[doc = "Checks if the value of the field is `CSEL4_12`"]
    #[inline(always)]
    pub fn is_csel4_12(&self) -> bool {
        *self == CSEL4_A::CSEL4_12
    }
    #[doc = "Checks if the value of the field is `CSEL4_13`"]
    #[inline(always)]
    pub fn is_csel4_13(&self) -> bool {
        *self == CSEL4_A::CSEL4_13
    }
    #[doc = "Checks if the value of the field is `CSEL4_14`"]
    #[inline(always)]
    pub fn is_csel4_14(&self) -> bool {
        *self == CSEL4_A::CSEL4_14
    }
    #[doc = "Checks if the value of the field is `CSEL4_15`"]
    #[inline(always)]
    pub fn is_csel4_15(&self) -> bool {
        *self == CSEL4_A::CSEL4_15
    }
}
#[doc = "Field `CSEL4` writer - ADC channel selection"]
pub type CSEL4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, CSEL4_A, 4, O>;
impl<'a, const O: u8> CSEL4_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel4_0(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel4_1(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel4_2(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel4_3(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel4_4(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel4_5(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel4_6(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel4_7(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel4_8(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel4_9(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel4_10(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel4_11(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel4_12(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel4_13(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel4_14(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel4_15(self) -> &'a mut W {
        self.variant(CSEL4_A::CSEL4_15)
    }
}
#[doc = "Field `HWTS4` reader - Segment 4 HWTS ADC hardware trigger selection"]
pub type HWTS4_R = crate::FieldReader<u8, HWTS4_A>;
#[doc = "Segment 4 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS4_A {
    #[doc = "0: no trigger selected"]
    HWTS4_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS4_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS4_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS4_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS4_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS4_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS4_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS4_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS4_128 = 128,
}
impl From<HWTS4_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS4_A) -> Self {
        variant as _
    }
}
impl HWTS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS4_A> {
        match self.bits {
            0 => Some(HWTS4_A::HWTS4_0),
            1 => Some(HWTS4_A::HWTS4_1),
            2 => Some(HWTS4_A::HWTS4_2),
            4 => Some(HWTS4_A::HWTS4_4),
            8 => Some(HWTS4_A::HWTS4_8),
            16 => Some(HWTS4_A::HWTS4_16),
            32 => Some(HWTS4_A::HWTS4_32),
            64 => Some(HWTS4_A::HWTS4_64),
            128 => Some(HWTS4_A::HWTS4_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS4_0`"]
    #[inline(always)]
    pub fn is_hwts4_0(&self) -> bool {
        *self == HWTS4_A::HWTS4_0
    }
    #[doc = "Checks if the value of the field is `HWTS4_1`"]
    #[inline(always)]
    pub fn is_hwts4_1(&self) -> bool {
        *self == HWTS4_A::HWTS4_1
    }
    #[doc = "Checks if the value of the field is `HWTS4_2`"]
    #[inline(always)]
    pub fn is_hwts4_2(&self) -> bool {
        *self == HWTS4_A::HWTS4_2
    }
    #[doc = "Checks if the value of the field is `HWTS4_4`"]
    #[inline(always)]
    pub fn is_hwts4_4(&self) -> bool {
        *self == HWTS4_A::HWTS4_4
    }
    #[doc = "Checks if the value of the field is `HWTS4_8`"]
    #[inline(always)]
    pub fn is_hwts4_8(&self) -> bool {
        *self == HWTS4_A::HWTS4_8
    }
    #[doc = "Checks if the value of the field is `HWTS4_16`"]
    #[inline(always)]
    pub fn is_hwts4_16(&self) -> bool {
        *self == HWTS4_A::HWTS4_16
    }
    #[doc = "Checks if the value of the field is `HWTS4_32`"]
    #[inline(always)]
    pub fn is_hwts4_32(&self) -> bool {
        *self == HWTS4_A::HWTS4_32
    }
    #[doc = "Checks if the value of the field is `HWTS4_64`"]
    #[inline(always)]
    pub fn is_hwts4_64(&self) -> bool {
        *self == HWTS4_A::HWTS4_64
    }
    #[doc = "Checks if the value of the field is `HWTS4_128`"]
    #[inline(always)]
    pub fn is_hwts4_128(&self) -> bool {
        *self == HWTS4_A::HWTS4_128
    }
}
#[doc = "Field `HWTS4` writer - Segment 4 HWTS ADC hardware trigger selection"]
pub type HWTS4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, HWTS4_A, 8, O>;
impl<'a, const O: u8> HWTS4_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts4_0(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts4_1(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts4_2(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts4_4(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts4_8(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts4_16(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts4_32(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts4_64(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts4_128(self) -> &'a mut W {
        self.variant(HWTS4_A::HWTS4_128)
    }
}
#[doc = "Field `B2B4` reader - Segment 4 B2B"]
pub type B2B4_R = crate::BitReader<B2B4_A>;
#[doc = "Segment 4 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B4_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B4_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B4_1 = 1,
}
impl From<B2B4_A> for bool {
    #[inline(always)]
    fn from(variant: B2B4_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B4_A {
        match self.bits {
            false => B2B4_A::B2B4_0,
            true => B2B4_A::B2B4_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B4_0`"]
    #[inline(always)]
    pub fn is_b2b4_0(&self) -> bool {
        *self == B2B4_A::B2B4_0
    }
    #[doc = "Checks if the value of the field is `B2B4_1`"]
    #[inline(always)]
    pub fn is_b2b4_1(&self) -> bool {
        *self == B2B4_A::B2B4_1
    }
}
#[doc = "Field `B2B4` writer - Segment 4 B2B"]
pub type B2B4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG2_CHAIN_5_4_SPEC, B2B4_A, O>;
impl<'a, const O: u8> B2B4_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG4_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b4_0(self) -> &'a mut W {
        self.variant(B2B4_A::B2B4_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b4_1(self) -> &'a mut W {
        self.variant(B2B4_A::B2B4_1)
    }
}
#[doc = "Field `IE4` reader - Segment 4 done interrupt selection"]
pub type IE4_R = crate::FieldReader<u8, IE4_A>;
#[doc = "Segment 4 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE4_A {
    #[doc = "0: No interrupt when finished"]
    IE4_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 4 finish."]
    IE4_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 4 finish."]
    IE4_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 4 finish."]
    IE4_3 = 3,
}
impl From<IE4_A> for u8 {
    #[inline(always)]
    fn from(variant: IE4_A) -> Self {
        variant as _
    }
}
impl IE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE4_A {
        match self.bits {
            0 => IE4_A::IE4_0,
            1 => IE4_A::IE4_1,
            2 => IE4_A::IE4_2,
            3 => IE4_A::IE4_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE4_0`"]
    #[inline(always)]
    pub fn is_ie4_0(&self) -> bool {
        *self == IE4_A::IE4_0
    }
    #[doc = "Checks if the value of the field is `IE4_1`"]
    #[inline(always)]
    pub fn is_ie4_1(&self) -> bool {
        *self == IE4_A::IE4_1
    }
    #[doc = "Checks if the value of the field is `IE4_2`"]
    #[inline(always)]
    pub fn is_ie4_2(&self) -> bool {
        *self == IE4_A::IE4_2
    }
    #[doc = "Checks if the value of the field is `IE4_3`"]
    #[inline(always)]
    pub fn is_ie4_3(&self) -> bool {
        *self == IE4_A::IE4_3
    }
}
#[doc = "Field `IE4` writer - Segment 4 done interrupt selection"]
pub type IE4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, IE4_A, 2, O>;
impl<'a, const O: u8> IE4_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie4_0(self) -> &'a mut W {
        self.variant(IE4_A::IE4_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 4 finish."]
    #[inline(always)]
    pub fn ie4_1(self) -> &'a mut W {
        self.variant(IE4_A::IE4_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 4 finish."]
    #[inline(always)]
    pub fn ie4_2(self) -> &'a mut W {
        self.variant(IE4_A::IE4_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 4 finish."]
    #[inline(always)]
    pub fn ie4_3(self) -> &'a mut W {
        self.variant(IE4_A::IE4_3)
    }
}
#[doc = "Field `CSEL5` reader - ADC channel selection"]
pub type CSEL5_R = crate::FieldReader<u8, CSEL5_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL5_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL5_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL5_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL5_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL5_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL5_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL5_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL5_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL5_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL5_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL5_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL5_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL5_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL5_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL5_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL5_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL5_15 = 15,
}
impl From<CSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL5_A) -> Self {
        variant as _
    }
}
impl CSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL5_A {
        match self.bits {
            0 => CSEL5_A::CSEL5_0,
            1 => CSEL5_A::CSEL5_1,
            2 => CSEL5_A::CSEL5_2,
            3 => CSEL5_A::CSEL5_3,
            4 => CSEL5_A::CSEL5_4,
            5 => CSEL5_A::CSEL5_5,
            6 => CSEL5_A::CSEL5_6,
            7 => CSEL5_A::CSEL5_7,
            8 => CSEL5_A::CSEL5_8,
            9 => CSEL5_A::CSEL5_9,
            10 => CSEL5_A::CSEL5_10,
            11 => CSEL5_A::CSEL5_11,
            12 => CSEL5_A::CSEL5_12,
            13 => CSEL5_A::CSEL5_13,
            14 => CSEL5_A::CSEL5_14,
            15 => CSEL5_A::CSEL5_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL5_0`"]
    #[inline(always)]
    pub fn is_csel5_0(&self) -> bool {
        *self == CSEL5_A::CSEL5_0
    }
    #[doc = "Checks if the value of the field is `CSEL5_1`"]
    #[inline(always)]
    pub fn is_csel5_1(&self) -> bool {
        *self == CSEL5_A::CSEL5_1
    }
    #[doc = "Checks if the value of the field is `CSEL5_2`"]
    #[inline(always)]
    pub fn is_csel5_2(&self) -> bool {
        *self == CSEL5_A::CSEL5_2
    }
    #[doc = "Checks if the value of the field is `CSEL5_3`"]
    #[inline(always)]
    pub fn is_csel5_3(&self) -> bool {
        *self == CSEL5_A::CSEL5_3
    }
    #[doc = "Checks if the value of the field is `CSEL5_4`"]
    #[inline(always)]
    pub fn is_csel5_4(&self) -> bool {
        *self == CSEL5_A::CSEL5_4
    }
    #[doc = "Checks if the value of the field is `CSEL5_5`"]
    #[inline(always)]
    pub fn is_csel5_5(&self) -> bool {
        *self == CSEL5_A::CSEL5_5
    }
    #[doc = "Checks if the value of the field is `CSEL5_6`"]
    #[inline(always)]
    pub fn is_csel5_6(&self) -> bool {
        *self == CSEL5_A::CSEL5_6
    }
    #[doc = "Checks if the value of the field is `CSEL5_7`"]
    #[inline(always)]
    pub fn is_csel5_7(&self) -> bool {
        *self == CSEL5_A::CSEL5_7
    }
    #[doc = "Checks if the value of the field is `CSEL5_8`"]
    #[inline(always)]
    pub fn is_csel5_8(&self) -> bool {
        *self == CSEL5_A::CSEL5_8
    }
    #[doc = "Checks if the value of the field is `CSEL5_9`"]
    #[inline(always)]
    pub fn is_csel5_9(&self) -> bool {
        *self == CSEL5_A::CSEL5_9
    }
    #[doc = "Checks if the value of the field is `CSEL5_10`"]
    #[inline(always)]
    pub fn is_csel5_10(&self) -> bool {
        *self == CSEL5_A::CSEL5_10
    }
    #[doc = "Checks if the value of the field is `CSEL5_11`"]
    #[inline(always)]
    pub fn is_csel5_11(&self) -> bool {
        *self == CSEL5_A::CSEL5_11
    }
    #[doc = "Checks if the value of the field is `CSEL5_12`"]
    #[inline(always)]
    pub fn is_csel5_12(&self) -> bool {
        *self == CSEL5_A::CSEL5_12
    }
    #[doc = "Checks if the value of the field is `CSEL5_13`"]
    #[inline(always)]
    pub fn is_csel5_13(&self) -> bool {
        *self == CSEL5_A::CSEL5_13
    }
    #[doc = "Checks if the value of the field is `CSEL5_14`"]
    #[inline(always)]
    pub fn is_csel5_14(&self) -> bool {
        *self == CSEL5_A::CSEL5_14
    }
    #[doc = "Checks if the value of the field is `CSEL5_15`"]
    #[inline(always)]
    pub fn is_csel5_15(&self) -> bool {
        *self == CSEL5_A::CSEL5_15
    }
}
#[doc = "Field `CSEL5` writer - ADC channel selection"]
pub type CSEL5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, CSEL5_A, 4, O>;
impl<'a, const O: u8> CSEL5_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel5_0(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel5_1(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel5_2(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel5_3(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel5_4(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel5_5(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel5_6(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel5_7(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel5_8(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel5_9(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel5_10(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel5_11(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel5_12(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel5_13(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel5_14(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel5_15(self) -> &'a mut W {
        self.variant(CSEL5_A::CSEL5_15)
    }
}
#[doc = "Field `HWTS5` reader - Segment 5 HWTS ADC hardware trigger selection"]
pub type HWTS5_R = crate::FieldReader<u8, HWTS5_A>;
#[doc = "Segment 5 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS5_A {
    #[doc = "0: no trigger selected"]
    HWTS5_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS5_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS5_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS5_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS5_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS5_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS5_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS5_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS5_128 = 128,
}
impl From<HWTS5_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS5_A) -> Self {
        variant as _
    }
}
impl HWTS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS5_A> {
        match self.bits {
            0 => Some(HWTS5_A::HWTS5_0),
            1 => Some(HWTS5_A::HWTS5_1),
            2 => Some(HWTS5_A::HWTS5_2),
            4 => Some(HWTS5_A::HWTS5_4),
            8 => Some(HWTS5_A::HWTS5_8),
            16 => Some(HWTS5_A::HWTS5_16),
            32 => Some(HWTS5_A::HWTS5_32),
            64 => Some(HWTS5_A::HWTS5_64),
            128 => Some(HWTS5_A::HWTS5_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS5_0`"]
    #[inline(always)]
    pub fn is_hwts5_0(&self) -> bool {
        *self == HWTS5_A::HWTS5_0
    }
    #[doc = "Checks if the value of the field is `HWTS5_1`"]
    #[inline(always)]
    pub fn is_hwts5_1(&self) -> bool {
        *self == HWTS5_A::HWTS5_1
    }
    #[doc = "Checks if the value of the field is `HWTS5_2`"]
    #[inline(always)]
    pub fn is_hwts5_2(&self) -> bool {
        *self == HWTS5_A::HWTS5_2
    }
    #[doc = "Checks if the value of the field is `HWTS5_4`"]
    #[inline(always)]
    pub fn is_hwts5_4(&self) -> bool {
        *self == HWTS5_A::HWTS5_4
    }
    #[doc = "Checks if the value of the field is `HWTS5_8`"]
    #[inline(always)]
    pub fn is_hwts5_8(&self) -> bool {
        *self == HWTS5_A::HWTS5_8
    }
    #[doc = "Checks if the value of the field is `HWTS5_16`"]
    #[inline(always)]
    pub fn is_hwts5_16(&self) -> bool {
        *self == HWTS5_A::HWTS5_16
    }
    #[doc = "Checks if the value of the field is `HWTS5_32`"]
    #[inline(always)]
    pub fn is_hwts5_32(&self) -> bool {
        *self == HWTS5_A::HWTS5_32
    }
    #[doc = "Checks if the value of the field is `HWTS5_64`"]
    #[inline(always)]
    pub fn is_hwts5_64(&self) -> bool {
        *self == HWTS5_A::HWTS5_64
    }
    #[doc = "Checks if the value of the field is `HWTS5_128`"]
    #[inline(always)]
    pub fn is_hwts5_128(&self) -> bool {
        *self == HWTS5_A::HWTS5_128
    }
}
#[doc = "Field `HWTS5` writer - Segment 5 HWTS ADC hardware trigger selection"]
pub type HWTS5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, HWTS5_A, 8, O>;
impl<'a, const O: u8> HWTS5_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts5_0(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts5_1(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts5_2(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts5_4(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts5_8(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts5_16(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts5_32(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts5_64(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts5_128(self) -> &'a mut W {
        self.variant(HWTS5_A::HWTS5_128)
    }
}
#[doc = "Field `B2B5` reader - Segment 5 B2B"]
pub type B2B5_R = crate::BitReader<B2B5_A>;
#[doc = "Segment 5 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B5_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B5_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B5_1 = 1,
}
impl From<B2B5_A> for bool {
    #[inline(always)]
    fn from(variant: B2B5_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B5_A {
        match self.bits {
            false => B2B5_A::B2B5_0,
            true => B2B5_A::B2B5_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B5_0`"]
    #[inline(always)]
    pub fn is_b2b5_0(&self) -> bool {
        *self == B2B5_A::B2B5_0
    }
    #[doc = "Checks if the value of the field is `B2B5_1`"]
    #[inline(always)]
    pub fn is_b2b5_1(&self) -> bool {
        *self == B2B5_A::B2B5_1
    }
}
#[doc = "Field `B2B5` writer - Segment 5 B2B"]
pub type B2B5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG2_CHAIN_5_4_SPEC, B2B5_A, O>;
impl<'a, const O: u8> B2B5_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG5_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b5_0(self) -> &'a mut W {
        self.variant(B2B5_A::B2B5_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b5_1(self) -> &'a mut W {
        self.variant(B2B5_A::B2B5_1)
    }
}
#[doc = "Field `IE5` reader - Segment 5 done interrupt selection"]
pub type IE5_R = crate::FieldReader<u8, IE5_A>;
#[doc = "Segment 5 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE5_A {
    #[doc = "0: No interrupt when finished"]
    IE5_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 5 finish."]
    IE5_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 5 finish."]
    IE5_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 5 finish."]
    IE5_3 = 3,
}
impl From<IE5_A> for u8 {
    #[inline(always)]
    fn from(variant: IE5_A) -> Self {
        variant as _
    }
}
impl IE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE5_A {
        match self.bits {
            0 => IE5_A::IE5_0,
            1 => IE5_A::IE5_1,
            2 => IE5_A::IE5_2,
            3 => IE5_A::IE5_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE5_0`"]
    #[inline(always)]
    pub fn is_ie5_0(&self) -> bool {
        *self == IE5_A::IE5_0
    }
    #[doc = "Checks if the value of the field is `IE5_1`"]
    #[inline(always)]
    pub fn is_ie5_1(&self) -> bool {
        *self == IE5_A::IE5_1
    }
    #[doc = "Checks if the value of the field is `IE5_2`"]
    #[inline(always)]
    pub fn is_ie5_2(&self) -> bool {
        *self == IE5_A::IE5_2
    }
    #[doc = "Checks if the value of the field is `IE5_3`"]
    #[inline(always)]
    pub fn is_ie5_3(&self) -> bool {
        *self == IE5_A::IE5_3
    }
}
#[doc = "Field `IE5` writer - Segment 5 done interrupt selection"]
pub type IE5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG2_CHAIN_5_4_SPEC, u8, IE5_A, 2, O>;
impl<'a, const O: u8> IE5_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie5_0(self) -> &'a mut W {
        self.variant(IE5_A::IE5_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 5 finish."]
    #[inline(always)]
    pub fn ie5_1(self) -> &'a mut W {
        self.variant(IE5_A::IE5_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 5 finish."]
    #[inline(always)]
    pub fn ie5_2(self) -> &'a mut W {
        self.variant(IE5_A::IE5_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 5 finish."]
    #[inline(always)]
    pub fn ie5_3(self) -> &'a mut W {
        self.variant(IE5_A::IE5_3)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    pub fn csel4(&self) -> CSEL4_R {
        CSEL4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts4(&self) -> HWTS4_R {
        HWTS4_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Segment 4 B2B"]
    #[inline(always)]
    pub fn b2b4(&self) -> B2B4_R {
        B2B4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Segment 4 done interrupt selection"]
    #[inline(always)]
    pub fn ie4(&self) -> IE4_R {
        IE4_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    pub fn csel5(&self) -> CSEL5_R {
        CSEL5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts5(&self) -> HWTS5_R {
        HWTS5_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Segment 5 B2B"]
    #[inline(always)]
    pub fn b2b5(&self) -> B2B5_R {
        B2B5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Segment 5 done interrupt selection"]
    #[inline(always)]
    pub fn ie5(&self) -> IE5_R {
        IE5_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel4(&mut self) -> CSEL4_W<0> {
        CSEL4_W::new(self)
    }
    #[doc = "Bits 4:11 - Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts4(&mut self) -> HWTS4_W<4> {
        HWTS4_W::new(self)
    }
    #[doc = "Bit 12 - Segment 4 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b4(&mut self) -> B2B4_W<12> {
        B2B4_W::new(self)
    }
    #[doc = "Bits 13:14 - Segment 4 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie4(&mut self) -> IE4_W<13> {
        IE4_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel5(&mut self) -> CSEL5_W<16> {
        CSEL5_W::new(self)
    }
    #[doc = "Bits 20:27 - Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts5(&mut self) -> HWTS5_W<20> {
        HWTS5_W::new(self)
    }
    #[doc = "Bit 28 - Segment 5 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b5(&mut self) -> B2B5_W<28> {
        B2B5_W::new(self)
    }
    #[doc = "Bits 29:30 - Segment 5 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie5(&mut self) -> IE5_W<29> {
        IE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_chain_5_4](index.html) module"]
pub struct TRIG2_CHAIN_5_4_SPEC;
impl crate::RegisterSpec for TRIG2_CHAIN_5_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig2_chain_5_4::R](R) reader structure"]
impl crate::Readable for TRIG2_CHAIN_5_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig2_chain_5_4::W](W) writer structure"]
impl crate::Writable for TRIG2_CHAIN_5_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG2_CHAIN_5_4 to value 0"]
impl crate::Resettable for TRIG2_CHAIN_5_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
