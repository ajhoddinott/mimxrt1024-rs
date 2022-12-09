#[doc = "Register `GPR8` reader"]
pub struct R(crate::R<GPR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR8` writer"]
pub struct W(crate::W<GPR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR8_SPEC>;
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
impl From<crate::W<GPR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPI2C1_IPG_STOP_MODE` reader - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C1_IPG_STOP_MODE_R = crate::BitReader<LPI2C1_IPG_STOP_MODE_A>;
#[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C1_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C1_IPG_STOP_MODE_1 = 1,
}
impl From<LPI2C1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C1_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0,
            true => LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPI2C1_IPG_STOP_MODE` writer - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C1_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C1_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPI2C1_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_STOP_MODE_A::LPI2C1_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPI2C1_IPG_DOZE` reader - LPI2C1 ipg_doze mode"]
pub type LPI2C1_IPG_DOZE_R = crate::BitReader<LPI2C1_IPG_DOZE_A>;
#[doc = "LPI2C1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C1_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPI2C1_IPG_DOZE_1 = 1,
}
impl From<LPI2C1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C1_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_IPG_DOZE_A {
        match self.bits {
            false => LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0,
            true => LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_doze_0(&self) -> bool {
        *self == LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_ipg_doze_1(&self) -> bool {
        *self == LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1
    }
}
#[doc = "Field `LPI2C1_IPG_DOZE` writer - LPI2C1 ipg_doze mode"]
pub type LPI2C1_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C1_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPI2C1_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C1_IPG_DOZE_A::LPI2C1_IPG_DOZE_1)
    }
}
#[doc = "Field `LPI2C2_IPG_STOP_MODE` reader - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C2_IPG_STOP_MODE_R = crate::BitReader<LPI2C2_IPG_STOP_MODE_A>;
#[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C2_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C2_IPG_STOP_MODE_1 = 1,
}
impl From<LPI2C2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C2_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0,
            true => LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPI2C2_IPG_STOP_MODE` writer - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C2_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C2_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPI2C2_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_STOP_MODE_A::LPI2C2_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPI2C2_IPG_DOZE` reader - LPI2C2 ipg_doze mode"]
pub type LPI2C2_IPG_DOZE_R = crate::BitReader<LPI2C2_IPG_DOZE_A>;
#[doc = "LPI2C2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C2_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPI2C2_IPG_DOZE_1 = 1,
}
impl From<LPI2C2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C2_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_IPG_DOZE_A {
        match self.bits {
            false => LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0,
            true => LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_doze_0(&self) -> bool {
        *self == LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_ipg_doze_1(&self) -> bool {
        *self == LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1
    }
}
#[doc = "Field `LPI2C2_IPG_DOZE` writer - LPI2C2 ipg_doze mode"]
pub type LPI2C2_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C2_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPI2C2_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C2_IPG_DOZE_A::LPI2C2_IPG_DOZE_1)
    }
}
#[doc = "Field `LPI2C3_IPG_STOP_MODE` reader - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C3_IPG_STOP_MODE_R = crate::BitReader<LPI2C3_IPG_STOP_MODE_A>;
#[doc = "LPI2C3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C3_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C3_IPG_STOP_MODE_1 = 1,
}
impl From<LPI2C3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C3_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0,
            true => LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPI2C3_IPG_STOP_MODE` writer - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C3_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C3_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPI2C3_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_STOP_MODE_A::LPI2C3_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPI2C3_IPG_DOZE` reader - LPI2C3 ipg_doze mode"]
pub type LPI2C3_IPG_DOZE_R = crate::BitReader<LPI2C3_IPG_DOZE_A>;
#[doc = "LPI2C3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C3_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPI2C3_IPG_DOZE_1 = 1,
}
impl From<LPI2C3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C3_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_IPG_DOZE_A {
        match self.bits {
            false => LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0,
            true => LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_doze_0(&self) -> bool {
        *self == LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_ipg_doze_1(&self) -> bool {
        *self == LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1
    }
}
#[doc = "Field `LPI2C3_IPG_DOZE` writer - LPI2C3 ipg_doze mode"]
pub type LPI2C3_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C3_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPI2C3_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C3_IPG_DOZE_A::LPI2C3_IPG_DOZE_1)
    }
}
#[doc = "Field `LPI2C4_IPG_STOP_MODE` reader - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C4_IPG_STOP_MODE_R = crate::BitReader<LPI2C4_IPG_STOP_MODE_A>;
#[doc = "LPI2C4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPI2C4_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C4_IPG_STOP_MODE_1 = 1,
}
impl From<LPI2C4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C4_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0,
            true => LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_stop_mode_0(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_stop_mode_1(&self) -> bool {
        *self == LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPI2C4_IPG_STOP_MODE` writer - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPI2C4_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C4_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPI2C4_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_STOP_MODE_A::LPI2C4_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPI2C4_IPG_DOZE` reader - LPI2C4 ipg_doze mode"]
pub type LPI2C4_IPG_DOZE_R = crate::BitReader<LPI2C4_IPG_DOZE_A>;
#[doc = "LPI2C4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPI2C4_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPI2C4_IPG_DOZE_1 = 1,
}
impl From<LPI2C4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C4_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_IPG_DOZE_A {
        match self.bits {
            false => LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0,
            true => LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_doze_0(&self) -> bool {
        *self == LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_ipg_doze_1(&self) -> bool {
        *self == LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1
    }
}
#[doc = "Field `LPI2C4_IPG_DOZE` writer - LPI2C4 ipg_doze mode"]
pub type LPI2C4_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPI2C4_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPI2C4_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPI2C4_IPG_DOZE_A::LPI2C4_IPG_DOZE_1)
    }
}
#[doc = "Field `LPSPI1_IPG_STOP_MODE` reader - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI1_IPG_STOP_MODE_R = crate::BitReader<LPSPI1_IPG_STOP_MODE_A>;
#[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI1_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI1_IPG_STOP_MODE_1 = 1,
}
impl From<LPSPI1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI1_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0,
            true => LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPSPI1_IPG_STOP_MODE` writer - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI1_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI1_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPSPI1_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_STOP_MODE_A::LPSPI1_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPSPI1_IPG_DOZE` reader - LPSPI1 ipg_doze mode"]
pub type LPSPI1_IPG_DOZE_R = crate::BitReader<LPSPI1_IPG_DOZE_A>;
#[doc = "LPSPI1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI1_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPSPI1_IPG_DOZE_1 = 1,
}
impl From<LPSPI1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI1_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_IPG_DOZE_A {
        match self.bits {
            false => LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0,
            true => LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_doze_0(&self) -> bool {
        *self == LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi1_ipg_doze_1(&self) -> bool {
        *self == LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1
    }
}
#[doc = "Field `LPSPI1_IPG_DOZE` writer - LPSPI1 ipg_doze mode"]
pub type LPSPI1_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI1_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPSPI1_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI1_IPG_DOZE_A::LPSPI1_IPG_DOZE_1)
    }
}
#[doc = "Field `LPSPI2_IPG_STOP_MODE` reader - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI2_IPG_STOP_MODE_R = crate::BitReader<LPSPI2_IPG_STOP_MODE_A>;
#[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI2_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI2_IPG_STOP_MODE_1 = 1,
}
impl From<LPSPI2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI2_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0,
            true => LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPSPI2_IPG_STOP_MODE` writer - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI2_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI2_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPSPI2_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_STOP_MODE_A::LPSPI2_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPSPI2_IPG_DOZE` reader - LPSPI2 ipg_doze mode"]
pub type LPSPI2_IPG_DOZE_R = crate::BitReader<LPSPI2_IPG_DOZE_A>;
#[doc = "LPSPI2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI2_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPSPI2_IPG_DOZE_1 = 1,
}
impl From<LPSPI2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI2_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_IPG_DOZE_A {
        match self.bits {
            false => LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0,
            true => LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_doze_0(&self) -> bool {
        *self == LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi2_ipg_doze_1(&self) -> bool {
        *self == LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1
    }
}
#[doc = "Field `LPSPI2_IPG_DOZE` writer - LPSPI2 ipg_doze mode"]
pub type LPSPI2_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI2_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPSPI2_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI2_IPG_DOZE_A::LPSPI2_IPG_DOZE_1)
    }
}
#[doc = "Field `LPSPI3_IPG_STOP_MODE` reader - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI3_IPG_STOP_MODE_R = crate::BitReader<LPSPI3_IPG_STOP_MODE_A>;
#[doc = "LPSPI3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI3_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI3_IPG_STOP_MODE_1 = 1,
}
impl From<LPSPI3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI3_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0,
            true => LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPSPI3_IPG_STOP_MODE` writer - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI3_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI3_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPSPI3_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_STOP_MODE_A::LPSPI3_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPSPI3_IPG_DOZE` reader - LPSPI3 ipg_doze mode"]
pub type LPSPI3_IPG_DOZE_R = crate::BitReader<LPSPI3_IPG_DOZE_A>;
#[doc = "LPSPI3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI3_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPSPI3_IPG_DOZE_1 = 1,
}
impl From<LPSPI3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI3_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_IPG_DOZE_A {
        match self.bits {
            false => LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0,
            true => LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_doze_0(&self) -> bool {
        *self == LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi3_ipg_doze_1(&self) -> bool {
        *self == LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1
    }
}
#[doc = "Field `LPSPI3_IPG_DOZE` writer - LPSPI3 ipg_doze mode"]
pub type LPSPI3_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI3_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPSPI3_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI3_IPG_DOZE_A::LPSPI3_IPG_DOZE_1)
    }
}
#[doc = "Field `LPSPI4_IPG_STOP_MODE` reader - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI4_IPG_STOP_MODE_R = crate::BitReader<LPSPI4_IPG_STOP_MODE_A>;
#[doc = "LPSPI4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPSPI4_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI4_IPG_STOP_MODE_1 = 1,
}
impl From<LPSPI4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI4_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0,
            true => LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_stop_mode_0(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_stop_mode_1(&self) -> bool {
        *self == LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPSPI4_IPG_STOP_MODE` writer - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPSPI4_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI4_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPSPI4_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_STOP_MODE_A::LPSPI4_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPSPI4_IPG_DOZE` reader - LPSPI4 ipg_doze mode"]
pub type LPSPI4_IPG_DOZE_R = crate::BitReader<LPSPI4_IPG_DOZE_A>;
#[doc = "LPSPI4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPSPI4_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPSPI4_IPG_DOZE_1 = 1,
}
impl From<LPSPI4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI4_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_IPG_DOZE_A {
        match self.bits {
            false => LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0,
            true => LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_doze_0(&self) -> bool {
        *self == LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpspi4_ipg_doze_1(&self) -> bool {
        *self == LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1
    }
}
#[doc = "Field `LPSPI4_IPG_DOZE` writer - LPSPI4 ipg_doze mode"]
pub type LPSPI4_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPSPI4_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPSPI4_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPSPI4_IPG_DOZE_A::LPSPI4_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART1_IPG_STOP_MODE` reader - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART1_IPG_STOP_MODE_R = crate::BitReader<LPUART1_IPG_STOP_MODE_A>;
#[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART1_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART1_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART1_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0,
            true => LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART1_IPG_STOP_MODE` writer - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART1_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART1_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART1_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_STOP_MODE_A::LPUART1_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART1_IPG_DOZE` reader - LPUART1 ipg_doze mode"]
pub type LPUART1_IPG_DOZE_R = crate::BitReader<LPUART1_IPG_DOZE_A>;
#[doc = "LPUART1 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART1_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART1_IPG_DOZE_1 = 1,
}
impl From<LPUART1_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_IPG_DOZE_A {
        match self.bits {
            false => LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0,
            true => LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_doze_0(&self) -> bool {
        *self == LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart1_ipg_doze_1(&self) -> bool {
        *self == LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART1_IPG_DOZE` writer - LPUART1 ipg_doze mode"]
pub type LPUART1_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART1_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART1_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART1_IPG_DOZE_A::LPUART1_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART2_IPG_STOP_MODE` reader - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART2_IPG_STOP_MODE_R = crate::BitReader<LPUART2_IPG_STOP_MODE_A>;
#[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART2_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART2_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART2_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART2_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART2_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0,
            true => LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART2_IPG_STOP_MODE` writer - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART2_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART2_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART2_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_STOP_MODE_A::LPUART2_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART2_IPG_DOZE` reader - LPUART2 ipg_doze mode"]
pub type LPUART2_IPG_DOZE_R = crate::BitReader<LPUART2_IPG_DOZE_A>;
#[doc = "LPUART2 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART2_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART2_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART2_IPG_DOZE_1 = 1,
}
impl From<LPUART2_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART2_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_IPG_DOZE_A {
        match self.bits {
            false => LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0,
            true => LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_doze_0(&self) -> bool {
        *self == LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart2_ipg_doze_1(&self) -> bool {
        *self == LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART2_IPG_DOZE` writer - LPUART2 ipg_doze mode"]
pub type LPUART2_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART2_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART2_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART2_IPG_DOZE_A::LPUART2_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART3_IPG_STOP_MODE` reader - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART3_IPG_STOP_MODE_R = crate::BitReader<LPUART3_IPG_STOP_MODE_A>;
#[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART3_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART3_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART3_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART3_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART3_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0,
            true => LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART3_IPG_STOP_MODE` writer - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART3_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART3_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART3_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_STOP_MODE_A::LPUART3_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART3_IPG_DOZE` reader - LPUART3 ipg_doze mode"]
pub type LPUART3_IPG_DOZE_R = crate::BitReader<LPUART3_IPG_DOZE_A>;
#[doc = "LPUART3 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART3_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART3_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART3_IPG_DOZE_1 = 1,
}
impl From<LPUART3_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART3_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_IPG_DOZE_A {
        match self.bits {
            false => LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0,
            true => LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_doze_0(&self) -> bool {
        *self == LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart3_ipg_doze_1(&self) -> bool {
        *self == LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART3_IPG_DOZE` writer - LPUART3 ipg_doze mode"]
pub type LPUART3_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART3_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART3_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART3_IPG_DOZE_A::LPUART3_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART4_IPG_STOP_MODE` reader - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART4_IPG_STOP_MODE_R = crate::BitReader<LPUART4_IPG_STOP_MODE_A>;
#[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART4_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART4_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART4_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART4_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART4_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0,
            true => LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART4_IPG_STOP_MODE` writer - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART4_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART4_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART4_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_STOP_MODE_A::LPUART4_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART4_IPG_DOZE` reader - LPUART4 ipg_doze mode"]
pub type LPUART4_IPG_DOZE_R = crate::BitReader<LPUART4_IPG_DOZE_A>;
#[doc = "LPUART4 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART4_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART4_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART4_IPG_DOZE_1 = 1,
}
impl From<LPUART4_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART4_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_IPG_DOZE_A {
        match self.bits {
            false => LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0,
            true => LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_doze_0(&self) -> bool {
        *self == LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart4_ipg_doze_1(&self) -> bool {
        *self == LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART4_IPG_DOZE` writer - LPUART4 ipg_doze mode"]
pub type LPUART4_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART4_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART4_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART4_IPG_DOZE_A::LPUART4_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART5_IPG_STOP_MODE` reader - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART5_IPG_STOP_MODE_R = crate::BitReader<LPUART5_IPG_STOP_MODE_A>;
#[doc = "LPUART5 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART5_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART5_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART5_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART5_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART5_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0,
            true => LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART5_IPG_STOP_MODE` writer - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART5_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART5_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART5_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_STOP_MODE_A::LPUART5_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART5_IPG_DOZE` reader - LPUART5 ipg_doze mode"]
pub type LPUART5_IPG_DOZE_R = crate::BitReader<LPUART5_IPG_DOZE_A>;
#[doc = "LPUART5 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART5_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART5_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART5_IPG_DOZE_1 = 1,
}
impl From<LPUART5_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART5_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_IPG_DOZE_A {
        match self.bits {
            false => LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0,
            true => LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_doze_0(&self) -> bool {
        *self == LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart5_ipg_doze_1(&self) -> bool {
        *self == LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART5_IPG_DOZE` writer - LPUART5 ipg_doze mode"]
pub type LPUART5_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART5_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART5_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART5_IPG_DOZE_A::LPUART5_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART6_IPG_STOP_MODE` reader - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART6_IPG_STOP_MODE_R = crate::BitReader<LPUART6_IPG_STOP_MODE_A>;
#[doc = "LPUART6 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART6_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART6_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART6_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART6_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART6_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0,
            true => LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART6_IPG_STOP_MODE` writer - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART6_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART6_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART6_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_STOP_MODE_A::LPUART6_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART6_IPG_DOZE` reader - LPUART6 ipg_doze mode"]
pub type LPUART6_IPG_DOZE_R = crate::BitReader<LPUART6_IPG_DOZE_A>;
#[doc = "LPUART6 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART6_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART6_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART6_IPG_DOZE_1 = 1,
}
impl From<LPUART6_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART6_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_IPG_DOZE_A {
        match self.bits {
            false => LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0,
            true => LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_doze_0(&self) -> bool {
        *self == LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart6_ipg_doze_1(&self) -> bool {
        *self == LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART6_IPG_DOZE` writer - LPUART6 ipg_doze mode"]
pub type LPUART6_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART6_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART6_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART6_IPG_DOZE_A::LPUART6_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART7_IPG_STOP_MODE` reader - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART7_IPG_STOP_MODE_R = crate::BitReader<LPUART7_IPG_STOP_MODE_A>;
#[doc = "LPUART7 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART7_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART7_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART7_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART7_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART7_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0,
            true => LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART7_IPG_STOP_MODE` writer - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART7_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART7_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART7_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_STOP_MODE_A::LPUART7_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART7_IPG_DOZE` reader - LPUART7 ipg_doze mode"]
pub type LPUART7_IPG_DOZE_R = crate::BitReader<LPUART7_IPG_DOZE_A>;
#[doc = "LPUART7 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART7_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART7_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART7_IPG_DOZE_1 = 1,
}
impl From<LPUART7_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART7_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_IPG_DOZE_A {
        match self.bits {
            false => LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0,
            true => LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_doze_0(&self) -> bool {
        *self == LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart7_ipg_doze_1(&self) -> bool {
        *self == LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART7_IPG_DOZE` writer - LPUART7 ipg_doze mode"]
pub type LPUART7_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART7_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART7_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART7_IPG_DOZE_A::LPUART7_IPG_DOZE_1)
    }
}
#[doc = "Field `LPUART8_IPG_STOP_MODE` reader - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART8_IPG_STOP_MODE_R = crate::BitReader<LPUART8_IPG_STOP_MODE_A>;
#[doc = "LPUART8 stop mode selection, cannot change when ipg_stop is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART8_IPG_STOP_MODE_A {
    #[doc = "0: the module is functional in Stop mode"]
    LPUART8_IPG_STOP_MODE_0 = 0,
    #[doc = "1: the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART8_IPG_STOP_MODE_1 = 1,
}
impl From<LPUART8_IPG_STOP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_IPG_STOP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART8_IPG_STOP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_IPG_STOP_MODE_A {
        match self.bits {
            false => LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0,
            true => LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_0`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_stop_mode_0(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_STOP_MODE_1`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_stop_mode_1(&self) -> bool {
        *self == LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1
    }
}
#[doc = "Field `LPUART8_IPG_STOP_MODE` writer - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
pub type LPUART8_IPG_STOP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART8_IPG_STOP_MODE_A, O>;
impl<'a, const O: u8> LPUART8_IPG_STOP_MODE_W<'a, O> {
    #[doc = "the module is functional in Stop mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_0)
    }
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_STOP_MODE_A::LPUART8_IPG_STOP_MODE_1)
    }
}
#[doc = "Field `LPUART8_IPG_DOZE` reader - LPUART8 ipg_doze mode"]
pub type LPUART8_IPG_DOZE_R = crate::BitReader<LPUART8_IPG_DOZE_A>;
#[doc = "LPUART8 ipg_doze mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART8_IPG_DOZE_A {
    #[doc = "0: not in doze mode"]
    LPUART8_IPG_DOZE_0 = 0,
    #[doc = "1: in doze mode"]
    LPUART8_IPG_DOZE_1 = 1,
}
impl From<LPUART8_IPG_DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_IPG_DOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART8_IPG_DOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_IPG_DOZE_A {
        match self.bits {
            false => LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0,
            true => LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_0`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_doze_0(&self) -> bool {
        *self == LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_IPG_DOZE_1`"]
    #[inline(always)]
    pub fn is_lpuart8_ipg_doze_1(&self) -> bool {
        *self == LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1
    }
}
#[doc = "Field `LPUART8_IPG_DOZE` writer - LPUART8 ipg_doze mode"]
pub type LPUART8_IPG_DOZE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR8_SPEC, LPUART8_IPG_DOZE_A, O>;
impl<'a, const O: u8> LPUART8_IPG_DOZE_W<'a, O> {
    #[doc = "not in doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze_0(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_0)
    }
    #[doc = "in doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze_1(self) -> &'a mut W {
        self.variant(LPUART8_IPG_DOZE_A::LPUART8_IPG_DOZE_1)
    }
}
impl R {
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c1_ipg_stop_mode(&self) -> LPI2C1_IPG_STOP_MODE_R {
        LPI2C1_IPG_STOP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c1_ipg_doze(&self) -> LPI2C1_IPG_DOZE_R {
        LPI2C1_IPG_DOZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c2_ipg_stop_mode(&self) -> LPI2C2_IPG_STOP_MODE_R {
        LPI2C2_IPG_STOP_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c2_ipg_doze(&self) -> LPI2C2_IPG_DOZE_R {
        LPI2C2_IPG_DOZE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c3_ipg_stop_mode(&self) -> LPI2C3_IPG_STOP_MODE_R {
        LPI2C3_IPG_STOP_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c3_ipg_doze(&self) -> LPI2C3_IPG_DOZE_R {
        LPI2C3_IPG_DOZE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpi2c4_ipg_stop_mode(&self) -> LPI2C4_IPG_STOP_MODE_R {
        LPI2C4_IPG_STOP_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpi2c4_ipg_doze(&self) -> LPI2C4_IPG_DOZE_R {
        LPI2C4_IPG_DOZE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi1_ipg_stop_mode(&self) -> LPSPI1_IPG_STOP_MODE_R {
        LPSPI1_IPG_STOP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi1_ipg_doze(&self) -> LPSPI1_IPG_DOZE_R {
        LPSPI1_IPG_DOZE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi2_ipg_stop_mode(&self) -> LPSPI2_IPG_STOP_MODE_R {
        LPSPI2_IPG_STOP_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi2_ipg_doze(&self) -> LPSPI2_IPG_DOZE_R {
        LPSPI2_IPG_DOZE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi3_ipg_stop_mode(&self) -> LPSPI3_IPG_STOP_MODE_R {
        LPSPI3_IPG_STOP_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi3_ipg_doze(&self) -> LPSPI3_IPG_DOZE_R {
        LPSPI3_IPG_DOZE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpspi4_ipg_stop_mode(&self) -> LPSPI4_IPG_STOP_MODE_R {
        LPSPI4_IPG_STOP_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpspi4_ipg_doze(&self) -> LPSPI4_IPG_DOZE_R {
        LPSPI4_IPG_DOZE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart1_ipg_stop_mode(&self) -> LPUART1_IPG_STOP_MODE_R {
        LPUART1_IPG_STOP_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart1_ipg_doze(&self) -> LPUART1_IPG_DOZE_R {
        LPUART1_IPG_DOZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart2_ipg_stop_mode(&self) -> LPUART2_IPG_STOP_MODE_R {
        LPUART2_IPG_STOP_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart2_ipg_doze(&self) -> LPUART2_IPG_DOZE_R {
        LPUART2_IPG_DOZE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart3_ipg_stop_mode(&self) -> LPUART3_IPG_STOP_MODE_R {
        LPUART3_IPG_STOP_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart3_ipg_doze(&self) -> LPUART3_IPG_DOZE_R {
        LPUART3_IPG_DOZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart4_ipg_stop_mode(&self) -> LPUART4_IPG_STOP_MODE_R {
        LPUART4_IPG_STOP_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart4_ipg_doze(&self) -> LPUART4_IPG_DOZE_R {
        LPUART4_IPG_DOZE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart5_ipg_stop_mode(&self) -> LPUART5_IPG_STOP_MODE_R {
        LPUART5_IPG_STOP_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart5_ipg_doze(&self) -> LPUART5_IPG_DOZE_R {
        LPUART5_IPG_DOZE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart6_ipg_stop_mode(&self) -> LPUART6_IPG_STOP_MODE_R {
        LPUART6_IPG_STOP_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart6_ipg_doze(&self) -> LPUART6_IPG_DOZE_R {
        LPUART6_IPG_DOZE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart7_ipg_stop_mode(&self) -> LPUART7_IPG_STOP_MODE_R {
        LPUART7_IPG_STOP_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart7_ipg_doze(&self) -> LPUART7_IPG_DOZE_R {
        LPUART7_IPG_DOZE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub fn lpuart8_ipg_stop_mode(&self) -> LPUART8_IPG_STOP_MODE_R {
        LPUART8_IPG_STOP_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline(always)]
    pub fn lpuart8_ipg_doze(&self) -> LPUART8_IPG_DOZE_R {
        LPUART8_IPG_DOZE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c1_ipg_stop_mode(&mut self) -> LPI2C1_IPG_STOP_MODE_W<0> {
        LPI2C1_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 1 - LPI2C1 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c1_ipg_doze(&mut self) -> LPI2C1_IPG_DOZE_W<1> {
        LPI2C1_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 2 - LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c2_ipg_stop_mode(&mut self) -> LPI2C2_IPG_STOP_MODE_W<2> {
        LPI2C2_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 3 - LPI2C2 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c2_ipg_doze(&mut self) -> LPI2C2_IPG_DOZE_W<3> {
        LPI2C2_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 4 - LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c3_ipg_stop_mode(&mut self) -> LPI2C3_IPG_STOP_MODE_W<4> {
        LPI2C3_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 5 - LPI2C3 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c3_ipg_doze(&mut self) -> LPI2C3_IPG_DOZE_W<5> {
        LPI2C3_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 6 - LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c4_ipg_stop_mode(&mut self) -> LPI2C4_IPG_STOP_MODE_W<6> {
        LPI2C4_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 7 - LPI2C4 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c4_ipg_doze(&mut self) -> LPI2C4_IPG_DOZE_W<7> {
        LPI2C4_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 8 - LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpspi1_ipg_stop_mode(&mut self) -> LPSPI1_IPG_STOP_MODE_W<8> {
        LPSPI1_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 9 - LPSPI1 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi1_ipg_doze(&mut self) -> LPSPI1_IPG_DOZE_W<9> {
        LPSPI1_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 10 - LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpspi2_ipg_stop_mode(&mut self) -> LPSPI2_IPG_STOP_MODE_W<10> {
        LPSPI2_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 11 - LPSPI2 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi2_ipg_doze(&mut self) -> LPSPI2_IPG_DOZE_W<11> {
        LPSPI2_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 12 - LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpspi3_ipg_stop_mode(&mut self) -> LPSPI3_IPG_STOP_MODE_W<12> {
        LPSPI3_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 13 - LPSPI3 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi3_ipg_doze(&mut self) -> LPSPI3_IPG_DOZE_W<13> {
        LPSPI3_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 14 - LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpspi4_ipg_stop_mode(&mut self) -> LPSPI4_IPG_STOP_MODE_W<14> {
        LPSPI4_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 15 - LPSPI4 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi4_ipg_doze(&mut self) -> LPSPI4_IPG_DOZE_W<15> {
        LPSPI4_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 16 - LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1_ipg_stop_mode(&mut self) -> LPUART1_IPG_STOP_MODE_W<16> {
        LPUART1_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 17 - LPUART1 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1_ipg_doze(&mut self) -> LPUART1_IPG_DOZE_W<17> {
        LPUART1_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 18 - LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2_ipg_stop_mode(&mut self) -> LPUART2_IPG_STOP_MODE_W<18> {
        LPUART2_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 19 - LPUART2 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2_ipg_doze(&mut self) -> LPUART2_IPG_DOZE_W<19> {
        LPUART2_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 20 - LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3_ipg_stop_mode(&mut self) -> LPUART3_IPG_STOP_MODE_W<20> {
        LPUART3_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 21 - LPUART3 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3_ipg_doze(&mut self) -> LPUART3_IPG_DOZE_W<21> {
        LPUART3_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 22 - LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart4_ipg_stop_mode(&mut self) -> LPUART4_IPG_STOP_MODE_W<22> {
        LPUART4_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 23 - LPUART4 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart4_ipg_doze(&mut self) -> LPUART4_IPG_DOZE_W<23> {
        LPUART4_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 24 - LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart5_ipg_stop_mode(&mut self) -> LPUART5_IPG_STOP_MODE_W<24> {
        LPUART5_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 25 - LPUART5 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart5_ipg_doze(&mut self) -> LPUART5_IPG_DOZE_W<25> {
        LPUART5_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 26 - LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart6_ipg_stop_mode(&mut self) -> LPUART6_IPG_STOP_MODE_W<26> {
        LPUART6_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 27 - LPUART6 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart6_ipg_doze(&mut self) -> LPUART6_IPG_DOZE_W<27> {
        LPUART6_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 28 - LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart7_ipg_stop_mode(&mut self) -> LPUART7_IPG_STOP_MODE_W<28> {
        LPUART7_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 29 - LPUART7 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart7_ipg_doze(&mut self) -> LPUART7_IPG_DOZE_W<29> {
        LPUART7_IPG_DOZE_W::new(self)
    }
    #[doc = "Bit 30 - LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart8_ipg_stop_mode(&mut self) -> LPUART8_IPG_STOP_MODE_W<30> {
        LPUART8_IPG_STOP_MODE_W::new(self)
    }
    #[doc = "Bit 31 - LPUART8 ipg_doze mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart8_ipg_doze(&mut self) -> LPUART8_IPG_DOZE_W<31> {
        LPUART8_IPG_DOZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR8 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr8](index.html) module"]
pub struct GPR8_SPEC;
impl crate::RegisterSpec for GPR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr8::R](R) reader structure"]
impl crate::Readable for GPR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr8::W](W) writer structure"]
impl crate::Writable for GPR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR8 to value 0"]
impl crate::Resettable for GPR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
