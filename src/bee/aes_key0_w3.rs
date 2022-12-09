#[doc = "Register `AES_KEY0_W3` writer"]
pub struct W(crate::W<AES_KEY0_W3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_KEY0_W3_SPEC>;
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
impl From<crate::W<AES_KEY0_W3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_KEY0_W3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY3` writer - AES 128 key from software"]
pub type KEY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AES_KEY0_W3_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - AES 128 key from software"]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> KEY3_W<0> {
        KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Key 3 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_key0_w3](index.html) module"]
pub struct AES_KEY0_W3_SPEC;
impl crate::RegisterSpec for AES_KEY0_W3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [aes_key0_w3::W](W) writer structure"]
impl crate::Writable for AES_KEY0_W3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_KEY0_W3 to value 0"]
impl crate::Resettable for AES_KEY0_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
