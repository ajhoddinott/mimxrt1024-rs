#[doc = "Register `HPCR` reader"]
pub struct R(crate::R<HPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPCR` writer"]
pub struct W(crate::W<HPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPCR_SPEC>;
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
impl From<crate::W<HPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_EN` reader - HP Real Time Counter Enable"]
pub type RTC_EN_R = crate::BitReader<RTC_EN_A>;
#[doc = "HP Real Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_EN_A {
    #[doc = "0: RTC is disabled"]
    DISABLED = 0,
    #[doc = "1: RTC is enabled"]
    ENABLED = 1,
}
impl From<RTC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_EN_A {
        match self.bits {
            false => RTC_EN_A::DISABLED,
            true => RTC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTC_EN_A::ENABLED
    }
}
#[doc = "Field `RTC_EN` writer - HP Real Time Counter Enable"]
pub type RTC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, RTC_EN_A, O>;
impl<'a, const O: u8> RTC_EN_W<'a, O> {
    #[doc = "RTC is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTC_EN_A::DISABLED)
    }
    #[doc = "RTC is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTC_EN_A::ENABLED)
    }
}
#[doc = "Field `HPTA_EN` reader - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
pub type HPTA_EN_R = crate::BitReader<HPTA_EN_A>;
#[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPTA_EN_A {
    #[doc = "0: HP Time Alarm Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: HP Time Alarm Interrupt is enabled"]
    ENABLED = 1,
}
impl From<HPTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPTA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPTA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPTA_EN_A {
        match self.bits {
            false => HPTA_EN_A::DISABLED,
            true => HPTA_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HPTA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HPTA_EN_A::ENABLED
    }
}
#[doc = "Field `HPTA_EN` writer - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
pub type HPTA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, HPTA_EN_A, O>;
impl<'a, const O: u8> HPTA_EN_W<'a, O> {
    #[doc = "HP Time Alarm Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HPTA_EN_A::DISABLED)
    }
    #[doc = "HP Time Alarm Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HPTA_EN_A::ENABLED)
    }
}
#[doc = "Field `DIS_PI` reader - Disable periodic interrupt in the functional interrupt"]
pub type DIS_PI_R = crate::BitReader<DIS_PI_A>;
#[doc = "Disable periodic interrupt in the functional interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_PI_A {
    #[doc = "0: Periodic interrupt will trigger a functional interrupt"]
    ENABLED = 0,
    #[doc = "1: Disable periodic interrupt in the function interrupt"]
    DISABLED = 1,
}
impl From<DIS_PI_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_PI_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_PI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_PI_A {
        match self.bits {
            false => DIS_PI_A::ENABLED,
            true => DIS_PI_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIS_PI_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIS_PI_A::DISABLED
    }
}
#[doc = "Field `DIS_PI` writer - Disable periodic interrupt in the functional interrupt"]
pub type DIS_PI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, DIS_PI_A, O>;
impl<'a, const O: u8> DIS_PI_W<'a, O> {
    #[doc = "Periodic interrupt will trigger a functional interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIS_PI_A::ENABLED)
    }
    #[doc = "Disable periodic interrupt in the function interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIS_PI_A::DISABLED)
    }
}
#[doc = "Field `PI_EN` reader - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
pub type PI_EN_R = crate::BitReader<PI_EN_A>;
#[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_EN_A {
    #[doc = "0: HP Periodic Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: HP Periodic Interrupt is enabled"]
    ENABLED = 1,
}
impl From<PI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_EN_A {
        match self.bits {
            false => PI_EN_A::DISABLED,
            true => PI_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PI_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PI_EN_A::ENABLED
    }
}
#[doc = "Field `PI_EN` writer - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
pub type PI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, PI_EN_A, O>;
impl<'a, const O: u8> PI_EN_W<'a, O> {
    #[doc = "HP Periodic Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PI_EN_A::DISABLED)
    }
    #[doc = "HP Periodic Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PI_EN_A::ENABLED)
    }
}
#[doc = "Field `PI_FREQ` reader - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
pub type PI_FREQ_R = crate::FieldReader<u8, PI_FREQ_A>;
#[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PI_FREQ_A {
    #[doc = "0: - bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_0 = 0,
    #[doc = "1: - bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1 = 1,
    #[doc = "2: - bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_2 = 2,
    #[doc = "3: - bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_3 = 3,
    #[doc = "4: - bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_4 = 4,
    #[doc = "5: - bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_5 = 5,
    #[doc = "6: - bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_6 = 6,
    #[doc = "7: - bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_7 = 7,
    #[doc = "8: - bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_8 = 8,
    #[doc = "9: - bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_9 = 9,
    #[doc = "10: - bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_10 = 10,
    #[doc = "11: - bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_11 = 11,
    #[doc = "12: - bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_12 = 12,
    #[doc = "13: - bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_13 = 13,
    #[doc = "14: - bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_14 = 14,
    #[doc = "15: - bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1R5 = 15,
}
impl From<PI_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PI_FREQ_A) -> Self {
        variant as _
    }
}
impl PI_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_FREQ_A {
        match self.bits {
            0 => PI_FREQ_A::USE_BIT_0,
            1 => PI_FREQ_A::USE_BIT_1,
            2 => PI_FREQ_A::USE_BIT_2,
            3 => PI_FREQ_A::USE_BIT_3,
            4 => PI_FREQ_A::USE_BIT_4,
            5 => PI_FREQ_A::USE_BIT_5,
            6 => PI_FREQ_A::USE_BIT_6,
            7 => PI_FREQ_A::USE_BIT_7,
            8 => PI_FREQ_A::USE_BIT_8,
            9 => PI_FREQ_A::USE_BIT_9,
            10 => PI_FREQ_A::USE_BIT_10,
            11 => PI_FREQ_A::USE_BIT_11,
            12 => PI_FREQ_A::USE_BIT_12,
            13 => PI_FREQ_A::USE_BIT_13,
            14 => PI_FREQ_A::USE_BIT_14,
            15 => PI_FREQ_A::USE_BIT_1R5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USE_BIT_0`"]
    #[inline(always)]
    pub fn is_use_bit_0(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_0
    }
    #[doc = "Checks if the value of the field is `USE_BIT_1`"]
    #[inline(always)]
    pub fn is_use_bit_1(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_1
    }
    #[doc = "Checks if the value of the field is `USE_BIT_2`"]
    #[inline(always)]
    pub fn is_use_bit_2(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_2
    }
    #[doc = "Checks if the value of the field is `USE_BIT_3`"]
    #[inline(always)]
    pub fn is_use_bit_3(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_3
    }
    #[doc = "Checks if the value of the field is `USE_BIT_4`"]
    #[inline(always)]
    pub fn is_use_bit_4(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_4
    }
    #[doc = "Checks if the value of the field is `USE_BIT_5`"]
    #[inline(always)]
    pub fn is_use_bit_5(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_5
    }
    #[doc = "Checks if the value of the field is `USE_BIT_6`"]
    #[inline(always)]
    pub fn is_use_bit_6(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_6
    }
    #[doc = "Checks if the value of the field is `USE_BIT_7`"]
    #[inline(always)]
    pub fn is_use_bit_7(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_7
    }
    #[doc = "Checks if the value of the field is `USE_BIT_8`"]
    #[inline(always)]
    pub fn is_use_bit_8(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_8
    }
    #[doc = "Checks if the value of the field is `USE_BIT_9`"]
    #[inline(always)]
    pub fn is_use_bit_9(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_9
    }
    #[doc = "Checks if the value of the field is `USE_BIT_10`"]
    #[inline(always)]
    pub fn is_use_bit_10(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_10
    }
    #[doc = "Checks if the value of the field is `USE_BIT_11`"]
    #[inline(always)]
    pub fn is_use_bit_11(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_11
    }
    #[doc = "Checks if the value of the field is `USE_BIT_12`"]
    #[inline(always)]
    pub fn is_use_bit_12(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_12
    }
    #[doc = "Checks if the value of the field is `USE_BIT_13`"]
    #[inline(always)]
    pub fn is_use_bit_13(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_13
    }
    #[doc = "Checks if the value of the field is `USE_BIT_14`"]
    #[inline(always)]
    pub fn is_use_bit_14(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_14
    }
    #[doc = "Checks if the value of the field is `USE_BIT_1R5`"]
    #[inline(always)]
    pub fn is_use_bit_1r5(&self) -> bool {
        *self == PI_FREQ_A::USE_BIT_1R5
    }
}
#[doc = "Field `PI_FREQ` writer - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
pub type PI_FREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HPCR_SPEC, u8, PI_FREQ_A, 4, O>;
impl<'a, const O: u8> PI_FREQ_W<'a, O> {
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_0(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_0)
    }
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_1(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_1)
    }
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_2(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_2)
    }
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_3(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_3)
    }
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_4(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_4)
    }
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_5(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_5)
    }
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_6(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_6)
    }
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_7(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_7)
    }
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_8(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_8)
    }
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_9(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_9)
    }
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_10(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_10)
    }
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_11(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_11)
    }
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_12(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_12)
    }
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_13(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_13)
    }
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_14(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_14)
    }
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn use_bit_1r5(self) -> &'a mut W {
        self.variant(PI_FREQ_A::USE_BIT_1R5)
    }
}
#[doc = "Field `HPCALB_EN` reader - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
pub type HPCALB_EN_R = crate::BitReader<HPCALB_EN_A>;
#[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPCALB_EN_A {
    #[doc = "0: HP Timer calibration disabled"]
    DISABLED = 0,
    #[doc = "1: HP Timer calibration enabled"]
    ENABLED = 1,
}
impl From<HPCALB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPCALB_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HPCALB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCALB_EN_A {
        match self.bits {
            false => HPCALB_EN_A::DISABLED,
            true => HPCALB_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HPCALB_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HPCALB_EN_A::ENABLED
    }
}
#[doc = "Field `HPCALB_EN` writer - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
pub type HPCALB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, HPCALB_EN_A, O>;
impl<'a, const O: u8> HPCALB_EN_W<'a, O> {
    #[doc = "HP Timer calibration disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HPCALB_EN_A::DISABLED)
    }
    #[doc = "HP Timer calibration enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HPCALB_EN_A::ENABLED)
    }
}
#[doc = "Field `HPCALB_VAL` reader - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
pub type HPCALB_VAL_R = crate::FieldReader<u8, HPCALB_VAL_A>;
#[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPCALB_VAL_A {
    #[doc = "0: +0 counts per each 32768 ticks of the counter"]
    ADD_0_PER_32768_TICKS = 0,
    #[doc = "1: +1 counts per each 32768 ticks of the counter"]
    ADD_1_PER_32768_TICKS = 1,
    #[doc = "2: +2 counts per each 32768 ticks of the counter"]
    ADD_2_PER_32768_TICKS = 2,
    #[doc = "15: +15 counts per each 32768 ticks of the counter"]
    ADD_15_PER_32768_TICKS = 15,
    #[doc = "16: -16 counts per each 32768 ticks of the counter"]
    SUB_16_PER_32768_TICKS = 16,
    #[doc = "17: -15 counts per each 32768 ticks of the counter"]
    SUB_15_PER_32768_TICKS = 17,
    #[doc = "30: -2 counts per each 32768 ticks of the counter"]
    SUB_2_PER_32768_TICKS = 30,
    #[doc = "31: -1 counts per each 32768 ticks of the counter"]
    SUB_1_PER_32768_TICKS = 31,
}
impl From<HPCALB_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: HPCALB_VAL_A) -> Self {
        variant as _
    }
}
impl HPCALB_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPCALB_VAL_A> {
        match self.bits {
            0 => Some(HPCALB_VAL_A::ADD_0_PER_32768_TICKS),
            1 => Some(HPCALB_VAL_A::ADD_1_PER_32768_TICKS),
            2 => Some(HPCALB_VAL_A::ADD_2_PER_32768_TICKS),
            15 => Some(HPCALB_VAL_A::ADD_15_PER_32768_TICKS),
            16 => Some(HPCALB_VAL_A::SUB_16_PER_32768_TICKS),
            17 => Some(HPCALB_VAL_A::SUB_15_PER_32768_TICKS),
            30 => Some(HPCALB_VAL_A::SUB_2_PER_32768_TICKS),
            31 => Some(HPCALB_VAL_A::SUB_1_PER_32768_TICKS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADD_0_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_0_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::ADD_0_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_1_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_1_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::ADD_1_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_2_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_2_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::ADD_2_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `ADD_15_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_add_15_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::ADD_15_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_16_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_16_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::SUB_16_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_15_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_15_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::SUB_15_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_2_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_2_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::SUB_2_PER_32768_TICKS
    }
    #[doc = "Checks if the value of the field is `SUB_1_PER_32768_TICKS`"]
    #[inline(always)]
    pub fn is_sub_1_per_32768_ticks(&self) -> bool {
        *self == HPCALB_VAL_A::SUB_1_PER_32768_TICKS
    }
}
#[doc = "Field `HPCALB_VAL` writer - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
pub type HPCALB_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPCR_SPEC, u8, HPCALB_VAL_A, 5, O>;
impl<'a, const O: u8> HPCALB_VAL_W<'a, O> {
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn add_0_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::ADD_0_PER_32768_TICKS)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn add_1_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::ADD_1_PER_32768_TICKS)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn add_2_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::ADD_2_PER_32768_TICKS)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn add_15_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::ADD_15_PER_32768_TICKS)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn sub_16_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::SUB_16_PER_32768_TICKS)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn sub_15_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::SUB_15_PER_32768_TICKS)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn sub_2_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::SUB_2_PER_32768_TICKS)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn sub_1_per_32768_ticks(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::SUB_1_PER_32768_TICKS)
    }
}
#[doc = "Field `HP_TS` reader - HP Time Synchronize"]
pub type HP_TS_R = crate::BitReader<HP_TS_A>;
#[doc = "HP Time Synchronize\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_TS_A {
    #[doc = "0: No Action"]
    NO_ACTION = 0,
    #[doc = "1: Synchronize the HP Time Counter to the LP Time Counter"]
    SYNC_TIME = 1,
}
impl From<HP_TS_A> for bool {
    #[inline(always)]
    fn from(variant: HP_TS_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_TS_A {
        match self.bits {
            false => HP_TS_A::NO_ACTION,
            true => HP_TS_A::SYNC_TIME,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACTION`"]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == HP_TS_A::NO_ACTION
    }
    #[doc = "Checks if the value of the field is `SYNC_TIME`"]
    #[inline(always)]
    pub fn is_sync_time(&self) -> bool {
        *self == HP_TS_A::SYNC_TIME
    }
}
#[doc = "Field `HP_TS` writer - HP Time Synchronize"]
pub type HP_TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, HP_TS_A, O>;
impl<'a, const O: u8> HP_TS_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(HP_TS_A::NO_ACTION)
    }
    #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
    #[inline(always)]
    pub fn sync_time(self) -> &'a mut W {
        self.variant(HP_TS_A::SYNC_TIME)
    }
}
#[doc = "Field `BTN_CONFIG` reader - Button Configuration"]
pub type BTN_CONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BTN_CONFIG` writer - Button Configuration"]
pub type BTN_CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BTN_MASK` reader - Button interrupt mask"]
pub type BTN_MASK_R = crate::BitReader<bool>;
#[doc = "Field `BTN_MASK` writer - Button interrupt mask"]
pub type BTN_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpta_en(&self) -> HPTA_EN_R {
        HPTA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub fn dis_pi(&self) -> DIS_PI_R {
        DIS_PI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub fn pi_en(&self) -> PI_EN_R {
        PI_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq(&self) -> PI_FREQ_R {
        PI_FREQ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub fn hpcalb_en(&self) -> HPCALB_EN_R {
        HPCALB_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpcalb_val(&self) -> HPCALB_VAL_R {
        HPCALB_VAL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline(always)]
    pub fn hp_ts(&self) -> HP_TS_R {
        HP_TS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline(always)]
    pub fn btn_config(&self) -> BTN_CONFIG_R {
        BTN_CONFIG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline(always)]
    pub fn btn_mask(&self) -> BTN_MASK_R {
        BTN_MASK_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_en(&mut self) -> RTC_EN_W<0> {
        RTC_EN_W::new(self)
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn hpta_en(&mut self) -> HPTA_EN_W<1> {
        HPTA_EN_W::new(self)
    }
    #[doc = "Bit 2 - Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dis_pi(&mut self) -> DIS_PI_W<2> {
        DIS_PI_W::new(self)
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pi_en(&mut self) -> PI_EN_W<3> {
        PI_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq(&mut self) -> PI_FREQ_W<4> {
        PI_FREQ_W::new(self)
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn hpcalb_en(&mut self) -> HPCALB_EN_W<8> {
        HPCALB_EN_W::new(self)
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    #[must_use]
    pub fn hpcalb_val(&mut self) -> HPCALB_VAL_W<10> {
        HPCALB_VAL_W::new(self)
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline(always)]
    #[must_use]
    pub fn hp_ts(&mut self) -> HP_TS_W<16> {
        HP_TS_W::new(self)
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn btn_config(&mut self) -> BTN_CONFIG_W<24> {
        BTN_CONFIG_W::new(self)
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn btn_mask(&mut self) -> BTN_MASK_W<27> {
        BTN_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcr](index.html) module"]
pub struct HPCR_SPEC;
impl crate::RegisterSpec for HPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpcr::R](R) reader structure"]
impl crate::Readable for HPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpcr::W](W) writer structure"]
impl crate::Writable for HPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPCR to value 0"]
impl crate::Resettable for HPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
