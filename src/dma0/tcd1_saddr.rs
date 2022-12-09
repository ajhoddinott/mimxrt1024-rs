#[doc = "Register `TCD1_SADDR` reader"]
pub struct R(crate::R<TCD1_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD1_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD1_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD1_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD1_SADDR` writer"]
pub struct W(crate::W<TCD1_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD1_SADDR_SPEC>;
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
impl From<crate::W<TCD1_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD1_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR` reader - Source Address"]
pub type SADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SADDR` writer - Source Address"]
pub type SADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCD1_SADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<0> {
        SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_saddr](index.html) module"]
pub struct TCD1_SADDR_SPEC;
impl crate::RegisterSpec for TCD1_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd1_saddr::R](R) reader structure"]
impl crate::Readable for TCD1_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd1_saddr::W](W) writer structure"]
impl crate::Writable for TCD1_SADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD1_SADDR to value 0"]
impl crate::Resettable for TCD1_SADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
