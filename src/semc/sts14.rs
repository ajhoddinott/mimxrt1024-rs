#[doc = "Register `STS14` reader"]
pub struct R(crate::R<STS14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts14](index.html) module"]
pub struct STS14_SPEC;
impl crate::RegisterSpec for STS14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts14::R](R) reader structure"]
impl crate::Readable for STS14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS14 to value 0"]
impl crate::Resettable for STS14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
