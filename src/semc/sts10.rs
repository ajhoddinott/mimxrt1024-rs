#[doc = "Register `STS10` reader"]
pub struct R(crate::R<STS10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts10](index.html) module"]
pub struct STS10_SPEC;
impl crate::RegisterSpec for STS10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts10::R](R) reader structure"]
impl crate::Readable for STS10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS10 to value 0"]
impl crate::Resettable for STS10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
