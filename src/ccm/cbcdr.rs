#[doc = "Register `CBCDR` reader"]
pub struct R(crate::R<CBCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBCDR` writer"]
pub struct W(crate::W<CBCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBCDR_SPEC>;
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
impl From<crate::W<CBCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEMC_CLK_SEL` reader - SEMC clock source select"]
pub type SEMC_CLK_SEL_R = crate::BitReader<SEMC_CLK_SEL_A>;
#[doc = "SEMC clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_CLK_SEL_A {
    #[doc = "0: Periph_clk output will be used as SEMC clock root"]
    SEMC_CLK_SEL_0 = 0,
    #[doc = "1: SEMC alternative clock will be used as SEMC clock root"]
    SEMC_CLK_SEL_1 = 1,
}
impl From<SEMC_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_CLK_SEL_A {
        match self.bits {
            false => SEMC_CLK_SEL_A::SEMC_CLK_SEL_0,
            true => SEMC_CLK_SEL_A::SEMC_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_semc_clk_sel_0(&self) -> bool {
        *self == SEMC_CLK_SEL_A::SEMC_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_semc_clk_sel_1(&self) -> bool {
        *self == SEMC_CLK_SEL_A::SEMC_CLK_SEL_1
    }
}
#[doc = "Field `SEMC_CLK_SEL` writer - SEMC clock source select"]
pub type SEMC_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CBCDR_SPEC, SEMC_CLK_SEL_A, O>;
impl<'a, const O: u8> SEMC_CLK_SEL_W<'a, O> {
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    #[inline(always)]
    pub fn semc_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_CLK_SEL_A::SEMC_CLK_SEL_0)
    }
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    #[inline(always)]
    pub fn semc_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_CLK_SEL_A::SEMC_CLK_SEL_1)
    }
}
#[doc = "Field `SEMC_ALT_CLK_SEL` reader - SEMC alternative clock select"]
pub type SEMC_ALT_CLK_SEL_R = crate::BitReader<SEMC_ALT_CLK_SEL_A>;
#[doc = "SEMC alternative clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_ALT_CLK_SEL_A {
    #[doc = "0: PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_0 = 0,
    #[doc = "1: PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_1 = 1,
}
impl From<SEMC_ALT_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_ALT_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_ALT_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_ALT_CLK_SEL_A {
        match self.bits {
            false => SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0,
            true => SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_semc_alt_clk_sel_0(&self) -> bool {
        *self == SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_semc_alt_clk_sel_1(&self) -> bool {
        *self == SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1
    }
}
#[doc = "Field `SEMC_ALT_CLK_SEL` writer - SEMC alternative clock select"]
pub type SEMC_ALT_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CBCDR_SPEC, SEMC_ALT_CLK_SEL_A, O>;
impl<'a, const O: u8> SEMC_ALT_CLK_SEL_W<'a, O> {
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    #[inline(always)]
    pub fn semc_alt_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0)
    }
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    #[inline(always)]
    pub fn semc_alt_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1)
    }
}
#[doc = "Field `IPG_PODF` reader - Divider for ipg podf."]
pub type IPG_PODF_R = crate::FieldReader<u8, IPG_PODF_A>;
#[doc = "Divider for ipg podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPG_PODF_A {
    #[doc = "0: divide by 1"]
    IPG_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    IPG_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    IPG_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    IPG_PODF_3 = 3,
}
impl From<IPG_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: IPG_PODF_A) -> Self {
        variant as _
    }
}
impl IPG_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPG_PODF_A {
        match self.bits {
            0 => IPG_PODF_A::IPG_PODF_0,
            1 => IPG_PODF_A::IPG_PODF_1,
            2 => IPG_PODF_A::IPG_PODF_2,
            3 => IPG_PODF_A::IPG_PODF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_0`"]
    #[inline(always)]
    pub fn is_ipg_podf_0(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_0
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_1`"]
    #[inline(always)]
    pub fn is_ipg_podf_1(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_1
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_2`"]
    #[inline(always)]
    pub fn is_ipg_podf_2(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_2
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_3`"]
    #[inline(always)]
    pub fn is_ipg_podf_3(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_3
    }
}
#[doc = "Field `IPG_PODF` writer - Divider for ipg podf."]
pub type IPG_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCDR_SPEC, u8, IPG_PODF_A, 2, O>;
impl<'a, const O: u8> IPG_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn ipg_podf_0(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn ipg_podf_1(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn ipg_podf_2(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn ipg_podf_3(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_3)
    }
}
#[doc = "Field `AHB_PODF` reader - Divider for AHB PODF"]
pub type AHB_PODF_R = crate::FieldReader<u8, AHB_PODF_A>;
#[doc = "Divider for AHB PODF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHB_PODF_A {
    #[doc = "0: divide by 1"]
    AHB_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    AHB_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    AHB_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    AHB_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    AHB_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    AHB_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    AHB_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    AHB_PODF_7 = 7,
}
impl From<AHB_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_PODF_A) -> Self {
        variant as _
    }
}
impl AHB_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_A {
        match self.bits {
            0 => AHB_PODF_A::AHB_PODF_0,
            1 => AHB_PODF_A::AHB_PODF_1,
            2 => AHB_PODF_A::AHB_PODF_2,
            3 => AHB_PODF_A::AHB_PODF_3,
            4 => AHB_PODF_A::AHB_PODF_4,
            5 => AHB_PODF_A::AHB_PODF_5,
            6 => AHB_PODF_A::AHB_PODF_6,
            7 => AHB_PODF_A::AHB_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_0(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_1(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_1
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_2`"]
    #[inline(always)]
    pub fn is_ahb_podf_2(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_2
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_3`"]
    #[inline(always)]
    pub fn is_ahb_podf_3(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_3
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_4`"]
    #[inline(always)]
    pub fn is_ahb_podf_4(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_4
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_5`"]
    #[inline(always)]
    pub fn is_ahb_podf_5(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_5
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_6`"]
    #[inline(always)]
    pub fn is_ahb_podf_6(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_6
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_7`"]
    #[inline(always)]
    pub fn is_ahb_podf_7(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_7
    }
}
#[doc = "Field `AHB_PODF` writer - Divider for AHB PODF"]
pub type AHB_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCDR_SPEC, u8, AHB_PODF_A, 3, O>;
impl<'a, const O: u8> AHB_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn ahb_podf_0(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn ahb_podf_1(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn ahb_podf_2(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn ahb_podf_3(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn ahb_podf_4(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn ahb_podf_5(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn ahb_podf_6(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn ahb_podf_7(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_7)
    }
}
#[doc = "Field `SEMC_PODF` reader - Post divider for SEMC clock"]
pub type SEMC_PODF_R = crate::FieldReader<u8, SEMC_PODF_A>;
#[doc = "Post divider for SEMC clock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEMC_PODF_A {
    #[doc = "0: divide by 1"]
    SEMC_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    SEMC_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    SEMC_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    SEMC_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    SEMC_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    SEMC_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    SEMC_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    SEMC_PODF_7 = 7,
}
impl From<SEMC_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: SEMC_PODF_A) -> Self {
        variant as _
    }
}
impl SEMC_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_A {
        match self.bits {
            0 => SEMC_PODF_A::SEMC_PODF_0,
            1 => SEMC_PODF_A::SEMC_PODF_1,
            2 => SEMC_PODF_A::SEMC_PODF_2,
            3 => SEMC_PODF_A::SEMC_PODF_3,
            4 => SEMC_PODF_A::SEMC_PODF_4,
            5 => SEMC_PODF_A::SEMC_PODF_5,
            6 => SEMC_PODF_A::SEMC_PODF_6,
            7 => SEMC_PODF_A::SEMC_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_0`"]
    #[inline(always)]
    pub fn is_semc_podf_0(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_1`"]
    #[inline(always)]
    pub fn is_semc_podf_1(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_1
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_2`"]
    #[inline(always)]
    pub fn is_semc_podf_2(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_2
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_3`"]
    #[inline(always)]
    pub fn is_semc_podf_3(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_3
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_4`"]
    #[inline(always)]
    pub fn is_semc_podf_4(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_4
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_5`"]
    #[inline(always)]
    pub fn is_semc_podf_5(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_5
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_6`"]
    #[inline(always)]
    pub fn is_semc_podf_6(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_6
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_7`"]
    #[inline(always)]
    pub fn is_semc_podf_7(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_7
    }
}
#[doc = "Field `SEMC_PODF` writer - Post divider for SEMC clock"]
pub type SEMC_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCDR_SPEC, u8, SEMC_PODF_A, 3, O>;
impl<'a, const O: u8> SEMC_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn semc_podf_0(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn semc_podf_1(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn semc_podf_2(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn semc_podf_3(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn semc_podf_4(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn semc_podf_5(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn semc_podf_6(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn semc_podf_7(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_7)
    }
}
#[doc = "Field `PERIPH_CLK_SEL` reader - Selector for peripheral main clock"]
pub type PERIPH_CLK_SEL_R = crate::BitReader<PERIPH_CLK_SEL_A>;
#[doc = "Selector for peripheral main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPH_CLK_SEL_A {
    #[doc = "0: derive clock from pre_periph_clk_sel"]
    PERIPH_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from periph_clk2_clk_divided"]
    PERIPH_CLK_SEL_1 = 1,
}
impl From<PERIPH_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPH_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_A {
        match self.bits {
            false => PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0,
            true => PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1
    }
}
#[doc = "Field `PERIPH_CLK_SEL` writer - Selector for peripheral main clock"]
pub type PERIPH_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CBCDR_SPEC, PERIPH_CLK_SEL_A, O>;
impl<'a, const O: u8> PERIPH_CLK_SEL_W<'a, O> {
    #[doc = "derive clock from pre_periph_clk_sel"]
    #[inline(always)]
    pub fn periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from periph_clk2_clk_divided"]
    #[inline(always)]
    pub fn periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1)
    }
}
#[doc = "Field `PERIPH_CLK2_PODF` reader - Divider for periph_clk2_podf."]
pub type PERIPH_CLK2_PODF_R = crate::FieldReader<u8, PERIPH_CLK2_PODF_A>;
#[doc = "Divider for periph_clk2_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIPH_CLK2_PODF_A {
    #[doc = "0: divide by 1"]
    PERIPH_CLK2_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    PERIPH_CLK2_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    PERIPH_CLK2_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    PERIPH_CLK2_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    PERIPH_CLK2_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    PERIPH_CLK2_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    PERIPH_CLK2_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    PERIPH_CLK2_PODF_7 = 7,
}
impl From<PERIPH_CLK2_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPH_CLK2_PODF_A) -> Self {
        variant as _
    }
}
impl PERIPH_CLK2_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK2_PODF_A {
        match self.bits {
            0 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0,
            1 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1,
            2 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2,
            3 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3,
            4 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4,
            5 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5,
            6 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6,
            7 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_0`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_0(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_1`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_1(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_2`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_2(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_3`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_3(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_4`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_4(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_5`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_5(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_6`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_6(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_7`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_7(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7
    }
}
#[doc = "Field `PERIPH_CLK2_PODF` writer - Divider for periph_clk2_podf."]
pub type PERIPH_CLK2_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CBCDR_SPEC, u8, PERIPH_CLK2_PODF_A, 3, O>;
impl<'a, const O: u8> PERIPH_CLK2_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn periph_clk2_podf_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn periph_clk2_podf_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn periph_clk2_podf_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn periph_clk2_podf_3(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn periph_clk2_podf_4(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn periph_clk2_podf_5(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn periph_clk2_podf_6(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn periph_clk2_podf_7(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7)
    }
}
impl R {
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline(always)]
    pub fn semc_clk_sel(&self) -> SEMC_CLK_SEL_R {
        SEMC_CLK_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline(always)]
    pub fn semc_alt_clk_sel(&self) -> SEMC_ALT_CLK_SEL_R {
        SEMC_ALT_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Divider for ipg podf."]
    #[inline(always)]
    pub fn ipg_podf(&self) -> IPG_PODF_R {
        IPG_PODF_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline(always)]
    pub fn ahb_podf(&self) -> AHB_PODF_R {
        AHB_PODF_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline(always)]
    pub fn semc_podf(&self) -> SEMC_PODF_R {
        SEMC_PODF_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline(always)]
    pub fn periph_clk_sel(&self) -> PERIPH_CLK_SEL_R {
        PERIPH_CLK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline(always)]
    pub fn periph_clk2_podf(&self) -> PERIPH_CLK2_PODF_R {
        PERIPH_CLK2_PODF_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn semc_clk_sel(&mut self) -> SEMC_CLK_SEL_W<6> {
        SEMC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline(always)]
    #[must_use]
    pub fn semc_alt_clk_sel(&mut self) -> SEMC_ALT_CLK_SEL_W<7> {
        SEMC_ALT_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Divider for ipg podf."]
    #[inline(always)]
    #[must_use]
    pub fn ipg_podf(&mut self) -> IPG_PODF_W<8> {
        IPG_PODF_W::new(self)
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_podf(&mut self) -> AHB_PODF_W<10> {
        AHB_PODF_W::new(self)
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline(always)]
    #[must_use]
    pub fn semc_podf(&mut self) -> SEMC_PODF_W<16> {
        SEMC_PODF_W::new(self)
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline(always)]
    #[must_use]
    pub fn periph_clk_sel(&mut self) -> PERIPH_CLK_SEL_W<25> {
        PERIPH_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline(always)]
    #[must_use]
    pub fn periph_clk2_podf(&mut self) -> PERIPH_CLK2_PODF_W<27> {
        PERIPH_CLK2_PODF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Bus Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbcdr](index.html) module"]
pub struct CBCDR_SPEC;
impl crate::RegisterSpec for CBCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbcdr::R](R) reader structure"]
impl crate::Readable for CBCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbcdr::W](W) writer structure"]
impl crate::Writable for CBCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBCDR to value 0x000a_8000"]
impl crate::Resettable for CBCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_8000;
}
