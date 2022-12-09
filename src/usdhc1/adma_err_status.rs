#[doc = "Register `ADMA_ERR_STATUS` reader"]
pub struct R(crate::R<ADMA_ERR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMA_ERR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMA_ERR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMA_ERR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADMAES` reader - ADMA error state (when ADMA error is occurred)"]
pub type ADMAES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMALME` reader - ADMA length mismatch error"]
pub type ADMALME_R = crate::BitReader<ADMALME_A>;
#[doc = "ADMA length mismatch error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMALME_A {
    #[doc = "0: No error"]
    ADMALME_0 = 0,
    #[doc = "1: Error"]
    ADMALME_1 = 1,
}
impl From<ADMALME_A> for bool {
    #[inline(always)]
    fn from(variant: ADMALME_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMALME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMALME_A {
        match self.bits {
            false => ADMALME_A::ADMALME_0,
            true => ADMALME_A::ADMALME_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMALME_0`"]
    #[inline(always)]
    pub fn is_admalme_0(&self) -> bool {
        *self == ADMALME_A::ADMALME_0
    }
    #[doc = "Checks if the value of the field is `ADMALME_1`"]
    #[inline(always)]
    pub fn is_admalme_1(&self) -> bool {
        *self == ADMALME_A::ADMALME_1
    }
}
#[doc = "Field `ADMADCE` reader - ADMA descriptor error"]
pub type ADMADCE_R = crate::BitReader<ADMADCE_A>;
#[doc = "ADMA descriptor error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMADCE_A {
    #[doc = "0: No error"]
    ADMADCE_0 = 0,
    #[doc = "1: Error"]
    ADMADCE_1 = 1,
}
impl From<ADMADCE_A> for bool {
    #[inline(always)]
    fn from(variant: ADMADCE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMADCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMADCE_A {
        match self.bits {
            false => ADMADCE_A::ADMADCE_0,
            true => ADMADCE_A::ADMADCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADMADCE_0`"]
    #[inline(always)]
    pub fn is_admadce_0(&self) -> bool {
        *self == ADMADCE_A::ADMADCE_0
    }
    #[doc = "Checks if the value of the field is `ADMADCE_1`"]
    #[inline(always)]
    pub fn is_admadce_1(&self) -> bool {
        *self == ADMADCE_A::ADMADCE_1
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA error state (when ADMA error is occurred)"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA length mismatch error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADMA descriptor error"]
    #[inline(always)]
    pub fn admadce(&self) -> ADMADCE_R {
        ADMADCE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "ADMA Error Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_err_status](index.html) module"]
pub struct ADMA_ERR_STATUS_SPEC;
impl crate::RegisterSpec for ADMA_ERR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adma_err_status::R](R) reader structure"]
impl crate::Readable for ADMA_ERR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADMA_ERR_STATUS to value 0"]
impl crate::Resettable for ADMA_ERR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
