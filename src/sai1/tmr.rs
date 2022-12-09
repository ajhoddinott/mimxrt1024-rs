#[doc = "Register `TMR` reader"]
pub struct R(crate::R<TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR` writer"]
pub struct W(crate::W<TMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_SPEC>;
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
impl From<crate::W<TMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWM` reader - Transmit Word Mask"]
pub type TWM_R = crate::FieldReader<u32, TWM_A>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TWM_A {
    #[doc = "0: Word N is enabled."]
    WORD_N_ENABLED = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    WORD_N_MASKED = 1,
}
impl From<TWM_A> for u32 {
    #[inline(always)]
    fn from(variant: TWM_A) -> Self {
        variant as _
    }
}
impl TWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TWM_A> {
        match self.bits {
            0 => Some(TWM_A::WORD_N_ENABLED),
            1 => Some(TWM_A::WORD_N_MASKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WORD_N_ENABLED`"]
    #[inline(always)]
    pub fn is_word_n_enabled(&self) -> bool {
        *self == TWM_A::WORD_N_ENABLED
    }
    #[doc = "Checks if the value of the field is `WORD_N_MASKED`"]
    #[inline(always)]
    pub fn is_word_n_masked(&self) -> bool {
        *self == TWM_A::WORD_N_MASKED
    }
}
#[doc = "Field `TWM` writer - Transmit Word Mask"]
pub type TWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMR_SPEC, u32, TWM_A, 32, O>;
impl<'a, const O: u8> TWM_W<'a, O> {
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn word_n_enabled(self) -> &'a mut W {
        self.variant(TWM_A::WORD_N_ENABLED)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated or drive zero when masked."]
    #[inline(always)]
    pub fn word_n_masked(self) -> &'a mut W {
        self.variant(TWM_A::WORD_N_MASKED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm(&self) -> TWM_R {
        TWM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm(&mut self) -> TWM_W<0> {
        TWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](index.html) module"]
pub struct TMR_SPEC;
impl crate::RegisterSpec for TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr::R](R) reader structure"]
impl crate::Readable for TMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr::W](W) writer structure"]
impl crate::Writable for TMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
