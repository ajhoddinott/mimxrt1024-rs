#[doc = "Register `STS13` reader"]
pub struct R(crate::R<STS13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts13](index.html) module"]
pub struct STS13_SPEC;
impl crate::RegisterSpec for STS13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts13::R](R) reader structure"]
impl crate::Readable for STS13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS13 to value 0"]
impl crate::Resettable for STS13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
