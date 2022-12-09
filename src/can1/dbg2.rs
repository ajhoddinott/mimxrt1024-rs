#[doc = "Register `DBG2` reader"]
pub struct R(crate::R<DBG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RMP` reader - Rx Matching Pointer"]
pub type RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPP` reader - Matching Process in Progress"]
pub type MPP_R = crate::BitReader<MPP_A>;
#[doc = "Matching Process in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPP_A {
    #[doc = "0: No matching process ongoing."]
    MPP_0 = 0,
    #[doc = "1: Matching process is in progress."]
    MPP_1 = 1,
}
impl From<MPP_A> for bool {
    #[inline(always)]
    fn from(variant: MPP_A) -> Self {
        variant as u8 != 0
    }
}
impl MPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPP_A {
        match self.bits {
            false => MPP_A::MPP_0,
            true => MPP_A::MPP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MPP_0`"]
    #[inline(always)]
    pub fn is_mpp_0(&self) -> bool {
        *self == MPP_A::MPP_0
    }
    #[doc = "Checks if the value of the field is `MPP_1`"]
    #[inline(always)]
    pub fn is_mpp_1(&self) -> bool {
        *self == MPP_A::MPP_1
    }
}
#[doc = "Field `TAP` reader - Tx Arbitration Pointer"]
pub type TAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APP` reader - Arbitration Process in Progress"]
pub type APP_R = crate::BitReader<APP_A>;
#[doc = "Arbitration Process in Progress\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APP_A {
    #[doc = "0: No matching process ongoing."]
    APP_0 = 0,
    #[doc = "1: Matching process is in progress."]
    APP_1 = 1,
}
impl From<APP_A> for bool {
    #[inline(always)]
    fn from(variant: APP_A) -> Self {
        variant as u8 != 0
    }
}
impl APP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APP_A {
        match self.bits {
            false => APP_A::APP_0,
            true => APP_A::APP_1,
        }
    }
    #[doc = "Checks if the value of the field is `APP_0`"]
    #[inline(always)]
    pub fn is_app_0(&self) -> bool {
        *self == APP_A::APP_0
    }
    #[doc = "Checks if the value of the field is `APP_1`"]
    #[inline(always)]
    pub fn is_app_1(&self) -> bool {
        *self == APP_A::APP_1
    }
}
impl R {
    #[doc = "Bits 0:6 - Rx Matching Pointer"]
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Matching Process in Progress"]
    #[inline(always)]
    pub fn mpp(&self) -> MPP_R {
        MPP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Tx Arbitration Pointer"]
    #[inline(always)]
    pub fn tap(&self) -> TAP_R {
        TAP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Arbitration Process in Progress"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Debug 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg2](index.html) module"]
pub struct DBG2_SPEC;
impl crate::RegisterSpec for DBG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg2::R](R) reader structure"]
impl crate::Readable for DBG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG2 to value 0"]
impl crate::Resettable for DBG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
