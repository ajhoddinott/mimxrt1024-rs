#[doc = "Register `SMCVAL4CYC` reader"]
pub struct R(crate::R<SMCVAL4CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL4CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL4CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL4CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL4CYC` reader - CVAL4CYC"]
pub type CVAL4CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL4CYC"]
    #[inline(always)]
    pub fn cval4cyc(&self) -> CVAL4CYC_R {
        CVAL4CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 4 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval4cyc](index.html) module"]
pub struct SMCVAL4CYC_SPEC;
impl crate::RegisterSpec for SMCVAL4CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval4cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL4CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL4CYC to value 0"]
impl crate::Resettable for SMCVAL4CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
