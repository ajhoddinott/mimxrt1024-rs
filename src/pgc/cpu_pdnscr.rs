#[doc = "Register `CPU_PDNSCR` reader"]
pub struct R(crate::R<CPU_PDNSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PDNSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PDNSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PDNSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PDNSCR` writer"]
pub struct W(crate::W<CPU_PDNSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PDNSCR_SPEC>;
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
impl From<crate::W<CPU_PDNSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PDNSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO` reader - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
pub type ISO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISO` writer - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
pub type ISO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_PDNSCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `ISO2SW` reader - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
pub type ISO2SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISO2SW` writer - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
pub type ISO2SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_PDNSCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub fn iso2sw(&self) -> ISO2SW_R {
        ISO2SW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> ISO_W<0> {
        ISO_W::new(self)
    }
    #[doc = "Bits 8:13 - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    #[must_use]
    pub fn iso2sw(&mut self) -> ISO2SW_W<8> {
        ISO2SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PGC CPU Pull Down Sequence Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_pdnscr](index.html) module"]
pub struct CPU_PDNSCR_SPEC;
impl crate::RegisterSpec for CPU_PDNSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_pdnscr::R](R) reader structure"]
impl crate::Readable for CPU_PDNSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_pdnscr::W](W) writer structure"]
impl crate::Writable for CPU_PDNSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PDNSCR to value 0x0101"]
impl crate::Resettable for CPU_PDNSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
