#[doc = "Register `STS5` reader"]
pub struct R(crate::R<STS5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts5](index.html) module"]
pub struct STS5_SPEC;
impl crate::RegisterSpec for STS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts5::R](R) reader structure"]
impl crate::Readable for STS5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS5 to value 0"]
impl crate::Resettable for STS5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
