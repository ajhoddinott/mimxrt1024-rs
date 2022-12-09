#[doc = "Register `SCML` reader"]
pub struct R(crate::R<SCML_MC_SCML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCML_MC_SCML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCML_MC_SCML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCML_MC_SCML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCML` writer"]
pub struct W(crate::W<SCML_MC_SCML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCML_MC_SCML_SPEC>;
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
impl From<crate::W<SCML_MC_SCML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCML_MC_SCML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONO_MAX` reader - Monobit Maximum Limit"]
pub type MONO_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MONO_MAX` writer - Monobit Maximum Limit"]
pub type MONO_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCML_MC_SCML_SPEC, u16, u16, 16, O>;
#[doc = "Field `MONO_RNG` reader - Monobit Range"]
pub type MONO_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MONO_RNG` writer - Monobit Range"]
pub type MONO_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCML_MC_SCML_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&self) -> MONO_MAX_R {
        MONO_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&self) -> MONO_RNG_R {
        MONO_RNG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn mono_max(&mut self) -> MONO_MAX_W<0> {
        MONO_MAX_W::new(self)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    #[must_use]
    pub fn mono_rng(&mut self) -> MONO_RNG_W<16> {
        MONO_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Monobit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scml_mc_scml](index.html) module"]
pub struct SCML_MC_SCML_SPEC;
impl crate::RegisterSpec for SCML_MC_SCML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scml_mc_scml::R](R) reader structure"]
impl crate::Readable for SCML_MC_SCML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scml_mc_scml::W](W) writer structure"]
impl crate::Writable for SCML_MC_SCML_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCML to value 0x010c_0568"]
impl crate::Resettable for SCML_MC_SCML_SPEC {
    const RESET_VALUE: Self::Ux = 0x010c_0568;
}
