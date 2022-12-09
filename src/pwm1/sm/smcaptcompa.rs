#[doc = "Register `SMCAPTCOMPA` reader"]
pub struct R(crate::R<SMCAPTCOMPA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCOMPA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCOMPA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCOMPA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCOMPA` writer"]
pub struct W(crate::W<SMCAPTCOMPA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCOMPA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMCAPTCOMPA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCOMPA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGCMPA` reader - Edge Compare A"]
pub type EDGCMPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGCMPA` writer - Edge Compare A"]
pub type EDGCMPA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCOMPA_SPEC, u8, u8, 8, O>;
#[doc = "Field `EDGCNTA` reader - Edge Counter A"]
pub type EDGCNTA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare A"]
    #[inline(always)]
    pub fn edgcmpa(&self) -> EDGCMPA_R {
        EDGCMPA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter A"]
    #[inline(always)]
    pub fn edgcnta(&self) -> EDGCNTA_R {
        EDGCNTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare A"]
    #[inline(always)]
    #[must_use]
    pub fn edgcmpa(&mut self) -> EDGCMPA_W<0> {
        EDGCMPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Compare A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompa](index.html) module"]
pub struct SMCAPTCOMPA_SPEC;
impl crate::RegisterSpec for SMCAPTCOMPA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptcompa::R](R) reader structure"]
impl crate::Readable for SMCAPTCOMPA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptcompa::W](W) writer structure"]
impl crate::Writable for SMCAPTCOMPA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCOMPA to value 0"]
impl crate::Resettable for SMCAPTCOMPA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
