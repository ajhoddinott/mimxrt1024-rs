#[doc = "Register `SMCNT` reader"]
pub struct R(crate::R<SMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Counter Register Bits"]
pub type CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Register Bits"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcnt](index.html) module"]
pub struct SMCNT_SPEC;
impl crate::RegisterSpec for SMCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcnt::R](R) reader structure"]
impl crate::Readable for SMCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SMCNT to value 0"]
impl crate::Resettable for SMCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
