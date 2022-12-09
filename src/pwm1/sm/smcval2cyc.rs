#[doc = "Register `SMCVAL2CYC` reader"]
pub struct R(crate::R<SMCVAL2CYC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL2CYC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL2CYC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL2CYC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CVAL2CYC` reader - CVAL2CYC"]
pub type CVAL2CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CVAL2CYC"]
    #[inline(always)]
    pub fn cval2cyc(&self) -> CVAL2CYC_R {
        CVAL2CYC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Capture Value 2 Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval2cyc](index.html) module"]
pub struct SMCVAL2CYC_SPEC;
impl crate::RegisterSpec for SMCVAL2CYC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval2cyc::R](R) reader structure"]
impl crate::Readable for SMCVAL2CYC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL2CYC to value 0"]
impl crate::Resettable for SMCVAL2CYC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
