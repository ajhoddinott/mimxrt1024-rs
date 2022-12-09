#[doc = "Register `TDR[%s]` reader"]
pub struct R(crate::R<TDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDR[%s]` writer"]
pub struct W(crate::W<TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDR_SPEC>;
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
impl From<crate::W<TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDR` reader - Transmit Data Register"]
pub type TDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TDR` writer - Transmit Data Register"]
pub type TDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TDR_W<0> {
        TDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](index.html) module"]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdr::R](R) reader structure"]
impl crate::Readable for TDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdr::W](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR[%s]
to value 0"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
