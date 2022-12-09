#[doc = "Register `HPRTCLR` reader"]
pub struct R(crate::R<HPRTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRTCLR` writer"]
pub struct W(crate::W<HPRTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRTCLR_SPEC>;
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
impl From<crate::W<HPRTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - HP Real Time Counter least-significant 32 bits"]
pub type RTC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC` writer - HP Real Time Counter least-significant 32 bits"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPRTCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HP Real Time Counter least-significant 32 bits"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HP Real Time Counter least-significant 32 bits"]
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
#[doc = "SNVS_HP Real Time Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprtclr](index.html) module"]
pub struct HPRTCLR_SPEC;
impl crate::RegisterSpec for HPRTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprtclr::R](R) reader structure"]
impl crate::Readable for HPRTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprtclr::W](W) writer structure"]
impl crate::Writable for HPRTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPRTCLR to value 0"]
impl crate::Resettable for HPRTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
