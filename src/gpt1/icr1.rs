#[doc = "Register `ICR1` reader"]
pub struct R(crate::R<ICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPT` reader - Capture Value"]
pub type CAPT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture Value"]
    #[inline(always)]
    pub fn capt(&self) -> CAPT_R {
        CAPT_R::new(self.bits)
    }
}
#[doc = "GPT Input Capture Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](index.html) module"]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr1::R](R) reader structure"]
impl crate::Readable for ICR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
