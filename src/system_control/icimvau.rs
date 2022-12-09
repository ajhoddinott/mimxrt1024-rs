#[doc = "Register `ICIMVAU` writer"]
pub struct W(crate::W<ICIMVAU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICIMVAU_SPEC>;
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
impl From<crate::W<ICIMVAU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICIMVAU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICIMVAU` writer - I-cache invalidate by MVA to PoU"]
pub type ICIMVAU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICIMVAU_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - I-cache invalidate by MVA to PoU"]
    #[inline(always)]
    #[must_use]
    pub fn icimvau(&mut self) -> ICIMVAU_W<0> {
        ICIMVAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction cache invalidate by address to PoU\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icimvau](index.html) module"]
pub struct ICIMVAU_SPEC;
impl crate::RegisterSpec for ICIMVAU_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icimvau::W](W) writer structure"]
impl crate::Writable for ICIMVAU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICIMVAU to value 0"]
impl crate::Resettable for ICIMVAU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
