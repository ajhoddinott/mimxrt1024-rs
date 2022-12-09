#[doc = "Register `UPOSH` reader"]
pub struct R(crate::R<UPOSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPOSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPOSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPOSH_SPEC>) -> Self {
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
#[doc = "Upper Position Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uposh](index.html) module"]
pub struct UPOSH_SPEC;
impl crate::RegisterSpec for UPOSH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uposh::R](R) reader structure"]
impl crate::Readable for UPOSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UPOSH to value 0"]
impl crate::Resettable for UPOSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
