#[doc = "Register `PKRRNG` reader"]
pub struct R(crate::R<PKRRNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRRNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRRNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRRNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKRRNG` writer"]
pub struct W(crate::W<PKRRNG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKRRNG_SPEC>;
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
impl From<crate::W<PKRRNG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKRRNG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKR_RNG` reader - Poker Range"]
pub type PKR_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_RNG` writer - Poker Range"]
pub type PKR_RNG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKRRNG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    pub fn pkr_rng(&self) -> PKR_RNG_R {
        PKR_RNG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Poker Range"]
    #[inline(always)]
    #[must_use]
    pub fn pkr_rng(&mut self) -> PKR_RNG_W<0> {
        PKR_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Poker Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrrng](index.html) module"]
pub struct PKRRNG_SPEC;
impl crate::RegisterSpec for PKRRNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrrng::R](R) reader structure"]
impl crate::Readable for PKRRNG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkrrng::W](W) writer structure"]
impl crate::Writable for PKRRNG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKRRNG to value 0x09a3"]
impl crate::Resettable for PKRRNG_SPEC {
    const RESET_VALUE: Self::Ux = 0x09a3;
}
