#[doc = "Register `STS3` reader"]
pub struct R(crate::R<STS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts3](index.html) module"]
pub struct STS3_SPEC;
impl crate::RegisterSpec for STS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts3::R](R) reader structure"]
impl crate::Readable for STS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS3 to value 0"]
impl crate::Resettable for STS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
