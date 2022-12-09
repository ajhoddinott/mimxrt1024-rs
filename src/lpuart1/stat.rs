#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA2F` reader - Match 2 Flag"]
pub type MA2F_R = crate::BitReader<MA2F_A>;
#[doc = "Match 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MA2F_A {
    #[doc = "0: Received data is not equal to MA2"]
    NOMATCH = 0,
    #[doc = "1: Received data is equal to MA2"]
    MATCH = 1,
}
impl From<MA2F_A> for bool {
    #[inline(always)]
    fn from(variant: MA2F_A) -> Self {
        variant as u8 != 0
    }
}
impl MA2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA2F_A {
        match self.bits {
            false => MA2F_A::NOMATCH,
            true => MA2F_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline(always)]
    pub fn is_nomatch(&self) -> bool {
        *self == MA2F_A::NOMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == MA2F_A::MATCH
    }
}
#[doc = "Field `MA2F` writer - Match 2 Flag"]
pub type MA2F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, MA2F_A, O>;
impl<'a, const O: u8> MA2F_W<'a, O> {
    #[doc = "Received data is not equal to MA2"]
    #[inline(always)]
    pub fn nomatch(self) -> &'a mut W {
        self.variant(MA2F_A::NOMATCH)
    }
    #[doc = "Received data is equal to MA2"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(MA2F_A::MATCH)
    }
}
#[doc = "Field `MA1F` reader - Match 1 Flag"]
pub type MA1F_R = crate::BitReader<MA1F_A>;
#[doc = "Match 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MA1F_A {
    #[doc = "0: Received data is not equal to MA1"]
    NOMATCH = 0,
    #[doc = "1: Received data is equal to MA1"]
    MATCH = 1,
}
impl From<MA1F_A> for bool {
    #[inline(always)]
    fn from(variant: MA1F_A) -> Self {
        variant as u8 != 0
    }
}
impl MA1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA1F_A {
        match self.bits {
            false => MA1F_A::NOMATCH,
            true => MA1F_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline(always)]
    pub fn is_nomatch(&self) -> bool {
        *self == MA1F_A::NOMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == MA1F_A::MATCH
    }
}
#[doc = "Field `MA1F` writer - Match 1 Flag"]
pub type MA1F_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, MA1F_A, O>;
impl<'a, const O: u8> MA1F_W<'a, O> {
    #[doc = "Received data is not equal to MA1"]
    #[inline(always)]
    pub fn nomatch(self) -> &'a mut W {
        self.variant(MA1F_A::NOMATCH)
    }
    #[doc = "Received data is equal to MA1"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(MA1F_A::MATCH)
    }
}
#[doc = "Field `PF` reader - Parity Error Flag"]
pub type PF_R = crate::BitReader<PF_A>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PF_A {
    #[doc = "0: No parity error."]
    NOPARITY = 0,
    #[doc = "1: Parity error."]
    PARITY = 1,
}
impl From<PF_A> for bool {
    #[inline(always)]
    fn from(variant: PF_A) -> Self {
        variant as u8 != 0
    }
}
impl PF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_A {
        match self.bits {
            false => PF_A::NOPARITY,
            true => PF_A::PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NOPARITY`"]
    #[inline(always)]
    pub fn is_noparity(&self) -> bool {
        *self == PF_A::NOPARITY
    }
    #[doc = "Checks if the value of the field is `PARITY`"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == PF_A::PARITY
    }
}
#[doc = "Field `PF` writer - Parity Error Flag"]
pub type PF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, PF_A, O>;
impl<'a, const O: u8> PF_W<'a, O> {
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn noparity(self) -> &'a mut W {
        self.variant(PF_A::NOPARITY)
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn parity(self) -> &'a mut W {
        self.variant(PF_A::PARITY)
    }
}
#[doc = "Field `FE` reader - Framing Error Flag"]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: No framing error detected. This does not guarantee the framing is correct."]
    NOERROR = 0,
    #[doc = "1: Framing error."]
    ERROR = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::NOERROR,
            true => FE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == FE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FE_A::ERROR
    }
}
#[doc = "Field `FE` writer - Framing Error Flag"]
pub type FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, FE_A, O>;
impl<'a, const O: u8> FE_W<'a, O> {
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline(always)]
    pub fn noerror(self) -> &'a mut W {
        self.variant(FE_A::NOERROR)
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(FE_A::ERROR)
    }
}
#[doc = "Field `NF` reader - Noise Flag"]
pub type NF_R = crate::BitReader<NF_A>;
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NF_A {
    #[doc = "0: No noise detected."]
    NONOISE = 0,
    #[doc = "1: Noise detected in the received character in the DATA register."]
    NOISE = 1,
}
impl From<NF_A> for bool {
    #[inline(always)]
    fn from(variant: NF_A) -> Self {
        variant as u8 != 0
    }
}
impl NF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NF_A {
        match self.bits {
            false => NF_A::NONOISE,
            true => NF_A::NOISE,
        }
    }
    #[doc = "Checks if the value of the field is `NONOISE`"]
    #[inline(always)]
    pub fn is_nonoise(&self) -> bool {
        *self == NF_A::NONOISE
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NF_A::NOISE
    }
}
#[doc = "Field `NF` writer - Noise Flag"]
pub type NF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, NF_A, O>;
impl<'a, const O: u8> NF_W<'a, O> {
    #[doc = "No noise detected."]
    #[inline(always)]
    pub fn nonoise(self) -> &'a mut W {
        self.variant(NF_A::NONOISE)
    }
    #[doc = "Noise detected in the received character in the DATA register."]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(NF_A::NOISE)
    }
}
#[doc = "Field `OR` reader - Receiver Overrun Flag"]
pub type OR_R = crate::BitReader<OR_A>;
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OR_A {
    #[doc = "0: No overrun."]
    NO_OVERRUN = 0,
    #[doc = "1: Receive overrun (new LPUART data lost)."]
    OVERRUN = 1,
}
impl From<OR_A> for bool {
    #[inline(always)]
    fn from(variant: OR_A) -> Self {
        variant as u8 != 0
    }
}
impl OR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OR_A {
        match self.bits {
            false => OR_A::NO_OVERRUN,
            true => OR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OR_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OR_A::OVERRUN
    }
}
#[doc = "Field `OR` writer - Receiver Overrun Flag"]
pub type OR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, OR_A, O>;
impl<'a, const O: u8> OR_W<'a, O> {
    #[doc = "No overrun."]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OR_A::NO_OVERRUN)
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OR_A::OVERRUN)
    }
}
#[doc = "Field `IDLE` reader - Idle Line Flag"]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: No idle line detected."]
    NOIDLE = 0,
    #[doc = "1: Idle line is detected."]
    IDLE = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::NOIDLE,
            true => IDLE_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOIDLE`"]
    #[inline(always)]
    pub fn is_noidle(&self) -> bool {
        *self == IDLE_A::NOIDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE_A::IDLE
    }
}
#[doc = "Field `IDLE` writer - Idle Line Flag"]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, IDLE_A, O>;
impl<'a, const O: u8> IDLE_W<'a, O> {
    #[doc = "No idle line detected."]
    #[inline(always)]
    pub fn noidle(self) -> &'a mut W {
        self.variant(IDLE_A::NOIDLE)
    }
    #[doc = "Idle line is detected."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(IDLE_A::IDLE)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Flag"]
pub type RDRF_R = crate::BitReader<RDRF_A>;
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRF_A {
    #[doc = "0: Receive FIFO level is less than watermark."]
    NO_RXDATA = 0,
    #[doc = "1: Receive FIFO level is equal or greater than watermark."]
    RXDATA = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::NO_RXDATA,
            true => RDRF_A::RXDATA,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RXDATA`"]
    #[inline(always)]
    pub fn is_no_rxdata(&self) -> bool {
        *self == RDRF_A::NO_RXDATA
    }
    #[doc = "Checks if the value of the field is `RXDATA`"]
    #[inline(always)]
    pub fn is_rxdata(&self) -> bool {
        *self == RDRF_A::RXDATA
    }
}
#[doc = "Field `TC` reader - Transmission Complete Flag"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transmission Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transmitter active (sending data, a preamble, or a break)."]
    ACTIVE = 0,
    #[doc = "1: Transmitter idle (transmission activity complete)."]
    COMPLETE = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::ACTIVE,
            true => TC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == TC_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC_A::COMPLETE
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Flag"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: Transmit FIFO level is greater than watermark."]
    TXDATA = 0,
    #[doc = "1: Transmit FIFO level is equal or less than watermark."]
    NO_TXDATA = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::TXDATA,
            true => TDRE_A::NO_TXDATA,
        }
    }
    #[doc = "Checks if the value of the field is `TXDATA`"]
    #[inline(always)]
    pub fn is_txdata(&self) -> bool {
        *self == TDRE_A::TXDATA
    }
    #[doc = "Checks if the value of the field is `NO_TXDATA`"]
    #[inline(always)]
    pub fn is_no_txdata(&self) -> bool {
        *self == TDRE_A::NO_TXDATA
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub type RAF_R = crate::BitReader<RAF_A>;
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAF_A {
    #[doc = "0: LPUART receiver idle waiting for a start bit."]
    IDLE = 0,
    #[doc = "1: LPUART receiver active (RXD input not idle)."]
    ACTIVE = 1,
}
impl From<RAF_A> for bool {
    #[inline(always)]
    fn from(variant: RAF_A) -> Self {
        variant as u8 != 0
    }
}
impl RAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAF_A {
        match self.bits {
            false => RAF_A::IDLE,
            true => RAF_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RAF_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RAF_A::ACTIVE
    }
}
#[doc = "Field `LBKDE` reader - LIN Break Detection Enable"]
pub type LBKDE_R = crate::BitReader<LBKDE_A>;
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDE_A {
    #[doc = "0: LIN break detect is disabled, normal break character can be detected."]
    DISABLED = 0,
    #[doc = "1: LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    ENABLED = 1,
}
impl From<LBKDE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDE_A {
        match self.bits {
            false => LBKDE_A::DISABLED,
            true => LBKDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBKDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBKDE_A::ENABLED
    }
}
#[doc = "Field `LBKDE` writer - LIN Break Detection Enable"]
pub type LBKDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, LBKDE_A, O>;
impl<'a, const O: u8> LBKDE_W<'a, O> {
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBKDE_A::DISABLED)
    }
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBKDE_A::ENABLED)
    }
}
#[doc = "Field `BRK13` reader - Break Character Generation Length"]
pub type BRK13_R = crate::BitReader<BRK13_A>;
#[doc = "Break Character Generation Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRK13_A {
    #[doc = "0: Break character is transmitted with length of 9 to 13 bit times."]
    SHORT = 0,
    #[doc = "1: Break character is transmitted with length of 12 to 15 bit times."]
    LONG = 1,
}
impl From<BRK13_A> for bool {
    #[inline(always)]
    fn from(variant: BRK13_A) -> Self {
        variant as u8 != 0
    }
}
impl BRK13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK13_A {
        match self.bits {
            false => BRK13_A::SHORT,
            true => BRK13_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == BRK13_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == BRK13_A::LONG
    }
}
#[doc = "Field `BRK13` writer - Break Character Generation Length"]
pub type BRK13_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, BRK13_A, O>;
impl<'a, const O: u8> BRK13_W<'a, O> {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(BRK13_A::SHORT)
    }
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(BRK13_A::LONG)
    }
}
#[doc = "Field `RWUID` reader - Receive Wake Up Idle Detect"]
pub type RWUID_R = crate::BitReader<RWUID_A>;
#[doc = "Receive Wake Up Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWUID_A {
    #[doc = "0: During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    IDLE_NOTSET = 0,
    #[doc = "1: During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    IDLE_SET = 1,
}
impl From<RWUID_A> for bool {
    #[inline(always)]
    fn from(variant: RWUID_A) -> Self {
        variant as u8 != 0
    }
}
impl RWUID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWUID_A {
        match self.bits {
            false => RWUID_A::IDLE_NOTSET,
            true => RWUID_A::IDLE_SET,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_NOTSET`"]
    #[inline(always)]
    pub fn is_idle_notset(&self) -> bool {
        *self == RWUID_A::IDLE_NOTSET
    }
    #[doc = "Checks if the value of the field is `IDLE_SET`"]
    #[inline(always)]
    pub fn is_idle_set(&self) -> bool {
        *self == RWUID_A::IDLE_SET
    }
}
#[doc = "Field `RWUID` writer - Receive Wake Up Idle Detect"]
pub type RWUID_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, RWUID_A, O>;
impl<'a, const O: u8> RWUID_W<'a, O> {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    #[inline(always)]
    pub fn idle_notset(self) -> &'a mut W {
        self.variant(RWUID_A::IDLE_NOTSET)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    #[inline(always)]
    pub fn idle_set(self) -> &'a mut W {
        self.variant(RWUID_A::IDLE_SET)
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub type RXINV_R = crate::BitReader<RXINV_A>;
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    #[doc = "0: Receive data not inverted."]
    NOT_INVERTED = 0,
    #[doc = "1: Receive data inverted."]
    INVERTED = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::NOT_INVERTED,
            true => RXINV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == RXINV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV_A::INVERTED
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, RXINV_A, O>;
impl<'a, const O: u8> RXINV_W<'a, O> {
    #[doc = "Receive data not inverted."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(RXINV_A::NOT_INVERTED)
    }
    #[doc = "Receive data inverted."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINV_A::INVERTED)
    }
}
#[doc = "Field `MSBF` reader - MSB First"]
pub type MSBF_R = crate::BitReader<MSBF_A>;
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBF_A {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    LSB_FIRST = 0,
    #[doc = "1: MSB (identified as bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. ."]
    MSB_FIRST = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::LSB_FIRST,
            true => MSBF_A::MSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == MSBF_A::LSB_FIRST
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == MSBF_A::MSB_FIRST
    }
}
#[doc = "Field `MSBF` writer - MSB First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, MSBF_A, O>;
impl<'a, const O: u8> MSBF_W<'a, O> {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(MSBF_A::LSB_FIRST)
    }
    #[doc = "MSB (identified as bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. ."]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(MSBF_A::MSB_FIRST)
    }
}
#[doc = "Field `RXEDGIF` reader - RXD Pin Active Edge Interrupt Flag"]
pub type RXEDGIF_R = crate::BitReader<RXEDGIF_A>;
#[doc = "RXD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEDGIF_A {
    #[doc = "0: No active edge on the receive pin has occurred."]
    NO_EDGE = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    EDGE = 1,
}
impl From<RXEDGIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEDGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIF_A {
        match self.bits {
            false => RXEDGIF_A::NO_EDGE,
            true => RXEDGIF_A::EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == RXEDGIF_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == RXEDGIF_A::EDGE
    }
}
#[doc = "Field `RXEDGIF` writer - RXD Pin Active Edge Interrupt Flag"]
pub type RXEDGIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, RXEDGIF_A, O>;
impl<'a, const O: u8> RXEDGIF_W<'a, O> {
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(RXEDGIF_A::NO_EDGE)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(RXEDGIF_A::EDGE)
    }
}
#[doc = "Field `LBKDIF` reader - LIN Break Detect Interrupt Flag"]
pub type LBKDIF_R = crate::BitReader<LBKDIF_A>;
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDIF_A {
    #[doc = "0: No LIN break character has been detected."]
    NOT_DETECTED = 0,
    #[doc = "1: LIN break character has been detected."]
    DETECTED = 1,
}
impl From<LBKDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIF_A {
        match self.bits {
            false => LBKDIF_A::NOT_DETECTED,
            true => LBKDIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LBKDIF_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LBKDIF_A::DETECTED
    }
}
#[doc = "Field `LBKDIF` writer - LIN Break Detect Interrupt Flag"]
pub type LBKDIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STAT_SPEC, LBKDIF_A, O>;
impl<'a, const O: u8> LBKDIF_W<'a, O> {
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LBKDIF_A::NOT_DETECTED)
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LBKDIF_A::DETECTED)
    }
}
impl R {
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    pub fn ma2f(&self) -> MA2F_R {
        MA2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    pub fn ma1f(&self) -> MA1F_R {
        MA1F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RAF_R {
        RAF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LBKDE_R {
        LBKDE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&self) -> BRK13_R {
        BRK13_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RWUID_R {
        RWUID_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RXEDGIF_R {
        RXEDGIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LBKDIF_R {
        LBKDIF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ma2f(&mut self) -> MA2F_W<14> {
        MA2F_W::new(self)
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ma1f(&mut self) -> MA1F_W<15> {
        MA1F_W::new(self)
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<16> {
        PF_W::new(self)
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<17> {
        FE_W::new(self)
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<18> {
        NF_W::new(self)
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OR_W<19> {
        OR_W::new(self)
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<20> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkde(&mut self) -> LBKDE_W<25> {
        LBKDE_W::new(self)
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    #[must_use]
    pub fn brk13(&mut self) -> BRK13_W<26> {
        BRK13_W::new(self)
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    #[must_use]
    pub fn rwuid(&mut self) -> RWUID_W<27> {
        RWUID_W::new(self)
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<28> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<29> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgif(&mut self) -> RXEDGIF_W<30> {
        RXEDGIF_W::new(self)
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdif(&mut self) -> LBKDIF_W<31> {
        LBKDIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xc01f_c000;
}
#[doc = "`reset()` method sets STAT to value 0x00c0_0000"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c0_0000;
}
