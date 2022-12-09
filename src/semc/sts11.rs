#[doc = "Register `STS11` reader"]
pub struct R(crate::R<STS11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts11](index.html) module"]
pub struct STS11_SPEC;
impl crate::RegisterSpec for STS11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts11::R](R) reader structure"]
impl crate::Readable for STS11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS11 to value 0"]
impl crate::Resettable for STS11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
