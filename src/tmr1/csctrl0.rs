#[doc = "Register `CSCTRL0` reader"]
pub struct R(crate::R<CSCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTRL0` writer"]
pub struct W(crate::W<CSCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTRL0_SPEC>;
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
impl From<crate::W<CSCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CL1` reader - Compare Load Control 1"]
pub type CL1_R = crate::FieldReader<u8, CL1_A>;
#[doc = "Compare Load Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CL1_A {
    #[doc = "0: Never preload"]
    NEVER = 0,
    #[doc = "1: Load upon successful compare with the value in COMP1"]
    COMP1 = 1,
    #[doc = "2: Load upon successful compare with the value in COMP2"]
    COMP2 = 2,
}
impl From<CL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CL1_A) -> Self {
        variant as _
    }
}
impl CL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CL1_A> {
        match self.bits {
            0 => Some(CL1_A::NEVER),
            1 => Some(CL1_A::COMP1),
            2 => Some(CL1_A::COMP2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == CL1_A::NEVER
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == CL1_A::COMP1
    }
    #[doc = "Checks if the value of the field is `COMP2`"]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == CL1_A::COMP2
    }
}
#[doc = "Field `CL1` writer - Compare Load Control 1"]
pub type CL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTRL0_SPEC, u8, CL1_A, 2, O>;
impl<'a, const O: u8> CL1_W<'a, O> {
    #[doc = "Never preload"]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(CL1_A::NEVER)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CL1_A::COMP1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut W {
        self.variant(CL1_A::COMP2)
    }
}
#[doc = "Field `CL2` reader - Compare Load Control 2"]
pub type CL2_R = crate::FieldReader<u8, CL2_A>;
#[doc = "Compare Load Control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CL2_A {
    #[doc = "0: Never preload"]
    NEVER = 0,
    #[doc = "1: Load upon successful compare with the value in COMP1"]
    COMP1 = 1,
    #[doc = "2: Load upon successful compare with the value in COMP2"]
    COMP2 = 2,
}
impl From<CL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CL2_A) -> Self {
        variant as _
    }
}
impl CL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CL2_A> {
        match self.bits {
            0 => Some(CL2_A::NEVER),
            1 => Some(CL2_A::COMP1),
            2 => Some(CL2_A::COMP2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == CL2_A::NEVER
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == CL2_A::COMP1
    }
    #[doc = "Checks if the value of the field is `COMP2`"]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == CL2_A::COMP2
    }
}
#[doc = "Field `CL2` writer - Compare Load Control 2"]
pub type CL2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTRL0_SPEC, u8, CL2_A, 2, O>;
impl<'a, const O: u8> CL2_W<'a, O> {
    #[doc = "Never preload"]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(CL2_A::NEVER)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CL2_A::COMP1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut W {
        self.variant(CL2_A::COMP2)
    }
}
#[doc = "Field `TCF1` reader - Timer Compare 1 Interrupt Flag"]
pub type TCF1_R = crate::BitReader<bool>;
#[doc = "Field `TCF1` writer - Timer Compare 1 Interrupt Flag"]
pub type TCF1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, bool, O>;
#[doc = "Field `TCF2` reader - Timer Compare 2 Interrupt Flag"]
pub type TCF2_R = crate::BitReader<bool>;
#[doc = "Field `TCF2` writer - Timer Compare 2 Interrupt Flag"]
pub type TCF2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, bool, O>;
#[doc = "Field `TCF1EN` reader - Timer Compare 1 Interrupt Enable"]
pub type TCF1EN_R = crate::BitReader<bool>;
#[doc = "Field `TCF1EN` writer - Timer Compare 1 Interrupt Enable"]
pub type TCF1EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, bool, O>;
#[doc = "Field `TCF2EN` reader - Timer Compare 2 Interrupt Enable"]
pub type TCF2EN_R = crate::BitReader<bool>;
#[doc = "Field `TCF2EN` writer - Timer Compare 2 Interrupt Enable"]
pub type TCF2EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, bool, O>;
#[doc = "Field `UP` reader - Counting Direction Indicator"]
pub type UP_R = crate::BitReader<UP_A>;
#[doc = "Counting Direction Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UP_A {
    #[doc = "0: The last count was in the DOWN direction."]
    DOWN = 0,
    #[doc = "1: The last count was in the UP direction."]
    UP = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
impl UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_A {
        match self.bits {
            false => UP_A::DOWN,
            true => UP_A::UP,
        }
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == UP_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == UP_A::UP
    }
}
#[doc = "Field `TCI` reader - Triggered Count Initialization Control"]
pub type TCI_R = crate::BitReader<TCI_A>;
#[doc = "Triggered Count Initialization Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCI_A {
    #[doc = "0: Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    STOP = 0,
    #[doc = "1: Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    RELOAD = 1,
}
impl From<TCI_A> for bool {
    #[inline(always)]
    fn from(variant: TCI_A) -> Self {
        variant as u8 != 0
    }
}
impl TCI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCI_A {
        match self.bits {
            false => TCI_A::STOP,
            true => TCI_A::RELOAD,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == TCI_A::STOP
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == TCI_A::RELOAD
    }
}
#[doc = "Field `TCI` writer - Triggered Count Initialization Control"]
pub type TCI_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, TCI_A, O>;
impl<'a, const O: u8> TCI_W<'a, O> {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(TCI_A::STOP)
    }
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(TCI_A::RELOAD)
    }
}
#[doc = "Field `ROC` reader - Reload on Capture"]
pub type ROC_R = crate::BitReader<ROC_A>;
#[doc = "Reload on Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROC_A {
    #[doc = "0: Do not reload the counter on a capture event."]
    DISABLE = 0,
    #[doc = "1: Reload the counter on a capture event."]
    ENABLE = 1,
}
impl From<ROC_A> for bool {
    #[inline(always)]
    fn from(variant: ROC_A) -> Self {
        variant as u8 != 0
    }
}
impl ROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROC_A {
        match self.bits {
            false => ROC_A::DISABLE,
            true => ROC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROC_A::ENABLE
    }
}
#[doc = "Field `ROC` writer - Reload on Capture"]
pub type ROC_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, ROC_A, O>;
impl<'a, const O: u8> ROC_W<'a, O> {
    #[doc = "Do not reload the counter on a capture event."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROC_A::DISABLE)
    }
    #[doc = "Reload the counter on a capture event."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROC_A::ENABLE)
    }
}
#[doc = "Field `ALT_LOAD` reader - Alternative Load Enable"]
pub type ALT_LOAD_R = crate::BitReader<ALT_LOAD_A>;
#[doc = "Alternative Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALT_LOAD_A {
    #[doc = "0: Counter can be re-initialized only with the LOAD register."]
    DISABLE = 0,
    #[doc = "1: Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    ENABLE = 1,
}
impl From<ALT_LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: ALT_LOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl ALT_LOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALT_LOAD_A {
        match self.bits {
            false => ALT_LOAD_A::DISABLE,
            true => ALT_LOAD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALT_LOAD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALT_LOAD_A::ENABLE
    }
}
#[doc = "Field `ALT_LOAD` writer - Alternative Load Enable"]
pub type ALT_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, ALT_LOAD_A, O>;
impl<'a, const O: u8> ALT_LOAD_W<'a, O> {
    #[doc = "Counter can be re-initialized only with the LOAD register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALT_LOAD_A::DISABLE)
    }
    #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALT_LOAD_A::ENABLE)
    }
}
#[doc = "Field `FAULT` reader - Fault Enable"]
pub type FAULT_R = crate::BitReader<FAULT_A>;
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULT_A {
    #[doc = "0: Fault function disabled."]
    DISABLE = 0,
    #[doc = "1: Fault function enabled."]
    ENABLE = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT_A {
        match self.bits {
            false => FAULT_A::DISABLE,
            true => FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FAULT_A::ENABLE
    }
}
#[doc = "Field `FAULT` writer - Fault Enable"]
pub type FAULT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTRL0_SPEC, FAULT_A, O>;
impl<'a, const O: u8> FAULT_W<'a, O> {
    #[doc = "Fault function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FAULT_A::DISABLE)
    }
    #[doc = "Fault function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FAULT_A::ENABLE)
    }
}
#[doc = "Field `DBG_EN` reader - Debug Actions Enable"]
pub type DBG_EN_R = crate::FieldReader<u8, DBG_EN_A>;
#[doc = "Debug Actions Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBG_EN_A {
    #[doc = "0: Continue with normal operation during debug mode. (default)"]
    NORMAL = 0,
    #[doc = "1: Halt TMR counter during debug mode."]
    HALT_TMR = 1,
    #[doc = "2: Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 2,
    #[doc = "3: Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 3,
}
impl From<DBG_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        variant as _
    }
}
impl DBG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            0 => DBG_EN_A::NORMAL,
            1 => DBG_EN_A::HALT_TMR,
            2 => DBG_EN_A::FORCE_0,
            3 => DBG_EN_A::HALT_AND_FORCE_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DBG_EN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HALT_TMR`"]
    #[inline(always)]
    pub fn is_halt_tmr(&self) -> bool {
        *self == DBG_EN_A::HALT_TMR
    }
    #[doc = "Checks if the value of the field is `FORCE_0`"]
    #[inline(always)]
    pub fn is_force_0(&self) -> bool {
        *self == DBG_EN_A::FORCE_0
    }
    #[doc = "Checks if the value of the field is `HALT_AND_FORCE_0`"]
    #[inline(always)]
    pub fn is_halt_and_force_0(&self) -> bool {
        *self == DBG_EN_A::HALT_AND_FORCE_0
    }
}
#[doc = "Field `DBG_EN` writer - Debug Actions Enable"]
pub type DBG_EN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTRL0_SPEC, u8, DBG_EN_A, 2, O>;
impl<'a, const O: u8> DBG_EN_W<'a, O> {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DBG_EN_A::NORMAL)
    }
    #[doc = "Halt TMR counter during debug mode."]
    #[inline(always)]
    pub fn halt_tmr(self) -> &'a mut W {
        self.variant(DBG_EN_A::HALT_TMR)
    }
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    #[inline(always)]
    pub fn force_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::FORCE_0)
    }
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    #[inline(always)]
    pub fn halt_and_force_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::HALT_AND_FORCE_0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf1(&self) -> TCF1_R {
        TCF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf2(&self) -> TCF2_R {
        TCF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf1en(&self) -> TCF1EN_R {
        TCF1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf2en(&self) -> TCF2EN_R {
        TCF2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Counting Direction Indicator"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline(always)]
    pub fn tci(&self) -> TCI_R {
        TCI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline(always)]
    pub fn roc(&self) -> ROC_R {
        ROC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline(always)]
    pub fn alt_load(&self) -> ALT_LOAD_R {
        ALT_LOAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline(always)]
    #[must_use]
    pub fn cl1(&mut self) -> CL1_W<0> {
        CL1_W::new(self)
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline(always)]
    #[must_use]
    pub fn cl2(&mut self) -> CL2_W<2> {
        CL2_W::new(self)
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf1(&mut self) -> TCF1_W<4> {
        TCF1_W::new(self)
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf2(&mut self) -> TCF2_W<5> {
        TCF2_W::new(self)
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcf1en(&mut self) -> TCF1EN_W<6> {
        TCF1EN_W::new(self)
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcf2en(&mut self) -> TCF2EN_W<7> {
        TCF2EN_W::new(self)
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline(always)]
    #[must_use]
    pub fn tci(&mut self) -> TCI_W<10> {
        TCI_W::new(self)
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline(always)]
    #[must_use]
    pub fn roc(&mut self) -> ROC_W<11> {
        ROC_W::new(self)
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alt_load(&mut self) -> ALT_LOAD_W<12> {
        ALT_LOAD_W::new(self)
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FAULT_W<13> {
        FAULT_W::new(self)
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_en(&mut self) -> DBG_EN_W<14> {
        DBG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Comparator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctrl0](index.html) module"]
pub struct CSCTRL0_SPEC;
impl crate::RegisterSpec for CSCTRL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctrl0::R](R) reader structure"]
impl crate::Readable for CSCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctrl0::W](W) writer structure"]
impl crate::Writable for CSCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCTRL0 to value 0"]
impl crate::Resettable for CSCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
