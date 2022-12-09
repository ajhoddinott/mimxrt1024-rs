#[doc = "Register `CDHIPR` reader"]
pub struct R(crate::R<CDHIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDHIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDHIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDHIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEMC_PODF_BUSY` reader - Busy indicator for semc_podf."]
pub type SEMC_PODF_BUSY_R = crate::BitReader<SEMC_PODF_BUSY_A>;
#[doc = "Busy indicator for semc_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEMC_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    SEMC_PODF_BUSY_0 = 0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied."]
    SEMC_PODF_BUSY_1 = 1,
}
impl From<SEMC_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_PODF_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl SEMC_PODF_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_BUSY_A {
        match self.bits {
            false => SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_0,
            true => SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_semc_podf_busy_0(&self) -> bool {
        *self == SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_semc_podf_busy_1(&self) -> bool {
        *self == SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_1
    }
}
#[doc = "Field `AHB_PODF_BUSY` reader - Busy indicator for ahb_podf."]
pub type AHB_PODF_BUSY_R = crate::BitReader<AHB_PODF_BUSY_A>;
#[doc = "Busy indicator for ahb_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    AHB_PODF_BUSY_0 = 0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
    AHB_PODF_BUSY_1 = 1,
}
impl From<AHB_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_PODF_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_PODF_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_BUSY_A {
        match self.bits {
            false => AHB_PODF_BUSY_A::AHB_PODF_BUSY_0,
            true => AHB_PODF_BUSY_A::AHB_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_busy_0(&self) -> bool {
        *self == AHB_PODF_BUSY_A::AHB_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_busy_1(&self) -> bool {
        *self == AHB_PODF_BUSY_A::AHB_PODF_BUSY_1
    }
}
#[doc = "Field `PERIPH2_CLK_SEL_BUSY` reader - Busy indicator for periph2_clk_sel mux control."]
pub type PERIPH2_CLK_SEL_BUSY_R = crate::BitReader<PERIPH2_CLK_SEL_BUSY_A>;
#[doc = "Busy indicator for periph2_clk_sel mux control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPH2_CLK_SEL_BUSY_A {
    #[doc = "0: mux is not busy and its value represents the actual division."]
    PERIPH2_CLK_SEL_BUSY_0 = 0,
    #[doc = "1: mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied."]
    PERIPH2_CLK_SEL_BUSY_1 = 1,
}
impl From<PERIPH2_CLK_SEL_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH2_CLK_SEL_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPH2_CLK_SEL_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH2_CLK_SEL_BUSY_A {
        match self.bits {
            false => PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_0,
            true => PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_0`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_1`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_1
    }
}
#[doc = "Field `PERIPH_CLK_SEL_BUSY` reader - Busy indicator for periph_clk_sel mux control."]
pub type PERIPH_CLK_SEL_BUSY_R = crate::BitReader<PERIPH_CLK_SEL_BUSY_A>;
#[doc = "Busy indicator for periph_clk_sel mux control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PERIPH_CLK_SEL_BUSY_A {
    #[doc = "0: mux is not busy and its value represents the actual division."]
    PERIPH_CLK_SEL_BUSY_0 = 0,
    #[doc = "1: mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
    PERIPH_CLK_SEL_BUSY_1 = 1,
}
impl From<PERIPH_CLK_SEL_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl PERIPH_CLK_SEL_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_BUSY_A {
        match self.bits {
            false => PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_0,
            true => PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_1
    }
}
#[doc = "Field `ARM_PODF_BUSY` reader - Busy indicator for arm_podf."]
pub type ARM_PODF_BUSY_R = crate::BitReader<ARM_PODF_BUSY_A>;
#[doc = "Busy indicator for arm_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    ARM_PODF_BUSY_0 = 0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied."]
    ARM_PODF_BUSY_1 = 1,
}
impl From<ARM_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_PODF_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_PODF_BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_BUSY_A {
        match self.bits {
            false => ARM_PODF_BUSY_A::ARM_PODF_BUSY_0,
            true => ARM_PODF_BUSY_A::ARM_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_arm_podf_busy_0(&self) -> bool {
        *self == ARM_PODF_BUSY_A::ARM_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_arm_podf_busy_1(&self) -> bool {
        *self == ARM_PODF_BUSY_A::ARM_PODF_BUSY_1
    }
}
impl R {
    #[doc = "Bit 0 - Busy indicator for semc_podf."]
    #[inline(always)]
    pub fn semc_podf_busy(&self) -> SEMC_PODF_BUSY_R {
        SEMC_PODF_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy indicator for ahb_podf."]
    #[inline(always)]
    pub fn ahb_podf_busy(&self) -> AHB_PODF_BUSY_R {
        AHB_PODF_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy indicator for periph2_clk_sel mux control."]
    #[inline(always)]
    pub fn periph2_clk_sel_busy(&self) -> PERIPH2_CLK_SEL_BUSY_R {
        PERIPH2_CLK_SEL_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy indicator for periph_clk_sel mux control."]
    #[inline(always)]
    pub fn periph_clk_sel_busy(&self) -> PERIPH_CLK_SEL_BUSY_R {
        PERIPH_CLK_SEL_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy indicator for arm_podf."]
    #[inline(always)]
    pub fn arm_podf_busy(&self) -> ARM_PODF_BUSY_R {
        ARM_PODF_BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "CCM Divider Handshake In-Process Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdhipr](index.html) module"]
pub struct CDHIPR_SPEC;
impl crate::RegisterSpec for CDHIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdhipr::R](R) reader structure"]
impl crate::Readable for CDHIPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDHIPR to value 0"]
impl crate::Resettable for CDHIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
