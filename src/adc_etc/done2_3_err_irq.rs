#[doc = "Register `DONE2_3_ERR_IRQ` reader"]
pub struct R(crate::R<DONE2_3_ERR_IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DONE2_3_ERR_IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DONE2_3_ERR_IRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DONE2_3_ERR_IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DONE2_3_ERR_IRQ` writer"]
pub struct W(crate::W<DONE2_3_ERR_IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DONE2_3_ERR_IRQ_SPEC>;
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
impl From<crate::W<DONE2_3_ERR_IRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DONE2_3_ERR_IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG0_DONE2` reader - TRIG0 done2 interrupt detection."]
pub type TRIG0_DONE2_R = crate::BitReader<TRIG0_DONE2_A>;
#[doc = "TRIG0 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_DONE2_A {
    #[doc = "0: No TRIG0_DONE2 interrupt detected"]
    TRIG0_DONE2_0 = 0,
    #[doc = "1: TRIG0_DONE2 interrupt detected"]
    TRIG0_DONE2_1 = 1,
}
impl From<TRIG0_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_DONE2_A {
        match self.bits {
            false => TRIG0_DONE2_A::TRIG0_DONE2_0,
            true => TRIG0_DONE2_A::TRIG0_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig0_done2_0(&self) -> bool {
        *self == TRIG0_DONE2_A::TRIG0_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig0_done2_1(&self) -> bool {
        *self == TRIG0_DONE2_A::TRIG0_DONE2_1
    }
}
#[doc = "Field `TRIG0_DONE2` writer - TRIG0 done2 interrupt detection."]
pub type TRIG0_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG0_DONE2_A, O>;
impl<'a, const O: u8> TRIG0_DONE2_W<'a, O> {
    #[doc = "No TRIG0_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done2_0(self) -> &'a mut W {
        self.variant(TRIG0_DONE2_A::TRIG0_DONE2_0)
    }
    #[doc = "TRIG0_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig0_done2_1(self) -> &'a mut W {
        self.variant(TRIG0_DONE2_A::TRIG0_DONE2_1)
    }
}
#[doc = "Field `TRIG1_DONE2` reader - TRIG1 done2 interrupt detection."]
pub type TRIG1_DONE2_R = crate::BitReader<TRIG1_DONE2_A>;
#[doc = "TRIG1 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_DONE2_A {
    #[doc = "0: No TRIG1_DONE2 interrupt detected"]
    TRIG1_DONE2_0 = 0,
    #[doc = "1: TRIG1_DONE2 interrupt detected"]
    TRIG1_DONE2_1 = 1,
}
impl From<TRIG1_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_DONE2_A {
        match self.bits {
            false => TRIG1_DONE2_A::TRIG1_DONE2_0,
            true => TRIG1_DONE2_A::TRIG1_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig1_done2_0(&self) -> bool {
        *self == TRIG1_DONE2_A::TRIG1_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig1_done2_1(&self) -> bool {
        *self == TRIG1_DONE2_A::TRIG1_DONE2_1
    }
}
#[doc = "Field `TRIG1_DONE2` writer - TRIG1 done2 interrupt detection."]
pub type TRIG1_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG1_DONE2_A, O>;
impl<'a, const O: u8> TRIG1_DONE2_W<'a, O> {
    #[doc = "No TRIG1_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done2_0(self) -> &'a mut W {
        self.variant(TRIG1_DONE2_A::TRIG1_DONE2_0)
    }
    #[doc = "TRIG1_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig1_done2_1(self) -> &'a mut W {
        self.variant(TRIG1_DONE2_A::TRIG1_DONE2_1)
    }
}
#[doc = "Field `TRIG2_DONE2` reader - TRIG2 done2 interrupt detection."]
pub type TRIG2_DONE2_R = crate::BitReader<TRIG2_DONE2_A>;
#[doc = "TRIG2 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_DONE2_A {
    #[doc = "0: No TRIG2_DONE2 interrupt detected"]
    TRIG2_DONE2_0 = 0,
    #[doc = "1: TRIG2_DONE2 interrupt detected"]
    TRIG2_DONE2_1 = 1,
}
impl From<TRIG2_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_DONE2_A {
        match self.bits {
            false => TRIG2_DONE2_A::TRIG2_DONE2_0,
            true => TRIG2_DONE2_A::TRIG2_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig2_done2_0(&self) -> bool {
        *self == TRIG2_DONE2_A::TRIG2_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig2_done2_1(&self) -> bool {
        *self == TRIG2_DONE2_A::TRIG2_DONE2_1
    }
}
#[doc = "Field `TRIG2_DONE2` writer - TRIG2 done2 interrupt detection."]
pub type TRIG2_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG2_DONE2_A, O>;
impl<'a, const O: u8> TRIG2_DONE2_W<'a, O> {
    #[doc = "No TRIG2_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done2_0(self) -> &'a mut W {
        self.variant(TRIG2_DONE2_A::TRIG2_DONE2_0)
    }
    #[doc = "TRIG2_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig2_done2_1(self) -> &'a mut W {
        self.variant(TRIG2_DONE2_A::TRIG2_DONE2_1)
    }
}
#[doc = "Field `TRIG3_DONE2` reader - TRIG3 done2 interrupt detection."]
pub type TRIG3_DONE2_R = crate::BitReader<TRIG3_DONE2_A>;
#[doc = "TRIG3 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_DONE2_A {
    #[doc = "0: No TRIG3_DONE2 interrupt detected"]
    TRIG3_DONE2_0 = 0,
    #[doc = "1: TRIG3_DONE2 interrupt detected"]
    TRIG3_DONE2_1 = 1,
}
impl From<TRIG3_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_DONE2_A {
        match self.bits {
            false => TRIG3_DONE2_A::TRIG3_DONE2_0,
            true => TRIG3_DONE2_A::TRIG3_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig3_done2_0(&self) -> bool {
        *self == TRIG3_DONE2_A::TRIG3_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig3_done2_1(&self) -> bool {
        *self == TRIG3_DONE2_A::TRIG3_DONE2_1
    }
}
#[doc = "Field `TRIG3_DONE2` writer - TRIG3 done2 interrupt detection."]
pub type TRIG3_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG3_DONE2_A, O>;
impl<'a, const O: u8> TRIG3_DONE2_W<'a, O> {
    #[doc = "No TRIG3_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done2_0(self) -> &'a mut W {
        self.variant(TRIG3_DONE2_A::TRIG3_DONE2_0)
    }
    #[doc = "TRIG3_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig3_done2_1(self) -> &'a mut W {
        self.variant(TRIG3_DONE2_A::TRIG3_DONE2_1)
    }
}
#[doc = "Field `TRIG4_DONE2` reader - TRIG4 done2 interrupt detection."]
pub type TRIG4_DONE2_R = crate::BitReader<TRIG4_DONE2_A>;
#[doc = "TRIG4 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_DONE2_A {
    #[doc = "0: No TRIG4_DONE2 interrupt detected"]
    TRIG4_DONE2_0 = 0,
    #[doc = "1: TRIG4_DONE2 interrupt detected"]
    TRIG4_DONE2_1 = 1,
}
impl From<TRIG4_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_DONE2_A {
        match self.bits {
            false => TRIG4_DONE2_A::TRIG4_DONE2_0,
            true => TRIG4_DONE2_A::TRIG4_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig4_done2_0(&self) -> bool {
        *self == TRIG4_DONE2_A::TRIG4_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig4_done2_1(&self) -> bool {
        *self == TRIG4_DONE2_A::TRIG4_DONE2_1
    }
}
#[doc = "Field `TRIG4_DONE2` writer - TRIG4 done2 interrupt detection."]
pub type TRIG4_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG4_DONE2_A, O>;
impl<'a, const O: u8> TRIG4_DONE2_W<'a, O> {
    #[doc = "No TRIG4_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done2_0(self) -> &'a mut W {
        self.variant(TRIG4_DONE2_A::TRIG4_DONE2_0)
    }
    #[doc = "TRIG4_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig4_done2_1(self) -> &'a mut W {
        self.variant(TRIG4_DONE2_A::TRIG4_DONE2_1)
    }
}
#[doc = "Field `TRIG5_DONE2` reader - TRIG5 done2 interrupt detection."]
pub type TRIG5_DONE2_R = crate::BitReader<TRIG5_DONE2_A>;
#[doc = "TRIG5 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_DONE2_A {
    #[doc = "0: No TRIG5_DONE2 interrupt detected"]
    TRIG5_DONE2_0 = 0,
    #[doc = "1: TRIG5_DONE2 interrupt detected"]
    TRIG5_DONE2_1 = 1,
}
impl From<TRIG5_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_DONE2_A {
        match self.bits {
            false => TRIG5_DONE2_A::TRIG5_DONE2_0,
            true => TRIG5_DONE2_A::TRIG5_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig5_done2_0(&self) -> bool {
        *self == TRIG5_DONE2_A::TRIG5_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig5_done2_1(&self) -> bool {
        *self == TRIG5_DONE2_A::TRIG5_DONE2_1
    }
}
#[doc = "Field `TRIG5_DONE2` writer - TRIG5 done2 interrupt detection."]
pub type TRIG5_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG5_DONE2_A, O>;
impl<'a, const O: u8> TRIG5_DONE2_W<'a, O> {
    #[doc = "No TRIG5_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done2_0(self) -> &'a mut W {
        self.variant(TRIG5_DONE2_A::TRIG5_DONE2_0)
    }
    #[doc = "TRIG5_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig5_done2_1(self) -> &'a mut W {
        self.variant(TRIG5_DONE2_A::TRIG5_DONE2_1)
    }
}
#[doc = "Field `TRIG6_DONE2` reader - TRIG6 done2 interrupt detection."]
pub type TRIG6_DONE2_R = crate::BitReader<TRIG6_DONE2_A>;
#[doc = "TRIG6 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_DONE2_A {
    #[doc = "0: No TRIG6_DONE2 interrupt detected"]
    TRIG6_DONE2_0 = 0,
    #[doc = "1: TRIG6_DONE2 interrupt detected"]
    TRIG6_DONE2_1 = 1,
}
impl From<TRIG6_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_DONE2_A {
        match self.bits {
            false => TRIG6_DONE2_A::TRIG6_DONE2_0,
            true => TRIG6_DONE2_A::TRIG6_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig6_done2_0(&self) -> bool {
        *self == TRIG6_DONE2_A::TRIG6_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig6_done2_1(&self) -> bool {
        *self == TRIG6_DONE2_A::TRIG6_DONE2_1
    }
}
#[doc = "Field `TRIG6_DONE2` writer - TRIG6 done2 interrupt detection."]
pub type TRIG6_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG6_DONE2_A, O>;
impl<'a, const O: u8> TRIG6_DONE2_W<'a, O> {
    #[doc = "No TRIG6_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done2_0(self) -> &'a mut W {
        self.variant(TRIG6_DONE2_A::TRIG6_DONE2_0)
    }
    #[doc = "TRIG6_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig6_done2_1(self) -> &'a mut W {
        self.variant(TRIG6_DONE2_A::TRIG6_DONE2_1)
    }
}
#[doc = "Field `TRIG7_DONE2` reader - TRIG7 done2 interrupt detection."]
pub type TRIG7_DONE2_R = crate::BitReader<TRIG7_DONE2_A>;
#[doc = "TRIG7 done2 interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_DONE2_A {
    #[doc = "0: No TRIG7_DONE2 interrupt detected"]
    TRIG7_DONE2_0 = 0,
    #[doc = "1: TRIG7_DONE2 interrupt detected"]
    TRIG7_DONE2_1 = 1,
}
impl From<TRIG7_DONE2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_DONE2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_DONE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_DONE2_A {
        match self.bits {
            false => TRIG7_DONE2_A::TRIG7_DONE2_0,
            true => TRIG7_DONE2_A::TRIG7_DONE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE2_0`"]
    #[inline(always)]
    pub fn is_trig7_done2_0(&self) -> bool {
        *self == TRIG7_DONE2_A::TRIG7_DONE2_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_DONE2_1`"]
    #[inline(always)]
    pub fn is_trig7_done2_1(&self) -> bool {
        *self == TRIG7_DONE2_A::TRIG7_DONE2_1
    }
}
#[doc = "Field `TRIG7_DONE2` writer - TRIG7 done2 interrupt detection."]
pub type TRIG7_DONE2_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG7_DONE2_A, O>;
impl<'a, const O: u8> TRIG7_DONE2_W<'a, O> {
    #[doc = "No TRIG7_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done2_0(self) -> &'a mut W {
        self.variant(TRIG7_DONE2_A::TRIG7_DONE2_0)
    }
    #[doc = "TRIG7_DONE2 interrupt detected"]
    #[inline(always)]
    pub fn trig7_done2_1(self) -> &'a mut W {
        self.variant(TRIG7_DONE2_A::TRIG7_DONE2_1)
    }
}
#[doc = "Field `TRIG0_ERR` reader - TRIG0 error interrupt detection."]
pub type TRIG0_ERR_R = crate::BitReader<TRIG0_ERR_A>;
#[doc = "TRIG0 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_ERR_A {
    #[doc = "0: No TRIG0_ERR interrupt detected"]
    TRIG0_ERR_0 = 0,
    #[doc = "1: TRIG0_ERR interrupt detected"]
    TRIG0_ERR_1 = 1,
}
impl From<TRIG0_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_ERR_A {
        match self.bits {
            false => TRIG0_ERR_A::TRIG0_ERR_0,
            true => TRIG0_ERR_A::TRIG0_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_ERR_0`"]
    #[inline(always)]
    pub fn is_trig0_err_0(&self) -> bool {
        *self == TRIG0_ERR_A::TRIG0_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_ERR_1`"]
    #[inline(always)]
    pub fn is_trig0_err_1(&self) -> bool {
        *self == TRIG0_ERR_A::TRIG0_ERR_1
    }
}
#[doc = "Field `TRIG0_ERR` writer - TRIG0 error interrupt detection."]
pub type TRIG0_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG0_ERR_A, O>;
impl<'a, const O: u8> TRIG0_ERR_W<'a, O> {
    #[doc = "No TRIG0_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig0_err_0(self) -> &'a mut W {
        self.variant(TRIG0_ERR_A::TRIG0_ERR_0)
    }
    #[doc = "TRIG0_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig0_err_1(self) -> &'a mut W {
        self.variant(TRIG0_ERR_A::TRIG0_ERR_1)
    }
}
#[doc = "Field `TRIG1_ERR` reader - TRIG1 error interrupt detection."]
pub type TRIG1_ERR_R = crate::BitReader<TRIG1_ERR_A>;
#[doc = "TRIG1 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_ERR_A {
    #[doc = "0: No TRIG1_ERR interrupt detected"]
    TRIG1_ERR_0 = 0,
    #[doc = "1: TRIG1_ERR interrupt detected"]
    TRIG1_ERR_1 = 1,
}
impl From<TRIG1_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_ERR_A {
        match self.bits {
            false => TRIG1_ERR_A::TRIG1_ERR_0,
            true => TRIG1_ERR_A::TRIG1_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_ERR_0`"]
    #[inline(always)]
    pub fn is_trig1_err_0(&self) -> bool {
        *self == TRIG1_ERR_A::TRIG1_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_ERR_1`"]
    #[inline(always)]
    pub fn is_trig1_err_1(&self) -> bool {
        *self == TRIG1_ERR_A::TRIG1_ERR_1
    }
}
#[doc = "Field `TRIG1_ERR` writer - TRIG1 error interrupt detection."]
pub type TRIG1_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG1_ERR_A, O>;
impl<'a, const O: u8> TRIG1_ERR_W<'a, O> {
    #[doc = "No TRIG1_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig1_err_0(self) -> &'a mut W {
        self.variant(TRIG1_ERR_A::TRIG1_ERR_0)
    }
    #[doc = "TRIG1_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig1_err_1(self) -> &'a mut W {
        self.variant(TRIG1_ERR_A::TRIG1_ERR_1)
    }
}
#[doc = "Field `TRIG2_ERR` reader - TRIG2 error interrupt detection."]
pub type TRIG2_ERR_R = crate::BitReader<TRIG2_ERR_A>;
#[doc = "TRIG2 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_ERR_A {
    #[doc = "0: No TRIG2_ERR interrupt detected"]
    TRIG2_ERR_0 = 0,
    #[doc = "1: TRIG2_ERR interrupt detected"]
    TRIG2_ERR_1 = 1,
}
impl From<TRIG2_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_ERR_A {
        match self.bits {
            false => TRIG2_ERR_A::TRIG2_ERR_0,
            true => TRIG2_ERR_A::TRIG2_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_ERR_0`"]
    #[inline(always)]
    pub fn is_trig2_err_0(&self) -> bool {
        *self == TRIG2_ERR_A::TRIG2_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_ERR_1`"]
    #[inline(always)]
    pub fn is_trig2_err_1(&self) -> bool {
        *self == TRIG2_ERR_A::TRIG2_ERR_1
    }
}
#[doc = "Field `TRIG2_ERR` writer - TRIG2 error interrupt detection."]
pub type TRIG2_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG2_ERR_A, O>;
impl<'a, const O: u8> TRIG2_ERR_W<'a, O> {
    #[doc = "No TRIG2_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig2_err_0(self) -> &'a mut W {
        self.variant(TRIG2_ERR_A::TRIG2_ERR_0)
    }
    #[doc = "TRIG2_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig2_err_1(self) -> &'a mut W {
        self.variant(TRIG2_ERR_A::TRIG2_ERR_1)
    }
}
#[doc = "Field `TRIG3_ERR` reader - TRIG3 error interrupt detection."]
pub type TRIG3_ERR_R = crate::BitReader<TRIG3_ERR_A>;
#[doc = "TRIG3 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG3_ERR_A {
    #[doc = "0: No TRIG3_ERR interrupt detected"]
    TRIG3_ERR_0 = 0,
    #[doc = "1: TRIG3_ERR interrupt detected"]
    TRIG3_ERR_1 = 1,
}
impl From<TRIG3_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG3_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG3_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG3_ERR_A {
        match self.bits {
            false => TRIG3_ERR_A::TRIG3_ERR_0,
            true => TRIG3_ERR_A::TRIG3_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3_ERR_0`"]
    #[inline(always)]
    pub fn is_trig3_err_0(&self) -> bool {
        *self == TRIG3_ERR_A::TRIG3_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG3_ERR_1`"]
    #[inline(always)]
    pub fn is_trig3_err_1(&self) -> bool {
        *self == TRIG3_ERR_A::TRIG3_ERR_1
    }
}
#[doc = "Field `TRIG3_ERR` writer - TRIG3 error interrupt detection."]
pub type TRIG3_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG3_ERR_A, O>;
impl<'a, const O: u8> TRIG3_ERR_W<'a, O> {
    #[doc = "No TRIG3_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig3_err_0(self) -> &'a mut W {
        self.variant(TRIG3_ERR_A::TRIG3_ERR_0)
    }
    #[doc = "TRIG3_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig3_err_1(self) -> &'a mut W {
        self.variant(TRIG3_ERR_A::TRIG3_ERR_1)
    }
}
#[doc = "Field `TRIG4_ERR` reader - TRIG4 error interrupt detection."]
pub type TRIG4_ERR_R = crate::BitReader<TRIG4_ERR_A>;
#[doc = "TRIG4 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG4_ERR_A {
    #[doc = "0: No TRIG4_ERR interrupt detected"]
    TRIG4_ERR_0 = 0,
    #[doc = "1: TRIG4_ERR interrupt detected"]
    TRIG4_ERR_1 = 1,
}
impl From<TRIG4_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG4_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG4_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG4_ERR_A {
        match self.bits {
            false => TRIG4_ERR_A::TRIG4_ERR_0,
            true => TRIG4_ERR_A::TRIG4_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG4_ERR_0`"]
    #[inline(always)]
    pub fn is_trig4_err_0(&self) -> bool {
        *self == TRIG4_ERR_A::TRIG4_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG4_ERR_1`"]
    #[inline(always)]
    pub fn is_trig4_err_1(&self) -> bool {
        *self == TRIG4_ERR_A::TRIG4_ERR_1
    }
}
#[doc = "Field `TRIG4_ERR` writer - TRIG4 error interrupt detection."]
pub type TRIG4_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG4_ERR_A, O>;
impl<'a, const O: u8> TRIG4_ERR_W<'a, O> {
    #[doc = "No TRIG4_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig4_err_0(self) -> &'a mut W {
        self.variant(TRIG4_ERR_A::TRIG4_ERR_0)
    }
    #[doc = "TRIG4_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig4_err_1(self) -> &'a mut W {
        self.variant(TRIG4_ERR_A::TRIG4_ERR_1)
    }
}
#[doc = "Field `TRIG5_ERR` reader - TRIG5 error interrupt detection."]
pub type TRIG5_ERR_R = crate::BitReader<TRIG5_ERR_A>;
#[doc = "TRIG5 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG5_ERR_A {
    #[doc = "0: No TRIG5_ERR interrupt detected"]
    TRIG5_ERR_0 = 0,
    #[doc = "1: TRIG5_ERR interrupt detected"]
    TRIG5_ERR_1 = 1,
}
impl From<TRIG5_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG5_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG5_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG5_ERR_A {
        match self.bits {
            false => TRIG5_ERR_A::TRIG5_ERR_0,
            true => TRIG5_ERR_A::TRIG5_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG5_ERR_0`"]
    #[inline(always)]
    pub fn is_trig5_err_0(&self) -> bool {
        *self == TRIG5_ERR_A::TRIG5_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG5_ERR_1`"]
    #[inline(always)]
    pub fn is_trig5_err_1(&self) -> bool {
        *self == TRIG5_ERR_A::TRIG5_ERR_1
    }
}
#[doc = "Field `TRIG5_ERR` writer - TRIG5 error interrupt detection."]
pub type TRIG5_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG5_ERR_A, O>;
impl<'a, const O: u8> TRIG5_ERR_W<'a, O> {
    #[doc = "No TRIG5_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig5_err_0(self) -> &'a mut W {
        self.variant(TRIG5_ERR_A::TRIG5_ERR_0)
    }
    #[doc = "TRIG5_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig5_err_1(self) -> &'a mut W {
        self.variant(TRIG5_ERR_A::TRIG5_ERR_1)
    }
}
#[doc = "Field `TRIG6_ERR` reader - TRIG6 error interrupt detection."]
pub type TRIG6_ERR_R = crate::BitReader<TRIG6_ERR_A>;
#[doc = "TRIG6 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG6_ERR_A {
    #[doc = "0: No TRIG6_ERR interrupt detected"]
    TRIG6_ERR_0 = 0,
    #[doc = "1: TRIG6_ERR interrupt detected"]
    TRIG6_ERR_1 = 1,
}
impl From<TRIG6_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG6_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG6_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG6_ERR_A {
        match self.bits {
            false => TRIG6_ERR_A::TRIG6_ERR_0,
            true => TRIG6_ERR_A::TRIG6_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG6_ERR_0`"]
    #[inline(always)]
    pub fn is_trig6_err_0(&self) -> bool {
        *self == TRIG6_ERR_A::TRIG6_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG6_ERR_1`"]
    #[inline(always)]
    pub fn is_trig6_err_1(&self) -> bool {
        *self == TRIG6_ERR_A::TRIG6_ERR_1
    }
}
#[doc = "Field `TRIG6_ERR` writer - TRIG6 error interrupt detection."]
pub type TRIG6_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG6_ERR_A, O>;
impl<'a, const O: u8> TRIG6_ERR_W<'a, O> {
    #[doc = "No TRIG6_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig6_err_0(self) -> &'a mut W {
        self.variant(TRIG6_ERR_A::TRIG6_ERR_0)
    }
    #[doc = "TRIG6_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig6_err_1(self) -> &'a mut W {
        self.variant(TRIG6_ERR_A::TRIG6_ERR_1)
    }
}
#[doc = "Field `TRIG7_ERR` reader - TRIG7 error interrupt detection."]
pub type TRIG7_ERR_R = crate::BitReader<TRIG7_ERR_A>;
#[doc = "TRIG7 error interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG7_ERR_A {
    #[doc = "0: No TRIG7_ERR interrupt detected"]
    TRIG7_ERR_0 = 0,
    #[doc = "1: TRIG7_ERR interrupt detected"]
    TRIG7_ERR_1 = 1,
}
impl From<TRIG7_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG7_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG7_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG7_ERR_A {
        match self.bits {
            false => TRIG7_ERR_A::TRIG7_ERR_0,
            true => TRIG7_ERR_A::TRIG7_ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG7_ERR_0`"]
    #[inline(always)]
    pub fn is_trig7_err_0(&self) -> bool {
        *self == TRIG7_ERR_A::TRIG7_ERR_0
    }
    #[doc = "Checks if the value of the field is `TRIG7_ERR_1`"]
    #[inline(always)]
    pub fn is_trig7_err_1(&self) -> bool {
        *self == TRIG7_ERR_A::TRIG7_ERR_1
    }
}
#[doc = "Field `TRIG7_ERR` writer - TRIG7 error interrupt detection."]
pub type TRIG7_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DONE2_3_ERR_IRQ_SPEC, TRIG7_ERR_A, O>;
impl<'a, const O: u8> TRIG7_ERR_W<'a, O> {
    #[doc = "No TRIG7_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig7_err_0(self) -> &'a mut W {
        self.variant(TRIG7_ERR_A::TRIG7_ERR_0)
    }
    #[doc = "TRIG7_ERR interrupt detected"]
    #[inline(always)]
    pub fn trig7_err_1(self) -> &'a mut W {
        self.variant(TRIG7_ERR_A::TRIG7_ERR_1)
    }
}
impl R {
    #[doc = "Bit 0 - TRIG0 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig0_done2(&self) -> TRIG0_DONE2_R {
        TRIG0_DONE2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TRIG1 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig1_done2(&self) -> TRIG1_DONE2_R {
        TRIG1_DONE2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRIG2 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig2_done2(&self) -> TRIG2_DONE2_R {
        TRIG2_DONE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRIG3 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig3_done2(&self) -> TRIG3_DONE2_R {
        TRIG3_DONE2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TRIG4 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig4_done2(&self) -> TRIG4_DONE2_R {
        TRIG4_DONE2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRIG5 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig5_done2(&self) -> TRIG5_DONE2_R {
        TRIG5_DONE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TRIG6 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig6_done2(&self) -> TRIG6_DONE2_R {
        TRIG6_DONE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TRIG7 done2 interrupt detection."]
    #[inline(always)]
    pub fn trig7_done2(&self) -> TRIG7_DONE2_R {
        TRIG7_DONE2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TRIG0 error interrupt detection."]
    #[inline(always)]
    pub fn trig0_err(&self) -> TRIG0_ERR_R {
        TRIG0_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRIG1 error interrupt detection."]
    #[inline(always)]
    pub fn trig1_err(&self) -> TRIG1_ERR_R {
        TRIG1_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRIG2 error interrupt detection."]
    #[inline(always)]
    pub fn trig2_err(&self) -> TRIG2_ERR_R {
        TRIG2_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRIG3 error interrupt detection."]
    #[inline(always)]
    pub fn trig3_err(&self) -> TRIG3_ERR_R {
        TRIG3_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRIG4 error interrupt detection."]
    #[inline(always)]
    pub fn trig4_err(&self) -> TRIG4_ERR_R {
        TRIG4_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRIG5 error interrupt detection."]
    #[inline(always)]
    pub fn trig5_err(&self) -> TRIG5_ERR_R {
        TRIG5_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRIG6 error interrupt detection."]
    #[inline(always)]
    pub fn trig6_err(&self) -> TRIG6_ERR_R {
        TRIG6_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TRIG7 error interrupt detection."]
    #[inline(always)]
    pub fn trig7_err(&self) -> TRIG7_ERR_R {
        TRIG7_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig0_done2(&mut self) -> TRIG0_DONE2_W<0> {
        TRIG0_DONE2_W::new(self)
    }
    #[doc = "Bit 1 - TRIG1 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig1_done2(&mut self) -> TRIG1_DONE2_W<1> {
        TRIG1_DONE2_W::new(self)
    }
    #[doc = "Bit 2 - TRIG2 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig2_done2(&mut self) -> TRIG2_DONE2_W<2> {
        TRIG2_DONE2_W::new(self)
    }
    #[doc = "Bit 3 - TRIG3 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig3_done2(&mut self) -> TRIG3_DONE2_W<3> {
        TRIG3_DONE2_W::new(self)
    }
    #[doc = "Bit 4 - TRIG4 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig4_done2(&mut self) -> TRIG4_DONE2_W<4> {
        TRIG4_DONE2_W::new(self)
    }
    #[doc = "Bit 5 - TRIG5 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig5_done2(&mut self) -> TRIG5_DONE2_W<5> {
        TRIG5_DONE2_W::new(self)
    }
    #[doc = "Bit 6 - TRIG6 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig6_done2(&mut self) -> TRIG6_DONE2_W<6> {
        TRIG6_DONE2_W::new(self)
    }
    #[doc = "Bit 7 - TRIG7 done2 interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig7_done2(&mut self) -> TRIG7_DONE2_W<7> {
        TRIG7_DONE2_W::new(self)
    }
    #[doc = "Bit 16 - TRIG0 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig0_err(&mut self) -> TRIG0_ERR_W<16> {
        TRIG0_ERR_W::new(self)
    }
    #[doc = "Bit 17 - TRIG1 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig1_err(&mut self) -> TRIG1_ERR_W<17> {
        TRIG1_ERR_W::new(self)
    }
    #[doc = "Bit 18 - TRIG2 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig2_err(&mut self) -> TRIG2_ERR_W<18> {
        TRIG2_ERR_W::new(self)
    }
    #[doc = "Bit 19 - TRIG3 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig3_err(&mut self) -> TRIG3_ERR_W<19> {
        TRIG3_ERR_W::new(self)
    }
    #[doc = "Bit 20 - TRIG4 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig4_err(&mut self) -> TRIG4_ERR_W<20> {
        TRIG4_ERR_W::new(self)
    }
    #[doc = "Bit 21 - TRIG5 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig5_err(&mut self) -> TRIG5_ERR_W<21> {
        TRIG5_ERR_W::new(self)
    }
    #[doc = "Bit 22 - TRIG6 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig6_err(&mut self) -> TRIG6_ERR_W<22> {
        TRIG6_ERR_W::new(self)
    }
    #[doc = "Bit 23 - TRIG7 error interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn trig7_err(&mut self) -> TRIG7_ERR_W<23> {
        TRIG7_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done2_3_err_irq](index.html) module"]
pub struct DONE2_3_ERR_IRQ_SPEC;
impl crate::RegisterSpec for DONE2_3_ERR_IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [done2_3_err_irq::R](R) reader structure"]
impl crate::Readable for DONE2_3_ERR_IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [done2_3_err_irq::W](W) writer structure"]
impl crate::Writable for DONE2_3_ERR_IRQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets DONE2_3_ERR_IRQ to value 0"]
impl crate::Resettable for DONE2_3_ERR_IRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
