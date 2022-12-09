#[doc = "Register `CAPT1` reader"]
pub struct R(crate::R<CAPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPT1` writer"]
pub struct W(crate::W<CAPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPT1_SPEC>;
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
impl From<crate::W<CAPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTURE` reader - Capture Value"]
pub type CAPTURE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPTURE` writer - Capture Value"]
pub type CAPTURE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CAPT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture Value"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture Value"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capt1](index.html) module"]
pub struct CAPT1_SPEC;
impl crate::RegisterSpec for CAPT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [capt1::R](R) reader structure"]
impl crate::Readable for CAPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capt1::W](W) writer structure"]
impl crate::Writable for CAPT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPT1 to value 0"]
impl crate::Resettable for CAPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
