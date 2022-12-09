#[doc = "Register `PLL_AUDIO_DENOM` reader"]
pub struct R(crate::R<PLL_AUDIO_DENOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO_DENOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO_DENOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO_DENOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_AUDIO_DENOM` writer"]
pub struct W(crate::W<PLL_AUDIO_DENOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO_DENOM_SPEC>;
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
impl From<crate::W<PLL_AUDIO_DENOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO_DENOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B` reader - 30 bit denominator of fractional loop divider."]
pub type B_R = crate::FieldReader<u32, u32>;
#[doc = "Field `B` writer - 30 bit denominator of fractional loop divider."]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_AUDIO_DENOM_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - 30 bit denominator of fractional loop divider."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 30 bit denominator of fractional loop divider."]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio_denom](index.html) module"]
pub struct PLL_AUDIO_DENOM_SPEC;
impl crate::RegisterSpec for PLL_AUDIO_DENOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio_denom::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO_DENOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio_denom::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO_DENOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_AUDIO_DENOM to value 0x2964_619c"]
impl crate::Resettable for PLL_AUDIO_DENOM_SPEC {
    const RESET_VALUE: Self::Ux = 0x2964_619c;
}
