#[doc = "Register `PLL_AUDIO_CLR` reader"]
pub struct R(crate::R<PLL_AUDIO_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_AUDIO_CLR` writer"]
pub struct W(crate::W<PLL_AUDIO_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO_CLR_SPEC>;
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
impl From<crate::W<PLL_AUDIO_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_SELECT` reader - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
pub type DIV_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_SELECT` writer - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
pub type DIV_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_AUDIO_CLR_SPEC, u8, u8, 7, O>;
#[doc = "Field `POWERDOWN` reader - Powers down the PLL."]
pub type POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `POWERDOWN` writer - Powers down the PLL."]
pub type POWERDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_AUDIO_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable PLL output"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable PLL output"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_AUDIO_CLR_SPEC, bool, O>;
#[doc = "Field `BYPASS_CLK_SRC` reader - Determines the bypass source."]
pub type BYPASS_CLK_SRC_R = crate::FieldReader<u8, BYPASS_CLK_SRC_A>;
#[doc = "Determines the bypass source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYPASS_CLK_SRC_A {
    #[doc = "0: Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
}
impl From<BYPASS_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_CLK_SRC_A) -> Self {
        variant as _
    }
}
impl BYPASS_CLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BYPASS_CLK_SRC_A> {
        match self.bits {
            0 => Some(BYPASS_CLK_SRC_A::REF_CLK_24M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REF_CLK_24M`"]
    #[inline(always)]
    pub fn is_ref_clk_24m(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::REF_CLK_24M
    }
}
#[doc = "Field `BYPASS_CLK_SRC` writer - Determines the bypass source."]
pub type BYPASS_CLK_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_AUDIO_CLR_SPEC, u8, BYPASS_CLK_SRC_A, 2, O>;
impl<'a, const O: u8> BYPASS_CLK_SRC_W<'a, O> {
    #[doc = "Select the 24MHz oscillator as source."]
    #[inline(always)]
    pub fn ref_clk_24m(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::REF_CLK_24M)
    }
}
#[doc = "Field `BYPASS` reader - Bypass the PLL."]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypass the PLL."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_AUDIO_CLR_SPEC, bool, O>;
#[doc = "Field `POST_DIV_SELECT` reader - These bits implement a divider after the PLL, but before the enable and bypass mux."]
pub type POST_DIV_SELECT_R = crate::FieldReader<u8, POST_DIV_SELECT_A>;
#[doc = "These bits implement a divider after the PLL, but before the enable and bypass mux.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_DIV_SELECT_A {
    #[doc = "0: Divide by 4."]
    POST_DIV_SELECT_0 = 0,
    #[doc = "1: Divide by 2."]
    POST_DIV_SELECT_1 = 1,
    #[doc = "2: Divide by 1."]
    POST_DIV_SELECT_2 = 2,
}
impl From<POST_DIV_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_DIV_SELECT_A) -> Self {
        variant as _
    }
}
impl POST_DIV_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POST_DIV_SELECT_A> {
        match self.bits {
            0 => Some(POST_DIV_SELECT_A::POST_DIV_SELECT_0),
            1 => Some(POST_DIV_SELECT_A::POST_DIV_SELECT_1),
            2 => Some(POST_DIV_SELECT_A::POST_DIV_SELECT_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_0`"]
    #[inline(always)]
    pub fn is_post_div_select_0(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_0
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_1`"]
    #[inline(always)]
    pub fn is_post_div_select_1(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_1
    }
    #[doc = "Checks if the value of the field is `POST_DIV_SELECT_2`"]
    #[inline(always)]
    pub fn is_post_div_select_2(&self) -> bool {
        *self == POST_DIV_SELECT_A::POST_DIV_SELECT_2
    }
}
#[doc = "Field `POST_DIV_SELECT` writer - These bits implement a divider after the PLL, but before the enable and bypass mux."]
pub type POST_DIV_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_AUDIO_CLR_SPEC, u8, POST_DIV_SELECT_A, 2, O>;
impl<'a, const O: u8> POST_DIV_SELECT_W<'a, O> {
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn post_div_select_0(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_0)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn post_div_select_1(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_1)
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn post_div_select_2(self) -> &'a mut W {
        self.variant(POST_DIV_SELECT_A::POST_DIV_SELECT_2)
    }
}
#[doc = "Field `LOCK` reader - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
pub type LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:6 - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    pub fn div_select(&self) -> DIV_SELECT_R {
        DIV_SELECT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable PLL output"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&self) -> BYPASS_CLK_SRC_R {
        BYPASS_CLK_SRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 19:20 - These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    pub fn post_div_select(&self) -> POST_DIV_SELECT_R {
        POST_DIV_SELECT_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 31 - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This field controls the PLL loop divider. Valid range for DIV_SELECT divider value: 27~54."]
    #[inline(always)]
    #[must_use]
    pub fn div_select(&mut self) -> DIV_SELECT_W<0> {
        DIV_SELECT_W::new(self)
    }
    #[doc = "Bit 12 - Powers down the PLL."]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> POWERDOWN_W<12> {
        POWERDOWN_W::new(self)
    }
    #[doc = "Bit 13 - Enable PLL output"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<13> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_clk_src(&mut self) -> BYPASS_CLK_SRC_W<14> {
        BYPASS_CLK_SRC_W::new(self)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<16> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 19:20 - These bits implement a divider after the PLL, but before the enable and bypass mux."]
    #[inline(always)]
    #[must_use]
    pub fn post_div_select(&mut self) -> POST_DIV_SELECT_W<19> {
        POST_DIV_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Audio PLL control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio_clr](index.html) module"]
pub struct PLL_AUDIO_CLR_SPEC;
impl crate::RegisterSpec for PLL_AUDIO_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio_clr::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio_clr::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_AUDIO_CLR to value 0x0001_1006"]
impl crate::Resettable for PLL_AUDIO_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1006;
}
