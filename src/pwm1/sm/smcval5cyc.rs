#[doc = "Register `SMCVAL5CYC` reader"]
pub struct R(crate::R<SMCVAL5CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL5CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL5CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL5CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL5CYC` reader - CVAL5CYC"]
pub type CVAL5CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL5CYC"]
    #[inline(always)]
    pub fn cval5cyc(&self) -> CVAL5CYC_R {
        CVAL5CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 5 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval5cyc](index.html) module"]
pub struct SMCVAL5CYC_SPEC;
impl crate::RegisterSpec for SMCVAL5CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval5cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL5CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL5CYC to value 0"]
impl crate::Resettable for SMCVAL5CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
