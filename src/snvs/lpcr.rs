#[doc = "Register `LPCR` reader"]
pub struct R(crate::R<LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCR` writer"]
pub struct W(crate::W<LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCR_SPEC>;
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
impl From<crate::W<LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRTC_ENV` reader - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
pub type SRTC_ENV_R = crate::BitReader<SRTC_ENV_A>;
#[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTC_ENV_A {
    #[doc = "0: SRTC is disabled or invalid."]
    DISABLED = 0,
    #[doc = "1: SRTC is enabled and valid."]
    ENABLED = 1,
}
impl From<SRTC_ENV_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_ENV_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTC_ENV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_ENV_A {
        match self.bits {
            false => SRTC_ENV_A::DISABLED,
            true => SRTC_ENV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRTC_ENV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRTC_ENV_A::ENABLED
    }
}
#[doc = "Field `SRTC_ENV` writer - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
pub type SRTC_ENV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, SRTC_ENV_A, O>;
impl<'a, const O: u8> SRTC_ENV_W<'a, O> {
    #[doc = "SRTC is disabled or invalid."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRTC_ENV_A::DISABLED)
    }
    #[doc = "SRTC is enabled and valid."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRTC_ENV_A::ENABLED)
    }
}
#[doc = "Field `LPTA_EN` reader - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
pub type LPTA_EN_R = crate::BitReader<LPTA_EN_A>;
#[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTA_EN_A {
    #[doc = "0: LP time alarm interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: LP time alarm interrupt is enabled."]
    ENABLED = 1,
}
impl From<LPTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTA_EN_A {
        match self.bits {
            false => LPTA_EN_A::DISABLED,
            true => LPTA_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTA_EN_A::ENABLED
    }
}
#[doc = "Field `LPTA_EN` writer - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
pub type LPTA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, LPTA_EN_A, O>;
impl<'a, const O: u8> LPTA_EN_W<'a, O> {
    #[doc = "LP time alarm interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTA_EN_A::DISABLED)
    }
    #[doc = "LP time alarm interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTA_EN_A::ENABLED)
    }
}
#[doc = "Field `MC_ENV` reader - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
pub type MC_ENV_R = crate::BitReader<MC_ENV_A>;
#[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MC_ENV_A {
    #[doc = "0: MC is disabled or invalid."]
    DISABLED = 0,
    #[doc = "1: MC is enabled and valid."]
    ENABLED = 1,
}
impl From<MC_ENV_A> for bool {
    #[inline(always)]
    fn from(variant: MC_ENV_A) -> Self {
        variant as u8 != 0
    }
}
impl MC_ENV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_ENV_A {
        match self.bits {
            false => MC_ENV_A::DISABLED,
            true => MC_ENV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MC_ENV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MC_ENV_A::ENABLED
    }
}
#[doc = "Field `MC_ENV` writer - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
pub type MC_ENV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, MC_ENV_A, O>;
impl<'a, const O: u8> MC_ENV_W<'a, O> {
    #[doc = "MC is disabled or invalid."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MC_ENV_A::DISABLED)
    }
    #[doc = "MC is enabled and valid."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MC_ENV_A::ENABLED)
    }
}
#[doc = "Field `LPWUI_EN` reader - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (MC rollover, SRTC rollover, or time alarm )"]
pub type LPWUI_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPWUI_EN` writer - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (MC rollover, SRTC rollover, or time alarm )"]
pub type LPWUI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `SRTC_INV_EN` reader - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
pub type SRTC_INV_EN_R = crate::BitReader<SRTC_INV_EN_A>;
#[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTC_INV_EN_A {
    #[doc = "0: SRTC stays valid in the case of security violation (other than a software violation (HPSVSR\\[SW_LPSV\\]
= 1 or HPCOMR\\[SW_LPSV\\]
= 1))."]
    KEEP_VALID = 0,
    #[doc = "1: SRTC is invalidated in the case of security violation."]
    INVALIDATE = 1,
}
impl From<SRTC_INV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_INV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTC_INV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_INV_EN_A {
        match self.bits {
            false => SRTC_INV_EN_A::KEEP_VALID,
            true => SRTC_INV_EN_A::INVALIDATE,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP_VALID`"]
    #[inline(always)]
    pub fn is_keep_valid(&self) -> bool {
        *self == SRTC_INV_EN_A::KEEP_VALID
    }
    #[doc = "Checks if the value of the field is `INVALIDATE`"]
    #[inline(always)]
    pub fn is_invalidate(&self) -> bool {
        *self == SRTC_INV_EN_A::INVALIDATE
    }
}
#[doc = "Field `SRTC_INV_EN` writer - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
pub type SRTC_INV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, SRTC_INV_EN_A, O>;
impl<'a, const O: u8> SRTC_INV_EN_W<'a, O> {
    #[doc = "SRTC stays valid in the case of security violation (other than a software violation (HPSVSR\\[SW_LPSV\\]
= 1 or HPCOMR\\[SW_LPSV\\]
= 1))."]
    #[inline(always)]
    pub fn keep_valid(self) -> &'a mut W {
        self.variant(SRTC_INV_EN_A::KEEP_VALID)
    }
    #[doc = "SRTC is invalidated in the case of security violation."]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut W {
        self.variant(SRTC_INV_EN_A::INVALIDATE)
    }
}
#[doc = "Field `DP_EN` reader - Dumb PMIC Enabled When set, software can control the system power"]
pub type DP_EN_R = crate::BitReader<DP_EN_A>;
#[doc = "Dumb PMIC Enabled When set, software can control the system power\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DP_EN_A {
    #[doc = "0: Smart PMIC enabled."]
    SMART_PMIC_ENABLED = 0,
    #[doc = "1: Dumb PMIC enabled."]
    DUMB_PMIC_ENABLED = 1,
}
impl From<DP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_EN_A {
        match self.bits {
            false => DP_EN_A::SMART_PMIC_ENABLED,
            true => DP_EN_A::DUMB_PMIC_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SMART_PMIC_ENABLED`"]
    #[inline(always)]
    pub fn is_smart_pmic_enabled(&self) -> bool {
        *self == DP_EN_A::SMART_PMIC_ENABLED
    }
    #[doc = "Checks if the value of the field is `DUMB_PMIC_ENABLED`"]
    #[inline(always)]
    pub fn is_dumb_pmic_enabled(&self) -> bool {
        *self == DP_EN_A::DUMB_PMIC_ENABLED
    }
}
#[doc = "Field `DP_EN` writer - Dumb PMIC Enabled When set, software can control the system power"]
pub type DP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, DP_EN_A, O>;
impl<'a, const O: u8> DP_EN_W<'a, O> {
    #[doc = "Smart PMIC enabled."]
    #[inline(always)]
    pub fn smart_pmic_enabled(self) -> &'a mut W {
        self.variant(DP_EN_A::SMART_PMIC_ENABLED)
    }
    #[doc = "Dumb PMIC enabled."]
    #[inline(always)]
    pub fn dumb_pmic_enabled(self) -> &'a mut W {
        self.variant(DP_EN_A::DUMB_PMIC_ENABLED)
    }
}
#[doc = "Field `TOP` reader - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
pub type TOP_R = crate::BitReader<TOP_A>;
#[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOP_A {
    #[doc = "0: Leave system power on."]
    KEEP_ON = 0,
    #[doc = "1: Turn off system power."]
    TURN_OFF = 1,
}
impl From<TOP_A> for bool {
    #[inline(always)]
    fn from(variant: TOP_A) -> Self {
        variant as u8 != 0
    }
}
impl TOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOP_A {
        match self.bits {
            false => TOP_A::KEEP_ON,
            true => TOP_A::TURN_OFF,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP_ON`"]
    #[inline(always)]
    pub fn is_keep_on(&self) -> bool {
        *self == TOP_A::KEEP_ON
    }
    #[doc = "Checks if the value of the field is `TURN_OFF`"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == TOP_A::TURN_OFF
    }
}
#[doc = "Field `TOP` writer - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
pub type TOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, TOP_A, O>;
impl<'a, const O: u8> TOP_W<'a, O> {
    #[doc = "Leave system power on."]
    #[inline(always)]
    pub fn keep_on(self) -> &'a mut W {
        self.variant(TOP_A::KEEP_ON)
    }
    #[doc = "Turn off system power."]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut W {
        self.variant(TOP_A::TURN_OFF)
    }
}
#[doc = "Field `LVD_EN` reader - Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
pub type LVD_EN_R = crate::BitReader<bool>;
#[doc = "Field `LVD_EN` writer - Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
pub type LVD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `LPCALB_EN` reader - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
pub type LPCALB_EN_R = crate::BitReader<LPCALB_EN_A>;
#[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCALB_EN_A {
    #[doc = "0: SRTC Time calibration is disabled."]
    DISABLED = 0,
    #[doc = "1: SRTC Time calibration is enabled."]
    ENABLED = 1,
}
impl From<LPCALB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCALB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_EN_A {
        match self.bits {
            false => LPCALB_EN_A::DISABLED,
            true => LPCALB_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPCALB_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPCALB_EN_A::ENABLED
    }
}
#[doc = "Field `LPCALB_EN` writer - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
pub type LPCALB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, LPCALB_EN_A, O>;
impl<'a, const O: u8> LPCALB_EN_W<'a, O> {
    #[doc = "SRTC Time calibration is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPCALB_EN_A::DISABLED)
    }
    #[doc = "SRTC Time calibration is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPCALB_EN_A::ENABLED)
    }
}
#[doc = "Field `LPCALB_VAL` reader - LP Calibration Value Defines signed calibration value for SRTC"]
pub type LPCALB_VAL_R = crate::FieldReader<u8, LPCALB_VAL_A>;
#[doc = "LP Calibration Value Defines signed calibration value for SRTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCALB_VAL_A {
    #[doc = "0: +0 counts per each 32768 ticks of the counter clock"]
    ADD_0_PER_32768_TICKS = 0,
    #[doc = "1: +1 counts per each 32768 ticks of the counter clock"]
    ADD_1_PER_32768_TICKS = 1,
    #[doc = "2: +2 counts per each 32768 ticks of the counter clock"]
    ADD_2_PER_32768_TICKS = 2,
    #[doc = "15: +15 counts per each 32768 ticks of the counter clock"]
    ADD_15_PER_32768_TICKS = 15,
    #[doc = "16: -16 counts per each 32768 ticks of the counter clock"]
    SUB_16_PER_32768_TICKS = 16,
    #[doc = "17: -15 counts per each 32768 ticks of the counter clock"]
    SUB_15_PER_32768_TICKS = 17,
    #[doc = "30: -2 counts per each 32768 ticks of the counter clock"]
    SUB_2_PER_32768_TICKS = 30,
    #[doc = "31: -1 counts per each 32768 ticks of the counter clock"]
    SUB_1_PER_32768_TICKS = 31,
}
impl From<LPCALB_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCALB_VAL_A) -> Self {
        variant as _
    }
}
impl LPCALB_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPCALB_VAL_A> {
        match self.bits {
            0 => Some(LPCALB_VAL_A::ADD_0_PER_32768_TICKS),
            1 => Some(LPCALB_VAL_A::ADD_1_PER_32768_TICKS),
            2 => Some(LPCALB_VAL_A::ADD_2_PER_32768_TICKS),
            15 => Some(LPCALB_VAL_A::ADD_15_PER_32768_TICKS),
            16 => Some(LPCALB_VAL_A::SUB_16_PER_32768_TICKS),
            17 => Some(LPCALB_VAL_A::SUB_15_PER_32768_TICKS),
            30 => Some(LPCALB_VAL_A::SUB_2_PER_32768_TICKS),
            31 => Some(LPCALB_VAL_A::SUB_1_PER_32768_TICKS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADD_0_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_0_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::ADD_0_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_1_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_1_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::ADD_1_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_2_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_2_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::ADD_2_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_15_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_15_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::ADD_15_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_16_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_16_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::SUB_16_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_15_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_15_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::SUB_15_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_2_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_2_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::SUB_2_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_1_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_1_per_32768_ticks(&self) -> bool {
        *self == LPCALB_VAL_A::SUB_1_PER_32768_TICKS
    }
}
#[doc = "Field `LPCALB_VAL` writer - LP Calibration Value Defines signed calibration value for SRTC"]
pub type LPCALB_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPCR_SPEC, u8, LPCALB_VAL_A, 5, O>;
impl<'a, const O: u8> LPCALB_VAL_W<'a, O> {
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn add_0_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::ADD_0_PER_32768_TICKS)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn add_1_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::ADD_1_PER_32768_TICKS)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn add_2_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::ADD_2_PER_32768_TICKS)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn add_15_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::ADD_15_PER_32768_TICKS)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn sub_16_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::SUB_16_PER_32768_TICKS)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn sub_15_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::SUB_15_PER_32768_TICKS)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn sub_2_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::SUB_2_PER_32768_TICKS)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn sub_1_per_32768_ticks(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::SUB_1_PER_32768_TICKS)
    }
}
#[doc = "Field `BTN_PRESS_TIME` reader - This field configures the button press time out values for the PMIC Logic"]
pub type BTN_PRESS_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BTN_PRESS_TIME` writer - This field configures the button press time out values for the PMIC Logic"]
pub type BTN_PRESS_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DEBOUNCE` reader - This field configures the amount of debounce time for the BTN input signal"]
pub type DEBOUNCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBOUNCE` writer - This field configures the amount of debounce time for the BTN input signal"]
pub type DEBOUNCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ON_TIME` reader - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
pub type ON_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ON_TIME` writer - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
pub type ON_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PK_EN` reader - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
pub type PK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PK_EN` writer - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
pub type PK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `PK_OVERRIDE` reader - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
pub type PK_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `PK_OVERRIDE` writer - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
pub type PK_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `GPR_Z_DIS` reader - General Purpose Registers Zeroization Disable"]
pub type GPR_Z_DIS_R = crate::BitReader<bool>;
#[doc = "Field `GPR_Z_DIS` writer - General Purpose Registers Zeroization Disable"]
pub type GPR_Z_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub fn srtc_env(&self) -> SRTC_ENV_R {
        SRTC_ENV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub fn lpta_en(&self) -> LPTA_EN_R {
        LPTA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub fn mc_env(&self) -> MC_ENV_R {
        MC_ENV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub fn lpwui_en(&self) -> LPWUI_EN_R {
        LPWUI_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub fn srtc_inv_en(&self) -> SRTC_INV_EN_R {
        SRTC_INV_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub fn dp_en(&self) -> DP_EN_R {
        DP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub fn lvd_en(&self) -> LVD_EN_R {
        LVD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub fn lpcalb_en(&self) -> LPCALB_EN_R {
        LPCALB_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub fn lpcalb_val(&self) -> LPCALB_VAL_R {
        LPCALB_VAL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub fn btn_press_time(&self) -> BTN_PRESS_TIME_R {
        BTN_PRESS_TIME_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub fn debounce(&self) -> DEBOUNCE_R {
        DEBOUNCE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub fn on_time(&self) -> ON_TIME_R {
        ON_TIME_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub fn pk_en(&self) -> PK_EN_R {
        PK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub fn pk_override(&self) -> PK_OVERRIDE_R {
        PK_OVERRIDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub fn gpr_z_dis(&self) -> GPR_Z_DIS_R {
        GPR_Z_DIS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    #[must_use]
    pub fn srtc_env(&mut self) -> SRTC_ENV_W<0> {
        SRTC_ENV_W::new(self)
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    #[must_use]
    pub fn lpta_en(&mut self) -> LPTA_EN_W<1> {
        LPTA_EN_W::new(self)
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    #[must_use]
    pub fn mc_env(&mut self) -> MC_ENV_W<2> {
        MC_ENV_W::new(self)
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    #[must_use]
    pub fn lpwui_en(&mut self) -> LPWUI_EN_W<3> {
        LPWUI_EN_W::new(self)
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    #[must_use]
    pub fn srtc_inv_en(&mut self) -> SRTC_INV_EN_W<4> {
        SRTC_INV_EN_W::new(self)
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    #[must_use]
    pub fn dp_en(&mut self) -> DP_EN_W<5> {
        DP_EN_W::new(self)
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<6> {
        TOP_W::new(self)
    }
    #[doc = "Bit 7 - Digital Low-Voltage Event Enable By default the detection of a low-voltage event does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    #[must_use]
    pub fn lvd_en(&mut self) -> LVD_EN_W<7> {
        LVD_EN_W::new(self)
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn lpcalb_en(&mut self) -> LPCALB_EN_W<8> {
        LPCALB_EN_W::new(self)
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    #[must_use]
    pub fn lpcalb_val(&mut self) -> LPCALB_VAL_W<10> {
        LPCALB_VAL_W::new(self)
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    #[must_use]
    pub fn btn_press_time(&mut self) -> BTN_PRESS_TIME_W<16> {
        BTN_PRESS_TIME_W::new(self)
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    #[must_use]
    pub fn debounce(&mut self) -> DEBOUNCE_W<18> {
        DEBOUNCE_W::new(self)
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    #[must_use]
    pub fn on_time(&mut self) -> ON_TIME_W<20> {
        ON_TIME_W::new(self)
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    #[must_use]
    pub fn pk_en(&mut self) -> PK_EN_W<22> {
        PK_EN_W::new(self)
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    #[must_use]
    pub fn pk_override(&mut self) -> PK_OVERRIDE_W<23> {
        PK_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gpr_z_dis(&mut self) -> GPR_Z_DIS_W<24> {
        GPR_Z_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](index.html) module"]
pub struct LPCR_SPEC;
impl crate::RegisterSpec for LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcr::R](R) reader structure"]
impl crate::Readable for LPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpcr::W](W) writer structure"]
impl crate::Writable for LPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCR to value 0x20"]
impl crate::Resettable for LPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
