#[doc = "Register `STS1` reader"]
pub struct R(crate::R<STS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts1](index.html) module"]
pub struct STS1_SPEC;
impl crate::RegisterSpec for STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts1::R](R) reader structure"]
impl crate::Readable for STS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
