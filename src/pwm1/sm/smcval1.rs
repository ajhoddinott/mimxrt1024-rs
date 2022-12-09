#[doc = "Register `SMCVAL1` reader"]
pub struct R(crate::R<SMCVAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL1` reader - CAPTVAL1"]
pub type CAPTVAL1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL1"]
    #[inline(always)]
    pub fn captval1(&self) -> CAPTVAL1_R {
        CAPTVAL1_R::new(self.bits)
    }
}
#[doc = "Capture Value 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval1](index.html) module"]
pub struct SMCVAL1_SPEC;
impl crate::RegisterSpec for SMCVAL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval1::R](R) reader structure"]
impl crate::Readable for SMCVAL1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL1 to value 0"]
impl crate::Resettable for SMCVAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
