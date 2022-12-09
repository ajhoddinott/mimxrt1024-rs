#[doc = "Register `STS8` reader"]
pub struct R(crate::R<STS8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts8](index.html) module"]
pub struct STS8_SPEC;
impl crate::RegisterSpec for STS8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts8::R](R) reader structure"]
impl crate::Readable for STS8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS8 to value 0"]
impl crate::Resettable for STS8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
