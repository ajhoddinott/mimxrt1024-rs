#[doc = "Register `HW_OCOTP_TIMING` reader"]
pub struct R(crate::R<HW_OCOTP_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_TIMING` writer"]
pub struct W(crate::W<HW_OCOTP_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_TIMING_SPEC>;
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
impl From<crate::W<HW_OCOTP_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STROBE_PROG` reader - Write Strobe Period"]
pub type STROBE_PROG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STROBE_PROG` writer - Write Strobe Period"]
pub type STROBE_PROG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_TIMING_SPEC, u16, u16, 12, O>;
#[doc = "Field `RELAX` reader - Relax Count Value"]
pub type RELAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RELAX` writer - Relax Count Value"]
pub type RELAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HW_OCOTP_TIMING_SPEC, u8, u8, 4, O>;
#[doc = "Field `STROBE_READ` reader - Read Strobe Period"]
pub type STROBE_READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STROBE_READ` writer - Read Strobe Period"]
pub type STROBE_READ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_TIMING_SPEC, u8, u8, 6, O>;
#[doc = "Field `WAIT` reader - Wait Interval"]
pub type WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT` writer - Wait Interval"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HW_OCOTP_TIMING_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:11 - Write Strobe Period"]
    #[inline(always)]
    pub fn strobe_prog(&self) -> STROBE_PROG_R {
        STROBE_PROG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Relax Count Value"]
    #[inline(always)]
    pub fn relax(&self) -> RELAX_R {
        RELAX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Read Strobe Period"]
    #[inline(always)]
    pub fn strobe_read(&self) -> STROBE_READ_R {
        STROBE_READ_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - Wait Interval"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Write Strobe Period"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_prog(&mut self) -> STROBE_PROG_W<0> {
        STROBE_PROG_W::new(self)
    }
    #[doc = "Bits 12:15 - Relax Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn relax(&mut self) -> RELAX_W<12> {
        RELAX_W::new(self)
    }
    #[doc = "Bits 16:21 - Read Strobe Period"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_read(&mut self) -> STROBE_READ_W<16> {
        STROBE_READ_W::new(self)
    }
    #[doc = "Bits 22:27 - Wait Interval"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<22> {
        WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Controller Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_timing](index.html) module"]
pub struct HW_OCOTP_TIMING_SPEC;
impl crate::RegisterSpec for HW_OCOTP_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_timing::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_timing::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_TIMING to value 0x060d_9755"]
impl crate::Resettable for HW_OCOTP_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0x060d_9755;
}
