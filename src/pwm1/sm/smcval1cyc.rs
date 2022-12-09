#[doc = "Register `SMCVAL1CYC` reader"]
pub struct R(crate::R<SMCVAL1CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL1CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL1CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL1CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL1CYC` reader - CVAL1CYC"]
pub type CVAL1CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL1CYC"]
    #[inline(always)]
    pub fn cval1cyc(&self) -> CVAL1CYC_R {
        CVAL1CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 1 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval1cyc](index.html) module"]
pub struct SMCVAL1CYC_SPEC;
impl crate::RegisterSpec for SMCVAL1CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval1cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL1CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL1CYC to value 0"]
impl crate::Resettable for SMCVAL1CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
