#[doc = "Register `STS7` reader"]
pub struct R(crate::R<STS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts7](index.html) module"]
pub struct STS7_SPEC;
impl crate::RegisterSpec for STS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts7::R](R) reader structure"]
impl crate::Readable for STS7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS7 to value 0"]
impl crate::Resettable for STS7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
