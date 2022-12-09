#[doc = "Register `SMCVAL3` reader"]
pub struct R(crate::R<SMCVAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL3` reader - CAPTVAL3"]
pub type CAPTVAL3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL3"]
    #[inline(always)]
    pub fn captval3(&self) -> CAPTVAL3_R {
        CAPTVAL3_R::new(self.bits)
    }
}
#[doc = "Capture Value 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval3](index.html) module"]
pub struct SMCVAL3_SPEC;
impl crate::RegisterSpec for SMCVAL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval3::R](R) reader structure"]
impl crate::Readable for SMCVAL3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL3 to value 0"]
impl crate::Resettable for SMCVAL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
