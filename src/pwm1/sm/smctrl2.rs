#[doc = "Register `SMCTRL2` reader"]
pub struct R(crate::R<SMCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCTRL2` writer"]
pub struct W(crate::W<SMCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCTRL2_SPEC>;
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
impl From<crate::W<SMCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_SEL` reader - Clock Source Select"]
pub type CLK_SEL_R = crate::FieldReader<u8, CLK_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "0: The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0,
    #[doc = "1: EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 1,
    #[doc = "2: Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    AUX_CLK = 2,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
impl CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SEL_A> {
        match self.bits {
            0 => Some(CLK_SEL_A::IPBUS),
            1 => Some(CLK_SEL_A::EXT_CLK),
            2 => Some(CLK_SEL_A::AUX_CLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IPBUS`"]
    #[inline(always)]
    pub fn is_ipbus(&self) -> bool {
        *self == CLK_SEL_A::IPBUS
    }
    #[doc = "Checks if the value of the field is `EXT_CLK`"]
    #[inline(always)]
    pub fn is_ext_clk(&self) -> bool {
        *self == CLK_SEL_A::EXT_CLK
    }
    #[doc = "Checks if the value of the field is `AUX_CLK`"]
    #[inline(always)]
    pub fn is_aux_clk(&self) -> bool {
        *self == CLK_SEL_A::AUX_CLK
    }
}
#[doc = "Field `CLK_SEL` writer - Clock Source Select"]
pub type CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SMCTRL2_SPEC, u8, CLK_SEL_A, 2, O>;
impl<'a, const O: u8> CLK_SEL_W<'a, O> {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    #[inline(always)]
    pub fn ipbus(self) -> &'a mut W {
        self.variant(CLK_SEL_A::IPBUS)
    }
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    #[inline(always)]
    pub fn ext_clk(self) -> &'a mut W {
        self.variant(CLK_SEL_A::EXT_CLK)
    }
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    #[inline(always)]
    pub fn aux_clk(self) -> &'a mut W {
        self.variant(CLK_SEL_A::AUX_CLK)
    }
}
#[doc = "Field `RELOAD_SEL` reader - Reload Source Select"]
pub type RELOAD_SEL_R = crate::BitReader<RELOAD_SEL_A>;
#[doc = "Reload Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_SEL_A {
    #[doc = "0: The local RELOAD signal is used to reload registers."]
    LOCAL = 0,
    #[doc = "1: The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    MASTER = 1,
}
impl From<RELOAD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_SEL_A {
        match self.bits {
            false => RELOAD_SEL_A::LOCAL,
            true => RELOAD_SEL_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == RELOAD_SEL_A::LOCAL
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == RELOAD_SEL_A::MASTER
    }
}
#[doc = "Field `RELOAD_SEL` writer - Reload Source Select"]
pub type RELOAD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, RELOAD_SEL_A, O>;
impl<'a, const O: u8> RELOAD_SEL_W<'a, O> {
    #[doc = "The local RELOAD signal is used to reload registers."]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(RELOAD_SEL_A::LOCAL)
    }
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(RELOAD_SEL_A::MASTER)
    }
}
#[doc = "Field `FORCE_SEL` reader - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
pub type FORCE_SEL_R = crate::FieldReader<u8, FORCE_SEL_A>;
#[doc = "This read/write bit determines the source of the FORCE OUTPUT signal for this submodule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORCE_SEL_A {
    #[doc = "0: The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0,
    #[doc = "1: The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER = 1,
    #[doc = "2: The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 2,
    #[doc = "3: The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 3,
    #[doc = "4: The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 4,
    #[doc = "5: The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 5,
    #[doc = "6: The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 6,
    #[doc = "7: The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 7,
}
impl From<FORCE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCE_SEL_A) -> Self {
        variant as _
    }
}
impl FORCE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_SEL_A {
        match self.bits {
            0 => FORCE_SEL_A::LOCAL,
            1 => FORCE_SEL_A::MASTER,
            2 => FORCE_SEL_A::LOCAL_RELOAD,
            3 => FORCE_SEL_A::MASTER_RELOAD,
            4 => FORCE_SEL_A::LOCAL_SYNC,
            5 => FORCE_SEL_A::MASTER_SYNC,
            6 => FORCE_SEL_A::EXT_FORCE,
            7 => FORCE_SEL_A::EXT_SYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == FORCE_SEL_A::LOCAL
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == FORCE_SEL_A::MASTER
    }
    #[doc = "Checks if the value of the field is `LOCAL_RELOAD`"]
    #[inline(always)]
    pub fn is_local_reload(&self) -> bool {
        *self == FORCE_SEL_A::LOCAL_RELOAD
    }
    #[doc = "Checks if the value of the field is `MASTER_RELOAD`"]
    #[inline(always)]
    pub fn is_master_reload(&self) -> bool {
        *self == FORCE_SEL_A::MASTER_RELOAD
    }
    #[doc = "Checks if the value of the field is `LOCAL_SYNC`"]
    #[inline(always)]
    pub fn is_local_sync(&self) -> bool {
        *self == FORCE_SEL_A::LOCAL_SYNC
    }
    #[doc = "Checks if the value of the field is `MASTER_SYNC`"]
    #[inline(always)]
    pub fn is_master_sync(&self) -> bool {
        *self == FORCE_SEL_A::MASTER_SYNC
    }
    #[doc = "Checks if the value of the field is `EXT_FORCE`"]
    #[inline(always)]
    pub fn is_ext_force(&self) -> bool {
        *self == FORCE_SEL_A::EXT_FORCE
    }
    #[doc = "Checks if the value of the field is `EXT_SYNC`"]
    #[inline(always)]
    pub fn is_ext_sync(&self) -> bool {
        *self == FORCE_SEL_A::EXT_SYNC
    }
}
#[doc = "Field `FORCE_SEL` writer - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
pub type FORCE_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCTRL2_SPEC, u8, FORCE_SEL_A, 3, O>;
impl<'a, const O: u8> FORCE_SEL_W<'a, O> {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::LOCAL)
    }
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::MASTER)
    }
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    #[inline(always)]
    pub fn local_reload(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::LOCAL_RELOAD)
    }
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn master_reload(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::MASTER_RELOAD)
    }
    #[doc = "The local sync signal from this submodule is used to force updates."]
    #[inline(always)]
    pub fn local_sync(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::LOCAL_SYNC)
    }
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn master_sync(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::MASTER_SYNC)
    }
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    #[inline(always)]
    pub fn ext_force(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::EXT_FORCE)
    }
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    #[inline(always)]
    pub fn ext_sync(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::EXT_SYNC)
    }
}
#[doc = "Field `FORCE` reader - Force Initialization"]
pub type FORCE_R = crate::BitReader<bool>;
#[doc = "Field `FORCE` writer - Force Initialization"]
pub type FORCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
#[doc = "Field `FRCEN` reader - FRCEN"]
pub type FRCEN_R = crate::BitReader<FRCEN_A>;
#[doc = "FRCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRCEN_A {
    #[doc = "0: Initialization from a FORCE_OUT is disabled."]
    DISABLED = 0,
    #[doc = "1: Initialization from a FORCE_OUT is enabled."]
    ENABLED = 1,
}
impl From<FRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRCEN_A {
        match self.bits {
            false => FRCEN_A::DISABLED,
            true => FRCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRCEN_A::ENABLED
    }
}
#[doc = "Field `FRCEN` writer - FRCEN"]
pub type FRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, FRCEN_A, O>;
impl<'a, const O: u8> FRCEN_W<'a, O> {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRCEN_A::DISABLED)
    }
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRCEN_A::ENABLED)
    }
}
#[doc = "Field `INIT_SEL` reader - Initialization Control Select"]
pub type INIT_SEL_R = crate::FieldReader<u8, INIT_SEL_A>;
#[doc = "Initialization Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INIT_SEL_A {
    #[doc = "0: Local sync (PWM_X) causes initialization."]
    PWM_X = 0,
    #[doc = "1: Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    MASTER_RELOAD = 1,
    #[doc = "2: Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    MASTER_SYNC = 2,
    #[doc = "3: EXT_SYNC causes initialization."]
    EXT_SYNC = 3,
}
impl From<INIT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INIT_SEL_A) -> Self {
        variant as _
    }
}
impl INIT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_SEL_A {
        match self.bits {
            0 => INIT_SEL_A::PWM_X,
            1 => INIT_SEL_A::MASTER_RELOAD,
            2 => INIT_SEL_A::MASTER_SYNC,
            3 => INIT_SEL_A::EXT_SYNC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_X`"]
    #[inline(always)]
    pub fn is_pwm_x(&self) -> bool {
        *self == INIT_SEL_A::PWM_X
    }
    #[doc = "Checks if the value of the field is `MASTER_RELOAD`"]
    #[inline(always)]
    pub fn is_master_reload(&self) -> bool {
        *self == INIT_SEL_A::MASTER_RELOAD
    }
    #[doc = "Checks if the value of the field is `MASTER_SYNC`"]
    #[inline(always)]
    pub fn is_master_sync(&self) -> bool {
        *self == INIT_SEL_A::MASTER_SYNC
    }
    #[doc = "Checks if the value of the field is `EXT_SYNC`"]
    #[inline(always)]
    pub fn is_ext_sync(&self) -> bool {
        *self == INIT_SEL_A::EXT_SYNC
    }
}
#[doc = "Field `INIT_SEL` writer - Initialization Control Select"]
pub type INIT_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SMCTRL2_SPEC, u8, INIT_SEL_A, 2, O>;
impl<'a, const O: u8> INIT_SEL_W<'a, O> {
    #[doc = "Local sync (PWM_X) causes initialization."]
    #[inline(always)]
    pub fn pwm_x(self) -> &'a mut W {
        self.variant(INIT_SEL_A::PWM_X)
    }
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    #[inline(always)]
    pub fn master_reload(self) -> &'a mut W {
        self.variant(INIT_SEL_A::MASTER_RELOAD)
    }
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    #[inline(always)]
    pub fn master_sync(self) -> &'a mut W {
        self.variant(INIT_SEL_A::MASTER_SYNC)
    }
    #[doc = "EXT_SYNC causes initialization."]
    #[inline(always)]
    pub fn ext_sync(self) -> &'a mut W {
        self.variant(INIT_SEL_A::EXT_SYNC)
    }
}
#[doc = "Field `PWMX_INIT` reader - PWM_X Initial Value"]
pub type PWMX_INIT_R = crate::BitReader<bool>;
#[doc = "Field `PWMX_INIT` writer - PWM_X Initial Value"]
pub type PWMX_INIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
#[doc = "Field `PWM45_INIT` reader - PWM45 Initial Value"]
pub type PWM45_INIT_R = crate::BitReader<bool>;
#[doc = "Field `PWM45_INIT` writer - PWM45 Initial Value"]
pub type PWM45_INIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
#[doc = "Field `PWM23_INIT` reader - PWM23 Initial Value"]
pub type PWM23_INIT_R = crate::BitReader<bool>;
#[doc = "Field `PWM23_INIT` writer - PWM23 Initial Value"]
pub type PWM23_INIT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
#[doc = "Field `INDEP` reader - Independent or Complementary Pair Operation"]
pub type INDEP_R = crate::BitReader<INDEP_A>;
#[doc = "Independent or Complementary Pair Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INDEP_A {
    #[doc = "0: PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0,
    #[doc = "1: PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 1,
}
impl From<INDEP_A> for bool {
    #[inline(always)]
    fn from(variant: INDEP_A) -> Self {
        variant as u8 != 0
    }
}
impl INDEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INDEP_A {
        match self.bits {
            false => INDEP_A::COMPLEMENTARY,
            true => INDEP_A::INDEPENDENT,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLEMENTARY`"]
    #[inline(always)]
    pub fn is_complementary(&self) -> bool {
        *self == INDEP_A::COMPLEMENTARY
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == INDEP_A::INDEPENDENT
    }
}
#[doc = "Field `INDEP` writer - Independent or Complementary Pair Operation"]
pub type INDEP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, INDEP_A, O>;
impl<'a, const O: u8> INDEP_W<'a, O> {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    #[inline(always)]
    pub fn complementary(self) -> &'a mut W {
        self.variant(INDEP_A::COMPLEMENTARY)
    }
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(INDEP_A::INDEPENDENT)
    }
}
#[doc = "Field `WAITEN` reader - WAIT Enable"]
pub type WAITEN_R = crate::BitReader<bool>;
#[doc = "Field `WAITEN` writer - WAIT Enable"]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
#[doc = "Field `DBGEN` reader - Debug Enable"]
pub type DBGEN_R = crate::BitReader<bool>;
#[doc = "Field `DBGEN` writer - Debug Enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    pub fn force_sel(&self) -> FORCE_SEL_R {
        FORCE_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Force Initialization"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline(always)]
    pub fn frcen(&self) -> FRCEN_R {
        FRCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline(always)]
    pub fn init_sel(&self) -> INIT_SEL_R {
        INIT_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline(always)]
    pub fn pwmx_init(&self) -> PWMX_INIT_R {
        PWMX_INIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline(always)]
    pub fn pwm45_init(&self) -> PWM45_INIT_R {
        PWM45_INIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline(always)]
    pub fn pwm23_init(&self) -> PWM23_INIT_R {
        PWM23_INIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub fn indep(&self) -> INDEP_R {
        INDEP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<0> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W<2> {
        RELOAD_SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    #[must_use]
    pub fn force_sel(&mut self) -> FORCE_SEL_W<3> {
        FORCE_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Force Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn force(&mut self) -> FORCE_W<6> {
        FORCE_W::new(self)
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline(always)]
    #[must_use]
    pub fn frcen(&mut self) -> FRCEN_W<7> {
        FRCEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn init_sel(&mut self) -> INIT_SEL_W<8> {
        INIT_SEL_W::new(self)
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwmx_init(&mut self) -> PWMX_INIT_W<10> {
        PWMX_INIT_W::new(self)
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm45_init(&mut self) -> PWM45_INIT_W<11> {
        PWM45_INIT_W::new(self)
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn pwm23_init(&mut self) -> PWM23_INIT_W<12> {
        PWM23_INIT_W::new(self)
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline(always)]
    #[must_use]
    pub fn indep(&mut self) -> INDEP_W<13> {
        INDEP_W::new(self)
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<14> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<15> {
        DBGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smctrl2](index.html) module"]
pub struct SMCTRL2_SPEC;
impl crate::RegisterSpec for SMCTRL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smctrl2::R](R) reader structure"]
impl crate::Readable for SMCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smctrl2::W](W) writer structure"]
impl crate::Writable for SMCTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCTRL2 to value 0"]
impl crate::Resettable for SMCTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
