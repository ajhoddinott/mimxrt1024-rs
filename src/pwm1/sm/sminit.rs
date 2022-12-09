#[doc = "Register `SMINIT` reader"]
pub struct R(crate::R<SMINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMINIT` writer"]
pub struct W(crate::W<SMINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMINIT_SPEC>;
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
impl From<crate::W<SMINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initial Count Register Bits"]
pub type INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT` writer - Initial Count Register Bits"]
pub type INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMINIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Initial Count Register Bits"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initial Count Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sminit](index.html) module"]
pub struct SMINIT_SPEC;
impl crate::RegisterSpec for SMINIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sminit::R](R) reader structure"]
impl crate::Readable for SMINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sminit::W](W) writer structure"]
impl crate::Writable for SMINIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMINIT to value 0"]
impl crate::Resettable for SMINIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
