#[doc = "Register `DONE0_1_IRQ` reader"]
pub struct R(crate::R<DONE0_1_IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DONE0_1_IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DONE0_1_IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DONE0_1_IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DONE0_1_IRQ` writer"]
pub struct W(crate::W<DONE0_1_IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DONE0_1_IRQ_SPEC>;
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
impl From<crate::W<DONE0_1_IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DONE0_1_IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG0_DONE0` reader - TRIG0 done0 interrupt detection."]
pub type TRIG0_DONE0_R = crate::BitReader<TRIG0_DONE0_A>;
#[doc = "TRIG0 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_DONE0_A {
    #[doc = "0: No TRIG0_DONE0 interrupt detected"]
    TRIG0_DONE0_0 = 0,
    #[doc = "1: TRIG0_DONE0 interrupt detected"]
    TRIG0_DONE0_1 = 1,
}
impl From<TRIG0_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_DONE0_A {
        match self.bits {
            false => TRIG0_DONE0_A::TRIG0_DONE0_0,
            true => TRIG0_DONE0_A::TRIG0_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig0_done0_0(&self) -> bool {
        *self == TRIG0_DONE0_A::TRIG0_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig0_done0_1(&self) -> bool {
        *self == TRIG0_DONE0_A::TRIG0_DONE0_1
    }
}
#[doc = "Field `TRIG0_DONE0` writer - TRIG0 done0 interrupt detection."]
pub type TRIG0_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG0_DONE0_A, O>;
impl<'a, const O: u8> TRIG0_DONE0_W<'a, O> {
    #[doc = "No TRIG0_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done0_0(self) -> &'a mut W {
        self.variant(TRIG0_DONE0_A::TRIG0_DONE0_0)
    }
    #[doc = "TRIG0_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done0_1(self) -> &'a mut W {
        self.variant(TRIG0_DONE0_A::TRIG0_DONE0_1)
    }
}
#[doc = "Field `TRIG1_DONE0` reader - TRIG1 done0 interrupt detection."]
pub type TRIG1_DONE0_R = crate::BitReader<TRIG1_DONE0_A>;
#[doc = "TRIG1 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_DONE0_A {
    #[doc = "0: No TRIG1_DONE0 interrupt detected"]
    TRIG1_DONE0_0 = 0,
    #[doc = "1: TRIG1_DONE0 interrupt detected"]
    TRIG1_DONE0_1 = 1,
}
impl From<TRIG1_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_DONE0_A {
        match self.bits {
            false => TRIG1_DONE0_A::TRIG1_DONE0_0,
            true => TRIG1_DONE0_A::TRIG1_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig1_done0_0(&self) -> bool {
        *self == TRIG1_DONE0_A::TRIG1_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig1_done0_1(&self) -> bool {
        *self == TRIG1_DONE0_A::TRIG1_DONE0_1
    }
}
#[doc = "Field `TRIG1_DONE0` writer - TRIG1 done0 interrupt detection."]
pub type TRIG1_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG1_DONE0_A, O>;
impl<'a, const O: u8> TRIG1_DONE0_W<'a, O> {
    #[doc = "No TRIG1_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done0_0(self) -> &'a mut W {
        self.variant(TRIG1_DONE0_A::TRIG1_DONE0_0)
    }
    #[doc = "TRIG1_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done0_1(self) -> &'a mut W {
        self.variant(TRIG1_DONE0_A::TRIG1_DONE0_1)
    }
}
#[doc = "Field `TRIG2_DONE0` reader - TRIG2 done0 interrupt detection."]
pub type TRIG2_DONE0_R = crate::BitReader<TRIG2_DONE0_A>;
#[doc = "TRIG2 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_DONE0_A {
    #[doc = "0: No TRIG2_DONE0 interrupt detected"]
    TRIG2_DONE0_0 = 0,
    #[doc = "1: TRIG2_DONE0 interrupt detected"]
    TRIG2_DONE0_1 = 1,
}
impl From<TRIG2_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_DONE0_A {
        match self.bits {
            false => TRIG2_DONE0_A::TRIG2_DONE0_0,
            true => TRIG2_DONE0_A::TRIG2_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig2_done0_0(&self) -> bool {
        *self == TRIG2_DONE0_A::TRIG2_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig2_done0_1(&self) -> bool {
        *self == TRIG2_DONE0_A::TRIG2_DONE0_1
    }
}
#[doc = "Field `TRIG2_DONE0` writer - TRIG2 done0 interrupt detection."]
pub type TRIG2_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG2_DONE0_A, O>;
impl<'a, const O: u8> TRIG2_DONE0_W<'a, O> {
    #[doc = "No TRIG2_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done0_0(self) -> &'a mut W {
        self.variant(TRIG2_DONE0_A::TRIG2_DONE0_0)
    }
    #[doc = "TRIG2_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done0_1(self) -> &'a mut W {
        self.variant(TRIG2_DONE0_A::TRIG2_DONE0_1)
    }
}
#[doc = "Field `TRIG3_DONE0` reader - TRIG3 done0 interrupt detection."]
pub type TRIG3_DONE0_R = crate::BitReader<TRIG3_DONE0_A>;
#[doc = "TRIG3 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_DONE0_A {
    #[doc = "0: No TRIG3_DONE0 interrupt detected"]
    TRIG3_DONE0_0 = 0,
    #[doc = "1: TRIG3_DONE0 interrupt detected"]
    TRIG3_DONE0_1 = 1,
}
impl From<TRIG3_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_DONE0_A {
        match self.bits {
            false => TRIG3_DONE0_A::TRIG3_DONE0_0,
            true => TRIG3_DONE0_A::TRIG3_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig3_done0_0(&self) -> bool {
        *self == TRIG3_DONE0_A::TRIG3_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig3_done0_1(&self) -> bool {
        *self == TRIG3_DONE0_A::TRIG3_DONE0_1
    }
}
#[doc = "Field `TRIG3_DONE0` writer - TRIG3 done0 interrupt detection."]
pub type TRIG3_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG3_DONE0_A, O>;
impl<'a, const O: u8> TRIG3_DONE0_W<'a, O> {
    #[doc = "No TRIG3_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done0_0(self) -> &'a mut W {
        self.variant(TRIG3_DONE0_A::TRIG3_DONE0_0)
    }
    #[doc = "TRIG3_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done0_1(self) -> &'a mut W {
        self.variant(TRIG3_DONE0_A::TRIG3_DONE0_1)
    }
}
#[doc = "Field `TRIG4_DONE0` reader - TRIG4 done0 interrupt detection."]
pub type TRIG4_DONE0_R = crate::BitReader<TRIG4_DONE0_A>;
#[doc = "TRIG4 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_DONE0_A {
    #[doc = "0: No TRIG4_DONE0 interrupt detected"]
    TRIG4_DONE0_0 = 0,
    #[doc = "1: TRIG4_DONE0 interrupt detected"]
    TRIG4_DONE0_1 = 1,
}
impl From<TRIG4_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_DONE0_A {
        match self.bits {
            false => TRIG4_DONE0_A::TRIG4_DONE0_0,
            true => TRIG4_DONE0_A::TRIG4_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig4_done0_0(&self) -> bool {
        *self == TRIG4_DONE0_A::TRIG4_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig4_done0_1(&self) -> bool {
        *self == TRIG4_DONE0_A::TRIG4_DONE0_1
    }
}
#[doc = "Field `TRIG4_DONE0` writer - TRIG4 done0 interrupt detection."]
pub type TRIG4_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG4_DONE0_A, O>;
impl<'a, const O: u8> TRIG4_DONE0_W<'a, O> {
    #[doc = "No TRIG4_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done0_0(self) -> &'a mut W {
        self.variant(TRIG4_DONE0_A::TRIG4_DONE0_0)
    }
    #[doc = "TRIG4_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done0_1(self) -> &'a mut W {
        self.variant(TRIG4_DONE0_A::TRIG4_DONE0_1)
    }
}
#[doc = "Field `TRIG5_DONE0` reader - TRIG5 done0 interrupt detection."]
pub type TRIG5_DONE0_R = crate::BitReader<TRIG5_DONE0_A>;
#[doc = "TRIG5 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_DONE0_A {
    #[doc = "0: No TRIG5_DONE0 interrupt detected"]
    TRIG5_DONE0_0 = 0,
    #[doc = "1: TRIG5_DONE0 interrupt detected"]
    TRIG5_DONE0_1 = 1,
}
impl From<TRIG5_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_DONE0_A {
        match self.bits {
            false => TRIG5_DONE0_A::TRIG5_DONE0_0,
            true => TRIG5_DONE0_A::TRIG5_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig5_done0_0(&self) -> bool {
        *self == TRIG5_DONE0_A::TRIG5_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig5_done0_1(&self) -> bool {
        *self == TRIG5_DONE0_A::TRIG5_DONE0_1
    }
}
#[doc = "Field `TRIG5_DONE0` writer - TRIG5 done0 interrupt detection."]
pub type TRIG5_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG5_DONE0_A, O>;
impl<'a, const O: u8> TRIG5_DONE0_W<'a, O> {
    #[doc = "No TRIG5_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done0_0(self) -> &'a mut W {
        self.variant(TRIG5_DONE0_A::TRIG5_DONE0_0)
    }
    #[doc = "TRIG5_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done0_1(self) -> &'a mut W {
        self.variant(TRIG5_DONE0_A::TRIG5_DONE0_1)
    }
}
#[doc = "Field `TRIG6_DONE0` reader - TRIG6 done0 interrupt detection."]
pub type TRIG6_DONE0_R = crate::BitReader<TRIG6_DONE0_A>;
#[doc = "TRIG6 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_DONE0_A {
    #[doc = "0: No TRIG6_DONE0 interrupt detected"]
    TRIG6_DONE0_0 = 0,
    #[doc = "1: TRIG6_DONE0 interrupt detected"]
    TRIG6_DONE0_1 = 1,
}
impl From<TRIG6_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_DONE0_A {
        match self.bits {
            false => TRIG6_DONE0_A::TRIG6_DONE0_0,
            true => TRIG6_DONE0_A::TRIG6_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig6_done0_0(&self) -> bool {
        *self == TRIG6_DONE0_A::TRIG6_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig6_done0_1(&self) -> bool {
        *self == TRIG6_DONE0_A::TRIG6_DONE0_1
    }
}
#[doc = "Field `TRIG6_DONE0` writer - TRIG6 done0 interrupt detection."]
pub type TRIG6_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG6_DONE0_A, O>;
impl<'a, const O: u8> TRIG6_DONE0_W<'a, O> {
    #[doc = "No TRIG6_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done0_0(self) -> &'a mut W {
        self.variant(TRIG6_DONE0_A::TRIG6_DONE0_0)
    }
    #[doc = "TRIG6_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done0_1(self) -> &'a mut W {
        self.variant(TRIG6_DONE0_A::TRIG6_DONE0_1)
    }
}
#[doc = "Field `TRIG7_DONE0` reader - TRIG7 done0 interrupt detection."]
pub type TRIG7_DONE0_R = crate::BitReader<TRIG7_DONE0_A>;
#[doc = "TRIG7 done0 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_DONE0_A {
    #[doc = "0: No TRIG7_DONE0 interrupt detected"]
    TRIG7_DONE0_0 = 0,
    #[doc = "1: TRIG7_DONE0 interrupt detected"]
    TRIG7_DONE0_1 = 1,
}
impl From<TRIG7_DONE0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_DONE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_DONE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_DONE0_A {
        match self.bits {
            false => TRIG7_DONE0_A::TRIG7_DONE0_0,
            true => TRIG7_DONE0_A::TRIG7_DONE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE0_0`"]
    #[inline(always)]
    pub fn is_trig7_done0_0(&self) -> bool {
        *self == TRIG7_DONE0_A::TRIG7_DONE0_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE0_1`"]
    #[inline(always)]
    pub fn is_trig7_done0_1(&self) -> bool {
        *self == TRIG7_DONE0_A::TRIG7_DONE0_1
    }
}
#[doc = "Field `TRIG7_DONE0` writer - TRIG7 done0 interrupt detection."]
pub type TRIG7_DONE0_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG7_DONE0_A, O>;
impl<'a, const O: u8> TRIG7_DONE0_W<'a, O> {
    #[doc = "No TRIG7_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done0_0(self) -> &'a mut W {
        self.variant(TRIG7_DONE0_A::TRIG7_DONE0_0)
    }
    #[doc = "TRIG7_DONE0 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done0_1(self) -> &'a mut W {
        self.variant(TRIG7_DONE0_A::TRIG7_DONE0_1)
    }
}
#[doc = "Field `TRIG0_DONE1` reader - TRIG0 done1 interrupt detection."]
pub type TRIG0_DONE1_R = crate::BitReader<TRIG0_DONE1_A>;
#[doc = "TRIG0 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_DONE1_A {
    #[doc = "0: No TRIG0_DONE1 interrupt detected"]
    TRIG0_DONE1_0 = 0,
    #[doc = "1: TRIG0_DONE1 interrupt detected"]
    TRIG0_DONE1_1 = 1,
}
impl From<TRIG0_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_DONE1_A {
        match self.bits {
            false => TRIG0_DONE1_A::TRIG0_DONE1_0,
            true => TRIG0_DONE1_A::TRIG0_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig0_done1_0(&self) -> bool {
        *self == TRIG0_DONE1_A::TRIG0_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig0_done1_1(&self) -> bool {
        *self == TRIG0_DONE1_A::TRIG0_DONE1_1
    }
}
#[doc = "Field `TRIG0_DONE1` writer - TRIG0 done1 interrupt detection."]
pub type TRIG0_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG0_DONE1_A, O>;
impl<'a, const O: u8> TRIG0_DONE1_W<'a, O> {
    #[doc = "No TRIG0_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done1_0(self) -> &'a mut W {
        self.variant(TRIG0_DONE1_A::TRIG0_DONE1_0)
    }
    #[doc = "TRIG0_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done1_1(self) -> &'a mut W {
        self.variant(TRIG0_DONE1_A::TRIG0_DONE1_1)
    }
}
#[doc = "Field `TRIG1_DONE1` reader - TRIG1 done1 interrupt detection."]
pub type TRIG1_DONE1_R = crate::BitReader<TRIG1_DONE1_A>;
#[doc = "TRIG1 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_DONE1_A {
    #[doc = "0: No TRIG1_DONE1 interrupt detected"]
    TRIG1_DONE1_0 = 0,
    #[doc = "1: TRIG1_DONE1 interrupt detected"]
    TRIG1_DONE1_1 = 1,
}
impl From<TRIG1_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_DONE1_A {
        match self.bits {
            false => TRIG1_DONE1_A::TRIG1_DONE1_0,
            true => TRIG1_DONE1_A::TRIG1_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig1_done1_0(&self) -> bool {
        *self == TRIG1_DONE1_A::TRIG1_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig1_done1_1(&self) -> bool {
        *self == TRIG1_DONE1_A::TRIG1_DONE1_1
    }
}
#[doc = "Field `TRIG1_DONE1` writer - TRIG1 done1 interrupt detection."]
pub type TRIG1_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG1_DONE1_A, O>;
impl<'a, const O: u8> TRIG1_DONE1_W<'a, O> {
    #[doc = "No TRIG1_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done1_0(self) -> &'a mut W {
        self.variant(TRIG1_DONE1_A::TRIG1_DONE1_0)
    }
    #[doc = "TRIG1_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done1_1(self) -> &'a mut W {
        self.variant(TRIG1_DONE1_A::TRIG1_DONE1_1)
    }
}
#[doc = "Field `TRIG2_DONE1` reader - TRIG2 done1 interrupt detection."]
pub type TRIG2_DONE1_R = crate::BitReader<TRIG2_DONE1_A>;
#[doc = "TRIG2 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_DONE1_A {
    #[doc = "0: No TRIG2_DONE1 interrupt detected"]
    TRIG2_DONE1_0 = 0,
    #[doc = "1: TRIG2_DONE1 interrupt detected"]
    TRIG2_DONE1_1 = 1,
}
impl From<TRIG2_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_DONE1_A {
        match self.bits {
            false => TRIG2_DONE1_A::TRIG2_DONE1_0,
            true => TRIG2_DONE1_A::TRIG2_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig2_done1_0(&self) -> bool {
        *self == TRIG2_DONE1_A::TRIG2_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig2_done1_1(&self) -> bool {
        *self == TRIG2_DONE1_A::TRIG2_DONE1_1
    }
}
#[doc = "Field `TRIG2_DONE1` writer - TRIG2 done1 interrupt detection."]
pub type TRIG2_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG2_DONE1_A, O>;
impl<'a, const O: u8> TRIG2_DONE1_W<'a, O> {
    #[doc = "No TRIG2_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done1_0(self) -> &'a mut W {
        self.variant(TRIG2_DONE1_A::TRIG2_DONE1_0)
    }
    #[doc = "TRIG2_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done1_1(self) -> &'a mut W {
        self.variant(TRIG2_DONE1_A::TRIG2_DONE1_1)
    }
}
#[doc = "Field `TRIG3_DONE1` reader - TRIG3 done1 interrupt detection."]
pub type TRIG3_DONE1_R = crate::BitReader<TRIG3_DONE1_A>;
#[doc = "TRIG3 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_DONE1_A {
    #[doc = "0: No TRIG3_DONE1 interrupt detected"]
    TRIG3_DONE1_0 = 0,
    #[doc = "1: TRIG3_DONE1 interrupt detected"]
    TRIG3_DONE1_1 = 1,
}
impl From<TRIG3_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_DONE1_A {
        match self.bits {
            false => TRIG3_DONE1_A::TRIG3_DONE1_0,
            true => TRIG3_DONE1_A::TRIG3_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig3_done1_0(&self) -> bool {
        *self == TRIG3_DONE1_A::TRIG3_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig3_done1_1(&self) -> bool {
        *self == TRIG3_DONE1_A::TRIG3_DONE1_1
    }
}
#[doc = "Field `TRIG3_DONE1` writer - TRIG3 done1 interrupt detection."]
pub type TRIG3_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG3_DONE1_A, O>;
impl<'a, const O: u8> TRIG3_DONE1_W<'a, O> {
    #[doc = "No TRIG3_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done1_0(self) -> &'a mut W {
        self.variant(TRIG3_DONE1_A::TRIG3_DONE1_0)
    }
    #[doc = "TRIG3_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done1_1(self) -> &'a mut W {
        self.variant(TRIG3_DONE1_A::TRIG3_DONE1_1)
    }
}
#[doc = "Field `TRIG4_DONE1` reader - TRIG4 done1 interrupt detection."]
pub type TRIG4_DONE1_R = crate::BitReader<TRIG4_DONE1_A>;
#[doc = "TRIG4 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_DONE1_A {
    #[doc = "0: No TRIG4_DONE1 interrupt detected"]
    TRIG4_DONE1_0 = 0,
    #[doc = "1: TRIG4_DONE1 interrupt detected"]
    TRIG4_DONE1_1 = 1,
}
impl From<TRIG4_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_DONE1_A {
        match self.bits {
            false => TRIG4_DONE1_A::TRIG4_DONE1_0,
            true => TRIG4_DONE1_A::TRIG4_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig4_done1_0(&self) -> bool {
        *self == TRIG4_DONE1_A::TRIG4_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig4_done1_1(&self) -> bool {
        *self == TRIG4_DONE1_A::TRIG4_DONE1_1
    }
}
#[doc = "Field `TRIG4_DONE1` writer - TRIG4 done1 interrupt detection."]
pub type TRIG4_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG4_DONE1_A, O>;
impl<'a, const O: u8> TRIG4_DONE1_W<'a, O> {
    #[doc = "No TRIG4_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done1_0(self) -> &'a mut W {
        self.variant(TRIG4_DONE1_A::TRIG4_DONE1_0)
    }
    #[doc = "TRIG4_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done1_1(self) -> &'a mut W {
        self.variant(TRIG4_DONE1_A::TRIG4_DONE1_1)
    }
}
#[doc = "Field `TRIG5_DONE1` reader - TRIG5 done1 interrupt detection."]
pub type TRIG5_DONE1_R = crate::BitReader<TRIG5_DONE1_A>;
#[doc = "TRIG5 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_DONE1_A {
    #[doc = "0: No TRIG5_DONE1 interrupt detected"]
    TRIG5_DONE1_0 = 0,
    #[doc = "1: TRIG5_DONE1 interrupt detected"]
    TRIG5_DONE1_1 = 1,
}
impl From<TRIG5_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_DONE1_A {
        match self.bits {
            false => TRIG5_DONE1_A::TRIG5_DONE1_0,
            true => TRIG5_DONE1_A::TRIG5_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig5_done1_0(&self) -> bool {
        *self == TRIG5_DONE1_A::TRIG5_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig5_done1_1(&self) -> bool {
        *self == TRIG5_DONE1_A::TRIG5_DONE1_1
    }
}
#[doc = "Field `TRIG5_DONE1` writer - TRIG5 done1 interrupt detection."]
pub type TRIG5_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG5_DONE1_A, O>;
impl<'a, const O: u8> TRIG5_DONE1_W<'a, O> {
    #[doc = "No TRIG5_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done1_0(self) -> &'a mut W {
        self.variant(TRIG5_DONE1_A::TRIG5_DONE1_0)
    }
    #[doc = "TRIG5_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done1_1(self) -> &'a mut W {
        self.variant(TRIG5_DONE1_A::TRIG5_DONE1_1)
    }
}
#[doc = "Field `TRIG6_DONE1` reader - TRIG6 done1 interrupt detection."]
pub type TRIG6_DONE1_R = crate::BitReader<TRIG6_DONE1_A>;
#[doc = "TRIG6 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_DONE1_A {
    #[doc = "0: No TRIG6_DONE1 interrupt detected"]
    TRIG6_DONE1_0 = 0,
    #[doc = "1: TRIG6_DONE1 interrupt detected"]
    TRIG6_DONE1_1 = 1,
}
impl From<TRIG6_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_DONE1_A {
        match self.bits {
            false => TRIG6_DONE1_A::TRIG6_DONE1_0,
            true => TRIG6_DONE1_A::TRIG6_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig6_done1_0(&self) -> bool {
        *self == TRIG6_DONE1_A::TRIG6_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig6_done1_1(&self) -> bool {
        *self == TRIG6_DONE1_A::TRIG6_DONE1_1
    }
}
#[doc = "Field `TRIG6_DONE1` writer - TRIG6 done1 interrupt detection."]
pub type TRIG6_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG6_DONE1_A, O>;
impl<'a, const O: u8> TRIG6_DONE1_W<'a, O> {
    #[doc = "No TRIG6_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done1_0(self) -> &'a mut W {
        self.variant(TRIG6_DONE1_A::TRIG6_DONE1_0)
    }
    #[doc = "TRIG6_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done1_1(self) -> &'a mut W {
        self.variant(TRIG6_DONE1_A::TRIG6_DONE1_1)
    }
}
#[doc = "Field `TRIG7_DONE1` reader - TRIG7 done1 interrupt detection."]
pub type TRIG7_DONE1_R = crate::BitReader<TRIG7_DONE1_A>;
#[doc = "TRIG7 done1 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_DONE1_A {
    #[doc = "0: No TRIG7_DONE1 interrupt detected"]
    TRIG7_DONE1_0 = 0,
    #[doc = "1: TRIG7_DONE1 interrupt detected"]
    TRIG7_DONE1_1 = 1,
}
impl From<TRIG7_DONE1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_DONE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_DONE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_DONE1_A {
        match self.bits {
            false => TRIG7_DONE1_A::TRIG7_DONE1_0,
            true => TRIG7_DONE1_A::TRIG7_DONE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE1_0`"]
    #[inline(always)]
    pub fn is_trig7_done1_0(&self) -> bool {
        *self == TRIG7_DONE1_A::TRIG7_DONE1_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE1_1`"]
    #[inline(always)]
    pub fn is_trig7_done1_1(&self) -> bool {
        *self == TRIG7_DONE1_A::TRIG7_DONE1_1
    }
}
#[doc = "Field `TRIG7_DONE1` writer - TRIG7 done1 interrupt detection."]
pub type TRIG7_DONE1_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE0_1_IRQ_SPEC, TRIG7_DONE1_A, O>;
impl<'a, const O: u8> TRIG7_DONE1_W<'a, O> {
    #[doc = "No TRIG7_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done1_0(self) -> &'a mut W {
        self.variant(TRIG7_DONE1_A::TRIG7_DONE1_0)
    }
    #[doc = "TRIG7_DONE1 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done1_1(self) -> &'a mut W {
        self.variant(TRIG7_DONE1_A::TRIG7_DONE1_1)
    }
}
impl R {
    #[doc = "Bit 0 - TRIG0 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig0_done0(&self) -> TRIG0_DONE0_R {
        TRIG0_DONE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TRIG1 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig1_done0(&self) -> TRIG1_DONE0_R {
        TRIG1_DONE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRIG2 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig2_done0(&self) -> TRIG2_DONE0_R {
        TRIG2_DONE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRIG3 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig3_done0(&self) -> TRIG3_DONE0_R {
        TRIG3_DONE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TRIG4 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig4_done0(&self) -> TRIG4_DONE0_R {
        TRIG4_DONE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRIG5 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig5_done0(&self) -> TRIG5_DONE0_R {
        TRIG5_DONE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRIG6 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig6_done0(&self) -> TRIG6_DONE0_R {
        TRIG6_DONE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TRIG7 done0 interrupt detection."]
    #[inline(always)]
    pub fn trig7_done0(&self) -> TRIG7_DONE0_R {
        TRIG7_DONE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TRIG0 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig0_done1(&self) -> TRIG0_DONE1_R {
        TRIG0_DONE1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRIG1 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig1_done1(&self) -> TRIG1_DONE1_R {
        TRIG1_DONE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRIG2 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig2_done1(&self) -> TRIG2_DONE1_R {
        TRIG2_DONE1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRIG3 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig3_done1(&self) -> TRIG3_DONE1_R {
        TRIG3_DONE1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRIG4 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig4_done1(&self) -> TRIG4_DONE1_R {
        TRIG4_DONE1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRIG5 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig5_done1(&self) -> TRIG5_DONE1_R {
        TRIG5_DONE1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRIG6 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig6_done1(&self) -> TRIG6_DONE1_R {
        TRIG6_DONE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TRIG7 done1 interrupt detection."]
    #[inline(always)]
    pub fn trig7_done1(&self) -> TRIG7_DONE1_R {
        TRIG7_DONE1_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig0_done0(&mut self) -> TRIG0_DONE0_W<0> {
        TRIG0_DONE0_W::new(self)
    }
    #[doc = "Bit 1 - TRIG1 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig1_done0(&mut self) -> TRIG1_DONE0_W<1> {
        TRIG1_DONE0_W::new(self)
    }
    #[doc = "Bit 2 - TRIG2 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig2_done0(&mut self) -> TRIG2_DONE0_W<2> {
        TRIG2_DONE0_W::new(self)
    }
    #[doc = "Bit 3 - TRIG3 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig3_done0(&mut self) -> TRIG3_DONE0_W<3> {
        TRIG3_DONE0_W::new(self)
    }
    #[doc = "Bit 4 - TRIG4 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig4_done0(&mut self) -> TRIG4_DONE0_W<4> {
        TRIG4_DONE0_W::new(self)
    }
    #[doc = "Bit 5 - TRIG5 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig5_done0(&mut self) -> TRIG5_DONE0_W<5> {
        TRIG5_DONE0_W::new(self)
    }
    #[doc = "Bit 6 - TRIG6 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig6_done0(&mut self) -> TRIG6_DONE0_W<6> {
        TRIG6_DONE0_W::new(self)
    }
    #[doc = "Bit 7 - TRIG7 done0 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig7_done0(&mut self) -> TRIG7_DONE0_W<7> {
        TRIG7_DONE0_W::new(self)
    }
    #[doc = "Bit 16 - TRIG0 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig0_done1(&mut self) -> TRIG0_DONE1_W<16> {
        TRIG0_DONE1_W::new(self)
    }
    #[doc = "Bit 17 - TRIG1 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig1_done1(&mut self) -> TRIG1_DONE1_W<17> {
        TRIG1_DONE1_W::new(self)
    }
    #[doc = "Bit 18 - TRIG2 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig2_done1(&mut self) -> TRIG2_DONE1_W<18> {
        TRIG2_DONE1_W::new(self)
    }
    #[doc = "Bit 19 - TRIG3 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig3_done1(&mut self) -> TRIG3_DONE1_W<19> {
        TRIG3_DONE1_W::new(self)
    }
    #[doc = "Bit 20 - TRIG4 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig4_done1(&mut self) -> TRIG4_DONE1_W<20> {
        TRIG4_DONE1_W::new(self)
    }
    #[doc = "Bit 21 - TRIG5 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig5_done1(&mut self) -> TRIG5_DONE1_W<21> {
        TRIG5_DONE1_W::new(self)
    }
    #[doc = "Bit 22 - TRIG6 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig6_done1(&mut self) -> TRIG6_DONE1_W<22> {
        TRIG6_DONE1_W::new(self)
    }
    #[doc = "Bit 23 - TRIG7 done1 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig7_done1(&mut self) -> TRIG7_DONE1_W<23> {
        TRIG7_DONE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done0_1_irq](index.html) module"]
pub struct DONE0_1_IRQ_SPEC;
impl crate::RegisterSpec for DONE0_1_IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [done0_1_irq::R](R) reader structure"]
impl crate::Readable for DONE0_1_IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [done0_1_irq::W](W) writer structure"]
impl crate::Writable for DONE0_1_IRQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x00ff_00ff;
}
#[doc = "`reset()` method sets DONE0_1_IRQ to value 0"]
impl crate::Resettable for DONE0_1_IRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
