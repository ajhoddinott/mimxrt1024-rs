#[doc = "Register `SMCAPTCOMPX` reader"]
pub struct R(crate::R<SMCAPTCOMPX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCOMPX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCOMPX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCOMPX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCOMPX` writer"]
pub struct W(crate::W<SMCAPTCOMPX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCOMPX_SPEC>;
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
impl From<crate::W<SMCAPTCOMPX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCOMPX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGCMPX` reader - Edge Compare X"]
pub type EDGCMPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGCMPX` writer - Edge Compare X"]
pub type EDGCMPX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCOMPX_SPEC, u8, u8, 8, O>;
#[doc = "Field `EDGCNTX` reader - Edge Counter X"]
pub type EDGCNTX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare X"]
    #[inline(always)]
    pub fn edgcmpx(&self) -> EDGCMPX_R {
        EDGCMPX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter X"]
    #[inline(always)]
    pub fn edgcntx(&self) -> EDGCNTX_R {
        EDGCNTX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare X"]
    #[inline(always)]
    #[must_use]
    pub fn edgcmpx(&mut self) -> EDGCMPX_W<0> {
        EDGCMPX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Compare X Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompx](index.html) module"]
pub struct SMCAPTCOMPX_SPEC;
impl crate::RegisterSpec for SMCAPTCOMPX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptcompx::R](R) reader structure"]
impl crate::Readable for SMCAPTCOMPX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptcompx::W](W) writer structure"]
impl crate::Writable for SMCAPTCOMPX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCOMPX to value 0"]
impl crate::Resettable for SMCAPTCOMPX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
