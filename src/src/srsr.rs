#[doc = "Register `SRSR` reader"]
pub struct R(crate::R<SRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSR` writer"]
pub struct W(crate::W<SRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSR_SPEC>;
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
impl From<crate::W<SRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ipp_reset_b` reader - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
pub type IPP_RESET_B_R = crate::BitReader<IPP_RESET_B_A>;
#[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPP_RESET_B_A {
    #[doc = "0: Reset is not a result of ipp_reset_b pin."]
    IPP_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of ipp_reset_b pin."]
    IPP_RESET_B_1 = 1,
}
impl From<IPP_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: IPP_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
impl IPP_RESET_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPP_RESET_B_A {
        match self.bits {
            false => IPP_RESET_B_A::IPP_RESET_B_0,
            true => IPP_RESET_B_A::IPP_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_0`"]
    #[inline(always)]
    pub fn is_ipp_reset_b_0(&self) -> bool {
        *self == IPP_RESET_B_A::IPP_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_1`"]
    #[inline(always)]
    pub fn is_ipp_reset_b_1(&self) -> bool {
        *self == IPP_RESET_B_A::IPP_RESET_B_1
    }
}
#[doc = "Field `ipp_reset_b` writer - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
pub type IPP_RESET_B_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, IPP_RESET_B_A, O>;
impl<'a, const O: u8> IPP_RESET_B_W<'a, O> {
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    #[inline(always)]
    pub fn ipp_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_RESET_B_A::IPP_RESET_B_0)
    }
    #[doc = "Reset is a result of ipp_reset_b pin."]
    #[inline(always)]
    pub fn ipp_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_RESET_B_A::IPP_RESET_B_1)
    }
}
#[doc = "Field `lockup` reader - Indicates a reset has been caused by CPU lockup."]
pub type LOCKUP_R = crate::BitReader<LOCKUP_A>;
#[doc = "Indicates a reset has been caused by CPU lockup.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_A {
    #[doc = "0: Reset is not a result of the mentioned case."]
    LOCKUP_0 = 0,
    #[doc = "1: Reset is a result of the mentioned case."]
    LOCKUP_1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::LOCKUP_0,
            true => LOCKUP_A::LOCKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_0`"]
    #[inline(always)]
    pub fn is_lockup_0(&self) -> bool {
        *self == LOCKUP_A::LOCKUP_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_1`"]
    #[inline(always)]
    pub fn is_lockup_1(&self) -> bool {
        *self == LOCKUP_A::LOCKUP_1
    }
}
#[doc = "Field `lockup` writer - Indicates a reset has been caused by CPU lockup."]
pub type LOCKUP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, LOCKUP_A, O>;
impl<'a, const O: u8> LOCKUP_W<'a, O> {
    #[doc = "Reset is not a result of the mentioned case."]
    #[inline(always)]
    pub fn lockup_0(self) -> &'a mut W {
        self.variant(LOCKUP_A::LOCKUP_0)
    }
    #[doc = "Reset is a result of the mentioned case."]
    #[inline(always)]
    pub fn lockup_1(self) -> &'a mut W {
        self.variant(LOCKUP_A::LOCKUP_1)
    }
}
#[doc = "Field `csu_reset_b` reader - Indicates whether the reset was the result of the csu_reset_b input."]
pub type CSU_RESET_B_R = crate::BitReader<CSU_RESET_B_A>;
#[doc = "Indicates whether the reset was the result of the csu_reset_b input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSU_RESET_B_A {
    #[doc = "0: Reset is not a result of the csu_reset_b event."]
    CSU_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of the csu_reset_b event."]
    CSU_RESET_B_1 = 1,
}
impl From<CSU_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: CSU_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
impl CSU_RESET_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSU_RESET_B_A {
        match self.bits {
            false => CSU_RESET_B_A::CSU_RESET_B_0,
            true => CSU_RESET_B_A::CSU_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_0`"]
    #[inline(always)]
    pub fn is_csu_reset_b_0(&self) -> bool {
        *self == CSU_RESET_B_A::CSU_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_1`"]
    #[inline(always)]
    pub fn is_csu_reset_b_1(&self) -> bool {
        *self == CSU_RESET_B_A::CSU_RESET_B_1
    }
}
#[doc = "Field `csu_reset_b` writer - Indicates whether the reset was the result of the csu_reset_b input."]
pub type CSU_RESET_B_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, CSU_RESET_B_A, O>;
impl<'a, const O: u8> CSU_RESET_B_W<'a, O> {
    #[doc = "Reset is not a result of the csu_reset_b event."]
    #[inline(always)]
    pub fn csu_reset_b_0(self) -> &'a mut W {
        self.variant(CSU_RESET_B_A::CSU_RESET_B_0)
    }
    #[doc = "Reset is a result of the csu_reset_b event."]
    #[inline(always)]
    pub fn csu_reset_b_1(self) -> &'a mut W {
        self.variant(CSU_RESET_B_A::CSU_RESET_B_1)
    }
}
#[doc = "Field `ipp_user_reset_b` reader - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
pub type IPP_USER_RESET_B_R = crate::BitReader<IPP_USER_RESET_B_A>;
#[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPP_USER_RESET_B_A {
    #[doc = "0: Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_1 = 1,
}
impl From<IPP_USER_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: IPP_USER_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
impl IPP_USER_RESET_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPP_USER_RESET_B_A {
        match self.bits {
            false => IPP_USER_RESET_B_A::IPP_USER_RESET_B_0,
            true => IPP_USER_RESET_B_A::IPP_USER_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_0`"]
    #[inline(always)]
    pub fn is_ipp_user_reset_b_0(&self) -> bool {
        *self == IPP_USER_RESET_B_A::IPP_USER_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_1`"]
    #[inline(always)]
    pub fn is_ipp_user_reset_b_1(&self) -> bool {
        *self == IPP_USER_RESET_B_A::IPP_USER_RESET_B_1
    }
}
#[doc = "Field `ipp_user_reset_b` writer - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
pub type IPP_USER_RESET_B_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, SRSR_SPEC, IPP_USER_RESET_B_A, O>;
impl<'a, const O: u8> IPP_USER_RESET_B_W<'a, O> {
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline(always)]
    pub fn ipp_user_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_B_A::IPP_USER_RESET_B_0)
    }
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline(always)]
    pub fn ipp_user_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_B_A::IPP_USER_RESET_B_1)
    }
}
#[doc = "Field `wdog_rst_b` reader - IC Watchdog Time-out reset"]
pub type WDOG_RST_B_R = crate::BitReader<WDOG_RST_B_A>;
#[doc = "IC Watchdog Time-out reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG_RST_B_A {
    #[doc = "0: Reset is not a result of the watchdog time-out event."]
    WDOG_RST_B_0 = 0,
    #[doc = "1: Reset is a result of the watchdog time-out event."]
    WDOG_RST_B_1 = 1,
}
impl From<WDOG_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG_RST_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_RST_B_A {
        match self.bits {
            false => WDOG_RST_B_A::WDOG_RST_B_0,
            true => WDOG_RST_B_A::WDOG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_0`"]
    #[inline(always)]
    pub fn is_wdog_rst_b_0(&self) -> bool {
        *self == WDOG_RST_B_A::WDOG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_1`"]
    #[inline(always)]
    pub fn is_wdog_rst_b_1(&self) -> bool {
        *self == WDOG_RST_B_A::WDOG_RST_B_1
    }
}
#[doc = "Field `wdog_rst_b` writer - IC Watchdog Time-out reset"]
pub type WDOG_RST_B_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, WDOG_RST_B_A, O>;
impl<'a, const O: u8> WDOG_RST_B_W<'a, O> {
    #[doc = "Reset is not a result of the watchdog time-out event."]
    #[inline(always)]
    pub fn wdog_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG_RST_B_A::WDOG_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog time-out event."]
    #[inline(always)]
    pub fn wdog_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG_RST_B_A::WDOG_RST_B_1)
    }
}
#[doc = "Field `jtag_rst_b` reader - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
pub type JTAG_RST_B_R = crate::BitReader<JTAG_RST_B_A>;
#[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAG_RST_B_A {
    #[doc = "0: Reset is not a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_0 = 0,
    #[doc = "1: Reset is a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_1 = 1,
}
impl From<JTAG_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
impl JTAG_RST_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_RST_B_A {
        match self.bits {
            false => JTAG_RST_B_A::JTAG_RST_B_0,
            true => JTAG_RST_B_A::JTAG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_0`"]
    #[inline(always)]
    pub fn is_jtag_rst_b_0(&self) -> bool {
        *self == JTAG_RST_B_A::JTAG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_1`"]
    #[inline(always)]
    pub fn is_jtag_rst_b_1(&self) -> bool {
        *self == JTAG_RST_B_A::JTAG_RST_B_1
    }
}
#[doc = "Field `jtag_rst_b` writer - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
pub type JTAG_RST_B_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, JTAG_RST_B_A, O>;
impl<'a, const O: u8> JTAG_RST_B_W<'a, O> {
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b_0(self) -> &'a mut W {
        self.variant(JTAG_RST_B_A::JTAG_RST_B_0)
    }
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b_1(self) -> &'a mut W {
        self.variant(JTAG_RST_B_A::JTAG_RST_B_1)
    }
}
#[doc = "Field `jtag_sw_rst` reader - JTAG software reset"]
pub type JTAG_SW_RST_R = crate::BitReader<JTAG_SW_RST_A>;
#[doc = "JTAG software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAG_SW_RST_A {
    #[doc = "0: Reset is not a result of the mentioned case."]
    JTAG_SW_RST_0 = 0,
    #[doc = "1: Reset is a result of the mentioned case."]
    JTAG_SW_RST_1 = 1,
}
impl From<JTAG_SW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_SW_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl JTAG_SW_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_SW_RST_A {
        match self.bits {
            false => JTAG_SW_RST_A::JTAG_SW_RST_0,
            true => JTAG_SW_RST_A::JTAG_SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_0`"]
    #[inline(always)]
    pub fn is_jtag_sw_rst_0(&self) -> bool {
        *self == JTAG_SW_RST_A::JTAG_SW_RST_0
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_1`"]
    #[inline(always)]
    pub fn is_jtag_sw_rst_1(&self) -> bool {
        *self == JTAG_SW_RST_A::JTAG_SW_RST_1
    }
}
#[doc = "Field `jtag_sw_rst` writer - JTAG software reset"]
pub type JTAG_SW_RST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, JTAG_SW_RST_A, O>;
impl<'a, const O: u8> JTAG_SW_RST_W<'a, O> {
    #[doc = "Reset is not a result of the mentioned case."]
    #[inline(always)]
    pub fn jtag_sw_rst_0(self) -> &'a mut W {
        self.variant(JTAG_SW_RST_A::JTAG_SW_RST_0)
    }
    #[doc = "Reset is a result of the mentioned case."]
    #[inline(always)]
    pub fn jtag_sw_rst_1(self) -> &'a mut W {
        self.variant(JTAG_SW_RST_A::JTAG_SW_RST_1)
    }
}
#[doc = "Field `wdog3_rst_b` reader - IC Watchdog3 Time-out reset"]
pub type WDOG3_RST_B_R = crate::BitReader<WDOG3_RST_B_A>;
#[doc = "IC Watchdog3 Time-out reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG3_RST_B_A {
    #[doc = "0: Reset is not a result of the watchdog3 time-out event."]
    WDOG3_RST_B_0 = 0,
    #[doc = "1: Reset is a result of the watchdog3 time-out event."]
    WDOG3_RST_B_1 = 1,
}
impl From<WDOG3_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG3_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG3_RST_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG3_RST_B_A {
        match self.bits {
            false => WDOG3_RST_B_A::WDOG3_RST_B_0,
            true => WDOG3_RST_B_A::WDOG3_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_0`"]
    #[inline(always)]
    pub fn is_wdog3_rst_b_0(&self) -> bool {
        *self == WDOG3_RST_B_A::WDOG3_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_1`"]
    #[inline(always)]
    pub fn is_wdog3_rst_b_1(&self) -> bool {
        *self == WDOG3_RST_B_A::WDOG3_RST_B_1
    }
}
#[doc = "Field `wdog3_rst_b` writer - IC Watchdog3 Time-out reset"]
pub type WDOG3_RST_B_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SRSR_SPEC, WDOG3_RST_B_A, O>;
impl<'a, const O: u8> WDOG3_RST_B_W<'a, O> {
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    #[inline(always)]
    pub fn wdog3_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG3_RST_B_A::WDOG3_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    #[inline(always)]
    pub fn wdog3_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG3_RST_B_A::WDOG3_RST_B_1)
    }
}
#[doc = "Field `tempsense_rst_b` reader - Temper Sensor software reset"]
pub type TEMPSENSE_RST_B_R = crate::BitReader<TEMPSENSE_RST_B_A>;
#[doc = "Temper Sensor software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPSENSE_RST_B_A {
    #[doc = "0: Reset is not a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_0 = 0,
    #[doc = "1: Reset is a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_1 = 1,
}
impl From<TEMPSENSE_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPSENSE_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
impl TEMPSENSE_RST_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPSENSE_RST_B_A {
        match self.bits {
            false => TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0,
            true => TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_0`"]
    #[inline(always)]
    pub fn is_tempsense_rst_b_0(&self) -> bool {
        *self == TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_1`"]
    #[inline(always)]
    pub fn is_tempsense_rst_b_1(&self) -> bool {
        *self == TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1
    }
}
#[doc = "Field `tempsense_rst_b` writer - Temper Sensor software reset"]
pub type TEMPSENSE_RST_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SRSR_SPEC, TEMPSENSE_RST_B_A, O>;
impl<'a, const O: u8> TEMPSENSE_RST_B_W<'a, O> {
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    #[inline(always)]
    pub fn tempsense_rst_b_0(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0)
    }
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    #[inline(always)]
    pub fn tempsense_rst_b_1(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline(always)]
    pub fn ipp_reset_b(&self) -> IPP_RESET_B_R {
        IPP_RESET_B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup."]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline(always)]
    pub fn csu_reset_b(&self) -> CSU_RESET_B_R {
        CSU_RESET_B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline(always)]
    pub fn ipp_user_reset_b(&self) -> IPP_USER_RESET_B_R {
        IPP_USER_RESET_B_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline(always)]
    pub fn wdog_rst_b(&self) -> WDOG_RST_B_R {
        WDOG_RST_B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b(&self) -> JTAG_RST_B_R {
        JTAG_RST_B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JTAG software reset"]
    #[inline(always)]
    pub fn jtag_sw_rst(&self) -> JTAG_SW_RST_R {
        JTAG_SW_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline(always)]
    pub fn wdog3_rst_b(&self) -> WDOG3_RST_B_R {
        WDOG3_RST_B_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline(always)]
    pub fn tempsense_rst_b(&self) -> TEMPSENSE_RST_B_R {
        TEMPSENSE_RST_B_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline(always)]
    #[must_use]
    pub fn ipp_reset_b(&mut self) -> IPP_RESET_B_W<0> {
        IPP_RESET_B_W::new(self)
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup."]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LOCKUP_W<1> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline(always)]
    #[must_use]
    pub fn csu_reset_b(&mut self) -> CSU_RESET_B_W<2> {
        CSU_RESET_B_W::new(self)
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline(always)]
    #[must_use]
    pub fn ipp_user_reset_b(&mut self) -> IPP_USER_RESET_B_W<3> {
        IPP_USER_RESET_B_W::new(self)
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_rst_b(&mut self) -> WDOG_RST_B_W<4> {
        WDOG_RST_B_W::new(self)
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_rst_b(&mut self) -> JTAG_RST_B_W<5> {
        JTAG_RST_B_W::new(self)
    }
    #[doc = "Bit 6 - JTAG software reset"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_sw_rst(&mut self) -> JTAG_SW_RST_W<6> {
        JTAG_SW_RST_W::new(self)
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdog3_rst_b(&mut self) -> WDOG3_RST_B_W<7> {
        WDOG3_RST_B_W::new(self)
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tempsense_rst_b(&mut self) -> TEMPSENSE_RST_B_W<8> {
        TEMPSENSE_RST_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRC Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srsr](index.html) module"]
pub struct SRSR_SPEC;
impl crate::RegisterSpec for SRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srsr::R](R) reader structure"]
impl crate::Readable for SRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srsr::W](W) writer structure"]
impl crate::Writable for SRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets SRSR to value 0x01"]
impl crate::Resettable for SRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
