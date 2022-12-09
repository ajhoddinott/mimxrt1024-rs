#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTMODE` reader - Output Mode"]
pub type OUTMODE_R = crate::FieldReader<u8, OUTMODE_A>;
#[doc = "Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTMODE_A {
    #[doc = "0: Asserted while counter is active"]
    COUNTER_ACTIVE = 0,
    #[doc = "1: Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 1,
    #[doc = "2: Set OFLAG output on successful compare"]
    SET_OFLAG = 2,
    #[doc = "3: Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 3,
    #[doc = "4: Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 4,
    #[doc = "5: Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 5,
    #[doc = "6: Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 6,
    #[doc = "7: Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 7,
}
impl From<OUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODE_A) -> Self {
        variant as _
    }
}
impl OUTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMODE_A {
        match self.bits {
            0 => OUTMODE_A::COUNTER_ACTIVE,
            1 => OUTMODE_A::CLEAR_OFLAG,
            2 => OUTMODE_A::SET_OFLAG,
            3 => OUTMODE_A::TOGGLE_OFLAG_SUCCESS,
            4 => OUTMODE_A::TOGGLE_OFLAG_ALT,
            5 => OUTMODE_A::CLEAR_ON_SECONDARY,
            6 => OUTMODE_A::CLEAR_ON_ROLLOVER,
            7 => OUTMODE_A::ENABLE_GATED_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNTER_ACTIVE`"]
    #[inline(always)]
    pub fn is_counter_active(&self) -> bool {
        *self == OUTMODE_A::COUNTER_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR_OFLAG`"]
    #[inline(always)]
    pub fn is_clear_oflag(&self) -> bool {
        *self == OUTMODE_A::CLEAR_OFLAG
    }
    #[doc = "Checks if the value of the field is `SET_OFLAG`"]
    #[inline(always)]
    pub fn is_set_oflag(&self) -> bool {
        *self == OUTMODE_A::SET_OFLAG
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OFLAG_SUCCESS`"]
    #[inline(always)]
    pub fn is_toggle_oflag_success(&self) -> bool {
        *self == OUTMODE_A::TOGGLE_OFLAG_SUCCESS
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OFLAG_ALT`"]
    #[inline(always)]
    pub fn is_toggle_oflag_alt(&self) -> bool {
        *self == OUTMODE_A::TOGGLE_OFLAG_ALT
    }
    #[doc = "Checks if the value of the field is `CLEAR_ON_SECONDARY`"]
    #[inline(always)]
    pub fn is_clear_on_secondary(&self) -> bool {
        *self == OUTMODE_A::CLEAR_ON_SECONDARY
    }
    #[doc = "Checks if the value of the field is `CLEAR_ON_ROLLOVER`"]
    #[inline(always)]
    pub fn is_clear_on_rollover(&self) -> bool {
        *self == OUTMODE_A::CLEAR_ON_ROLLOVER
    }
    #[doc = "Checks if the value of the field is `ENABLE_GATED_OUT`"]
    #[inline(always)]
    pub fn is_enable_gated_out(&self) -> bool {
        *self == OUTMODE_A::ENABLE_GATED_OUT
    }
}
#[doc = "Field `OUTMODE` writer - Output Mode"]
pub type OUTMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, OUTMODE_A, 3, O>;
impl<'a, const O: u8> OUTMODE_W<'a, O> {
    #[doc = "Asserted while counter is active"]
    #[inline(always)]
    pub fn counter_active(self) -> &'a mut W {
        self.variant(OUTMODE_A::COUNTER_ACTIVE)
    }
    #[doc = "Clear OFLAG output on successful compare"]
    #[inline(always)]
    pub fn clear_oflag(self) -> &'a mut W {
        self.variant(OUTMODE_A::CLEAR_OFLAG)
    }
    #[doc = "Set OFLAG output on successful compare"]
    #[inline(always)]
    pub fn set_oflag(self) -> &'a mut W {
        self.variant(OUTMODE_A::SET_OFLAG)
    }
    #[doc = "Toggle OFLAG output on successful compare"]
    #[inline(always)]
    pub fn toggle_oflag_success(self) -> &'a mut W {
        self.variant(OUTMODE_A::TOGGLE_OFLAG_SUCCESS)
    }
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    #[inline(always)]
    pub fn toggle_oflag_alt(self) -> &'a mut W {
        self.variant(OUTMODE_A::TOGGLE_OFLAG_ALT)
    }
    #[doc = "Set on compare, cleared on secondary source input edge"]
    #[inline(always)]
    pub fn clear_on_secondary(self) -> &'a mut W {
        self.variant(OUTMODE_A::CLEAR_ON_SECONDARY)
    }
    #[doc = "Set on compare, cleared on counter rollover"]
    #[inline(always)]
    pub fn clear_on_rollover(self) -> &'a mut W {
        self.variant(OUTMODE_A::CLEAR_ON_ROLLOVER)
    }
    #[doc = "Enable gated clock output while counter is active"]
    #[inline(always)]
    pub fn enable_gated_out(self) -> &'a mut W {
        self.variant(OUTMODE_A::ENABLE_GATED_OUT)
    }
}
#[doc = "Field `COINIT` reader - Co-Channel Initialization"]
pub type COINIT_R = crate::BitReader<COINIT_A>;
#[doc = "Co-Channel Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COINIT_A {
    #[doc = "0: Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    DISABLE = 0,
    #[doc = "1: Co-channel counter/timers may force a re-initialization of this counter/timer"]
    ENABLE = 1,
}
impl From<COINIT_A> for bool {
    #[inline(always)]
    fn from(variant: COINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl COINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COINIT_A {
        match self.bits {
            false => COINIT_A::DISABLE,
            true => COINIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COINIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COINIT_A::ENABLE
    }
}
#[doc = "Field `COINIT` writer - Co-Channel Initialization"]
pub type COINIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, COINIT_A, O>;
impl<'a, const O: u8> COINIT_W<'a, O> {
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COINIT_A::DISABLE)
    }
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COINIT_A::ENABLE)
    }
}
#[doc = "Field `DIR` reader - Count Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Count up."]
    COUNTUP = 0,
    #[doc = "1: Count down."]
    COUNTDOWN = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::COUNTUP,
            true => DIR_A::COUNTDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `COUNTUP`"]
    #[inline(always)]
    pub fn is_countup(&self) -> bool {
        *self == DIR_A::COUNTUP
    }
    #[doc = "Checks if the value of the field is `COUNTDOWN`"]
    #[inline(always)]
    pub fn is_countdown(&self) -> bool {
        *self == DIR_A::COUNTDOWN
    }
}
#[doc = "Field `DIR` writer - Count Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Count up."]
    #[inline(always)]
    pub fn countup(self) -> &'a mut W {
        self.variant(DIR_A::COUNTUP)
    }
    #[doc = "Count down."]
    #[inline(always)]
    pub fn countdown(self) -> &'a mut W {
        self.variant(DIR_A::COUNTDOWN)
    }
}
#[doc = "Field `LENGTH` reader - Count Length"]
pub type LENGTH_R = crate::BitReader<LENGTH_A>;
#[doc = "Count Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LENGTH_A {
    #[doc = "0: Count until roll over at $FFFF and continue from $0000."]
    UNTIL_ROLLOVER = 0,
    #[doc = "1: Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    UNTIL_COMPARE = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
impl LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::UNTIL_ROLLOVER,
            true => LENGTH_A::UNTIL_COMPARE,
        }
    }
    #[doc = "Checks if the value of the field is `UNTIL_ROLLOVER`"]
    #[inline(always)]
    pub fn is_until_rollover(&self) -> bool {
        *self == LENGTH_A::UNTIL_ROLLOVER
    }
    #[doc = "Checks if the value of the field is `UNTIL_COMPARE`"]
    #[inline(always)]
    pub fn is_until_compare(&self) -> bool {
        *self == LENGTH_A::UNTIL_COMPARE
    }
}
#[doc = "Field `LENGTH` writer - Count Length"]
pub type LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, LENGTH_A, O>;
impl<'a, const O: u8> LENGTH_W<'a, O> {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    #[inline(always)]
    pub fn until_rollover(self) -> &'a mut W {
        self.variant(LENGTH_A::UNTIL_ROLLOVER)
    }
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    #[inline(always)]
    pub fn until_compare(self) -> &'a mut W {
        self.variant(LENGTH_A::UNTIL_COMPARE)
    }
}
#[doc = "Field `ONCE` reader - Count Once"]
pub type ONCE_R = crate::BitReader<ONCE_A>;
#[doc = "Count Once\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONCE_A {
    #[doc = "0: Count repeatedly."]
    REPEAT = 0,
    #[doc = "1: Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    UNTIL_COMPARE = 1,
}
impl From<ONCE_A> for bool {
    #[inline(always)]
    fn from(variant: ONCE_A) -> Self {
        variant as u8 != 0
    }
}
impl ONCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONCE_A {
        match self.bits {
            false => ONCE_A::REPEAT,
            true => ONCE_A::UNTIL_COMPARE,
        }
    }
    #[doc = "Checks if the value of the field is `REPEAT`"]
    #[inline(always)]
    pub fn is_repeat(&self) -> bool {
        *self == ONCE_A::REPEAT
    }
    #[doc = "Checks if the value of the field is `UNTIL_COMPARE`"]
    #[inline(always)]
    pub fn is_until_compare(&self) -> bool {
        *self == ONCE_A::UNTIL_COMPARE
    }
}
#[doc = "Field `ONCE` writer - Count Once"]
pub type ONCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL0_SPEC, ONCE_A, O>;
impl<'a, const O: u8> ONCE_W<'a, O> {
    #[doc = "Count repeatedly."]
    #[inline(always)]
    pub fn repeat(self) -> &'a mut W {
        self.variant(ONCE_A::REPEAT)
    }
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    #[inline(always)]
    pub fn until_compare(self) -> &'a mut W {
        self.variant(ONCE_A::UNTIL_COMPARE)
    }
}
#[doc = "Field `SCS` reader - Secondary Count Source"]
pub type SCS_R = crate::FieldReader<u8, SCS_A>;
#[doc = "Secondary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "0: Counter 0 input pin"]
    COUNTER0_IN = 0,
    #[doc = "1: Counter 1 input pin"]
    COUNTER1_IN = 1,
    #[doc = "2: Counter 2 input pin"]
    COUNTER2_IN = 2,
    #[doc = "3: Counter 3 input pin"]
    COUNTER3_IN = 3,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
impl SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            0 => SCS_A::COUNTER0_IN,
            1 => SCS_A::COUNTER1_IN,
            2 => SCS_A::COUNTER2_IN,
            3 => SCS_A::COUNTER3_IN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNTER0_IN`"]
    #[inline(always)]
    pub fn is_counter0_in(&self) -> bool {
        *self == SCS_A::COUNTER0_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER1_IN`"]
    #[inline(always)]
    pub fn is_counter1_in(&self) -> bool {
        *self == SCS_A::COUNTER1_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER2_IN`"]
    #[inline(always)]
    pub fn is_counter2_in(&self) -> bool {
        *self == SCS_A::COUNTER2_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER3_IN`"]
    #[inline(always)]
    pub fn is_counter3_in(&self) -> bool {
        *self == SCS_A::COUNTER3_IN
    }
}
#[doc = "Field `SCS` writer - Secondary Count Source"]
pub type SCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, SCS_A, 2, O>;
impl<'a, const O: u8> SCS_W<'a, O> {
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn counter0_in(self) -> &'a mut W {
        self.variant(SCS_A::COUNTER0_IN)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn counter1_in(self) -> &'a mut W {
        self.variant(SCS_A::COUNTER1_IN)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn counter2_in(self) -> &'a mut W {
        self.variant(SCS_A::COUNTER2_IN)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn counter3_in(self) -> &'a mut W {
        self.variant(SCS_A::COUNTER3_IN)
    }
}
#[doc = "Field `PCS` reader - Primary Count Source"]
pub type PCS_R = crate::FieldReader<u8, PCS_A>;
#[doc = "Primary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Counter 0 input pin"]
    COUNTER0_IN = 0,
    #[doc = "1: Counter 1 input pin"]
    COUNTER1_IN = 1,
    #[doc = "2: Counter 2 input pin"]
    COUNTER2_IN = 2,
    #[doc = "3: Counter 3 input pin"]
    COUNTER3_IN = 3,
    #[doc = "4: Counter 0 output"]
    COUNTER0_OUT = 4,
    #[doc = "5: Counter 1 output"]
    COUNTER1_OUT = 5,
    #[doc = "6: Counter 2 output"]
    COUNTER2_OUT = 6,
    #[doc = "7: Counter 3 output"]
    COUNTER3_OUT = 7,
    #[doc = "8: IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 8,
    #[doc = "9: IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 9,
    #[doc = "10: IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 10,
    #[doc = "11: IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 11,
    #[doc = "12: IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 12,
    #[doc = "13: IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 13,
    #[doc = "14: IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 14,
    #[doc = "15: IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 15,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
impl PCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::COUNTER0_IN,
            1 => PCS_A::COUNTER1_IN,
            2 => PCS_A::COUNTER2_IN,
            3 => PCS_A::COUNTER3_IN,
            4 => PCS_A::COUNTER0_OUT,
            5 => PCS_A::COUNTER1_OUT,
            6 => PCS_A::COUNTER2_OUT,
            7 => PCS_A::COUNTER3_OUT,
            8 => PCS_A::BUS_DIVBY1,
            9 => PCS_A::BUS_DIVBY2,
            10 => PCS_A::BUS_DIVBY4,
            11 => PCS_A::BUS_DIVBY8,
            12 => PCS_A::BUS_DIVBY16,
            13 => PCS_A::BUS_DIVBY32,
            14 => PCS_A::BUS_DIVBY64,
            15 => PCS_A::BUS_DIVBY128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNTER0_IN`"]
    #[inline(always)]
    pub fn is_counter0_in(&self) -> bool {
        *self == PCS_A::COUNTER0_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER1_IN`"]
    #[inline(always)]
    pub fn is_counter1_in(&self) -> bool {
        *self == PCS_A::COUNTER1_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER2_IN`"]
    #[inline(always)]
    pub fn is_counter2_in(&self) -> bool {
        *self == PCS_A::COUNTER2_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER3_IN`"]
    #[inline(always)]
    pub fn is_counter3_in(&self) -> bool {
        *self == PCS_A::COUNTER3_IN
    }
    #[doc = "Checks if the value of the field is `COUNTER0_OUT`"]
    #[inline(always)]
    pub fn is_counter0_out(&self) -> bool {
        *self == PCS_A::COUNTER0_OUT
    }
    #[doc = "Checks if the value of the field is `COUNTER1_OUT`"]
    #[inline(always)]
    pub fn is_counter1_out(&self) -> bool {
        *self == PCS_A::COUNTER1_OUT
    }
    #[doc = "Checks if the value of the field is `COUNTER2_OUT`"]
    #[inline(always)]
    pub fn is_counter2_out(&self) -> bool {
        *self == PCS_A::COUNTER2_OUT
    }
    #[doc = "Checks if the value of the field is `COUNTER3_OUT`"]
    #[inline(always)]
    pub fn is_counter3_out(&self) -> bool {
        *self == PCS_A::COUNTER3_OUT
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY1`"]
    #[inline(always)]
    pub fn is_bus_divby1(&self) -> bool {
        *self == PCS_A::BUS_DIVBY1
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY2`"]
    #[inline(always)]
    pub fn is_bus_divby2(&self) -> bool {
        *self == PCS_A::BUS_DIVBY2
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY4`"]
    #[inline(always)]
    pub fn is_bus_divby4(&self) -> bool {
        *self == PCS_A::BUS_DIVBY4
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY8`"]
    #[inline(always)]
    pub fn is_bus_divby8(&self) -> bool {
        *self == PCS_A::BUS_DIVBY8
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY16`"]
    #[inline(always)]
    pub fn is_bus_divby16(&self) -> bool {
        *self == PCS_A::BUS_DIVBY16
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY32`"]
    #[inline(always)]
    pub fn is_bus_divby32(&self) -> bool {
        *self == PCS_A::BUS_DIVBY32
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY64`"]
    #[inline(always)]
    pub fn is_bus_divby64(&self) -> bool {
        *self == PCS_A::BUS_DIVBY64
    }
    #[doc = "Checks if the value of the field is `BUS_DIVBY128`"]
    #[inline(always)]
    pub fn is_bus_divby128(&self) -> bool {
        *self == PCS_A::BUS_DIVBY128
    }
}
#[doc = "Field `PCS` writer - Primary Count Source"]
pub type PCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, PCS_A, 4, O>;
impl<'a, const O: u8> PCS_W<'a, O> {
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn counter0_in(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER0_IN)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn counter1_in(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER1_IN)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn counter2_in(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER2_IN)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn counter3_in(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER3_IN)
    }
    #[doc = "Counter 0 output"]
    #[inline(always)]
    pub fn counter0_out(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER0_OUT)
    }
    #[doc = "Counter 1 output"]
    #[inline(always)]
    pub fn counter1_out(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER1_OUT)
    }
    #[doc = "Counter 2 output"]
    #[inline(always)]
    pub fn counter2_out(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER2_OUT)
    }
    #[doc = "Counter 3 output"]
    #[inline(always)]
    pub fn counter3_out(self) -> &'a mut W {
        self.variant(PCS_A::COUNTER3_OUT)
    }
    #[doc = "IP bus clock divide by 1 prescaler"]
    #[inline(always)]
    pub fn bus_divby1(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY1)
    }
    #[doc = "IP bus clock divide by 2 prescaler"]
    #[inline(always)]
    pub fn bus_divby2(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY2)
    }
    #[doc = "IP bus clock divide by 4 prescaler"]
    #[inline(always)]
    pub fn bus_divby4(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY4)
    }
    #[doc = "IP bus clock divide by 8 prescaler"]
    #[inline(always)]
    pub fn bus_divby8(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY8)
    }
    #[doc = "IP bus clock divide by 16 prescaler"]
    #[inline(always)]
    pub fn bus_divby16(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY16)
    }
    #[doc = "IP bus clock divide by 32 prescaler"]
    #[inline(always)]
    pub fn bus_divby32(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY32)
    }
    #[doc = "IP bus clock divide by 64 prescaler"]
    #[inline(always)]
    pub fn bus_divby64(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY64)
    }
    #[doc = "IP bus clock divide by 128 prescaler"]
    #[inline(always)]
    pub fn bus_divby128(self) -> &'a mut W {
        self.variant(PCS_A::BUS_DIVBY128)
    }
}
#[doc = "Field `CM` reader - Count Mode"]
pub type CM_R = crate::FieldReader<u8, CM_A>;
#[doc = "Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No operation"]
    NOOP = 0,
    #[doc = "1: Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 1,
    #[doc = "2: Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 2,
    #[doc = "3: Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 3,
    #[doc = "4: Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 4,
    #[doc = "5: Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    RISING_SEC_DIR = 5,
    #[doc = "6: Edge of secondary source triggers primary count until compare"]
    SECONDARY = 6,
    #[doc = "7: Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 7,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::NOOP,
            1 => CM_A::RISING_ONLY,
            2 => CM_A::RISING_AND_FALLING,
            3 => CM_A::RISING_WHILE_SEC_HIGH,
            4 => CM_A::QUADRATURE,
            5 => CM_A::RISING_SEC_DIR,
            6 => CM_A::SECONDARY,
            7 => CM_A::CASCADE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOOP`"]
    #[inline(always)]
    pub fn is_noop(&self) -> bool {
        *self == CM_A::NOOP
    }
    #[doc = "Checks if the value of the field is `RISING_ONLY`"]
    #[inline(always)]
    pub fn is_rising_only(&self) -> bool {
        *self == CM_A::RISING_ONLY
    }
    #[doc = "Checks if the value of the field is `RISING_AND_FALLING`"]
    #[inline(always)]
    pub fn is_rising_and_falling(&self) -> bool {
        *self == CM_A::RISING_AND_FALLING
    }
    #[doc = "Checks if the value of the field is `RISING_WHILE_SEC_HIGH`"]
    #[inline(always)]
    pub fn is_rising_while_sec_high(&self) -> bool {
        *self == CM_A::RISING_WHILE_SEC_HIGH
    }
    #[doc = "Checks if the value of the field is `QUADRATURE`"]
    #[inline(always)]
    pub fn is_quadrature(&self) -> bool {
        *self == CM_A::QUADRATURE
    }
    #[doc = "Checks if the value of the field is `RISING_SEC_DIR`"]
    #[inline(always)]
    pub fn is_rising_sec_dir(&self) -> bool {
        *self == CM_A::RISING_SEC_DIR
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == CM_A::SECONDARY
    }
    #[doc = "Checks if the value of the field is `CASCADE`"]
    #[inline(always)]
    pub fn is_cascade(&self) -> bool {
        *self == CM_A::CASCADE
    }
}
#[doc = "Field `CM` writer - Count Mode"]
pub type CM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL0_SPEC, u8, CM_A, 3, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn noop(self) -> &'a mut W {
        self.variant(CM_A::NOOP)
    }
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    #[inline(always)]
    pub fn rising_only(self) -> &'a mut W {
        self.variant(CM_A::RISING_ONLY)
    }
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    #[inline(always)]
    pub fn rising_and_falling(self) -> &'a mut W {
        self.variant(CM_A::RISING_AND_FALLING)
    }
    #[doc = "Count rising edges of primary source while secondary input high active"]
    #[inline(always)]
    pub fn rising_while_sec_high(self) -> &'a mut W {
        self.variant(CM_A::RISING_WHILE_SEC_HIGH)
    }
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    #[inline(always)]
    pub fn quadrature(self) -> &'a mut W {
        self.variant(CM_A::QUADRATURE)
    }
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    #[inline(always)]
    pub fn rising_sec_dir(self) -> &'a mut W {
        self.variant(CM_A::RISING_SEC_DIR)
    }
    #[doc = "Edge of secondary source triggers primary count until compare"]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(CM_A::SECONDARY)
    }
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    #[inline(always)]
    pub fn cascade(self) -> &'a mut W {
        self.variant(CM_A::CASCADE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    pub fn coinit(&self) -> COINIT_R {
        COINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    pub fn once(&self) -> ONCE_R {
        ONCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    #[must_use]
    pub fn outmode(&mut self) -> OUTMODE_W<0> {
        OUTMODE_W::new(self)
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn coinit(&mut self) -> COINIT_W<3> {
        COINIT_W::new(self)
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<5> {
        LENGTH_W::new(self)
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    #[must_use]
    pub fn once(&mut self) -> ONCE_W<6> {
        ONCE_W::new(self)
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<7> {
        SCS_W::new(self)
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<9> {
        PCS_W::new(self)
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<13> {
        CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
