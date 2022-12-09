#[doc = "Register `MISC0_SET` reader"]
pub struct R(crate::R<MISC0_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC0_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC0_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC0_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC0_SET` writer"]
pub struct W(crate::W<MISC0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC0_SET_SPEC>;
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
impl From<crate::W<MISC0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFTOP_PWD` reader - Control bit to power-down the analog bandgap reference circuitry"]
pub type REFTOP_PWD_R = crate::BitReader<bool>;
#[doc = "Field `REFTOP_PWD` writer - Control bit to power-down the analog bandgap reference circuitry"]
pub type REFTOP_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
#[doc = "Field `REFTOP_PWDVBGUP` reader - Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
pub type REFTOP_PWDVBGUP_R = crate::BitReader<bool>;
#[doc = "Field `REFTOP_PWDVBGUP` writer - Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
pub type REFTOP_PWDVBGUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
#[doc = "Field `REFTOP_LOWPOWER` reader - Control bit to enable the low-power mode in the analog bandgap."]
pub type REFTOP_LOWPOWER_R = crate::BitReader<bool>;
#[doc = "Field `REFTOP_LOWPOWER` writer - Control bit to enable the low-power mode in the analog bandgap."]
pub type REFTOP_LOWPOWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
#[doc = "Field `REFTOP_SELFBIASOFF` reader - Control bit to disable the self-bias circuit in the analog bandgap"]
pub type REFTOP_SELFBIASOFF_R = crate::BitReader<REFTOP_SELFBIASOFF_A>;
#[doc = "Control bit to disable the self-bias circuit in the analog bandgap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFTOP_SELFBIASOFF_A {
    #[doc = "0: Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0,
    #[doc = "1: Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 1,
}
impl From<REFTOP_SELFBIASOFF_A> for bool {
    #[inline(always)]
    fn from(variant: REFTOP_SELFBIASOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl REFTOP_SELFBIASOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTOP_SELFBIASOFF_A {
        match self.bits {
            false => REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0,
            true => REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_0`"]
    #[inline(always)]
    pub fn is_reftop_selfbiasoff_0(&self) -> bool {
        *self == REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_1`"]
    #[inline(always)]
    pub fn is_reftop_selfbiasoff_1(&self) -> bool {
        *self == REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1
    }
}
#[doc = "Field `REFTOP_SELFBIASOFF` writer - Control bit to disable the self-bias circuit in the analog bandgap"]
pub type REFTOP_SELFBIASOFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC0_SET_SPEC, REFTOP_SELFBIASOFF_A, O>;
impl<'a, const O: u8> REFTOP_SELFBIASOFF_W<'a, O> {
    #[doc = "Uses coarse bias currents for startup"]
    #[inline(always)]
    pub fn reftop_selfbiasoff_0(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0)
    }
    #[doc = "Uses bandgap-based bias currents for best performance."]
    #[inline(always)]
    pub fn reftop_selfbiasoff_1(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1)
    }
}
#[doc = "Field `REFTOP_VBGADJ` reader - no description available"]
pub type REFTOP_VBGADJ_R = crate::FieldReader<u8, REFTOP_VBGADJ_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFTOP_VBGADJ_A {
    #[doc = "0: Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
    #[doc = "1: VBG+0.78%"]
    REFTOP_VBGADJ_1 = 1,
    #[doc = "2: VBG+1.56%"]
    REFTOP_VBGADJ_2 = 2,
    #[doc = "3: VBG+2.34%"]
    REFTOP_VBGADJ_3 = 3,
    #[doc = "4: VBG-0.78%"]
    REFTOP_VBGADJ_4 = 4,
    #[doc = "5: VBG-1.56%"]
    REFTOP_VBGADJ_5 = 5,
    #[doc = "6: VBG-2.34%"]
    REFTOP_VBGADJ_6 = 6,
    #[doc = "7: VBG-3.12%"]
    REFTOP_VBGADJ_7 = 7,
}
impl From<REFTOP_VBGADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REFTOP_VBGADJ_A) -> Self {
        variant as _
    }
}
impl REFTOP_VBGADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTOP_VBGADJ_A {
        match self.bits {
            0 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_0,
            1 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_1,
            2 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_2,
            3 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_3,
            4 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_4,
            5 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_5,
            6 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_6,
            7 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_0`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_0(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_1`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_1(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_1
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_2`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_2(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_2
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_3`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_3(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_3
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_4`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_4(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_4
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_5`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_5(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_5
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_6`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_6(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_6
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_7`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_7(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_7
    }
}
#[doc = "Field `REFTOP_VBGADJ` writer - no description available"]
pub type REFTOP_VBGADJ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC0_SET_SPEC, u8, REFTOP_VBGADJ_A, 3, O>;
impl<'a, const O: u8> REFTOP_VBGADJ_W<'a, O> {
    #[doc = "Nominal VBG"]
    #[inline(always)]
    pub fn reftop_vbgadj_0(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_0)
    }
    #[doc = "VBG+0.78%"]
    #[inline(always)]
    pub fn reftop_vbgadj_1(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_1)
    }
    #[doc = "VBG+1.56%"]
    #[inline(always)]
    pub fn reftop_vbgadj_2(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_2)
    }
    #[doc = "VBG+2.34%"]
    #[inline(always)]
    pub fn reftop_vbgadj_3(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_3)
    }
    #[doc = "VBG-0.78%"]
    #[inline(always)]
    pub fn reftop_vbgadj_4(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_4)
    }
    #[doc = "VBG-1.56%"]
    #[inline(always)]
    pub fn reftop_vbgadj_5(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_5)
    }
    #[doc = "VBG-2.34%"]
    #[inline(always)]
    pub fn reftop_vbgadj_6(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_6)
    }
    #[doc = "VBG-3.12%"]
    #[inline(always)]
    pub fn reftop_vbgadj_7(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_7)
    }
}
#[doc = "Field `REFTOP_VBGUP` reader - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
pub type REFTOP_VBGUP_R = crate::BitReader<bool>;
#[doc = "Field `REFTOP_VBGUP` writer - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
pub type REFTOP_VBGUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
#[doc = "Field `STOP_MODE_CONFIG` reader - Configure the analog behavior in stop mode."]
pub type STOP_MODE_CONFIG_R = crate::FieldReader<u8, STOP_MODE_CONFIG_A>;
#[doc = "Configure the analog behavior in stop mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP_MODE_CONFIG_A {
    #[doc = "0: SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "1: Analog regulators are ON."]
    STANDBY = 1,
    #[doc = "2: STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 2,
    #[doc = "3: STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 3,
}
impl From<STOP_MODE_CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_MODE_CONFIG_A) -> Self {
        variant as _
    }
}
impl STOP_MODE_CONFIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_MODE_CONFIG_A {
        match self.bits {
            0 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0,
            1 => STOP_MODE_CONFIG_A::STANDBY,
            2 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2,
            3 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_0`"]
    #[inline(always)]
    pub fn is_stop_mode_config_0(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_2`"]
    #[inline(always)]
    pub fn is_stop_mode_config_2(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_3`"]
    #[inline(always)]
    pub fn is_stop_mode_config_3(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3
    }
}
#[doc = "Field `STOP_MODE_CONFIG` writer - Configure the analog behavior in stop mode."]
pub type STOP_MODE_CONFIG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC0_SET_SPEC, u8, STOP_MODE_CONFIG_A, 2, O>;
impl<'a, const O: u8> STOP_MODE_CONFIG_W<'a, O> {
    #[doc = "SUSPEND (DSM)"]
    #[inline(always)]
    pub fn stop_mode_config_0(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0)
    }
    #[doc = "Analog regulators are ON."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STANDBY)
    }
    #[doc = "STOP (lower power)"]
    #[inline(always)]
    pub fn stop_mode_config_2(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2)
    }
    #[doc = "STOP (very lower power)"]
    #[inline(always)]
    pub fn stop_mode_config_3(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3)
    }
}
#[doc = "Field `DISCON_HIGH_SNVS` reader - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
pub type DISCON_HIGH_SNVS_R = crate::BitReader<DISCON_HIGH_SNVS_A>;
#[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCON_HIGH_SNVS_A {
    #[doc = "0: Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0,
    #[doc = "1: Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 1,
}
impl From<DISCON_HIGH_SNVS_A> for bool {
    #[inline(always)]
    fn from(variant: DISCON_HIGH_SNVS_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCON_HIGH_SNVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCON_HIGH_SNVS_A {
        match self.bits {
            false => DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0,
            true => DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_0`"]
    #[inline(always)]
    pub fn is_discon_high_snvs_0(&self) -> bool {
        *self == DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_1`"]
    #[inline(always)]
    pub fn is_discon_high_snvs_1(&self) -> bool {
        *self == DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1
    }
}
#[doc = "Field `DISCON_HIGH_SNVS` writer - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
pub type DISCON_HIGH_SNVS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC0_SET_SPEC, DISCON_HIGH_SNVS_A, O>;
impl<'a, const O: u8> DISCON_HIGH_SNVS_W<'a, O> {
    #[doc = "Turn on the switch"]
    #[inline(always)]
    pub fn discon_high_snvs_0(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0)
    }
    #[doc = "Turn off the switch"]
    #[inline(always)]
    pub fn discon_high_snvs_1(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1)
    }
}
#[doc = "Field `OSC_I` reader - This field determines the bias current in the 24MHz oscillator"]
pub type OSC_I_R = crate::FieldReader<u8, OSC_I_A>;
#[doc = "This field determines the bias current in the 24MHz oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSC_I_A {
    #[doc = "0: Nominal"]
    NOMINAL = 0,
    #[doc = "1: Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 1,
    #[doc = "2: Decrease current by 25.0%"]
    MINUS_25_PERCENT = 2,
    #[doc = "3: Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 3,
}
impl From<OSC_I_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_I_A) -> Self {
        variant as _
    }
}
impl OSC_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_I_A {
        match self.bits {
            0 => OSC_I_A::NOMINAL,
            1 => OSC_I_A::MINUS_12_5_PERCENT,
            2 => OSC_I_A::MINUS_25_PERCENT,
            3 => OSC_I_A::MINUS_37_5_PERCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMINAL`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == OSC_I_A::NOMINAL
    }
    #[doc = "Checks if the value of the field is `MINUS_12_5_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_12_5_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_12_5_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_25_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_25_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_25_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_37_5_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_37_5_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_37_5_PERCENT
    }
}
#[doc = "Field `OSC_I` writer - This field determines the bias current in the 24MHz oscillator"]
pub type OSC_I_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC0_SET_SPEC, u8, OSC_I_A, 2, O>;
impl<'a, const O: u8> OSC_I_W<'a, O> {
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn nominal(self) -> &'a mut W {
        self.variant(OSC_I_A::NOMINAL)
    }
    #[doc = "Decrease current by 12.5%"]
    #[inline(always)]
    pub fn minus_12_5_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_12_5_PERCENT)
    }
    #[doc = "Decrease current by 25.0%"]
    #[inline(always)]
    pub fn minus_25_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_25_PERCENT)
    }
    #[doc = "Decrease current by 37.5%"]
    #[inline(always)]
    pub fn minus_37_5_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_37_5_PERCENT)
    }
}
#[doc = "Field `OSC_XTALOK` reader - Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
pub type OSC_XTALOK_R = crate::BitReader<bool>;
#[doc = "Field `OSC_XTALOK_EN` reader - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
pub type OSC_XTALOK_EN_R = crate::BitReader<bool>;
#[doc = "Field `OSC_XTALOK_EN` writer - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
pub type OSC_XTALOK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
#[doc = "Field `CLKGATE_CTRL` reader - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
pub type CLKGATE_CTRL_R = crate::BitReader<CLKGATE_CTRL_A>;
#[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKGATE_CTRL_A {
    #[doc = "0: Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0,
    #[doc = "1: Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 1,
}
impl From<CLKGATE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGATE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKGATE_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGATE_CTRL_A {
        match self.bits {
            false => CLKGATE_CTRL_A::ALLOW_AUTO_GATE,
            true => CLKGATE_CTRL_A::NO_AUTO_GATE,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW_AUTO_GATE`"]
    #[inline(always)]
    pub fn is_allow_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRL_A::ALLOW_AUTO_GATE
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_GATE`"]
    #[inline(always)]
    pub fn is_no_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRL_A::NO_AUTO_GATE
    }
}
#[doc = "Field `CLKGATE_CTRL` writer - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
pub type CLKGATE_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC0_SET_SPEC, CLKGATE_CTRL_A, O>;
impl<'a, const O: u8> CLKGATE_CTRL_W<'a, O> {
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    #[inline(always)]
    pub fn allow_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRL_A::ALLOW_AUTO_GATE)
    }
    #[doc = "Prevent the logic from ever gating off the clock."]
    #[inline(always)]
    pub fn no_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRL_A::NO_AUTO_GATE)
    }
}
#[doc = "Field `CLKGATE_DELAY` reader - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
pub type CLKGATE_DELAY_R = crate::FieldReader<u8, CLKGATE_DELAY_A>;
#[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKGATE_DELAY_A {
    #[doc = "0: 0.5ms"]
    CLKGATE_DELAY_0 = 0,
    #[doc = "1: 1.0ms"]
    CLKGATE_DELAY_1 = 1,
    #[doc = "2: 2.0ms"]
    CLKGATE_DELAY_2 = 2,
    #[doc = "3: 3.0ms"]
    CLKGATE_DELAY_3 = 3,
    #[doc = "4: 4.0ms"]
    CLKGATE_DELAY_4 = 4,
    #[doc = "5: 5.0ms"]
    CLKGATE_DELAY_5 = 5,
    #[doc = "6: 6.0ms"]
    CLKGATE_DELAY_6 = 6,
    #[doc = "7: 7.0ms"]
    CLKGATE_DELAY_7 = 7,
}
impl From<CLKGATE_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKGATE_DELAY_A) -> Self {
        variant as _
    }
}
impl CLKGATE_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGATE_DELAY_A {
        match self.bits {
            0 => CLKGATE_DELAY_A::CLKGATE_DELAY_0,
            1 => CLKGATE_DELAY_A::CLKGATE_DELAY_1,
            2 => CLKGATE_DELAY_A::CLKGATE_DELAY_2,
            3 => CLKGATE_DELAY_A::CLKGATE_DELAY_3,
            4 => CLKGATE_DELAY_A::CLKGATE_DELAY_4,
            5 => CLKGATE_DELAY_A::CLKGATE_DELAY_5,
            6 => CLKGATE_DELAY_A::CLKGATE_DELAY_6,
            7 => CLKGATE_DELAY_A::CLKGATE_DELAY_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_0`"]
    #[inline(always)]
    pub fn is_clkgate_delay_0(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_0
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_1`"]
    #[inline(always)]
    pub fn is_clkgate_delay_1(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_1
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_2`"]
    #[inline(always)]
    pub fn is_clkgate_delay_2(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_2
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_3`"]
    #[inline(always)]
    pub fn is_clkgate_delay_3(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_3
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_4`"]
    #[inline(always)]
    pub fn is_clkgate_delay_4(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_4
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_5`"]
    #[inline(always)]
    pub fn is_clkgate_delay_5(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_5
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_6`"]
    #[inline(always)]
    pub fn is_clkgate_delay_6(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_6
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_7`"]
    #[inline(always)]
    pub fn is_clkgate_delay_7(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_7
    }
}
#[doc = "Field `CLKGATE_DELAY` writer - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
pub type CLKGATE_DELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC0_SET_SPEC, u8, CLKGATE_DELAY_A, 3, O>;
impl<'a, const O: u8> CLKGATE_DELAY_W<'a, O> {
    #[doc = "0.5ms"]
    #[inline(always)]
    pub fn clkgate_delay_0(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_0)
    }
    #[doc = "1.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_1(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_1)
    }
    #[doc = "2.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_2(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_2)
    }
    #[doc = "3.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_3(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_3)
    }
    #[doc = "4.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_4(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_4)
    }
    #[doc = "5.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_5(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_5)
    }
    #[doc = "6.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_6(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_6)
    }
    #[doc = "7.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_7(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_7)
    }
}
#[doc = "Field `RTC_XTAL_SOURCE` reader - This field indicates which chip source is being used for the rtc clock."]
pub type RTC_XTAL_SOURCE_R = crate::BitReader<RTC_XTAL_SOURCE_A>;
#[doc = "This field indicates which chip source is being used for the rtc clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_XTAL_SOURCE_A {
    #[doc = "0: Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0,
    #[doc = "1: RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 1,
}
impl From<RTC_XTAL_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_XTAL_SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_XTAL_SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_XTAL_SOURCE_A {
        match self.bits {
            false => RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_0,
            true => RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_0`"]
    #[inline(always)]
    pub fn is_rtc_xtal_source_0(&self) -> bool {
        *self == RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_1`"]
    #[inline(always)]
    pub fn is_rtc_xtal_source_1(&self) -> bool {
        *self == RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_1
    }
}
#[doc = "Field `XTAL_24M_PWD` reader - This field powers down the 24M crystal oscillator if set true."]
pub type XTAL_24M_PWD_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_24M_PWD` writer - This field powers down the 24M crystal oscillator if set true."]
pub type XTAL_24M_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC0_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub fn reftop_pwd(&self) -> REFTOP_PWD_R {
        REFTOP_PWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    pub fn reftop_pwdvbgup(&self) -> REFTOP_PWDVBGUP_R {
        REFTOP_PWDVBGUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    pub fn reftop_lowpower(&self) -> REFTOP_LOWPOWER_R {
        REFTOP_LOWPOWER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub fn reftop_selfbiasoff(&self) -> REFTOP_SELFBIASOFF_R {
        REFTOP_SELFBIASOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    pub fn reftop_vbgadj(&self) -> REFTOP_VBGADJ_R {
        REFTOP_VBGADJ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub fn reftop_vbgup(&self) -> REFTOP_VBGUP_R {
        REFTOP_VBGUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub fn stop_mode_config(&self) -> STOP_MODE_CONFIG_R {
        STOP_MODE_CONFIG_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub fn discon_high_snvs(&self) -> DISCON_HIGH_SNVS_R {
        DISCON_HIGH_SNVS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub fn osc_i(&self) -> OSC_I_R {
        OSC_I_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub fn osc_xtalok(&self) -> OSC_XTALOK_R {
        OSC_XTALOK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub fn osc_xtalok_en(&self) -> OSC_XTALOK_EN_R {
        OSC_XTALOK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub fn clkgate_ctrl(&self) -> CLKGATE_CTRL_R {
        CLKGATE_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub fn clkgate_delay(&self) -> CLKGATE_DELAY_R {
        CLKGATE_DELAY_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub fn rtc_xtal_source(&self) -> RTC_XTAL_SOURCE_R {
        RTC_XTAL_SOURCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub fn xtal_24m_pwd(&self) -> XTAL_24M_PWD_R {
        XTAL_24M_PWD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    #[must_use]
    pub fn reftop_pwd(&mut self) -> REFTOP_PWD_W<0> {
        REFTOP_PWD_W::new(self)
    }
    #[doc = "Bit 1 - Control bit to power down the VBG-up detection circuitry in the analog bandgap."]
    #[inline(always)]
    #[must_use]
    pub fn reftop_pwdvbgup(&mut self) -> REFTOP_PWDVBGUP_W<1> {
        REFTOP_PWDVBGUP_W::new(self)
    }
    #[doc = "Bit 2 - Control bit to enable the low-power mode in the analog bandgap."]
    #[inline(always)]
    #[must_use]
    pub fn reftop_lowpower(&mut self) -> REFTOP_LOWPOWER_W<2> {
        REFTOP_LOWPOWER_W::new(self)
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    #[must_use]
    pub fn reftop_selfbiasoff(&mut self) -> REFTOP_SELFBIASOFF_W<3> {
        REFTOP_SELFBIASOFF_W::new(self)
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn reftop_vbgadj(&mut self) -> REFTOP_VBGADJ_W<4> {
        REFTOP_VBGADJ_W::new(self)
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    #[must_use]
    pub fn reftop_vbgup(&mut self) -> REFTOP_VBGUP_W<7> {
        REFTOP_VBGUP_W::new(self)
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn stop_mode_config(&mut self) -> STOP_MODE_CONFIG_W<10> {
        STOP_MODE_CONFIG_W::new(self)
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    #[must_use]
    pub fn discon_high_snvs(&mut self) -> DISCON_HIGH_SNVS_W<12> {
        DISCON_HIGH_SNVS_W::new(self)
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn osc_i(&mut self) -> OSC_I_W<13> {
        OSC_I_W::new(self)
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    #[must_use]
    pub fn osc_xtalok_en(&mut self) -> OSC_XTALOK_EN_W<16> {
        OSC_XTALOK_EN_W::new(self)
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate_ctrl(&mut self) -> CLKGATE_CTRL_W<25> {
        CLKGATE_CTRL_W::new(self)
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate_delay(&mut self) -> CLKGATE_DELAY_W<26> {
        CLKGATE_DELAY_W::new(self)
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    #[must_use]
    pub fn xtal_24m_pwd(&mut self) -> XTAL_24M_PWD_W<30> {
        XTAL_24M_PWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc0_set](index.html) module"]
pub struct MISC0_SET_SPEC;
impl crate::RegisterSpec for MISC0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc0_set::R](R) reader structure"]
impl crate::Readable for MISC0_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc0_set::W](W) writer structure"]
impl crate::Writable for MISC0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC0_SET to value 0x0400_0000"]
impl crate::Resettable for MISC0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
