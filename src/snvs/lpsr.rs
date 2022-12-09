#[doc = "Register `LPSR` reader"]
pub struct R(crate::R<LPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSR` writer"]
pub struct W(crate::W<LPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSR_SPEC>;
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
impl From<crate::W<LPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTA` reader - LP Time Alarm"]
pub type LPTA_R = crate::BitReader<LPTA_A>;
#[doc = "LP Time Alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTA_A {
    #[doc = "0: No time alarm interrupt occurred."]
    NOREPORT = 0,
    #[doc = "1: A time alarm interrupt occurred."]
    REPORTED = 1,
}
impl From<LPTA_A> for bool {
    #[inline(always)]
    fn from(variant: LPTA_A) -> Self {
        variant as u8 != 0
    }
}
impl LPTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTA_A {
        match self.bits {
            false => LPTA_A::NOREPORT,
            true => LPTA_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == LPTA_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == LPTA_A::REPORTED
    }
}
#[doc = "Field `LPTA` writer - LP Time Alarm"]
pub type LPTA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, LPTA_A, O>;
impl<'a, const O: u8> LPTA_W<'a, O> {
    #[doc = "No time alarm interrupt occurred."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(LPTA_A::NOREPORT)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(LPTA_A::REPORTED)
    }
}
#[doc = "Field `SRTCR` reader - Secure Real Time Counter Rollover"]
pub type SRTCR_R = crate::BitReader<SRTCR_A>;
#[doc = "Secure Real Time Counter Rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTCR_A {
    #[doc = "0: SRTC has not reached its maximum value."]
    NOREPORT = 0,
    #[doc = "1: SRTC has reached its maximum value."]
    REPORTED = 1,
}
impl From<SRTCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRTCR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTCR_A {
        match self.bits {
            false => SRTCR_A::NOREPORT,
            true => SRTCR_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SRTCR_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SRTCR_A::REPORTED
    }
}
#[doc = "Field `SRTCR` writer - Secure Real Time Counter Rollover"]
pub type SRTCR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, SRTCR_A, O>;
impl<'a, const O: u8> SRTCR_W<'a, O> {
    #[doc = "SRTC has not reached its maximum value."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SRTCR_A::NOREPORT)
    }
    #[doc = "SRTC has reached its maximum value."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SRTCR_A::REPORTED)
    }
}
#[doc = "Field `MCR` reader - Monotonic Counter Rollover"]
pub type MCR_R = crate::BitReader<MCR_A>;
#[doc = "Monotonic Counter Rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCR_A {
    #[doc = "0: MC has not reached its maximum value."]
    NOREPORT = 0,
    #[doc = "1: MC has reached its maximum value."]
    REPORTED = 1,
}
impl From<MCR_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_A) -> Self {
        variant as u8 != 0
    }
}
impl MCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCR_A {
        match self.bits {
            false => MCR_A::NOREPORT,
            true => MCR_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == MCR_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == MCR_A::REPORTED
    }
}
#[doc = "Field `MCR` writer - Monotonic Counter Rollover"]
pub type MCR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, MCR_A, O>;
impl<'a, const O: u8> MCR_W<'a, O> {
    #[doc = "MC has not reached its maximum value."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(MCR_A::NOREPORT)
    }
    #[doc = "MC has reached its maximum value."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(MCR_A::REPORTED)
    }
}
#[doc = "Field `LVD` reader - Digital Low Voltage Event Detected"]
pub type LVD_R = crate::BitReader<LVD_A>;
#[doc = "Digital Low Voltage Event Detected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD_A {
    #[doc = "0: No low voltage event detected."]
    NOLOWVOLT = 0,
    #[doc = "1: Low voltage event is detected."]
    LOWVOLTDETECTED = 1,
}
impl From<LVD_A> for bool {
    #[inline(always)]
    fn from(variant: LVD_A) -> Self {
        variant as u8 != 0
    }
}
impl LVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVD_A {
        match self.bits {
            false => LVD_A::NOLOWVOLT,
            true => LVD_A::LOWVOLTDETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOLOWVOLT`"]
    #[inline(always)]
    pub fn is_nolowvolt(&self) -> bool {
        *self == LVD_A::NOLOWVOLT
    }
    #[doc = "Checks if the value of the field is `LOWVOLTDETECTED`"]
    #[inline(always)]
    pub fn is_lowvoltdetected(&self) -> bool {
        *self == LVD_A::LOWVOLTDETECTED
    }
}
#[doc = "Field `LVD` writer - Digital Low Voltage Event Detected"]
pub type LVD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, LVD_A, O>;
impl<'a, const O: u8> LVD_W<'a, O> {
    #[doc = "No low voltage event detected."]
    #[inline(always)]
    pub fn nolowvolt(self) -> &'a mut W {
        self.variant(LVD_A::NOLOWVOLT)
    }
    #[doc = "Low voltage event is detected."]
    #[inline(always)]
    pub fn lowvoltdetected(self) -> &'a mut W {
        self.variant(LVD_A::LOWVOLTDETECTED)
    }
}
#[doc = "Field `ESVD` reader - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
pub type ESVD_R = crate::BitReader<ESVD_A>;
#[doc = "External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESVD_A {
    #[doc = "0: No external security violation."]
    NOREPORT = 0,
    #[doc = "1: External security violation is detected."]
    REPORTED = 1,
}
impl From<ESVD_A> for bool {
    #[inline(always)]
    fn from(variant: ESVD_A) -> Self {
        variant as u8 != 0
    }
}
impl ESVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESVD_A {
        match self.bits {
            false => ESVD_A::NOREPORT,
            true => ESVD_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == ESVD_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == ESVD_A::REPORTED
    }
}
#[doc = "Field `ESVD` writer - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
pub type ESVD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, ESVD_A, O>;
impl<'a, const O: u8> ESVD_W<'a, O> {
    #[doc = "No external security violation."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(ESVD_A::NOREPORT)
    }
    #[doc = "External security violation is detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(ESVD_A::REPORTED)
    }
}
#[doc = "Field `EO` reader - Emergency Off This bit is set when a power off is requested."]
pub type EO_R = crate::BitReader<EO_A>;
#[doc = "Emergency Off This bit is set when a power off is requested.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EO_A {
    #[doc = "0: Emergency off was not detected."]
    NOREPORT = 0,
    #[doc = "1: Emergency off was detected."]
    REPORTED = 1,
}
impl From<EO_A> for bool {
    #[inline(always)]
    fn from(variant: EO_A) -> Self {
        variant as u8 != 0
    }
}
impl EO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EO_A {
        match self.bits {
            false => EO_A::NOREPORT,
            true => EO_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == EO_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == EO_A::REPORTED
    }
}
#[doc = "Field `EO` writer - Emergency Off This bit is set when a power off is requested."]
pub type EO_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, EO_A, O>;
impl<'a, const O: u8> EO_W<'a, O> {
    #[doc = "Emergency off was not detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(EO_A::NOREPORT)
    }
    #[doc = "Emergency off was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(EO_A::REPORTED)
    }
}
#[doc = "Field `SPOF` reader - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
pub type SPOF_R = crate::BitReader<SPOF_A>;
#[doc = "Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOF_A {
    #[doc = "0: Set Power Off was not detected."]
    NOREPORT = 0,
    #[doc = "1: Set Power Off was detected."]
    REPORTED = 1,
}
impl From<SPOF_A> for bool {
    #[inline(always)]
    fn from(variant: SPOF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOF_A {
        match self.bits {
            false => SPOF_A::NOREPORT,
            true => SPOF_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SPOF_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SPOF_A::REPORTED
    }
}
#[doc = "Field `SPOF` writer - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
pub type SPOF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, SPOF_A, O>;
impl<'a, const O: u8> SPOF_W<'a, O> {
    #[doc = "Set Power Off was not detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SPOF_A::NOREPORT)
    }
    #[doc = "Set Power Off was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SPOF_A::REPORTED)
    }
}
#[doc = "Field `SPON` reader - Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
pub type SPON_R = crate::BitReader<SPON_A>;
#[doc = "Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPON_A {
    #[doc = "0: Set Power On Interrupt was not detected."]
    NOREPORT = 0,
    #[doc = "1: Set Power On Interrupt was detected."]
    REPORTED = 1,
}
impl From<SPON_A> for bool {
    #[inline(always)]
    fn from(variant: SPON_A) -> Self {
        variant as u8 != 0
    }
}
impl SPON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPON_A {
        match self.bits {
            false => SPON_A::NOREPORT,
            true => SPON_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SPON_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SPON_A::REPORTED
    }
}
#[doc = "Field `SPON` writer - Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
pub type SPON_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, LPSR_SPEC, SPON_A, O>;
impl<'a, const O: u8> SPON_W<'a, O> {
    #[doc = "Set Power On Interrupt was not detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SPON_A::NOREPORT)
    }
    #[doc = "Set Power On Interrupt was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SPON_A::REPORTED)
    }
}
#[doc = "Field `LPNS` reader - LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
pub type LPNS_R = crate::BitReader<LPNS_A>;
#[doc = "LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPNS_A {
    #[doc = "0: LP section was not programmed in the non-secure state."]
    NOT_PRGRMD_IN_NON_SECURE_STATE = 0,
    #[doc = "1: LP section was programmed in the non-secure state."]
    WAS_PRGRMD_IN_NON_SECURE_STATE = 1,
}
impl From<LPNS_A> for bool {
    #[inline(always)]
    fn from(variant: LPNS_A) -> Self {
        variant as u8 != 0
    }
}
impl LPNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPNS_A {
        match self.bits {
            false => LPNS_A::NOT_PRGRMD_IN_NON_SECURE_STATE,
            true => LPNS_A::WAS_PRGRMD_IN_NON_SECURE_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRGRMD_IN_NON_SECURE_STATE`"]
    #[inline(always)]
    pub fn is_not_prgrmd_in_non_secure_state(&self) -> bool {
        *self == LPNS_A::NOT_PRGRMD_IN_NON_SECURE_STATE
    }
    #[doc = "Checks if the value of the field is `WAS_PRGRMD_IN_NON_SECURE_STATE`"]
    #[inline(always)]
    pub fn is_was_prgrmd_in_non_secure_state(&self) -> bool {
        *self == LPNS_A::WAS_PRGRMD_IN_NON_SECURE_STATE
    }
}
#[doc = "Field `LPS` reader - LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
pub type LPS_R = crate::BitReader<LPS_A>;
#[doc = "LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPS_A {
    #[doc = "0: LP section was not programmed in secure or trusted state."]
    NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE = 0,
    #[doc = "1: LP section was programmed in secure or trusted state."]
    WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE = 1,
}
impl From<LPS_A> for bool {
    #[inline(always)]
    fn from(variant: LPS_A) -> Self {
        variant as u8 != 0
    }
}
impl LPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPS_A {
        match self.bits {
            false => LPS_A::NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE,
            true => LPS_A::WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE`"]
    #[inline(always)]
    pub fn is_not_prgrmd_in_secure_or_trusted_state(&self) -> bool {
        *self == LPS_A::NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE
    }
    #[doc = "Checks if the value of the field is `WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE`"]
    #[inline(always)]
    pub fn is_was_prgrmd_in_secure_or_trusted_state(&self) -> bool {
        *self == LPS_A::WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE
    }
}
impl R {
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline(always)]
    pub fn lpta(&self) -> LPTA_R {
        LPTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline(always)]
    pub fn srtcr(&self) -> SRTCR_R {
        SRTCR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Low Voltage Event Detected"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    pub fn esvd(&self) -> ESVD_R {
        ESVD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    pub fn eo(&self) -> EO_R {
        EO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub fn spof(&self) -> SPOF_R {
        SPOF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    pub fn spon(&self) -> SPON_R {
        SPON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - LP Section is Non-Secured Indicates that LP section was provisioned/programmed in the non-secure state"]
    #[inline(always)]
    pub fn lpns(&self) -> LPNS_R {
        LPNS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LP Section is Secured Indicates that the LP section is provisioned/programmed in the secure or trusted state"]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LP Time Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn lpta(&mut self) -> LPTA_W<0> {
        LPTA_W::new(self)
    }
    #[doc = "Bit 1 - Secure Real Time Counter Rollover"]
    #[inline(always)]
    #[must_use]
    pub fn srtcr(&mut self) -> SRTCR_W<1> {
        SRTCR_W::new(self)
    }
    #[doc = "Bit 2 - Monotonic Counter Rollover"]
    #[inline(always)]
    #[must_use]
    pub fn mcr(&mut self) -> MCR_W<2> {
        MCR_W::new(self)
    }
    #[doc = "Bit 3 - Digital Low Voltage Event Detected"]
    #[inline(always)]
    #[must_use]
    pub fn lvd(&mut self) -> LVD_W<3> {
        LVD_W::new(self)
    }
    #[doc = "Bit 16 - External Security Violation Detected Indicates that a security violation is detected on one of the HP security violation ports"]
    #[inline(always)]
    #[must_use]
    pub fn esvd(&mut self) -> ESVD_W<16> {
        ESVD_W::new(self)
    }
    #[doc = "Bit 17 - Emergency Off This bit is set when a power off is requested."]
    #[inline(always)]
    #[must_use]
    pub fn eo(&mut self) -> EO_W<17> {
        EO_W::new(self)
    }
    #[doc = "Bit 18 - Set Power Off The SPO bit is set when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    #[must_use]
    pub fn spof(&mut self) -> SPOF_W<18> {
        SPOF_W::new(self)
    }
    #[doc = "Bit 19 - Set Power On The SPON bit is set when the set_pwr_on_irq interrupt is triggered, which happens when the power button is pressed longer than the configured debounce time"]
    #[inline(always)]
    #[must_use]
    pub fn spon(&mut self) -> SPON_W<19> {
        SPON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsr](index.html) module"]
pub struct LPSR_SPEC;
impl crate::RegisterSpec for LPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsr::R](R) reader structure"]
impl crate::Readable for LPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsr::W](W) writer structure"]
impl crate::Writable for LPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x000f_000f;
}
#[doc = "`reset()` method sets LPSR to value 0x08"]
impl crate::Resettable for LPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
