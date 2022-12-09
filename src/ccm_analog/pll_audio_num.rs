#[doc = "Register `PLL_AUDIO_NUM` reader"]
pub struct R(crate::R<PLL_AUDIO_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_AUDIO_NUM` writer"]
pub struct W(crate::W<PLL_AUDIO_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO_NUM_SPEC>;
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
impl From<crate::W<PLL_AUDIO_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A` reader - 30 bit numerator of fractional loop divider."]
pub type A_R = crate::FieldReader<u32, u32>;
#[doc = "Field `A` writer - 30 bit numerator of fractional loop divider."]
pub type A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_AUDIO_NUM_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - 30 bit numerator of fractional loop divider."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 30 bit numerator of fractional loop divider."]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> A_W<0> {
        A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio_num](index.html) module"]
pub struct PLL_AUDIO_NUM_SPEC;
impl crate::RegisterSpec for PLL_AUDIO_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio_num::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio_num::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_AUDIO_NUM to value 0x05f5_e100"]
impl crate::Resettable for PLL_AUDIO_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x05f5_e100;
}
