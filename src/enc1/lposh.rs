#[doc = "Register `LPOSH` reader"]
pub struct R(crate::R<LPOSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POSH` reader - POSH"]
pub type POSH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - POSH"]
    #[inline(always)]
    pub fn posh(&self) -> POSH_R {
        POSH_R::new(self.bits)
    }
}
#[doc = "Lower Position Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lposh](index.html) module"]
pub struct LPOSH_SPEC;
impl crate::RegisterSpec for LPOSH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lposh::R](R) reader structure"]
impl crate::Readable for LPOSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPOSH to value 0"]
impl crate::Resettable for LPOSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
