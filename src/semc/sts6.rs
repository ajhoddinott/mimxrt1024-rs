#[doc = "Register `STS6` reader"]
pub struct R(crate::R<STS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts6](index.html) module"]
pub struct STS6_SPEC;
impl crate::RegisterSpec for STS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts6::R](R) reader structure"]
impl crate::Readable for STS6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS6 to value 0"]
impl crate::Resettable for STS6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
