#[doc = "Register `LPSRTCLR` reader"]
pub struct R(crate::R<LPSRTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSRTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSRTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSRTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSRTCLR` writer"]
pub struct W(crate::W<LPSRTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSRTCLR_SPEC>;
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
impl From<crate::W<LPSRTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSRTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRTC` reader - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
pub type SRTC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRTC` writer - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
pub type SRTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPSRTCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
    #[inline(always)]
    pub fn srtc(&self) -> SRTC_R {
        SRTC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LP Secure Real Time Counter least-significant 32 bits This register can be programmed only when SRTC is not active and not locked, meaning the SRTC_ENV, SRTC_SL, and SRTC_HL bits are not set"]
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
#[doc = "SNVS_LP Secure Real Time Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsrtclr](index.html) module"]
pub struct LPSRTCLR_SPEC;
impl crate::RegisterSpec for LPSRTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsrtclr::R](R) reader structure"]
impl crate::Readable for LPSRTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsrtclr::W](W) writer structure"]
impl crate::Writable for LPSRTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSRTCLR to value 0"]
impl crate::Resettable for LPSRTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
