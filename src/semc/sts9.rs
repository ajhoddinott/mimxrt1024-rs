#[doc = "Register `STS9` reader"]
pub struct R(crate::R<STS9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts9](index.html) module"]
pub struct STS9_SPEC;
impl crate::RegisterSpec for STS9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts9::R](R) reader structure"]
impl crate::Readable for STS9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS9 to value 0"]
impl crate::Resettable for STS9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
