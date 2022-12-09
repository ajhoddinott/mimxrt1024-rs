#[doc = "Register `TCD16_SOFF` reader"]
pub struct R(crate::R<TCD16_SOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD16_SOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD16_SOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD16_SOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD16_SOFF` writer"]
pub struct W(crate::W<TCD16_SOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD16_SOFF_SPEC>;
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
impl From<crate::W<TCD16_SOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD16_SOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFF` reader - Source address signed offset"]
pub type SOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SOFF` writer - Source address signed offset"]
pub type SOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD16_SOFF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&self) -> SOFF_R {
        SOFF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    #[must_use]
    pub fn soff(&mut self) -> SOFF_W<0> {
        SOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd16_soff](index.html) module"]
pub struct TCD16_SOFF_SPEC;
impl crate::RegisterSpec for TCD16_SOFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd16_soff::R](R) reader structure"]
impl crate::Readable for TCD16_SOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd16_soff::W](W) writer structure"]
impl crate::Writable for TCD16_SOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD16_SOFF to value 0"]
impl crate::Resettable for TCD16_SOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
