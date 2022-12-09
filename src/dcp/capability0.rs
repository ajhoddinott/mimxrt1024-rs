#[doc = "Register `CAPABILITY0` reader"]
pub struct R(crate::R<CAPABILITY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPABILITY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPABILITY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPABILITY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPABILITY0` writer"]
pub struct W(crate::W<CAPABILITY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPABILITY0_SPEC>;
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
impl From<crate::W<CAPABILITY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPABILITY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUM_KEYS` reader - Encoded value indicating the number of key-storage locations implemented in the design"]
pub type NUM_KEYS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_CHANNELS` reader - Encoded value indicating the number of channels implemented in the design"]
pub type NUM_CHANNELS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISABLE_UNIQUE_KEY` reader - Write to a 1 to disable the per-device unique key"]
pub type DISABLE_UNIQUE_KEY_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_UNIQUE_KEY` writer - Write to a 1 to disable the per-device unique key"]
pub type DISABLE_UNIQUE_KEY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CAPABILITY0_SPEC, bool, O>;
#[doc = "Field `DISABLE_DECRYPT` reader - Write to 1 to disable the decryption"]
pub type DISABLE_DECRYPT_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_DECRYPT` writer - Write to 1 to disable the decryption"]
pub type DISABLE_DECRYPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPABILITY0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Encoded value indicating the number of key-storage locations implemented in the design"]
    #[inline(always)]
    pub fn num_keys(&self) -> NUM_KEYS_R {
        NUM_KEYS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Encoded value indicating the number of channels implemented in the design"]
    #[inline(always)]
    pub fn num_channels(&self) -> NUM_CHANNELS_R {
        NUM_CHANNELS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    pub fn disable_unique_key(&self) -> DISABLE_UNIQUE_KEY_R {
        DISABLE_UNIQUE_KEY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Write to 1 to disable the decryption"]
    #[inline(always)]
    pub fn disable_decrypt(&self) -> DISABLE_DECRYPT_R {
        DISABLE_DECRYPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Write to a 1 to disable the per-device unique key"]
    #[inline(always)]
    #[must_use]
    pub fn disable_unique_key(&mut self) -> DISABLE_UNIQUE_KEY_W<29> {
        DISABLE_UNIQUE_KEY_W::new(self)
    }
    #[doc = "Bit 31 - Write to 1 to disable the decryption"]
    #[inline(always)]
    #[must_use]
    pub fn disable_decrypt(&mut self) -> DISABLE_DECRYPT_W<31> {
        DISABLE_DECRYPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP capability 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capability0](index.html) module"]
pub struct CAPABILITY0_SPEC;
impl crate::RegisterSpec for CAPABILITY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capability0::R](R) reader structure"]
impl crate::Readable for CAPABILITY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capability0::W](W) writer structure"]
impl crate::Writable for CAPABILITY0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPABILITY0 to value 0x0404"]
impl crate::Resettable for CAPABILITY0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0404;
}
