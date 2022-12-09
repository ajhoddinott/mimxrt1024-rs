#[doc = "Register `HPCOMR` reader"]
pub struct R(crate::R<HPCOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPCOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPCOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPCOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPCOMR` writer"]
pub struct W(crate::W<HPCOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPCOMR_SPEC>;
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
impl From<crate::W<HPCOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPCOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSM_ST` writer - SSM State Transition Transition state of the system security monitor"]
pub type SSM_ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
#[doc = "Field `SSM_ST_DIS` reader - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
pub type SSM_ST_DIS_R = crate::BitReader<SSM_ST_DIS_A>;
#[doc = "SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_ST_DIS_A {
    #[doc = "0: Secure to Trusted State transition is enabled"]
    ENABLED = 0,
    #[doc = "1: Secure to Trusted State transition is disabled"]
    DISABLED = 1,
}
impl From<SSM_ST_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_ST_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SSM_ST_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_ST_DIS_A {
        match self.bits {
            false => SSM_ST_DIS_A::ENABLED,
            true => SSM_ST_DIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_ST_DIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_ST_DIS_A::DISABLED
    }
}
#[doc = "Field `SSM_ST_DIS` writer - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
pub type SSM_ST_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, SSM_ST_DIS_A, O>;
impl<'a, const O: u8> SSM_ST_DIS_W<'a, O> {
    #[doc = "Secure to Trusted State transition is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_ST_DIS_A::ENABLED)
    }
    #[doc = "Secure to Trusted State transition is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_ST_DIS_A::DISABLED)
    }
}
#[doc = "Field `SSM_SFNS_DIS` reader - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
pub type SSM_SFNS_DIS_R = crate::BitReader<SSM_SFNS_DIS_A>;
#[doc = "SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_SFNS_DIS_A {
    #[doc = "0: Soft Fail to Non-Secure State transition is enabled"]
    ENABLED = 0,
    #[doc = "1: Soft Fail to Non-Secure State transition is disabled"]
    DISABLED = 1,
}
impl From<SSM_SFNS_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_SFNS_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SSM_SFNS_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_SFNS_DIS_A {
        match self.bits {
            false => SSM_SFNS_DIS_A::ENABLED,
            true => SSM_SFNS_DIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_SFNS_DIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_SFNS_DIS_A::DISABLED
    }
}
#[doc = "Field `SSM_SFNS_DIS` writer - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
pub type SSM_SFNS_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HPCOMR_SPEC, SSM_SFNS_DIS_A, O>;
impl<'a, const O: u8> SSM_SFNS_DIS_W<'a, O> {
    #[doc = "Soft Fail to Non-Secure State transition is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_SFNS_DIS_A::ENABLED)
    }
    #[doc = "Soft Fail to Non-Secure State transition is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_SFNS_DIS_A::DISABLED)
    }
}
#[doc = "LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_SWR_AW {
    #[doc = "0: No Action"]
    NO_ACTION = 0,
    #[doc = "1: Reset LP section"]
    RESET = 1,
}
impl From<LP_SWR_AW> for bool {
    #[inline(always)]
    fn from(variant: LP_SWR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP_SWR` writer - LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
pub type LP_SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, LP_SWR_AW, O>;
impl<'a, const O: u8> LP_SWR_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(LP_SWR_AW::NO_ACTION)
    }
    #[doc = "Reset LP section"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LP_SWR_AW::RESET)
    }
}
#[doc = "Field `LP_SWR_DIS` reader - LP Software Reset Disable When set, disables the LP software reset"]
pub type LP_SWR_DIS_R = crate::BitReader<LP_SWR_DIS_A>;
#[doc = "LP Software Reset Disable When set, disables the LP software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_SWR_DIS_A {
    #[doc = "0: LP software reset is enabled"]
    ENABLED = 0,
    #[doc = "1: LP software reset is disabled"]
    DISABLED = 1,
}
impl From<LP_SWR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: LP_SWR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LP_SWR_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_SWR_DIS_A {
        match self.bits {
            false => LP_SWR_DIS_A::ENABLED,
            true => LP_SWR_DIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LP_SWR_DIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LP_SWR_DIS_A::DISABLED
    }
}
#[doc = "Field `LP_SWR_DIS` writer - LP Software Reset Disable When set, disables the LP software reset"]
pub type LP_SWR_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, LP_SWR_DIS_A, O>;
impl<'a, const O: u8> LP_SWR_DIS_W<'a, O> {
    #[doc = "LP software reset is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LP_SWR_DIS_A::ENABLED)
    }
    #[doc = "LP software reset is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LP_SWR_DIS_A::DISABLED)
    }
}
#[doc = "Field `SW_SV` reader - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
pub type SW_SV_R = crate::BitReader<bool>;
#[doc = "Field `SW_SV` writer - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
pub type SW_SV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
#[doc = "Field `SW_FSV` reader - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
pub type SW_FSV_R = crate::BitReader<bool>;
#[doc = "Field `SW_FSV` writer - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
pub type SW_FSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
#[doc = "Field `SW_LPSV` reader - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
pub type SW_LPSV_R = crate::BitReader<bool>;
#[doc = "Field `SW_LPSV` writer - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
pub type SW_LPSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
#[doc = "Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROG_ZMK_AW {
    #[doc = "0: No Action"]
    NO_ACTION = 0,
    #[doc = "1: Activate hardware key programming mechanism"]
    PROGRAM_KEY = 1,
}
impl From<PROG_ZMK_AW> for bool {
    #[inline(always)]
    fn from(variant: PROG_ZMK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROG_ZMK` writer - Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
pub type PROG_ZMK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, PROG_ZMK_AW, O>;
impl<'a, const O: u8> PROG_ZMK_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(PROG_ZMK_AW::NO_ACTION)
    }
    #[doc = "Activate hardware key programming mechanism"]
    #[inline(always)]
    pub fn program_key(self) -> &'a mut W {
        self.variant(PROG_ZMK_AW::PROGRAM_KEY)
    }
}
#[doc = "Field `MKS_EN` reader - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
pub type MKS_EN_R = crate::BitReader<MKS_EN_A>;
#[doc = "Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MKS_EN_A {
    #[doc = "0: OTP master key is selected as an SNVS master key"]
    SELECT_OTP = 0,
    #[doc = "1: SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR"]
    SELECT_PER_LPMKCR = 1,
}
impl From<MKS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MKS_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_EN_A {
        match self.bits {
            false => MKS_EN_A::SELECT_OTP,
            true => MKS_EN_A::SELECT_PER_LPMKCR,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_OTP`"]
    #[inline(always)]
    pub fn is_select_otp(&self) -> bool {
        *self == MKS_EN_A::SELECT_OTP
    }
    #[doc = "Checks if the value of the field is `SELECT_PER_LPMKCR`"]
    #[inline(always)]
    pub fn is_select_per_lpmkcr(&self) -> bool {
        *self == MKS_EN_A::SELECT_PER_LPMKCR
    }
}
#[doc = "Field `MKS_EN` writer - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
pub type MKS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, MKS_EN_A, O>;
impl<'a, const O: u8> MKS_EN_W<'a, O> {
    #[doc = "OTP master key is selected as an SNVS master key"]
    #[inline(always)]
    pub fn select_otp(self) -> &'a mut W {
        self.variant(MKS_EN_A::SELECT_OTP)
    }
    #[doc = "SNVS master key is selected according to the setting of the MASTER_KEY_SEL field of LPMKCR"]
    #[inline(always)]
    pub fn select_per_lpmkcr(self) -> &'a mut W {
        self.variant(MKS_EN_A::SELECT_PER_LPMKCR)
    }
}
#[doc = "Field `HAC_EN` reader - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
pub type HAC_EN_R = crate::BitReader<HAC_EN_A>;
#[doc = "High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAC_EN_A {
    #[doc = "0: High Assurance Counter is disabled"]
    DISABLED = 0,
    #[doc = "1: High Assurance Counter is enabled"]
    ENABLED = 1,
}
impl From<HAC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HAC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HAC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAC_EN_A {
        match self.bits {
            false => HAC_EN_A::DISABLED,
            true => HAC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HAC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HAC_EN_A::ENABLED
    }
}
#[doc = "Field `HAC_EN` writer - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
pub type HAC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, HAC_EN_A, O>;
impl<'a, const O: u8> HAC_EN_W<'a, O> {
    #[doc = "High Assurance Counter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HAC_EN_A::DISABLED)
    }
    #[doc = "High Assurance Counter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HAC_EN_A::ENABLED)
    }
}
#[doc = "High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAC_LOAD_AW {
    #[doc = "0: No Action"]
    NO_ACTION = 0,
    #[doc = "1: Load the HAC"]
    LOAD_HAC = 1,
}
impl From<HAC_LOAD_AW> for bool {
    #[inline(always)]
    fn from(variant: HAC_LOAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HAC_LOAD` writer - High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
pub type HAC_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, HAC_LOAD_AW, O>;
impl<'a, const O: u8> HAC_LOAD_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(HAC_LOAD_AW::NO_ACTION)
    }
    #[doc = "Load the HAC"]
    #[inline(always)]
    pub fn load_hac(self) -> &'a mut W {
        self.variant(HAC_LOAD_AW::LOAD_HAC)
    }
}
#[doc = "High Assurance Counter Clear When set, it clears the High Assurance Counter Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAC_CLEAR_AW {
    #[doc = "0: No Action"]
    NO_ACTION = 0,
    #[doc = "1: Clear the HAC"]
    CLEAR_HAC = 1,
}
impl From<HAC_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: HAC_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HAC_CLEAR` writer - High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
pub type HAC_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, HAC_CLEAR_AW, O>;
impl<'a, const O: u8> HAC_CLEAR_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(HAC_CLEAR_AW::NO_ACTION)
    }
    #[doc = "Clear the HAC"]
    #[inline(always)]
    pub fn clear_hac(self) -> &'a mut W {
        self.variant(HAC_CLEAR_AW::CLEAR_HAC)
    }
}
#[doc = "Field `HAC_STOP` reader - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
pub type HAC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `HAC_STOP` writer - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
pub type HAC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
#[doc = "Field `NPSWA_EN` reader - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
pub type NPSWA_EN_R = crate::BitReader<bool>;
#[doc = "Field `NPSWA_EN` writer - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
pub type NPSWA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCOMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    pub fn ssm_st_dis(&self) -> SSM_ST_DIS_R {
        SSM_ST_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    pub fn ssm_sfns_dis(&self) -> SSM_SFNS_DIS_R {
        SSM_SFNS_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    pub fn lp_swr_dis(&self) -> LP_SWR_DIS_R {
        LP_SWR_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    pub fn sw_sv(&self) -> SW_SV_R {
        SW_SV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    pub fn sw_fsv(&self) -> SW_FSV_R {
        SW_FSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    pub fn sw_lpsv(&self) -> SW_LPSV_R {
        SW_LPSV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    pub fn mks_en(&self) -> MKS_EN_R {
        MKS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    pub fn hac_en(&self) -> HAC_EN_R {
        HAC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    pub fn hac_stop(&self) -> HAC_STOP_R {
        HAC_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    pub fn npswa_en(&self) -> NPSWA_EN_R {
        NPSWA_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSM State Transition Transition state of the system security monitor"]
    #[inline(always)]
    #[must_use]
    pub fn ssm_st(&mut self) -> SSM_ST_W<0> {
        SSM_ST_W::new(self)
    }
    #[doc = "Bit 1 - SSM Secure to Trusted State Transition Disable When set, disables the SSM transition from secure to trusted state"]
    #[inline(always)]
    #[must_use]
    pub fn ssm_st_dis(&mut self) -> SSM_ST_DIS_W<1> {
        SSM_ST_DIS_W::new(self)
    }
    #[doc = "Bit 2 - SSM Soft Fail to Non-Secure State Transition Disable When set, it disables the SSM transition from soft fail to non-secure state"]
    #[inline(always)]
    #[must_use]
    pub fn ssm_sfns_dis(&mut self) -> SSM_SFNS_DIS_W<2> {
        SSM_SFNS_DIS_W::new(self)
    }
    #[doc = "Bit 4 - LP Software Reset When set to 1, most registers in the SNVS_LP section are reset, but the following registers are not reset by an LP software reset: Monotonic Counter Secure Real Time Counter Time Alarm Register This bit cannot be set when the LP_SWR_DIS bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn lp_swr(&mut self) -> LP_SWR_W<4> {
        LP_SWR_W::new(self)
    }
    #[doc = "Bit 5 - LP Software Reset Disable When set, disables the LP software reset"]
    #[inline(always)]
    #[must_use]
    pub fn lp_swr_dis(&mut self) -> LP_SWR_DIS_W<5> {
        LP_SWR_DIS_W::new(self)
    }
    #[doc = "Bit 8 - Software Security Violation When set, the system security monitor treats this bit as a non-fatal security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sv(&mut self) -> SW_SV_W<8> {
        SW_SV_W::new(self)
    }
    #[doc = "Bit 9 - Software Fatal Security Violation When set, the system security monitor treats this bit as a fatal security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fsv(&mut self) -> SW_FSV_W<9> {
        SW_FSV_W::new(self)
    }
    #[doc = "Bit 10 - LP Software Security Violation When set, SNVS_LP treats this bit as a security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sw_lpsv(&mut self) -> SW_LPSV_W<10> {
        SW_LPSV_W::new(self)
    }
    #[doc = "Bit 12 - Program Zeroizable Master Key This bit activates ZMK hardware programming mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn prog_zmk(&mut self) -> PROG_ZMK_W<12> {
        PROG_ZMK_W::new(self)
    }
    #[doc = "Bit 13 - Master Key Select Enable When not set, the one time programmable (OTP) master key is selected by default"]
    #[inline(always)]
    #[must_use]
    pub fn mks_en(&mut self) -> MKS_EN_W<13> {
        MKS_EN_W::new(self)
    }
    #[doc = "Bit 16 - High Assurance Counter Enable This bit controls the SSM transition from the soft fail to the hard fail state"]
    #[inline(always)]
    #[must_use]
    pub fn hac_en(&mut self) -> HAC_EN_W<16> {
        HAC_EN_W::new(self)
    }
    #[doc = "Bit 17 - High Assurance Counter Load When set, it loads the High Assurance Counter Register with the value of the High Assurance Counter Load Register"]
    #[inline(always)]
    #[must_use]
    pub fn hac_load(&mut self) -> HAC_LOAD_W<17> {
        HAC_LOAD_W::new(self)
    }
    #[doc = "Bit 18 - High Assurance Counter Clear When set, it clears the High Assurance Counter Register"]
    #[inline(always)]
    #[must_use]
    pub fn hac_clear(&mut self) -> HAC_CLEAR_W<18> {
        HAC_CLEAR_W::new(self)
    }
    #[doc = "Bit 19 - High Assurance Counter Stop This bit can be set only when SSM is in soft fail state"]
    #[inline(always)]
    #[must_use]
    pub fn hac_stop(&mut self) -> HAC_STOP_W<19> {
        HAC_STOP_W::new(self)
    }
    #[doc = "Bit 31 - Non-Privileged Software Access Enable When set, allows non-privileged software to access all SNVS registers, including those that are privileged software read/write access only"]
    #[inline(always)]
    #[must_use]
    pub fn npswa_en(&mut self) -> NPSWA_EN_W<31> {
        NPSWA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcomr](index.html) module"]
pub struct HPCOMR_SPEC;
impl crate::RegisterSpec for HPCOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpcomr::R](R) reader structure"]
impl crate::Readable for HPCOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpcomr::W](W) writer structure"]
impl crate::Writable for HPCOMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPCOMR to value 0"]
impl crate::Resettable for HPCOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
