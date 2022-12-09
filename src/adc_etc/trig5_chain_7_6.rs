#[doc = "Register `TRIG5_CHAIN_7_6` reader"]
pub struct R(crate::R<TRIG5_CHAIN_7_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG5_CHAIN_7_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG5_CHAIN_7_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG5_CHAIN_7_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG5_CHAIN_7_6` writer"]
pub struct W(crate::W<TRIG5_CHAIN_7_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG5_CHAIN_7_6_SPEC>;
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
impl From<crate::W<TRIG5_CHAIN_7_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG5_CHAIN_7_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEL6` reader - ADC channel selection"]
pub type CSEL6_R = crate::FieldReader<u8, CSEL6_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL6_A {
    #[doc = "0: ADC Channel 0 selected"]
    CSEL6_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL6_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL6_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL6_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL6_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL6_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL6_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL6_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL6_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL6_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL6_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL6_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL6_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL6_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL6_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL6_15 = 15,
}
impl From<CSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL6_A) -> Self {
        variant as _
    }
}
impl CSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL6_A {
        match self.bits {
            0 => CSEL6_A::CSEL6_0,
            1 => CSEL6_A::CSEL6_1,
            2 => CSEL6_A::CSEL6_2,
            3 => CSEL6_A::CSEL6_3,
            4 => CSEL6_A::CSEL6_4,
            5 => CSEL6_A::CSEL6_5,
            6 => CSEL6_A::CSEL6_6,
            7 => CSEL6_A::CSEL6_7,
            8 => CSEL6_A::CSEL6_8,
            9 => CSEL6_A::CSEL6_9,
            10 => CSEL6_A::CSEL6_10,
            11 => CSEL6_A::CSEL6_11,
            12 => CSEL6_A::CSEL6_12,
            13 => CSEL6_A::CSEL6_13,
            14 => CSEL6_A::CSEL6_14,
            15 => CSEL6_A::CSEL6_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL6_0`"]
    #[inline(always)]
    pub fn is_csel6_0(&self) -> bool {
        *self == CSEL6_A::CSEL6_0
    }
    #[doc = "Checks if the value of the field is `CSEL6_1`"]
    #[inline(always)]
    pub fn is_csel6_1(&self) -> bool {
        *self == CSEL6_A::CSEL6_1
    }
    #[doc = "Checks if the value of the field is `CSEL6_2`"]
    #[inline(always)]
    pub fn is_csel6_2(&self) -> bool {
        *self == CSEL6_A::CSEL6_2
    }
    #[doc = "Checks if the value of the field is `CSEL6_3`"]
    #[inline(always)]
    pub fn is_csel6_3(&self) -> bool {
        *self == CSEL6_A::CSEL6_3
    }
    #[doc = "Checks if the value of the field is `CSEL6_4`"]
    #[inline(always)]
    pub fn is_csel6_4(&self) -> bool {
        *self == CSEL6_A::CSEL6_4
    }
    #[doc = "Checks if the value of the field is `CSEL6_5`"]
    #[inline(always)]
    pub fn is_csel6_5(&self) -> bool {
        *self == CSEL6_A::CSEL6_5
    }
    #[doc = "Checks if the value of the field is `CSEL6_6`"]
    #[inline(always)]
    pub fn is_csel6_6(&self) -> bool {
        *self == CSEL6_A::CSEL6_6
    }
    #[doc = "Checks if the value of the field is `CSEL6_7`"]
    #[inline(always)]
    pub fn is_csel6_7(&self) -> bool {
        *self == CSEL6_A::CSEL6_7
    }
    #[doc = "Checks if the value of the field is `CSEL6_8`"]
    #[inline(always)]
    pub fn is_csel6_8(&self) -> bool {
        *self == CSEL6_A::CSEL6_8
    }
    #[doc = "Checks if the value of the field is `CSEL6_9`"]
    #[inline(always)]
    pub fn is_csel6_9(&self) -> bool {
        *self == CSEL6_A::CSEL6_9
    }
    #[doc = "Checks if the value of the field is `CSEL6_10`"]
    #[inline(always)]
    pub fn is_csel6_10(&self) -> bool {
        *self == CSEL6_A::CSEL6_10
    }
    #[doc = "Checks if the value of the field is `CSEL6_11`"]
    #[inline(always)]
    pub fn is_csel6_11(&self) -> bool {
        *self == CSEL6_A::CSEL6_11
    }
    #[doc = "Checks if the value of the field is `CSEL6_12`"]
    #[inline(always)]
    pub fn is_csel6_12(&self) -> bool {
        *self == CSEL6_A::CSEL6_12
    }
    #[doc = "Checks if the value of the field is `CSEL6_13`"]
    #[inline(always)]
    pub fn is_csel6_13(&self) -> bool {
        *self == CSEL6_A::CSEL6_13
    }
    #[doc = "Checks if the value of the field is `CSEL6_14`"]
    #[inline(always)]
    pub fn is_csel6_14(&self) -> bool {
        *self == CSEL6_A::CSEL6_14
    }
    #[doc = "Checks if the value of the field is `CSEL6_15`"]
    #[inline(always)]
    pub fn is_csel6_15(&self) -> bool {
        *self == CSEL6_A::CSEL6_15
    }
}
#[doc = "Field `CSEL6` writer - ADC channel selection"]
pub type CSEL6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, CSEL6_A, 4, O>;
impl<'a, const O: u8> CSEL6_W<'a, O> {
    #[doc = "ADC Channel 0 selected"]
    #[inline(always)]
    pub fn csel6_0(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel6_1(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel6_2(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel6_3(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel6_4(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel6_5(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel6_6(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel6_7(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel6_8(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel6_9(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel6_10(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel6_11(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel6_12(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel6_13(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel6_14(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel6_15(self) -> &'a mut W {
        self.variant(CSEL6_A::CSEL6_15)
    }
}
#[doc = "Field `HWTS6` reader - Segment 6 HWTS ADC hardware trigger selection"]
pub type HWTS6_R = crate::FieldReader<u8, HWTS6_A>;
#[doc = "Segment 6 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS6_A {
    #[doc = "0: no trigger selected"]
    HWTS6_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS6_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS6_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS6_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS6_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS6_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS6_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS6_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS6_128 = 128,
}
impl From<HWTS6_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS6_A) -> Self {
        variant as _
    }
}
impl HWTS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS6_A> {
        match self.bits {
            0 => Some(HWTS6_A::HWTS6_0),
            1 => Some(HWTS6_A::HWTS6_1),
            2 => Some(HWTS6_A::HWTS6_2),
            4 => Some(HWTS6_A::HWTS6_4),
            8 => Some(HWTS6_A::HWTS6_8),
            16 => Some(HWTS6_A::HWTS6_16),
            32 => Some(HWTS6_A::HWTS6_32),
            64 => Some(HWTS6_A::HWTS6_64),
            128 => Some(HWTS6_A::HWTS6_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS6_0`"]
    #[inline(always)]
    pub fn is_hwts6_0(&self) -> bool {
        *self == HWTS6_A::HWTS6_0
    }
    #[doc = "Checks if the value of the field is `HWTS6_1`"]
    #[inline(always)]
    pub fn is_hwts6_1(&self) -> bool {
        *self == HWTS6_A::HWTS6_1
    }
    #[doc = "Checks if the value of the field is `HWTS6_2`"]
    #[inline(always)]
    pub fn is_hwts6_2(&self) -> bool {
        *self == HWTS6_A::HWTS6_2
    }
    #[doc = "Checks if the value of the field is `HWTS6_4`"]
    #[inline(always)]
    pub fn is_hwts6_4(&self) -> bool {
        *self == HWTS6_A::HWTS6_4
    }
    #[doc = "Checks if the value of the field is `HWTS6_8`"]
    #[inline(always)]
    pub fn is_hwts6_8(&self) -> bool {
        *self == HWTS6_A::HWTS6_8
    }
    #[doc = "Checks if the value of the field is `HWTS6_16`"]
    #[inline(always)]
    pub fn is_hwts6_16(&self) -> bool {
        *self == HWTS6_A::HWTS6_16
    }
    #[doc = "Checks if the value of the field is `HWTS6_32`"]
    #[inline(always)]
    pub fn is_hwts6_32(&self) -> bool {
        *self == HWTS6_A::HWTS6_32
    }
    #[doc = "Checks if the value of the field is `HWTS6_64`"]
    #[inline(always)]
    pub fn is_hwts6_64(&self) -> bool {
        *self == HWTS6_A::HWTS6_64
    }
    #[doc = "Checks if the value of the field is `HWTS6_128`"]
    #[inline(always)]
    pub fn is_hwts6_128(&self) -> bool {
        *self == HWTS6_A::HWTS6_128
    }
}
#[doc = "Field `HWTS6` writer - Segment 6 HWTS ADC hardware trigger selection"]
pub type HWTS6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, HWTS6_A, 8, O>;
impl<'a, const O: u8> HWTS6_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts6_0(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts6_1(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts6_2(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts6_4(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts6_8(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts6_16(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts6_32(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts6_64(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts6_128(self) -> &'a mut W {
        self.variant(HWTS6_A::HWTS6_128)
    }
}
#[doc = "Field `B2B6` reader - Segment 6 B2B"]
pub type B2B6_R = crate::BitReader<B2B6_A>;
#[doc = "Segment 6 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B6_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B6_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B6_1 = 1,
}
impl From<B2B6_A> for bool {
    #[inline(always)]
    fn from(variant: B2B6_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B6_A {
        match self.bits {
            false => B2B6_A::B2B6_0,
            true => B2B6_A::B2B6_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B6_0`"]
    #[inline(always)]
    pub fn is_b2b6_0(&self) -> bool {
        *self == B2B6_A::B2B6_0
    }
    #[doc = "Checks if the value of the field is `B2B6_1`"]
    #[inline(always)]
    pub fn is_b2b6_1(&self) -> bool {
        *self == B2B6_A::B2B6_1
    }
}
#[doc = "Field `B2B6` writer - Segment 6 B2B"]
pub type B2B6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG5_CHAIN_7_6_SPEC, B2B6_A, O>;
impl<'a, const O: u8> B2B6_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG6_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b6_0(self) -> &'a mut W {
        self.variant(B2B6_A::B2B6_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b6_1(self) -> &'a mut W {
        self.variant(B2B6_A::B2B6_1)
    }
}
#[doc = "Field `IE6` reader - Segment 6 done interrupt selection"]
pub type IE6_R = crate::FieldReader<u8, IE6_A>;
#[doc = "Segment 6 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE6_A {
    #[doc = "0: No interrupt when finished"]
    IE6_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 6 finish."]
    IE6_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 6 finish."]
    IE6_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 6 finish."]
    IE6_3 = 3,
}
impl From<IE6_A> for u8 {
    #[inline(always)]
    fn from(variant: IE6_A) -> Self {
        variant as _
    }
}
impl IE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE6_A {
        match self.bits {
            0 => IE6_A::IE6_0,
            1 => IE6_A::IE6_1,
            2 => IE6_A::IE6_2,
            3 => IE6_A::IE6_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE6_0`"]
    #[inline(always)]
    pub fn is_ie6_0(&self) -> bool {
        *self == IE6_A::IE6_0
    }
    #[doc = "Checks if the value of the field is `IE6_1`"]
    #[inline(always)]
    pub fn is_ie6_1(&self) -> bool {
        *self == IE6_A::IE6_1
    }
    #[doc = "Checks if the value of the field is `IE6_2`"]
    #[inline(always)]
    pub fn is_ie6_2(&self) -> bool {
        *self == IE6_A::IE6_2
    }
    #[doc = "Checks if the value of the field is `IE6_3`"]
    #[inline(always)]
    pub fn is_ie6_3(&self) -> bool {
        *self == IE6_A::IE6_3
    }
}
#[doc = "Field `IE6` writer - Segment 6 done interrupt selection"]
pub type IE6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, IE6_A, 2, O>;
impl<'a, const O: u8> IE6_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie6_0(self) -> &'a mut W {
        self.variant(IE6_A::IE6_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 6 finish."]
    #[inline(always)]
    pub fn ie6_1(self) -> &'a mut W {
        self.variant(IE6_A::IE6_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 6 finish."]
    #[inline(always)]
    pub fn ie6_2(self) -> &'a mut W {
        self.variant(IE6_A::IE6_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 6 finish."]
    #[inline(always)]
    pub fn ie6_3(self) -> &'a mut W {
        self.variant(IE6_A::IE6_3)
    }
}
#[doc = "Field `CSEL7` reader - ADC channel selection"]
pub type CSEL7_R = crate::FieldReader<u8, CSEL7_A>;
#[doc = "ADC channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSEL7_A {
    #[doc = "0: ADC Channel 0 selected."]
    CSEL7_0 = 0,
    #[doc = "1: ADC Channel 1 selected."]
    CSEL7_1 = 1,
    #[doc = "2: ADC Channel 2 selected."]
    CSEL7_2 = 2,
    #[doc = "3: ADC Channel 3 selected."]
    CSEL7_3 = 3,
    #[doc = "4: ADC Channel 4 selected."]
    CSEL7_4 = 4,
    #[doc = "5: ADC Channel 5 selected."]
    CSEL7_5 = 5,
    #[doc = "6: ADC Channel 6 selected."]
    CSEL7_6 = 6,
    #[doc = "7: ADC Channel 7 selected."]
    CSEL7_7 = 7,
    #[doc = "8: ADC Channel 8 selected."]
    CSEL7_8 = 8,
    #[doc = "9: ADC Channel 9 selected."]
    CSEL7_9 = 9,
    #[doc = "10: ADC Channel 10 selected."]
    CSEL7_10 = 10,
    #[doc = "11: ADC Channel 11 selected."]
    CSEL7_11 = 11,
    #[doc = "12: ADC Channel 12 selected."]
    CSEL7_12 = 12,
    #[doc = "13: ADC Channel 13 selected."]
    CSEL7_13 = 13,
    #[doc = "14: ADC Channel 14 selected."]
    CSEL7_14 = 14,
    #[doc = "15: ADC Channel 15 selected."]
    CSEL7_15 = 15,
}
impl From<CSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL7_A) -> Self {
        variant as _
    }
}
impl CSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEL7_A {
        match self.bits {
            0 => CSEL7_A::CSEL7_0,
            1 => CSEL7_A::CSEL7_1,
            2 => CSEL7_A::CSEL7_2,
            3 => CSEL7_A::CSEL7_3,
            4 => CSEL7_A::CSEL7_4,
            5 => CSEL7_A::CSEL7_5,
            6 => CSEL7_A::CSEL7_6,
            7 => CSEL7_A::CSEL7_7,
            8 => CSEL7_A::CSEL7_8,
            9 => CSEL7_A::CSEL7_9,
            10 => CSEL7_A::CSEL7_10,
            11 => CSEL7_A::CSEL7_11,
            12 => CSEL7_A::CSEL7_12,
            13 => CSEL7_A::CSEL7_13,
            14 => CSEL7_A::CSEL7_14,
            15 => CSEL7_A::CSEL7_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSEL7_0`"]
    #[inline(always)]
    pub fn is_csel7_0(&self) -> bool {
        *self == CSEL7_A::CSEL7_0
    }
    #[doc = "Checks if the value of the field is `CSEL7_1`"]
    #[inline(always)]
    pub fn is_csel7_1(&self) -> bool {
        *self == CSEL7_A::CSEL7_1
    }
    #[doc = "Checks if the value of the field is `CSEL7_2`"]
    #[inline(always)]
    pub fn is_csel7_2(&self) -> bool {
        *self == CSEL7_A::CSEL7_2
    }
    #[doc = "Checks if the value of the field is `CSEL7_3`"]
    #[inline(always)]
    pub fn is_csel7_3(&self) -> bool {
        *self == CSEL7_A::CSEL7_3
    }
    #[doc = "Checks if the value of the field is `CSEL7_4`"]
    #[inline(always)]
    pub fn is_csel7_4(&self) -> bool {
        *self == CSEL7_A::CSEL7_4
    }
    #[doc = "Checks if the value of the field is `CSEL7_5`"]
    #[inline(always)]
    pub fn is_csel7_5(&self) -> bool {
        *self == CSEL7_A::CSEL7_5
    }
    #[doc = "Checks if the value of the field is `CSEL7_6`"]
    #[inline(always)]
    pub fn is_csel7_6(&self) -> bool {
        *self == CSEL7_A::CSEL7_6
    }
    #[doc = "Checks if the value of the field is `CSEL7_7`"]
    #[inline(always)]
    pub fn is_csel7_7(&self) -> bool {
        *self == CSEL7_A::CSEL7_7
    }
    #[doc = "Checks if the value of the field is `CSEL7_8`"]
    #[inline(always)]
    pub fn is_csel7_8(&self) -> bool {
        *self == CSEL7_A::CSEL7_8
    }
    #[doc = "Checks if the value of the field is `CSEL7_9`"]
    #[inline(always)]
    pub fn is_csel7_9(&self) -> bool {
        *self == CSEL7_A::CSEL7_9
    }
    #[doc = "Checks if the value of the field is `CSEL7_10`"]
    #[inline(always)]
    pub fn is_csel7_10(&self) -> bool {
        *self == CSEL7_A::CSEL7_10
    }
    #[doc = "Checks if the value of the field is `CSEL7_11`"]
    #[inline(always)]
    pub fn is_csel7_11(&self) -> bool {
        *self == CSEL7_A::CSEL7_11
    }
    #[doc = "Checks if the value of the field is `CSEL7_12`"]
    #[inline(always)]
    pub fn is_csel7_12(&self) -> bool {
        *self == CSEL7_A::CSEL7_12
    }
    #[doc = "Checks if the value of the field is `CSEL7_13`"]
    #[inline(always)]
    pub fn is_csel7_13(&self) -> bool {
        *self == CSEL7_A::CSEL7_13
    }
    #[doc = "Checks if the value of the field is `CSEL7_14`"]
    #[inline(always)]
    pub fn is_csel7_14(&self) -> bool {
        *self == CSEL7_A::CSEL7_14
    }
    #[doc = "Checks if the value of the field is `CSEL7_15`"]
    #[inline(always)]
    pub fn is_csel7_15(&self) -> bool {
        *self == CSEL7_A::CSEL7_15
    }
}
#[doc = "Field `CSEL7` writer - ADC channel selection"]
pub type CSEL7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, CSEL7_A, 4, O>;
impl<'a, const O: u8> CSEL7_W<'a, O> {
    #[doc = "ADC Channel 0 selected."]
    #[inline(always)]
    pub fn csel7_0(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_0)
    }
    #[doc = "ADC Channel 1 selected."]
    #[inline(always)]
    pub fn csel7_1(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_1)
    }
    #[doc = "ADC Channel 2 selected."]
    #[inline(always)]
    pub fn csel7_2(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_2)
    }
    #[doc = "ADC Channel 3 selected."]
    #[inline(always)]
    pub fn csel7_3(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_3)
    }
    #[doc = "ADC Channel 4 selected."]
    #[inline(always)]
    pub fn csel7_4(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_4)
    }
    #[doc = "ADC Channel 5 selected."]
    #[inline(always)]
    pub fn csel7_5(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_5)
    }
    #[doc = "ADC Channel 6 selected."]
    #[inline(always)]
    pub fn csel7_6(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_6)
    }
    #[doc = "ADC Channel 7 selected."]
    #[inline(always)]
    pub fn csel7_7(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_7)
    }
    #[doc = "ADC Channel 8 selected."]
    #[inline(always)]
    pub fn csel7_8(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_8)
    }
    #[doc = "ADC Channel 9 selected."]
    #[inline(always)]
    pub fn csel7_9(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_9)
    }
    #[doc = "ADC Channel 10 selected."]
    #[inline(always)]
    pub fn csel7_10(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_10)
    }
    #[doc = "ADC Channel 11 selected."]
    #[inline(always)]
    pub fn csel7_11(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_11)
    }
    #[doc = "ADC Channel 12 selected."]
    #[inline(always)]
    pub fn csel7_12(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_12)
    }
    #[doc = "ADC Channel 13 selected."]
    #[inline(always)]
    pub fn csel7_13(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_13)
    }
    #[doc = "ADC Channel 14 selected."]
    #[inline(always)]
    pub fn csel7_14(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_14)
    }
    #[doc = "ADC Channel 15 selected."]
    #[inline(always)]
    pub fn csel7_15(self) -> &'a mut W {
        self.variant(CSEL7_A::CSEL7_15)
    }
}
#[doc = "Field `HWTS7` reader - Segment 7 HWTS ADC hardware trigger selection"]
pub type HWTS7_R = crate::FieldReader<u8, HWTS7_A>;
#[doc = "Segment 7 HWTS ADC hardware trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HWTS7_A {
    #[doc = "0: no trigger selected"]
    HWTS7_0 = 0,
    #[doc = "1: ADC TRIG0 selected"]
    HWTS7_1 = 1,
    #[doc = "2: ADC TRIG1 selected"]
    HWTS7_2 = 2,
    #[doc = "4: ADC TRIG2 selected"]
    HWTS7_4 = 4,
    #[doc = "8: ADC TRIG3 selected"]
    HWTS7_8 = 8,
    #[doc = "16: ADC TRIG4 selected"]
    HWTS7_16 = 16,
    #[doc = "32: ADC TRIG5 selected"]
    HWTS7_32 = 32,
    #[doc = "64: ADC TRIG6 selected"]
    HWTS7_64 = 64,
    #[doc = "128: ADC TRIG7 selected"]
    HWTS7_128 = 128,
}
impl From<HWTS7_A> for u8 {
    #[inline(always)]
    fn from(variant: HWTS7_A) -> Self {
        variant as _
    }
}
impl HWTS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HWTS7_A> {
        match self.bits {
            0 => Some(HWTS7_A::HWTS7_0),
            1 => Some(HWTS7_A::HWTS7_1),
            2 => Some(HWTS7_A::HWTS7_2),
            4 => Some(HWTS7_A::HWTS7_4),
            8 => Some(HWTS7_A::HWTS7_8),
            16 => Some(HWTS7_A::HWTS7_16),
            32 => Some(HWTS7_A::HWTS7_32),
            64 => Some(HWTS7_A::HWTS7_64),
            128 => Some(HWTS7_A::HWTS7_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HWTS7_0`"]
    #[inline(always)]
    pub fn is_hwts7_0(&self) -> bool {
        *self == HWTS7_A::HWTS7_0
    }
    #[doc = "Checks if the value of the field is `HWTS7_1`"]
    #[inline(always)]
    pub fn is_hwts7_1(&self) -> bool {
        *self == HWTS7_A::HWTS7_1
    }
    #[doc = "Checks if the value of the field is `HWTS7_2`"]
    #[inline(always)]
    pub fn is_hwts7_2(&self) -> bool {
        *self == HWTS7_A::HWTS7_2
    }
    #[doc = "Checks if the value of the field is `HWTS7_4`"]
    #[inline(always)]
    pub fn is_hwts7_4(&self) -> bool {
        *self == HWTS7_A::HWTS7_4
    }
    #[doc = "Checks if the value of the field is `HWTS7_8`"]
    #[inline(always)]
    pub fn is_hwts7_8(&self) -> bool {
        *self == HWTS7_A::HWTS7_8
    }
    #[doc = "Checks if the value of the field is `HWTS7_16`"]
    #[inline(always)]
    pub fn is_hwts7_16(&self) -> bool {
        *self == HWTS7_A::HWTS7_16
    }
    #[doc = "Checks if the value of the field is `HWTS7_32`"]
    #[inline(always)]
    pub fn is_hwts7_32(&self) -> bool {
        *self == HWTS7_A::HWTS7_32
    }
    #[doc = "Checks if the value of the field is `HWTS7_64`"]
    #[inline(always)]
    pub fn is_hwts7_64(&self) -> bool {
        *self == HWTS7_A::HWTS7_64
    }
    #[doc = "Checks if the value of the field is `HWTS7_128`"]
    #[inline(always)]
    pub fn is_hwts7_128(&self) -> bool {
        *self == HWTS7_A::HWTS7_128
    }
}
#[doc = "Field `HWTS7` writer - Segment 7 HWTS ADC hardware trigger selection"]
pub type HWTS7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, HWTS7_A, 8, O>;
impl<'a, const O: u8> HWTS7_W<'a, O> {
    #[doc = "no trigger selected"]
    #[inline(always)]
    pub fn hwts7_0(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_0)
    }
    #[doc = "ADC TRIG0 selected"]
    #[inline(always)]
    pub fn hwts7_1(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_1)
    }
    #[doc = "ADC TRIG1 selected"]
    #[inline(always)]
    pub fn hwts7_2(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_2)
    }
    #[doc = "ADC TRIG2 selected"]
    #[inline(always)]
    pub fn hwts7_4(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_4)
    }
    #[doc = "ADC TRIG3 selected"]
    #[inline(always)]
    pub fn hwts7_8(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_8)
    }
    #[doc = "ADC TRIG4 selected"]
    #[inline(always)]
    pub fn hwts7_16(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_16)
    }
    #[doc = "ADC TRIG5 selected"]
    #[inline(always)]
    pub fn hwts7_32(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_32)
    }
    #[doc = "ADC TRIG6 selected"]
    #[inline(always)]
    pub fn hwts7_64(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_64)
    }
    #[doc = "ADC TRIG7 selected"]
    #[inline(always)]
    pub fn hwts7_128(self) -> &'a mut W {
        self.variant(HWTS7_A::HWTS7_128)
    }
}
#[doc = "Field `B2B7` reader - Segment 7 B2B"]
pub type B2B7_R = crate::BitReader<B2B7_A>;
#[doc = "Segment 7 B2B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2B7_A {
    #[doc = "0: Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    B2B7_0 = 0,
    #[doc = "1: Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    B2B7_1 = 1,
}
impl From<B2B7_A> for bool {
    #[inline(always)]
    fn from(variant: B2B7_A) -> Self {
        variant as u8 != 0
    }
}
impl B2B7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B2B7_A {
        match self.bits {
            false => B2B7_A::B2B7_0,
            true => B2B7_A::B2B7_1,
        }
    }
    #[doc = "Checks if the value of the field is `B2B7_0`"]
    #[inline(always)]
    pub fn is_b2b7_0(&self) -> bool {
        *self == B2B7_A::B2B7_0
    }
    #[doc = "Checks if the value of the field is `B2B7_1`"]
    #[inline(always)]
    pub fn is_b2b7_1(&self) -> bool {
        *self == B2B7_A::B2B7_1
    }
}
#[doc = "Field `B2B7` writer - Segment 7 B2B"]
pub type B2B7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIG5_CHAIN_7_6_SPEC, B2B7_A, O>;
impl<'a, const O: u8> B2B7_W<'a, O> {
    #[doc = "Disable B2B. Wait until delay value defined by TRIG7_COUNTER\\[SAMPLE_INTERVAL\\]
is reached"]
    #[inline(always)]
    pub fn b2b7_0(self) -> &'a mut W {
        self.variant(B2B7_A::B2B7_0)
    }
    #[doc = "Enable B2B. When Segment 0 finished (ADC COCO) then automatically trigger next ADC conversion, no need to wait until interval delay reached."]
    #[inline(always)]
    pub fn b2b7_1(self) -> &'a mut W {
        self.variant(B2B7_A::B2B7_1)
    }
}
#[doc = "Field `IE7` reader - Segment 7 done interrupt selection"]
pub type IE7_R = crate::FieldReader<u8, IE7_A>;
#[doc = "Segment 7 done interrupt selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IE7_A {
    #[doc = "0: No interrupt when finished"]
    IE7_0 = 0,
    #[doc = "1: Generate interrupt on Done0 when segment 7 finish."]
    IE7_1 = 1,
    #[doc = "2: Generate interrupt on Done1 when segment 7 finish."]
    IE7_2 = 2,
    #[doc = "3: Generate interrupt on Done2 when segment 7 finish."]
    IE7_3 = 3,
}
impl From<IE7_A> for u8 {
    #[inline(always)]
    fn from(variant: IE7_A) -> Self {
        variant as _
    }
}
impl IE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE7_A {
        match self.bits {
            0 => IE7_A::IE7_0,
            1 => IE7_A::IE7_1,
            2 => IE7_A::IE7_2,
            3 => IE7_A::IE7_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IE7_0`"]
    #[inline(always)]
    pub fn is_ie7_0(&self) -> bool {
        *self == IE7_A::IE7_0
    }
    #[doc = "Checks if the value of the field is `IE7_1`"]
    #[inline(always)]
    pub fn is_ie7_1(&self) -> bool {
        *self == IE7_A::IE7_1
    }
    #[doc = "Checks if the value of the field is `IE7_2`"]
    #[inline(always)]
    pub fn is_ie7_2(&self) -> bool {
        *self == IE7_A::IE7_2
    }
    #[doc = "Checks if the value of the field is `IE7_3`"]
    #[inline(always)]
    pub fn is_ie7_3(&self) -> bool {
        *self == IE7_A::IE7_3
    }
}
#[doc = "Field `IE7` writer - Segment 7 done interrupt selection"]
pub type IE7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRIG5_CHAIN_7_6_SPEC, u8, IE7_A, 2, O>;
impl<'a, const O: u8> IE7_W<'a, O> {
    #[doc = "No interrupt when finished"]
    #[inline(always)]
    pub fn ie7_0(self) -> &'a mut W {
        self.variant(IE7_A::IE7_0)
    }
    #[doc = "Generate interrupt on Done0 when segment 7 finish."]
    #[inline(always)]
    pub fn ie7_1(self) -> &'a mut W {
        self.variant(IE7_A::IE7_1)
    }
    #[doc = "Generate interrupt on Done1 when segment 7 finish."]
    #[inline(always)]
    pub fn ie7_2(self) -> &'a mut W {
        self.variant(IE7_A::IE7_2)
    }
    #[doc = "Generate interrupt on Done2 when segment 7 finish."]
    #[inline(always)]
    pub fn ie7_3(self) -> &'a mut W {
        self.variant(IE7_A::IE7_3)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    pub fn csel6(&self) -> CSEL6_R {
        CSEL6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts6(&self) -> HWTS6_R {
        HWTS6_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 12 - Segment 6 B2B"]
    #[inline(always)]
    pub fn b2b6(&self) -> B2B6_R {
        B2B6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Segment 6 done interrupt selection"]
    #[inline(always)]
    pub fn ie6(&self) -> IE6_R {
        IE6_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    pub fn csel7(&self) -> CSEL7_R {
        CSEL7_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub fn hwts7(&self) -> HWTS7_R {
        HWTS7_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Segment 7 B2B"]
    #[inline(always)]
    pub fn b2b7(&self) -> B2B7_R {
        B2B7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Segment 7 done interrupt selection"]
    #[inline(always)]
    pub fn ie7(&self) -> IE7_R {
        IE7_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel6(&mut self) -> CSEL6_W<0> {
        CSEL6_W::new(self)
    }
    #[doc = "Bits 4:11 - Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts6(&mut self) -> HWTS6_W<4> {
        HWTS6_W::new(self)
    }
    #[doc = "Bit 12 - Segment 6 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b6(&mut self) -> B2B6_W<12> {
        B2B6_W::new(self)
    }
    #[doc = "Bits 13:14 - Segment 6 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie6(&mut self) -> IE6_W<13> {
        IE6_W::new(self)
    }
    #[doc = "Bits 16:19 - ADC channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn csel7(&mut self) -> CSEL7_W<16> {
        CSEL7_W::new(self)
    }
    #[doc = "Bits 20:27 - Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn hwts7(&mut self) -> HWTS7_W<20> {
        HWTS7_W::new(self)
    }
    #[doc = "Bit 28 - Segment 7 B2B"]
    #[inline(always)]
    #[must_use]
    pub fn b2b7(&mut self) -> B2B7_W<28> {
        B2B7_W::new(self)
    }
    #[doc = "Bits 29:30 - Segment 7 done interrupt selection"]
    #[inline(always)]
    #[must_use]
    pub fn ie7(&mut self) -> IE7_W<29> {
        IE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig5_chain_7_6](index.html) module"]
pub struct TRIG5_CHAIN_7_6_SPEC;
impl crate::RegisterSpec for TRIG5_CHAIN_7_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig5_chain_7_6::R](R) reader structure"]
impl crate::Readable for TRIG5_CHAIN_7_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig5_chain_7_6::W](W) writer structure"]
impl crate::Writable for TRIG5_CHAIN_7_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG5_CHAIN_7_6 to value 0"]
impl crate::Resettable for TRIG5_CHAIN_7_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
