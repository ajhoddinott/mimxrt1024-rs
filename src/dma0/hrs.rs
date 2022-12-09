#[doc = "Register `HRS` reader"]
pub struct R(crate::R<HRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRS0` reader - Hardware Request Status Channel 0"]
pub type HRS0_R = crate::BitReader<HRS0_A>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for channel 0 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 0 is present"]
    HWRQST = 1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::NO_HWRQST,
            true => HRS0_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS0_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS0_A::HWRQST
    }
}
#[doc = "Field `HRS1` reader - Hardware Request Status Channel 1"]
pub type HRS1_R = crate::BitReader<HRS1_A>;
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for channel 1 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 1 is present"]
    HWRQST = 1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::NO_HWRQST,
            true => HRS1_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS1_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS1_A::HWRQST
    }
}
#[doc = "Field `HRS2` reader - Hardware Request Status Channel 2"]
pub type HRS2_R = crate::BitReader<HRS2_A>;
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for channel 2 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 2 is present"]
    HWRQST = 1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::NO_HWRQST,
            true => HRS2_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS2_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS2_A::HWRQST
    }
}
#[doc = "Field `HRS3` reader - Hardware Request Status Channel 3"]
pub type HRS3_R = crate::BitReader<HRS3_A>;
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for channel 3 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 3 is present"]
    HWRQST = 1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::NO_HWRQST,
            true => HRS3_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS3_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS3_A::HWRQST
    }
}
#[doc = "Field `HRS4` reader - Hardware Request Status Channel 4"]
pub type HRS4_R = crate::BitReader<HRS4_A>;
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for channel 4 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 4 is present"]
    HWRQST = 1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::NO_HWRQST,
            true => HRS4_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS4_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS4_A::HWRQST
    }
}
#[doc = "Field `HRS5` reader - Hardware Request Status Channel 5"]
pub type HRS5_R = crate::BitReader<HRS5_A>;
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for channel 5 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 5 is present"]
    HWRQST = 1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::NO_HWRQST,
            true => HRS5_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS5_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS5_A::HWRQST
    }
}
#[doc = "Field `HRS6` reader - Hardware Request Status Channel 6"]
pub type HRS6_R = crate::BitReader<HRS6_A>;
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for channel 6 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 6 is present"]
    HWRQST = 1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::NO_HWRQST,
            true => HRS6_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS6_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS6_A::HWRQST
    }
}
#[doc = "Field `HRS7` reader - Hardware Request Status Channel 7"]
pub type HRS7_R = crate::BitReader<HRS7_A>;
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for channel 7 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 7 is present"]
    HWRQST = 1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::NO_HWRQST,
            true => HRS7_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS7_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS7_A::HWRQST
    }
}
#[doc = "Field `HRS8` reader - Hardware Request Status Channel 8"]
pub type HRS8_R = crate::BitReader<HRS8_A>;
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for channel 8 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 8 is present"]
    HWRQST = 1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::NO_HWRQST,
            true => HRS8_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS8_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS8_A::HWRQST
    }
}
#[doc = "Field `HRS9` reader - Hardware Request Status Channel 9"]
pub type HRS9_R = crate::BitReader<HRS9_A>;
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for channel 9 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 9 is present"]
    HWRQST = 1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::NO_HWRQST,
            true => HRS9_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS9_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS9_A::HWRQST
    }
}
#[doc = "Field `HRS10` reader - Hardware Request Status Channel 10"]
pub type HRS10_R = crate::BitReader<HRS10_A>;
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for channel 10 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 10 is present"]
    HWRQST = 1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::NO_HWRQST,
            true => HRS10_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS10_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS10_A::HWRQST
    }
}
#[doc = "Field `HRS11` reader - Hardware Request Status Channel 11"]
pub type HRS11_R = crate::BitReader<HRS11_A>;
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for channel 11 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 11 is present"]
    HWRQST = 1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::NO_HWRQST,
            true => HRS11_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS11_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS11_A::HWRQST
    }
}
#[doc = "Field `HRS12` reader - Hardware Request Status Channel 12"]
pub type HRS12_R = crate::BitReader<HRS12_A>;
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for channel 12 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 12 is present"]
    HWRQST = 1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::NO_HWRQST,
            true => HRS12_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS12_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS12_A::HWRQST
    }
}
#[doc = "Field `HRS13` reader - Hardware Request Status Channel 13"]
pub type HRS13_R = crate::BitReader<HRS13_A>;
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for channel 13 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 13 is present"]
    HWRQST = 1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::NO_HWRQST,
            true => HRS13_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS13_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS13_A::HWRQST
    }
}
#[doc = "Field `HRS14` reader - Hardware Request Status Channel 14"]
pub type HRS14_R = crate::BitReader<HRS14_A>;
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for channel 14 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 14 is present"]
    HWRQST = 1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::NO_HWRQST,
            true => HRS14_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS14_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS14_A::HWRQST
    }
}
#[doc = "Field `HRS15` reader - Hardware Request Status Channel 15"]
pub type HRS15_R = crate::BitReader<HRS15_A>;
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for channel 15 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 15 is present"]
    HWRQST = 1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::NO_HWRQST,
            true => HRS15_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS15_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS15_A::HWRQST
    }
}
#[doc = "Field `HRS16` reader - Hardware Request Status Channel 16"]
pub type HRS16_R = crate::BitReader<HRS16_A>;
#[doc = "Hardware Request Status Channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS16_A {
    #[doc = "0: A hardware service request for channel 16 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 16 is present"]
    HWRQST = 1,
}
impl From<HRS16_A> for bool {
    #[inline(always)]
    fn from(variant: HRS16_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS16_A {
        match self.bits {
            false => HRS16_A::NO_HWRQST,
            true => HRS16_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS16_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS16_A::HWRQST
    }
}
#[doc = "Field `HRS17` reader - Hardware Request Status Channel 17"]
pub type HRS17_R = crate::BitReader<HRS17_A>;
#[doc = "Hardware Request Status Channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS17_A {
    #[doc = "0: A hardware service request for channel 17 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 17 is present"]
    HWRQST = 1,
}
impl From<HRS17_A> for bool {
    #[inline(always)]
    fn from(variant: HRS17_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS17_A {
        match self.bits {
            false => HRS17_A::NO_HWRQST,
            true => HRS17_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS17_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS17_A::HWRQST
    }
}
#[doc = "Field `HRS18` reader - Hardware Request Status Channel 18"]
pub type HRS18_R = crate::BitReader<HRS18_A>;
#[doc = "Hardware Request Status Channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS18_A {
    #[doc = "0: A hardware service request for channel 18 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 18 is present"]
    HWRQST = 1,
}
impl From<HRS18_A> for bool {
    #[inline(always)]
    fn from(variant: HRS18_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS18_A {
        match self.bits {
            false => HRS18_A::NO_HWRQST,
            true => HRS18_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS18_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS18_A::HWRQST
    }
}
#[doc = "Field `HRS19` reader - Hardware Request Status Channel 19"]
pub type HRS19_R = crate::BitReader<HRS19_A>;
#[doc = "Hardware Request Status Channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS19_A {
    #[doc = "0: A hardware service request for channel 19 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 19 is present"]
    HWRQST = 1,
}
impl From<HRS19_A> for bool {
    #[inline(always)]
    fn from(variant: HRS19_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS19_A {
        match self.bits {
            false => HRS19_A::NO_HWRQST,
            true => HRS19_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS19_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS19_A::HWRQST
    }
}
#[doc = "Field `HRS20` reader - Hardware Request Status Channel 20"]
pub type HRS20_R = crate::BitReader<HRS20_A>;
#[doc = "Hardware Request Status Channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS20_A {
    #[doc = "0: A hardware service request for channel 20 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 20 is present"]
    HWRQST = 1,
}
impl From<HRS20_A> for bool {
    #[inline(always)]
    fn from(variant: HRS20_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS20_A {
        match self.bits {
            false => HRS20_A::NO_HWRQST,
            true => HRS20_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS20_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS20_A::HWRQST
    }
}
#[doc = "Field `HRS21` reader - Hardware Request Status Channel 21"]
pub type HRS21_R = crate::BitReader<HRS21_A>;
#[doc = "Hardware Request Status Channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS21_A {
    #[doc = "0: A hardware service request for channel 21 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 21 is present"]
    HWRQST = 1,
}
impl From<HRS21_A> for bool {
    #[inline(always)]
    fn from(variant: HRS21_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS21_A {
        match self.bits {
            false => HRS21_A::NO_HWRQST,
            true => HRS21_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS21_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS21_A::HWRQST
    }
}
#[doc = "Field `HRS22` reader - Hardware Request Status Channel 22"]
pub type HRS22_R = crate::BitReader<HRS22_A>;
#[doc = "Hardware Request Status Channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS22_A {
    #[doc = "0: A hardware service request for channel 22 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 22 is present"]
    HWRQST = 1,
}
impl From<HRS22_A> for bool {
    #[inline(always)]
    fn from(variant: HRS22_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS22_A {
        match self.bits {
            false => HRS22_A::NO_HWRQST,
            true => HRS22_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS22_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS22_A::HWRQST
    }
}
#[doc = "Field `HRS23` reader - Hardware Request Status Channel 23"]
pub type HRS23_R = crate::BitReader<HRS23_A>;
#[doc = "Hardware Request Status Channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS23_A {
    #[doc = "0: A hardware service request for channel 23 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 23 is present"]
    HWRQST = 1,
}
impl From<HRS23_A> for bool {
    #[inline(always)]
    fn from(variant: HRS23_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS23_A {
        match self.bits {
            false => HRS23_A::NO_HWRQST,
            true => HRS23_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS23_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS23_A::HWRQST
    }
}
#[doc = "Field `HRS24` reader - Hardware Request Status Channel 24"]
pub type HRS24_R = crate::BitReader<HRS24_A>;
#[doc = "Hardware Request Status Channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS24_A {
    #[doc = "0: A hardware service request for channel 24 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 24 is present"]
    HWRQST = 1,
}
impl From<HRS24_A> for bool {
    #[inline(always)]
    fn from(variant: HRS24_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS24_A {
        match self.bits {
            false => HRS24_A::NO_HWRQST,
            true => HRS24_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS24_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS24_A::HWRQST
    }
}
#[doc = "Field `HRS25` reader - Hardware Request Status Channel 25"]
pub type HRS25_R = crate::BitReader<HRS25_A>;
#[doc = "Hardware Request Status Channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS25_A {
    #[doc = "0: A hardware service request for channel 25 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 25 is present"]
    HWRQST = 1,
}
impl From<HRS25_A> for bool {
    #[inline(always)]
    fn from(variant: HRS25_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS25_A {
        match self.bits {
            false => HRS25_A::NO_HWRQST,
            true => HRS25_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS25_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS25_A::HWRQST
    }
}
#[doc = "Field `HRS26` reader - Hardware Request Status Channel 26"]
pub type HRS26_R = crate::BitReader<HRS26_A>;
#[doc = "Hardware Request Status Channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS26_A {
    #[doc = "0: A hardware service request for channel 26 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 26 is present"]
    HWRQST = 1,
}
impl From<HRS26_A> for bool {
    #[inline(always)]
    fn from(variant: HRS26_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS26_A {
        match self.bits {
            false => HRS26_A::NO_HWRQST,
            true => HRS26_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS26_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS26_A::HWRQST
    }
}
#[doc = "Field `HRS27` reader - Hardware Request Status Channel 27"]
pub type HRS27_R = crate::BitReader<HRS27_A>;
#[doc = "Hardware Request Status Channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS27_A {
    #[doc = "0: A hardware service request for channel 27 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 27 is present"]
    HWRQST = 1,
}
impl From<HRS27_A> for bool {
    #[inline(always)]
    fn from(variant: HRS27_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS27_A {
        match self.bits {
            false => HRS27_A::NO_HWRQST,
            true => HRS27_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS27_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS27_A::HWRQST
    }
}
#[doc = "Field `HRS28` reader - Hardware Request Status Channel 28"]
pub type HRS28_R = crate::BitReader<HRS28_A>;
#[doc = "Hardware Request Status Channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS28_A {
    #[doc = "0: A hardware service request for channel 28 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 28 is present"]
    HWRQST = 1,
}
impl From<HRS28_A> for bool {
    #[inline(always)]
    fn from(variant: HRS28_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS28_A {
        match self.bits {
            false => HRS28_A::NO_HWRQST,
            true => HRS28_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS28_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS28_A::HWRQST
    }
}
#[doc = "Field `HRS29` reader - Hardware Request Status Channel 29"]
pub type HRS29_R = crate::BitReader<HRS29_A>;
#[doc = "Hardware Request Status Channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS29_A {
    #[doc = "0: A hardware service request for channel 29 is not preset"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 29 is present"]
    HWRQST = 1,
}
impl From<HRS29_A> for bool {
    #[inline(always)]
    fn from(variant: HRS29_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS29_A {
        match self.bits {
            false => HRS29_A::NO_HWRQST,
            true => HRS29_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS29_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS29_A::HWRQST
    }
}
#[doc = "Field `HRS30` reader - Hardware Request Status Channel 30"]
pub type HRS30_R = crate::BitReader<HRS30_A>;
#[doc = "Hardware Request Status Channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS30_A {
    #[doc = "0: A hardware service request for channel 30 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 30 is present"]
    HWRQST = 1,
}
impl From<HRS30_A> for bool {
    #[inline(always)]
    fn from(variant: HRS30_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS30_A {
        match self.bits {
            false => HRS30_A::NO_HWRQST,
            true => HRS30_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS30_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS30_A::HWRQST
    }
}
#[doc = "Field `HRS31` reader - Hardware Request Status Channel 31"]
pub type HRS31_R = crate::BitReader<HRS31_A>;
#[doc = "Hardware Request Status Channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS31_A {
    #[doc = "0: A hardware service request for channel 31 is not present"]
    NO_HWRQST = 0,
    #[doc = "1: A hardware service request for channel 31 is present"]
    HWRQST = 1,
}
impl From<HRS31_A> for bool {
    #[inline(always)]
    fn from(variant: HRS31_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS31_A {
        match self.bits {
            false => HRS31_A::NO_HWRQST,
            true => HRS31_A::HWRQST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HWRQST`"]
    #[inline(always)]
    pub fn is_no_hwrqst(&self) -> bool {
        *self == HRS31_A::NO_HWRQST
    }
    #[doc = "Checks if the value of the field is `HWRQST`"]
    #[inline(always)]
    pub fn is_hwrqst(&self) -> bool {
        *self == HRS31_A::HWRQST
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Hardware Request Status Channel 16"]
    #[inline(always)]
    pub fn hrs16(&self) -> HRS16_R {
        HRS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hardware Request Status Channel 17"]
    #[inline(always)]
    pub fn hrs17(&self) -> HRS17_R {
        HRS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hardware Request Status Channel 18"]
    #[inline(always)]
    pub fn hrs18(&self) -> HRS18_R {
        HRS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hardware Request Status Channel 19"]
    #[inline(always)]
    pub fn hrs19(&self) -> HRS19_R {
        HRS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Hardware Request Status Channel 20"]
    #[inline(always)]
    pub fn hrs20(&self) -> HRS20_R {
        HRS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Hardware Request Status Channel 21"]
    #[inline(always)]
    pub fn hrs21(&self) -> HRS21_R {
        HRS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Hardware Request Status Channel 22"]
    #[inline(always)]
    pub fn hrs22(&self) -> HRS22_R {
        HRS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hardware Request Status Channel 23"]
    #[inline(always)]
    pub fn hrs23(&self) -> HRS23_R {
        HRS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Hardware Request Status Channel 24"]
    #[inline(always)]
    pub fn hrs24(&self) -> HRS24_R {
        HRS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Hardware Request Status Channel 25"]
    #[inline(always)]
    pub fn hrs25(&self) -> HRS25_R {
        HRS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Hardware Request Status Channel 26"]
    #[inline(always)]
    pub fn hrs26(&self) -> HRS26_R {
        HRS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Hardware Request Status Channel 27"]
    #[inline(always)]
    pub fn hrs27(&self) -> HRS27_R {
        HRS27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Hardware Request Status Channel 28"]
    #[inline(always)]
    pub fn hrs28(&self) -> HRS28_R {
        HRS28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Hardware Request Status Channel 29"]
    #[inline(always)]
    pub fn hrs29(&self) -> HRS29_R {
        HRS29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Hardware Request Status Channel 30"]
    #[inline(always)]
    pub fn hrs30(&self) -> HRS30_R {
        HRS30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Hardware Request Status Channel 31"]
    #[inline(always)]
    pub fn hrs31(&self) -> HRS31_R {
        HRS31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Hardware Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](index.html) module"]
pub struct HRS_SPEC;
impl crate::RegisterSpec for HRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrs::R](R) reader structure"]
impl crate::Readable for HRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
