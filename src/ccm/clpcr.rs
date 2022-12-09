#[doc = "Register `CLPCR` reader"]
pub struct R(crate::R<CLPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPCR` writer"]
pub struct W(crate::W<CLPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPCR_SPEC>;
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
impl From<crate::W<CLPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPM` reader - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
pub type LPM_R = crate::FieldReader<u8, LPM_A>;
#[doc = "Setting the low power mode that system will enter on next assertion of dsm_request signal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPM_A {
    #[doc = "0: Remain in run mode"]
    LPM_0 = 0,
    #[doc = "1: Transfer to wait mode"]
    LPM_1 = 1,
    #[doc = "2: Transfer to stop mode"]
    LPM_2 = 2,
}
impl From<LPM_A> for u8 {
    #[inline(always)]
    fn from(variant: LPM_A) -> Self {
        variant as _
    }
}
impl LPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPM_A> {
        match self.bits {
            0 => Some(LPM_A::LPM_0),
            1 => Some(LPM_A::LPM_1),
            2 => Some(LPM_A::LPM_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_0`"]
    #[inline(always)]
    pub fn is_lpm_0(&self) -> bool {
        *self == LPM_A::LPM_0
    }
    #[doc = "Checks if the value of the field is `LPM_1`"]
    #[inline(always)]
    pub fn is_lpm_1(&self) -> bool {
        *self == LPM_A::LPM_1
    }
    #[doc = "Checks if the value of the field is `LPM_2`"]
    #[inline(always)]
    pub fn is_lpm_2(&self) -> bool {
        *self == LPM_A::LPM_2
    }
}
#[doc = "Field `LPM` writer - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
pub type LPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLPCR_SPEC, u8, LPM_A, 2, O>;
impl<'a, const O: u8> LPM_W<'a, O> {
    #[doc = "Remain in run mode"]
    #[inline(always)]
    pub fn lpm_0(self) -> &'a mut W {
        self.variant(LPM_A::LPM_0)
    }
    #[doc = "Transfer to wait mode"]
    #[inline(always)]
    pub fn lpm_1(self) -> &'a mut W {
        self.variant(LPM_A::LPM_1)
    }
    #[doc = "Transfer to stop mode"]
    #[inline(always)]
    pub fn lpm_2(self) -> &'a mut W {
        self.variant(LPM_A::LPM_2)
    }
}
#[doc = "Field `ARM_CLK_DIS_ON_LPM` reader - Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
pub type ARM_CLK_DIS_ON_LPM_R = crate::BitReader<ARM_CLK_DIS_ON_LPM_A>;
#[doc = "Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARM_CLK_DIS_ON_LPM_A {
    #[doc = "0: Arm clock enabled on wait mode."]
    ARM_CLK_DIS_ON_LPM_0 = 0,
    #[doc = "1: Arm clock disabled on wait mode. ."]
    ARM_CLK_DIS_ON_LPM_1 = 1,
}
impl From<ARM_CLK_DIS_ON_LPM_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_CLK_DIS_ON_LPM_A) -> Self {
        variant as u8 != 0
    }
}
impl ARM_CLK_DIS_ON_LPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_CLK_DIS_ON_LPM_A {
        match self.bits {
            false => ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0,
            true => ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_0`"]
    #[inline(always)]
    pub fn is_arm_clk_dis_on_lpm_0(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0
    }
    #[doc = "Checks if the value of the field is `ARM_CLK_DIS_ON_LPM_1`"]
    #[inline(always)]
    pub fn is_arm_clk_dis_on_lpm_1(&self) -> bool {
        *self == ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1
    }
}
#[doc = "Field `ARM_CLK_DIS_ON_LPM` writer - Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
pub type ARM_CLK_DIS_ON_LPM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLPCR_SPEC, ARM_CLK_DIS_ON_LPM_A, O>;
impl<'a, const O: u8> ARM_CLK_DIS_ON_LPM_W<'a, O> {
    #[doc = "Arm clock enabled on wait mode."]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm_0(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_0)
    }
    #[doc = "Arm clock disabled on wait mode. ."]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm_1(self) -> &'a mut W {
        self.variant(ARM_CLK_DIS_ON_LPM_A::ARM_CLK_DIS_ON_LPM_1)
    }
}
#[doc = "Field `SBYOS` reader - Standby clock oscillator bit"]
pub type SBYOS_R = crate::BitReader<SBYOS_A>;
#[doc = "Standby clock oscillator bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBYOS_A {
    #[doc = "0: On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    SBYOS_0 = 0,
    #[doc = "1: On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    SBYOS_1 = 1,
}
impl From<SBYOS_A> for bool {
    #[inline(always)]
    fn from(variant: SBYOS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBYOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBYOS_A {
        match self.bits {
            false => SBYOS_A::SBYOS_0,
            true => SBYOS_A::SBYOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBYOS_0`"]
    #[inline(always)]
    pub fn is_sbyos_0(&self) -> bool {
        *self == SBYOS_A::SBYOS_0
    }
    #[doc = "Checks if the value of the field is `SBYOS_1`"]
    #[inline(always)]
    pub fn is_sbyos_1(&self) -> bool {
        *self == SBYOS_A::SBYOS_1
    }
}
#[doc = "Field `SBYOS` writer - Standby clock oscillator bit"]
pub type SBYOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, SBYOS_A, O>;
impl<'a, const O: u8> SBYOS_W<'a, O> {
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')"]
    #[inline(always)]
    pub fn sbyos_0(self) -> &'a mut W {
        self.variant(SBYOS_A::SBYOS_0)
    }
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    #[inline(always)]
    pub fn sbyos_1(self) -> &'a mut W {
        self.variant(SBYOS_A::SBYOS_1)
    }
}
#[doc = "Field `DIS_REF_OSC` reader - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
pub type DIS_REF_OSC_R = crate::BitReader<DIS_REF_OSC_A>;
#[doc = "dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_REF_OSC_A {
    #[doc = "0: external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DIS_REF_OSC_0 = 0,
    #[doc = "1: external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    DIS_REF_OSC_1 = 1,
}
impl From<DIS_REF_OSC_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_REF_OSC_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_REF_OSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_REF_OSC_A {
        match self.bits {
            false => DIS_REF_OSC_A::DIS_REF_OSC_0,
            true => DIS_REF_OSC_A::DIS_REF_OSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_0`"]
    #[inline(always)]
    pub fn is_dis_ref_osc_0(&self) -> bool {
        *self == DIS_REF_OSC_A::DIS_REF_OSC_0
    }
    #[doc = "Checks if the value of the field is `DIS_REF_OSC_1`"]
    #[inline(always)]
    pub fn is_dis_ref_osc_1(&self) -> bool {
        *self == DIS_REF_OSC_A::DIS_REF_OSC_1
    }
}
#[doc = "Field `DIS_REF_OSC` writer - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
pub type DIS_REF_OSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, DIS_REF_OSC_A, O>;
impl<'a, const O: u8> DIS_REF_OSC_W<'a, O> {
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    #[inline(always)]
    pub fn dis_ref_osc_0(self) -> &'a mut W {
        self.variant(DIS_REF_OSC_A::DIS_REF_OSC_0)
    }
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'"]
    #[inline(always)]
    pub fn dis_ref_osc_1(self) -> &'a mut W {
        self.variant(DIS_REF_OSC_A::DIS_REF_OSC_1)
    }
}
#[doc = "Field `VSTBY` reader - Voltage standby request bit"]
pub type VSTBY_R = crate::BitReader<VSTBY_A>;
#[doc = "Voltage standby request bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSTBY_A {
    #[doc = "0: Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    VSTBY_0 = 0,
    #[doc = "1: Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    VSTBY_1 = 1,
}
impl From<VSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: VSTBY_A) -> Self {
        variant as u8 != 0
    }
}
impl VSTBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSTBY_A {
        match self.bits {
            false => VSTBY_A::VSTBY_0,
            true => VSTBY_A::VSTBY_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSTBY_0`"]
    #[inline(always)]
    pub fn is_vstby_0(&self) -> bool {
        *self == VSTBY_A::VSTBY_0
    }
    #[doc = "Checks if the value of the field is `VSTBY_1`"]
    #[inline(always)]
    pub fn is_vstby_1(&self) -> bool {
        *self == VSTBY_A::VSTBY_1
    }
}
#[doc = "Field `VSTBY` writer - Voltage standby request bit"]
pub type VSTBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, VSTBY_A, O>;
impl<'a, const O: u8> VSTBY_W<'a, O> {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')"]
    #[inline(always)]
    pub fn vstby_0(self) -> &'a mut W {
        self.variant(VSTBY_A::VSTBY_0)
    }
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    #[inline(always)]
    pub fn vstby_1(self) -> &'a mut W {
        self.variant(VSTBY_A::VSTBY_1)
    }
}
#[doc = "Field `STBY_COUNT` reader - Standby counter definition"]
pub type STBY_COUNT_R = crate::FieldReader<u8, STBY_COUNT_A>;
#[doc = "Standby counter definition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STBY_COUNT_A {
    #[doc = "0: CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_0 = 0,
    #[doc = "1: CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_1 = 1,
    #[doc = "2: CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_2 = 2,
    #[doc = "3: CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    STBY_COUNT_3 = 3,
}
impl From<STBY_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: STBY_COUNT_A) -> Self {
        variant as _
    }
}
impl STBY_COUNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBY_COUNT_A {
        match self.bits {
            0 => STBY_COUNT_A::STBY_COUNT_0,
            1 => STBY_COUNT_A::STBY_COUNT_1,
            2 => STBY_COUNT_A::STBY_COUNT_2,
            3 => STBY_COUNT_A::STBY_COUNT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_0`"]
    #[inline(always)]
    pub fn is_stby_count_0(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_0
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_1`"]
    #[inline(always)]
    pub fn is_stby_count_1(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_1
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_2`"]
    #[inline(always)]
    pub fn is_stby_count_2(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_2
    }
    #[doc = "Checks if the value of the field is `STBY_COUNT_3`"]
    #[inline(always)]
    pub fn is_stby_count_3(&self) -> bool {
        *self == STBY_COUNT_A::STBY_COUNT_3
    }
}
#[doc = "Field `STBY_COUNT` writer - Standby counter definition"]
pub type STBY_COUNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLPCR_SPEC, u8, STBY_COUNT_A, 2, O>;
impl<'a, const O: u8> STBY_COUNT_W<'a, O> {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_0(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_0)
    }
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_1(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_1)
    }
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_2(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_2)
    }
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles"]
    #[inline(always)]
    pub fn stby_count_3(self) -> &'a mut W {
        self.variant(STBY_COUNT_A::STBY_COUNT_3)
    }
}
#[doc = "Field `COSC_PWRDOWN` reader - In run mode, software can manually control powering down of on chip oscillator, i"]
pub type COSC_PWRDOWN_R = crate::BitReader<COSC_PWRDOWN_A>;
#[doc = "In run mode, software can manually control powering down of on chip oscillator, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSC_PWRDOWN_A {
    #[doc = "0: On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    COSC_PWRDOWN_0 = 0,
    #[doc = "1: On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    COSC_PWRDOWN_1 = 1,
}
impl From<COSC_PWRDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_PWRDOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl COSC_PWRDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_PWRDOWN_A {
        match self.bits {
            false => COSC_PWRDOWN_A::COSC_PWRDOWN_0,
            true => COSC_PWRDOWN_A::COSC_PWRDOWN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_0`"]
    #[inline(always)]
    pub fn is_cosc_pwrdown_0(&self) -> bool {
        *self == COSC_PWRDOWN_A::COSC_PWRDOWN_0
    }
    #[doc = "Checks if the value of the field is `COSC_PWRDOWN_1`"]
    #[inline(always)]
    pub fn is_cosc_pwrdown_1(&self) -> bool {
        *self == COSC_PWRDOWN_A::COSC_PWRDOWN_1
    }
}
#[doc = "Field `COSC_PWRDOWN` writer - In run mode, software can manually control powering down of on chip oscillator, i"]
pub type COSC_PWRDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, COSC_PWRDOWN_A, O>;
impl<'a, const O: u8> COSC_PWRDOWN_W<'a, O> {
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    #[inline(always)]
    pub fn cosc_pwrdown_0(self) -> &'a mut W {
        self.variant(COSC_PWRDOWN_A::COSC_PWRDOWN_0)
    }
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    #[inline(always)]
    pub fn cosc_pwrdown_1(self) -> &'a mut W {
        self.variant(COSC_PWRDOWN_A::COSC_PWRDOWN_1)
    }
}
#[doc = "Field `BYPASS_LPM_HS1` reader - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
pub type BYPASS_LPM_HS1_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS_LPM_HS1` writer - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
pub type BYPASS_LPM_HS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, bool, O>;
#[doc = "Field `BYPASS_LPM_HS0` reader - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
pub type BYPASS_LPM_HS0_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS_LPM_HS0` writer - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
pub type BYPASS_LPM_HS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLPCR_SPEC, bool, O>;
#[doc = "Field `MASK_CORE0_WFI` reader - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
pub type MASK_CORE0_WFI_R = crate::BitReader<MASK_CORE0_WFI_A>;
#[doc = "Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_CORE0_WFI_A {
    #[doc = "0: WFI of core0 is not masked"]
    MASK_CORE0_WFI_0 = 0,
    #[doc = "1: WFI of core0 is masked"]
    MASK_CORE0_WFI_1 = 1,
}
impl From<MASK_CORE0_WFI_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_CORE0_WFI_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_CORE0_WFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_CORE0_WFI_A {
        match self.bits {
            false => MASK_CORE0_WFI_A::MASK_CORE0_WFI_0,
            true => MASK_CORE0_WFI_A::MASK_CORE0_WFI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_0`"]
    #[inline(always)]
    pub fn is_mask_core0_wfi_0(&self) -> bool {
        *self == MASK_CORE0_WFI_A::MASK_CORE0_WFI_0
    }
    #[doc = "Checks if the value of the field is `MASK_CORE0_WFI_1`"]
    #[inline(always)]
    pub fn is_mask_core0_wfi_1(&self) -> bool {
        *self == MASK_CORE0_WFI_A::MASK_CORE0_WFI_1
    }
}
#[doc = "Field `MASK_CORE0_WFI` writer - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
pub type MASK_CORE0_WFI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLPCR_SPEC, MASK_CORE0_WFI_A, O>;
impl<'a, const O: u8> MASK_CORE0_WFI_W<'a, O> {
    #[doc = "WFI of core0 is not masked"]
    #[inline(always)]
    pub fn mask_core0_wfi_0(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFI_A::MASK_CORE0_WFI_0)
    }
    #[doc = "WFI of core0 is masked"]
    #[inline(always)]
    pub fn mask_core0_wfi_1(self) -> &'a mut W {
        self.variant(MASK_CORE0_WFI_A::MASK_CORE0_WFI_1)
    }
}
#[doc = "Field `MASK_SCU_IDLE` reader - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
pub type MASK_SCU_IDLE_R = crate::BitReader<MASK_SCU_IDLE_A>;
#[doc = "Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_SCU_IDLE_A {
    #[doc = "0: SCU IDLE is not masked"]
    MASK_SCU_IDLE_0 = 0,
    #[doc = "1: SCU IDLE is masked"]
    MASK_SCU_IDLE_1 = 1,
}
impl From<MASK_SCU_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_SCU_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_SCU_IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_SCU_IDLE_A {
        match self.bits {
            false => MASK_SCU_IDLE_A::MASK_SCU_IDLE_0,
            true => MASK_SCU_IDLE_A::MASK_SCU_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_0`"]
    #[inline(always)]
    pub fn is_mask_scu_idle_0(&self) -> bool {
        *self == MASK_SCU_IDLE_A::MASK_SCU_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_SCU_IDLE_1`"]
    #[inline(always)]
    pub fn is_mask_scu_idle_1(&self) -> bool {
        *self == MASK_SCU_IDLE_A::MASK_SCU_IDLE_1
    }
}
#[doc = "Field `MASK_SCU_IDLE` writer - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
pub type MASK_SCU_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLPCR_SPEC, MASK_SCU_IDLE_A, O>;
impl<'a, const O: u8> MASK_SCU_IDLE_W<'a, O> {
    #[doc = "SCU IDLE is not masked"]
    #[inline(always)]
    pub fn mask_scu_idle_0(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLE_A::MASK_SCU_IDLE_0)
    }
    #[doc = "SCU IDLE is masked"]
    #[inline(always)]
    pub fn mask_scu_idle_1(self) -> &'a mut W {
        self.variant(MASK_SCU_IDLE_A::MASK_SCU_IDLE_1)
    }
}
#[doc = "Field `MASK_L2CC_IDLE` reader - Mask L2CC IDLE for entering low power mode"]
pub type MASK_L2CC_IDLE_R = crate::BitReader<MASK_L2CC_IDLE_A>;
#[doc = "Mask L2CC IDLE for entering low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASK_L2CC_IDLE_A {
    #[doc = "0: L2CC IDLE is not masked"]
    MASK_L2CC_IDLE_0 = 0,
    #[doc = "1: L2CC IDLE is masked"]
    MASK_L2CC_IDLE_1 = 1,
}
impl From<MASK_L2CC_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_L2CC_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl MASK_L2CC_IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_L2CC_IDLE_A {
        match self.bits {
            false => MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0,
            true => MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_0`"]
    #[inline(always)]
    pub fn is_mask_l2cc_idle_0(&self) -> bool {
        *self == MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0
    }
    #[doc = "Checks if the value of the field is `MASK_L2CC_IDLE_1`"]
    #[inline(always)]
    pub fn is_mask_l2cc_idle_1(&self) -> bool {
        *self == MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1
    }
}
#[doc = "Field `MASK_L2CC_IDLE` writer - Mask L2CC IDLE for entering low power mode"]
pub type MASK_L2CC_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLPCR_SPEC, MASK_L2CC_IDLE_A, O>;
impl<'a, const O: u8> MASK_L2CC_IDLE_W<'a, O> {
    #[doc = "L2CC IDLE is not masked"]
    #[inline(always)]
    pub fn mask_l2cc_idle_0(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_0)
    }
    #[doc = "L2CC IDLE is masked"]
    #[inline(always)]
    pub fn mask_l2cc_idle_1(self) -> &'a mut W {
        self.variant(MASK_L2CC_IDLE_A::MASK_L2CC_IDLE_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    pub fn arm_clk_dis_on_lpm(&self) -> ARM_CLK_DIS_ON_LPM_R {
        ARM_CLK_DIS_ON_LPM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline(always)]
    pub fn sbyos(&self) -> SBYOS_R {
        SBYOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    pub fn dis_ref_osc(&self) -> DIS_REF_OSC_R {
        DIS_REF_OSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline(always)]
    pub fn vstby(&self) -> VSTBY_R {
        VSTBY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline(always)]
    pub fn stby_count(&self) -> STBY_COUNT_R {
        STBY_COUNT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    pub fn cosc_pwrdown(&self) -> COSC_PWRDOWN_R {
        COSC_PWRDOWN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs1(&self) -> BYPASS_LPM_HS1_R {
        BYPASS_LPM_HS1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    pub fn bypass_lpm_hs0(&self) -> BYPASS_LPM_HS0_R {
        BYPASS_LPM_HS0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
    #[inline(always)]
    pub fn mask_core0_wfi(&self) -> MASK_CORE0_WFI_R {
        MASK_CORE0_WFI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
    #[inline(always)]
    pub fn mask_scu_idle(&self) -> MASK_SCU_IDLE_R {
        MASK_SCU_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    pub fn mask_l2cc_idle(&self) -> MASK_L2CC_IDLE_R {
        MASK_L2CC_IDLE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Setting the low power mode that system will enter on next assertion of dsm_request signal."]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<0> {
        LPM_W::new(self)
    }
    #[doc = "Bit 5 - Define if Arm clocks (arm_clk, soc_mxclk, soc_pclk, soc_dbg_pclk, vl_wrck) will be disabled on wait mode"]
    #[inline(always)]
    #[must_use]
    pub fn arm_clk_dis_on_lpm(&mut self) -> ARM_CLK_DIS_ON_LPM_W<5> {
        ARM_CLK_DIS_ON_LPM_W::new(self)
    }
    #[doc = "Bit 6 - Standby clock oscillator bit"]
    #[inline(always)]
    #[must_use]
    pub fn sbyos(&mut self) -> SBYOS_W<6> {
        SBYOS_W::new(self)
    }
    #[doc = "Bit 7 - dis_ref_osc - in run mode, software can manually control closing of external reference oscillator clock, i"]
    #[inline(always)]
    #[must_use]
    pub fn dis_ref_osc(&mut self) -> DIS_REF_OSC_W<7> {
        DIS_REF_OSC_W::new(self)
    }
    #[doc = "Bit 8 - Voltage standby request bit"]
    #[inline(always)]
    #[must_use]
    pub fn vstby(&mut self) -> VSTBY_W<8> {
        VSTBY_W::new(self)
    }
    #[doc = "Bits 9:10 - Standby counter definition"]
    #[inline(always)]
    #[must_use]
    pub fn stby_count(&mut self) -> STBY_COUNT_W<9> {
        STBY_COUNT_W::new(self)
    }
    #[doc = "Bit 11 - In run mode, software can manually control powering down of on chip oscillator, i"]
    #[inline(always)]
    #[must_use]
    pub fn cosc_pwrdown(&mut self) -> COSC_PWRDOWN_W<11> {
        COSC_PWRDOWN_W::new(self)
    }
    #[doc = "Bit 19 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_lpm_hs1(&mut self) -> BYPASS_LPM_HS1_W<19> {
        BYPASS_LPM_HS1_W::new(self)
    }
    #[doc = "Bit 21 - Bypass low power mode handshake. This bit should always be set to 1'b1 by software."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_lpm_hs0(&mut self) -> BYPASS_LPM_HS0_W<21> {
        BYPASS_LPM_HS0_W::new(self)
    }
    #[doc = "Bit 22 - Mask WFI of core0 for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mask_core0_wfi(&mut self) -> MASK_CORE0_WFI_W<22> {
        MASK_CORE0_WFI_W::new(self)
    }
    #[doc = "Bit 26 - Mask SCU IDLE for entering low power mode Assertion of all bits\\[27:22\\]
will generate low power mode request"]
    #[inline(always)]
    #[must_use]
    pub fn mask_scu_idle(&mut self) -> MASK_SCU_IDLE_W<26> {
        MASK_SCU_IDLE_W::new(self)
    }
    #[doc = "Bit 27 - Mask L2CC IDLE for entering low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn mask_l2cc_idle(&mut self) -> MASK_L2CC_IDLE_W<27> {
        MASK_L2CC_IDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpcr](index.html) module"]
pub struct CLPCR_SPEC;
impl crate::RegisterSpec for CLPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clpcr::R](R) reader structure"]
impl crate::Readable for CLPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clpcr::W](W) writer structure"]
impl crate::Writable for CLPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLPCR to value 0x79"]
impl crate::Resettable for CLPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x79;
}
