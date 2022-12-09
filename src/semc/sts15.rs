#[doc = "Register `STS15` reader"]
pub struct R(crate::R<STS15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts15](index.html) module"]
pub struct STS15_SPEC;
impl crate::RegisterSpec for STS15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts15::R](R) reader structure"]
impl crate::Readable for STS15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS15 to value 0"]
impl crate::Resettable for STS15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
