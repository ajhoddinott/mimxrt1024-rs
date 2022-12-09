#[doc = "Register `EARS` reader"]
pub struct R(crate::R<EARS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EARS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EARS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EARS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EARS` writer"]
pub struct W(crate::W<EARS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EARS_SPEC>;
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
impl From<crate::W<EARS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EARS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDREQ_0` reader - Enable asynchronous DMA request in stop mode for channel 0."]
pub type EDREQ_0_R = crate::BitReader<EDREQ_0_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_0_A {
    #[doc = "0: Disable asynchronous DMA request for channel 0"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 0"]
    ENABLE = 1,
}
impl From<EDREQ_0_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_0_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_0_A {
        match self.bits {
            false => EDREQ_0_A::DISABLE,
            true => EDREQ_0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_0_A::ENABLE
    }
}
#[doc = "Field `EDREQ_0` writer - Enable asynchronous DMA request in stop mode for channel 0."]
pub type EDREQ_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_0_A, O>;
impl<'a, const O: u8> EDREQ_0_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_0_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_0_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_1` reader - Enable asynchronous DMA request in stop mode for channel 1."]
pub type EDREQ_1_R = crate::BitReader<EDREQ_1_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_1_A {
    #[doc = "0: Disable asynchronous DMA request for channel 1"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 1"]
    ENABLE = 1,
}
impl From<EDREQ_1_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_1_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_1_A {
        match self.bits {
            false => EDREQ_1_A::DISABLE,
            true => EDREQ_1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_1_A::ENABLE
    }
}
#[doc = "Field `EDREQ_1` writer - Enable asynchronous DMA request in stop mode for channel 1."]
pub type EDREQ_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_1_A, O>;
impl<'a, const O: u8> EDREQ_1_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_1_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_1_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_2` reader - Enable asynchronous DMA request in stop mode for channel 2."]
pub type EDREQ_2_R = crate::BitReader<EDREQ_2_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_2_A {
    #[doc = "0: Disable asynchronous DMA request for channel 2"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 2"]
    ENABLE = 1,
}
impl From<EDREQ_2_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_2_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_2_A {
        match self.bits {
            false => EDREQ_2_A::DISABLE,
            true => EDREQ_2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_2_A::ENABLE
    }
}
#[doc = "Field `EDREQ_2` writer - Enable asynchronous DMA request in stop mode for channel 2."]
pub type EDREQ_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_2_A, O>;
impl<'a, const O: u8> EDREQ_2_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_2_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 2"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_2_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_3` reader - Enable asynchronous DMA request in stop mode for channel 3."]
pub type EDREQ_3_R = crate::BitReader<EDREQ_3_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_3_A {
    #[doc = "0: Disable asynchronous DMA request for channel 3"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 3"]
    ENABLE = 1,
}
impl From<EDREQ_3_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_3_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_3_A {
        match self.bits {
            false => EDREQ_3_A::DISABLE,
            true => EDREQ_3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_3_A::ENABLE
    }
}
#[doc = "Field `EDREQ_3` writer - Enable asynchronous DMA request in stop mode for channel 3."]
pub type EDREQ_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_3_A, O>;
impl<'a, const O: u8> EDREQ_3_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 3"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_3_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 3"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_3_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_4` reader - Enable asynchronous DMA request in stop mode for channel 4."]
pub type EDREQ_4_R = crate::BitReader<EDREQ_4_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_4_A {
    #[doc = "0: Disable asynchronous DMA request for channel 4"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 4"]
    ENABLE = 1,
}
impl From<EDREQ_4_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_4_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_4_A {
        match self.bits {
            false => EDREQ_4_A::DISABLE,
            true => EDREQ_4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_4_A::ENABLE
    }
}
#[doc = "Field `EDREQ_4` writer - Enable asynchronous DMA request in stop mode for channel 4."]
pub type EDREQ_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_4_A, O>;
impl<'a, const O: u8> EDREQ_4_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_4_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 4"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_4_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_5` reader - Enable asynchronous DMA request in stop mode for channel 5."]
pub type EDREQ_5_R = crate::BitReader<EDREQ_5_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_5_A {
    #[doc = "0: Disable asynchronous DMA request for channel 5"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 5"]
    ENABLE = 1,
}
impl From<EDREQ_5_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_5_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_5_A {
        match self.bits {
            false => EDREQ_5_A::DISABLE,
            true => EDREQ_5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_5_A::ENABLE
    }
}
#[doc = "Field `EDREQ_5` writer - Enable asynchronous DMA request in stop mode for channel 5."]
pub type EDREQ_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_5_A, O>;
impl<'a, const O: u8> EDREQ_5_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 5"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_5_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 5"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_5_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_6` reader - Enable asynchronous DMA request in stop mode for channel 6."]
pub type EDREQ_6_R = crate::BitReader<EDREQ_6_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_6_A {
    #[doc = "0: Disable asynchronous DMA request for channel 6"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 6"]
    ENABLE = 1,
}
impl From<EDREQ_6_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_6_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_6_A {
        match self.bits {
            false => EDREQ_6_A::DISABLE,
            true => EDREQ_6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_6_A::ENABLE
    }
}
#[doc = "Field `EDREQ_6` writer - Enable asynchronous DMA request in stop mode for channel 6."]
pub type EDREQ_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_6_A, O>;
impl<'a, const O: u8> EDREQ_6_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 6"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_6_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 6"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_6_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_7` reader - Enable asynchronous DMA request in stop mode for channel 7."]
pub type EDREQ_7_R = crate::BitReader<EDREQ_7_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_7_A {
    #[doc = "0: Disable asynchronous DMA request for channel 7"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 7"]
    ENABLE = 1,
}
impl From<EDREQ_7_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_7_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_7_A {
        match self.bits {
            false => EDREQ_7_A::DISABLE,
            true => EDREQ_7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_7_A::ENABLE
    }
}
#[doc = "Field `EDREQ_7` writer - Enable asynchronous DMA request in stop mode for channel 7."]
pub type EDREQ_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_7_A, O>;
impl<'a, const O: u8> EDREQ_7_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 7"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_7_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 7"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_7_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_8` reader - Enable asynchronous DMA request in stop mode for channel 8."]
pub type EDREQ_8_R = crate::BitReader<EDREQ_8_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_8_A {
    #[doc = "0: Disable asynchronous DMA request for channel 8"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 8"]
    ENABLE = 1,
}
impl From<EDREQ_8_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_8_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_8_A {
        match self.bits {
            false => EDREQ_8_A::DISABLE,
            true => EDREQ_8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_8_A::ENABLE
    }
}
#[doc = "Field `EDREQ_8` writer - Enable asynchronous DMA request in stop mode for channel 8."]
pub type EDREQ_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_8_A, O>;
impl<'a, const O: u8> EDREQ_8_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_8_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_8_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_9` reader - Enable asynchronous DMA request in stop mode for channel 9."]
pub type EDREQ_9_R = crate::BitReader<EDREQ_9_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_9_A {
    #[doc = "0: Disable asynchronous DMA request for channel 9"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 9"]
    ENABLE = 1,
}
impl From<EDREQ_9_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_9_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_9_A {
        match self.bits {
            false => EDREQ_9_A::DISABLE,
            true => EDREQ_9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_9_A::ENABLE
    }
}
#[doc = "Field `EDREQ_9` writer - Enable asynchronous DMA request in stop mode for channel 9."]
pub type EDREQ_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_9_A, O>;
impl<'a, const O: u8> EDREQ_9_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 9"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_9_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 9"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_9_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_10` reader - Enable asynchronous DMA request in stop mode for channel 10."]
pub type EDREQ_10_R = crate::BitReader<EDREQ_10_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_10_A {
    #[doc = "0: Disable asynchronous DMA request for channel 10"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 10"]
    ENABLE = 1,
}
impl From<EDREQ_10_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_10_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_10_A {
        match self.bits {
            false => EDREQ_10_A::DISABLE,
            true => EDREQ_10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_10_A::ENABLE
    }
}
#[doc = "Field `EDREQ_10` writer - Enable asynchronous DMA request in stop mode for channel 10."]
pub type EDREQ_10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_10_A, O>;
impl<'a, const O: u8> EDREQ_10_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 10"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_10_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 10"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_10_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_11` reader - Enable asynchronous DMA request in stop mode for channel 11."]
pub type EDREQ_11_R = crate::BitReader<EDREQ_11_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_11_A {
    #[doc = "0: Disable asynchronous DMA request for channel 11"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 11"]
    ENABLE = 1,
}
impl From<EDREQ_11_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_11_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_11_A {
        match self.bits {
            false => EDREQ_11_A::DISABLE,
            true => EDREQ_11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_11_A::ENABLE
    }
}
#[doc = "Field `EDREQ_11` writer - Enable asynchronous DMA request in stop mode for channel 11."]
pub type EDREQ_11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_11_A, O>;
impl<'a, const O: u8> EDREQ_11_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 11"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_11_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 11"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_11_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_12` reader - Enable asynchronous DMA request in stop mode for channel 12."]
pub type EDREQ_12_R = crate::BitReader<EDREQ_12_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_12_A {
    #[doc = "0: Disable asynchronous DMA request for channel 12"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 12"]
    ENABLE = 1,
}
impl From<EDREQ_12_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_12_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_12_A {
        match self.bits {
            false => EDREQ_12_A::DISABLE,
            true => EDREQ_12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_12_A::ENABLE
    }
}
#[doc = "Field `EDREQ_12` writer - Enable asynchronous DMA request in stop mode for channel 12."]
pub type EDREQ_12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_12_A, O>;
impl<'a, const O: u8> EDREQ_12_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 12"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_12_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 12"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_12_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_13` reader - Enable asynchronous DMA request in stop mode for channel 13."]
pub type EDREQ_13_R = crate::BitReader<EDREQ_13_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_13_A {
    #[doc = "0: Disable asynchronous DMA request for channel 13"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 13"]
    ENABLE = 1,
}
impl From<EDREQ_13_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_13_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_13_A {
        match self.bits {
            false => EDREQ_13_A::DISABLE,
            true => EDREQ_13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_13_A::ENABLE
    }
}
#[doc = "Field `EDREQ_13` writer - Enable asynchronous DMA request in stop mode for channel 13."]
pub type EDREQ_13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_13_A, O>;
impl<'a, const O: u8> EDREQ_13_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 13"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_13_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 13"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_13_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_14` reader - Enable asynchronous DMA request in stop mode for channel 14."]
pub type EDREQ_14_R = crate::BitReader<EDREQ_14_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_14_A {
    #[doc = "0: Disable asynchronous DMA request for channel 14"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 14"]
    ENABLE = 1,
}
impl From<EDREQ_14_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_14_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_14_A {
        match self.bits {
            false => EDREQ_14_A::DISABLE,
            true => EDREQ_14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_14_A::ENABLE
    }
}
#[doc = "Field `EDREQ_14` writer - Enable asynchronous DMA request in stop mode for channel 14."]
pub type EDREQ_14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_14_A, O>;
impl<'a, const O: u8> EDREQ_14_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 14"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_14_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 14"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_14_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_15` reader - Enable asynchronous DMA request in stop mode for channel 15."]
pub type EDREQ_15_R = crate::BitReader<EDREQ_15_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_15_A {
    #[doc = "0: Disable asynchronous DMA request for channel 15"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 15"]
    ENABLE = 1,
}
impl From<EDREQ_15_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_15_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_15_A {
        match self.bits {
            false => EDREQ_15_A::DISABLE,
            true => EDREQ_15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_15_A::ENABLE
    }
}
#[doc = "Field `EDREQ_15` writer - Enable asynchronous DMA request in stop mode for channel 15."]
pub type EDREQ_15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_15_A, O>;
impl<'a, const O: u8> EDREQ_15_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 15"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_15_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 15"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_15_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_16` reader - Enable asynchronous DMA request in stop mode for channel 16."]
pub type EDREQ_16_R = crate::BitReader<EDREQ_16_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_16_A {
    #[doc = "0: Disable asynchronous DMA request for channel 16"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 16"]
    ENABLE = 1,
}
impl From<EDREQ_16_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_16_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_16_A {
        match self.bits {
            false => EDREQ_16_A::DISABLE,
            true => EDREQ_16_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_16_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_16_A::ENABLE
    }
}
#[doc = "Field `EDREQ_16` writer - Enable asynchronous DMA request in stop mode for channel 16."]
pub type EDREQ_16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_16_A, O>;
impl<'a, const O: u8> EDREQ_16_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 16"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_16_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 16"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_16_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_17` reader - Enable asynchronous DMA request in stop mode for channel 17."]
pub type EDREQ_17_R = crate::BitReader<EDREQ_17_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_17_A {
    #[doc = "0: Disable asynchronous DMA request for channel 17"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 17"]
    ENABLE = 1,
}
impl From<EDREQ_17_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_17_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_17_A {
        match self.bits {
            false => EDREQ_17_A::DISABLE,
            true => EDREQ_17_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_17_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_17_A::ENABLE
    }
}
#[doc = "Field `EDREQ_17` writer - Enable asynchronous DMA request in stop mode for channel 17."]
pub type EDREQ_17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_17_A, O>;
impl<'a, const O: u8> EDREQ_17_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 17"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_17_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 17"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_17_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_18` reader - Enable asynchronous DMA request in stop mode for channel 18."]
pub type EDREQ_18_R = crate::BitReader<EDREQ_18_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_18_A {
    #[doc = "0: Disable asynchronous DMA request for channel 18"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 18"]
    ENABLE = 1,
}
impl From<EDREQ_18_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_18_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_18_A {
        match self.bits {
            false => EDREQ_18_A::DISABLE,
            true => EDREQ_18_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_18_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_18_A::ENABLE
    }
}
#[doc = "Field `EDREQ_18` writer - Enable asynchronous DMA request in stop mode for channel 18."]
pub type EDREQ_18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_18_A, O>;
impl<'a, const O: u8> EDREQ_18_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 18"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_18_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 18"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_18_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_19` reader - Enable asynchronous DMA request in stop mode for channel 19."]
pub type EDREQ_19_R = crate::BitReader<EDREQ_19_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_19_A {
    #[doc = "0: Disable asynchronous DMA request for channel 19"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 19"]
    ENABLE = 1,
}
impl From<EDREQ_19_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_19_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_19_A {
        match self.bits {
            false => EDREQ_19_A::DISABLE,
            true => EDREQ_19_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_19_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_19_A::ENABLE
    }
}
#[doc = "Field `EDREQ_19` writer - Enable asynchronous DMA request in stop mode for channel 19."]
pub type EDREQ_19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_19_A, O>;
impl<'a, const O: u8> EDREQ_19_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 19"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_19_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 19"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_19_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_20` reader - Enable asynchronous DMA request in stop mode for channel 20."]
pub type EDREQ_20_R = crate::BitReader<EDREQ_20_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_20_A {
    #[doc = "0: Disable asynchronous DMA request for channel 20"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 20"]
    ENABLE = 1,
}
impl From<EDREQ_20_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_20_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_20_A {
        match self.bits {
            false => EDREQ_20_A::DISABLE,
            true => EDREQ_20_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_20_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_20_A::ENABLE
    }
}
#[doc = "Field `EDREQ_20` writer - Enable asynchronous DMA request in stop mode for channel 20."]
pub type EDREQ_20_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_20_A, O>;
impl<'a, const O: u8> EDREQ_20_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 20"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_20_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 20"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_20_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_21` reader - Enable asynchronous DMA request in stop mode for channel 21."]
pub type EDREQ_21_R = crate::BitReader<EDREQ_21_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_21_A {
    #[doc = "0: Disable asynchronous DMA request for channel 21"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 21"]
    ENABLE = 1,
}
impl From<EDREQ_21_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_21_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_21_A {
        match self.bits {
            false => EDREQ_21_A::DISABLE,
            true => EDREQ_21_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_21_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_21_A::ENABLE
    }
}
#[doc = "Field `EDREQ_21` writer - Enable asynchronous DMA request in stop mode for channel 21."]
pub type EDREQ_21_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_21_A, O>;
impl<'a, const O: u8> EDREQ_21_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 21"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_21_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 21"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_21_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_22` reader - Enable asynchronous DMA request in stop mode for channel 22."]
pub type EDREQ_22_R = crate::BitReader<EDREQ_22_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_22_A {
    #[doc = "0: Disable asynchronous DMA request for channel 22"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 22"]
    ENABLE = 1,
}
impl From<EDREQ_22_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_22_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_22_A {
        match self.bits {
            false => EDREQ_22_A::DISABLE,
            true => EDREQ_22_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_22_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_22_A::ENABLE
    }
}
#[doc = "Field `EDREQ_22` writer - Enable asynchronous DMA request in stop mode for channel 22."]
pub type EDREQ_22_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_22_A, O>;
impl<'a, const O: u8> EDREQ_22_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 22"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_22_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 22"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_22_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_23` reader - Enable asynchronous DMA request in stop mode for channel 23."]
pub type EDREQ_23_R = crate::BitReader<EDREQ_23_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_23_A {
    #[doc = "0: Disable asynchronous DMA request for channel 23"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 23"]
    ENABLE = 1,
}
impl From<EDREQ_23_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_23_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_23_A {
        match self.bits {
            false => EDREQ_23_A::DISABLE,
            true => EDREQ_23_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_23_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_23_A::ENABLE
    }
}
#[doc = "Field `EDREQ_23` writer - Enable asynchronous DMA request in stop mode for channel 23."]
pub type EDREQ_23_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_23_A, O>;
impl<'a, const O: u8> EDREQ_23_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 23"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_23_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 23"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_23_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_24` reader - Enable asynchronous DMA request in stop mode for channel 24."]
pub type EDREQ_24_R = crate::BitReader<EDREQ_24_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_24_A {
    #[doc = "0: Disable asynchronous DMA request for channel 24"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 24"]
    ENABLE = 1,
}
impl From<EDREQ_24_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_24_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_24_A {
        match self.bits {
            false => EDREQ_24_A::DISABLE,
            true => EDREQ_24_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_24_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_24_A::ENABLE
    }
}
#[doc = "Field `EDREQ_24` writer - Enable asynchronous DMA request in stop mode for channel 24."]
pub type EDREQ_24_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_24_A, O>;
impl<'a, const O: u8> EDREQ_24_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 24"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_24_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 24"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_24_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_25` reader - Enable asynchronous DMA request in stop mode for channel 25."]
pub type EDREQ_25_R = crate::BitReader<EDREQ_25_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_25_A {
    #[doc = "0: Disable asynchronous DMA request for channel 25"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 25"]
    ENABLE = 1,
}
impl From<EDREQ_25_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_25_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_25_A {
        match self.bits {
            false => EDREQ_25_A::DISABLE,
            true => EDREQ_25_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_25_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_25_A::ENABLE
    }
}
#[doc = "Field `EDREQ_25` writer - Enable asynchronous DMA request in stop mode for channel 25."]
pub type EDREQ_25_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_25_A, O>;
impl<'a, const O: u8> EDREQ_25_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 25"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_25_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 25"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_25_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_26` reader - Enable asynchronous DMA request in stop mode for channel 26."]
pub type EDREQ_26_R = crate::BitReader<EDREQ_26_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_26_A {
    #[doc = "0: Disable asynchronous DMA request for channel 26"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 26"]
    ENABLE = 1,
}
impl From<EDREQ_26_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_26_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_26_A {
        match self.bits {
            false => EDREQ_26_A::DISABLE,
            true => EDREQ_26_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_26_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_26_A::ENABLE
    }
}
#[doc = "Field `EDREQ_26` writer - Enable asynchronous DMA request in stop mode for channel 26."]
pub type EDREQ_26_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_26_A, O>;
impl<'a, const O: u8> EDREQ_26_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 26"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_26_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 26"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_26_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_27` reader - Enable asynchronous DMA request in stop mode for channel 27."]
pub type EDREQ_27_R = crate::BitReader<EDREQ_27_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_27_A {
    #[doc = "0: Disable asynchronous DMA request for channel 27"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 27"]
    ENABLE = 1,
}
impl From<EDREQ_27_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_27_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_27_A {
        match self.bits {
            false => EDREQ_27_A::DISABLE,
            true => EDREQ_27_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_27_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_27_A::ENABLE
    }
}
#[doc = "Field `EDREQ_27` writer - Enable asynchronous DMA request in stop mode for channel 27."]
pub type EDREQ_27_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_27_A, O>;
impl<'a, const O: u8> EDREQ_27_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 27"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_27_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 27"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_27_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_28` reader - Enable asynchronous DMA request in stop mode for channel 28."]
pub type EDREQ_28_R = crate::BitReader<EDREQ_28_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_28_A {
    #[doc = "0: Disable asynchronous DMA request for channel 28"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 28"]
    ENABLE = 1,
}
impl From<EDREQ_28_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_28_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_28_A {
        match self.bits {
            false => EDREQ_28_A::DISABLE,
            true => EDREQ_28_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_28_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_28_A::ENABLE
    }
}
#[doc = "Field `EDREQ_28` writer - Enable asynchronous DMA request in stop mode for channel 28."]
pub type EDREQ_28_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_28_A, O>;
impl<'a, const O: u8> EDREQ_28_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 28"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_28_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 28"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_28_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_29` reader - Enable asynchronous DMA request in stop mode for channel 29."]
pub type EDREQ_29_R = crate::BitReader<EDREQ_29_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_29_A {
    #[doc = "0: Disable asynchronous DMA request for channel 29"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 29"]
    ENABLE = 1,
}
impl From<EDREQ_29_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_29_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_29_A {
        match self.bits {
            false => EDREQ_29_A::DISABLE,
            true => EDREQ_29_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_29_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_29_A::ENABLE
    }
}
#[doc = "Field `EDREQ_29` writer - Enable asynchronous DMA request in stop mode for channel 29."]
pub type EDREQ_29_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_29_A, O>;
impl<'a, const O: u8> EDREQ_29_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 29"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_29_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 29"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_29_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_30` reader - Enable asynchronous DMA request in stop mode for channel 30."]
pub type EDREQ_30_R = crate::BitReader<EDREQ_30_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_30_A {
    #[doc = "0: Disable asynchronous DMA request for channel 30"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 30"]
    ENABLE = 1,
}
impl From<EDREQ_30_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_30_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_30_A {
        match self.bits {
            false => EDREQ_30_A::DISABLE,
            true => EDREQ_30_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_30_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_30_A::ENABLE
    }
}
#[doc = "Field `EDREQ_30` writer - Enable asynchronous DMA request in stop mode for channel 30."]
pub type EDREQ_30_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_30_A, O>;
impl<'a, const O: u8> EDREQ_30_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 30"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_30_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 30"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_30_A::ENABLE)
    }
}
#[doc = "Field `EDREQ_31` reader - Enable asynchronous DMA request in stop mode for channel 31."]
pub type EDREQ_31_R = crate::BitReader<EDREQ_31_A>;
#[doc = "Enable asynchronous DMA request in stop mode for channel 31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_31_A {
    #[doc = "0: Disable asynchronous DMA request for channel 31"]
    DISABLE = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 31"]
    ENABLE = 1,
}
impl From<EDREQ_31_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_31_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_31_A {
        match self.bits {
            false => EDREQ_31_A::DISABLE,
            true => EDREQ_31_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDREQ_31_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EDREQ_31_A::ENABLE
    }
}
#[doc = "Field `EDREQ_31` writer - Enable asynchronous DMA request in stop mode for channel 31."]
pub type EDREQ_31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_31_A, O>;
impl<'a, const O: u8> EDREQ_31_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 31"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDREQ_31_A::DISABLE)
    }
    #[doc = "Enable asynchronous DMA request for channel 31"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDREQ_31_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&self) -> EDREQ_0_R {
        EDREQ_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&self) -> EDREQ_1_R {
        EDREQ_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&self) -> EDREQ_2_R {
        EDREQ_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&self) -> EDREQ_3_R {
        EDREQ_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4."]
    #[inline(always)]
    pub fn edreq_4(&self) -> EDREQ_4_R {
        EDREQ_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5."]
    #[inline(always)]
    pub fn edreq_5(&self) -> EDREQ_5_R {
        EDREQ_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6."]
    #[inline(always)]
    pub fn edreq_6(&self) -> EDREQ_6_R {
        EDREQ_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7."]
    #[inline(always)]
    pub fn edreq_7(&self) -> EDREQ_7_R {
        EDREQ_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8."]
    #[inline(always)]
    pub fn edreq_8(&self) -> EDREQ_8_R {
        EDREQ_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9."]
    #[inline(always)]
    pub fn edreq_9(&self) -> EDREQ_9_R {
        EDREQ_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10."]
    #[inline(always)]
    pub fn edreq_10(&self) -> EDREQ_10_R {
        EDREQ_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11."]
    #[inline(always)]
    pub fn edreq_11(&self) -> EDREQ_11_R {
        EDREQ_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12."]
    #[inline(always)]
    pub fn edreq_12(&self) -> EDREQ_12_R {
        EDREQ_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13."]
    #[inline(always)]
    pub fn edreq_13(&self) -> EDREQ_13_R {
        EDREQ_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14."]
    #[inline(always)]
    pub fn edreq_14(&self) -> EDREQ_14_R {
        EDREQ_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15."]
    #[inline(always)]
    pub fn edreq_15(&self) -> EDREQ_15_R {
        EDREQ_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16."]
    #[inline(always)]
    pub fn edreq_16(&self) -> EDREQ_16_R {
        EDREQ_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17."]
    #[inline(always)]
    pub fn edreq_17(&self) -> EDREQ_17_R {
        EDREQ_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18."]
    #[inline(always)]
    pub fn edreq_18(&self) -> EDREQ_18_R {
        EDREQ_18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19."]
    #[inline(always)]
    pub fn edreq_19(&self) -> EDREQ_19_R {
        EDREQ_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20."]
    #[inline(always)]
    pub fn edreq_20(&self) -> EDREQ_20_R {
        EDREQ_20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21."]
    #[inline(always)]
    pub fn edreq_21(&self) -> EDREQ_21_R {
        EDREQ_21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22."]
    #[inline(always)]
    pub fn edreq_22(&self) -> EDREQ_22_R {
        EDREQ_22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23."]
    #[inline(always)]
    pub fn edreq_23(&self) -> EDREQ_23_R {
        EDREQ_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24."]
    #[inline(always)]
    pub fn edreq_24(&self) -> EDREQ_24_R {
        EDREQ_24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25."]
    #[inline(always)]
    pub fn edreq_25(&self) -> EDREQ_25_R {
        EDREQ_25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26."]
    #[inline(always)]
    pub fn edreq_26(&self) -> EDREQ_26_R {
        EDREQ_26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27."]
    #[inline(always)]
    pub fn edreq_27(&self) -> EDREQ_27_R {
        EDREQ_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28."]
    #[inline(always)]
    pub fn edreq_28(&self) -> EDREQ_28_R {
        EDREQ_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29."]
    #[inline(always)]
    pub fn edreq_29(&self) -> EDREQ_29_R {
        EDREQ_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30."]
    #[inline(always)]
    pub fn edreq_30(&self) -> EDREQ_30_R {
        EDREQ_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31."]
    #[inline(always)]
    pub fn edreq_31(&self) -> EDREQ_31_R {
        EDREQ_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_0(&mut self) -> EDREQ_0_W<0> {
        EDREQ_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_1(&mut self) -> EDREQ_1_W<1> {
        EDREQ_1_W::new(self)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_2(&mut self) -> EDREQ_2_W<2> {
        EDREQ_2_W::new(self)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_3(&mut self) -> EDREQ_3_W<3> {
        EDREQ_3_W::new(self)
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_4(&mut self) -> EDREQ_4_W<4> {
        EDREQ_4_W::new(self)
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_5(&mut self) -> EDREQ_5_W<5> {
        EDREQ_5_W::new(self)
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_6(&mut self) -> EDREQ_6_W<6> {
        EDREQ_6_W::new(self)
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_7(&mut self) -> EDREQ_7_W<7> {
        EDREQ_7_W::new(self)
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_8(&mut self) -> EDREQ_8_W<8> {
        EDREQ_8_W::new(self)
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_9(&mut self) -> EDREQ_9_W<9> {
        EDREQ_9_W::new(self)
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_10(&mut self) -> EDREQ_10_W<10> {
        EDREQ_10_W::new(self)
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_11(&mut self) -> EDREQ_11_W<11> {
        EDREQ_11_W::new(self)
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_12(&mut self) -> EDREQ_12_W<12> {
        EDREQ_12_W::new(self)
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_13(&mut self) -> EDREQ_13_W<13> {
        EDREQ_13_W::new(self)
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_14(&mut self) -> EDREQ_14_W<14> {
        EDREQ_14_W::new(self)
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_15(&mut self) -> EDREQ_15_W<15> {
        EDREQ_15_W::new(self)
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_16(&mut self) -> EDREQ_16_W<16> {
        EDREQ_16_W::new(self)
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_17(&mut self) -> EDREQ_17_W<17> {
        EDREQ_17_W::new(self)
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_18(&mut self) -> EDREQ_18_W<18> {
        EDREQ_18_W::new(self)
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_19(&mut self) -> EDREQ_19_W<19> {
        EDREQ_19_W::new(self)
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_20(&mut self) -> EDREQ_20_W<20> {
        EDREQ_20_W::new(self)
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_21(&mut self) -> EDREQ_21_W<21> {
        EDREQ_21_W::new(self)
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_22(&mut self) -> EDREQ_22_W<22> {
        EDREQ_22_W::new(self)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_23(&mut self) -> EDREQ_23_W<23> {
        EDREQ_23_W::new(self)
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_24(&mut self) -> EDREQ_24_W<24> {
        EDREQ_24_W::new(self)
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_25(&mut self) -> EDREQ_25_W<25> {
        EDREQ_25_W::new(self)
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_26(&mut self) -> EDREQ_26_W<26> {
        EDREQ_26_W::new(self)
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_27(&mut self) -> EDREQ_27_W<27> {
        EDREQ_27_W::new(self)
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_28(&mut self) -> EDREQ_28_W<28> {
        EDREQ_28_W::new(self)
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_29(&mut self) -> EDREQ_29_W<29> {
        EDREQ_29_W::new(self)
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_30(&mut self) -> EDREQ_30_W<30> {
        EDREQ_30_W::new(self)
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_31(&mut self) -> EDREQ_31_W<31> {
        EDREQ_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Asynchronous Request in Stop\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](index.html) module"]
pub struct EARS_SPEC;
impl crate::RegisterSpec for EARS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ears::R](R) reader structure"]
impl crate::Readable for EARS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ears::W](W) writer structure"]
impl crate::Writable for EARS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EARS to value 0"]
impl crate::Resettable for EARS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
