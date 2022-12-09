#[doc = "Register `SMCVAL0CYC` reader"]
pub struct R(crate::R<SMCVAL0CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL0CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL0CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL0CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL0CYC` reader - CVAL0CYC"]
pub type CVAL0CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL0CYC"]
    #[inline(always)]
    pub fn cval0cyc(&self) -> CVAL0CYC_R {
        CVAL0CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 0 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval0cyc](index.html) module"]
pub struct SMCVAL0CYC_SPEC;
impl crate::RegisterSpec for SMCVAL0CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval0cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL0CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL0CYC to value 0"]
impl crate::Resettable for SMCVAL0CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
