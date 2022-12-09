#[doc = "Register `LOWPWR_CTRL_SET` reader"]
pub struct R(crate::R<LOWPWR_CTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOWPWR_CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOWPWR_CTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOWPWR_CTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOWPWR_CTRL_SET` writer"]
pub struct W(crate::W<LOWPWR_CTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOWPWR_CTRL_SET_SPEC>;
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
impl From<crate::W<LOWPWR_CTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOWPWR_CTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC_OSC_EN` reader - RC Osc. enable control."]
pub type RC_OSC_EN_R = crate::BitReader<RC_OSC_EN_A>;
#[doc = "RC Osc. enable control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RC_OSC_EN_A {
    #[doc = "0: Use XTAL OSC to source the 24MHz clock"]
    RC_OSC_EN_0 = 0,
    #[doc = "1: Use RC OSC"]
    RC_OSC_EN_1 = 1,
}
impl From<RC_OSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RC_OSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RC_OSC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RC_OSC_EN_A {
        match self.bits {
            false => RC_OSC_EN_A::RC_OSC_EN_0,
            true => RC_OSC_EN_A::RC_OSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_0`"]
    #[inline(always)]
    pub fn is_rc_osc_en_0(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_EN_0
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_1`"]
    #[inline(always)]
    pub fn is_rc_osc_en_1(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_EN_1
    }
}
#[doc = "Field `RC_OSC_EN` writer - RC Osc. enable control."]
pub type RC_OSC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, RC_OSC_EN_A, O>;
impl<'a, const O: u8> RC_OSC_EN_W<'a, O> {
    #[doc = "Use XTAL OSC to source the 24MHz clock"]
    #[inline(always)]
    pub fn rc_osc_en_0(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_EN_0)
    }
    #[doc = "Use RC OSC"]
    #[inline(always)]
    pub fn rc_osc_en_1(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_EN_1)
    }
}
#[doc = "Field `OSC_SEL` reader - Select the source for the 24MHz clock."]
pub type OSC_SEL_R = crate::BitReader<OSC_SEL_A>;
#[doc = "Select the source for the 24MHz clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSC_SEL_A {
    #[doc = "0: XTAL OSC"]
    OSC_SEL_0 = 0,
    #[doc = "1: RC OSC"]
    OSC_SEL_1 = 1,
}
impl From<OSC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_SEL_A {
        match self.bits {
            false => OSC_SEL_A::OSC_SEL_0,
            true => OSC_SEL_A::OSC_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_0`"]
    #[inline(always)]
    pub fn is_osc_sel_0(&self) -> bool {
        *self == OSC_SEL_A::OSC_SEL_0
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_1`"]
    #[inline(always)]
    pub fn is_osc_sel_1(&self) -> bool {
        *self == OSC_SEL_A::OSC_SEL_1
    }
}
#[doc = "Field `OSC_SEL` writer - Select the source for the 24MHz clock."]
pub type OSC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, OSC_SEL_A, O>;
impl<'a, const O: u8> OSC_SEL_W<'a, O> {
    #[doc = "XTAL OSC"]
    #[inline(always)]
    pub fn osc_sel_0(self) -> &'a mut W {
        self.variant(OSC_SEL_A::OSC_SEL_0)
    }
    #[doc = "RC OSC"]
    #[inline(always)]
    pub fn osc_sel_1(self) -> &'a mut W {
        self.variant(OSC_SEL_A::OSC_SEL_1)
    }
}
#[doc = "Field `LPBG_SEL` reader - Bandgap select. Not related to oscillator."]
pub type LPBG_SEL_R = crate::BitReader<LPBG_SEL_A>;
#[doc = "Bandgap select. Not related to oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPBG_SEL_A {
    #[doc = "0: Normal power bandgap"]
    LPBG_SEL_0 = 0,
    #[doc = "1: Low power bandgap"]
    LPBG_SEL_1 = 1,
}
impl From<LPBG_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LPBG_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPBG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBG_SEL_A {
        match self.bits {
            false => LPBG_SEL_A::LPBG_SEL_0,
            true => LPBG_SEL_A::LPBG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_0`"]
    #[inline(always)]
    pub fn is_lpbg_sel_0(&self) -> bool {
        *self == LPBG_SEL_A::LPBG_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_1`"]
    #[inline(always)]
    pub fn is_lpbg_sel_1(&self) -> bool {
        *self == LPBG_SEL_A::LPBG_SEL_1
    }
}
#[doc = "Field `LPBG_SEL` writer - Bandgap select. Not related to oscillator."]
pub type LPBG_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, LPBG_SEL_A, O>;
impl<'a, const O: u8> LPBG_SEL_W<'a, O> {
    #[doc = "Normal power bandgap"]
    #[inline(always)]
    pub fn lpbg_sel_0(self) -> &'a mut W {
        self.variant(LPBG_SEL_A::LPBG_SEL_0)
    }
    #[doc = "Low power bandgap"]
    #[inline(always)]
    pub fn lpbg_sel_1(self) -> &'a mut W {
        self.variant(LPBG_SEL_A::LPBG_SEL_1)
    }
}
#[doc = "Field `LPBG_TEST` reader - Low power bandgap test bit. Not related to oscillator."]
pub type LPBG_TEST_R = crate::BitReader<bool>;
#[doc = "Field `LPBG_TEST` writer - Low power bandgap test bit. Not related to oscillator."]
pub type LPBG_TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `REFTOP_IBIAS_OFF` reader - Low power reftop ibias disable. Not related to oscillator."]
pub type REFTOP_IBIAS_OFF_R = crate::BitReader<bool>;
#[doc = "Field `REFTOP_IBIAS_OFF` writer - Low power reftop ibias disable. Not related to oscillator."]
pub type REFTOP_IBIAS_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `L1_PWRGATE` reader - L1 power gate control. Used as software override. Not related to oscillator."]
pub type L1_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `L1_PWRGATE` writer - L1 power gate control. Used as software override. Not related to oscillator."]
pub type L1_PWRGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `L2_PWRGATE` reader - L2 power gate control. Used as software override. Not related to oscillator."]
pub type L2_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `L2_PWRGATE` writer - L2 power gate control. Used as software override. Not related to oscillator."]
pub type L2_PWRGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `CPU_PWRGATE` reader - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
pub type CPU_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `CPU_PWRGATE` writer - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
pub type CPU_PWRGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `DISPLAY_PWRGATE` reader - Display logic power gate control. Used as software override. Not related to oscillator."]
pub type DISPLAY_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `DISPLAY_PWRGATE` writer - Display logic power gate control. Used as software override. Not related to oscillator."]
pub type DISPLAY_PWRGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RCOSC_CG_OVERRIDE` reader - For debug purposes only"]
pub type RCOSC_CG_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_CG_OVERRIDE` writer - For debug purposes only"]
pub type RCOSC_CG_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `XTALOSC_PWRUP_DELAY` reader - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
pub type XTALOSC_PWRUP_DELAY_R = crate::FieldReader<u8, XTALOSC_PWRUP_DELAY_A>;
#[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XTALOSC_PWRUP_DELAY_A {
    #[doc = "0: 0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0,
    #[doc = "1: 0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 1,
    #[doc = "2: 1ms"]
    XTALOSC_PWRUP_DELAY_2 = 2,
    #[doc = "3: 2ms"]
    XTALOSC_PWRUP_DELAY_3 = 3,
}
impl From<XTALOSC_PWRUP_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: XTALOSC_PWRUP_DELAY_A) -> Self {
        variant as _
    }
}
impl XTALOSC_PWRUP_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOSC_PWRUP_DELAY_A {
        match self.bits {
            0 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0,
            1 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1,
            2 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2,
            3 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_0`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_0(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_1`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_1(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_2`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_2(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_3`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_3(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3
    }
}
#[doc = "Field `XTALOSC_PWRUP_DELAY` writer - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
pub type XTALOSC_PWRUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LOWPWR_CTRL_SET_SPEC, u8, XTALOSC_PWRUP_DELAY_A, 2, O>;
impl<'a, const O: u8> XTALOSC_PWRUP_DELAY_W<'a, O> {
    #[doc = "0.25ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_0(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0)
    }
    #[doc = "0.5ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_1(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1)
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_2(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2)
    }
    #[doc = "2ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_3(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3)
    }
}
#[doc = "Field `XTALOSC_PWRUP_STAT` reader - Status of the 24MHz xtal oscillator."]
pub type XTALOSC_PWRUP_STAT_R = crate::BitReader<XTALOSC_PWRUP_STAT_A>;
#[doc = "Status of the 24MHz xtal oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTALOSC_PWRUP_STAT_A {
    #[doc = "0: Not stable"]
    XTALOSC_PWRUP_STAT_0 = 0,
    #[doc = "1: Stable and ready to use"]
    XTALOSC_PWRUP_STAT_1 = 1,
}
impl From<XTALOSC_PWRUP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: XTALOSC_PWRUP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
impl XTALOSC_PWRUP_STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOSC_PWRUP_STAT_A {
        match self.bits {
            false => XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_0,
            true => XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_0`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_stat_0(&self) -> bool {
        *self == XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_1`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_stat_1(&self) -> bool {
        *self == XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_1
    }
}
#[doc = "Field `MIX_PWRGATE` reader - Display power gate control. Used as software mask. Set to zero to force ungated."]
pub type MIX_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `MIX_PWRGATE` writer - Display power gate control. Used as software mask. Set to zero to force ungated."]
pub type MIX_PWRGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
#[doc = "Field `GPU_PWRGATE` reader - GPU power gate control. Used as software mask. Set to zero to force ungated."]
pub type GPU_PWRGATE_R = crate::BitReader<bool>;
#[doc = "Field `GPU_PWRGATE` writer - GPU power gate control. Used as software mask. Set to zero to force ungated."]
pub type GPU_PWRGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOWPWR_CTRL_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline(always)]
    pub fn rc_osc_en(&self) -> RC_OSC_EN_R {
        RC_OSC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline(always)]
    pub fn osc_sel(&self) -> OSC_SEL_R {
        OSC_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_sel(&self) -> LPBG_SEL_R {
        LPBG_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_test(&self) -> LPBG_TEST_R {
        LPBG_TEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub fn reftop_ibias_off(&self) -> REFTOP_IBIAS_OFF_R {
        REFTOP_IBIAS_OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l1_pwrgate(&self) -> L1_PWRGATE_R {
        L1_PWRGATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l2_pwrgate(&self) -> L2_PWRGATE_R {
        L2_PWRGATE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub fn cpu_pwrgate(&self) -> CPU_PWRGATE_R {
        CPU_PWRGATE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn display_pwrgate(&self) -> DISPLAY_PWRGATE_R {
        DISPLAY_PWRGATE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline(always)]
    pub fn rcosc_cg_override(&self) -> RCOSC_CG_OVERRIDE_R {
        RCOSC_CG_OVERRIDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay(&self) -> XTALOSC_PWRUP_DELAY_R {
        XTALOSC_PWRUP_DELAY_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub fn xtalosc_pwrup_stat(&self) -> XTALOSC_PWRUP_STAT_R {
        XTALOSC_PWRUP_STAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn mix_pwrgate(&self) -> MIX_PWRGATE_R {
        MIX_PWRGATE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn gpu_pwrgate(&self) -> GPU_PWRGATE_R {
        GPU_PWRGATE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline(always)]
    #[must_use]
    pub fn rc_osc_en(&mut self) -> RC_OSC_EN_W<0> {
        RC_OSC_EN_W::new(self)
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn osc_sel(&mut self) -> OSC_SEL_W<4> {
        OSC_SEL_W::new(self)
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn lpbg_sel(&mut self) -> LPBG_SEL_W<5> {
        LPBG_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn lpbg_test(&mut self) -> LPBG_TEST_W<6> {
        LPBG_TEST_W::new(self)
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn reftop_ibias_off(&mut self) -> REFTOP_IBIAS_OFF_W<7> {
        REFTOP_IBIAS_OFF_W::new(self)
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn l1_pwrgate(&mut self) -> L1_PWRGATE_W<8> {
        L1_PWRGATE_W::new(self)
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn l2_pwrgate(&mut self) -> L2_PWRGATE_W<9> {
        L2_PWRGATE_W::new(self)
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_pwrgate(&mut self) -> CPU_PWRGATE_W<10> {
        CPU_PWRGATE_W::new(self)
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn display_pwrgate(&mut self) -> DISPLAY_PWRGATE_W<11> {
        DISPLAY_PWRGATE_W::new(self)
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_cg_override(&mut self) -> RCOSC_CG_OVERRIDE_W<13> {
        RCOSC_CG_OVERRIDE_W::new(self)
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    #[must_use]
    pub fn xtalosc_pwrup_delay(&mut self) -> XTALOSC_PWRUP_DELAY_W<14> {
        XTALOSC_PWRUP_DELAY_W::new(self)
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    #[must_use]
    pub fn mix_pwrgate(&mut self) -> MIX_PWRGATE_W<17> {
        MIX_PWRGATE_W::new(self)
    }
    #[doc = "Bit 18 - GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    #[must_use]
    pub fn gpu_pwrgate(&mut self) -> GPU_PWRGATE_W<18> {
        GPU_PWRGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL OSC (LP) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpwr_ctrl_set](index.html) module"]
pub struct LOWPWR_CTRL_SET_SPEC;
impl crate::RegisterSpec for LOWPWR_CTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lowpwr_ctrl_set::R](R) reader structure"]
impl crate::Readable for LOWPWR_CTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lowpwr_ctrl_set::W](W) writer structure"]
impl crate::Writable for LOWPWR_CTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOWPWR_CTRL_SET to value 0x4001"]
impl crate::Resettable for LOWPWR_CTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x4001;
}
