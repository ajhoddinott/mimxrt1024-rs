#[doc = "Register `GPR4` reader"]
pub struct R(crate::R<GPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR4` writer"]
pub struct W(crate::W<GPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR4_SPEC>;
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
impl From<crate::W<GPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_STOP_REQ` reader - EDMA stop request"]
pub type EDMA_STOP_REQ_R = crate::BitReader<EDMA_STOP_REQ_A>;
#[doc = "EDMA stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDMA_STOP_REQ_A {
    #[doc = "0: stop request off"]
    EDMA_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    EDMA_STOP_REQ_1 = 1,
}
impl From<EDMA_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: EDMA_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl EDMA_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDMA_STOP_REQ_A {
        match self.bits {
            false => EDMA_STOP_REQ_A::EDMA_STOP_REQ_0,
            true => EDMA_STOP_REQ_A::EDMA_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_edma_stop_req_0(&self) -> bool {
        *self == EDMA_STOP_REQ_A::EDMA_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_edma_stop_req_1(&self) -> bool {
        *self == EDMA_STOP_REQ_A::EDMA_STOP_REQ_1
    }
}
#[doc = "Field `EDMA_STOP_REQ` writer - EDMA stop request"]
pub type EDMA_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, EDMA_STOP_REQ_A, O>;
impl<'a, const O: u8> EDMA_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn edma_stop_req_0(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQ_A::EDMA_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn edma_stop_req_1(self) -> &'a mut W {
        self.variant(EDMA_STOP_REQ_A::EDMA_STOP_REQ_1)
    }
}
#[doc = "Field `CAN1_STOP_REQ` reader - CAN1 stop request"]
pub type CAN1_STOP_REQ_R = crate::BitReader<CAN1_STOP_REQ_A>;
#[doc = "CAN1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    CAN1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    CAN1_STOP_REQ_1 = 1,
}
impl From<CAN1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN1_STOP_REQ_A {
        match self.bits {
            false => CAN1_STOP_REQ_A::CAN1_STOP_REQ_0,
            true => CAN1_STOP_REQ_A::CAN1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_can1_stop_req_0(&self) -> bool {
        *self == CAN1_STOP_REQ_A::CAN1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_can1_stop_req_1(&self) -> bool {
        *self == CAN1_STOP_REQ_A::CAN1_STOP_REQ_1
    }
}
#[doc = "Field `CAN1_STOP_REQ` writer - CAN1 stop request"]
pub type CAN1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, CAN1_STOP_REQ_A, O>;
impl<'a, const O: u8> CAN1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn can1_stop_req_0(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQ_A::CAN1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn can1_stop_req_1(self) -> &'a mut W {
        self.variant(CAN1_STOP_REQ_A::CAN1_STOP_REQ_1)
    }
}
#[doc = "Field `CAN2_STOP_REQ` reader - CAN2 stop request"]
pub type CAN2_STOP_REQ_R = crate::BitReader<CAN2_STOP_REQ_A>;
#[doc = "CAN2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    CAN2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    CAN2_STOP_REQ_1 = 1,
}
impl From<CAN2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CAN2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN2_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN2_STOP_REQ_A {
        match self.bits {
            false => CAN2_STOP_REQ_A::CAN2_STOP_REQ_0,
            true => CAN2_STOP_REQ_A::CAN2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_can2_stop_req_0(&self) -> bool {
        *self == CAN2_STOP_REQ_A::CAN2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_can2_stop_req_1(&self) -> bool {
        *self == CAN2_STOP_REQ_A::CAN2_STOP_REQ_1
    }
}
#[doc = "Field `CAN2_STOP_REQ` writer - CAN2 stop request"]
pub type CAN2_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, CAN2_STOP_REQ_A, O>;
impl<'a, const O: u8> CAN2_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn can2_stop_req_0(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQ_A::CAN2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn can2_stop_req_1(self) -> &'a mut W {
        self.variant(CAN2_STOP_REQ_A::CAN2_STOP_REQ_1)
    }
}
#[doc = "Field `TRNG_STOP_REQ` reader - TRNG stop request"]
pub type TRNG_STOP_REQ_R = crate::BitReader<TRNG_STOP_REQ_A>;
#[doc = "TRNG stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNG_STOP_REQ_A {
    #[doc = "0: stop request off"]
    TRNG_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    TRNG_STOP_REQ_1 = 1,
}
impl From<TRNG_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNG_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_STOP_REQ_A {
        match self.bits {
            false => TRNG_STOP_REQ_A::TRNG_STOP_REQ_0,
            true => TRNG_STOP_REQ_A::TRNG_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_trng_stop_req_0(&self) -> bool {
        *self == TRNG_STOP_REQ_A::TRNG_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_trng_stop_req_1(&self) -> bool {
        *self == TRNG_STOP_REQ_A::TRNG_STOP_REQ_1
    }
}
#[doc = "Field `TRNG_STOP_REQ` writer - TRNG stop request"]
pub type TRNG_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, TRNG_STOP_REQ_A, O>;
impl<'a, const O: u8> TRNG_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn trng_stop_req_0(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQ_A::TRNG_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn trng_stop_req_1(self) -> &'a mut W {
        self.variant(TRNG_STOP_REQ_A::TRNG_STOP_REQ_1)
    }
}
#[doc = "Field `ENET_STOP_REQ` reader - ENET stop request"]
pub type ENET_STOP_REQ_R = crate::BitReader<ENET_STOP_REQ_A>;
#[doc = "ENET stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_STOP_REQ_A {
    #[doc = "0: stop request off"]
    ENET_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    ENET_STOP_REQ_1 = 1,
}
impl From<ENET_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_STOP_REQ_A {
        match self.bits {
            false => ENET_STOP_REQ_A::ENET_STOP_REQ_0,
            true => ENET_STOP_REQ_A::ENET_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_enet_stop_req_0(&self) -> bool {
        *self == ENET_STOP_REQ_A::ENET_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_enet_stop_req_1(&self) -> bool {
        *self == ENET_STOP_REQ_A::ENET_STOP_REQ_1
    }
}
#[doc = "Field `ENET_STOP_REQ` writer - ENET stop request"]
pub type ENET_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, ENET_STOP_REQ_A, O>;
impl<'a, const O: u8> ENET_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn enet_stop_req_0(self) -> &'a mut W {
        self.variant(ENET_STOP_REQ_A::ENET_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn enet_stop_req_1(self) -> &'a mut W {
        self.variant(ENET_STOP_REQ_A::ENET_STOP_REQ_1)
    }
}
#[doc = "Field `SAI1_STOP_REQ` reader - SAI1 stop request"]
pub type SAI1_STOP_REQ_R = crate::BitReader<SAI1_STOP_REQ_A>;
#[doc = "SAI1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    SAI1_STOP_REQ_1 = 1,
}
impl From<SAI1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_STOP_REQ_A {
        match self.bits {
            false => SAI1_STOP_REQ_A::SAI1_STOP_REQ_0,
            true => SAI1_STOP_REQ_A::SAI1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai1_stop_req_0(&self) -> bool {
        *self == SAI1_STOP_REQ_A::SAI1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai1_stop_req_1(&self) -> bool {
        *self == SAI1_STOP_REQ_A::SAI1_STOP_REQ_1
    }
}
#[doc = "Field `SAI1_STOP_REQ` writer - SAI1 stop request"]
pub type SAI1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, SAI1_STOP_REQ_A, O>;
impl<'a, const O: u8> SAI1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai1_stop_req_0(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQ_A::SAI1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai1_stop_req_1(self) -> &'a mut W {
        self.variant(SAI1_STOP_REQ_A::SAI1_STOP_REQ_1)
    }
}
#[doc = "Field `SAI2_STOP_REQ` reader - SAI2 stop request"]
pub type SAI2_STOP_REQ_R = crate::BitReader<SAI2_STOP_REQ_A>;
#[doc = "SAI2 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI2_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI2_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    SAI2_STOP_REQ_1 = 1,
}
impl From<SAI2_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI2_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_STOP_REQ_A {
        match self.bits {
            false => SAI2_STOP_REQ_A::SAI2_STOP_REQ_0,
            true => SAI2_STOP_REQ_A::SAI2_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai2_stop_req_0(&self) -> bool {
        *self == SAI2_STOP_REQ_A::SAI2_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai2_stop_req_1(&self) -> bool {
        *self == SAI2_STOP_REQ_A::SAI2_STOP_REQ_1
    }
}
#[doc = "Field `SAI2_STOP_REQ` writer - SAI2 stop request"]
pub type SAI2_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, SAI2_STOP_REQ_A, O>;
impl<'a, const O: u8> SAI2_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai2_stop_req_0(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQ_A::SAI2_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai2_stop_req_1(self) -> &'a mut W {
        self.variant(SAI2_STOP_REQ_A::SAI2_STOP_REQ_1)
    }
}
#[doc = "Field `SAI3_STOP_REQ` reader - SAI3 stop request"]
pub type SAI3_STOP_REQ_R = crate::BitReader<SAI3_STOP_REQ_A>;
#[doc = "SAI3 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI3_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SAI3_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    SAI3_STOP_REQ_1 = 1,
}
impl From<SAI3_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI3_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_STOP_REQ_A {
        match self.bits {
            false => SAI3_STOP_REQ_A::SAI3_STOP_REQ_0,
            true => SAI3_STOP_REQ_A::SAI3_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_sai3_stop_req_0(&self) -> bool {
        *self == SAI3_STOP_REQ_A::SAI3_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_sai3_stop_req_1(&self) -> bool {
        *self == SAI3_STOP_REQ_A::SAI3_STOP_REQ_1
    }
}
#[doc = "Field `SAI3_STOP_REQ` writer - SAI3 stop request"]
pub type SAI3_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, SAI3_STOP_REQ_A, O>;
impl<'a, const O: u8> SAI3_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn sai3_stop_req_0(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQ_A::SAI3_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn sai3_stop_req_1(self) -> &'a mut W {
        self.variant(SAI3_STOP_REQ_A::SAI3_STOP_REQ_1)
    }
}
#[doc = "Field `SEMC_STOP_REQ` reader - SEMC stop request"]
pub type SEMC_STOP_REQ_R = crate::BitReader<SEMC_STOP_REQ_A>;
#[doc = "SEMC stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_STOP_REQ_A {
    #[doc = "0: stop request off"]
    SEMC_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    SEMC_STOP_REQ_1 = 1,
}
impl From<SEMC_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_STOP_REQ_A {
        match self.bits {
            false => SEMC_STOP_REQ_A::SEMC_STOP_REQ_0,
            true => SEMC_STOP_REQ_A::SEMC_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_semc_stop_req_0(&self) -> bool {
        *self == SEMC_STOP_REQ_A::SEMC_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_semc_stop_req_1(&self) -> bool {
        *self == SEMC_STOP_REQ_A::SEMC_STOP_REQ_1
    }
}
#[doc = "Field `SEMC_STOP_REQ` writer - SEMC stop request"]
pub type SEMC_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, SEMC_STOP_REQ_A, O>;
impl<'a, const O: u8> SEMC_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn semc_stop_req_0(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQ_A::SEMC_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn semc_stop_req_1(self) -> &'a mut W {
        self.variant(SEMC_STOP_REQ_A::SEMC_STOP_REQ_1)
    }
}
#[doc = "Field `PIT_STOP_REQ` reader - PIT stop request"]
pub type PIT_STOP_REQ_R = crate::BitReader<PIT_STOP_REQ_A>;
#[doc = "PIT stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIT_STOP_REQ_A {
    #[doc = "0: stop request off"]
    PIT_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    PIT_STOP_REQ_1 = 1,
}
impl From<PIT_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl PIT_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_STOP_REQ_A {
        match self.bits {
            false => PIT_STOP_REQ_A::PIT_STOP_REQ_0,
            true => PIT_STOP_REQ_A::PIT_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_pit_stop_req_0(&self) -> bool {
        *self == PIT_STOP_REQ_A::PIT_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_pit_stop_req_1(&self) -> bool {
        *self == PIT_STOP_REQ_A::PIT_STOP_REQ_1
    }
}
#[doc = "Field `PIT_STOP_REQ` writer - PIT stop request"]
pub type PIT_STOP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR4_SPEC, PIT_STOP_REQ_A, O>;
impl<'a, const O: u8> PIT_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn pit_stop_req_0(self) -> &'a mut W {
        self.variant(PIT_STOP_REQ_A::PIT_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn pit_stop_req_1(self) -> &'a mut W {
        self.variant(PIT_STOP_REQ_A::PIT_STOP_REQ_1)
    }
}
#[doc = "Field `FLEXSPI_STOP_REQ` reader - FlexSPI stop request"]
pub type FLEXSPI_STOP_REQ_R = crate::BitReader<FLEXSPI_STOP_REQ_A>;
#[doc = "FlexSPI stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXSPI_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    FLEXSPI_STOP_REQ_1 = 1,
}
impl From<FLEXSPI_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_STOP_REQ_A {
        match self.bits {
            false => FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0,
            true => FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexspi_stop_req_0(&self) -> bool {
        *self == FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexspi_stop_req_1(&self) -> bool {
        *self == FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1
    }
}
#[doc = "Field `FLEXSPI_STOP_REQ` writer - FlexSPI stop request"]
pub type FLEXSPI_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, FLEXSPI_STOP_REQ_A, O>;
impl<'a, const O: u8> FLEXSPI_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexspi_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexspi_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXSPI_STOP_REQ_A::FLEXSPI_STOP_REQ_1)
    }
}
#[doc = "Field `FLEXIO1_STOP_REQ` reader - FlexIO1 stop request"]
pub type FLEXIO1_STOP_REQ_R = crate::BitReader<FLEXIO1_STOP_REQ_A>;
#[doc = "FlexIO1 stop request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO1_STOP_REQ_A {
    #[doc = "0: stop request off"]
    FLEXIO1_STOP_REQ_0 = 0,
    #[doc = "1: stop request on"]
    FLEXIO1_STOP_REQ_1 = 1,
}
impl From<FLEXIO1_STOP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_STOP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXIO1_STOP_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_STOP_REQ_A {
        match self.bits {
            false => FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0,
            true => FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_0`"]
    #[inline(always)]
    pub fn is_flexio1_stop_req_0(&self) -> bool {
        *self == FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_REQ_1`"]
    #[inline(always)]
    pub fn is_flexio1_stop_req_1(&self) -> bool {
        *self == FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1
    }
}
#[doc = "Field `FLEXIO1_STOP_REQ` writer - FlexIO1 stop request"]
pub type FLEXIO1_STOP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR4_SPEC, FLEXIO1_STOP_REQ_A, O>;
impl<'a, const O: u8> FLEXIO1_STOP_REQ_W<'a, O> {
    #[doc = "stop request off"]
    #[inline(always)]
    pub fn flexio1_stop_req_0(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_0)
    }
    #[doc = "stop request on"]
    #[inline(always)]
    pub fn flexio1_stop_req_1(self) -> &'a mut W {
        self.variant(FLEXIO1_STOP_REQ_A::FLEXIO1_STOP_REQ_1)
    }
}
#[doc = "Field `EDMA_STOP_ACK` reader - EDMA stop acknowledge"]
pub type EDMA_STOP_ACK_R = crate::BitReader<EDMA_STOP_ACK_A>;
#[doc = "EDMA stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDMA_STOP_ACK_A {
    #[doc = "0: EDMA stop acknowledge is not asserted"]
    EDMA_STOP_ACK_0 = 0,
    #[doc = "1: EDMA stop acknowledge is asserted (EDMA is in STOP mode)"]
    EDMA_STOP_ACK_1 = 1,
}
impl From<EDMA_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: EDMA_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl EDMA_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDMA_STOP_ACK_A {
        match self.bits {
            false => EDMA_STOP_ACK_A::EDMA_STOP_ACK_0,
            true => EDMA_STOP_ACK_A::EDMA_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_edma_stop_ack_0(&self) -> bool {
        *self == EDMA_STOP_ACK_A::EDMA_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `EDMA_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_edma_stop_ack_1(&self) -> bool {
        *self == EDMA_STOP_ACK_A::EDMA_STOP_ACK_1
    }
}
#[doc = "Field `CAN1_STOP_ACK` reader - CAN1 stop acknowledge"]
pub type CAN1_STOP_ACK_R = crate::BitReader<CAN1_STOP_ACK_A>;
#[doc = "CAN1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN1_STOP_ACK_A {
    #[doc = "0: CAN1 stop acknowledge is not asserted"]
    CAN1_STOP_ACK_0 = 0,
    #[doc = "1: CAN1 stop acknowledge is asserted"]
    CAN1_STOP_ACK_1 = 1,
}
impl From<CAN1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: CAN1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN1_STOP_ACK_A {
        match self.bits {
            false => CAN1_STOP_ACK_A::CAN1_STOP_ACK_0,
            true => CAN1_STOP_ACK_A::CAN1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_can1_stop_ack_0(&self) -> bool {
        *self == CAN1_STOP_ACK_A::CAN1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_can1_stop_ack_1(&self) -> bool {
        *self == CAN1_STOP_ACK_A::CAN1_STOP_ACK_1
    }
}
#[doc = "Field `CAN2_STOP_ACK` reader - CAN2 stop acknowledge"]
pub type CAN2_STOP_ACK_R = crate::BitReader<CAN2_STOP_ACK_A>;
#[doc = "CAN2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAN2_STOP_ACK_A {
    #[doc = "0: CAN2 stop acknowledge is not asserted"]
    CAN2_STOP_ACK_0 = 0,
    #[doc = "1: CAN2 stop acknowledge is asserted"]
    CAN2_STOP_ACK_1 = 1,
}
impl From<CAN2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: CAN2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl CAN2_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN2_STOP_ACK_A {
        match self.bits {
            false => CAN2_STOP_ACK_A::CAN2_STOP_ACK_0,
            true => CAN2_STOP_ACK_A::CAN2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_can2_stop_ack_0(&self) -> bool {
        *self == CAN2_STOP_ACK_A::CAN2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `CAN2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_can2_stop_ack_1(&self) -> bool {
        *self == CAN2_STOP_ACK_A::CAN2_STOP_ACK_1
    }
}
#[doc = "Field `TRNG_STOP_ACK` reader - TRNG stop acknowledge"]
pub type TRNG_STOP_ACK_R = crate::BitReader<TRNG_STOP_ACK_A>;
#[doc = "TRNG stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRNG_STOP_ACK_A {
    #[doc = "0: TRNG stop acknowledge is not asserted"]
    TRNG_STOP_ACK_0 = 0,
    #[doc = "1: TRNG stop acknowledge is asserted"]
    TRNG_STOP_ACK_1 = 1,
}
impl From<TRNG_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: TRNG_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl TRNG_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRNG_STOP_ACK_A {
        match self.bits {
            false => TRNG_STOP_ACK_A::TRNG_STOP_ACK_0,
            true => TRNG_STOP_ACK_A::TRNG_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_trng_stop_ack_0(&self) -> bool {
        *self == TRNG_STOP_ACK_A::TRNG_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `TRNG_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_trng_stop_ack_1(&self) -> bool {
        *self == TRNG_STOP_ACK_A::TRNG_STOP_ACK_1
    }
}
#[doc = "Field `ENET_STOP_ACK` reader - ENET stop acknowledge"]
pub type ENET_STOP_ACK_R = crate::BitReader<ENET_STOP_ACK_A>;
#[doc = "ENET stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_STOP_ACK_A {
    #[doc = "0: ENET stop acknowledge is not asserted"]
    ENET_STOP_ACK_0 = 0,
    #[doc = "1: ENET stop acknowledge is asserted"]
    ENET_STOP_ACK_1 = 1,
}
impl From<ENET_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_STOP_ACK_A {
        match self.bits {
            false => ENET_STOP_ACK_A::ENET_STOP_ACK_0,
            true => ENET_STOP_ACK_A::ENET_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_enet_stop_ack_0(&self) -> bool {
        *self == ENET_STOP_ACK_A::ENET_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `ENET_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_enet_stop_ack_1(&self) -> bool {
        *self == ENET_STOP_ACK_A::ENET_STOP_ACK_1
    }
}
#[doc = "Field `SAI1_STOP_ACK` reader - SAI1 stop acknowledge"]
pub type SAI1_STOP_ACK_R = crate::BitReader<SAI1_STOP_ACK_A>;
#[doc = "SAI1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1_STOP_ACK_A {
    #[doc = "0: SAI1 stop acknowledge is not asserted"]
    SAI1_STOP_ACK_0 = 0,
    #[doc = "1: SAI1 stop acknowledge is asserted"]
    SAI1_STOP_ACK_1 = 1,
}
impl From<SAI1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_STOP_ACK_A {
        match self.bits {
            false => SAI1_STOP_ACK_A::SAI1_STOP_ACK_0,
            true => SAI1_STOP_ACK_A::SAI1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai1_stop_ack_0(&self) -> bool {
        *self == SAI1_STOP_ACK_A::SAI1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai1_stop_ack_1(&self) -> bool {
        *self == SAI1_STOP_ACK_A::SAI1_STOP_ACK_1
    }
}
#[doc = "Field `SAI2_STOP_ACK` reader - SAI2 stop acknowledge"]
pub type SAI2_STOP_ACK_R = crate::BitReader<SAI2_STOP_ACK_A>;
#[doc = "SAI2 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI2_STOP_ACK_A {
    #[doc = "0: SAI2 stop acknowledge is not asserted"]
    SAI2_STOP_ACK_0 = 0,
    #[doc = "1: SAI2 stop acknowledge is asserted"]
    SAI2_STOP_ACK_1 = 1,
}
impl From<SAI2_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI2_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_STOP_ACK_A {
        match self.bits {
            false => SAI2_STOP_ACK_A::SAI2_STOP_ACK_0,
            true => SAI2_STOP_ACK_A::SAI2_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai2_stop_ack_0(&self) -> bool {
        *self == SAI2_STOP_ACK_A::SAI2_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI2_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai2_stop_ack_1(&self) -> bool {
        *self == SAI2_STOP_ACK_A::SAI2_STOP_ACK_1
    }
}
#[doc = "Field `SAI3_STOP_ACK` reader - SAI3 stop acknowledge"]
pub type SAI3_STOP_ACK_R = crate::BitReader<SAI3_STOP_ACK_A>;
#[doc = "SAI3 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI3_STOP_ACK_A {
    #[doc = "0: SAI3 stop acknowledge is not asserted"]
    SAI3_STOP_ACK_0 = 0,
    #[doc = "1: SAI3 stop acknowledge is asserted"]
    SAI3_STOP_ACK_1 = 1,
}
impl From<SAI3_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI3_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_STOP_ACK_A {
        match self.bits {
            false => SAI3_STOP_ACK_A::SAI3_STOP_ACK_0,
            true => SAI3_STOP_ACK_A::SAI3_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_sai3_stop_ack_0(&self) -> bool {
        *self == SAI3_STOP_ACK_A::SAI3_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SAI3_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_sai3_stop_ack_1(&self) -> bool {
        *self == SAI3_STOP_ACK_A::SAI3_STOP_ACK_1
    }
}
#[doc = "Field `SEMC_STOP_ACK` reader - SEMC stop acknowledge"]
pub type SEMC_STOP_ACK_R = crate::BitReader<SEMC_STOP_ACK_A>;
#[doc = "SEMC stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_STOP_ACK_A {
    #[doc = "0: SEMC stop acknowledge is not asserted"]
    SEMC_STOP_ACK_0 = 0,
    #[doc = "1: SEMC stop acknowledge is asserted"]
    SEMC_STOP_ACK_1 = 1,
}
impl From<SEMC_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_STOP_ACK_A {
        match self.bits {
            false => SEMC_STOP_ACK_A::SEMC_STOP_ACK_0,
            true => SEMC_STOP_ACK_A::SEMC_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_semc_stop_ack_0(&self) -> bool {
        *self == SEMC_STOP_ACK_A::SEMC_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `SEMC_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_semc_stop_ack_1(&self) -> bool {
        *self == SEMC_STOP_ACK_A::SEMC_STOP_ACK_1
    }
}
#[doc = "Field `PIT_STOP_ACK` reader - PIT stop acknowledge"]
pub type PIT_STOP_ACK_R = crate::BitReader<PIT_STOP_ACK_A>;
#[doc = "PIT stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIT_STOP_ACK_A {
    #[doc = "0: PIT stop acknowledge is not asserted"]
    PIT_STOP_ACK_0 = 0,
    #[doc = "1: PIT stop acknowledge is asserted"]
    PIT_STOP_ACK_1 = 1,
}
impl From<PIT_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIT_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_STOP_ACK_A {
        match self.bits {
            false => PIT_STOP_ACK_A::PIT_STOP_ACK_0,
            true => PIT_STOP_ACK_A::PIT_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_pit_stop_ack_0(&self) -> bool {
        *self == PIT_STOP_ACK_A::PIT_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `PIT_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_pit_stop_ack_1(&self) -> bool {
        *self == PIT_STOP_ACK_A::PIT_STOP_ACK_1
    }
}
#[doc = "Field `FLEXSPI_STOP_ACK` reader - FLEXSPI stop acknowledge"]
pub type FLEXSPI_STOP_ACK_R = crate::BitReader<FLEXSPI_STOP_ACK_A>;
#[doc = "FLEXSPI stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXSPI_STOP_ACK_A {
    #[doc = "0: FLEXSPI stop acknowledge is not asserted"]
    FLEXSPI_STOP_ACK_0 = 0,
    #[doc = "1: FLEXSPI stop acknowledge is asserted"]
    FLEXSPI_STOP_ACK_1 = 1,
}
impl From<FLEXSPI_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXSPI_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXSPI_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI_STOP_ACK_A {
        match self.bits {
            false => FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_0,
            true => FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexspi_stop_ack_0(&self) -> bool {
        *self == FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexspi_stop_ack_1(&self) -> bool {
        *self == FLEXSPI_STOP_ACK_A::FLEXSPI_STOP_ACK_1
    }
}
#[doc = "Field `FLEXIO1_STOP_ACK` reader - FLEXIO1 stop acknowledge"]
pub type FLEXIO1_STOP_ACK_R = crate::BitReader<FLEXIO1_STOP_ACK_A>;
#[doc = "FLEXIO1 stop acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXIO1_STOP_ACK_A {
    #[doc = "0: FLEXIO1 stop acknowledge is not asserted"]
    FLEXIO1_STOP_ACK_0 = 0,
    #[doc = "1: FLEXIO1 stop acknowledge is asserted"]
    FLEXIO1_STOP_ACK_1 = 1,
}
impl From<FLEXIO1_STOP_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXIO1_STOP_ACK_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXIO1_STOP_ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXIO1_STOP_ACK_A {
        match self.bits {
            false => FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_0,
            true => FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_0`"]
    #[inline(always)]
    pub fn is_flexio1_stop_ack_0(&self) -> bool {
        *self == FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_0
    }
    #[doc = "Checks if the value of the field is `FLEXIO1_STOP_ACK_1`"]
    #[inline(always)]
    pub fn is_flexio1_stop_ack_1(&self) -> bool {
        *self == FLEXIO1_STOP_ACK_A::FLEXIO1_STOP_ACK_1
    }
}
impl R {
    #[doc = "Bit 0 - EDMA stop request"]
    #[inline(always)]
    pub fn edma_stop_req(&self) -> EDMA_STOP_REQ_R {
        EDMA_STOP_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAN1 stop request"]
    #[inline(always)]
    pub fn can1_stop_req(&self) -> CAN1_STOP_REQ_R {
        CAN1_STOP_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAN2 stop request"]
    #[inline(always)]
    pub fn can2_stop_req(&self) -> CAN2_STOP_REQ_R {
        CAN2_STOP_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRNG stop request"]
    #[inline(always)]
    pub fn trng_stop_req(&self) -> TRNG_STOP_REQ_R {
        TRNG_STOP_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ENET stop request"]
    #[inline(always)]
    pub fn enet_stop_req(&self) -> ENET_STOP_REQ_R {
        ENET_STOP_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1 stop request"]
    #[inline(always)]
    pub fn sai1_stop_req(&self) -> SAI1_STOP_REQ_R {
        SAI1_STOP_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2 stop request"]
    #[inline(always)]
    pub fn sai2_stop_req(&self) -> SAI2_STOP_REQ_R {
        SAI2_STOP_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SAI3 stop request"]
    #[inline(always)]
    pub fn sai3_stop_req(&self) -> SAI3_STOP_REQ_R {
        SAI3_STOP_REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SEMC stop request"]
    #[inline(always)]
    pub fn semc_stop_req(&self) -> SEMC_STOP_REQ_R {
        SEMC_STOP_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PIT stop request"]
    #[inline(always)]
    pub fn pit_stop_req(&self) -> PIT_STOP_REQ_R {
        PIT_STOP_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FlexSPI stop request"]
    #[inline(always)]
    pub fn flexspi_stop_req(&self) -> FLEXSPI_STOP_REQ_R {
        FLEXSPI_STOP_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FlexIO1 stop request"]
    #[inline(always)]
    pub fn flexio1_stop_req(&self) -> FLEXIO1_STOP_REQ_R {
        FLEXIO1_STOP_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - EDMA stop acknowledge"]
    #[inline(always)]
    pub fn edma_stop_ack(&self) -> EDMA_STOP_ACK_R {
        EDMA_STOP_ACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CAN1 stop acknowledge"]
    #[inline(always)]
    pub fn can1_stop_ack(&self) -> CAN1_STOP_ACK_R {
        CAN1_STOP_ACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAN2 stop acknowledge"]
    #[inline(always)]
    pub fn can2_stop_ack(&self) -> CAN2_STOP_ACK_R {
        CAN2_STOP_ACK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRNG stop acknowledge"]
    #[inline(always)]
    pub fn trng_stop_ack(&self) -> TRNG_STOP_ACK_R {
        TRNG_STOP_ACK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ENET stop acknowledge"]
    #[inline(always)]
    pub fn enet_stop_ack(&self) -> ENET_STOP_ACK_R {
        ENET_STOP_ACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 stop acknowledge"]
    #[inline(always)]
    pub fn sai1_stop_ack(&self) -> SAI1_STOP_ACK_R {
        SAI1_STOP_ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 stop acknowledge"]
    #[inline(always)]
    pub fn sai2_stop_ack(&self) -> SAI2_STOP_ACK_R {
        SAI2_STOP_ACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI3 stop acknowledge"]
    #[inline(always)]
    pub fn sai3_stop_ack(&self) -> SAI3_STOP_ACK_R {
        SAI3_STOP_ACK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - SEMC stop acknowledge"]
    #[inline(always)]
    pub fn semc_stop_ack(&self) -> SEMC_STOP_ACK_R {
        SEMC_STOP_ACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIT stop acknowledge"]
    #[inline(always)]
    pub fn pit_stop_ack(&self) -> PIT_STOP_ACK_R {
        PIT_STOP_ACK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub fn flexspi_stop_ack(&self) -> FLEXSPI_STOP_ACK_R {
        FLEXSPI_STOP_ACK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub fn flexio1_stop_ack(&self) -> FLEXIO1_STOP_ACK_R {
        FLEXIO1_STOP_ACK_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EDMA stop request"]
    #[inline(always)]
    #[must_use]
    pub fn edma_stop_req(&mut self) -> EDMA_STOP_REQ_W<0> {
        EDMA_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 1 - CAN1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn can1_stop_req(&mut self) -> CAN1_STOP_REQ_W<1> {
        CAN1_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 2 - CAN2 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn can2_stop_req(&mut self) -> CAN2_STOP_REQ_W<2> {
        CAN2_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 3 - TRNG stop request"]
    #[inline(always)]
    #[must_use]
    pub fn trng_stop_req(&mut self) -> TRNG_STOP_REQ_W<3> {
        TRNG_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 4 - ENET stop request"]
    #[inline(always)]
    #[must_use]
    pub fn enet_stop_req(&mut self) -> ENET_STOP_REQ_W<4> {
        ENET_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 5 - SAI1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn sai1_stop_req(&mut self) -> SAI1_STOP_REQ_W<5> {
        SAI1_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 6 - SAI2 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn sai2_stop_req(&mut self) -> SAI2_STOP_REQ_W<6> {
        SAI2_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 7 - SAI3 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn sai3_stop_req(&mut self) -> SAI3_STOP_REQ_W<7> {
        SAI3_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 9 - SEMC stop request"]
    #[inline(always)]
    #[must_use]
    pub fn semc_stop_req(&mut self) -> SEMC_STOP_REQ_W<9> {
        SEMC_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 10 - PIT stop request"]
    #[inline(always)]
    #[must_use]
    pub fn pit_stop_req(&mut self) -> PIT_STOP_REQ_W<10> {
        PIT_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 11 - FlexSPI stop request"]
    #[inline(always)]
    #[must_use]
    pub fn flexspi_stop_req(&mut self) -> FLEXSPI_STOP_REQ_W<11> {
        FLEXSPI_STOP_REQ_W::new(self)
    }
    #[doc = "Bit 12 - FlexIO1 stop request"]
    #[inline(always)]
    #[must_use]
    pub fn flexio1_stop_req(&mut self) -> FLEXIO1_STOP_REQ_W<12> {
        FLEXIO1_STOP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR4 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr4](index.html) module"]
pub struct GPR4_SPEC;
impl crate::RegisterSpec for GPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr4::R](R) reader structure"]
impl crate::Readable for GPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr4::W](W) writer structure"]
impl crate::Writable for GPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR4 to value 0"]
impl crate::Resettable for GPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
