#[doc = "Register `CCSR` reader"]
pub struct R(crate::R<CCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCSR` writer"]
pub struct W(crate::W<CCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCSR_SPEC>;
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
impl From<crate::W<CCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL3_SW_CLK_SEL` reader - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
pub type PLL3_SW_CLK_SEL_R = crate::BitReader<PLL3_SW_CLK_SEL_A>;
#[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL3_SW_CLK_SEL_A {
    #[doc = "0: pll3_main_clk"]
    PLL3_SW_CLK_SEL_0 = 0,
    #[doc = "1: pll3 bypass clock"]
    PLL3_SW_CLK_SEL_1 = 1,
}
impl From<PLL3_SW_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3_SW_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL3_SW_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3_SW_CLK_SEL_A {
        match self.bits {
            false => PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0,
            true => PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_pll3_sw_clk_sel_0(&self) -> bool {
        *self == PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_pll3_sw_clk_sel_1(&self) -> bool {
        *self == PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1
    }
}
#[doc = "Field `PLL3_SW_CLK_SEL` writer - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
pub type PLL3_SW_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCSR_SPEC, PLL3_SW_CLK_SEL_A, O>;
impl<'a, const O: u8> PLL3_SW_CLK_SEL_W<'a, O> {
    #[doc = "pll3_main_clk"]
    #[inline(always)]
    pub fn pll3_sw_clk_sel_0(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0)
    }
    #[doc = "pll3 bypass clock"]
    #[inline(always)]
    pub fn pll3_sw_clk_sel_1(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub fn pll3_sw_clk_sel(&self) -> PLL3_SW_CLK_SEL_R {
        PLL3_SW_CLK_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    #[must_use]
    pub fn pll3_sw_clk_sel(&mut self) -> PLL3_SW_CLK_SEL_W<0> {
        PLL3_SW_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Clock Switcher Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsr](index.html) module"]
pub struct CCSR_SPEC;
impl crate::RegisterSpec for CCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccsr::R](R) reader structure"]
impl crate::Readable for CCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccsr::W](W) writer structure"]
impl crate::Writable for CCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCSR to value 0x0100"]
impl crate::Resettable for CCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
