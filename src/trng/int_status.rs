#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HW_ERR` reader - Read: Error status"]
pub type HW_ERR_R = crate::BitReader<HW_ERR_A>;
#[doc = "Read: Error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HW_ERR_A {
    #[doc = "0: no error"]
    HW_ERR_NO = 0,
    #[doc = "1: error detected."]
    HW_ERR_YES = 1,
}
impl From<HW_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: HW_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl HW_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_ERR_A {
        match self.bits {
            false => HW_ERR_A::HW_ERR_NO,
            true => HW_ERR_A::HW_ERR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `HW_ERR_NO`"]
    #[inline(always)]
    pub fn is_hw_err_no(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_NO
    }
    #[doc = "Checks if the value of the field is `HW_ERR_YES`"]
    #[inline(always)]
    pub fn is_hw_err_yes(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_YES
    }
}
#[doc = "Field `ENT_VAL` reader - Read only: Entropy Valid"]
pub type ENT_VAL_R = crate::BitReader<ENT_VAL_A>;
#[doc = "Read only: Entropy Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENT_VAL_A {
    #[doc = "0: Busy generation entropy. Any value read is invalid."]
    ENT_VAL_INVALID = 0,
    #[doc = "1: TRNG can be stopped and entropy is valid if read."]
    ENT_VAL_VALID = 1,
}
impl From<ENT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ENT_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENT_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENT_VAL_A {
        match self.bits {
            false => ENT_VAL_A::ENT_VAL_INVALID,
            true => ENT_VAL_A::ENT_VAL_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_INVALID`"]
    #[inline(always)]
    pub fn is_ent_val_invalid(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_INVALID
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_VALID`"]
    #[inline(always)]
    pub fn is_ent_val_valid(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_VALID
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Read only: Frequency Count Fail"]
pub type FRQ_CT_FAIL_R = crate::BitReader<FRQ_CT_FAIL_A>;
#[doc = "Read only: Frequency Count Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_NO_ERR = 0,
    #[doc = "1: The frequency counter has detected a failure."]
    FRQ_CT_FAIL_ERR = 1,
}
impl From<FRQ_CT_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: FRQ_CT_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl FRQ_CT_FAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRQ_CT_FAIL_A {
        match self.bits {
            false => FRQ_CT_FAIL_A::FRQ_CT_FAIL_NO_ERR,
            true => FRQ_CT_FAIL_A::FRQ_CT_FAIL_ERR,
        }
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_NO_ERR`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_no_err(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_NO_ERR
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_ERR`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_err(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_ERR
    }
}
impl R {
    #[doc = "Bit 0 - Read: Error status"]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read only: Entropy Valid"]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read only: Frequency Count Fail"]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
