#[doc = "Register `DMA_CTRL` reader"]
pub struct R(crate::R<DMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CTRL` writer"]
pub struct W(crate::W<DMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CTRL_SPEC>;
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
impl From<crate::W<DMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG0_ENABLE` reader - Enable DMA request when TRIG0 done."]
pub type TRIG0_ENABLE_R = crate::BitReader<TRIG0_ENABLE_A>;
#[doc = "Enable DMA request when TRIG0 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_ENABLE_A {
    #[doc = "0: TRIG0 DMA request disabled."]
    TRIG0_ENABLE_0 = 0,
    #[doc = "1: TRIG0 DMA request enabled."]
    TRIG0_ENABLE_1 = 1,
}
impl From<TRIG0_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_ENABLE_A {
        match self.bits {
            false => TRIG0_ENABLE_A::TRIG0_ENABLE_0,
            true => TRIG0_ENABLE_A::TRIG0_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig0_enable_0(&self) -> bool {
        *self == TRIG0_ENABLE_A::TRIG0_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig0_enable_1(&self) -> bool {
        *self == TRIG0_ENABLE_A::TRIG0_ENABLE_1
    }
}
#[doc = "Field `TRIG0_ENABLE` writer - Enable DMA request when TRIG0 done."]
pub type TRIG0_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG0_ENABLE_A, O>;
impl<'a, const O: u8> TRIG0_ENABLE_W<'a, O> {
    #[doc = "TRIG0 DMA request disabled."]
    #[inline(always)]
    pub fn trig0_enable_0(self) -> &'a mut W {
        self.variant(TRIG0_ENABLE_A::TRIG0_ENABLE_0)
    }
    #[doc = "TRIG0 DMA request enabled."]
    #[inline(always)]
    pub fn trig0_enable_1(self) -> &'a mut W {
        self.variant(TRIG0_ENABLE_A::TRIG0_ENABLE_1)
    }
}
#[doc = "Field `TRIG1_ENABLE` reader - Enable DMA request when TRIG1 done."]
pub type TRIG1_ENABLE_R = crate::BitReader<TRIG1_ENABLE_A>;
#[doc = "Enable DMA request when TRIG1 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_ENABLE_A {
    #[doc = "0: TRIG1 DMA request disabled."]
    TRIG1_ENABLE_0 = 0,
    #[doc = "1: TRIG1 DMA request enabled."]
    TRIG1_ENABLE_1 = 1,
}
impl From<TRIG1_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_ENABLE_A {
        match self.bits {
            false => TRIG1_ENABLE_A::TRIG1_ENABLE_0,
            true => TRIG1_ENABLE_A::TRIG1_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig1_enable_0(&self) -> bool {
        *self == TRIG1_ENABLE_A::TRIG1_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig1_enable_1(&self) -> bool {
        *self == TRIG1_ENABLE_A::TRIG1_ENABLE_1
    }
}
#[doc = "Field `TRIG1_ENABLE` writer - Enable DMA request when TRIG1 done."]
pub type TRIG1_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG1_ENABLE_A, O>;
impl<'a, const O: u8> TRIG1_ENABLE_W<'a, O> {
    #[doc = "TRIG1 DMA request disabled."]
    #[inline(always)]
    pub fn trig1_enable_0(self) -> &'a mut W {
        self.variant(TRIG1_ENABLE_A::TRIG1_ENABLE_0)
    }
    #[doc = "TRIG1 DMA request enabled."]
    #[inline(always)]
    pub fn trig1_enable_1(self) -> &'a mut W {
        self.variant(TRIG1_ENABLE_A::TRIG1_ENABLE_1)
    }
}
#[doc = "Field `TRIG2_ENABLE` reader - Enable DMA request when TRIG2 done."]
pub type TRIG2_ENABLE_R = crate::BitReader<TRIG2_ENABLE_A>;
#[doc = "Enable DMA request when TRIG2 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_ENABLE_A {
    #[doc = "0: TRIG2 DMA request disabled."]
    TRIG2_ENABLE_0 = 0,
    #[doc = "1: TRIG2 DMA request enabled."]
    TRIG2_ENABLE_1 = 1,
}
impl From<TRIG2_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_ENABLE_A {
        match self.bits {
            false => TRIG2_ENABLE_A::TRIG2_ENABLE_0,
            true => TRIG2_ENABLE_A::TRIG2_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig2_enable_0(&self) -> bool {
        *self == TRIG2_ENABLE_A::TRIG2_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig2_enable_1(&self) -> bool {
        *self == TRIG2_ENABLE_A::TRIG2_ENABLE_1
    }
}
#[doc = "Field `TRIG2_ENABLE` writer - Enable DMA request when TRIG2 done."]
pub type TRIG2_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG2_ENABLE_A, O>;
impl<'a, const O: u8> TRIG2_ENABLE_W<'a, O> {
    #[doc = "TRIG2 DMA request disabled."]
    #[inline(always)]
    pub fn trig2_enable_0(self) -> &'a mut W {
        self.variant(TRIG2_ENABLE_A::TRIG2_ENABLE_0)
    }
    #[doc = "TRIG2 DMA request enabled."]
    #[inline(always)]
    pub fn trig2_enable_1(self) -> &'a mut W {
        self.variant(TRIG2_ENABLE_A::TRIG2_ENABLE_1)
    }
}
#[doc = "Field `TRIG3_ENABLE` reader - Enable DMA request when TRIG3 done."]
pub type TRIG3_ENABLE_R = crate::BitReader<TRIG3_ENABLE_A>;
#[doc = "Enable DMA request when TRIG3 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_ENABLE_A {
    #[doc = "0: TRIG3 DMA request disabled."]
    TRIG3_ENABLE_0 = 0,
    #[doc = "1: TRIG3 DMA request enabled."]
    TRIG3_ENABLE_1 = 1,
}
impl From<TRIG3_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_ENABLE_A {
        match self.bits {
            false => TRIG3_ENABLE_A::TRIG3_ENABLE_0,
            true => TRIG3_ENABLE_A::TRIG3_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig3_enable_0(&self) -> bool {
        *self == TRIG3_ENABLE_A::TRIG3_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig3_enable_1(&self) -> bool {
        *self == TRIG3_ENABLE_A::TRIG3_ENABLE_1
    }
}
#[doc = "Field `TRIG3_ENABLE` writer - Enable DMA request when TRIG3 done."]
pub type TRIG3_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG3_ENABLE_A, O>;
impl<'a, const O: u8> TRIG3_ENABLE_W<'a, O> {
    #[doc = "TRIG3 DMA request disabled."]
    #[inline(always)]
    pub fn trig3_enable_0(self) -> &'a mut W {
        self.variant(TRIG3_ENABLE_A::TRIG3_ENABLE_0)
    }
    #[doc = "TRIG3 DMA request enabled."]
    #[inline(always)]
    pub fn trig3_enable_1(self) -> &'a mut W {
        self.variant(TRIG3_ENABLE_A::TRIG3_ENABLE_1)
    }
}
#[doc = "Field `TRIG4_ENABLE` reader - Enable DMA request when TRIG4 done."]
pub type TRIG4_ENABLE_R = crate::BitReader<TRIG4_ENABLE_A>;
#[doc = "Enable DMA request when TRIG4 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_ENABLE_A {
    #[doc = "0: TRIG4 DMA request disabled."]
    TRIG4_ENABLE_0 = 0,
    #[doc = "1: TRIG4 DMA request enabled."]
    TRIG4_ENABLE_1 = 1,
}
impl From<TRIG4_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_ENABLE_A {
        match self.bits {
            false => TRIG4_ENABLE_A::TRIG4_ENABLE_0,
            true => TRIG4_ENABLE_A::TRIG4_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig4_enable_0(&self) -> bool {
        *self == TRIG4_ENABLE_A::TRIG4_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig4_enable_1(&self) -> bool {
        *self == TRIG4_ENABLE_A::TRIG4_ENABLE_1
    }
}
#[doc = "Field `TRIG4_ENABLE` writer - Enable DMA request when TRIG4 done."]
pub type TRIG4_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG4_ENABLE_A, O>;
impl<'a, const O: u8> TRIG4_ENABLE_W<'a, O> {
    #[doc = "TRIG4 DMA request disabled."]
    #[inline(always)]
    pub fn trig4_enable_0(self) -> &'a mut W {
        self.variant(TRIG4_ENABLE_A::TRIG4_ENABLE_0)
    }
    #[doc = "TRIG4 DMA request enabled."]
    #[inline(always)]
    pub fn trig4_enable_1(self) -> &'a mut W {
        self.variant(TRIG4_ENABLE_A::TRIG4_ENABLE_1)
    }
}
#[doc = "Field `TRIG5_ENABLE` reader - Enable DMA request when TRIG5 done."]
pub type TRIG5_ENABLE_R = crate::BitReader<TRIG5_ENABLE_A>;
#[doc = "Enable DMA request when TRIG5 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_ENABLE_A {
    #[doc = "0: TRIG5 DMA request disabled."]
    TRIG5_ENABLE_0 = 0,
    #[doc = "1: TRIG5 DMA request enabled."]
    TRIG5_ENABLE_1 = 1,
}
impl From<TRIG5_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_ENABLE_A {
        match self.bits {
            false => TRIG5_ENABLE_A::TRIG5_ENABLE_0,
            true => TRIG5_ENABLE_A::TRIG5_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig5_enable_0(&self) -> bool {
        *self == TRIG5_ENABLE_A::TRIG5_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig5_enable_1(&self) -> bool {
        *self == TRIG5_ENABLE_A::TRIG5_ENABLE_1
    }
}
#[doc = "Field `TRIG5_ENABLE` writer - Enable DMA request when TRIG5 done."]
pub type TRIG5_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG5_ENABLE_A, O>;
impl<'a, const O: u8> TRIG5_ENABLE_W<'a, O> {
    #[doc = "TRIG5 DMA request disabled."]
    #[inline(always)]
    pub fn trig5_enable_0(self) -> &'a mut W {
        self.variant(TRIG5_ENABLE_A::TRIG5_ENABLE_0)
    }
    #[doc = "TRIG5 DMA request enabled."]
    #[inline(always)]
    pub fn trig5_enable_1(self) -> &'a mut W {
        self.variant(TRIG5_ENABLE_A::TRIG5_ENABLE_1)
    }
}
#[doc = "Field `TRIG6_ENABLE` reader - Enable DMA request when TRIG6 done."]
pub type TRIG6_ENABLE_R = crate::BitReader<TRIG6_ENABLE_A>;
#[doc = "Enable DMA request when TRIG6 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_ENABLE_A {
    #[doc = "0: TRIG6 DMA request disabled."]
    TRIG6_ENABLE_0 = 0,
    #[doc = "1: TRIG6 DMA request enabled."]
    TRIG6_ENABLE_1 = 1,
}
impl From<TRIG6_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_ENABLE_A {
        match self.bits {
            false => TRIG6_ENABLE_A::TRIG6_ENABLE_0,
            true => TRIG6_ENABLE_A::TRIG6_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig6_enable_0(&self) -> bool {
        *self == TRIG6_ENABLE_A::TRIG6_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig6_enable_1(&self) -> bool {
        *self == TRIG6_ENABLE_A::TRIG6_ENABLE_1
    }
}
#[doc = "Field `TRIG6_ENABLE` writer - Enable DMA request when TRIG6 done."]
pub type TRIG6_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG6_ENABLE_A, O>;
impl<'a, const O: u8> TRIG6_ENABLE_W<'a, O> {
    #[doc = "TRIG6 DMA request disabled."]
    #[inline(always)]
    pub fn trig6_enable_0(self) -> &'a mut W {
        self.variant(TRIG6_ENABLE_A::TRIG6_ENABLE_0)
    }
    #[doc = "TRIG6 DMA request enabled."]
    #[inline(always)]
    pub fn trig6_enable_1(self) -> &'a mut W {
        self.variant(TRIG6_ENABLE_A::TRIG6_ENABLE_1)
    }
}
#[doc = "Field `TRIG7_ENABLE` reader - Enable DMA request when TRIG7 done."]
pub type TRIG7_ENABLE_R = crate::BitReader<TRIG7_ENABLE_A>;
#[doc = "Enable DMA request when TRIG7 done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_ENABLE_A {
    #[doc = "0: TRIG7 DMA request disabled."]
    TRIG7_ENABLE_0 = 0,
    #[doc = "1: TRIG7 DMA request enabled."]
    TRIG7_ENABLE_1 = 1,
}
impl From<TRIG7_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_ENABLE_A {
        match self.bits {
            false => TRIG7_ENABLE_A::TRIG7_ENABLE_0,
            true => TRIG7_ENABLE_A::TRIG7_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig7_enable_0(&self) -> bool {
        *self == TRIG7_ENABLE_A::TRIG7_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig7_enable_1(&self) -> bool {
        *self == TRIG7_ENABLE_A::TRIG7_ENABLE_1
    }
}
#[doc = "Field `TRIG7_ENABLE` writer - Enable DMA request when TRIG7 done."]
pub type TRIG7_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_CTRL_SPEC, TRIG7_ENABLE_A, O>;
impl<'a, const O: u8> TRIG7_ENABLE_W<'a, O> {
    #[doc = "TRIG7 DMA request disabled."]
    #[inline(always)]
    pub fn trig7_enable_0(self) -> &'a mut W {
        self.variant(TRIG7_ENABLE_A::TRIG7_ENABLE_0)
    }
    #[doc = "TRIG7 DMA request enabled."]
    #[inline(always)]
    pub fn trig7_enable_1(self) -> &'a mut W {
        self.variant(TRIG7_ENABLE_A::TRIG7_ENABLE_1)
    }
}
#[doc = "Field `TRIG0_REQ` reader - Flag bit for DMA request"]
pub type TRIG0_REQ_R = crate::BitReader<TRIG0_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_REQ_A {
    #[doc = "0: TRIG0_REQ not detected."]
    TRIG0_REQ_0 = 0,
    #[doc = "1: TRIG0_REQ detected."]
    TRIG0_REQ_1 = 1,
}
impl From<TRIG0_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_REQ_A {
        match self.bits {
            false => TRIG0_REQ_A::TRIG0_REQ_0,
            true => TRIG0_REQ_A::TRIG0_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_REQ_0`"]
    #[inline(always)]
    pub fn is_trig0_req_0(&self) -> bool {
        *self == TRIG0_REQ_A::TRIG0_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_REQ_1`"]
    #[inline(always)]
    pub fn is_trig0_req_1(&self) -> bool {
        *self == TRIG0_REQ_A::TRIG0_REQ_1
    }
}
#[doc = "Field `TRIG0_REQ` writer - Flag bit for DMA request"]
pub type TRIG0_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG0_REQ_A, O>;
impl<'a, const O: u8> TRIG0_REQ_W<'a, O> {
    #[doc = "TRIG0_REQ not detected."]
    #[inline(always)]
    pub fn trig0_req_0(self) -> &'a mut W {
        self.variant(TRIG0_REQ_A::TRIG0_REQ_0)
    }
    #[doc = "TRIG0_REQ detected."]
    #[inline(always)]
    pub fn trig0_req_1(self) -> &'a mut W {
        self.variant(TRIG0_REQ_A::TRIG0_REQ_1)
    }
}
#[doc = "Field `TRIG1_REQ` reader - Flag bit for DMA request"]
pub type TRIG1_REQ_R = crate::BitReader<TRIG1_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_REQ_A {
    #[doc = "0: TRIG1_REQ not detected."]
    TRIG1_REQ_0 = 0,
    #[doc = "1: TRIG1_REQ detected."]
    TRIG1_REQ_1 = 1,
}
impl From<TRIG1_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_REQ_A {
        match self.bits {
            false => TRIG1_REQ_A::TRIG1_REQ_0,
            true => TRIG1_REQ_A::TRIG1_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_REQ_0`"]
    #[inline(always)]
    pub fn is_trig1_req_0(&self) -> bool {
        *self == TRIG1_REQ_A::TRIG1_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_REQ_1`"]
    #[inline(always)]
    pub fn is_trig1_req_1(&self) -> bool {
        *self == TRIG1_REQ_A::TRIG1_REQ_1
    }
}
#[doc = "Field `TRIG1_REQ` writer - Flag bit for DMA request"]
pub type TRIG1_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG1_REQ_A, O>;
impl<'a, const O: u8> TRIG1_REQ_W<'a, O> {
    #[doc = "TRIG1_REQ not detected."]
    #[inline(always)]
    pub fn trig1_req_0(self) -> &'a mut W {
        self.variant(TRIG1_REQ_A::TRIG1_REQ_0)
    }
    #[doc = "TRIG1_REQ detected."]
    #[inline(always)]
    pub fn trig1_req_1(self) -> &'a mut W {
        self.variant(TRIG1_REQ_A::TRIG1_REQ_1)
    }
}
#[doc = "Field `TRIG2_REQ` reader - Flag bit for DMA request"]
pub type TRIG2_REQ_R = crate::BitReader<TRIG2_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_REQ_A {
    #[doc = "0: TRIG2_REQ not detected."]
    TRIG2_REQ_0 = 0,
    #[doc = "1: TRIG2_REQ detected."]
    TRIG2_REQ_1 = 1,
}
impl From<TRIG2_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_REQ_A {
        match self.bits {
            false => TRIG2_REQ_A::TRIG2_REQ_0,
            true => TRIG2_REQ_A::TRIG2_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_REQ_0`"]
    #[inline(always)]
    pub fn is_trig2_req_0(&self) -> bool {
        *self == TRIG2_REQ_A::TRIG2_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_REQ_1`"]
    #[inline(always)]
    pub fn is_trig2_req_1(&self) -> bool {
        *self == TRIG2_REQ_A::TRIG2_REQ_1
    }
}
#[doc = "Field `TRIG2_REQ` writer - Flag bit for DMA request"]
pub type TRIG2_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG2_REQ_A, O>;
impl<'a, const O: u8> TRIG2_REQ_W<'a, O> {
    #[doc = "TRIG2_REQ not detected."]
    #[inline(always)]
    pub fn trig2_req_0(self) -> &'a mut W {
        self.variant(TRIG2_REQ_A::TRIG2_REQ_0)
    }
    #[doc = "TRIG2_REQ detected."]
    #[inline(always)]
    pub fn trig2_req_1(self) -> &'a mut W {
        self.variant(TRIG2_REQ_A::TRIG2_REQ_1)
    }
}
#[doc = "Field `TRIG3_REQ` reader - Flag bit for DMA request"]
pub type TRIG3_REQ_R = crate::BitReader<TRIG3_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_REQ_A {
    #[doc = "0: TRIG3_REQ not detected."]
    TRIG3_REQ_0 = 0,
    #[doc = "1: TRIG3_REQ detected."]
    TRIG3_REQ_1 = 1,
}
impl From<TRIG3_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_REQ_A {
        match self.bits {
            false => TRIG3_REQ_A::TRIG3_REQ_0,
            true => TRIG3_REQ_A::TRIG3_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_REQ_0`"]
    #[inline(always)]
    pub fn is_trig3_req_0(&self) -> bool {
        *self == TRIG3_REQ_A::TRIG3_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_REQ_1`"]
    #[inline(always)]
    pub fn is_trig3_req_1(&self) -> bool {
        *self == TRIG3_REQ_A::TRIG3_REQ_1
    }
}
#[doc = "Field `TRIG3_REQ` writer - Flag bit for DMA request"]
pub type TRIG3_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG3_REQ_A, O>;
impl<'a, const O: u8> TRIG3_REQ_W<'a, O> {
    #[doc = "TRIG3_REQ not detected."]
    #[inline(always)]
    pub fn trig3_req_0(self) -> &'a mut W {
        self.variant(TRIG3_REQ_A::TRIG3_REQ_0)
    }
    #[doc = "TRIG3_REQ detected."]
    #[inline(always)]
    pub fn trig3_req_1(self) -> &'a mut W {
        self.variant(TRIG3_REQ_A::TRIG3_REQ_1)
    }
}
#[doc = "Field `TRIG4_REQ` reader - Flag bit for DMA request"]
pub type TRIG4_REQ_R = crate::BitReader<TRIG4_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_REQ_A {
    #[doc = "0: TRIG4_REQ not detected."]
    TRIG4_REQ_0 = 0,
    #[doc = "1: TRIG4_REQ detected."]
    TRIG4_REQ_1 = 1,
}
impl From<TRIG4_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_REQ_A {
        match self.bits {
            false => TRIG4_REQ_A::TRIG4_REQ_0,
            true => TRIG4_REQ_A::TRIG4_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_REQ_0`"]
    #[inline(always)]
    pub fn is_trig4_req_0(&self) -> bool {
        *self == TRIG4_REQ_A::TRIG4_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_REQ_1`"]
    #[inline(always)]
    pub fn is_trig4_req_1(&self) -> bool {
        *self == TRIG4_REQ_A::TRIG4_REQ_1
    }
}
#[doc = "Field `TRIG4_REQ` writer - Flag bit for DMA request"]
pub type TRIG4_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG4_REQ_A, O>;
impl<'a, const O: u8> TRIG4_REQ_W<'a, O> {
    #[doc = "TRIG4_REQ not detected."]
    #[inline(always)]
    pub fn trig4_req_0(self) -> &'a mut W {
        self.variant(TRIG4_REQ_A::TRIG4_REQ_0)
    }
    #[doc = "TRIG4_REQ detected."]
    #[inline(always)]
    pub fn trig4_req_1(self) -> &'a mut W {
        self.variant(TRIG4_REQ_A::TRIG4_REQ_1)
    }
}
#[doc = "Field `TRIG5_REQ` reader - Flag bit for DMA request"]
pub type TRIG5_REQ_R = crate::BitReader<TRIG5_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_REQ_A {
    #[doc = "0: TRIG5_REQ not detected."]
    TRIG5_REQ_0 = 0,
    #[doc = "1: TRIG5_REQ detected."]
    TRIG5_REQ_1 = 1,
}
impl From<TRIG5_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_REQ_A {
        match self.bits {
            false => TRIG5_REQ_A::TRIG5_REQ_0,
            true => TRIG5_REQ_A::TRIG5_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_REQ_0`"]
    #[inline(always)]
    pub fn is_trig5_req_0(&self) -> bool {
        *self == TRIG5_REQ_A::TRIG5_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_REQ_1`"]
    #[inline(always)]
    pub fn is_trig5_req_1(&self) -> bool {
        *self == TRIG5_REQ_A::TRIG5_REQ_1
    }
}
#[doc = "Field `TRIG5_REQ` writer - Flag bit for DMA request"]
pub type TRIG5_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG5_REQ_A, O>;
impl<'a, const O: u8> TRIG5_REQ_W<'a, O> {
    #[doc = "TRIG5_REQ not detected."]
    #[inline(always)]
    pub fn trig5_req_0(self) -> &'a mut W {
        self.variant(TRIG5_REQ_A::TRIG5_REQ_0)
    }
    #[doc = "TRIG5_REQ detected."]
    #[inline(always)]
    pub fn trig5_req_1(self) -> &'a mut W {
        self.variant(TRIG5_REQ_A::TRIG5_REQ_1)
    }
}
#[doc = "Field `TRIG6_REQ` reader - Flag bit for DMA request"]
pub type TRIG6_REQ_R = crate::BitReader<TRIG6_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_REQ_A {
    #[doc = "0: TRIG6_REQ not detected."]
    TRIG6_REQ_0 = 0,
    #[doc = "1: TRIG6_REQ detected."]
    TRIG6_REQ_1 = 1,
}
impl From<TRIG6_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_REQ_A {
        match self.bits {
            false => TRIG6_REQ_A::TRIG6_REQ_0,
            true => TRIG6_REQ_A::TRIG6_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_REQ_0`"]
    #[inline(always)]
    pub fn is_trig6_req_0(&self) -> bool {
        *self == TRIG6_REQ_A::TRIG6_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_REQ_1`"]
    #[inline(always)]
    pub fn is_trig6_req_1(&self) -> bool {
        *self == TRIG6_REQ_A::TRIG6_REQ_1
    }
}
#[doc = "Field `TRIG6_REQ` writer - Flag bit for DMA request"]
pub type TRIG6_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG6_REQ_A, O>;
impl<'a, const O: u8> TRIG6_REQ_W<'a, O> {
    #[doc = "TRIG6_REQ not detected."]
    #[inline(always)]
    pub fn trig6_req_0(self) -> &'a mut W {
        self.variant(TRIG6_REQ_A::TRIG6_REQ_0)
    }
    #[doc = "TRIG6_REQ detected."]
    #[inline(always)]
    pub fn trig6_req_1(self) -> &'a mut W {
        self.variant(TRIG6_REQ_A::TRIG6_REQ_1)
    }
}
#[doc = "Field `TRIG7_REQ` reader - Flag bit for DMA request"]
pub type TRIG7_REQ_R = crate::BitReader<TRIG7_REQ_A>;
#[doc = "Flag bit for DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_REQ_A {
    #[doc = "0: TRIG7_REQ not detected."]
    TRIG7_REQ_0 = 0,
    #[doc = "1: TRIG7_REQ detected."]
    TRIG7_REQ_1 = 1,
}
impl From<TRIG7_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_REQ_A {
        match self.bits {
            false => TRIG7_REQ_A::TRIG7_REQ_0,
            true => TRIG7_REQ_A::TRIG7_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_REQ_0`"]
    #[inline(always)]
    pub fn is_trig7_req_0(&self) -> bool {
        *self == TRIG7_REQ_A::TRIG7_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_REQ_1`"]
    #[inline(always)]
    pub fn is_trig7_req_1(&self) -> bool {
        *self == TRIG7_REQ_A::TRIG7_REQ_1
    }
}
#[doc = "Field `TRIG7_REQ` writer - Flag bit for DMA request"]
pub type TRIG7_REQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, DMA_CTRL_SPEC, TRIG7_REQ_A, O>;
impl<'a, const O: u8> TRIG7_REQ_W<'a, O> {
    #[doc = "TRIG7_REQ not detected."]
    #[inline(always)]
    pub fn trig7_req_0(self) -> &'a mut W {
        self.variant(TRIG7_REQ_A::TRIG7_REQ_0)
    }
    #[doc = "TRIG7_REQ detected."]
    #[inline(always)]
    pub fn trig7_req_1(self) -> &'a mut W {
        self.variant(TRIG7_REQ_A::TRIG7_REQ_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA request when TRIG0 done."]
    #[inline(always)]
    pub fn trig0_enable(&self) -> TRIG0_ENABLE_R {
        TRIG0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DMA request when TRIG1 done."]
    #[inline(always)]
    pub fn trig1_enable(&self) -> TRIG1_ENABLE_R {
        TRIG1_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DMA request when TRIG2 done."]
    #[inline(always)]
    pub fn trig2_enable(&self) -> TRIG2_ENABLE_R {
        TRIG2_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA request when TRIG3 done."]
    #[inline(always)]
    pub fn trig3_enable(&self) -> TRIG3_ENABLE_R {
        TRIG3_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable DMA request when TRIG4 done."]
    #[inline(always)]
    pub fn trig4_enable(&self) -> TRIG4_ENABLE_R {
        TRIG4_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable DMA request when TRIG5 done."]
    #[inline(always)]
    pub fn trig5_enable(&self) -> TRIG5_ENABLE_R {
        TRIG5_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DMA request when TRIG6 done."]
    #[inline(always)]
    pub fn trig6_enable(&self) -> TRIG6_ENABLE_R {
        TRIG6_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable DMA request when TRIG7 done."]
    #[inline(always)]
    pub fn trig7_enable(&self) -> TRIG7_ENABLE_R {
        TRIG7_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig0_req(&self) -> TRIG0_REQ_R {
        TRIG0_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig1_req(&self) -> TRIG1_REQ_R {
        TRIG1_REQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig2_req(&self) -> TRIG2_REQ_R {
        TRIG2_REQ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig3_req(&self) -> TRIG3_REQ_R {
        TRIG3_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig4_req(&self) -> TRIG4_REQ_R {
        TRIG4_REQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig5_req(&self) -> TRIG5_REQ_R {
        TRIG5_REQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig6_req(&self) -> TRIG6_REQ_R {
        TRIG6_REQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Flag bit for DMA request"]
    #[inline(always)]
    pub fn trig7_req(&self) -> TRIG7_REQ_R {
        TRIG7_REQ_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA request when TRIG0 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig0_enable(&mut self) -> TRIG0_ENABLE_W<0> {
        TRIG0_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable DMA request when TRIG1 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig1_enable(&mut self) -> TRIG1_ENABLE_W<1> {
        TRIG1_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enable DMA request when TRIG2 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig2_enable(&mut self) -> TRIG2_ENABLE_W<2> {
        TRIG2_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable DMA request when TRIG3 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig3_enable(&mut self) -> TRIG3_ENABLE_W<3> {
        TRIG3_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Enable DMA request when TRIG4 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig4_enable(&mut self) -> TRIG4_ENABLE_W<4> {
        TRIG4_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Enable DMA request when TRIG5 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig5_enable(&mut self) -> TRIG5_ENABLE_W<5> {
        TRIG5_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Enable DMA request when TRIG6 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig6_enable(&mut self) -> TRIG6_ENABLE_W<6> {
        TRIG6_ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable DMA request when TRIG7 done."]
    #[inline(always)]
    #[must_use]
    pub fn trig7_enable(&mut self) -> TRIG7_ENABLE_W<7> {
        TRIG7_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig0_req(&mut self) -> TRIG0_REQ_W<16> {
        TRIG0_REQ_W::new(self)
    }
    #[doc = "Bit 17 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig1_req(&mut self) -> TRIG1_REQ_W<17> {
        TRIG1_REQ_W::new(self)
    }
    #[doc = "Bit 18 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig2_req(&mut self) -> TRIG2_REQ_W<18> {
        TRIG2_REQ_W::new(self)
    }
    #[doc = "Bit 19 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig3_req(&mut self) -> TRIG3_REQ_W<19> {
        TRIG3_REQ_W::new(self)
    }
    #[doc = "Bit 20 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig4_req(&mut self) -> TRIG4_REQ_W<20> {
        TRIG4_REQ_W::new(self)
    }
    #[doc = "Bit 21 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig5_req(&mut self) -> TRIG5_REQ_W<21> {
        TRIG5_REQ_W::new(self)
    }
    #[doc = "Bit 22 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig6_req(&mut self) -> TRIG6_REQ_W<22> {
        TRIG6_REQ_W::new(self)
    }
    #[doc = "Bit 23 - Flag bit for DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn trig7_req(&mut self) -> TRIG7_REQ_W<23> {
        TRIG7_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC DMA control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ctrl](index.html) module"]
pub struct DMA_CTRL_SPEC;
impl crate::RegisterSpec for DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ctrl::R](R) reader structure"]
impl crate::Readable for DMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ctrl::W](W) writer structure"]
impl crate::Writable for DMA_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x00ff_0000;
}
#[doc = "`reset()` method sets DMA_CTRL to value 0"]
impl crate::Resettable for DMA_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
