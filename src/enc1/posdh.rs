#[doc = "Register `POSDH` reader"]
pub struct R(crate::R<POSDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POSDH` reader - POSDH"]
pub type POSDH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - POSDH"]
    #[inline(always)]
    pub fn posdh(&self) -> POSDH_R {
        POSDH_R::new(self.bits)
    }
}
#[doc = "Position Difference Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [posdh](index.html) module"]
pub struct POSDH_SPEC;
impl crate::RegisterSpec for POSDH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [posdh::R](R) reader structure"]
impl crate::Readable for POSDH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POSDH to value 0"]
impl crate::Resettable for POSDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
