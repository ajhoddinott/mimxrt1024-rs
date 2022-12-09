#[doc = "Register `REVH` reader"]
pub struct R(crate::R<REVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVH` reader - REVH"]
pub type REVH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - REVH"]
    #[inline(always)]
    pub fn revh(&self) -> REVH_R {
        REVH_R::new(self.bits)
    }
}
#[doc = "Revolution Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revh](index.html) module"]
pub struct REVH_SPEC;
impl crate::RegisterSpec for REVH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [revh::R](R) reader structure"]
impl crate::Readable for REVH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REVH to value 0"]
impl crate::Resettable for REVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
