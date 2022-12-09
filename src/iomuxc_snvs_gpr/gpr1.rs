#[doc = "Register `GPR1` reader"]
pub struct R(crate::R<GPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GPR1 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr1](index.html) module"]
pub struct GPR1_SPEC;
impl crate::RegisterSpec for GPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr1::R](R) reader structure"]
impl crate::Readable for GPR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPR1 to value 0"]
impl crate::Resettable for GPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
