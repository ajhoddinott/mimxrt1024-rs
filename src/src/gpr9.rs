#[doc = "Register `GPR9` reader"]
pub struct R(crate::R<GPR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "SRC General Purpose Register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr9](index.html) module"]
pub struct GPR9_SPEC;
impl crate::RegisterSpec for GPR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr9::R](R) reader structure"]
impl crate::Readable for GPR9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR9 to value 0"]
impl crate::Resettable for GPR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
