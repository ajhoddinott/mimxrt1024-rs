#[doc = "Register `CIMR` reader"]
pub struct R(crate::R<CIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIMR` writer"]
pub struct W(crate::W<CIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIMR_SPEC>;
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
impl From<crate::W<CIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_LRF_PLL` reader - mask interrupt generation due to lrf of PLLs"]
pub type MASK_LRF_PLL_R = crate::BitReader<MASK_LRF_PLL_A>;
#[doc = "mask interrupt generation due to lrf of PLLs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_LRF_PLL_A {
    #[doc = "0: don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    MASK_LRF_PLL_0 = 0,
    #[doc = "1: mask interrupt due to lrf of PLLs"]
    MASK_LRF_PLL_1 = 1,
}
impl From<MASK_LRF_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_LRF_PLL_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_LRF_PLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_LRF_PLL_A {
        match self.bits {
            false => MASK_LRF_PLL_A::MASK_LRF_PLL_0,
            true => MASK_LRF_PLL_A::MASK_LRF_PLL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_0`"]
    #[inline(always)]
    pub fn is_mask_lrf_pll_0(&self) -> bool {
        *self == MASK_LRF_PLL_A::MASK_LRF_PLL_0
    }
    #[doc = "Checks if the value of the field is `MASK_LRF_PLL_1`"]
    #[inline(always)]
    pub fn is_mask_lrf_pll_1(&self) -> bool {
        *self == MASK_LRF_PLL_A::MASK_LRF_PLL_1
    }
}
#[doc = "Field `MASK_LRF_PLL` writer - mask interrupt generation due to lrf of PLLs"]
pub type MASK_LRF_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIMR_SPEC, MASK_LRF_PLL_A, O>;
impl<'a, const O: u8> MASK_LRF_PLL_W<'a, O> {
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created"]
    #[inline(always)]
    pub fn mask_lrf_pll_0(self) -> &'a mut W {
        self.variant(MASK_LRF_PLL_A::MASK_LRF_PLL_0)
    }
    #[doc = "mask interrupt due to lrf of PLLs"]
    #[inline(always)]
    pub fn mask_lrf_pll_1(self) -> &'a mut W {
        self.variant(MASK_LRF_PLL_A::MASK_LRF_PLL_1)
    }
}
#[doc = "Field `MASK_COSC_READY` reader - mask interrupt generation due to on board oscillator ready"]
pub type MASK_COSC_READY_R = crate::BitReader<MASK_COSC_READY_A>;
#[doc = "mask interrupt generation due to on board oscillator ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_COSC_READY_A {
    #[doc = "0: don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    MASK_COSC_READY_0 = 0,
    #[doc = "1: mask interrupt due to on board oscillator ready"]
    MASK_COSC_READY_1 = 1,
}
impl From<MASK_COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_COSC_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_COSC_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_COSC_READY_A {
        match self.bits {
            false => MASK_COSC_READY_A::MASK_COSC_READY_0,
            true => MASK_COSC_READY_A::MASK_COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_0`"]
    #[inline(always)]
    pub fn is_mask_cosc_ready_0(&self) -> bool {
        *self == MASK_COSC_READY_A::MASK_COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `MASK_COSC_READY_1`"]
    #[inline(always)]
    pub fn is_mask_cosc_ready_1(&self) -> bool {
        *self == MASK_COSC_READY_A::MASK_COSC_READY_1
    }
}
#[doc = "Field `MASK_COSC_READY` writer - mask interrupt generation due to on board oscillator ready"]
pub type MASK_COSC_READY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, MASK_COSC_READY_A, O>;
impl<'a, const O: u8> MASK_COSC_READY_W<'a, O> {
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created"]
    #[inline(always)]
    pub fn mask_cosc_ready_0(self) -> &'a mut W {
        self.variant(MASK_COSC_READY_A::MASK_COSC_READY_0)
    }
    #[doc = "mask interrupt due to on board oscillator ready"]
    #[inline(always)]
    pub fn mask_cosc_ready_1(self) -> &'a mut W {
        self.variant(MASK_COSC_READY_A::MASK_COSC_READY_1)
    }
}
#[doc = "Field `MASK_SEMC_PODF_LOADED` reader - mask interrupt generation due to frequency change of semc_podf"]
pub type MASK_SEMC_PODF_LOADED_R = crate::BitReader<MASK_SEMC_PODF_LOADED_A>;
#[doc = "mask interrupt generation due to frequency change of semc_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_SEMC_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    MASK_SEMC_PODF_LOADED_0 = 0,
    #[doc = "1: mask interrupt due to frequency change of semc_podf"]
    MASK_SEMC_PODF_LOADED_1 = 1,
}
impl From<MASK_SEMC_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_SEMC_PODF_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_SEMC_PODF_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_SEMC_PODF_LOADED_A {
        match self.bits {
            false => MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0,
            true => MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_semc_podf_loaded_0(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_SEMC_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_semc_podf_loaded_1(&self) -> bool {
        *self == MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1
    }
}
#[doc = "Field `MASK_SEMC_PODF_LOADED` writer - mask interrupt generation due to frequency change of semc_podf"]
pub type MASK_SEMC_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, MASK_SEMC_PODF_LOADED_A, O>;
impl<'a, const O: u8> MASK_SEMC_PODF_LOADED_W<'a, O> {
    #[doc = "don't mask interrupt due to frequency change of semc_podf - interrupt will be created"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_SEMC_PODF_LOADED_A::MASK_SEMC_PODF_LOADED_1)
    }
}
#[doc = "Field `MASK_PERIPH2_CLK_SEL_LOADED` reader - mask interrupt generation due to update of periph2_clk_sel."]
pub type MASK_PERIPH2_CLK_SEL_LOADED_R = crate::BitReader<MASK_PERIPH2_CLK_SEL_LOADED_A>;
#[doc = "mask interrupt generation due to update of periph2_clk_sel.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_PERIPH2_CLK_SEL_LOADED_A {
    #[doc = "0: don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    MASK_PERIPH2_CLK_SEL_LOADED_0 = 0,
    #[doc = "1: mask interrupt due to update of periph2_clk_sel"]
    MASK_PERIPH2_CLK_SEL_LOADED_1 = 1,
}
impl From<MASK_PERIPH2_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_PERIPH2_CLK_SEL_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_PERIPH2_CLK_SEL_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_PERIPH2_CLK_SEL_LOADED_A {
        match self.bits {
            false => MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0,
            true => MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_periph2_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH2_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_periph2_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1
    }
}
#[doc = "Field `MASK_PERIPH2_CLK_SEL_LOADED` writer - mask interrupt generation due to update of periph2_clk_sel."]
pub type MASK_PERIPH2_CLK_SEL_LOADED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, MASK_PERIPH2_CLK_SEL_LOADED_A, O>;
impl<'a, const O: u8> MASK_PERIPH2_CLK_SEL_LOADED_W<'a, O> {
    #[doc = "don't mask interrupt due to update of periph2_clk_sel - interrupt will be created"]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph2_clk_sel"]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH2_CLK_SEL_LOADED_A::MASK_PERIPH2_CLK_SEL_LOADED_1)
    }
}
#[doc = "Field `MASK_AHB_PODF_LOADED` reader - mask interrupt generation due to frequency change of ahb_podf"]
pub type MASK_AHB_PODF_LOADED_R = crate::BitReader<MASK_AHB_PODF_LOADED_A>;
#[doc = "mask interrupt generation due to frequency change of ahb_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_AHB_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    MASK_AHB_PODF_LOADED_0 = 0,
    #[doc = "1: mask interrupt due to frequency change of ahb_podf"]
    MASK_AHB_PODF_LOADED_1 = 1,
}
impl From<MASK_AHB_PODF_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_AHB_PODF_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_AHB_PODF_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_AHB_PODF_LOADED_A {
        match self.bits {
            false => MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0,
            true => MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_ahb_podf_loaded_0(&self) -> bool {
        *self == MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_AHB_PODF_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_ahb_podf_loaded_1(&self) -> bool {
        *self == MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1
    }
}
#[doc = "Field `MASK_AHB_PODF_LOADED` writer - mask interrupt generation due to frequency change of ahb_podf"]
pub type MASK_AHB_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, MASK_AHB_PODF_LOADED_A, O>;
impl<'a, const O: u8> MASK_AHB_PODF_LOADED_W<'a, O> {
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded_0(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded_1(self) -> &'a mut W {
        self.variant(MASK_AHB_PODF_LOADED_A::MASK_AHB_PODF_LOADED_1)
    }
}
#[doc = "Field `MASK_PERIPH_CLK_SEL_LOADED` reader - mask interrupt generation due to update of periph_clk_sel."]
pub type MASK_PERIPH_CLK_SEL_LOADED_R = crate::BitReader<MASK_PERIPH_CLK_SEL_LOADED_A>;
#[doc = "mask interrupt generation due to update of periph_clk_sel.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_PERIPH_CLK_SEL_LOADED_A {
    #[doc = "0: don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    MASK_PERIPH_CLK_SEL_LOADED_0 = 0,
    #[doc = "1: mask interrupt due to update of periph_clk_sel"]
    MASK_PERIPH_CLK_SEL_LOADED_1 = 1,
}
impl From<MASK_PERIPH_CLK_SEL_LOADED_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_PERIPH_CLK_SEL_LOADED_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_PERIPH_CLK_SEL_LOADED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_PERIPH_CLK_SEL_LOADED_A {
        match self.bits {
            false => MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0,
            true => MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_0`"]
    #[inline(always)]
    pub fn is_mask_periph_clk_sel_loaded_0(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0
    }
    #[doc = "Checks if the value of the field is `MASK_PERIPH_CLK_SEL_LOADED_1`"]
    #[inline(always)]
    pub fn is_mask_periph_clk_sel_loaded_1(&self) -> bool {
        *self == MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1
    }
}
#[doc = "Field `MASK_PERIPH_CLK_SEL_LOADED` writer - mask interrupt generation due to update of periph_clk_sel."]
pub type MASK_PERIPH_CLK_SEL_LOADED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, MASK_PERIPH_CLK_SEL_LOADED_A, O>;
impl<'a, const O: u8> MASK_PERIPH_CLK_SEL_LOADED_W<'a, O> {
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created"]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded_0(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_0)
    }
    #[doc = "mask interrupt due to update of periph_clk_sel"]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded_1(self) -> &'a mut W {
        self.variant(MASK_PERIPH_CLK_SEL_LOADED_A::MASK_PERIPH_CLK_SEL_LOADED_1)
    }
}
#[doc = "Field `ARM_PODF_LOADED` reader - mask interrupt generation due to frequency change of arm_podf"]
pub type ARM_PODF_LOADED_R = crate::BitReader<ARM_PODF_LOADED_A>;
#[doc = "mask interrupt generation due to frequency change of arm_podf\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_PODF_LOADED_A {
    #[doc = "0: don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    ARM_PODF_LOADED_0 = 0,
    #[doc = "1: mask interrupt due to frequency change of arm_podf"]
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
#[doc = "Field `ARM_PODF_LOADED` writer - mask interrupt generation due to frequency change of arm_podf"]
pub type ARM_PODF_LOADED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CIMR_SPEC, ARM_PODF_LOADED_A, O>;
impl<'a, const O: u8> ARM_PODF_LOADED_W<'a, O> {
    #[doc = "don't mask interrupt due to frequency change of arm_podf - interrupt will be created"]
    #[inline(always)]
    pub fn arm_podf_loaded_0(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_0)
    }
    #[doc = "mask interrupt due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded_1(self) -> &'a mut W {
        self.variant(ARM_PODF_LOADED_A::ARM_PODF_LOADED_1)
    }
}
impl R {
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    pub fn mask_lrf_pll(&self) -> MASK_LRF_PLL_R {
        MASK_LRF_PLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    pub fn mask_cosc_ready(&self) -> MASK_COSC_READY_R {
        MASK_COSC_READY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline(always)]
    pub fn mask_semc_podf_loaded(&self) -> MASK_SEMC_PODF_LOADED_R {
        MASK_SEMC_PODF_LOADED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline(always)]
    pub fn mask_periph2_clk_sel_loaded(&self) -> MASK_PERIPH2_CLK_SEL_LOADED_R {
        MASK_PERIPH2_CLK_SEL_LOADED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    pub fn mask_ahb_podf_loaded(&self) -> MASK_AHB_PODF_LOADED_R {
        MASK_AHB_PODF_LOADED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    pub fn mask_periph_clk_sel_loaded(&self) -> MASK_PERIPH_CLK_SEL_LOADED_R {
        MASK_PERIPH_CLK_SEL_LOADED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
    #[inline(always)]
    pub fn arm_podf_loaded(&self) -> ARM_PODF_LOADED_R {
        ARM_PODF_LOADED_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mask interrupt generation due to lrf of PLLs"]
    #[inline(always)]
    #[must_use]
    pub fn mask_lrf_pll(&mut self) -> MASK_LRF_PLL_W<0> {
        MASK_LRF_PLL_W::new(self)
    }
    #[doc = "Bit 6 - mask interrupt generation due to on board oscillator ready"]
    #[inline(always)]
    #[must_use]
    pub fn mask_cosc_ready(&mut self) -> MASK_COSC_READY_W<6> {
        MASK_COSC_READY_W::new(self)
    }
    #[doc = "Bit 17 - mask interrupt generation due to frequency change of semc_podf"]
    #[inline(always)]
    #[must_use]
    pub fn mask_semc_podf_loaded(&mut self) -> MASK_SEMC_PODF_LOADED_W<17> {
        MASK_SEMC_PODF_LOADED_W::new(self)
    }
    #[doc = "Bit 19 - mask interrupt generation due to update of periph2_clk_sel."]
    #[inline(always)]
    #[must_use]
    pub fn mask_periph2_clk_sel_loaded(&mut self) -> MASK_PERIPH2_CLK_SEL_LOADED_W<19> {
        MASK_PERIPH2_CLK_SEL_LOADED_W::new(self)
    }
    #[doc = "Bit 20 - mask interrupt generation due to frequency change of ahb_podf"]
    #[inline(always)]
    #[must_use]
    pub fn mask_ahb_podf_loaded(&mut self) -> MASK_AHB_PODF_LOADED_W<20> {
        MASK_AHB_PODF_LOADED_W::new(self)
    }
    #[doc = "Bit 22 - mask interrupt generation due to update of periph_clk_sel."]
    #[inline(always)]
    #[must_use]
    pub fn mask_periph_clk_sel_loaded(&mut self) -> MASK_PERIPH_CLK_SEL_LOADED_W<22> {
        MASK_PERIPH_CLK_SEL_LOADED_W::new(self)
    }
    #[doc = "Bit 26 - mask interrupt generation due to frequency change of arm_podf"]
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
#[doc = "CCM Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cimr](index.html) module"]
pub struct CIMR_SPEC;
impl crate::RegisterSpec for CIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cimr::R](R) reader structure"]
impl crate::Readable for CIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cimr::W](W) writer structure"]
impl crate::Writable for CIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIMR to value 0xffff_ffff"]
impl crate::Resettable for CIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
