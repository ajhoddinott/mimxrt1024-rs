#[doc = "Register `SMDTCNT0` reader"]
pub struct R(crate::R<SMDTCNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMDTCNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMDTCNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMDTCNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMDTCNT0` writer"]
pub struct W(crate::W<SMDTCNT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMDTCNT0_SPEC>;
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
impl From<crate::W<SMDTCNT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMDTCNT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCNT0` reader - DTCNT0"]
pub type DTCNT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTCNT0` writer - DTCNT0"]
pub type DTCNT0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMDTCNT0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DTCNT0"]
    #[inline(always)]
    pub fn dtcnt0(&self) -> DTCNT0_R {
        DTCNT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTCNT0"]
    #[inline(always)]
    #[must_use]
    pub fn dtcnt0(&mut self) -> DTCNT0_W<0> {
        DTCNT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deadtime Count Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdtcnt0](index.html) module"]
pub struct SMDTCNT0_SPEC;
impl crate::RegisterSpec for SMDTCNT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smdtcnt0::R](R) reader structure"]
impl crate::Readable for SMDTCNT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smdtcnt0::W](W) writer structure"]
impl crate::Writable for SMDTCNT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMDTCNT0 to value 0x07ff"]
impl crate::Resettable for SMDTCNT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
