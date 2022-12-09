#[doc = "Register `RMR` reader"]
pub struct R(crate::R<RMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMR` writer"]
pub struct W(crate::W<RMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMR_SPEC>;
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
impl From<crate::W<RMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWM` reader - Receive Word Mask"]
pub type RWM_R = crate::FieldReader<u32, RWM_A>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RWM_A {
    #[doc = "0: Word N is enabled."]
    WORD_N_ENABLED = 0,
    #[doc = "1: Word N is masked."]
    WORD_N_MASKED = 1,
}
impl From<RWM_A> for u32 {
    #[inline(always)]
    fn from(variant: RWM_A) -> Self {
        variant as _
    }
}
impl RWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RWM_A> {
        match self.bits {
            0 => Some(RWM_A::WORD_N_ENABLED),
            1 => Some(RWM_A::WORD_N_MASKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WORD_N_ENABLED`"]
    #[inline(always)]
    pub fn is_word_n_enabled(&self) -> bool {
        *self == RWM_A::WORD_N_ENABLED
    }
    #[doc = "Checks if the value of the field is `WORD_N_MASKED`"]
    #[inline(always)]
    pub fn is_word_n_masked(&self) -> bool {
        *self == RWM_A::WORD_N_MASKED
    }
}
#[doc = "Field `RWM` writer - Receive Word Mask"]
pub type RWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RMR_SPEC, u32, RWM_A, 32, O>;
impl<'a, const O: u8> RWM_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn word_n_enabled(self) -> &'a mut W {
        self.variant(RWM_A::WORD_N_ENABLED)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn word_n_masked(self) -> &'a mut W {
        self.variant(RWM_A::WORD_N_MASKED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm(&self) -> RWM_R {
        RWM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm(&mut self) -> RWM_W<0> {
        RWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmr](index.html) module"]
pub struct RMR_SPEC;
impl crate::RegisterSpec for RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmr::R](R) reader structure"]
impl crate::Readable for RMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmr::W](W) writer structure"]
impl crate::Writable for RMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
