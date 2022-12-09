#[doc = "Register `LPSRTCMR` reader"]
pub struct R(crate::R<LPSRTCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSRTCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSRTCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSRTCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSRTCMR` writer"]
pub struct W(crate::W<LPSRTCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSRTCMR_SPEC>;
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
impl From<crate::W<LPSRTCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSRTCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRTC` reader - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
pub type SRTC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRTC` writer - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
pub type SRTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPSRTCMR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    pub fn srtc(&self) -> SRTC_R {
        SRTC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - LP Secure Real Time Counter The most-significant 15 bits of the SRTC"]
    #[inline(always)]
    #[must_use]
    pub fn srtc(&mut self) -> SRTC_W<0> {
        SRTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Secure Real Time Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsrtcmr](index.html) module"]
pub struct LPSRTCMR_SPEC;
impl crate::RegisterSpec for LPSRTCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsrtcmr::R](R) reader structure"]
impl crate::Readable for LPSRTCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsrtcmr::W](W) writer structure"]
impl crate::Writable for LPSRTCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSRTCMR to value 0"]
impl crate::Resettable for LPSRTCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
