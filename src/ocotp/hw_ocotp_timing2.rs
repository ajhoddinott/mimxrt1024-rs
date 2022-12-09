#[doc = "Register `HW_OCOTP_TIMING2` reader"]
pub struct R(crate::R<HW_OCOTP_TIMING2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_TIMING2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_TIMING2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_TIMING2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_TIMING2` writer"]
pub struct W(crate::W<HW_OCOTP_TIMING2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_TIMING2_SPEC>;
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
impl From<crate::W<HW_OCOTP_TIMING2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_TIMING2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELAX_PROG` reader - Relax Prog. count value"]
pub type RELAX_PROG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELAX_PROG` writer - Relax Prog. count value"]
pub type RELAX_PROG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_TIMING2_SPEC, u16, u16, 12, O>;
#[doc = "Field `RELAX_READ` reader - Relax Read count value"]
pub type RELAX_READ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RELAX_READ` writer - Relax Read count value"]
pub type RELAX_READ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_TIMING2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:11 - Relax Prog. count value"]
    #[inline(always)]
    pub fn relax_prog(&self) -> RELAX_PROG_R {
        RELAX_PROG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Relax Read count value"]
    #[inline(always)]
    pub fn relax_read(&self) -> RELAX_READ_R {
        RELAX_READ_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Relax Prog. count value"]
    #[inline(always)]
    #[must_use]
    pub fn relax_prog(&mut self) -> RELAX_PROG_W<0> {
        RELAX_PROG_W::new(self)
    }
    #[doc = "Bits 16:21 - Relax Read count value"]
    #[inline(always)]
    #[must_use]
    pub fn relax_read(&mut self) -> RELAX_READ_W<16> {
        RELAX_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Controller Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_timing2](index.html) module"]
pub struct HW_OCOTP_TIMING2_SPEC;
impl crate::RegisterSpec for HW_OCOTP_TIMING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_timing2::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_TIMING2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_timing2::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_TIMING2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_TIMING2 to value 0x0003_0092"]
impl crate::Resettable for HW_OCOTP_TIMING2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0092;
}
