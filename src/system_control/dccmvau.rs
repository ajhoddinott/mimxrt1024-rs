#[doc = "Register `DCCMVAU` writer"]
pub struct W(crate::W<DCCMVAU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCMVAU_SPEC>;
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
impl From<crate::W<DCCMVAU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCMVAU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCCMVAU` writer - D-cache clean by MVA to PoU"]
pub type DCCMVAU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCCMVAU_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - D-cache clean by MVA to PoU"]
    #[inline(always)]
    #[must_use]
    pub fn dccmvau(&mut self) -> DCCMVAU_W<0> {
        DCCMVAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data cache by address to PoU\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmvau](index.html) module"]
pub struct DCCMVAU_SPEC;
impl crate::RegisterSpec for DCCMVAU_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dccmvau::W](W) writer structure"]
impl crate::Writable for DCCMVAU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCMVAU to value 0"]
impl crate::Resettable for DCCMVAU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
