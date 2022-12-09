#[doc = "Register `CBCMR` reader"]
pub struct R(crate::R<CBCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCMR` writer"]
pub struct W(crate::W<CBCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCMR_SPEC>;
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
impl From<crate::W<CBCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPSPI_CLK_SEL` reader - Selector for lpspi clock multiplexer"]
pub type LPSPI_CLK_SEL_R = crate::FieldReader<u8, LPSPI_CLK_SEL_A>;
#[doc = "Selector for lpspi clock multiplexer\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPSPI_CLK_SEL_A {
    #[doc = "0: derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2"]
    LPSPI_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3 = 3,
}
impl From<LPSPI_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSPI_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl LPSPI_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI_CLK_SEL_A {
        match self.bits {
            0 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0,
            1 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1,
            2 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2,
            3 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_0(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_1(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_2(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_3(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3
    }
}
#[doc = "Field `LPSPI_CLK_SEL` writer - Selector for lpspi clock multiplexer"]
pub type LPSPI_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCMR_SPEC, u8, LPSPI_CLK_SEL_A, 2, O>;
impl<'a, const O: u8> LPSPI_CLK_SEL_W<'a, O> {
    #[doc = "derive clock from PLL3 PFD1 clk"]
    #[inline(always)]
    pub fn lpspi_clk_sel_0(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline(always)]
    pub fn lpspi_clk_sel_1(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn lpspi_clk_sel_2(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn lpspi_clk_sel_3(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3)
    }
}
#[doc = "Field `PERIPH_CLK2_SEL` reader - Selector for peripheral clk2 clock multiplexer"]
pub type PERIPH_CLK2_SEL_R = crate::FieldReader<u8, PERIPH_CLK2_SEL_A>;
#[doc = "Selector for peripheral clk2 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIPH_CLK2_SEL_A {
    #[doc = "0: derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0 = 0,
    #[doc = "1: derive clock from osc_clk"]
    PERIPH_CLK2_SEL_1 = 1,
    #[doc = "2: derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2 = 2,
}
impl From<PERIPH_CLK2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPH_CLK2_SEL_A) -> Self {
        variant as _
    }
}
impl PERIPH_CLK2_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERIPH_CLK2_SEL_A> {
        match self.bits {
            0 => Some(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0),
            1 => Some(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1),
            2 => Some(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_0`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_0(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_1`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_1(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_2`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_2(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2
    }
}
#[doc = "Field `PERIPH_CLK2_SEL` writer - Selector for peripheral clk2 clock multiplexer"]
pub type PERIPH_CLK2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CBCMR_SPEC, u8, PERIPH_CLK2_SEL_A, 2, O>;
impl<'a, const O: u8> PERIPH_CLK2_SEL_W<'a, O> {
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn periph_clk2_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline(always)]
    pub fn periph_clk2_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1)
    }
    #[doc = "derive clock from pll2_bypass_clk"]
    #[inline(always)]
    pub fn periph_clk2_sel_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2)
    }
}
#[doc = "Field `TRACE_CLK_SEL` reader - Selector for Trace clock multiplexer"]
pub type TRACE_CLK_SEL_R = crate::FieldReader<u8, TRACE_CLK_SEL_A>;
#[doc = "Selector for Trace clock multiplexer\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACE_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2"]
    TRACE_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3 = 3,
}
impl From<TRACE_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl TRACE_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACE_CLK_SEL_A {
        match self.bits {
            0 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_0,
            1 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_1,
            2 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_2,
            3 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_0(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_1(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_2(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_3(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_3
    }
}
#[doc = "Field `TRACE_CLK_SEL` writer - Selector for Trace clock multiplexer"]
pub type TRACE_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCMR_SPEC, u8, TRACE_CLK_SEL_A, 2, O>;
impl<'a, const O: u8> TRACE_CLK_SEL_W<'a, O> {
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn trace_clk_sel_0(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn trace_clk_sel_1(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline(always)]
    pub fn trace_clk_sel_2(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD1"]
    #[inline(always)]
    pub fn trace_clk_sel_3(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_3)
    }
}
#[doc = "Field `PRE_PERIPH_CLK_SEL` reader - Selector for pre_periph clock multiplexer"]
pub type PRE_PERIPH_CLK_SEL_R = crate::FieldReader<u8, PRE_PERIPH_CLK_SEL_A>;
#[doc = "Selector for pre_periph clock multiplexer\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRE_PERIPH_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL3 PFD3"]
    PRE_PERIPH_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2 PFD3"]
    PRE_PERIPH_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from divided PLL6"]
    PRE_PERIPH_CLK_SEL_3 = 3,
}
impl From<PRE_PERIPH_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_PERIPH_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl PRE_PERIPH_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_PERIPH_CLK_SEL_A {
        match self.bits {
            0 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0,
            1 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1,
            2 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2,
            3 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_0(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_1(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_2(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_3(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3
    }
}
#[doc = "Field `PRE_PERIPH_CLK_SEL` writer - Selector for pre_periph clock multiplexer"]
pub type PRE_PERIPH_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCMR_SPEC, u8, PRE_PERIPH_CLK_SEL_A, 2, O>;
impl<'a, const O: u8> PRE_PERIPH_CLK_SEL_W<'a, O> {
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD3"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD3"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_2(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2)
    }
    #[doc = "derive clock from divided PLL6"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_3(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3)
    }
}
#[doc = "Field `LPSPI_PODF` reader - Divider for LPSPI. Divider should be updated when output clock is gated."]
pub type LPSPI_PODF_R = crate::FieldReader<u8, LPSPI_PODF_A>;
#[doc = "Divider for LPSPI. Divider should be updated when output clock is gated.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPSPI_PODF_A {
    #[doc = "0: divide by 1"]
    LPSPI_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    LPSPI_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    LPSPI_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    LPSPI_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    LPSPI_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    LPSPI_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    LPSPI_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    LPSPI_PODF_7 = 7,
}
impl From<LPSPI_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSPI_PODF_A) -> Self {
        variant as _
    }
}
impl LPSPI_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI_PODF_A {
        match self.bits {
            0 => LPSPI_PODF_A::LPSPI_PODF_0,
            1 => LPSPI_PODF_A::LPSPI_PODF_1,
            2 => LPSPI_PODF_A::LPSPI_PODF_2,
            3 => LPSPI_PODF_A::LPSPI_PODF_3,
            4 => LPSPI_PODF_A::LPSPI_PODF_4,
            5 => LPSPI_PODF_A::LPSPI_PODF_5,
            6 => LPSPI_PODF_A::LPSPI_PODF_6,
            7 => LPSPI_PODF_A::LPSPI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_0`"]
    #[inline(always)]
    pub fn is_lpspi_podf_0(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_1`"]
    #[inline(always)]
    pub fn is_lpspi_podf_1(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_2`"]
    #[inline(always)]
    pub fn is_lpspi_podf_2(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_3`"]
    #[inline(always)]
    pub fn is_lpspi_podf_3(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_3
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_4`"]
    #[inline(always)]
    pub fn is_lpspi_podf_4(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_4
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_5`"]
    #[inline(always)]
    pub fn is_lpspi_podf_5(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_5
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_6`"]
    #[inline(always)]
    pub fn is_lpspi_podf_6(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_6
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_7`"]
    #[inline(always)]
    pub fn is_lpspi_podf_7(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_7
    }
}
#[doc = "Field `LPSPI_PODF` writer - Divider for LPSPI. Divider should be updated when output clock is gated."]
pub type LPSPI_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCMR_SPEC, u8, LPSPI_PODF_A, 3, O>;
impl<'a, const O: u8> LPSPI_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn lpspi_podf_0(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn lpspi_podf_1(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn lpspi_podf_2(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn lpspi_podf_3(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn lpspi_podf_4(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn lpspi_podf_5(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn lpspi_podf_6(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn lpspi_podf_7(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_7)
    }
}
impl R {
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub fn lpspi_clk_sel(&self) -> LPSPI_CLK_SEL_R {
        LPSPI_CLK_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub fn periph_clk2_sel(&self) -> PERIPH_CLK2_SEL_R {
        PERIPH_CLK2_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub fn trace_clk_sel(&self) -> TRACE_CLK_SEL_R {
        TRACE_CLK_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub fn pre_periph_clk_sel(&self) -> PRE_PERIPH_CLK_SEL_R {
        PRE_PERIPH_CLK_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn lpspi_podf(&self) -> LPSPI_PODF_R {
        LPSPI_PODF_R::new(((self.bits >> 26) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn lpspi_clk_sel(&mut self) -> LPSPI_CLK_SEL_W<4> {
        LPSPI_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn periph_clk2_sel(&mut self) -> PERIPH_CLK2_SEL_W<12> {
        PERIPH_CLK2_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn trace_clk_sel(&mut self) -> TRACE_CLK_SEL_W<14> {
        TRACE_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn pre_periph_clk_sel(&mut self) -> PRE_PERIPH_CLK_SEL_W<18> {
        PRE_PERIPH_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn lpspi_podf(&mut self) -> LPSPI_PODF_W<26> {
        LPSPI_PODF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Bus Clock Multiplexer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbcmr](index.html) module"]
pub struct CBCMR_SPEC;
impl crate::RegisterSpec for CBCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbcmr::R](R) reader structure"]
impl crate::Readable for CBCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbcmr::W](W) writer structure"]
impl crate::Writable for CBCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCMR to value 0x2daa_8324"]
impl crate::Resettable for CBCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2daa_8324;
}
