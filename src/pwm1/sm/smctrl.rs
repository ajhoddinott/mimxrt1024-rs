#[doc = "Register `SMCTRL` reader"]
pub struct R(crate::R<SMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCTRL` writer"]
pub struct W(crate::W<SMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCTRL_SPEC>;
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
impl From<crate::W<SMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBLEN` reader - Double Switching Enable"]
pub type DBLEN_R = crate::BitReader<DBLEN_A>;
#[doc = "Double Switching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLEN_A {
    #[doc = "0: Double switching disabled."]
    DISABLED = 0,
    #[doc = "1: Double switching enabled."]
    ENABLED = 1,
}
impl From<DBLEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLEN_A {
        match self.bits {
            false => DBLEN_A::DISABLED,
            true => DBLEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBLEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBLEN_A::ENABLED
    }
}
#[doc = "Field `DBLEN` writer - Double Switching Enable"]
pub type DBLEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, DBLEN_A, O>;
impl<'a, const O: u8> DBLEN_W<'a, O> {
    #[doc = "Double switching disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLEN_A::DISABLED)
    }
    #[doc = "Double switching enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLEN_A::ENABLED)
    }
}
#[doc = "Field `DBLX` reader - PWMX Double Switching Enable"]
pub type DBLX_R = crate::BitReader<DBLX_A>;
#[doc = "PWMX Double Switching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLX_A {
    #[doc = "0: PWMX double pulse disabled."]
    DISABLED = 0,
    #[doc = "1: PWMX double pulse enabled."]
    ENABLED = 1,
}
impl From<DBLX_A> for bool {
    #[inline(always)]
    fn from(variant: DBLX_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLX_A {
        match self.bits {
            false => DBLX_A::DISABLED,
            true => DBLX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBLX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBLX_A::ENABLED
    }
}
#[doc = "Field `DBLX` writer - PWMX Double Switching Enable"]
pub type DBLX_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, DBLX_A, O>;
impl<'a, const O: u8> DBLX_W<'a, O> {
    #[doc = "PWMX double pulse disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLX_A::DISABLED)
    }
    #[doc = "PWMX double pulse enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLX_A::ENABLED)
    }
}
#[doc = "Field `LDMOD` reader - Load Mode Select"]
pub type LDMOD_R = crate::BitReader<LDMOD_A>;
#[doc = "Load Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMOD_A {
    #[doc = "0: Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\]
is set."]
    NEXT_PWM_RELOAD = 0,
    #[doc = "1: Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\]
being set. In this case it is not necessary to set CTRL\\[FULL\\]
or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 1,
}
impl From<LDMOD_A> for bool {
    #[inline(always)]
    fn from(variant: LDMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl LDMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMOD_A {
        match self.bits {
            false => LDMOD_A::NEXT_PWM_RELOAD,
            true => LDMOD_A::MTCTRL_LDOK_SET,
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_PWM_RELOAD`"]
    #[inline(always)]
    pub fn is_next_pwm_reload(&self) -> bool {
        *self == LDMOD_A::NEXT_PWM_RELOAD
    }
    #[doc = "Checks if the value of the field is `MTCTRL_LDOK_SET`"]
    #[inline(always)]
    pub fn is_mtctrl_ldok_set(&self) -> bool {
        *self == LDMOD_A::MTCTRL_LDOK_SET
    }
}
#[doc = "Field `LDMOD` writer - Load Mode Select"]
pub type LDMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, LDMOD_A, O>;
impl<'a, const O: u8> LDMOD_W<'a, O> {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\]
is set."]
    #[inline(always)]
    pub fn next_pwm_reload(self) -> &'a mut W {
        self.variant(LDMOD_A::NEXT_PWM_RELOAD)
    }
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\]
being set. In this case it is not necessary to set CTRL\\[FULL\\]
or CTRL\\[HALF\\]."]
    #[inline(always)]
    pub fn mtctrl_ldok_set(self) -> &'a mut W {
        self.variant(LDMOD_A::MTCTRL_LDOK_SET)
    }
}
#[doc = "Field `SPLIT` reader - Split the DBLPWM signal to PWMA and PWMB"]
pub type SPLIT_R = crate::BitReader<SPLIT_A>;
#[doc = "Split the DBLPWM signal to PWMA and PWMB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPLIT_A {
    #[doc = "0: DBLPWM is not split. PWMA and PWMB each have double pulses."]
    DISABLED = 0,
    #[doc = "1: DBLPWM is split to PWMA and PWMB."]
    ENABLED = 1,
}
impl From<SPLIT_A> for bool {
    #[inline(always)]
    fn from(variant: SPLIT_A) -> Self {
        variant as u8 != 0
    }
}
impl SPLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLIT_A {
        match self.bits {
            false => SPLIT_A::DISABLED,
            true => SPLIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPLIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPLIT_A::ENABLED
    }
}
#[doc = "Field `SPLIT` writer - Split the DBLPWM signal to PWMA and PWMB"]
pub type SPLIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, SPLIT_A, O>;
impl<'a, const O: u8> SPLIT_W<'a, O> {
    #[doc = "DBLPWM is not split. PWMA and PWMB each have double pulses."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPLIT_A::DISABLED)
    }
    #[doc = "DBLPWM is split to PWMA and PWMB."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPLIT_A::ENABLED)
    }
}
#[doc = "Field `PRSC` reader - Prescaler"]
pub type PRSC_R = crate::FieldReader<u8, PRSC_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSC_A {
    #[doc = "0: Prescaler 1"]
    ONE = 0,
    #[doc = "1: Prescaler 2"]
    TWO = 1,
    #[doc = "2: Prescaler 4"]
    FOUR = 2,
    #[doc = "3: Prescaler 8"]
    EIGHT = 3,
    #[doc = "4: Prescaler 16"]
    SIXTEEN = 4,
    #[doc = "5: Prescaler 32"]
    THIRTYTWO = 5,
    #[doc = "6: Prescaler 64"]
    SIXTYFOUR = 6,
    #[doc = "7: Prescaler 128"]
    HUNDREDTWENTYEIGHT = 7,
}
impl From<PRSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSC_A) -> Self {
        variant as _
    }
}
impl PRSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSC_A {
        match self.bits {
            0 => PRSC_A::ONE,
            1 => PRSC_A::TWO,
            2 => PRSC_A::FOUR,
            3 => PRSC_A::EIGHT,
            4 => PRSC_A::SIXTEEN,
            5 => PRSC_A::THIRTYTWO,
            6 => PRSC_A::SIXTYFOUR,
            7 => PRSC_A::HUNDREDTWENTYEIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PRSC_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == PRSC_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == PRSC_A::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == PRSC_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == PRSC_A::SIXTEEN
    }
    #[doc = "Checks if the value of the field is `THIRTYTWO`"]
    #[inline(always)]
    pub fn is_thirtytwo(&self) -> bool {
        *self == PRSC_A::THIRTYTWO
    }
    #[doc = "Checks if the value of the field is `SIXTYFOUR`"]
    #[inline(always)]
    pub fn is_sixtyfour(&self) -> bool {
        *self == PRSC_A::SIXTYFOUR
    }
    #[doc = "Checks if the value of the field is `HUNDREDTWENTYEIGHT`"]
    #[inline(always)]
    pub fn is_hundredtwentyeight(&self) -> bool {
        *self == PRSC_A::HUNDREDTWENTYEIGHT
    }
}
#[doc = "Field `PRSC` writer - Prescaler"]
pub type PRSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, SMCTRL_SPEC, u8, PRSC_A, 3, O>;
impl<'a, const O: u8> PRSC_W<'a, O> {
    #[doc = "Prescaler 1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PRSC_A::ONE)
    }
    #[doc = "Prescaler 2"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(PRSC_A::TWO)
    }
    #[doc = "Prescaler 4"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(PRSC_A::FOUR)
    }
    #[doc = "Prescaler 8"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(PRSC_A::EIGHT)
    }
    #[doc = "Prescaler 16"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(PRSC_A::SIXTEEN)
    }
    #[doc = "Prescaler 32"]
    #[inline(always)]
    pub fn thirtytwo(self) -> &'a mut W {
        self.variant(PRSC_A::THIRTYTWO)
    }
    #[doc = "Prescaler 64"]
    #[inline(always)]
    pub fn sixtyfour(self) -> &'a mut W {
        self.variant(PRSC_A::SIXTYFOUR)
    }
    #[doc = "Prescaler 128"]
    #[inline(always)]
    pub fn hundredtwentyeight(self) -> &'a mut W {
        self.variant(PRSC_A::HUNDREDTWENTYEIGHT)
    }
}
#[doc = "Field `COMPMODE` reader - Compare Mode"]
pub type COMPMODE_R = crate::BitReader<COMPMODE_A>;
#[doc = "Compare Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPMODE_A {
    #[doc = "0: The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0,
    #[doc = "1: The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 1,
}
impl From<COMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPMODE_A {
        match self.bits {
            false => COMPMODE_A::EQUAL_TO,
            true => COMPMODE_A::EQUAL_TO_OR_GREATER_THAN,
        }
    }
    #[doc = "Checks if the value of the field is `EQUAL_TO`"]
    #[inline(always)]
    pub fn is_equal_to(&self) -> bool {
        *self == COMPMODE_A::EQUAL_TO
    }
    #[doc = "Checks if the value of the field is `EQUAL_TO_OR_GREATER_THAN`"]
    #[inline(always)]
    pub fn is_equal_to_or_greater_than(&self) -> bool {
        *self == COMPMODE_A::EQUAL_TO_OR_GREATER_THAN
    }
}
#[doc = "Field `COMPMODE` writer - Compare Mode"]
pub type COMPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, COMPMODE_A, O>;
impl<'a, const O: u8> COMPMODE_W<'a, O> {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWMA output that is high at the end of a period will maintain this state until a match with VAL3 clears the output in the following period."]
    #[inline(always)]
    pub fn equal_to(self) -> &'a mut W {
        self.variant(COMPMODE_A::EQUAL_TO)
    }
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWMA output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    #[inline(always)]
    pub fn equal_to_or_greater_than(self) -> &'a mut W {
        self.variant(COMPMODE_A::EQUAL_TO_OR_GREATER_THAN)
    }
}
#[doc = "Field `DT` reader - Deadtime"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FULL` reader - Full Cycle Reload"]
pub type FULL_R = crate::BitReader<FULL_A>;
#[doc = "Full Cycle Reload\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL_A {
    #[doc = "0: Full-cycle reloads disabled."]
    DISABLED = 0,
    #[doc = "1: Full-cycle reloads enabled."]
    ENABLED = 1,
}
impl From<FULL_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL_A {
        match self.bits {
            false => FULL_A::DISABLED,
            true => FULL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FULL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FULL_A::ENABLED
    }
}
#[doc = "Field `FULL` writer - Full Cycle Reload"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, FULL_A, O>;
impl<'a, const O: u8> FULL_W<'a, O> {
    #[doc = "Full-cycle reloads disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FULL_A::DISABLED)
    }
    #[doc = "Full-cycle reloads enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FULL_A::ENABLED)
    }
}
#[doc = "Field `HALF` reader - Half Cycle Reload"]
pub type HALF_R = crate::BitReader<HALF_A>;
#[doc = "Half Cycle Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF_A {
    #[doc = "0: Half-cycle reloads disabled."]
    DISABLED = 0,
    #[doc = "1: Half-cycle reloads enabled."]
    ENABLED = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
impl HALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::DISABLED,
            true => HALF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF_A::ENABLED
    }
}
#[doc = "Field `HALF` writer - Half Cycle Reload"]
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL_SPEC, HALF_A, O>;
impl<'a, const O: u8> HALF_W<'a, O> {
    #[doc = "Half-cycle reloads disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HALF_A::DISABLED)
    }
    #[doc = "Half-cycle reloads enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HALF_A::ENABLED)
    }
}
#[doc = "Field `LDFQ` reader - Load Frequency"]
pub type LDFQ_R = crate::FieldReader<u8, LDFQ_A>;
#[doc = "Load Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDFQ_A {
    #[doc = "0: Every PWM opportunity"]
    EVERYPWM = 0,
    #[doc = "1: Every 2 PWM opportunities"]
    EVERY2PWM = 1,
    #[doc = "2: Every 3 PWM opportunities"]
    EVERY3PWM = 2,
    #[doc = "3: Every 4 PWM opportunities"]
    EVERY4PWM = 3,
    #[doc = "4: Every 5 PWM opportunities"]
    EVERY5PWM = 4,
    #[doc = "5: Every 6 PWM opportunities"]
    EVERY6PWM = 5,
    #[doc = "6: Every 7 PWM opportunities"]
    EVERY7PWM = 6,
    #[doc = "7: Every 8 PWM opportunities"]
    EVERY8PWM = 7,
    #[doc = "8: Every 9 PWM opportunities"]
    EVERY9PWM = 8,
    #[doc = "9: Every 10 PWM opportunities"]
    EVERY10PWM = 9,
    #[doc = "10: Every 11 PWM opportunities"]
    EVERY11PWM = 10,
    #[doc = "11: Every 12 PWM opportunities"]
    EVERY12PWM = 11,
    #[doc = "12: Every 13 PWM opportunities"]
    EVERY13PWM = 12,
    #[doc = "13: Every 14 PWM opportunities"]
    EVERY14PWM = 13,
    #[doc = "14: Every 15 PWM opportunities"]
    EVERY15PWM = 14,
    #[doc = "15: Every 16 PWM opportunities"]
    EVERY16PWM = 15,
}
impl From<LDFQ_A> for u8 {
    #[inline(always)]
    fn from(variant: LDFQ_A) -> Self {
        variant as _
    }
}
impl LDFQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDFQ_A {
        match self.bits {
            0 => LDFQ_A::EVERYPWM,
            1 => LDFQ_A::EVERY2PWM,
            2 => LDFQ_A::EVERY3PWM,
            3 => LDFQ_A::EVERY4PWM,
            4 => LDFQ_A::EVERY5PWM,
            5 => LDFQ_A::EVERY6PWM,
            6 => LDFQ_A::EVERY7PWM,
            7 => LDFQ_A::EVERY8PWM,
            8 => LDFQ_A::EVERY9PWM,
            9 => LDFQ_A::EVERY10PWM,
            10 => LDFQ_A::EVERY11PWM,
            11 => LDFQ_A::EVERY12PWM,
            12 => LDFQ_A::EVERY13PWM,
            13 => LDFQ_A::EVERY14PWM,
            14 => LDFQ_A::EVERY15PWM,
            15 => LDFQ_A::EVERY16PWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVERYPWM`"]
    #[inline(always)]
    pub fn is_everypwm(&self) -> bool {
        *self == LDFQ_A::EVERYPWM
    }
    #[doc = "Checks if the value of the field is `EVERY2PWM`"]
    #[inline(always)]
    pub fn is_every2pwm(&self) -> bool {
        *self == LDFQ_A::EVERY2PWM
    }
    #[doc = "Checks if the value of the field is `EVERY3PWM`"]
    #[inline(always)]
    pub fn is_every3pwm(&self) -> bool {
        *self == LDFQ_A::EVERY3PWM
    }
    #[doc = "Checks if the value of the field is `EVERY4PWM`"]
    #[inline(always)]
    pub fn is_every4pwm(&self) -> bool {
        *self == LDFQ_A::EVERY4PWM
    }
    #[doc = "Checks if the value of the field is `EVERY5PWM`"]
    #[inline(always)]
    pub fn is_every5pwm(&self) -> bool {
        *self == LDFQ_A::EVERY5PWM
    }
    #[doc = "Checks if the value of the field is `EVERY6PWM`"]
    #[inline(always)]
    pub fn is_every6pwm(&self) -> bool {
        *self == LDFQ_A::EVERY6PWM
    }
    #[doc = "Checks if the value of the field is `EVERY7PWM`"]
    #[inline(always)]
    pub fn is_every7pwm(&self) -> bool {
        *self == LDFQ_A::EVERY7PWM
    }
    #[doc = "Checks if the value of the field is `EVERY8PWM`"]
    #[inline(always)]
    pub fn is_every8pwm(&self) -> bool {
        *self == LDFQ_A::EVERY8PWM
    }
    #[doc = "Checks if the value of the field is `EVERY9PWM`"]
    #[inline(always)]
    pub fn is_every9pwm(&self) -> bool {
        *self == LDFQ_A::EVERY9PWM
    }
    #[doc = "Checks if the value of the field is `EVERY10PWM`"]
    #[inline(always)]
    pub fn is_every10pwm(&self) -> bool {
        *self == LDFQ_A::EVERY10PWM
    }
    #[doc = "Checks if the value of the field is `EVERY11PWM`"]
    #[inline(always)]
    pub fn is_every11pwm(&self) -> bool {
        *self == LDFQ_A::EVERY11PWM
    }
    #[doc = "Checks if the value of the field is `EVERY12PWM`"]
    #[inline(always)]
    pub fn is_every12pwm(&self) -> bool {
        *self == LDFQ_A::EVERY12PWM
    }
    #[doc = "Checks if the value of the field is `EVERY13PWM`"]
    #[inline(always)]
    pub fn is_every13pwm(&self) -> bool {
        *self == LDFQ_A::EVERY13PWM
    }
    #[doc = "Checks if the value of the field is `EVERY14PWM`"]
    #[inline(always)]
    pub fn is_every14pwm(&self) -> bool {
        *self == LDFQ_A::EVERY14PWM
    }
    #[doc = "Checks if the value of the field is `EVERY15PWM`"]
    #[inline(always)]
    pub fn is_every15pwm(&self) -> bool {
        *self == LDFQ_A::EVERY15PWM
    }
    #[doc = "Checks if the value of the field is `EVERY16PWM`"]
    #[inline(always)]
    pub fn is_every16pwm(&self) -> bool {
        *self == LDFQ_A::EVERY16PWM
    }
}
#[doc = "Field `LDFQ` writer - Load Frequency"]
pub type LDFQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, SMCTRL_SPEC, u8, LDFQ_A, 4, O>;
impl<'a, const O: u8> LDFQ_W<'a, O> {
    #[doc = "Every PWM opportunity"]
    #[inline(always)]
    pub fn everypwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERYPWM)
    }
    #[doc = "Every 2 PWM opportunities"]
    #[inline(always)]
    pub fn every2pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY2PWM)
    }
    #[doc = "Every 3 PWM opportunities"]
    #[inline(always)]
    pub fn every3pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY3PWM)
    }
    #[doc = "Every 4 PWM opportunities"]
    #[inline(always)]
    pub fn every4pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY4PWM)
    }
    #[doc = "Every 5 PWM opportunities"]
    #[inline(always)]
    pub fn every5pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY5PWM)
    }
    #[doc = "Every 6 PWM opportunities"]
    #[inline(always)]
    pub fn every6pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY6PWM)
    }
    #[doc = "Every 7 PWM opportunities"]
    #[inline(always)]
    pub fn every7pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY7PWM)
    }
    #[doc = "Every 8 PWM opportunities"]
    #[inline(always)]
    pub fn every8pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY8PWM)
    }
    #[doc = "Every 9 PWM opportunities"]
    #[inline(always)]
    pub fn every9pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY9PWM)
    }
    #[doc = "Every 10 PWM opportunities"]
    #[inline(always)]
    pub fn every10pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY10PWM)
    }
    #[doc = "Every 11 PWM opportunities"]
    #[inline(always)]
    pub fn every11pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY11PWM)
    }
    #[doc = "Every 12 PWM opportunities"]
    #[inline(always)]
    pub fn every12pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY12PWM)
    }
    #[doc = "Every 13 PWM opportunities"]
    #[inline(always)]
    pub fn every13pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY13PWM)
    }
    #[doc = "Every 14 PWM opportunities"]
    #[inline(always)]
    pub fn every14pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY14PWM)
    }
    #[doc = "Every 15 PWM opportunities"]
    #[inline(always)]
    pub fn every15pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY15PWM)
    }
    #[doc = "Every 16 PWM opportunities"]
    #[inline(always)]
    pub fn every16pwm(self) -> &'a mut W {
        self.variant(LDFQ_A::EVERY16PWM)
    }
}
impl R {
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline(always)]
    pub fn dblen(&self) -> DBLEN_R {
        DBLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline(always)]
    pub fn dblx(&self) -> DBLX_R {
        DBLX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&self) -> LDMOD_R {
        LDMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    pub fn split(&self) -> SPLIT_R {
        SPLIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline(always)]
    pub fn compmode(&self) -> COMPMODE_R {
        COMPMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Deadtime"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline(always)]
    pub fn ldfq(&self) -> LDFQ_R {
        LDFQ_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Double Switching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dblen(&mut self) -> DBLEN_W<0> {
        DBLEN_W::new(self)
    }
    #[doc = "Bit 1 - PWMX Double Switching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dblx(&mut self) -> DBLX_W<1> {
        DBLX_W::new(self)
    }
    #[doc = "Bit 2 - Load Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ldmod(&mut self) -> LDMOD_W<2> {
        LDMOD_W::new(self)
    }
    #[doc = "Bit 3 - Split the DBLPWM signal to PWMA and PWMB"]
    #[inline(always)]
    #[must_use]
    pub fn split(&mut self) -> SPLIT_W<3> {
        SPLIT_W::new(self)
    }
    #[doc = "Bits 4:6 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prsc(&mut self) -> PRSC_W<4> {
        PRSC_W::new(self)
    }
    #[doc = "Bit 7 - Compare Mode"]
    #[inline(always)]
    #[must_use]
    pub fn compmode(&mut self) -> COMPMODE_W<7> {
        COMPMODE_W::new(self)
    }
    #[doc = "Bit 10 - Full Cycle Reload"]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<10> {
        FULL_W::new(self)
    }
    #[doc = "Bit 11 - Half Cycle Reload"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<11> {
        HALF_W::new(self)
    }
    #[doc = "Bits 12:15 - Load Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn ldfq(&mut self) -> LDFQ_W<12> {
        LDFQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smctrl](index.html) module"]
pub struct SMCTRL_SPEC;
impl crate::RegisterSpec for SMCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smctrl::R](R) reader structure"]
impl crate::Readable for SMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smctrl::W](W) writer structure"]
impl crate::Writable for SMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCTRL to value 0x0400"]
impl crate::Resettable for SMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
