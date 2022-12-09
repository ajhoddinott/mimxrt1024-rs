#[doc = "Register `INT_MASK` reader"]
pub struct R(crate::R<INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MASK` writer"]
pub struct W(crate::W<INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MASK_SPEC>;
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
impl From<crate::W<INT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW_ERR` reader - Bit position that can be cleared or set to enable the corresponding bit of INT_STATUS to show interupt status"]
pub type HW_ERR_R = crate::BitReader<HW_ERR_A>;
#[doc = "Bit position that can be cleared or set to enable the corresponding bit of INT_STATUS to show interupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HW_ERR_A {
    #[doc = "0: Corresponding interrupt of INT_STATUS is masked."]
    HW_ERR_MASKED = 0,
    #[doc = "1: Corresponding bit of INT_STATUS is active."]
    HW_ERR_ACTIVE = 1,
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
            false => HW_ERR_A::HW_ERR_MASKED,
            true => HW_ERR_A::HW_ERR_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `HW_ERR_MASKED`"]
    #[inline(always)]
    pub fn is_hw_err_masked(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_MASKED
    }
    #[doc = "Checks if the value of the field is `HW_ERR_ACTIVE`"]
    #[inline(always)]
    pub fn is_hw_err_active(&self) -> bool {
        *self == HW_ERR_A::HW_ERR_ACTIVE
    }
}
#[doc = "Field `HW_ERR` writer - Bit position that can be cleared or set to enable the corresponding bit of INT_STATUS to show interupt status"]
pub type HW_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, HW_ERR_A, O>;
impl<'a, const O: u8> HW_ERR_W<'a, O> {
    #[doc = "Corresponding interrupt of INT_STATUS is masked."]
    #[inline(always)]
    pub fn hw_err_masked(self) -> &'a mut W {
        self.variant(HW_ERR_A::HW_ERR_MASKED)
    }
    #[doc = "Corresponding bit of INT_STATUS is active."]
    #[inline(always)]
    pub fn hw_err_active(self) -> &'a mut W {
        self.variant(HW_ERR_A::HW_ERR_ACTIVE)
    }
}
#[doc = "Field `ENT_VAL` reader - Same behavior as bit 0 of this register."]
pub type ENT_VAL_R = crate::BitReader<ENT_VAL_A>;
#[doc = "Same behavior as bit 0 of this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENT_VAL_A {
    #[doc = "0: Same behavior as bit 0 of this register."]
    ENT_VAL_MASKED = 0,
    #[doc = "1: Same behavior as bit 0 of this register."]
    ENT_VAL_ACTIVE = 1,
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
            false => ENT_VAL_A::ENT_VAL_MASKED,
            true => ENT_VAL_A::ENT_VAL_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_MASKED`"]
    #[inline(always)]
    pub fn is_ent_val_masked(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_MASKED
    }
    #[doc = "Checks if the value of the field is `ENT_VAL_ACTIVE`"]
    #[inline(always)]
    pub fn is_ent_val_active(&self) -> bool {
        *self == ENT_VAL_A::ENT_VAL_ACTIVE
    }
}
#[doc = "Field `ENT_VAL` writer - Same behavior as bit 0 of this register."]
pub type ENT_VAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, ENT_VAL_A, O>;
impl<'a, const O: u8> ENT_VAL_W<'a, O> {
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val_masked(self) -> &'a mut W {
        self.variant(ENT_VAL_A::ENT_VAL_MASKED)
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val_active(self) -> &'a mut W {
        self.variant(ENT_VAL_A::ENT_VAL_ACTIVE)
    }
}
#[doc = "Field `FRQ_CT_FAIL` reader - Same behavior as bit 0 of this register."]
pub type FRQ_CT_FAIL_R = crate::BitReader<FRQ_CT_FAIL_A>;
#[doc = "Same behavior as bit 0 of this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRQ_CT_FAIL_A {
    #[doc = "0: Same behavior as bit 0 of this register."]
    FRQ_CT_FAIL_MASKED = 0,
    #[doc = "1: Same behavior as bit 0 of this register."]
    FRQ_CT_FAIL_ACTIVE = 1,
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
            false => FRQ_CT_FAIL_A::FRQ_CT_FAIL_MASKED,
            true => FRQ_CT_FAIL_A::FRQ_CT_FAIL_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_MASKED`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_masked(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_MASKED
    }
    #[doc = "Checks if the value of the field is `FRQ_CT_FAIL_ACTIVE`"]
    #[inline(always)]
    pub fn is_frq_ct_fail_active(&self) -> bool {
        *self == FRQ_CT_FAIL_A::FRQ_CT_FAIL_ACTIVE
    }
}
#[doc = "Field `FRQ_CT_FAIL` writer - Same behavior as bit 0 of this register."]
pub type FRQ_CT_FAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_MASK_SPEC, FRQ_CT_FAIL_A, O>;
impl<'a, const O: u8> FRQ_CT_FAIL_W<'a, O> {
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail_masked(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::FRQ_CT_FAIL_MASKED)
    }
    #[doc = "Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail_active(self) -> &'a mut W {
        self.variant(FRQ_CT_FAIL_A::FRQ_CT_FAIL_ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Bit position that can be cleared or set to enable the corresponding bit of INT_STATUS to show interupt status"]
    #[inline(always)]
    pub fn hw_err(&self) -> HW_ERR_R {
        HW_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn ent_val(&self) -> ENT_VAL_R {
        ENT_VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    pub fn frq_ct_fail(&self) -> FRQ_CT_FAIL_R {
        FRQ_CT_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit position that can be cleared or set to enable the corresponding bit of INT_STATUS to show interupt status"]
    #[inline(always)]
    #[must_use]
    pub fn hw_err(&mut self) -> HW_ERR_W<0> {
        HW_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    #[must_use]
    pub fn ent_val(&mut self) -> ENT_VAL_W<1> {
        ENT_VAL_W::new(self)
    }
    #[doc = "Bit 2 - Same behavior as bit 0 of this register."]
    #[inline(always)]
    #[must_use]
    pub fn frq_ct_fail(&mut self) -> FRQ_CT_FAIL_W<2> {
        FRQ_CT_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mask](index.html) module"]
pub struct INT_MASK_SPEC;
impl crate::RegisterSpec for INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mask::R](R) reader structure"]
impl crate::Readable for INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_mask::W](W) writer structure"]
impl crate::Writable for INT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for INT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
