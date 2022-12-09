#[doc = "Register `CISR` reader"]
pub struct R(crate::R<CISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CISR` writer"]
pub struct W(crate::W<CISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CISR_SPEC>;
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
impl From<crate::W<CISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRF_PLL` reader - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
pub type LRF_PLL_R = crate::BitReader<LRF_PLL_A>;
#[doc = "CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRF_PLL_A {
    #[doc = "0: interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_0 = 0,
    #[doc = "1: interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    LRF_PLL_1 = 1,
}
impl From<LRF_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: LRF_PLL_A) -> Self {
        variant as u8 != 0
    }
}
impl LRF_PLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRF_PLL_A {
        match self.bits {
            false => LRF_PLL_A::LRF_PLL_0,
            true => LRF_PLL_A::LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_0`"]
    #[inline(always)]
    pub fn is_lrf_pll_0(&self) -> bool {
        *self == LRF_PLL_A::LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `LRF_PLL_1`"]
    #[inline(always)]
    pub fn is_lrf_pll_1(&self) -> bool {
        *self == LRF_PLL_A::LRF_PLL_1
    }
}
#[doc = "Field `LRF_PLL` writer - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
pub type LRF_PLL_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CISR_SPEC, LRF_PLL_A, O>;
impl<'a, const O: u8> LRF_PLL_W<'a, O> {
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll_0(self) -> &'a mut W {
        self.variant(LRF_PLL_A::LRF_PLL_0)
    }
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll_1(self) -> &'a mut W {
        self.variant(LRF_PLL_A::LRF_PLL_1)
    }
}
#[doc = "Field `COSC_READY` reader - CCM interrupt request 2 generated due to on board oscillator ready, i"]
pub type COSC_READY_R = crate::BitReader<COSC_READY_A>;
#[doc = "CCM interrupt request 2 generated due to on board oscillator ready, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSC_READY_A {
    #[doc = "0: interrupt is not generated due to on board oscillator ready"]
    COSC_READY_0 = 0,
    #[doc = "1: interrupt generated due to on board oscillator ready"]
    COSC_READY_1 = 1,
}
impl From<COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl COSC_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_READY_A {
        match self.bits {
            false => COSC_READY_A::COSC_READY_0,
            true => COSC_READY_A::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline(always)]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline(always)]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_1
    }
}
#[doc = "Field `COSC_READY` writer - CCM interrupt request 2 generated due to on board oscillator ready, i"]
pub type COSC_READY_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CISR_SPEC, COSC_READY_A, O>;
impl<'a, const O: u8> COSC_READY_W<'a, O> {
    #[doc = "interrupt is not generated due to on board oscillator ready"]
    #[inline(always)]
    pub fn cosc_ready_0(self) -> &'a mut W {
        self.variant(COSC_READY_A::COSC_READY_0)
    }
    #[doc = "interrupt generated due to on board oscillator ready"]
    #[inline(always)]
    pub fn cosc_ready_1(self) -> &'a mut W {
        self.variant(COSC_READY_A::COSC_READY_1)
    }
}
#[doc = "Field `SEMC_PODF_LOADED` reader - CCM interrupt request 1 generated due to frequency change of semc_podf"]
pub type SEMC_PODF_LOADED_R = crate::BitReader<SEMC_PODF_LOADED_A>;
#[doc = "CCM interrupt request 1 generated due to frequency change of semc_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_0 = 0,
    #[doc = "1: interrupt generated due to frequency change of semc_podf"]
    SEMC_PODF_LOADED_1 = 1,
}
impl From<SEMC_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_PODF_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_PODF_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_LOADED_A {
        match self.bits {
            false => SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0,
            true => SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_semc_podf_loaded_0(&self) -> bool {
        *self == SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_semc_podf_loaded_1(&self) -> bool {
        *self == SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1
    }
}
#[doc = "Field `SEMC_PODF_LOADED` writer - CCM interrupt request 1 generated due to frequency change of semc_podf"]
pub type SEMC_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CISR_SPEC, SEMC_PODF_LOADED_A, O>;
impl<'a, const O: u8> SEMC_PODF_LOADED_W<'a, O> {
    #[doc = "interrupt is not generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(SEMC_PODF_LOADED_A::SEMC_PODF_LOADED_1)
    }
}
#[doc = "Field `PERIPH2_CLK_SEL_LOADED` reader - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
pub type PERIPH2_CLK_SEL_LOADED_R = crate::BitReader<PERIPH2_CLK_SEL_LOADED_A>;
#[doc = "CCM interrupt request 1 generated due to frequency change of periph2_clk_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPH2_CLK_SEL_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_0 = 0,
    #[doc = "1: interrupt generated due to frequency change of periph2_clk_sel"]
    PERIPH2_CLK_SEL_LOADED_1 = 1,
}
impl From<PERIPH2_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH2_CLK_SEL_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPH2_CLK_SEL_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH2_CLK_SEL_LOADED_A {
        match self.bits {
            false => PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0,
            true => PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Field `PERIPH2_CLK_SEL_LOADED` writer - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
pub type PERIPH2_CLK_SEL_LOADED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CISR_SPEC, PERIPH2_CLK_SEL_LOADED_A, O>;
impl<'a, const O: u8> PERIPH2_CLK_SEL_LOADED_W<'a, O> {
    #[doc = "interrupt is not generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH2_CLK_SEL_LOADED_A::PERIPH2_CLK_SEL_LOADED_1)
    }
}
#[doc = "Field `AHB_PODF_LOADED` reader - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
pub type AHB_PODF_LOADED_R = crate::BitReader<AHB_PODF_LOADED_A>;
#[doc = "CCM interrupt request 1 generated due to frequency change of ahb_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_0 = 0,
    #[doc = "1: interrupt generated due to frequency change of ahb_podf"]
    AHB_PODF_LOADED_1 = 1,
}
impl From<AHB_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_PODF_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_PODF_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_LOADED_A {
        match self.bits {
            false => AHB_PODF_LOADED_A::AHB_PODF_LOADED_0,
            true => AHB_PODF_LOADED_A::AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_loaded_0(&self) -> bool {
        *self == AHB_PODF_LOADED_A::AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_loaded_1(&self) -> bool {
        *self == AHB_PODF_LOADED_A::AHB_PODF_LOADED_1
    }
}
#[doc = "Field `AHB_PODF_LOADED` writer - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
pub type AHB_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CISR_SPEC, AHB_PODF_LOADED_A, O>;
impl<'a, const O: u8> AHB_PODF_LOADED_W<'a, O> {
    #[doc = "interrupt is not generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADED_A::AHB_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(AHB_PODF_LOADED_A::AHB_PODF_LOADED_1)
    }
}
#[doc = "Field `PERIPH_CLK_SEL_LOADED` reader - CCM interrupt request 1 generated due to update of periph_clk_sel."]
pub type PERIPH_CLK_SEL_LOADED_R = crate::BitReader<PERIPH_CLK_SEL_LOADED_A>;
#[doc = "CCM interrupt request 1 generated due to update of periph_clk_sel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPH_CLK_SEL_LOADED_A {
    #[doc = "0: interrupt is not generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_0 = 0,
    #[doc = "1: interrupt generated due to update of periph_clk_sel."]
    PERIPH_CLK_SEL_LOADED_1 = 1,
}
impl From<PERIPH_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPH_CLK_SEL_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_LOADED_A {
        match self.bits {
            false => PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0,
            true => PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_loaded_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_loaded_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Field `PERIPH_CLK_SEL_LOADED` writer - CCM interrupt request 1 generated due to update of periph_clk_sel."]
pub type PERIPH_CLK_SEL_LOADED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CISR_SPEC, PERIPH_CLK_SEL_LOADED_A, O>;
impl<'a, const O: u8> PERIPH_CLK_SEL_LOADED_W<'a, O> {
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_LOADED_A::PERIPH_CLK_SEL_LOADED_1)
    }
}
#[doc = "Field `ARM_PODF_LOADED` reader - CCM interrupt request 1 generated due to frequency change of arm_podf"]
pub type ARM_PODF_LOADED_R = crate::BitReader<ARM_PODF_LOADED_A>;
#[doc = "CCM interrupt request 1 generated due to frequency change of arm_podf\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_PODF_LOADED_A {
    #[doc = "0: interrupt is not generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_0 = 0,
    #[doc = "1: interrupt generated due to frequency change of arm_podf"]
    ARM_PODF_LOADED_1 = 1,
}
impl From<ARM_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_PODF_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_PODF_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_LOADED_A {
        match self.bits {
            false => ARM_PODF_LOADED_A::ARM_PODF_LOADED_0,
            true => ARM_PODF_LOADED_A::ARM_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_arm_podf_loaded_0(&self) -> bool {
        *self == ARM_PODF_LOADED_A::ARM_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_arm_podf_loaded_1(&self) -> bool {
        *self == ARM_PODF_LOADED_A::ARM_PODF_LOADED_1
    }
}
#[doc = "Field `ARM_PODF_LOADED` writer - CCM interrupt request 1 generated due to frequency change of arm_podf"]
pub type ARM_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, CISR_SPEC, ARM_PODF_LOADED_A, O>;
impl<'a, const O: u8> ARM_PODF_LOADED_W<'a, O> {
    #[doc = "interrupt is not generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_0)
    }
    #[doc = "interrupt generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded_1(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_1)
    }
}
impl R {
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    pub fn lrf_pll(&self) -> LRF_PLL_R {
        LRF_PLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    pub fn cosc_ready(&self) -> COSC_READY_R {
        COSC_READY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn semc_podf_loaded(&self) -> SEMC_PODF_LOADED_R {
        SEMC_PODF_LOADED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    pub fn periph2_clk_sel_loaded(&self) -> PERIPH2_CLK_SEL_LOADED_R {
        PERIPH2_CLK_SEL_LOADED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn ahb_podf_loaded(&self) -> AHB_PODF_LOADED_R {
        AHB_PODF_LOADED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn periph_clk_sel_loaded(&self) -> PERIPH_CLK_SEL_LOADED_R {
        PERIPH_CLK_SEL_LOADED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&self) -> ARM_PODF_LOADED_R {
        ARM_PODF_LOADED_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM interrupt request 2 generated due to lock of all enabled and not bypaseed PLLs"]
    #[inline(always)]
    #[must_use]
    pub fn lrf_pll(&mut self) -> LRF_PLL_W<0> {
        LRF_PLL_W::new(self)
    }
    #[doc = "Bit 6 - CCM interrupt request 2 generated due to on board oscillator ready, i"]
    #[inline(always)]
    #[must_use]
    pub fn cosc_ready(&mut self) -> COSC_READY_W<6> {
        COSC_READY_W::new(self)
    }
    #[doc = "Bit 17 - CCM interrupt request 1 generated due to frequency change of semc_podf"]
    #[inline(always)]
    #[must_use]
    pub fn semc_podf_loaded(&mut self) -> SEMC_PODF_LOADED_W<17> {
        SEMC_PODF_LOADED_W::new(self)
    }
    #[doc = "Bit 19 - CCM interrupt request 1 generated due to frequency change of periph2_clk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn periph2_clk_sel_loaded(&mut self) -> PERIPH2_CLK_SEL_LOADED_W<19> {
        PERIPH2_CLK_SEL_LOADED_W::new(self)
    }
    #[doc = "Bit 20 - CCM interrupt request 1 generated due to frequency change of ahb_podf"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_podf_loaded(&mut self) -> AHB_PODF_LOADED_W<20> {
        AHB_PODF_LOADED_W::new(self)
    }
    #[doc = "Bit 22 - CCM interrupt request 1 generated due to update of periph_clk_sel."]
    #[inline(always)]
    #[must_use]
    pub fn periph_clk_sel_loaded(&mut self) -> PERIPH_CLK_SEL_LOADED_W<22> {
        PERIPH_CLK_SEL_LOADED_W::new(self)
    }
    #[doc = "Bit 26 - CCM interrupt request 1 generated due to frequency change of arm_podf"]
    #[inline(always)]
    #[must_use]
    pub fn arm_podf_loaded(&mut self) -> ARM_PODF_LOADED_W<26> {
        ARM_PODF_LOADED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cisr](index.html) module"]
pub struct CISR_SPEC;
impl crate::RegisterSpec for CISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cisr::R](R) reader structure"]
impl crate::Readable for CISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cisr::W](W) writer structure"]
impl crate::Writable for CISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x045a_0041;
}
#[doc = "`reset()` method sets CISR to value 0"]
impl crate::Resettable for CISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
