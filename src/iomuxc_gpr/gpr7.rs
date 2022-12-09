#[doc = "Register `GPR7` reader"]
pub struct R(crate::R<GPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR7` writer"]
pub struct W(crate::W<GPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR7_SPEC>;
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
impl From<crate::W<GPR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPI2C1_STOP_REQ` reader - LPI2C1 stop request"]
pub type LPI2C1_STOP_REQ_R = crate::BitReader<LPI2C1_STOP_REQ_A>;
#[doc = "LPI2C1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C1_STOP_REQ_1 = 1,
}
impl From<LPI2C1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_STOP_REQ_A {
        match self.bits {
            false => LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0,
            true => LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_req_0(&self) -> bool {
        *self == LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_req_1(&self) -> bool {
        *self == LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1
    }
}
#[doc = "Field `LPI2C1_STOP_REQ` writer - LPI2C1 stop request"]
pub type LPI2C1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPI2C1_STOP_REQ_A, O>;
impl<'a, const O: u8> LPI2C1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c1_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c1_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C1_STOP_REQ_A::LPI2C1_STOP_REQ_1)
    }
}
#[doc = "Field `LPI2C2_STOP_REQ` reader - LPI2C2 stop request"]
pub type LPI2C2_STOP_REQ_R = crate::BitReader<LPI2C2_STOP_REQ_A>;
#[doc = "LPI2C2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C2_STOP_REQ_1 = 1,
}
impl From<LPI2C2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C2_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_STOP_REQ_A {
        match self.bits {
            false => LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0,
            true => LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_req_0(&self) -> bool {
        *self == LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_req_1(&self) -> bool {
        *self == LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1
    }
}
#[doc = "Field `LPI2C2_STOP_REQ` writer - LPI2C2 stop request"]
pub type LPI2C2_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPI2C2_STOP_REQ_A, O>;
impl<'a, const O: u8> LPI2C2_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c2_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c2_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C2_STOP_REQ_A::LPI2C2_STOP_REQ_1)
    }
}
#[doc = "Field `LPI2C3_STOP_REQ` reader - LPI2C3 stop request"]
pub type LPI2C3_STOP_REQ_R = crate::BitReader<LPI2C3_STOP_REQ_A>;
#[doc = "LPI2C3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C3_STOP_REQ_1 = 1,
}
impl From<LPI2C3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C3_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_STOP_REQ_A {
        match self.bits {
            false => LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0,
            true => LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_req_0(&self) -> bool {
        *self == LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_req_1(&self) -> bool {
        *self == LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1
    }
}
#[doc = "Field `LPI2C3_STOP_REQ` writer - LPI2C3 stop request"]
pub type LPI2C3_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPI2C3_STOP_REQ_A, O>;
impl<'a, const O: u8> LPI2C3_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c3_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c3_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C3_STOP_REQ_A::LPI2C3_STOP_REQ_1)
    }
}
#[doc = "Field `LPI2C4_STOP_REQ` reader - LPI2C4 stop request"]
pub type LPI2C4_STOP_REQ_R = crate::BitReader<LPI2C4_STOP_REQ_A>;
#[doc = "LPI2C4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPI2C4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPI2C4_STOP_REQ_1 = 1,
}
impl From<LPI2C4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C4_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_STOP_REQ_A {
        match self.bits {
            false => LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0,
            true => LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_req_0(&self) -> bool {
        *self == LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_req_1(&self) -> bool {
        *self == LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1
    }
}
#[doc = "Field `LPI2C4_STOP_REQ` writer - LPI2C4 stop request"]
pub type LPI2C4_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPI2C4_STOP_REQ_A, O>;
impl<'a, const O: u8> LPI2C4_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpi2c4_stop_req_0(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpi2c4_stop_req_1(self) -> &'a mut W {
        self.variant(LPI2C4_STOP_REQ_A::LPI2C4_STOP_REQ_1)
    }
}
#[doc = "Field `LPSPI1_STOP_REQ` reader - LPSPI1 stop request"]
pub type LPSPI1_STOP_REQ_R = crate::BitReader<LPSPI1_STOP_REQ_A>;
#[doc = "LPSPI1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI1_STOP_REQ_1 = 1,
}
impl From<LPSPI1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_STOP_REQ_A {
        match self.bits {
            false => LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0,
            true => LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_req_0(&self) -> bool {
        *self == LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_req_1(&self) -> bool {
        *self == LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1
    }
}
#[doc = "Field `LPSPI1_STOP_REQ` writer - LPSPI1 stop request"]
pub type LPSPI1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPSPI1_STOP_REQ_A, O>;
impl<'a, const O: u8> LPSPI1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi1_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi1_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI1_STOP_REQ_A::LPSPI1_STOP_REQ_1)
    }
}
#[doc = "Field `LPSPI2_STOP_REQ` reader - LPSPI2 stop request"]
pub type LPSPI2_STOP_REQ_R = crate::BitReader<LPSPI2_STOP_REQ_A>;
#[doc = "LPSPI2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI2_STOP_REQ_1 = 1,
}
impl From<LPSPI2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI2_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_STOP_REQ_A {
        match self.bits {
            false => LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0,
            true => LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_req_0(&self) -> bool {
        *self == LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_req_1(&self) -> bool {
        *self == LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1
    }
}
#[doc = "Field `LPSPI2_STOP_REQ` writer - LPSPI2 stop request"]
pub type LPSPI2_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPSPI2_STOP_REQ_A, O>;
impl<'a, const O: u8> LPSPI2_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi2_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi2_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI2_STOP_REQ_A::LPSPI2_STOP_REQ_1)
    }
}
#[doc = "Field `LPSPI3_STOP_REQ` reader - LPSPI3 stop request"]
pub type LPSPI3_STOP_REQ_R = crate::BitReader<LPSPI3_STOP_REQ_A>;
#[doc = "LPSPI3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI3_STOP_REQ_1 = 1,
}
impl From<LPSPI3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI3_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_STOP_REQ_A {
        match self.bits {
            false => LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0,
            true => LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_req_0(&self) -> bool {
        *self == LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_req_1(&self) -> bool {
        *self == LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1
    }
}
#[doc = "Field `LPSPI3_STOP_REQ` writer - LPSPI3 stop request"]
pub type LPSPI3_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPSPI3_STOP_REQ_A, O>;
impl<'a, const O: u8> LPSPI3_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi3_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi3_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI3_STOP_REQ_A::LPSPI3_STOP_REQ_1)
    }
}
#[doc = "Field `LPSPI4_STOP_REQ` reader - LPSPI4 stop request"]
pub type LPSPI4_STOP_REQ_R = crate::BitReader<LPSPI4_STOP_REQ_A>;
#[doc = "LPSPI4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPSPI4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPSPI4_STOP_REQ_1 = 1,
}
impl From<LPSPI4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI4_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_STOP_REQ_A {
        match self.bits {
            false => LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0,
            true => LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_req_0(&self) -> bool {
        *self == LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_req_1(&self) -> bool {
        *self == LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1
    }
}
#[doc = "Field `LPSPI4_STOP_REQ` writer - LPSPI4 stop request"]
pub type LPSPI4_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPSPI4_STOP_REQ_A, O>;
impl<'a, const O: u8> LPSPI4_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpspi4_stop_req_0(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpspi4_stop_req_1(self) -> &'a mut W {
        self.variant(LPSPI4_STOP_REQ_A::LPSPI4_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART1_STOP_REQ` reader - LPUART1 stop request"]
pub type LPUART1_STOP_REQ_R = crate::BitReader<LPUART1_STOP_REQ_A>;
#[doc = "LPUART1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART1_STOP_REQ_1 = 1,
}
impl From<LPUART1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_STOP_REQ_A {
        match self.bits {
            false => LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0,
            true => LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_req_0(&self) -> bool {
        *self == LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_req_1(&self) -> bool {
        *self == LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1
    }
}
#[doc = "Field `LPUART1_STOP_REQ` writer - LPUART1 stop request"]
pub type LPUART1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART1_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart1_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart1_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART1_STOP_REQ_A::LPUART1_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART2_STOP_REQ` reader - LPUART2 stop request"]
pub type LPUART2_STOP_REQ_R = crate::BitReader<LPUART2_STOP_REQ_A>;
#[doc = "LPUART2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART2_STOP_REQ_1 = 1,
}
impl From<LPUART2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART2_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_STOP_REQ_A {
        match self.bits {
            false => LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0,
            true => LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_req_0(&self) -> bool {
        *self == LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_req_1(&self) -> bool {
        *self == LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1
    }
}
#[doc = "Field `LPUART2_STOP_REQ` writer - LPUART2 stop request"]
pub type LPUART2_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART2_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART2_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart2_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart2_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART2_STOP_REQ_A::LPUART2_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART3_STOP_REQ` reader - LPUART3 stop request"]
pub type LPUART3_STOP_REQ_R = crate::BitReader<LPUART3_STOP_REQ_A>;
#[doc = "LPUART3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART3_STOP_REQ_1 = 1,
}
impl From<LPUART3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART3_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_STOP_REQ_A {
        match self.bits {
            false => LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0,
            true => LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_req_0(&self) -> bool {
        *self == LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_req_1(&self) -> bool {
        *self == LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1
    }
}
#[doc = "Field `LPUART3_STOP_REQ` writer - LPUART3 stop request"]
pub type LPUART3_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART3_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART3_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart3_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart3_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART3_STOP_REQ_A::LPUART3_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART4_STOP_REQ` reader - LPUART4 stop request"]
pub type LPUART4_STOP_REQ_R = crate::BitReader<LPUART4_STOP_REQ_A>;
#[doc = "LPUART4 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART4_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART4_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART4_STOP_REQ_1 = 1,
}
impl From<LPUART4_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART4_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_STOP_REQ_A {
        match self.bits {
            false => LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0,
            true => LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_req_0(&self) -> bool {
        *self == LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_req_1(&self) -> bool {
        *self == LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1
    }
}
#[doc = "Field `LPUART4_STOP_REQ` writer - LPUART4 stop request"]
pub type LPUART4_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART4_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART4_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart4_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart4_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART4_STOP_REQ_A::LPUART4_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART5_STOP_REQ` reader - LPUART5 stop request"]
pub type LPUART5_STOP_REQ_R = crate::BitReader<LPUART5_STOP_REQ_A>;
#[doc = "LPUART5 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART5_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART5_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART5_STOP_REQ_1 = 1,
}
impl From<LPUART5_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART5_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_STOP_REQ_A {
        match self.bits {
            false => LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0,
            true => LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_req_0(&self) -> bool {
        *self == LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_req_1(&self) -> bool {
        *self == LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1
    }
}
#[doc = "Field `LPUART5_STOP_REQ` writer - LPUART5 stop request"]
pub type LPUART5_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART5_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART5_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart5_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart5_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART5_STOP_REQ_A::LPUART5_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART6_STOP_REQ` reader - LPUART6 stop request"]
pub type LPUART6_STOP_REQ_R = crate::BitReader<LPUART6_STOP_REQ_A>;
#[doc = "LPUART6 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART6_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART6_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART6_STOP_REQ_1 = 1,
}
impl From<LPUART6_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART6_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_STOP_REQ_A {
        match self.bits {
            false => LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0,
            true => LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_req_0(&self) -> bool {
        *self == LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_req_1(&self) -> bool {
        *self == LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1
    }
}
#[doc = "Field `LPUART6_STOP_REQ` writer - LPUART6 stop request"]
pub type LPUART6_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART6_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART6_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart6_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart6_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART6_STOP_REQ_A::LPUART6_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART7_STOP_REQ` reader - LPUART7 stop request"]
pub type LPUART7_STOP_REQ_R = crate::BitReader<LPUART7_STOP_REQ_A>;
#[doc = "LPUART7 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART7_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART7_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART7_STOP_REQ_1 = 1,
}
impl From<LPUART7_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART7_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_STOP_REQ_A {
        match self.bits {
            false => LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0,
            true => LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_req_0(&self) -> bool {
        *self == LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_req_1(&self) -> bool {
        *self == LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1
    }
}
#[doc = "Field `LPUART7_STOP_REQ` writer - LPUART7 stop request"]
pub type LPUART7_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART7_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART7_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart7_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart7_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART7_STOP_REQ_A::LPUART7_STOP_REQ_1)
    }
}
#[doc = "Field `LPUART8_STOP_REQ` reader - LPUART8 stop request"]
pub type LPUART8_STOP_REQ_R = crate::BitReader<LPUART8_STOP_REQ_A>;
#[doc = "LPUART8 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART8_STOP_REQ_A {
    #[doc = "0: stop request off"]
    LPUART8_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    LPUART8_STOP_REQ_1 = 1,
}
impl From<LPUART8_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART8_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_STOP_REQ_A {
        match self.bits {
            false => LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0,
            true => LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_req_0(&self) -> bool {
        *self == LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_req_1(&self) -> bool {
        *self == LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1
    }
}
#[doc = "Field `LPUART8_STOP_REQ` writer - LPUART8 stop request"]
pub type LPUART8_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR7_SPEC, LPUART8_STOP_REQ_A, O>;
impl<'a, const O: u8> LPUART8_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn lpuart8_stop_req_0(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn lpuart8_stop_req_1(self) -> &'a mut W {
        self.variant(LPUART8_STOP_REQ_A::LPUART8_STOP_REQ_1)
    }
}
#[doc = "Field `LPI2C1_STOP_ACK` reader - LPI2C1 stop acknowledge"]
pub type LPI2C1_STOP_ACK_R = crate::BitReader<LPI2C1_STOP_ACK_A>;
#[doc = "LPI2C1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted (the module is in Stop mode)"]
    LPI2C1_STOP_ACK_1 = 1,
}
impl From<LPI2C1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C1_STOP_ACK_A {
        match self.bits {
            false => LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_0,
            true => LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_ack_0(&self) -> bool {
        *self == LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c1_stop_ack_1(&self) -> bool {
        *self == LPI2C1_STOP_ACK_A::LPI2C1_STOP_ACK_1
    }
}
#[doc = "Field `LPI2C2_STOP_ACK` reader - LPI2C2 stop acknowledge"]
pub type LPI2C2_STOP_ACK_R = crate::BitReader<LPI2C2_STOP_ACK_A>;
#[doc = "LPI2C2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C2_STOP_ACK_1 = 1,
}
impl From<LPI2C2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C2_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C2_STOP_ACK_A {
        match self.bits {
            false => LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_0,
            true => LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_ack_0(&self) -> bool {
        *self == LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c2_stop_ack_1(&self) -> bool {
        *self == LPI2C2_STOP_ACK_A::LPI2C2_STOP_ACK_1
    }
}
#[doc = "Field `LPI2C3_STOP_ACK` reader - LPI2C3 stop acknowledge"]
pub type LPI2C3_STOP_ACK_R = crate::BitReader<LPI2C3_STOP_ACK_A>;
#[doc = "LPI2C3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C3_STOP_ACK_1 = 1,
}
impl From<LPI2C3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C3_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C3_STOP_ACK_A {
        match self.bits {
            false => LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_0,
            true => LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_ack_0(&self) -> bool {
        *self == LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c3_stop_ack_1(&self) -> bool {
        *self == LPI2C3_STOP_ACK_A::LPI2C3_STOP_ACK_1
    }
}
#[doc = "Field `LPI2C4_STOP_ACK` reader - LPI2C4 stop acknowledge"]
pub type LPI2C4_STOP_ACK_R = crate::BitReader<LPI2C4_STOP_ACK_A>;
#[doc = "LPI2C4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPI2C4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPI2C4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPI2C4_STOP_ACK_1 = 1,
}
impl From<LPI2C4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPI2C4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPI2C4_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPI2C4_STOP_ACK_A {
        match self.bits {
            false => LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_0,
            true => LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_ack_0(&self) -> bool {
        *self == LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPI2C4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpi2c4_stop_ack_1(&self) -> bool {
        *self == LPI2C4_STOP_ACK_A::LPI2C4_STOP_ACK_1
    }
}
#[doc = "Field `LPSPI1_STOP_ACK` reader - LPSPI1 stop acknowledge"]
pub type LPSPI1_STOP_ACK_R = crate::BitReader<LPSPI1_STOP_ACK_A>;
#[doc = "LPSPI1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI1_STOP_ACK_1 = 1,
}
impl From<LPSPI1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI1_STOP_ACK_A {
        match self.bits {
            false => LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_0,
            true => LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_ack_0(&self) -> bool {
        *self == LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi1_stop_ack_1(&self) -> bool {
        *self == LPSPI1_STOP_ACK_A::LPSPI1_STOP_ACK_1
    }
}
#[doc = "Field `LPSPI2_STOP_ACK` reader - LPSPI2 stop acknowledge"]
pub type LPSPI2_STOP_ACK_R = crate::BitReader<LPSPI2_STOP_ACK_A>;
#[doc = "LPSPI2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI2_STOP_ACK_1 = 1,
}
impl From<LPSPI2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI2_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI2_STOP_ACK_A {
        match self.bits {
            false => LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_0,
            true => LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_ack_0(&self) -> bool {
        *self == LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi2_stop_ack_1(&self) -> bool {
        *self == LPSPI2_STOP_ACK_A::LPSPI2_STOP_ACK_1
    }
}
#[doc = "Field `LPSPI3_STOP_ACK` reader - LPSPI3 stop acknowledge"]
pub type LPSPI3_STOP_ACK_R = crate::BitReader<LPSPI3_STOP_ACK_A>;
#[doc = "LPSPI3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI3_STOP_ACK_1 = 1,
}
impl From<LPSPI3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI3_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI3_STOP_ACK_A {
        match self.bits {
            false => LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_0,
            true => LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_ack_0(&self) -> bool {
        *self == LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi3_stop_ack_1(&self) -> bool {
        *self == LPSPI3_STOP_ACK_A::LPSPI3_STOP_ACK_1
    }
}
#[doc = "Field `LPSPI4_STOP_ACK` reader - LPSPI4 stop acknowledge"]
pub type LPSPI4_STOP_ACK_R = crate::BitReader<LPSPI4_STOP_ACK_A>;
#[doc = "LPSPI4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSPI4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPSPI4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPSPI4_STOP_ACK_1 = 1,
}
impl From<LPSPI4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPSPI4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSPI4_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI4_STOP_ACK_A {
        match self.bits {
            false => LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_0,
            true => LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_ack_0(&self) -> bool {
        *self == LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPSPI4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpspi4_stop_ack_1(&self) -> bool {
        *self == LPSPI4_STOP_ACK_A::LPSPI4_STOP_ACK_1
    }
}
#[doc = "Field `LPUART1_STOP_ACK` reader - LPUART1 stop acknowledge"]
pub type LPUART1_STOP_ACK_R = crate::BitReader<LPUART1_STOP_ACK_A>;
#[doc = "LPUART1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART1_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART1_STOP_ACK_1 = 1,
}
impl From<LPUART1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1_STOP_ACK_A {
        match self.bits {
            false => LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_0,
            true => LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_ack_0(&self) -> bool {
        *self == LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart1_stop_ack_1(&self) -> bool {
        *self == LPUART1_STOP_ACK_A::LPUART1_STOP_ACK_1
    }
}
#[doc = "Field `LPUART2_STOP_ACK` reader - LPUART2 stop acknowledge"]
pub type LPUART2_STOP_ACK_R = crate::BitReader<LPUART2_STOP_ACK_A>;
#[doc = "LPUART2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART2_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART2_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART2_STOP_ACK_1 = 1,
}
impl From<LPUART2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART2_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART2_STOP_ACK_A {
        match self.bits {
            false => LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_0,
            true => LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_ack_0(&self) -> bool {
        *self == LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart2_stop_ack_1(&self) -> bool {
        *self == LPUART2_STOP_ACK_A::LPUART2_STOP_ACK_1
    }
}
#[doc = "Field `LPUART3_STOP_ACK` reader - LPUART3 stop acknowledge"]
pub type LPUART3_STOP_ACK_R = crate::BitReader<LPUART3_STOP_ACK_A>;
#[doc = "LPUART3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART3_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART3_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART3_STOP_ACK_1 = 1,
}
impl From<LPUART3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART3_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART3_STOP_ACK_A {
        match self.bits {
            false => LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_0,
            true => LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_ack_0(&self) -> bool {
        *self == LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart3_stop_ack_1(&self) -> bool {
        *self == LPUART3_STOP_ACK_A::LPUART3_STOP_ACK_1
    }
}
#[doc = "Field `LPUART4_STOP_ACK` reader - LPUART4 stop acknowledge"]
pub type LPUART4_STOP_ACK_R = crate::BitReader<LPUART4_STOP_ACK_A>;
#[doc = "LPUART4 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART4_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART4_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART4_STOP_ACK_1 = 1,
}
impl From<LPUART4_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART4_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART4_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART4_STOP_ACK_A {
        match self.bits {
            false => LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_0,
            true => LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_ack_0(&self) -> bool {
        *self == LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART4_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart4_stop_ack_1(&self) -> bool {
        *self == LPUART4_STOP_ACK_A::LPUART4_STOP_ACK_1
    }
}
#[doc = "Field `LPUART5_STOP_ACK` reader - LPUART5 stop acknowledge"]
pub type LPUART5_STOP_ACK_R = crate::BitReader<LPUART5_STOP_ACK_A>;
#[doc = "LPUART5 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART5_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART5_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART5_STOP_ACK_1 = 1,
}
impl From<LPUART5_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART5_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART5_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART5_STOP_ACK_A {
        match self.bits {
            false => LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_0,
            true => LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_ack_0(&self) -> bool {
        *self == LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART5_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart5_stop_ack_1(&self) -> bool {
        *self == LPUART5_STOP_ACK_A::LPUART5_STOP_ACK_1
    }
}
#[doc = "Field `LPUART6_STOP_ACK` reader - LPUART6 stop acknowledge"]
pub type LPUART6_STOP_ACK_R = crate::BitReader<LPUART6_STOP_ACK_A>;
#[doc = "LPUART6 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART6_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART6_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART6_STOP_ACK_1 = 1,
}
impl From<LPUART6_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART6_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART6_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART6_STOP_ACK_A {
        match self.bits {
            false => LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_0,
            true => LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_ack_0(&self) -> bool {
        *self == LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART6_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart6_stop_ack_1(&self) -> bool {
        *self == LPUART6_STOP_ACK_A::LPUART6_STOP_ACK_1
    }
}
#[doc = "Field `LPUART7_STOP_ACK` reader - LPUART7 stop acknowledge"]
pub type LPUART7_STOP_ACK_R = crate::BitReader<LPUART7_STOP_ACK_A>;
#[doc = "LPUART7 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART7_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART7_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted"]
    LPUART7_STOP_ACK_1 = 1,
}
impl From<LPUART7_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART7_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART7_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART7_STOP_ACK_A {
        match self.bits {
            false => LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_0,
            true => LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_ack_0(&self) -> bool {
        *self == LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART7_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart7_stop_ack_1(&self) -> bool {
        *self == LPUART7_STOP_ACK_A::LPUART7_STOP_ACK_1
    }
}
#[doc = "Field `LPUART8_STOP_ACK` reader - LPUART8 stop acknowledge"]
pub type LPUART8_STOP_ACK_R = crate::BitReader<LPUART8_STOP_ACK_A>;
#[doc = "LPUART8 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART8_STOP_ACK_A {
    #[doc = "0: stop acknowledge is not asserted"]
    LPUART8_STOP_ACK_0 = 0,
    #[doc = "1: stop acknowledge is asserted (the module is in Stop mode)"]
    LPUART8_STOP_ACK_1 = 1,
}
impl From<LPUART8_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART8_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART8_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART8_STOP_ACK_A {
        match self.bits {
            false => LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_0,
            true => LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_ack_0(&self) -> bool {
        *self == LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `LPUART8_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_lpuart8_stop_ack_1(&self) -> bool {
        *self == LPUART8_STOP_ACK_A::LPUART8_STOP_ACK_1
    }
}
impl R {
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline(always)]
    pub fn lpi2c1_stop_req(&self) -> LPI2C1_STOP_REQ_R {
        LPI2C1_STOP_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline(always)]
    pub fn lpi2c2_stop_req(&self) -> LPI2C2_STOP_REQ_R {
        LPI2C2_STOP_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline(always)]
    pub fn lpi2c3_stop_req(&self) -> LPI2C3_STOP_REQ_R {
        LPI2C3_STOP_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline(always)]
    pub fn lpi2c4_stop_req(&self) -> LPI2C4_STOP_REQ_R {
        LPI2C4_STOP_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline(always)]
    pub fn lpspi1_stop_req(&self) -> LPSPI1_STOP_REQ_R {
        LPSPI1_STOP_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline(always)]
    pub fn lpspi2_stop_req(&self) -> LPSPI2_STOP_REQ_R {
        LPSPI2_STOP_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline(always)]
    pub fn lpspi3_stop_req(&self) -> LPSPI3_STOP_REQ_R {
        LPSPI3_STOP_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline(always)]
    pub fn lpspi4_stop_req(&self) -> LPSPI4_STOP_REQ_R {
        LPSPI4_STOP_REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline(always)]
    pub fn lpuart1_stop_req(&self) -> LPUART1_STOP_REQ_R {
        LPUART1_STOP_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPUART2 stop request"]
    #[inline(always)]
    pub fn lpuart2_stop_req(&self) -> LPUART2_STOP_REQ_R {
        LPUART2_STOP_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline(always)]
    pub fn lpuart3_stop_req(&self) -> LPUART3_STOP_REQ_R {
        LPUART3_STOP_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline(always)]
    pub fn lpuart4_stop_req(&self) -> LPUART4_STOP_REQ_R {
        LPUART4_STOP_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline(always)]
    pub fn lpuart5_stop_req(&self) -> LPUART5_STOP_REQ_R {
        LPUART5_STOP_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline(always)]
    pub fn lpuart6_stop_req(&self) -> LPUART6_STOP_REQ_R {
        LPUART6_STOP_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline(always)]
    pub fn lpuart7_stop_req(&self) -> LPUART7_STOP_REQ_R {
        LPUART7_STOP_REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline(always)]
    pub fn lpuart8_stop_req(&self) -> LPUART8_STOP_REQ_R {
        LPUART8_STOP_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c1_stop_ack(&self) -> LPI2C1_STOP_ACK_R {
        LPI2C1_STOP_ACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c2_stop_ack(&self) -> LPI2C2_STOP_ACK_R {
        LPI2C2_STOP_ACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPI2C3 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c3_stop_ack(&self) -> LPI2C3_STOP_ACK_R {
        LPI2C3_STOP_ACK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI2C4 stop acknowledge"]
    #[inline(always)]
    pub fn lpi2c4_stop_ack(&self) -> LPI2C4_STOP_ACK_R {
        LPI2C4_STOP_ACK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi1_stop_ack(&self) -> LPSPI1_STOP_ACK_R {
        LPSPI1_STOP_ACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi2_stop_ack(&self) -> LPSPI2_STOP_ACK_R {
        LPSPI2_STOP_ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LPSPI3 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi3_stop_ack(&self) -> LPSPI3_STOP_ACK_R {
        LPSPI3_STOP_ACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPSPI4 stop acknowledge"]
    #[inline(always)]
    pub fn lpspi4_stop_ack(&self) -> LPSPI4_STOP_ACK_R {
        LPSPI4_STOP_ACK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LPUART1 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart1_stop_ack(&self) -> LPUART1_STOP_ACK_R {
        LPUART1_STOP_ACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LPUART2 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart2_stop_ack(&self) -> LPUART2_STOP_ACK_R {
        LPUART2_STOP_ACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPUART3 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart3_stop_ack(&self) -> LPUART3_STOP_ACK_R {
        LPUART3_STOP_ACK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LPUART4 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart4_stop_ack(&self) -> LPUART4_STOP_ACK_R {
        LPUART4_STOP_ACK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LPUART5 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart5_stop_ack(&self) -> LPUART5_STOP_ACK_R {
        LPUART5_STOP_ACK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LPUART6 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart6_stop_ack(&self) -> LPUART6_STOP_ACK_R {
        LPUART6_STOP_ACK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPUART7 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart7_stop_ack(&self) -> LPUART7_STOP_ACK_R {
        LPUART7_STOP_ACK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPUART8 stop acknowledge"]
    #[inline(always)]
    pub fn lpuart8_stop_ack(&self) -> LPUART8_STOP_ACK_R {
        LPUART8_STOP_ACK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPI2C1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c1_stop_req(&mut self) -> LPI2C1_STOP_REQ_W<0> {
        LPI2C1_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 1 - LPI2C2 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c2_stop_req(&mut self) -> LPI2C2_STOP_REQ_W<1> {
        LPI2C2_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 2 - LPI2C3 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c3_stop_req(&mut self) -> LPI2C3_STOP_REQ_W<2> {
        LPI2C3_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 3 - LPI2C4 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpi2c4_stop_req(&mut self) -> LPI2C4_STOP_REQ_W<3> {
        LPI2C4_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 4 - LPSPI1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi1_stop_req(&mut self) -> LPSPI1_STOP_REQ_W<4> {
        LPSPI1_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 5 - LPSPI2 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi2_stop_req(&mut self) -> LPSPI2_STOP_REQ_W<5> {
        LPSPI2_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 6 - LPSPI3 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi3_stop_req(&mut self) -> LPSPI3_STOP_REQ_W<6> {
        LPSPI3_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 7 - LPSPI4 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi4_stop_req(&mut self) -> LPSPI4_STOP_REQ_W<7> {
        LPSPI4_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 8 - LPUART1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1_stop_req(&mut self) -> LPUART1_STOP_REQ_W<8> {
        LPUART1_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 9 - LPUART2 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2_stop_req(&mut self) -> LPUART2_STOP_REQ_W<9> {
        LPUART2_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 10 - LPUART3 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart3_stop_req(&mut self) -> LPUART3_STOP_REQ_W<10> {
        LPUART3_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 11 - LPUART4 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart4_stop_req(&mut self) -> LPUART4_STOP_REQ_W<11> {
        LPUART4_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 12 - LPUART5 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart5_stop_req(&mut self) -> LPUART5_STOP_REQ_W<12> {
        LPUART5_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 13 - LPUART6 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart6_stop_req(&mut self) -> LPUART6_STOP_REQ_W<13> {
        LPUART6_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 14 - LPUART7 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart7_stop_req(&mut self) -> LPUART7_STOP_REQ_W<14> {
        LPUART7_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 15 - LPUART8 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart8_stop_req(&mut self) -> LPUART8_STOP_REQ_W<15> {
        LPUART8_STOP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR7 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr7](index.html) module"]
pub struct GPR7_SPEC;
impl crate::RegisterSpec for GPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr7::R](R) reader structure"]
impl crate::Readable for GPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr7::W](W) writer structure"]
impl crate::Writable for GPR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR7 to value 0"]
impl crate::Resettable for GPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
