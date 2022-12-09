#[doc = "Register `SMCVAL2` reader"]
pub struct R(crate::R<SMCVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL2` reader - CAPTVAL2"]
pub type CAPTVAL2_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL2"]
    #[inline(always)]
    pub fn captval2(&self) -> CAPTVAL2_R {
        CAPTVAL2_R::new(self.bits)
    }
}
#[doc = "Capture Value 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval2](index.html) module"]
pub struct SMCVAL2_SPEC;
impl crate::RegisterSpec for SMCVAL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval2::R](R) reader structure"]
impl crate::Readable for SMCVAL2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL2 to value 0"]
impl crate::Resettable for SMCVAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
