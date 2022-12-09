#[doc = "Register `TIMIEN` reader"]
pub struct R(crate::R<TIMIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMIEN` writer"]
pub struct W(crate::W<TIMIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMIEN_SPEC>;
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
impl From<crate::W<TIMIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEIE` reader - Timer Status Interrupt Enable"]
pub type TEIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEIE` writer - Timer Status Interrupt Enable"]
pub type TEIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMIEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer Status Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<0> {
        TEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timien](index.html) module"]
pub struct TIMIEN_SPEC;
impl crate::RegisterSpec for TIMIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timien::R](R) reader structure"]
impl crate::Readable for TIMIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timien::W](W) writer structure"]
impl crate::Writable for TIMIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMIEN to value 0"]
impl crate::Resettable for TIMIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
