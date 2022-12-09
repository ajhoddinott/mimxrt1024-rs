#[doc = "Register `SMCVAL5` reader"]
pub struct R(crate::R<SMCVAL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL5` reader - CAPTVAL5"]
pub type CAPTVAL5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL5"]
    #[inline(always)]
    pub fn captval5(&self) -> CAPTVAL5_R {
        CAPTVAL5_R::new(self.bits)
    }
}
#[doc = "Capture Value 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval5](index.html) module"]
pub struct SMCVAL5_SPEC;
impl crate::RegisterSpec for SMCVAL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval5::R](R) reader structure"]
impl crate::Readable for SMCVAL5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL5 to value 0"]
impl crate::Resettable for SMCVAL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
