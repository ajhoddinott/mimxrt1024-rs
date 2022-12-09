#[doc = "Register `SMCVAL4` reader"]
pub struct R(crate::R<SMCVAL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCVAL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCVAL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCVAL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPTVAL4` reader - CAPTVAL4"]
pub type CAPTVAL4_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPTVAL4"]
    #[inline(always)]
    pub fn captval4(&self) -> CAPTVAL4_R {
        CAPTVAL4_R::new(self.bits)
    }
}
#[doc = "Capture Value 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcval4](index.html) module"]
pub struct SMCVAL4_SPEC;
impl crate::RegisterSpec for SMCVAL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcval4::R](R) reader structure"]
impl crate::Readable for SMCVAL4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCVAL4 to value 0"]
impl crate::Resettable for SMCVAL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
