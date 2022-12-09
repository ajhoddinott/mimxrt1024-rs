#[doc = "Register `SMCVAL0` reader"]
pub struct R(crate::R<SMCVAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL0` reader - CAPTVAL0"]
pub type CAPTVAL0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL0"]
    #[inline(always)]
    pub fn captval0(&self) -> CAPTVAL0_R {
        CAPTVAL0_R::new(self.bits)
    }
}
#[doc = "Capture Value 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval0](index.html) module"]
pub struct SMCVAL0_SPEC;
impl crate::RegisterSpec for SMCVAL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval0::R](R) reader structure"]
impl crate::Readable for SMCVAL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL0 to value 0"]
impl crate::Resettable for SMCVAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
