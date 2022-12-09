#[doc = "Register `SMCVAL3CYC` reader"]
pub struct R(crate::R<SMCVAL3CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL3CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL3CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL3CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL3CYC` reader - CVAL3CYC"]
pub type CVAL3CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL3CYC"]
    #[inline(always)]
    pub fn cval3cyc(&self) -> CVAL3CYC_R {
        CVAL3CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 3 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval3cyc](index.html) module"]
pub struct SMCVAL3CYC_SPEC;
impl crate::RegisterSpec for SMCVAL3CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval3cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL3CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL3CYC to value 0"]
impl crate::Resettable for SMCVAL3CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
