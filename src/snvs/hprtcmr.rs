#[doc = "Register `HPRTCMR` reader"]
pub struct R(crate::R<HPRTCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRTCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRTCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRTCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRTCMR` writer"]
pub struct W(crate::W<HPRTCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRTCMR_SPEC>;
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
impl From<crate::W<HPRTCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRTCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - HP Real Time Counter The most-significant 15 bits of the RTC"]
pub type RTC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC` writer - HP Real Time Counter The most-significant 15 bits of the RTC"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPRTCMR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HP Real Time Counter The most-significant 15 bits of the RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Real Time Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprtcmr](index.html) module"]
pub struct HPRTCMR_SPEC;
impl crate::RegisterSpec for HPRTCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprtcmr::R](R) reader structure"]
impl crate::Readable for HPRTCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprtcmr::W](W) writer structure"]
impl crate::Writable for HPRTCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPRTCMR to value 0"]
impl crate::Resettable for HPRTCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
