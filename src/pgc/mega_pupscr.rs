#[doc = "Register `MEGA_PUPSCR` reader"]
pub struct R(crate::R<MEGA_PUPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEGA_PUPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEGA_PUPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEGA_PUPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEGA_PUPSCR` writer"]
pub struct W(crate::W<MEGA_PUPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEGA_PUPSCR_SPEC>;
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
impl From<crate::W<MEGA_PUPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEGA_PUPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
pub type SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` writer - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEGA_PUPSCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SW2ISO` reader - After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
pub type SW2ISO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW2ISO` writer - After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
pub type SW2ISO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEGA_PUPSCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub fn sw2iso(&self) -> SW2ISO_R {
        SW2ISO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 8:13 - After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    #[must_use]
    pub fn sw2iso(&mut self) -> SW2ISO_W<8> {
        SW2ISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PGC Mega Power Up Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_pupscr](index.html) module"]
pub struct MEGA_PUPSCR_SPEC;
impl crate::RegisterSpec for MEGA_PUPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mega_pupscr::R](R) reader structure"]
impl crate::Readable for MEGA_PUPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mega_pupscr::W](W) writer structure"]
impl crate::Writable for MEGA_PUPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEGA_PUPSCR to value 0x0f01"]
impl crate::Resettable for MEGA_PUPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f01;
}
