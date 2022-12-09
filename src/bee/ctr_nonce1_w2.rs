#[doc = "Register `CTR_NONCE1_W2` writer"]
pub struct W(crate::W<CTR_NONCE1_W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_NONCE1_W2_SPEC>;
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
impl From<crate::W<CTR_NONCE1_W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_NONCE1_W2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONCE12` writer - Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
pub type NONCE12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTR_NONCE1_W2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Nonce1 from software for CTR, for region1. Nonce1={Nonce13,Nonce12,Nonce11,Nonce10}"]
    #[inline(always)]
    #[must_use]
    pub fn nonce12(&mut self) -> NONCE12_W<0> {
        NONCE12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NONCE12 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr_nonce1_w2](index.html) module"]
pub struct CTR_NONCE1_W2_SPEC;
impl crate::RegisterSpec for CTR_NONCE1_W2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctr_nonce1_w2::W](W) writer structure"]
impl crate::Writable for CTR_NONCE1_W2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR_NONCE1_W2 to value 0"]
impl crate::Resettable for CTR_NONCE1_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
