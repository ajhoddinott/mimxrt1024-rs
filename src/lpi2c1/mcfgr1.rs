#[doc = "Register `MCFGR1` reader"]
pub struct R(crate::R<MCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR1` writer"]
pub struct W(crate::W<MCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR1_SPEC>;
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
impl From<crate::W<MCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALE` reader - Prescaler"]
pub type PRESCALE_R = crate::FieldReader<u8, PRESCALE_A>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_BY_1 = 0,
    #[doc = "1: Divide by 2"]
    DIVIDE_BY_2 = 1,
    #[doc = "2: Divide by 4"]
    DIVIDE_BY_4 = 2,
    #[doc = "3: Divide by 8"]
    DIVIDE_BY_8 = 3,
    #[doc = "4: Divide by 16"]
    DIVIDE_BY_16 = 4,
    #[doc = "5: Divide by 32"]
    DIVIDE_BY_32 = 5,
    #[doc = "6: Divide by 64"]
    DIVIDE_BY_64 = 6,
    #[doc = "7: Divide by 128"]
    DIVIDE_BY_128 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
impl PRESCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::DIVIDE_BY_1,
            1 => PRESCALE_A::DIVIDE_BY_2,
            2 => PRESCALE_A::DIVIDE_BY_4,
            3 => PRESCALE_A::DIVIDE_BY_8,
            4 => PRESCALE_A::DIVIDE_BY_16,
            5 => PRESCALE_A::DIVIDE_BY_32,
            6 => PRESCALE_A::DIVIDE_BY_64,
            7 => PRESCALE_A::DIVIDE_BY_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_1`"]
    #[inline(always)]
    pub fn is_divide_by_1(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_2`"]
    #[inline(always)]
    pub fn is_divide_by_2(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_4`"]
    #[inline(always)]
    pub fn is_divide_by_4(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_8`"]
    #[inline(always)]
    pub fn is_divide_by_8(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_8
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_16`"]
    #[inline(always)]
    pub fn is_divide_by_16(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_32`"]
    #[inline(always)]
    pub fn is_divide_by_32(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_32
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_64`"]
    #[inline(always)]
    pub fn is_divide_by_64(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_64
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY_128`"]
    #[inline(always)]
    pub fn is_divide_by_128(&self) -> bool {
        *self == PRESCALE_A::DIVIDE_BY_128
    }
}
#[doc = "Field `PRESCALE` writer - Prescaler"]
pub type PRESCALE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCFGR1_SPEC, u8, PRESCALE_A, 3, O>;
impl<'a, const O: u8> PRESCALE_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_by_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_by_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_by_4(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_by_8(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divide_by_16(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divide_by_32(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_by_64(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn divide_by_128(self) -> &'a mut W {
        self.variant(PRESCALE_A::DIVIDE_BY_128)
    }
}
#[doc = "Field `AUTOSTOP` reader - Automatic STOP Generation"]
pub type AUTOSTOP_R = crate::BitReader<AUTOSTOP_A>;
#[doc = "Automatic STOP Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOSTOP_A {
    #[doc = "0: No effect"]
    DISABLED = 0,
    #[doc = "1: STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    ENABLED = 1,
}
impl From<AUTOSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSTOP_A {
        match self.bits {
            false => AUTOSTOP_A::DISABLED,
            true => AUTOSTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOSTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOSTOP_A::ENABLED
    }
}
#[doc = "Field `AUTOSTOP` writer - Automatic STOP Generation"]
pub type AUTOSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR1_SPEC, AUTOSTOP_A, O>;
impl<'a, const O: u8> AUTOSTOP_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::DISABLED)
    }
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and the LPI2C master is busy"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::ENABLED)
    }
}
#[doc = "Field `IGNACK` reader - IGNACK"]
pub type IGNACK_R = crate::BitReader<IGNACK_A>;
#[doc = "IGNACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IGNACK_A {
    #[doc = "0: LPI2C Master receives ACK and NACK normally"]
    DISABLED = 0,
    #[doc = "1: LPI2C Master treats a received NACK as if it (NACK) was an ACK"]
    ENABLED = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl IGNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::DISABLED,
            true => IGNACK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IGNACK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IGNACK_A::ENABLED
    }
}
#[doc = "Field `IGNACK` writer - IGNACK"]
pub type IGNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR1_SPEC, IGNACK_A, O>;
impl<'a, const O: u8> IGNACK_W<'a, O> {
    #[doc = "LPI2C Master receives ACK and NACK normally"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IGNACK_A::DISABLED)
    }
    #[doc = "LPI2C Master treats a received NACK as if it (NACK) was an ACK"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IGNACK_A::ENABLED)
    }
}
#[doc = "Field `TIMECFG` reader - Timeout Configuration"]
pub type TIMECFG_R = crate::BitReader<TIMECFG_A>;
#[doc = "Timeout Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMECFG_A {
    #[doc = "0: MSR\\[PLTF\\]
sets if SCL is low for longer than the configured timeout"]
    IF_SCL_LOW = 0,
    #[doc = "1: MSR\\[PLTF\\]
sets if either SCL or SDA is low for longer than the configured timeout"]
    IF_SCL_OR_SDA_LOW = 1,
}
impl From<TIMECFG_A> for bool {
    #[inline(always)]
    fn from(variant: TIMECFG_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMECFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMECFG_A {
        match self.bits {
            false => TIMECFG_A::IF_SCL_LOW,
            true => TIMECFG_A::IF_SCL_OR_SDA_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `IF_SCL_LOW`"]
    #[inline(always)]
    pub fn is_if_scl_low(&self) -> bool {
        *self == TIMECFG_A::IF_SCL_LOW
    }
    #[doc = "Checks if the value of the field is `IF_SCL_OR_SDA_LOW`"]
    #[inline(always)]
    pub fn is_if_scl_or_sda_low(&self) -> bool {
        *self == TIMECFG_A::IF_SCL_OR_SDA_LOW
    }
}
#[doc = "Field `TIMECFG` writer - Timeout Configuration"]
pub type TIMECFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR1_SPEC, TIMECFG_A, O>;
impl<'a, const O: u8> TIMECFG_W<'a, O> {
    #[doc = "MSR\\[PLTF\\]
sets if SCL is low for longer than the configured timeout"]
    #[inline(always)]
    pub fn if_scl_low(self) -> &'a mut W {
        self.variant(TIMECFG_A::IF_SCL_LOW)
    }
    #[doc = "MSR\\[PLTF\\]
sets if either SCL or SDA is low for longer than the configured timeout"]
    #[inline(always)]
    pub fn if_scl_or_sda_low(self) -> &'a mut W {
        self.variant(TIMECFG_A::IF_SCL_OR_SDA_LOW)
    }
}
#[doc = "Field `MATCFG` reader - Match Configuration"]
pub type MATCFG_R = crate::FieldReader<u8, MATCFG_A>;
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match is disabled"]
    DISABLED = 0,
    #[doc = "2: Match is enabled (1st data word equals MDMR\\[MATCH0\\]
OR MDMR\\[MATCH1\\])"]
    FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 2,
    #[doc = "3: Match is enabled (any data word equals MDMR\\[MATCH0\\]
OR MDMR\\[MATCH1\\])"]
    ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 3,
    #[doc = "4: Match is enabled (1st data word equals MDMR\\[MATCH0\\]
AND 2nd data word equals MDMR\\[MATCH1)"]
    FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1 = 4,
    #[doc = "5: Match is enabled (any data word equals MDMR\\[MATCH0\\]
AND next data word equals MDMR\\[MATCH1)"]
    ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1 = 5,
    #[doc = "6: Match is enabled (1st data word AND MDMR\\[MATCH1\\]
equals MDMR\\[MATCH0\\]
AND MDMR\\[MATCH1\\])"]
    FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1 = 6,
    #[doc = "7: Match is enabled (any data word AND MDMR\\[MATCH1\\]
equals MDMR\\[MATCH0\\]
AND MDMR\\[MATCH1\\])"]
    ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1 = 7,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
impl MATCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MATCFG_A> {
        match self.bits {
            0 => Some(MATCFG_A::DISABLED),
            2 => Some(MATCFG_A::FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1),
            3 => Some(MATCFG_A::ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1),
            4 => Some(MATCFG_A::FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1),
            5 => Some(MATCFG_A::ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1),
            6 => Some(MATCFG_A::FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1),
            7 => Some(MATCFG_A::ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MATCFG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1`"]
    #[inline(always)]
    pub fn is_first_data_word_equals_match0_or_match1(&self) -> bool {
        *self == MATCFG_A::FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1
    }
    #[doc = "Checks if the value of the field is `ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1`"]
    #[inline(always)]
    pub fn is_any_data_word_equals_match0_or_match1(&self) -> bool {
        *self == MATCFG_A::ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1
    }
    #[doc = "Checks if the value of the field is `FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1`"]
    #[inline(always)]
    pub fn is_first_data_word_match0_and_second_data_word_match1(&self) -> bool {
        *self == MATCFG_A::FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1
    }
    #[doc = "Checks if the value of the field is `ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1`"]
    #[inline(always)]
    pub fn is_any_data_word_match0_next_data_word_match1(&self) -> bool {
        *self == MATCFG_A::ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1
    }
    #[doc = "Checks if the value of the field is `FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1`"]
    #[inline(always)]
    pub fn is_first_data_word_and_match1_equals_match0_and_match1(&self) -> bool {
        *self == MATCFG_A::FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1
    }
    #[doc = "Checks if the value of the field is `ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1`"]
    #[inline(always)]
    pub fn is_any_data_word_and_match1_equals_match0_and_match1(&self) -> bool {
        *self == MATCFG_A::ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1
    }
}
#[doc = "Field `MATCFG` writer - Match Configuration"]
pub type MATCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFGR1_SPEC, u8, MATCFG_A, 3, O>;
impl<'a, const O: u8> MATCFG_W<'a, O> {
    #[doc = "Match is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MATCFG_A::DISABLED)
    }
    #[doc = "Match is enabled (1st data word equals MDMR\\[MATCH0\\]
OR MDMR\\[MATCH1\\])"]
    #[inline(always)]
    pub fn first_data_word_equals_match0_or_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1)
    }
    #[doc = "Match is enabled (any data word equals MDMR\\[MATCH0\\]
OR MDMR\\[MATCH1\\])"]
    #[inline(always)]
    pub fn any_data_word_equals_match0_or_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1)
    }
    #[doc = "Match is enabled (1st data word equals MDMR\\[MATCH0\\]
AND 2nd data word equals MDMR\\[MATCH1)"]
    #[inline(always)]
    pub fn first_data_word_match0_and_second_data_word_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1)
    }
    #[doc = "Match is enabled (any data word equals MDMR\\[MATCH0\\]
AND next data word equals MDMR\\[MATCH1)"]
    #[inline(always)]
    pub fn any_data_word_match0_next_data_word_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1)
    }
    #[doc = "Match is enabled (1st data word AND MDMR\\[MATCH1\\]
equals MDMR\\[MATCH0\\]
AND MDMR\\[MATCH1\\])"]
    #[inline(always)]
    pub fn first_data_word_and_match1_equals_match0_and_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1)
    }
    #[doc = "Match is enabled (any data word AND MDMR\\[MATCH1\\]
equals MDMR\\[MATCH0\\]
AND MDMR\\[MATCH1\\])"]
    #[inline(always)]
    pub fn any_data_word_and_match1_equals_match0_and_match1(self) -> &'a mut W {
        self.variant(MATCFG_A::ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1)
    }
}
#[doc = "Field `PINCFG` reader - Pin Configuration"]
pub type PINCFG_R = crate::FieldReader<u8, PINCFG_A>;
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: 2-pin open drain mode"]
    OPEN_DRAIN_2_PIN = 0,
    #[doc = "1: 2-pin output only mode (ultra-fast mode)"]
    OUTPUT_2_PIN_ONLY = 1,
    #[doc = "2: 2-pin push-pull mode"]
    PUSH_PULL_2_PIN = 2,
    #[doc = "3: 4-pin push-pull mode"]
    PUSH_PULL_4_PIN = 3,
    #[doc = "4: 2-pin open drain mode with separate LPI2C slave"]
    OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE = 4,
    #[doc = "5: 2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE = 5,
    #[doc = "6: 2-pin push-pull mode with separate LPI2C slave"]
    PUSH_PULL_2_PIN_W_LPI2C_SLAVE = 6,
    #[doc = "7: 4-pin push-pull mode (inverted outputs)"]
    PUSH_PULL_4_PIN_W_LPI2C_SLAVE = 7,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
impl PINCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::OPEN_DRAIN_2_PIN,
            1 => PINCFG_A::OUTPUT_2_PIN_ONLY,
            2 => PINCFG_A::PUSH_PULL_2_PIN,
            3 => PINCFG_A::PUSH_PULL_4_PIN,
            4 => PINCFG_A::OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE,
            5 => PINCFG_A::OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE,
            6 => PINCFG_A::PUSH_PULL_2_PIN_W_LPI2C_SLAVE,
            7 => PINCFG_A::PUSH_PULL_4_PIN_W_LPI2C_SLAVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_2_PIN`"]
    #[inline(always)]
    pub fn is_open_drain_2_pin(&self) -> bool {
        *self == PINCFG_A::OPEN_DRAIN_2_PIN
    }
    #[doc = "Checks if the value of the field is `OUTPUT_2_PIN_ONLY`"]
    #[inline(always)]
    pub fn is_output_2_pin_only(&self) -> bool {
        *self == PINCFG_A::OUTPUT_2_PIN_ONLY
    }
    #[doc = "Checks if the value of the field is `PUSH_PULL_2_PIN`"]
    #[inline(always)]
    pub fn is_push_pull_2_pin(&self) -> bool {
        *self == PINCFG_A::PUSH_PULL_2_PIN
    }
    #[doc = "Checks if the value of the field is `PUSH_PULL_4_PIN`"]
    #[inline(always)]
    pub fn is_push_pull_4_pin(&self) -> bool {
        *self == PINCFG_A::PUSH_PULL_4_PIN
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE`"]
    #[inline(always)]
    pub fn is_open_drain_2_pin_w_lpi2c_slave(&self) -> bool {
        *self == PINCFG_A::OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE`"]
    #[inline(always)]
    pub fn is_output_2_pin_only_w_lpi2c_slave(&self) -> bool {
        *self == PINCFG_A::OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `PUSH_PULL_2_PIN_W_LPI2C_SLAVE`"]
    #[inline(always)]
    pub fn is_push_pull_2_pin_w_lpi2c_slave(&self) -> bool {
        *self == PINCFG_A::PUSH_PULL_2_PIN_W_LPI2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `PUSH_PULL_4_PIN_W_LPI2C_SLAVE`"]
    #[inline(always)]
    pub fn is_push_pull_4_pin_w_lpi2c_slave(&self) -> bool {
        *self == PINCFG_A::PUSH_PULL_4_PIN_W_LPI2C_SLAVE
    }
}
#[doc = "Field `PINCFG` writer - Pin Configuration"]
pub type PINCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCFGR1_SPEC, u8, PINCFG_A, 3, O>;
impl<'a, const O: u8> PINCFG_W<'a, O> {
    #[doc = "2-pin open drain mode"]
    #[inline(always)]
    pub fn open_drain_2_pin(self) -> &'a mut W {
        self.variant(PINCFG_A::OPEN_DRAIN_2_PIN)
    }
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    #[inline(always)]
    pub fn output_2_pin_only(self) -> &'a mut W {
        self.variant(PINCFG_A::OUTPUT_2_PIN_ONLY)
    }
    #[doc = "2-pin push-pull mode"]
    #[inline(always)]
    pub fn push_pull_2_pin(self) -> &'a mut W {
        self.variant(PINCFG_A::PUSH_PULL_2_PIN)
    }
    #[doc = "4-pin push-pull mode"]
    #[inline(always)]
    pub fn push_pull_4_pin(self) -> &'a mut W {
        self.variant(PINCFG_A::PUSH_PULL_4_PIN)
    }
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    #[inline(always)]
    pub fn open_drain_2_pin_w_lpi2c_slave(self) -> &'a mut W {
        self.variant(PINCFG_A::OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE)
    }
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    #[inline(always)]
    pub fn output_2_pin_only_w_lpi2c_slave(self) -> &'a mut W {
        self.variant(PINCFG_A::OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE)
    }
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    #[inline(always)]
    pub fn push_pull_2_pin_w_lpi2c_slave(self) -> &'a mut W {
        self.variant(PINCFG_A::PUSH_PULL_2_PIN_W_LPI2C_SLAVE)
    }
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    #[inline(always)]
    pub fn push_pull_4_pin_w_lpi2c_slave(self) -> &'a mut W {
        self.variant(PINCFG_A::PUSH_PULL_4_PIN_W_LPI2C_SLAVE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    pub fn timecfg(&self) -> TIMECFG_R {
        TIMECFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<0> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    #[must_use]
    pub fn autostop(&mut self) -> AUTOSTOP_W<8> {
        AUTOSTOP_W::new(self)
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    #[must_use]
    pub fn ignack(&mut self) -> IGNACK_W<9> {
        IGNACK_W::new(self)
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn timecfg(&mut self) -> TIMECFG_W<10> {
        TIMECFG_W::new(self)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn matcfg(&mut self) -> MATCFG_W<16> {
        MATCFG_W::new(self)
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PINCFG_W<24> {
        PINCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr1](index.html) module"]
pub struct MCFGR1_SPEC;
impl crate::RegisterSpec for MCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr1::R](R) reader structure"]
impl crate::Readable for MCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr1::W](W) writer structure"]
impl crate::Writable for MCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFGR1 to value 0"]
impl crate::Resettable for MCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
