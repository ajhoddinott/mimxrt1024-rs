#[doc = "Register `CCOSR` reader"]
pub struct R(crate::R<CCOSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCOSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCOSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCOSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCOSR` writer"]
pub struct W(crate::W<CCOSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCOSR_SPEC>;
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
impl From<crate::W<CCOSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCOSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKO1_SEL` reader - Selection of the clock to be generated on CCM_CLKO1"]
pub type CLKO1_SEL_R = crate::FieldReader<u8, CLKO1_SEL_A>;
#[doc = "Selection of the clock to be generated on CCM_CLKO1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKO1_SEL_A {
    #[doc = "0: pll3_sw_clk (divided by 2)"]
    CLKO1_SEL_0 = 0,
    #[doc = "1: PLL2 (divided by 2)"]
    CLKO1_SEL_1 = 1,
    #[doc = "2: ENET PLL (divided by 2)"]
    CLKO1_SEL_2 = 2,
    #[doc = "5: semc_clk_root"]
    CLKO1_SEL_5 = 5,
    #[doc = "11: ahb_clk_root"]
    CLKO1_SEL_11 = 11,
    #[doc = "12: ipg_clk_root"]
    CLKO1_SEL_12 = 12,
    #[doc = "13: perclk_root"]
    CLKO1_SEL_13 = 13,
    #[doc = "15: pll4_main_clk"]
    CLKO1_SEL_15 = 15,
}
impl From<CLKO1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO1_SEL_A) -> Self {
        variant as _
    }
}
impl CLKO1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKO1_SEL_A> {
        match self.bits {
            0 => Some(CLKO1_SEL_A::CLKO1_SEL_0),
            1 => Some(CLKO1_SEL_A::CLKO1_SEL_1),
            2 => Some(CLKO1_SEL_A::CLKO1_SEL_2),
            5 => Some(CLKO1_SEL_A::CLKO1_SEL_5),
            11 => Some(CLKO1_SEL_A::CLKO1_SEL_11),
            12 => Some(CLKO1_SEL_A::CLKO1_SEL_12),
            13 => Some(CLKO1_SEL_A::CLKO1_SEL_13),
            15 => Some(CLKO1_SEL_A::CLKO1_SEL_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_0`"]
    #[inline(always)]
    pub fn is_clko1_sel_0(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_1`"]
    #[inline(always)]
    pub fn is_clko1_sel_1(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_2`"]
    #[inline(always)]
    pub fn is_clko1_sel_2(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_2
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_5`"]
    #[inline(always)]
    pub fn is_clko1_sel_5(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_11`"]
    #[inline(always)]
    pub fn is_clko1_sel_11(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_11
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_12`"]
    #[inline(always)]
    pub fn is_clko1_sel_12(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_12
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_13`"]
    #[inline(always)]
    pub fn is_clko1_sel_13(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_13
    }
    #[doc = "Checks if the value of the field is `CLKO1_SEL_15`"]
    #[inline(always)]
    pub fn is_clko1_sel_15(&self) -> bool {
        *self == CLKO1_SEL_A::CLKO1_SEL_15
    }
}
#[doc = "Field `CLKO1_SEL` writer - Selection of the clock to be generated on CCM_CLKO1"]
pub type CLKO1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCOSR_SPEC, u8, CLKO1_SEL_A, 4, O>;
impl<'a, const O: u8> CLKO1_SEL_W<'a, O> {
    #[doc = "pll3_sw_clk (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_0(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_0)
    }
    #[doc = "PLL2 (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_1(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_1)
    }
    #[doc = "ENET PLL (divided by 2)"]
    #[inline(always)]
    pub fn clko1_sel_2(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_2)
    }
    #[doc = "semc_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_5(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_5)
    }
    #[doc = "ahb_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_11(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_11)
    }
    #[doc = "ipg_clk_root"]
    #[inline(always)]
    pub fn clko1_sel_12(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_12)
    }
    #[doc = "perclk_root"]
    #[inline(always)]
    pub fn clko1_sel_13(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_13)
    }
    #[doc = "pll4_main_clk"]
    #[inline(always)]
    pub fn clko1_sel_15(self) -> &'a mut W {
        self.variant(CLKO1_SEL_A::CLKO1_SEL_15)
    }
}
#[doc = "Field `CLKO1_DIV` reader - Setting the divider of CCM_CLKO1"]
pub type CLKO1_DIV_R = crate::FieldReader<u8, CLKO1_DIV_A>;
#[doc = "Setting the divider of CCM_CLKO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKO1_DIV_A {
    #[doc = "0: divide by 1"]
    CLKO1_DIV_0 = 0,
    #[doc = "1: divide by 2"]
    CLKO1_DIV_1 = 1,
    #[doc = "2: divide by 3"]
    CLKO1_DIV_2 = 2,
    #[doc = "3: divide by 4"]
    CLKO1_DIV_3 = 3,
    #[doc = "4: divide by 5"]
    CLKO1_DIV_4 = 4,
    #[doc = "5: divide by 6"]
    CLKO1_DIV_5 = 5,
    #[doc = "6: divide by 7"]
    CLKO1_DIV_6 = 6,
    #[doc = "7: divide by 8"]
    CLKO1_DIV_7 = 7,
}
impl From<CLKO1_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO1_DIV_A) -> Self {
        variant as _
    }
}
impl CLKO1_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO1_DIV_A {
        match self.bits {
            0 => CLKO1_DIV_A::CLKO1_DIV_0,
            1 => CLKO1_DIV_A::CLKO1_DIV_1,
            2 => CLKO1_DIV_A::CLKO1_DIV_2,
            3 => CLKO1_DIV_A::CLKO1_DIV_3,
            4 => CLKO1_DIV_A::CLKO1_DIV_4,
            5 => CLKO1_DIV_A::CLKO1_DIV_5,
            6 => CLKO1_DIV_A::CLKO1_DIV_6,
            7 => CLKO1_DIV_A::CLKO1_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_0`"]
    #[inline(always)]
    pub fn is_clko1_div_0(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_1`"]
    #[inline(always)]
    pub fn is_clko1_div_1(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_2`"]
    #[inline(always)]
    pub fn is_clko1_div_2(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_3`"]
    #[inline(always)]
    pub fn is_clko1_div_3(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_4`"]
    #[inline(always)]
    pub fn is_clko1_div_4(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_5`"]
    #[inline(always)]
    pub fn is_clko1_div_5(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_6`"]
    #[inline(always)]
    pub fn is_clko1_div_6(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO1_DIV_7`"]
    #[inline(always)]
    pub fn is_clko1_div_7(&self) -> bool {
        *self == CLKO1_DIV_A::CLKO1_DIV_7
    }
}
#[doc = "Field `CLKO1_DIV` writer - Setting the divider of CCM_CLKO1"]
pub type CLKO1_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCOSR_SPEC, u8, CLKO1_DIV_A, 3, O>;
impl<'a, const O: u8> CLKO1_DIV_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn clko1_div_0(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn clko1_div_1(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn clko1_div_2(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn clko1_div_3(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn clko1_div_4(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn clko1_div_5(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn clko1_div_6(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn clko1_div_7(self) -> &'a mut W {
        self.variant(CLKO1_DIV_A::CLKO1_DIV_7)
    }
}
#[doc = "Field `CLKO1_EN` reader - Enable of CCM_CLKO1 clock"]
pub type CLKO1_EN_R = crate::BitReader<CLKO1_EN_A>;
#[doc = "Enable of CCM_CLKO1 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKO1_EN_A {
    #[doc = "0: CCM_CLKO1 disabled."]
    CLKO1_EN_0 = 0,
    #[doc = "1: CCM_CLKO1 enabled."]
    CLKO1_EN_1 = 1,
}
impl From<CLKO1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKO1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO1_EN_A {
        match self.bits {
            false => CLKO1_EN_A::CLKO1_EN_0,
            true => CLKO1_EN_A::CLKO1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_0`"]
    #[inline(always)]
    pub fn is_clko1_en_0(&self) -> bool {
        *self == CLKO1_EN_A::CLKO1_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO1_EN_1`"]
    #[inline(always)]
    pub fn is_clko1_en_1(&self) -> bool {
        *self == CLKO1_EN_A::CLKO1_EN_1
    }
}
#[doc = "Field `CLKO1_EN` writer - Enable of CCM_CLKO1 clock"]
pub type CLKO1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCOSR_SPEC, CLKO1_EN_A, O>;
impl<'a, const O: u8> CLKO1_EN_W<'a, O> {
    #[doc = "CCM_CLKO1 disabled."]
    #[inline(always)]
    pub fn clko1_en_0(self) -> &'a mut W {
        self.variant(CLKO1_EN_A::CLKO1_EN_0)
    }
    #[doc = "CCM_CLKO1 enabled."]
    #[inline(always)]
    pub fn clko1_en_1(self) -> &'a mut W {
        self.variant(CLKO1_EN_A::CLKO1_EN_1)
    }
}
#[doc = "Field `CLK_OUT_SEL` reader - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
pub type CLK_OUT_SEL_R = crate::BitReader<CLK_OUT_SEL_A>;
#[doc = "CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_OUT_SEL_A {
    #[doc = "0: CCM_CLKO1 output drives CCM_CLKO1 clock"]
    CLK_OUT_SEL_0 = 0,
    #[doc = "1: CCM_CLKO1 output drives CCM_CLKO2 clock"]
    CLK_OUT_SEL_1 = 1,
}
impl From<CLK_OUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_OUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_OUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_OUT_SEL_A {
        match self.bits {
            false => CLK_OUT_SEL_A::CLK_OUT_SEL_0,
            true => CLK_OUT_SEL_A::CLK_OUT_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_0`"]
    #[inline(always)]
    pub fn is_clk_out_sel_0(&self) -> bool {
        *self == CLK_OUT_SEL_A::CLK_OUT_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_OUT_SEL_1`"]
    #[inline(always)]
    pub fn is_clk_out_sel_1(&self) -> bool {
        *self == CLK_OUT_SEL_A::CLK_OUT_SEL_1
    }
}
#[doc = "Field `CLK_OUT_SEL` writer - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
pub type CLK_OUT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCOSR_SPEC, CLK_OUT_SEL_A, O>;
impl<'a, const O: u8> CLK_OUT_SEL_W<'a, O> {
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock"]
    #[inline(always)]
    pub fn clk_out_sel_0(self) -> &'a mut W {
        self.variant(CLK_OUT_SEL_A::CLK_OUT_SEL_0)
    }
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock"]
    #[inline(always)]
    pub fn clk_out_sel_1(self) -> &'a mut W {
        self.variant(CLK_OUT_SEL_A::CLK_OUT_SEL_1)
    }
}
#[doc = "Field `CLKO2_SEL` reader - Selection of the clock to be generated on CCM_CLKO2"]
pub type CLKO2_SEL_R = crate::FieldReader<u8, CLKO2_SEL_A>;
#[doc = "Selection of the clock to be generated on CCM_CLKO2\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKO2_SEL_A {
    #[doc = "3: usdhc1_clk_root"]
    CLKO2_SEL_3 = 3,
    #[doc = "6: lpi2c_clk_root"]
    CLKO2_SEL_6 = 6,
    #[doc = "14: osc_clk"]
    CLKO2_SEL_14 = 14,
    #[doc = "16: lpspi_clk_root"]
    CLKO2_SEL_16 = 16,
    #[doc = "17: usdhc2_clk_root"]
    CLKO2_SEL_17 = 17,
    #[doc = "18: sai1_clk_root"]
    CLKO2_SEL_18 = 18,
    #[doc = "19: sai2_clk_root"]
    CLKO2_SEL_19 = 19,
    #[doc = "20: sai3_clk_root"]
    CLKO2_SEL_20 = 20,
    #[doc = "22: trace_clk_root"]
    CLKO2_SEL_22 = 22,
    #[doc = "23: can_clk_root"]
    CLKO2_SEL_23 = 23,
    #[doc = "27: flexspi_clk_root"]
    CLKO2_SEL_27 = 27,
    #[doc = "28: uart_clk_root"]
    CLKO2_SEL_28 = 28,
    #[doc = "29: spdif0_clk_root"]
    CLKO2_SEL_29 = 29,
}
impl From<CLKO2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO2_SEL_A) -> Self {
        variant as _
    }
}
impl CLKO2_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKO2_SEL_A> {
        match self.bits {
            3 => Some(CLKO2_SEL_A::CLKO2_SEL_3),
            6 => Some(CLKO2_SEL_A::CLKO2_SEL_6),
            14 => Some(CLKO2_SEL_A::CLKO2_SEL_14),
            16 => Some(CLKO2_SEL_A::CLKO2_SEL_16),
            17 => Some(CLKO2_SEL_A::CLKO2_SEL_17),
            18 => Some(CLKO2_SEL_A::CLKO2_SEL_18),
            19 => Some(CLKO2_SEL_A::CLKO2_SEL_19),
            20 => Some(CLKO2_SEL_A::CLKO2_SEL_20),
            22 => Some(CLKO2_SEL_A::CLKO2_SEL_22),
            23 => Some(CLKO2_SEL_A::CLKO2_SEL_23),
            27 => Some(CLKO2_SEL_A::CLKO2_SEL_27),
            28 => Some(CLKO2_SEL_A::CLKO2_SEL_28),
            29 => Some(CLKO2_SEL_A::CLKO2_SEL_29),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_3`"]
    #[inline(always)]
    pub fn is_clko2_sel_3(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_6`"]
    #[inline(always)]
    pub fn is_clko2_sel_6(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_14`"]
    #[inline(always)]
    pub fn is_clko2_sel_14(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_14
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_16`"]
    #[inline(always)]
    pub fn is_clko2_sel_16(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_16
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_17`"]
    #[inline(always)]
    pub fn is_clko2_sel_17(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_17
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_18`"]
    #[inline(always)]
    pub fn is_clko2_sel_18(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_18
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_19`"]
    #[inline(always)]
    pub fn is_clko2_sel_19(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_19
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_20`"]
    #[inline(always)]
    pub fn is_clko2_sel_20(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_20
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_22`"]
    #[inline(always)]
    pub fn is_clko2_sel_22(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_22
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_23`"]
    #[inline(always)]
    pub fn is_clko2_sel_23(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_23
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_27`"]
    #[inline(always)]
    pub fn is_clko2_sel_27(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_27
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_28`"]
    #[inline(always)]
    pub fn is_clko2_sel_28(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_28
    }
    #[doc = "Checks if the value of the field is `CLKO2_SEL_29`"]
    #[inline(always)]
    pub fn is_clko2_sel_29(&self) -> bool {
        *self == CLKO2_SEL_A::CLKO2_SEL_29
    }
}
#[doc = "Field `CLKO2_SEL` writer - Selection of the clock to be generated on CCM_CLKO2"]
pub type CLKO2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCOSR_SPEC, u8, CLKO2_SEL_A, 5, O>;
impl<'a, const O: u8> CLKO2_SEL_W<'a, O> {
    #[doc = "usdhc1_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_3(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_3)
    }
    #[doc = "lpi2c_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_6(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_6)
    }
    #[doc = "osc_clk"]
    #[inline(always)]
    pub fn clko2_sel_14(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_14)
    }
    #[doc = "lpspi_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_16(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_16)
    }
    #[doc = "usdhc2_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_17(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_17)
    }
    #[doc = "sai1_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_18(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_18)
    }
    #[doc = "sai2_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_19(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_19)
    }
    #[doc = "sai3_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_20(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_20)
    }
    #[doc = "trace_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_22(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_22)
    }
    #[doc = "can_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_23(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_23)
    }
    #[doc = "flexspi_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_27(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_27)
    }
    #[doc = "uart_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_28(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_28)
    }
    #[doc = "spdif0_clk_root"]
    #[inline(always)]
    pub fn clko2_sel_29(self) -> &'a mut W {
        self.variant(CLKO2_SEL_A::CLKO2_SEL_29)
    }
}
#[doc = "Field `CLKO2_DIV` reader - Setting the divider of CCM_CLKO2"]
pub type CLKO2_DIV_R = crate::FieldReader<u8, CLKO2_DIV_A>;
#[doc = "Setting the divider of CCM_CLKO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKO2_DIV_A {
    #[doc = "0: divide by 1"]
    CLKO2_DIV_0 = 0,
    #[doc = "1: divide by 2"]
    CLKO2_DIV_1 = 1,
    #[doc = "2: divide by 3"]
    CLKO2_DIV_2 = 2,
    #[doc = "3: divide by 4"]
    CLKO2_DIV_3 = 3,
    #[doc = "4: divide by 5"]
    CLKO2_DIV_4 = 4,
    #[doc = "5: divide by 6"]
    CLKO2_DIV_5 = 5,
    #[doc = "6: divide by 7"]
    CLKO2_DIV_6 = 6,
    #[doc = "7: divide by 8"]
    CLKO2_DIV_7 = 7,
}
impl From<CLKO2_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKO2_DIV_A) -> Self {
        variant as _
    }
}
impl CLKO2_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO2_DIV_A {
        match self.bits {
            0 => CLKO2_DIV_A::CLKO2_DIV_0,
            1 => CLKO2_DIV_A::CLKO2_DIV_1,
            2 => CLKO2_DIV_A::CLKO2_DIV_2,
            3 => CLKO2_DIV_A::CLKO2_DIV_3,
            4 => CLKO2_DIV_A::CLKO2_DIV_4,
            5 => CLKO2_DIV_A::CLKO2_DIV_5,
            6 => CLKO2_DIV_A::CLKO2_DIV_6,
            7 => CLKO2_DIV_A::CLKO2_DIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_0`"]
    #[inline(always)]
    pub fn is_clko2_div_0(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_1`"]
    #[inline(always)]
    pub fn is_clko2_div_1(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_1
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_2`"]
    #[inline(always)]
    pub fn is_clko2_div_2(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_2
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_3`"]
    #[inline(always)]
    pub fn is_clko2_div_3(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_3
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_4`"]
    #[inline(always)]
    pub fn is_clko2_div_4(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_4
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_5`"]
    #[inline(always)]
    pub fn is_clko2_div_5(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_5
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_6`"]
    #[inline(always)]
    pub fn is_clko2_div_6(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_6
    }
    #[doc = "Checks if the value of the field is `CLKO2_DIV_7`"]
    #[inline(always)]
    pub fn is_clko2_div_7(&self) -> bool {
        *self == CLKO2_DIV_A::CLKO2_DIV_7
    }
}
#[doc = "Field `CLKO2_DIV` writer - Setting the divider of CCM_CLKO2"]
pub type CLKO2_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCOSR_SPEC, u8, CLKO2_DIV_A, 3, O>;
impl<'a, const O: u8> CLKO2_DIV_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn clko2_div_0(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn clko2_div_1(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn clko2_div_2(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn clko2_div_3(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn clko2_div_4(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn clko2_div_5(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn clko2_div_6(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn clko2_div_7(self) -> &'a mut W {
        self.variant(CLKO2_DIV_A::CLKO2_DIV_7)
    }
}
#[doc = "Field `CLKO2_EN` reader - Enable of CCM_CLKO2 clock"]
pub type CLKO2_EN_R = crate::BitReader<CLKO2_EN_A>;
#[doc = "Enable of CCM_CLKO2 clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKO2_EN_A {
    #[doc = "0: CCM_CLKO2 disabled."]
    CLKO2_EN_0 = 0,
    #[doc = "1: CCM_CLKO2 enabled."]
    CLKO2_EN_1 = 1,
}
impl From<CLKO2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKO2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO2_EN_A {
        match self.bits {
            false => CLKO2_EN_A::CLKO2_EN_0,
            true => CLKO2_EN_A::CLKO2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_0`"]
    #[inline(always)]
    pub fn is_clko2_en_0(&self) -> bool {
        *self == CLKO2_EN_A::CLKO2_EN_0
    }
    #[doc = "Checks if the value of the field is `CLKO2_EN_1`"]
    #[inline(always)]
    pub fn is_clko2_en_1(&self) -> bool {
        *self == CLKO2_EN_A::CLKO2_EN_1
    }
}
#[doc = "Field `CLKO2_EN` writer - Enable of CCM_CLKO2 clock"]
pub type CLKO2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCOSR_SPEC, CLKO2_EN_A, O>;
impl<'a, const O: u8> CLKO2_EN_W<'a, O> {
    #[doc = "CCM_CLKO2 disabled."]
    #[inline(always)]
    pub fn clko2_en_0(self) -> &'a mut W {
        self.variant(CLKO2_EN_A::CLKO2_EN_0)
    }
    #[doc = "CCM_CLKO2 enabled."]
    #[inline(always)]
    pub fn clko2_en_1(self) -> &'a mut W {
        self.variant(CLKO2_EN_A::CLKO2_EN_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_sel(&self) -> CLKO1_SEL_R {
        CLKO1_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    pub fn clko1_div(&self) -> CLKO1_DIV_R {
        CLKO1_DIV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    pub fn clko1_en(&self) -> CLKO1_EN_R {
        CLKO1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    pub fn clk_out_sel(&self) -> CLK_OUT_SEL_R {
        CLK_OUT_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_sel(&self) -> CLKO2_SEL_R {
        CLKO2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    pub fn clko2_div(&self) -> CLKO2_DIV_R {
        CLKO2_DIV_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    pub fn clko2_en(&self) -> CLKO2_EN_R {
        CLKO2_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of the clock to be generated on CCM_CLKO1"]
    #[inline(always)]
    #[must_use]
    pub fn clko1_sel(&mut self) -> CLKO1_SEL_W<0> {
        CLKO1_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Setting the divider of CCM_CLKO1"]
    #[inline(always)]
    #[must_use]
    pub fn clko1_div(&mut self) -> CLKO1_DIV_W<4> {
        CLKO1_DIV_W::new(self)
    }
    #[doc = "Bit 7 - Enable of CCM_CLKO1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn clko1_en(&mut self) -> CLKO1_EN_W<7> {
        CLKO1_EN_W::new(self)
    }
    #[doc = "Bit 8 - CCM_CLKO1 output to reflect CCM_CLKO1 or CCM_CLKO2 clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clk_out_sel(&mut self) -> CLK_OUT_SEL_W<8> {
        CLK_OUT_SEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Selection of the clock to be generated on CCM_CLKO2"]
    #[inline(always)]
    #[must_use]
    pub fn clko2_sel(&mut self) -> CLKO2_SEL_W<16> {
        CLKO2_SEL_W::new(self)
    }
    #[doc = "Bits 21:23 - Setting the divider of CCM_CLKO2"]
    #[inline(always)]
    #[must_use]
    pub fn clko2_div(&mut self) -> CLKO2_DIV_W<21> {
        CLKO2_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Enable of CCM_CLKO2 clock"]
    #[inline(always)]
    #[must_use]
    pub fn clko2_en(&mut self) -> CLKO2_EN_W<24> {
        CLKO2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Clock Output Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccosr](index.html) module"]
pub struct CCOSR_SPEC;
impl crate::RegisterSpec for CCOSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccosr::R](R) reader structure"]
impl crate::Readable for CCOSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccosr::W](W) writer structure"]
impl crate::Writable for CCOSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCOSR to value 0x000a_0001"]
impl crate::Resettable for CCOSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0001;
}
