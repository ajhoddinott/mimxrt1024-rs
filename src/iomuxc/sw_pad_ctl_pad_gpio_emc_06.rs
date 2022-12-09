#[doc = "Register `SW_PAD_CTL_PAD_GPIO_EMC_06` reader"]
pub struct R(crate::R<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_PAD_CTL_PAD_GPIO_EMC_06` writer"]
pub struct W(crate::W<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>;
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
impl From<crate::W<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRE` reader - Slew Rate Field"]
pub type SRE_R = crate::BitReader<SRE_A>;
#[doc = "Slew Rate Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRE_A {
    #[doc = "0: Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0,
    #[doc = "1: Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 1,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::SRE_0_SLOW_SLEW_RATE,
            true => SRE_A::SRE_1_FAST_SLEW_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0_SLOW_SLEW_RATE`"]
    #[inline(always)]
    pub fn is_sre_0_slow_slew_rate(&self) -> bool {
        *self == SRE_A::SRE_0_SLOW_SLEW_RATE
    }
    #[doc = "Checks if the value of the field is `SRE_1_FAST_SLEW_RATE`"]
    #[inline(always)]
    pub fn is_sre_1_fast_slew_rate(&self) -> bool {
        *self == SRE_A::SRE_1_FAST_SLEW_RATE
    }
}
#[doc = "Field `SRE` writer - Slew Rate Field"]
pub type SRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, SRE_A, O>;
impl<'a, const O: u8> SRE_W<'a, O> {
    #[doc = "Slow Slew Rate"]
    #[inline(always)]
    pub fn sre_0_slow_slew_rate(self) -> &'a mut W {
        self.variant(SRE_A::SRE_0_SLOW_SLEW_RATE)
    }
    #[doc = "Fast Slew Rate"]
    #[inline(always)]
    pub fn sre_1_fast_slew_rate(self) -> &'a mut W {
        self.variant(SRE_A::SRE_1_FAST_SLEW_RATE)
    }
}
#[doc = "Field `DSE` reader - Drive Strength Field"]
pub type DSE_R = crate::FieldReader<u8, DSE_A>;
#[doc = "Drive Strength Field\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSE_A {
    #[doc = "0: output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0,
    #[doc = "1: R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V = 1,
    #[doc = "2: R0/2"]
    DSE_2_R0_2 = 2,
    #[doc = "3: R0/3"]
    DSE_3_R0_3 = 3,
    #[doc = "4: R0/4"]
    DSE_4_R0_4 = 4,
    #[doc = "5: R0/5"]
    DSE_5_R0_5 = 5,
    #[doc = "6: R0/6"]
    DSE_6_R0_6 = 6,
    #[doc = "7: R0/7"]
    DSE_7_R0_7 = 7,
}
impl From<DSE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSE_A) -> Self {
        variant as _
    }
}
impl DSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSE_A {
        match self.bits {
            0 => DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_,
            1 => DSE_A::DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V,
            2 => DSE_A::DSE_2_R0_2,
            3 => DSE_A::DSE_3_R0_3,
            4 => DSE_A::DSE_4_R0_4,
            5 => DSE_A::DSE_5_R0_5,
            6 => DSE_A::DSE_6_R0_6,
            7 => DSE_A::DSE_7_R0_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DSE_0_OUTPUT_DRIVER_DISABLED_`"]
    #[inline(always)]
    pub fn is_dse_0_output_driver_disabled_(&self) -> bool {
        *self == DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V`"]
    #[inline(always)]
    pub fn is_dse_1_r0_150_ohm___3_3v__260_ohm_1_8v(&self) -> bool {
        *self == DSE_A::DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V
    }
    #[doc = "Checks if the value of the field is `DSE_2_R0_2`"]
    #[inline(always)]
    pub fn is_dse_2_r0_2(&self) -> bool {
        *self == DSE_A::DSE_2_R0_2
    }
    #[doc = "Checks if the value of the field is `DSE_3_R0_3`"]
    #[inline(always)]
    pub fn is_dse_3_r0_3(&self) -> bool {
        *self == DSE_A::DSE_3_R0_3
    }
    #[doc = "Checks if the value of the field is `DSE_4_R0_4`"]
    #[inline(always)]
    pub fn is_dse_4_r0_4(&self) -> bool {
        *self == DSE_A::DSE_4_R0_4
    }
    #[doc = "Checks if the value of the field is `DSE_5_R0_5`"]
    #[inline(always)]
    pub fn is_dse_5_r0_5(&self) -> bool {
        *self == DSE_A::DSE_5_R0_5
    }
    #[doc = "Checks if the value of the field is `DSE_6_R0_6`"]
    #[inline(always)]
    pub fn is_dse_6_r0_6(&self) -> bool {
        *self == DSE_A::DSE_6_R0_6
    }
    #[doc = "Checks if the value of the field is `DSE_7_R0_7`"]
    #[inline(always)]
    pub fn is_dse_7_r0_7(&self) -> bool {
        *self == DSE_A::DSE_7_R0_7
    }
}
#[doc = "Field `DSE` writer - Drive Strength Field"]
pub type DSE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, u8, DSE_A, 3, O>;
impl<'a, const O: u8> DSE_W<'a, O> {
    #[doc = "output driver disabled;"]
    #[inline(always)]
    pub fn dse_0_output_driver_disabled_(self) -> &'a mut W {
        self.variant(DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_)
    }
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    #[inline(always)]
    pub fn dse_1_r0_150_ohm___3_3v__260_ohm_1_8v(self) -> &'a mut W {
        self.variant(DSE_A::DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V)
    }
    #[doc = "R0/2"]
    #[inline(always)]
    pub fn dse_2_r0_2(self) -> &'a mut W {
        self.variant(DSE_A::DSE_2_R0_2)
    }
    #[doc = "R0/3"]
    #[inline(always)]
    pub fn dse_3_r0_3(self) -> &'a mut W {
        self.variant(DSE_A::DSE_3_R0_3)
    }
    #[doc = "R0/4"]
    #[inline(always)]
    pub fn dse_4_r0_4(self) -> &'a mut W {
        self.variant(DSE_A::DSE_4_R0_4)
    }
    #[doc = "R0/5"]
    #[inline(always)]
    pub fn dse_5_r0_5(self) -> &'a mut W {
        self.variant(DSE_A::DSE_5_R0_5)
    }
    #[doc = "R0/6"]
    #[inline(always)]
    pub fn dse_6_r0_6(self) -> &'a mut W {
        self.variant(DSE_A::DSE_6_R0_6)
    }
    #[doc = "R0/7"]
    #[inline(always)]
    pub fn dse_7_r0_7(self) -> &'a mut W {
        self.variant(DSE_A::DSE_7_R0_7)
    }
}
#[doc = "Field `SPEED` reader - Speed Field"]
pub type SPEED_R = crate::FieldReader<u8, SPEED_A>;
#[doc = "Speed Field\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: 50MHz"]
    SPEED_0_LOW = 0,
    #[doc = "1: 100MHz - 150MHz"]
    SPEED_1_MEDIUM = 1,
    #[doc = "2: 100MHz - 150MHz"]
    SPEED_2_MEDIUM = 2,
    #[doc = "3: 150MHz - 200MHz"]
    SPEED_3_MAXIMUM = 3,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::SPEED_0_LOW,
            1 => SPEED_A::SPEED_1_MEDIUM,
            2 => SPEED_A::SPEED_2_MEDIUM,
            3 => SPEED_A::SPEED_3_MAXIMUM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_0_LOW`"]
    #[inline(always)]
    pub fn is_speed_0_low(&self) -> bool {
        *self == SPEED_A::SPEED_0_LOW
    }
    #[doc = "Checks if the value of the field is `SPEED_1_MEDIUM`"]
    #[inline(always)]
    pub fn is_speed_1_medium(&self) -> bool {
        *self == SPEED_A::SPEED_1_MEDIUM
    }
    #[doc = "Checks if the value of the field is `SPEED_2_MEDIUM`"]
    #[inline(always)]
    pub fn is_speed_2_medium(&self) -> bool {
        *self == SPEED_A::SPEED_2_MEDIUM
    }
    #[doc = "Checks if the value of the field is `SPEED_3_MAXIMUM`"]
    #[inline(always)]
    pub fn is_speed_3_maximum(&self) -> bool {
        *self == SPEED_A::SPEED_3_MAXIMUM
    }
}
#[doc = "Field `SPEED` writer - Speed Field"]
pub type SPEED_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, u8, SPEED_A, 2, O>;
impl<'a, const O: u8> SPEED_W<'a, O> {
    #[doc = "50MHz"]
    #[inline(always)]
    pub fn speed_0_low(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_0_LOW)
    }
    #[doc = "100MHz - 150MHz"]
    #[inline(always)]
    pub fn speed_1_medium(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_1_MEDIUM)
    }
    #[doc = "100MHz - 150MHz"]
    #[inline(always)]
    pub fn speed_2_medium(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_2_MEDIUM)
    }
    #[doc = "150MHz - 200MHz"]
    #[inline(always)]
    pub fn speed_3_maximum(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_3_MAXIMUM)
    }
}
#[doc = "Field `ODE` reader - Open Drain Enable Field"]
pub type ODE_R = crate::BitReader<ODE_A>;
#[doc = "Open Drain Enable Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODE_A {
    #[doc = "0: Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED = 0,
    #[doc = "1: Open Drain Enabled"]
    ODE_1_OPEN_DRAIN_ENABLED = 1,
}
impl From<ODE_A> for bool {
    #[inline(always)]
    fn from(variant: ODE_A) -> Self {
        variant as u8 != 0
    }
}
impl ODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODE_A {
        match self.bits {
            false => ODE_A::ODE_0_OPEN_DRAIN_DISABLED,
            true => ODE_A::ODE_1_OPEN_DRAIN_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ODE_0_OPEN_DRAIN_DISABLED`"]
    #[inline(always)]
    pub fn is_ode_0_open_drain_disabled(&self) -> bool {
        *self == ODE_A::ODE_0_OPEN_DRAIN_DISABLED
    }
    #[doc = "Checks if the value of the field is `ODE_1_OPEN_DRAIN_ENABLED`"]
    #[inline(always)]
    pub fn is_ode_1_open_drain_enabled(&self) -> bool {
        *self == ODE_A::ODE_1_OPEN_DRAIN_ENABLED
    }
}
#[doc = "Field `ODE` writer - Open Drain Enable Field"]
pub type ODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, ODE_A, O>;
impl<'a, const O: u8> ODE_W<'a, O> {
    #[doc = "Open Drain Disabled"]
    #[inline(always)]
    pub fn ode_0_open_drain_disabled(self) -> &'a mut W {
        self.variant(ODE_A::ODE_0_OPEN_DRAIN_DISABLED)
    }
    #[doc = "Open Drain Enabled"]
    #[inline(always)]
    pub fn ode_1_open_drain_enabled(self) -> &'a mut W {
        self.variant(ODE_A::ODE_1_OPEN_DRAIN_ENABLED)
    }
}
#[doc = "Field `PKE` reader - Pull / Keep Enable Field"]
pub type PKE_R = crate::BitReader<PKE_A>;
#[doc = "Pull / Keep Enable Field\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKE_A {
    #[doc = "0: Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0,
    #[doc = "1: Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 1,
}
impl From<PKE_A> for bool {
    #[inline(always)]
    fn from(variant: PKE_A) -> Self {
        variant as u8 != 0
    }
}
impl PKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKE_A {
        match self.bits {
            false => PKE_A::PKE_0_PULL_KEEPER_DISABLED,
            true => PKE_A::PKE_1_PULL_KEEPER_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PKE_0_PULL_KEEPER_DISABLED`"]
    #[inline(always)]
    pub fn is_pke_0_pull_keeper_disabled(&self) -> bool {
        *self == PKE_A::PKE_0_PULL_KEEPER_DISABLED
    }
    #[doc = "Checks if the value of the field is `PKE_1_PULL_KEEPER_ENABLED`"]
    #[inline(always)]
    pub fn is_pke_1_pull_keeper_enabled(&self) -> bool {
        *self == PKE_A::PKE_1_PULL_KEEPER_ENABLED
    }
}
#[doc = "Field `PKE` writer - Pull / Keep Enable Field"]
pub type PKE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, PKE_A, O>;
impl<'a, const O: u8> PKE_W<'a, O> {
    #[doc = "Pull/Keeper Disabled"]
    #[inline(always)]
    pub fn pke_0_pull_keeper_disabled(self) -> &'a mut W {
        self.variant(PKE_A::PKE_0_PULL_KEEPER_DISABLED)
    }
    #[doc = "Pull/Keeper Enabled"]
    #[inline(always)]
    pub fn pke_1_pull_keeper_enabled(self) -> &'a mut W {
        self.variant(PKE_A::PKE_1_PULL_KEEPER_ENABLED)
    }
}
#[doc = "Field `PUE` reader - Pull / Keep Select Field"]
pub type PUE_R = crate::BitReader<PUE_A>;
#[doc = "Pull / Keep Select Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PUE_A {
    #[doc = "0: Keeper"]
    PUE_0_KEEPER = 0,
    #[doc = "1: Pull"]
    PUE_1_PULL = 1,
}
impl From<PUE_A> for bool {
    #[inline(always)]
    fn from(variant: PUE_A) -> Self {
        variant as u8 != 0
    }
}
impl PUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUE_A {
        match self.bits {
            false => PUE_A::PUE_0_KEEPER,
            true => PUE_A::PUE_1_PULL,
        }
    }
    #[doc = "Checks if the value of the field is `PUE_0_KEEPER`"]
    #[inline(always)]
    pub fn is_pue_0_keeper(&self) -> bool {
        *self == PUE_A::PUE_0_KEEPER
    }
    #[doc = "Checks if the value of the field is `PUE_1_PULL`"]
    #[inline(always)]
    pub fn is_pue_1_pull(&self) -> bool {
        *self == PUE_A::PUE_1_PULL
    }
}
#[doc = "Field `PUE` writer - Pull / Keep Select Field"]
pub type PUE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, PUE_A, O>;
impl<'a, const O: u8> PUE_W<'a, O> {
    #[doc = "Keeper"]
    #[inline(always)]
    pub fn pue_0_keeper(self) -> &'a mut W {
        self.variant(PUE_A::PUE_0_KEEPER)
    }
    #[doc = "Pull"]
    #[inline(always)]
    pub fn pue_1_pull(self) -> &'a mut W {
        self.variant(PUE_A::PUE_1_PULL)
    }
}
#[doc = "Field `PUS` reader - Pull Up / Down Config. Field"]
pub type PUS_R = crate::FieldReader<u8, PUS_A>;
#[doc = "Pull Up / Down Config. Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUS_A {
    #[doc = "0: 100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0,
    #[doc = "1: 47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 1,
    #[doc = "2: 100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 2,
    #[doc = "3: 22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 3,
}
impl From<PUS_A> for u8 {
    #[inline(always)]
    fn from(variant: PUS_A) -> Self {
        variant as _
    }
}
impl PUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUS_A {
        match self.bits {
            0 => PUS_A::PUS_0_100K_OHM_PULL_DOWN,
            1 => PUS_A::PUS_1_47K_OHM_PULL_UP,
            2 => PUS_A::PUS_2_100K_OHM_PULL_UP,
            3 => PUS_A::PUS_3_22K_OHM_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUS_0_100K_OHM_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pus_0_100k_ohm_pull_down(&self) -> bool {
        *self == PUS_A::PUS_0_100K_OHM_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PUS_1_47K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_1_47k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_1_47K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_2_100K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_2_100k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_2_100K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_3_22K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_3_22k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_3_22K_OHM_PULL_UP
    }
}
#[doc = "Field `PUS` writer - Pull Up / Down Config. Field"]
pub type PUS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, u8, PUS_A, 2, O>;
impl<'a, const O: u8> PUS_W<'a, O> {
    #[doc = "100K Ohm Pull Down"]
    #[inline(always)]
    pub fn pus_0_100k_ohm_pull_down(self) -> &'a mut W {
        self.variant(PUS_A::PUS_0_100K_OHM_PULL_DOWN)
    }
    #[doc = "47K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_1_47k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_1_47K_OHM_PULL_UP)
    }
    #[doc = "100K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_2_100k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_2_100K_OHM_PULL_UP)
    }
    #[doc = "22K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_3_22k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_3_22K_OHM_PULL_UP)
    }
}
#[doc = "Field `HYS` reader - Hyst. Enable Field"]
pub type HYS_R = crate::BitReader<HYS_A>;
#[doc = "Hyst. Enable Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HYS_A {
    #[doc = "0: Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED = 0,
    #[doc = "1: Hysteresis Enabled"]
    HYS_1_HYSTERESIS_ENABLED = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
impl HYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::HYS_0_HYSTERESIS_DISABLED,
            true => HYS_A::HYS_1_HYSTERESIS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `HYS_0_HYSTERESIS_DISABLED`"]
    #[inline(always)]
    pub fn is_hys_0_hysteresis_disabled(&self) -> bool {
        *self == HYS_A::HYS_0_HYSTERESIS_DISABLED
    }
    #[doc = "Checks if the value of the field is `HYS_1_HYSTERESIS_ENABLED`"]
    #[inline(always)]
    pub fn is_hys_1_hysteresis_enabled(&self) -> bool {
        *self == HYS_A::HYS_1_HYSTERESIS_ENABLED
    }
}
#[doc = "Field `HYS` writer - Hyst. Enable Field"]
pub type HYS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC, HYS_A, O>;
impl<'a, const O: u8> HYS_W<'a, O> {
    #[doc = "Hysteresis Disabled"]
    #[inline(always)]
    pub fn hys_0_hysteresis_disabled(self) -> &'a mut W {
        self.variant(HYS_A::HYS_0_HYSTERESIS_DISABLED)
    }
    #[doc = "Hysteresis Enabled"]
    #[inline(always)]
    pub fn hys_1_hysteresis_enabled(self) -> &'a mut W {
        self.variant(HYS_A::HYS_1_HYSTERESIS_ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline(always)]
    pub fn dse(&self) -> DSE_R {
        DSE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline(always)]
    pub fn ode(&self) -> ODE_R {
        ODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline(always)]
    pub fn pue(&self) -> PUE_R {
        PUE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline(always)]
    pub fn pus(&self) -> PUS_R {
        PUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<0> {
        SRE_W::new(self)
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline(always)]
    #[must_use]
    pub fn dse(&mut self) -> DSE_W<3> {
        DSE_W::new(self)
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<6> {
        SPEED_W::new(self)
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline(always)]
    #[must_use]
    pub fn ode(&mut self) -> ODE_W<11> {
        ODE_W::new(self)
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline(always)]
    #[must_use]
    pub fn pke(&mut self) -> PKE_W<12> {
        PKE_W::new(self)
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline(always)]
    #[must_use]
    pub fn pue(&mut self) -> PUE_W<13> {
        PUE_W::new(self)
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline(always)]
    #[must_use]
    pub fn pus(&mut self) -> PUS_W<14> {
        PUS_W::new(self)
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HYS_W<16> {
        HYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_EMC_06 SW PAD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_pad_ctl_pad_gpio_emc_06](index.html) module"]
pub struct SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC;
impl crate::RegisterSpec for SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_pad_ctl_pad_gpio_emc_06::R](R) reader structure"]
impl crate::Readable for SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_pad_ctl_pad_gpio_emc_06::W](W) writer structure"]
impl crate::Writable for SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_PAD_CTL_PAD_GPIO_EMC_06 to value 0x10b0"]
impl crate::Resettable for SW_PAD_CTL_PAD_GPIO_EMC_06_SPEC {
    const RESET_VALUE: Self::Ux = 0x10b0;
}
