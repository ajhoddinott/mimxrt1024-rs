#[doc = "Register `GPR6` reader"]
pub struct R(crate::R<GPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR6` writer"]
pub struct W(crate::W<GPR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR6_SPEC>;
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
impl From<crate::W<GPR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QTIMER1_TRM0_INPUT_SEL` reader - QTIMER1 TMR0 input select"]
pub type QTIMER1_TRM0_INPUT_SEL_R = crate::BitReader<QTIMER1_TRM0_INPUT_SEL_A>;
#[doc = "QTIMER1 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER1_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM0_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM0_INPUT_SEL_1 = 1,
}
impl From<QTIMER1_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM0_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER1_TRM0_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0,
            true => QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER1_TRM0_INPUT_SEL` writer - QTIMER1 TMR0 input select"]
pub type QTIMER1_TRM0_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER1_TRM0_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER1_TRM0_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM0_INPUT_SEL_A::QTIMER1_TRM0_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER1_TRM1_INPUT_SEL` reader - QTIMER1 TMR1 input select"]
pub type QTIMER1_TRM1_INPUT_SEL_R = crate::BitReader<QTIMER1_TRM1_INPUT_SEL_A>;
#[doc = "QTIMER1 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER1_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM1_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM1_INPUT_SEL_1 = 1,
}
impl From<QTIMER1_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM1_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER1_TRM1_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0,
            true => QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER1_TRM1_INPUT_SEL` writer - QTIMER1 TMR1 input select"]
pub type QTIMER1_TRM1_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER1_TRM1_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER1_TRM1_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM1_INPUT_SEL_A::QTIMER1_TRM1_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER1_TRM2_INPUT_SEL` reader - QTIMER1 TMR2 input select"]
pub type QTIMER1_TRM2_INPUT_SEL_R = crate::BitReader<QTIMER1_TRM2_INPUT_SEL_A>;
#[doc = "QTIMER1 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER1_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM2_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM2_INPUT_SEL_1 = 1,
}
impl From<QTIMER1_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM2_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER1_TRM2_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0,
            true => QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER1_TRM2_INPUT_SEL` writer - QTIMER1 TMR2 input select"]
pub type QTIMER1_TRM2_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER1_TRM2_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER1_TRM2_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM2_INPUT_SEL_A::QTIMER1_TRM2_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER1_TRM3_INPUT_SEL` reader - QTIMER1 TMR3 input select"]
pub type QTIMER1_TRM3_INPUT_SEL_R = crate::BitReader<QTIMER1_TRM3_INPUT_SEL_A>;
#[doc = "QTIMER1 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER1_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER1_TRM3_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER1_TRM3_INPUT_SEL_1 = 1,
}
impl From<QTIMER1_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER1_TRM3_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER1_TRM3_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER1_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0,
            true => QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer1_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER1_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer1_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER1_TRM3_INPUT_SEL` writer - QTIMER1 TMR3 input select"]
pub type QTIMER1_TRM3_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER1_TRM3_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER1_TRM3_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER1_TRM3_INPUT_SEL_A::QTIMER1_TRM3_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER2_TRM0_INPUT_SEL` reader - QTIMER2 TMR0 input select"]
pub type QTIMER2_TRM0_INPUT_SEL_R = crate::BitReader<QTIMER2_TRM0_INPUT_SEL_A>;
#[doc = "QTIMER2 TMR0 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER2_TRM0_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM0_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM0_INPUT_SEL_1 = 1,
}
impl From<QTIMER2_TRM0_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM0_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER2_TRM0_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM0_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0,
            true => QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm0_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM0_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm0_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER2_TRM0_INPUT_SEL` writer - QTIMER2 TMR0 input select"]
pub type QTIMER2_TRM0_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER2_TRM0_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER2_TRM0_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM0_INPUT_SEL_A::QTIMER2_TRM0_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER2_TRM1_INPUT_SEL` reader - QTIMER2 TMR1 input select"]
pub type QTIMER2_TRM1_INPUT_SEL_R = crate::BitReader<QTIMER2_TRM1_INPUT_SEL_A>;
#[doc = "QTIMER2 TMR1 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER2_TRM1_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM1_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM1_INPUT_SEL_1 = 1,
}
impl From<QTIMER2_TRM1_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM1_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER2_TRM1_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM1_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0,
            true => QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm1_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM1_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm1_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER2_TRM1_INPUT_SEL` writer - QTIMER2 TMR1 input select"]
pub type QTIMER2_TRM1_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER2_TRM1_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER2_TRM1_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM1_INPUT_SEL_A::QTIMER2_TRM1_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER2_TRM2_INPUT_SEL` reader - QTIMER2 TMR2 input select"]
pub type QTIMER2_TRM2_INPUT_SEL_R = crate::BitReader<QTIMER2_TRM2_INPUT_SEL_A>;
#[doc = "QTIMER2 TMR2 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER2_TRM2_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM2_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM2_INPUT_SEL_1 = 1,
}
impl From<QTIMER2_TRM2_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM2_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER2_TRM2_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM2_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0,
            true => QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm2_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM2_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm2_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER2_TRM2_INPUT_SEL` writer - QTIMER2 TMR2 input select"]
pub type QTIMER2_TRM2_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER2_TRM2_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER2_TRM2_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM2_INPUT_SEL_A::QTIMER2_TRM2_INPUT_SEL_1)
    }
}
#[doc = "Field `QTIMER2_TRM3_INPUT_SEL` reader - QTIMER2 TMR3 input select"]
pub type QTIMER2_TRM3_INPUT_SEL_R = crate::BitReader<QTIMER2_TRM3_INPUT_SEL_A>;
#[doc = "QTIMER2 TMR3 input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QTIMER2_TRM3_INPUT_SEL_A {
    #[doc = "0: input from IOMUX"]
    QTIMER2_TRM3_INPUT_SEL_0 = 0,
    #[doc = "1: input from XBAR"]
    QTIMER2_TRM3_INPUT_SEL_1 = 1,
}
impl From<QTIMER2_TRM3_INPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QTIMER2_TRM3_INPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl QTIMER2_TRM3_INPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QTIMER2_TRM3_INPUT_SEL_A {
        match self.bits {
            false => QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0,
            true => QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_0`"]
    #[inline(always)]
    pub fn is_qtimer2_trm3_input_sel_0(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `QTIMER2_TRM3_INPUT_SEL_1`"]
    #[inline(always)]
    pub fn is_qtimer2_trm3_input_sel_1(&self) -> bool {
        *self == QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1
    }
}
#[doc = "Field `QTIMER2_TRM3_INPUT_SEL` writer - QTIMER2 TMR3 input select"]
pub type QTIMER2_TRM3_INPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, QTIMER2_TRM3_INPUT_SEL_A, O>;
impl<'a, const O: u8> QTIMER2_TRM3_INPUT_SEL_W<'a, O> {
    #[doc = "input from IOMUX"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel_0(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_0)
    }
    #[doc = "input from XBAR"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel_1(self) -> &'a mut W {
        self.variant(QTIMER2_TRM3_INPUT_SEL_A::QTIMER2_TRM3_INPUT_SEL_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_4` reader - IOMUXC XBAR_INOUT4 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_4_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_4_A>;
#[doc = "IOMUXC XBAR_INOUT4 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_4_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_4_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_4_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_4_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_4_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_4_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0,
            true => IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_4_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_4_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_4_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_4` writer - IOMUXC XBAR_INOUT4 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_4_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_4_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_4_A::IOMUXC_XBAR_DIR_SEL_4_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_5` reader - IOMUXC XBAR_INOUT5 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_5_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_5_A>;
#[doc = "IOMUXC XBAR_INOUT5 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_5_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_5_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_5_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_5_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_5_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_5_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0,
            true => IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_5_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_5_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_5_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_5` writer - IOMUXC XBAR_INOUT5 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_5_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_5_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_5_A::IOMUXC_XBAR_DIR_SEL_5_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_6` reader - IOMUXC XBAR_INOUT6 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_6_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_6_A>;
#[doc = "IOMUXC XBAR_INOUT6 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_6_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_6_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_6_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_6_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_6_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_6_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0,
            true => IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_6_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_6_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_6_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_6` writer - IOMUXC XBAR_INOUT6 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_6_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_6_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_6_A::IOMUXC_XBAR_DIR_SEL_6_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_7` reader - IOMUXC XBAR_INOUT7 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_7_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_7_A>;
#[doc = "IOMUXC XBAR_INOUT7 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_7_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_7_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_7_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_7_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_7_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_7_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0,
            true => IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_7_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_7_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_7_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_7` writer - IOMUXC XBAR_INOUT7 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_7_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_7_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_7_A::IOMUXC_XBAR_DIR_SEL_7_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_8` reader - IOMUXC XBAR_INOUT8 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_8_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_8_A>;
#[doc = "IOMUXC XBAR_INOUT8 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_8_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_8_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_8_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_8_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_8_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_8_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0,
            true => IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_8_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_8_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_8_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_8` writer - IOMUXC XBAR_INOUT8 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_8_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_8_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_8_A::IOMUXC_XBAR_DIR_SEL_8_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_9` reader - IOMUXC XBAR_INOUT9 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_9_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_9_A>;
#[doc = "IOMUXC XBAR_INOUT9 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_9_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_9_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_9_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_9_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_9_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_9_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0,
            true => IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_9_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_9_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_9_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_9` writer - IOMUXC XBAR_INOUT9 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_9_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_9_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_9_A::IOMUXC_XBAR_DIR_SEL_9_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_10` reader - IOMUXC XBAR_INOUT10 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_10_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_10_A>;
#[doc = "IOMUXC XBAR_INOUT10 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_10_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_10_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_10_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_10_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_10_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_10_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0,
            true => IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_10_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_10_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_10_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_10` writer - IOMUXC XBAR_INOUT10 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_10_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_10_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_10_A::IOMUXC_XBAR_DIR_SEL_10_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_11` reader - IOMUXC XBAR_INOUT11 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_11_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_11_A>;
#[doc = "IOMUXC XBAR_INOUT11 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_11_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_11_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_11_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_11_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_11_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_11_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0,
            true => IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_11_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_11_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_11_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_11` writer - IOMUXC XBAR_INOUT11 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_11_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_11_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_11_A::IOMUXC_XBAR_DIR_SEL_11_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_12` reader - IOMUXC XBAR_INOUT12 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_12_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_12_A>;
#[doc = "IOMUXC XBAR_INOUT12 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_12_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_12_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_12_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_12_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_12_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_12_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0,
            true => IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_12_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_12_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_12_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_12` writer - IOMUXC XBAR_INOUT12 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_12_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_12_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_12_A::IOMUXC_XBAR_DIR_SEL_12_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_13` reader - IOMUXC XBAR_INOUT13 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_13_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_13_A>;
#[doc = "IOMUXC XBAR_INOUT13 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_13_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_13_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_13_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_13_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_13_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_13_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0,
            true => IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_13_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_13_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_13_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_13` writer - IOMUXC XBAR_INOUT13 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_13_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_13_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_13_A::IOMUXC_XBAR_DIR_SEL_13_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_14` reader - IOMUXC XBAR_INOUT14 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_14_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_14_A>;
#[doc = "IOMUXC XBAR_INOUT14 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_14_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_14_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_14_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_14_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_14_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_14_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0,
            true => IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_14_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_14_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_14_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_14` writer - IOMUXC XBAR_INOUT14 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_14_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_14_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_14_A::IOMUXC_XBAR_DIR_SEL_14_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_15` reader - IOMUXC XBAR_INOUT15 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_15_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_15_A>;
#[doc = "IOMUXC XBAR_INOUT15 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_15_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_15_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_15_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_15_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_15_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_15_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0,
            true => IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_15_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_15_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_15_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_15` writer - IOMUXC XBAR_INOUT15 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_15_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_15_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_15_A::IOMUXC_XBAR_DIR_SEL_15_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_16` reader - IOMUXC XBAR_INOUT16 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_16_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_16_A>;
#[doc = "IOMUXC XBAR_INOUT16 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_16_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_16_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_16_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_16_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_16_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_16_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0,
            true => IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_16_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_16_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_16_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_16` writer - IOMUXC XBAR_INOUT16 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_16_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_16_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_16_A::IOMUXC_XBAR_DIR_SEL_16_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_17` reader - IOMUXC XBAR_INOUT17 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_17_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_17_A>;
#[doc = "IOMUXC XBAR_INOUT17 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_17_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_17_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_17_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_17_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_17_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_17_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0,
            true => IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_17_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_17_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_17_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_17` writer - IOMUXC XBAR_INOUT17 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_17_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_17_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_17_A::IOMUXC_XBAR_DIR_SEL_17_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_18` reader - IOMUXC XBAR_INOUT18 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_18_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_18_A>;
#[doc = "IOMUXC XBAR_INOUT18 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_18_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_18_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_18_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_18_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_18_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_18_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0,
            true => IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_18_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_18_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_18_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_18` writer - IOMUXC XBAR_INOUT18 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_18_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_18_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_18_A::IOMUXC_XBAR_DIR_SEL_18_1)
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_19` reader - IOMUXC XBAR_INOUT19 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_19_R = crate::BitReader<IOMUXC_XBAR_DIR_SEL_19_A>;
#[doc = "IOMUXC XBAR_INOUT19 function direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOMUXC_XBAR_DIR_SEL_19_A {
    #[doc = "0: XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_19_0 = 0,
    #[doc = "1: XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_19_1 = 1,
}
impl From<IOMUXC_XBAR_DIR_SEL_19_A> for bool {
    #[inline(always)]
    fn from(variant: IOMUXC_XBAR_DIR_SEL_19_A) -> Self {
        variant as u8 != 0
    }
}
impl IOMUXC_XBAR_DIR_SEL_19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOMUXC_XBAR_DIR_SEL_19_A {
        match self.bits {
            false => IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0,
            true => IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_0`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_19_0(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0
    }
    #[doc = "Checks if the value of the field is `IOMUXC_XBAR_DIR_SEL_19_1`"]
    #[inline(always)]
    pub fn is_iomuxc_xbar_dir_sel_19_1(&self) -> bool {
        *self == IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1
    }
}
#[doc = "Field `IOMUXC_XBAR_DIR_SEL_19` writer - IOMUXC XBAR_INOUT19 function direction select"]
pub type IOMUXC_XBAR_DIR_SEL_19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR6_SPEC, IOMUXC_XBAR_DIR_SEL_19_A, O>;
impl<'a, const O: u8> IOMUXC_XBAR_DIR_SEL_19_W<'a, O> {
    #[doc = "XBAR_INOUT as input"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19_0(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_0)
    }
    #[doc = "XBAR_INOUT as output"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19_1(self) -> &'a mut W {
        self.variant(IOMUXC_XBAR_DIR_SEL_19_A::IOMUXC_XBAR_DIR_SEL_19_1)
    }
}
impl R {
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer1_trm0_input_sel(&self) -> QTIMER1_TRM0_INPUT_SEL_R {
        QTIMER1_TRM0_INPUT_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer1_trm1_input_sel(&self) -> QTIMER1_TRM1_INPUT_SEL_R {
        QTIMER1_TRM1_INPUT_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer1_trm2_input_sel(&self) -> QTIMER1_TRM2_INPUT_SEL_R {
        QTIMER1_TRM2_INPUT_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer1_trm3_input_sel(&self) -> QTIMER1_TRM3_INPUT_SEL_R {
        QTIMER1_TRM3_INPUT_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline(always)]
    pub fn qtimer2_trm0_input_sel(&self) -> QTIMER2_TRM0_INPUT_SEL_R {
        QTIMER2_TRM0_INPUT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline(always)]
    pub fn qtimer2_trm1_input_sel(&self) -> QTIMER2_TRM1_INPUT_SEL_R {
        QTIMER2_TRM1_INPUT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline(always)]
    pub fn qtimer2_trm2_input_sel(&self) -> QTIMER2_TRM2_INPUT_SEL_R {
        QTIMER2_TRM2_INPUT_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline(always)]
    pub fn qtimer2_trm3_input_sel(&self) -> QTIMER2_TRM3_INPUT_SEL_R {
        QTIMER2_TRM3_INPUT_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_4(&self) -> IOMUXC_XBAR_DIR_SEL_4_R {
        IOMUXC_XBAR_DIR_SEL_4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_5(&self) -> IOMUXC_XBAR_DIR_SEL_5_R {
        IOMUXC_XBAR_DIR_SEL_5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_6(&self) -> IOMUXC_XBAR_DIR_SEL_6_R {
        IOMUXC_XBAR_DIR_SEL_6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_7(&self) -> IOMUXC_XBAR_DIR_SEL_7_R {
        IOMUXC_XBAR_DIR_SEL_7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_8(&self) -> IOMUXC_XBAR_DIR_SEL_8_R {
        IOMUXC_XBAR_DIR_SEL_8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_9(&self) -> IOMUXC_XBAR_DIR_SEL_9_R {
        IOMUXC_XBAR_DIR_SEL_9_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_10(&self) -> IOMUXC_XBAR_DIR_SEL_10_R {
        IOMUXC_XBAR_DIR_SEL_10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_11(&self) -> IOMUXC_XBAR_DIR_SEL_11_R {
        IOMUXC_XBAR_DIR_SEL_11_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_12(&self) -> IOMUXC_XBAR_DIR_SEL_12_R {
        IOMUXC_XBAR_DIR_SEL_12_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_13(&self) -> IOMUXC_XBAR_DIR_SEL_13_R {
        IOMUXC_XBAR_DIR_SEL_13_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_14(&self) -> IOMUXC_XBAR_DIR_SEL_14_R {
        IOMUXC_XBAR_DIR_SEL_14_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_15(&self) -> IOMUXC_XBAR_DIR_SEL_15_R {
        IOMUXC_XBAR_DIR_SEL_15_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_16(&self) -> IOMUXC_XBAR_DIR_SEL_16_R {
        IOMUXC_XBAR_DIR_SEL_16_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_17(&self) -> IOMUXC_XBAR_DIR_SEL_17_R {
        IOMUXC_XBAR_DIR_SEL_17_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_18(&self) -> IOMUXC_XBAR_DIR_SEL_18_R {
        IOMUXC_XBAR_DIR_SEL_18_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    pub fn iomuxc_xbar_dir_sel_19(&self) -> IOMUXC_XBAR_DIR_SEL_19_R {
        IOMUXC_XBAR_DIR_SEL_19_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QTIMER1 TMR0 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer1_trm0_input_sel(&mut self) -> QTIMER1_TRM0_INPUT_SEL_W<0> {
        QTIMER1_TRM0_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 1 - QTIMER1 TMR1 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer1_trm1_input_sel(&mut self) -> QTIMER1_TRM1_INPUT_SEL_W<1> {
        QTIMER1_TRM1_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 2 - QTIMER1 TMR2 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer1_trm2_input_sel(&mut self) -> QTIMER1_TRM2_INPUT_SEL_W<2> {
        QTIMER1_TRM2_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 3 - QTIMER1 TMR3 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer1_trm3_input_sel(&mut self) -> QTIMER1_TRM3_INPUT_SEL_W<3> {
        QTIMER1_TRM3_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 4 - QTIMER2 TMR0 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer2_trm0_input_sel(&mut self) -> QTIMER2_TRM0_INPUT_SEL_W<4> {
        QTIMER2_TRM0_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 5 - QTIMER2 TMR1 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer2_trm1_input_sel(&mut self) -> QTIMER2_TRM1_INPUT_SEL_W<5> {
        QTIMER2_TRM1_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 6 - QTIMER2 TMR2 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer2_trm2_input_sel(&mut self) -> QTIMER2_TRM2_INPUT_SEL_W<6> {
        QTIMER2_TRM2_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 7 - QTIMER2 TMR3 input select"]
    #[inline(always)]
    #[must_use]
    pub fn qtimer2_trm3_input_sel(&mut self) -> QTIMER2_TRM3_INPUT_SEL_W<7> {
        QTIMER2_TRM3_INPUT_SEL_W::new(self)
    }
    #[doc = "Bit 16 - IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_4(&mut self) -> IOMUXC_XBAR_DIR_SEL_4_W<16> {
        IOMUXC_XBAR_DIR_SEL_4_W::new(self)
    }
    #[doc = "Bit 17 - IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_5(&mut self) -> IOMUXC_XBAR_DIR_SEL_5_W<17> {
        IOMUXC_XBAR_DIR_SEL_5_W::new(self)
    }
    #[doc = "Bit 18 - IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_6(&mut self) -> IOMUXC_XBAR_DIR_SEL_6_W<18> {
        IOMUXC_XBAR_DIR_SEL_6_W::new(self)
    }
    #[doc = "Bit 19 - IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_7(&mut self) -> IOMUXC_XBAR_DIR_SEL_7_W<19> {
        IOMUXC_XBAR_DIR_SEL_7_W::new(self)
    }
    #[doc = "Bit 20 - IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_8(&mut self) -> IOMUXC_XBAR_DIR_SEL_8_W<20> {
        IOMUXC_XBAR_DIR_SEL_8_W::new(self)
    }
    #[doc = "Bit 21 - IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_9(&mut self) -> IOMUXC_XBAR_DIR_SEL_9_W<21> {
        IOMUXC_XBAR_DIR_SEL_9_W::new(self)
    }
    #[doc = "Bit 22 - IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_10(&mut self) -> IOMUXC_XBAR_DIR_SEL_10_W<22> {
        IOMUXC_XBAR_DIR_SEL_10_W::new(self)
    }
    #[doc = "Bit 23 - IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_11(&mut self) -> IOMUXC_XBAR_DIR_SEL_11_W<23> {
        IOMUXC_XBAR_DIR_SEL_11_W::new(self)
    }
    #[doc = "Bit 24 - IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_12(&mut self) -> IOMUXC_XBAR_DIR_SEL_12_W<24> {
        IOMUXC_XBAR_DIR_SEL_12_W::new(self)
    }
    #[doc = "Bit 25 - IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_13(&mut self) -> IOMUXC_XBAR_DIR_SEL_13_W<25> {
        IOMUXC_XBAR_DIR_SEL_13_W::new(self)
    }
    #[doc = "Bit 26 - IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_14(&mut self) -> IOMUXC_XBAR_DIR_SEL_14_W<26> {
        IOMUXC_XBAR_DIR_SEL_14_W::new(self)
    }
    #[doc = "Bit 27 - IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_15(&mut self) -> IOMUXC_XBAR_DIR_SEL_15_W<27> {
        IOMUXC_XBAR_DIR_SEL_15_W::new(self)
    }
    #[doc = "Bit 28 - IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_16(&mut self) -> IOMUXC_XBAR_DIR_SEL_16_W<28> {
        IOMUXC_XBAR_DIR_SEL_16_W::new(self)
    }
    #[doc = "Bit 29 - IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_17(&mut self) -> IOMUXC_XBAR_DIR_SEL_17_W<29> {
        IOMUXC_XBAR_DIR_SEL_17_W::new(self)
    }
    #[doc = "Bit 30 - IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_18(&mut self) -> IOMUXC_XBAR_DIR_SEL_18_W<30> {
        IOMUXC_XBAR_DIR_SEL_18_W::new(self)
    }
    #[doc = "Bit 31 - IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    #[must_use]
    pub fn iomuxc_xbar_dir_sel_19(&mut self) -> IOMUXC_XBAR_DIR_SEL_19_W<31> {
        IOMUXC_XBAR_DIR_SEL_19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR6 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr6](index.html) module"]
pub struct GPR6_SPEC;
impl crate::RegisterSpec for GPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr6::R](R) reader structure"]
impl crate::Readable for GPR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr6::W](W) writer structure"]
impl crate::Writable for GPR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR6 to value 0"]
impl crate::Resettable for GPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
