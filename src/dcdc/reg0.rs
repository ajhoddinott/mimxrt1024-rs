#[doc = "Register `REG0` reader"]
pub struct R(crate::R<REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG0` writer"]
pub struct W(crate::W<REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG0_SPEC>;
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
impl From<crate::W<REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWD_ZCD` reader - Power Down Zero Cross Detection"]
pub type PWD_ZCD_R = crate::BitReader<PWD_ZCD_A>;
#[doc = "Power Down Zero Cross Detection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_ZCD_A {
    #[doc = "0: Zero cross detetion function powered up"]
    POWERED_UP = 0,
    #[doc = "1: Zero cross detetion function powered down"]
    POWERED_DOWN = 1,
}
impl From<PWD_ZCD_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_ZCD_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_ZCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_ZCD_A {
        match self.bits {
            false => PWD_ZCD_A::POWERED_UP,
            true => PWD_ZCD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_UP`"]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == PWD_ZCD_A::POWERED_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWD_ZCD_A::POWERED_DOWN
    }
}
#[doc = "Field `PWD_ZCD` writer - Power Down Zero Cross Detection"]
pub type PWD_ZCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG0_SPEC, PWD_ZCD_A, O>;
impl<'a, const O: u8> PWD_ZCD_W<'a, O> {
    #[doc = "Zero cross detetion function powered up"]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut W {
        self.variant(PWD_ZCD_A::POWERED_UP)
    }
    #[doc = "Zero cross detetion function powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWD_ZCD_A::POWERED_DOWN)
    }
}
#[doc = "Field `DISABLE_AUTO_CLK_SWITCH` reader - Disable Auto Clock Switch"]
pub type DISABLE_AUTO_CLK_SWITCH_R = crate::BitReader<DISABLE_AUTO_CLK_SWITCH_A>;
#[doc = "Disable Auto Clock Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLE_AUTO_CLK_SWITCH_A {
    #[doc = "0: If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring OSC to 24M xtal automatically"]
    XTAL_CLK = 0,
    #[doc = "1: If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses"]
    SEL_CLK = 1,
}
impl From<DISABLE_AUTO_CLK_SWITCH_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_AUTO_CLK_SWITCH_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLE_AUTO_CLK_SWITCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_AUTO_CLK_SWITCH_A {
        match self.bits {
            false => DISABLE_AUTO_CLK_SWITCH_A::XTAL_CLK,
            true => DISABLE_AUTO_CLK_SWITCH_A::SEL_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL_CLK`"]
    #[inline(always)]
    pub fn is_xtal_clk(&self) -> bool {
        *self == DISABLE_AUTO_CLK_SWITCH_A::XTAL_CLK
    }
    #[doc = "Checks if the value of the field is `SEL_CLK`"]
    #[inline(always)]
    pub fn is_sel_clk(&self) -> bool {
        *self == DISABLE_AUTO_CLK_SWITCH_A::SEL_CLK
    }
}
#[doc = "Field `DISABLE_AUTO_CLK_SWITCH` writer - Disable Auto Clock Switch"]
pub type DISABLE_AUTO_CLK_SWITCH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, DISABLE_AUTO_CLK_SWITCH_A, O>;
impl<'a, const O: u8> DISABLE_AUTO_CLK_SWITCH_W<'a, O> {
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring OSC to 24M xtal automatically"]
    #[inline(always)]
    pub fn xtal_clk(self) -> &'a mut W {
        self.variant(DISABLE_AUTO_CLK_SWITCH_A::XTAL_CLK)
    }
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses"]
    #[inline(always)]
    pub fn sel_clk(self) -> &'a mut W {
        self.variant(DISABLE_AUTO_CLK_SWITCH_A::SEL_CLK)
    }
}
#[doc = "Field `SEL_CLK` reader - Select Clock"]
pub type SEL_CLK_R = crate::BitReader<SEL_CLK_A>;
#[doc = "Select Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_CLK_A {
    #[doc = "0: DCDC uses internal ring oscillator"]
    INT_RNG_OSC = 0,
    #[doc = "1: DCDC uses 24M xtal"]
    XTAL_24M = 1,
}
impl From<SEL_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_CLK_A {
        match self.bits {
            false => SEL_CLK_A::INT_RNG_OSC,
            true => SEL_CLK_A::XTAL_24M,
        }
    }
    #[doc = "Checks if the value of the field is `INT_RNG_OSC`"]
    #[inline(always)]
    pub fn is_int_rng_osc(&self) -> bool {
        *self == SEL_CLK_A::INT_RNG_OSC
    }
    #[doc = "Checks if the value of the field is `XTAL_24M`"]
    #[inline(always)]
    pub fn is_xtal_24m(&self) -> bool {
        *self == SEL_CLK_A::XTAL_24M
    }
}
#[doc = "Field `SEL_CLK` writer - Select Clock"]
pub type SEL_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG0_SPEC, SEL_CLK_A, O>;
impl<'a, const O: u8> SEL_CLK_W<'a, O> {
    #[doc = "DCDC uses internal ring oscillator"]
    #[inline(always)]
    pub fn int_rng_osc(self) -> &'a mut W {
        self.variant(SEL_CLK_A::INT_RNG_OSC)
    }
    #[doc = "DCDC uses 24M xtal"]
    #[inline(always)]
    pub fn xtal_24m(self) -> &'a mut W {
        self.variant(SEL_CLK_A::XTAL_24M)
    }
}
#[doc = "Field `PWD_OSC_INT` reader - Power down internal osc"]
pub type PWD_OSC_INT_R = crate::BitReader<PWD_OSC_INT_A>;
#[doc = "Power down internal osc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_OSC_INT_A {
    #[doc = "0: Internal oscillator powered up"]
    POWERED_UP = 0,
    #[doc = "1: Internal oscillator powered down"]
    POWERED_DOWN = 1,
}
impl From<PWD_OSC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_OSC_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_OSC_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_OSC_INT_A {
        match self.bits {
            false => PWD_OSC_INT_A::POWERED_UP,
            true => PWD_OSC_INT_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_UP`"]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == PWD_OSC_INT_A::POWERED_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWD_OSC_INT_A::POWERED_DOWN
    }
}
#[doc = "Field `PWD_OSC_INT` writer - Power down internal osc"]
pub type PWD_OSC_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG0_SPEC, PWD_OSC_INT_A, O>;
impl<'a, const O: u8> PWD_OSC_INT_W<'a, O> {
    #[doc = "Internal oscillator powered up"]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut W {
        self.variant(PWD_OSC_INT_A::POWERED_UP)
    }
    #[doc = "Internal oscillator powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWD_OSC_INT_A::POWERED_DOWN)
    }
}
#[doc = "Field `PWD_CUR_SNS_CMP` reader - Power down signal of the current detector."]
pub type PWD_CUR_SNS_CMP_R = crate::BitReader<PWD_CUR_SNS_CMP_A>;
#[doc = "Power down signal of the current detector.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_CUR_SNS_CMP_A {
    #[doc = "0: Current Detector powered up"]
    POWERED_UP = 0,
    #[doc = "1: Current Detector powered down"]
    POWERED_DOWN = 1,
}
impl From<PWD_CUR_SNS_CMP_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_CUR_SNS_CMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_CUR_SNS_CMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_CUR_SNS_CMP_A {
        match self.bits {
            false => PWD_CUR_SNS_CMP_A::POWERED_UP,
            true => PWD_CUR_SNS_CMP_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_UP`"]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == PWD_CUR_SNS_CMP_A::POWERED_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWD_CUR_SNS_CMP_A::POWERED_DOWN
    }
}
#[doc = "Field `PWD_CUR_SNS_CMP` writer - Power down signal of the current detector."]
pub type PWD_CUR_SNS_CMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, PWD_CUR_SNS_CMP_A, O>;
impl<'a, const O: u8> PWD_CUR_SNS_CMP_W<'a, O> {
    #[doc = "Current Detector powered up"]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut W {
        self.variant(PWD_CUR_SNS_CMP_A::POWERED_UP)
    }
    #[doc = "Current Detector powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWD_CUR_SNS_CMP_A::POWERED_DOWN)
    }
}
#[doc = "Field `CUR_SNS_THRSH` reader - Current Sense (detector) Threshold"]
pub type CUR_SNS_THRSH_R = crate::FieldReader<u8, CUR_SNS_THRSH_A>;
#[doc = "Current Sense (detector) Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CUR_SNS_THRSH_A {
    #[doc = "0: 150 mA"]
    SELECT_ZERO = 0,
    #[doc = "1: 250 mA"]
    SELECT_ONE = 1,
    #[doc = "2: 350 mA"]
    SELECT_TWO = 2,
    #[doc = "3: 450 mA"]
    SELECT_THREE = 3,
    #[doc = "4: 550 mA"]
    SELECT_FOUR = 4,
    #[doc = "5: 650 mA"]
    SELECT_FIVE = 5,
}
impl From<CUR_SNS_THRSH_A> for u8 {
    #[inline(always)]
    fn from(variant: CUR_SNS_THRSH_A) -> Self {
        variant as _
    }
}
impl CUR_SNS_THRSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CUR_SNS_THRSH_A> {
        match self.bits {
            0 => Some(CUR_SNS_THRSH_A::SELECT_ZERO),
            1 => Some(CUR_SNS_THRSH_A::SELECT_ONE),
            2 => Some(CUR_SNS_THRSH_A::SELECT_TWO),
            3 => Some(CUR_SNS_THRSH_A::SELECT_THREE),
            4 => Some(CUR_SNS_THRSH_A::SELECT_FOUR),
            5 => Some(CUR_SNS_THRSH_A::SELECT_FIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_ZERO`"]
    #[inline(always)]
    pub fn is_select_zero(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_ZERO
    }
    #[doc = "Checks if the value of the field is `SELECT_ONE`"]
    #[inline(always)]
    pub fn is_select_one(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_ONE
    }
    #[doc = "Checks if the value of the field is `SELECT_TWO`"]
    #[inline(always)]
    pub fn is_select_two(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_TWO
    }
    #[doc = "Checks if the value of the field is `SELECT_THREE`"]
    #[inline(always)]
    pub fn is_select_three(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_THREE
    }
    #[doc = "Checks if the value of the field is `SELECT_FOUR`"]
    #[inline(always)]
    pub fn is_select_four(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_FOUR
    }
    #[doc = "Checks if the value of the field is `SELECT_FIVE`"]
    #[inline(always)]
    pub fn is_select_five(&self) -> bool {
        *self == CUR_SNS_THRSH_A::SELECT_FIVE
    }
}
#[doc = "Field `CUR_SNS_THRSH` writer - Current Sense (detector) Threshold"]
pub type CUR_SNS_THRSH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG0_SPEC, u8, CUR_SNS_THRSH_A, 3, O>;
impl<'a, const O: u8> CUR_SNS_THRSH_W<'a, O> {
    #[doc = "150 mA"]
    #[inline(always)]
    pub fn select_zero(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_ZERO)
    }
    #[doc = "250 mA"]
    #[inline(always)]
    pub fn select_one(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_ONE)
    }
    #[doc = "350 mA"]
    #[inline(always)]
    pub fn select_two(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_TWO)
    }
    #[doc = "450 mA"]
    #[inline(always)]
    pub fn select_three(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_THREE)
    }
    #[doc = "550 mA"]
    #[inline(always)]
    pub fn select_four(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_FOUR)
    }
    #[doc = "650 mA"]
    #[inline(always)]
    pub fn select_five(self) -> &'a mut W {
        self.variant(CUR_SNS_THRSH_A::SELECT_FIVE)
    }
}
#[doc = "Field `PWD_OVERCUR_DET` reader - Power down overcurrent detection comparator"]
pub type PWD_OVERCUR_DET_R = crate::BitReader<PWD_OVERCUR_DET_A>;
#[doc = "Power down overcurrent detection comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_OVERCUR_DET_A {
    #[doc = "0: Overcurrent detection comparator is enabled"]
    ENABLED = 0,
    #[doc = "1: Overcurrent detection comparator is disabled"]
    DISABLED = 1,
}
impl From<PWD_OVERCUR_DET_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_OVERCUR_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_OVERCUR_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_OVERCUR_DET_A {
        match self.bits {
            false => PWD_OVERCUR_DET_A::ENABLED,
            true => PWD_OVERCUR_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWD_OVERCUR_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWD_OVERCUR_DET_A::DISABLED
    }
}
#[doc = "Field `PWD_OVERCUR_DET` writer - Power down overcurrent detection comparator"]
pub type PWD_OVERCUR_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, PWD_OVERCUR_DET_A, O>;
impl<'a, const O: u8> PWD_OVERCUR_DET_W<'a, O> {
    #[doc = "Overcurrent detection comparator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWD_OVERCUR_DET_A::ENABLED)
    }
    #[doc = "Overcurrent detection comparator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWD_OVERCUR_DET_A::DISABLED)
    }
}
#[doc = "Field `OVERCUR_TRIG_ADJ` reader - Overcurrent Trigger Adjust"]
pub type OVERCUR_TRIG_ADJ_R = crate::FieldReader<u8, OVERCUR_TRIG_ADJ_A>;
#[doc = "Overcurrent Trigger Adjust\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVERCUR_TRIG_ADJ_A {
    #[doc = "0: In Run Mode, 1 A. In Power Save Mode, 0.25 A"]
    SELECT_ZERO = 0,
    #[doc = "1: In Run Mode, 2 A. In Power Save Mode, 0.25 A"]
    SELECT_ONE = 1,
    #[doc = "2: In Run Mode, 1 A. In Power Save Mode, 0.2 A"]
    SELECT_TWO = 2,
    #[doc = "3: In Run Mode, 2 A. In Power Save Mode, 0.2 A"]
    SELECT_THREE = 3,
}
impl From<OVERCUR_TRIG_ADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERCUR_TRIG_ADJ_A) -> Self {
        variant as _
    }
}
impl OVERCUR_TRIG_ADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERCUR_TRIG_ADJ_A {
        match self.bits {
            0 => OVERCUR_TRIG_ADJ_A::SELECT_ZERO,
            1 => OVERCUR_TRIG_ADJ_A::SELECT_ONE,
            2 => OVERCUR_TRIG_ADJ_A::SELECT_TWO,
            3 => OVERCUR_TRIG_ADJ_A::SELECT_THREE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_ZERO`"]
    #[inline(always)]
    pub fn is_select_zero(&self) -> bool {
        *self == OVERCUR_TRIG_ADJ_A::SELECT_ZERO
    }
    #[doc = "Checks if the value of the field is `SELECT_ONE`"]
    #[inline(always)]
    pub fn is_select_one(&self) -> bool {
        *self == OVERCUR_TRIG_ADJ_A::SELECT_ONE
    }
    #[doc = "Checks if the value of the field is `SELECT_TWO`"]
    #[inline(always)]
    pub fn is_select_two(&self) -> bool {
        *self == OVERCUR_TRIG_ADJ_A::SELECT_TWO
    }
    #[doc = "Checks if the value of the field is `SELECT_THREE`"]
    #[inline(always)]
    pub fn is_select_three(&self) -> bool {
        *self == OVERCUR_TRIG_ADJ_A::SELECT_THREE
    }
}
#[doc = "Field `OVERCUR_TRIG_ADJ` writer - Overcurrent Trigger Adjust"]
pub type OVERCUR_TRIG_ADJ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG0_SPEC, u8, OVERCUR_TRIG_ADJ_A, 2, O>;
impl<'a, const O: u8> OVERCUR_TRIG_ADJ_W<'a, O> {
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.25 A"]
    #[inline(always)]
    pub fn select_zero(self) -> &'a mut W {
        self.variant(OVERCUR_TRIG_ADJ_A::SELECT_ZERO)
    }
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.25 A"]
    #[inline(always)]
    pub fn select_one(self) -> &'a mut W {
        self.variant(OVERCUR_TRIG_ADJ_A::SELECT_ONE)
    }
    #[doc = "In Run Mode, 1 A. In Power Save Mode, 0.2 A"]
    #[inline(always)]
    pub fn select_two(self) -> &'a mut W {
        self.variant(OVERCUR_TRIG_ADJ_A::SELECT_TWO)
    }
    #[doc = "In Run Mode, 2 A. In Power Save Mode, 0.2 A"]
    #[inline(always)]
    pub fn select_three(self) -> &'a mut W {
        self.variant(OVERCUR_TRIG_ADJ_A::SELECT_THREE)
    }
}
#[doc = "Field `PWD_CMP_BATT_DET` reader - Power Down Battery Detection Comparator"]
pub type PWD_CMP_BATT_DET_R = crate::BitReader<PWD_CMP_BATT_DET_A>;
#[doc = "Power Down Battery Detection Comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_CMP_BATT_DET_A {
    #[doc = "0: Low voltage detection comparator is enabled"]
    ENABLED = 0,
    #[doc = "1: Low voltage detection comparator is disabled"]
    DISABLED = 1,
}
impl From<PWD_CMP_BATT_DET_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_CMP_BATT_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_CMP_BATT_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_CMP_BATT_DET_A {
        match self.bits {
            false => PWD_CMP_BATT_DET_A::ENABLED,
            true => PWD_CMP_BATT_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWD_CMP_BATT_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWD_CMP_BATT_DET_A::DISABLED
    }
}
#[doc = "Field `PWD_CMP_BATT_DET` writer - Power Down Battery Detection Comparator"]
pub type PWD_CMP_BATT_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, PWD_CMP_BATT_DET_A, O>;
impl<'a, const O: u8> PWD_CMP_BATT_DET_W<'a, O> {
    #[doc = "Low voltage detection comparator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWD_CMP_BATT_DET_A::ENABLED)
    }
    #[doc = "Low voltage detection comparator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWD_CMP_BATT_DET_A::DISABLED)
    }
}
#[doc = "Field `EN_LP_OVERLOAD_SNS` reader - Low Power Overload Sense Enable"]
pub type EN_LP_OVERLOAD_SNS_R = crate::BitReader<EN_LP_OVERLOAD_SNS_A>;
#[doc = "Low Power Overload Sense Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_LP_OVERLOAD_SNS_A {
    #[doc = "0: Overload Detection in power save mode disabled"]
    DISABLED = 0,
    #[doc = "1: Overload Detection in power save mode enabled"]
    ENABLED = 1,
}
impl From<EN_LP_OVERLOAD_SNS_A> for bool {
    #[inline(always)]
    fn from(variant: EN_LP_OVERLOAD_SNS_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_LP_OVERLOAD_SNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_LP_OVERLOAD_SNS_A {
        match self.bits {
            false => EN_LP_OVERLOAD_SNS_A::DISABLED,
            true => EN_LP_OVERLOAD_SNS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_LP_OVERLOAD_SNS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_LP_OVERLOAD_SNS_A::ENABLED
    }
}
#[doc = "Field `EN_LP_OVERLOAD_SNS` writer - Low Power Overload Sense Enable"]
pub type EN_LP_OVERLOAD_SNS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, EN_LP_OVERLOAD_SNS_A, O>;
impl<'a, const O: u8> EN_LP_OVERLOAD_SNS_W<'a, O> {
    #[doc = "Overload Detection in power save mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_LP_OVERLOAD_SNS_A::DISABLED)
    }
    #[doc = "Overload Detection in power save mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_LP_OVERLOAD_SNS_A::ENABLED)
    }
}
#[doc = "Field `PWD_HIGH_VOLT_DET` reader - Power Down High Voltage Detection"]
pub type PWD_HIGH_VOLT_DET_R = crate::BitReader<PWD_HIGH_VOLT_DET_A>;
#[doc = "Power Down High Voltage Detection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_HIGH_VOLT_DET_A {
    #[doc = "0: Overvoltage detection comparator is enabled"]
    ENABLED = 0,
    #[doc = "1: Overvoltage detection comparator is disabled"]
    DISABLED = 1,
}
impl From<PWD_HIGH_VOLT_DET_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_HIGH_VOLT_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_HIGH_VOLT_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_HIGH_VOLT_DET_A {
        match self.bits {
            false => PWD_HIGH_VOLT_DET_A::ENABLED,
            true => PWD_HIGH_VOLT_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWD_HIGH_VOLT_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWD_HIGH_VOLT_DET_A::DISABLED
    }
}
#[doc = "Field `PWD_HIGH_VOLT_DET` writer - Power Down High Voltage Detection"]
pub type PWD_HIGH_VOLT_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, PWD_HIGH_VOLT_DET_A, O>;
impl<'a, const O: u8> PWD_HIGH_VOLT_DET_W<'a, O> {
    #[doc = "Overvoltage detection comparator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWD_HIGH_VOLT_DET_A::ENABLED)
    }
    #[doc = "Overvoltage detection comparator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWD_HIGH_VOLT_DET_A::DISABLED)
    }
}
#[doc = "Field `LP_OVERLOAD_THRSH` reader - Low Power Overload Threshold"]
pub type LP_OVERLOAD_THRSH_R = crate::FieldReader<u8, LP_OVERLOAD_THRSH_A>;
#[doc = "Low Power Overload Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LP_OVERLOAD_THRSH_A {
    #[doc = "0: 32"]
    THRSH_32 = 0,
    #[doc = "1: 64"]
    THRSH_64 = 1,
    #[doc = "2: 16"]
    THRSH_16 = 2,
    #[doc = "3: 8"]
    THRSH_8 = 3,
}
impl From<LP_OVERLOAD_THRSH_A> for u8 {
    #[inline(always)]
    fn from(variant: LP_OVERLOAD_THRSH_A) -> Self {
        variant as _
    }
}
impl LP_OVERLOAD_THRSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_OVERLOAD_THRSH_A {
        match self.bits {
            0 => LP_OVERLOAD_THRSH_A::THRSH_32,
            1 => LP_OVERLOAD_THRSH_A::THRSH_64,
            2 => LP_OVERLOAD_THRSH_A::THRSH_16,
            3 => LP_OVERLOAD_THRSH_A::THRSH_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THRSH_32`"]
    #[inline(always)]
    pub fn is_thrsh_32(&self) -> bool {
        *self == LP_OVERLOAD_THRSH_A::THRSH_32
    }
    #[doc = "Checks if the value of the field is `THRSH_64`"]
    #[inline(always)]
    pub fn is_thrsh_64(&self) -> bool {
        *self == LP_OVERLOAD_THRSH_A::THRSH_64
    }
    #[doc = "Checks if the value of the field is `THRSH_16`"]
    #[inline(always)]
    pub fn is_thrsh_16(&self) -> bool {
        *self == LP_OVERLOAD_THRSH_A::THRSH_16
    }
    #[doc = "Checks if the value of the field is `THRSH_8`"]
    #[inline(always)]
    pub fn is_thrsh_8(&self) -> bool {
        *self == LP_OVERLOAD_THRSH_A::THRSH_8
    }
}
#[doc = "Field `LP_OVERLOAD_THRSH` writer - Low Power Overload Threshold"]
pub type LP_OVERLOAD_THRSH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG0_SPEC, u8, LP_OVERLOAD_THRSH_A, 2, O>;
impl<'a, const O: u8> LP_OVERLOAD_THRSH_W<'a, O> {
    #[doc = "32"]
    #[inline(always)]
    pub fn thrsh_32(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_THRSH_A::THRSH_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn thrsh_64(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_THRSH_A::THRSH_64)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn thrsh_16(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_THRSH_A::THRSH_16)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn thrsh_8(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_THRSH_A::THRSH_8)
    }
}
#[doc = "Field `LP_OVERLOAD_FREQ_SEL` reader - Low Power Overload Frequency Select"]
pub type LP_OVERLOAD_FREQ_SEL_R = crate::BitReader<LP_OVERLOAD_FREQ_SEL_A>;
#[doc = "Low Power Overload Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_OVERLOAD_FREQ_SEL_A {
    #[doc = "0: eight 32k cycle"]
    EIGHT_32K_CYCLE = 0,
    #[doc = "1: sixteen 32k cycle"]
    SIXTEEN_32K_CYCLE = 1,
}
impl From<LP_OVERLOAD_FREQ_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LP_OVERLOAD_FREQ_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LP_OVERLOAD_FREQ_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_OVERLOAD_FREQ_SEL_A {
        match self.bits {
            false => LP_OVERLOAD_FREQ_SEL_A::EIGHT_32K_CYCLE,
            true => LP_OVERLOAD_FREQ_SEL_A::SIXTEEN_32K_CYCLE,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHT_32K_CYCLE`"]
    #[inline(always)]
    pub fn is_eight_32k_cycle(&self) -> bool {
        *self == LP_OVERLOAD_FREQ_SEL_A::EIGHT_32K_CYCLE
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_32K_CYCLE`"]
    #[inline(always)]
    pub fn is_sixteen_32k_cycle(&self) -> bool {
        *self == LP_OVERLOAD_FREQ_SEL_A::SIXTEEN_32K_CYCLE
    }
}
#[doc = "Field `LP_OVERLOAD_FREQ_SEL` writer - Low Power Overload Frequency Select"]
pub type LP_OVERLOAD_FREQ_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, LP_OVERLOAD_FREQ_SEL_A, O>;
impl<'a, const O: u8> LP_OVERLOAD_FREQ_SEL_W<'a, O> {
    #[doc = "eight 32k cycle"]
    #[inline(always)]
    pub fn eight_32k_cycle(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_FREQ_SEL_A::EIGHT_32K_CYCLE)
    }
    #[doc = "sixteen 32k cycle"]
    #[inline(always)]
    pub fn sixteen_32k_cycle(self) -> &'a mut W {
        self.variant(LP_OVERLOAD_FREQ_SEL_A::SIXTEEN_32K_CYCLE)
    }
}
#[doc = "Field `LP_HIGH_HYS` reader - Low Power High Hysteric Value"]
pub type LP_HIGH_HYS_R = crate::BitReader<LP_HIGH_HYS_A>;
#[doc = "Low Power High Hysteric Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_HIGH_HYS_A {
    #[doc = "0: Adjust hysteretic value in low power to 12.5mV"]
    LP_12P5M_V = 0,
    #[doc = "1: Adjust hysteretic value in low power to 25mV"]
    LP_25M_V = 1,
}
impl From<LP_HIGH_HYS_A> for bool {
    #[inline(always)]
    fn from(variant: LP_HIGH_HYS_A) -> Self {
        variant as u8 != 0
    }
}
impl LP_HIGH_HYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_HIGH_HYS_A {
        match self.bits {
            false => LP_HIGH_HYS_A::LP_12P5M_V,
            true => LP_HIGH_HYS_A::LP_25M_V,
        }
    }
    #[doc = "Checks if the value of the field is `LP_12P5M_V`"]
    #[inline(always)]
    pub fn is_lp_12p5m_v(&self) -> bool {
        *self == LP_HIGH_HYS_A::LP_12P5M_V
    }
    #[doc = "Checks if the value of the field is `LP_25M_V`"]
    #[inline(always)]
    pub fn is_lp_25m_v(&self) -> bool {
        *self == LP_HIGH_HYS_A::LP_25M_V
    }
}
#[doc = "Field `LP_HIGH_HYS` writer - Low Power High Hysteric Value"]
pub type LP_HIGH_HYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG0_SPEC, LP_HIGH_HYS_A, O>;
impl<'a, const O: u8> LP_HIGH_HYS_W<'a, O> {
    #[doc = "Adjust hysteretic value in low power to 12.5mV"]
    #[inline(always)]
    pub fn lp_12p5m_v(self) -> &'a mut W {
        self.variant(LP_HIGH_HYS_A::LP_12P5M_V)
    }
    #[doc = "Adjust hysteretic value in low power to 25mV"]
    #[inline(always)]
    pub fn lp_25m_v(self) -> &'a mut W {
        self.variant(LP_HIGH_HYS_A::LP_25M_V)
    }
}
#[doc = "Field `PWD_CMP_OFFSET` reader - Power down output range comparator"]
pub type PWD_CMP_OFFSET_R = crate::BitReader<PWD_CMP_OFFSET_A>;
#[doc = "Power down output range comparator\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWD_CMP_OFFSET_A {
    #[doc = "0: Output range comparator powered up"]
    POWERED_UP = 0,
    #[doc = "1: Output range comparator powered down"]
    POWERED_DOWN = 1,
}
impl From<PWD_CMP_OFFSET_A> for bool {
    #[inline(always)]
    fn from(variant: PWD_CMP_OFFSET_A) -> Self {
        variant as u8 != 0
    }
}
impl PWD_CMP_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWD_CMP_OFFSET_A {
        match self.bits {
            false => PWD_CMP_OFFSET_A::POWERED_UP,
            true => PWD_CMP_OFFSET_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_UP`"]
    #[inline(always)]
    pub fn is_powered_up(&self) -> bool {
        *self == PWD_CMP_OFFSET_A::POWERED_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PWD_CMP_OFFSET_A::POWERED_DOWN
    }
}
#[doc = "Field `PWD_CMP_OFFSET` writer - Power down output range comparator"]
pub type PWD_CMP_OFFSET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, PWD_CMP_OFFSET_A, O>;
impl<'a, const O: u8> PWD_CMP_OFFSET_W<'a, O> {
    #[doc = "Output range comparator powered up"]
    #[inline(always)]
    pub fn powered_up(self) -> &'a mut W {
        self.variant(PWD_CMP_OFFSET_A::POWERED_UP)
    }
    #[doc = "Output range comparator powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWD_CMP_OFFSET_A::POWERED_DOWN)
    }
}
#[doc = "Field `XTALOK_DISABLE` reader - Disable xtalok detection circuit"]
pub type XTALOK_DISABLE_R = crate::BitReader<XTALOK_DISABLE_A>;
#[doc = "Disable xtalok detection circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTALOK_DISABLE_A {
    #[doc = "0: Enable xtalok detection circuit"]
    ENABLED = 0,
    #[doc = "1: Disable xtalok detection circuit and always outputs OK signal \"1\""]
    DISABLED = 1,
}
impl From<XTALOK_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: XTALOK_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl XTALOK_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOK_DISABLE_A {
        match self.bits {
            false => XTALOK_DISABLE_A::ENABLED,
            true => XTALOK_DISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == XTALOK_DISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == XTALOK_DISABLE_A::DISABLED
    }
}
#[doc = "Field `XTALOK_DISABLE` writer - Disable xtalok detection circuit"]
pub type XTALOK_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, XTALOK_DISABLE_A, O>;
impl<'a, const O: u8> XTALOK_DISABLE_W<'a, O> {
    #[doc = "Enable xtalok detection circuit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALOK_DISABLE_A::ENABLED)
    }
    #[doc = "Disable xtalok detection circuit and always outputs OK signal \"1\""]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALOK_DISABLE_A::DISABLED)
    }
}
#[doc = "Field `CURRENT_ALERT_RESET` reader - Reset Current Alert Signal"]
pub type CURRENT_ALERT_RESET_R = crate::BitReader<CURRENT_ALERT_RESET_A>;
#[doc = "Reset Current Alert Signal\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CURRENT_ALERT_RESET_A {
    #[doc = "0: Current Alert Signal not reset"]
    NOT_RESET = 0,
    #[doc = "1: Current Alert Signal reset"]
    RESET = 1,
}
impl From<CURRENT_ALERT_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: CURRENT_ALERT_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl CURRENT_ALERT_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CURRENT_ALERT_RESET_A {
        match self.bits {
            false => CURRENT_ALERT_RESET_A::NOT_RESET,
            true => CURRENT_ALERT_RESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == CURRENT_ALERT_RESET_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CURRENT_ALERT_RESET_A::RESET
    }
}
#[doc = "Field `CURRENT_ALERT_RESET` writer - Reset Current Alert Signal"]
pub type CURRENT_ALERT_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG0_SPEC, CURRENT_ALERT_RESET_A, O>;
impl<'a, const O: u8> CURRENT_ALERT_RESET_W<'a, O> {
    #[doc = "Current Alert Signal not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(CURRENT_ALERT_RESET_A::NOT_RESET)
    }
    #[doc = "Current Alert Signal reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CURRENT_ALERT_RESET_A::RESET)
    }
}
#[doc = "Field `XTAL_24M_OK` reader - 24M XTAL OK"]
pub type XTAL_24M_OK_R = crate::BitReader<XTAL_24M_OK_A>;
#[doc = "24M XTAL OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTAL_24M_OK_A {
    #[doc = "0: DCDC uses internal ring OSC"]
    INT_RNG_OSC = 0,
    #[doc = "1: DCDC uses xtal 24M"]
    XTAL_24M = 1,
}
impl From<XTAL_24M_OK_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL_24M_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl XTAL_24M_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL_24M_OK_A {
        match self.bits {
            false => XTAL_24M_OK_A::INT_RNG_OSC,
            true => XTAL_24M_OK_A::XTAL_24M,
        }
    }
    #[doc = "Checks if the value of the field is `INT_RNG_OSC`"]
    #[inline(always)]
    pub fn is_int_rng_osc(&self) -> bool {
        *self == XTAL_24M_OK_A::INT_RNG_OSC
    }
    #[doc = "Checks if the value of the field is `XTAL_24M`"]
    #[inline(always)]
    pub fn is_xtal_24m(&self) -> bool {
        *self == XTAL_24M_OK_A::XTAL_24M
    }
}
#[doc = "Field `XTAL_24M_OK` writer - 24M XTAL OK"]
pub type XTAL_24M_OK_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG0_SPEC, XTAL_24M_OK_A, O>;
impl<'a, const O: u8> XTAL_24M_OK_W<'a, O> {
    #[doc = "DCDC uses internal ring OSC"]
    #[inline(always)]
    pub fn int_rng_osc(self) -> &'a mut W {
        self.variant(XTAL_24M_OK_A::INT_RNG_OSC)
    }
    #[doc = "DCDC uses xtal 24M"]
    #[inline(always)]
    pub fn xtal_24m(self) -> &'a mut W {
        self.variant(XTAL_24M_OK_A::XTAL_24M)
    }
}
#[doc = "Field `STS_DC_OK` reader - DCDC Output OK"]
pub type STS_DC_OK_R = crate::BitReader<STS_DC_OK_A>;
#[doc = "DCDC Output OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STS_DC_OK_A {
    #[doc = "0: DCDC is settling"]
    NOT_SETTLED = 0,
    #[doc = "1: DCDC already settled"]
    SETTLED = 1,
}
impl From<STS_DC_OK_A> for bool {
    #[inline(always)]
    fn from(variant: STS_DC_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl STS_DC_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS_DC_OK_A {
        match self.bits {
            false => STS_DC_OK_A::NOT_SETTLED,
            true => STS_DC_OK_A::SETTLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SETTLED`"]
    #[inline(always)]
    pub fn is_not_settled(&self) -> bool {
        *self == STS_DC_OK_A::NOT_SETTLED
    }
    #[doc = "Checks if the value of the field is `SETTLED`"]
    #[inline(always)]
    pub fn is_settled(&self) -> bool {
        *self == STS_DC_OK_A::SETTLED
    }
}
impl R {
    #[doc = "Bit 0 - Power Down Zero Cross Detection"]
    #[inline(always)]
    pub fn pwd_zcd(&self) -> PWD_ZCD_R {
        PWD_ZCD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Auto Clock Switch"]
    #[inline(always)]
    pub fn disable_auto_clk_switch(&self) -> DISABLE_AUTO_CLK_SWITCH_R {
        DISABLE_AUTO_CLK_SWITCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Clock"]
    #[inline(always)]
    pub fn sel_clk(&self) -> SEL_CLK_R {
        SEL_CLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down internal osc"]
    #[inline(always)]
    pub fn pwd_osc_int(&self) -> PWD_OSC_INT_R {
        PWD_OSC_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power down signal of the current detector."]
    #[inline(always)]
    pub fn pwd_cur_sns_cmp(&self) -> PWD_CUR_SNS_CMP_R {
        PWD_CUR_SNS_CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Current Sense (detector) Threshold"]
    #[inline(always)]
    pub fn cur_sns_thrsh(&self) -> CUR_SNS_THRSH_R {
        CUR_SNS_THRSH_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Power down overcurrent detection comparator"]
    #[inline(always)]
    pub fn pwd_overcur_det(&self) -> PWD_OVERCUR_DET_R {
        PWD_OVERCUR_DET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Overcurrent Trigger Adjust"]
    #[inline(always)]
    pub fn overcur_trig_adj(&self) -> OVERCUR_TRIG_ADJ_R {
        OVERCUR_TRIG_ADJ_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Power Down Battery Detection Comparator"]
    #[inline(always)]
    pub fn pwd_cmp_batt_det(&self) -> PWD_CMP_BATT_DET_R {
        PWD_CMP_BATT_DET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Low Power Overload Sense Enable"]
    #[inline(always)]
    pub fn en_lp_overload_sns(&self) -> EN_LP_OVERLOAD_SNS_R {
        EN_LP_OVERLOAD_SNS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power Down High Voltage Detection"]
    #[inline(always)]
    pub fn pwd_high_volt_det(&self) -> PWD_HIGH_VOLT_DET_R {
        PWD_HIGH_VOLT_DET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Low Power Overload Threshold"]
    #[inline(always)]
    pub fn lp_overload_thrsh(&self) -> LP_OVERLOAD_THRSH_R {
        LP_OVERLOAD_THRSH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Low Power Overload Frequency Select"]
    #[inline(always)]
    pub fn lp_overload_freq_sel(&self) -> LP_OVERLOAD_FREQ_SEL_R {
        LP_OVERLOAD_FREQ_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Low Power High Hysteric Value"]
    #[inline(always)]
    pub fn lp_high_hys(&self) -> LP_HIGH_HYS_R {
        LP_HIGH_HYS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - Power down output range comparator"]
    #[inline(always)]
    pub fn pwd_cmp_offset(&self) -> PWD_CMP_OFFSET_R {
        PWD_CMP_OFFSET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable xtalok detection circuit"]
    #[inline(always)]
    pub fn xtalok_disable(&self) -> XTALOK_DISABLE_R {
        XTALOK_DISABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset Current Alert Signal"]
    #[inline(always)]
    pub fn current_alert_reset(&self) -> CURRENT_ALERT_RESET_R {
        CURRENT_ALERT_RESET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 24M XTAL OK"]
    #[inline(always)]
    pub fn xtal_24m_ok(&self) -> XTAL_24M_OK_R {
        XTAL_24M_OK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DCDC Output OK"]
    #[inline(always)]
    pub fn sts_dc_ok(&self) -> STS_DC_OK_R {
        STS_DC_OK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down Zero Cross Detection"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_zcd(&mut self) -> PWD_ZCD_W<0> {
        PWD_ZCD_W::new(self)
    }
    #[doc = "Bit 1 - Disable Auto Clock Switch"]
    #[inline(always)]
    #[must_use]
    pub fn disable_auto_clk_switch(&mut self) -> DISABLE_AUTO_CLK_SWITCH_W<1> {
        DISABLE_AUTO_CLK_SWITCH_W::new(self)
    }
    #[doc = "Bit 2 - Select Clock"]
    #[inline(always)]
    #[must_use]
    pub fn sel_clk(&mut self) -> SEL_CLK_W<2> {
        SEL_CLK_W::new(self)
    }
    #[doc = "Bit 3 - Power down internal osc"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_osc_int(&mut self) -> PWD_OSC_INT_W<3> {
        PWD_OSC_INT_W::new(self)
    }
    #[doc = "Bit 4 - Power down signal of the current detector."]
    #[inline(always)]
    #[must_use]
    pub fn pwd_cur_sns_cmp(&mut self) -> PWD_CUR_SNS_CMP_W<4> {
        PWD_CUR_SNS_CMP_W::new(self)
    }
    #[doc = "Bits 5:7 - Current Sense (detector) Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn cur_sns_thrsh(&mut self) -> CUR_SNS_THRSH_W<5> {
        CUR_SNS_THRSH_W::new(self)
    }
    #[doc = "Bit 8 - Power down overcurrent detection comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_overcur_det(&mut self) -> PWD_OVERCUR_DET_W<8> {
        PWD_OVERCUR_DET_W::new(self)
    }
    #[doc = "Bits 9:10 - Overcurrent Trigger Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn overcur_trig_adj(&mut self) -> OVERCUR_TRIG_ADJ_W<9> {
        OVERCUR_TRIG_ADJ_W::new(self)
    }
    #[doc = "Bit 11 - Power Down Battery Detection Comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_cmp_batt_det(&mut self) -> PWD_CMP_BATT_DET_W<11> {
        PWD_CMP_BATT_DET_W::new(self)
    }
    #[doc = "Bit 16 - Low Power Overload Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_lp_overload_sns(&mut self) -> EN_LP_OVERLOAD_SNS_W<16> {
        EN_LP_OVERLOAD_SNS_W::new(self)
    }
    #[doc = "Bit 17 - Power Down High Voltage Detection"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_high_volt_det(&mut self) -> PWD_HIGH_VOLT_DET_W<17> {
        PWD_HIGH_VOLT_DET_W::new(self)
    }
    #[doc = "Bits 18:19 - Low Power Overload Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lp_overload_thrsh(&mut self) -> LP_OVERLOAD_THRSH_W<18> {
        LP_OVERLOAD_THRSH_W::new(self)
    }
    #[doc = "Bit 20 - Low Power Overload Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn lp_overload_freq_sel(&mut self) -> LP_OVERLOAD_FREQ_SEL_W<20> {
        LP_OVERLOAD_FREQ_SEL_W::new(self)
    }
    #[doc = "Bit 21 - Low Power High Hysteric Value"]
    #[inline(always)]
    #[must_use]
    pub fn lp_high_hys(&mut self) -> LP_HIGH_HYS_W<21> {
        LP_HIGH_HYS_W::new(self)
    }
    #[doc = "Bit 26 - Power down output range comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pwd_cmp_offset(&mut self) -> PWD_CMP_OFFSET_W<26> {
        PWD_CMP_OFFSET_W::new(self)
    }
    #[doc = "Bit 27 - Disable xtalok detection circuit"]
    #[inline(always)]
    #[must_use]
    pub fn xtalok_disable(&mut self) -> XTALOK_DISABLE_W<27> {
        XTALOK_DISABLE_W::new(self)
    }
    #[doc = "Bit 28 - Reset Current Alert Signal"]
    #[inline(always)]
    #[must_use]
    pub fn current_alert_reset(&mut self) -> CURRENT_ALERT_RESET_W<28> {
        CURRENT_ALERT_RESET_W::new(self)
    }
    #[doc = "Bit 29 - 24M XTAL OK"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_24m_ok(&mut self) -> XTAL_24M_OK_W<29> {
        XTAL_24M_OK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0](index.html) module"]
pub struct REG0_SPEC;
impl crate::RegisterSpec for REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg0::R](R) reader structure"]
impl crate::Readable for REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg0::W](W) writer structure"]
impl crate::Writable for REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG0 to value 0x1403_0111"]
impl crate::Resettable for REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1403_0111;
}
