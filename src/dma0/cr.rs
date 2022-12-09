#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDBG` reader - Enable Debug"]
pub type EDBG_R = crate::BitReader<EDBG_A>;
#[doc = "Enable Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDBG_A {
    #[doc = "0: When the chip is in Debug mode, the eDMA continues to operate."]
    DISABLED = 0,
    #[doc = "1: When the chip is in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete."]
    ENABLED = 1,
}
impl From<EDBG_A> for bool {
    #[inline(always)]
    fn from(variant: EDBG_A) -> Self {
        variant as u8 != 0
    }
}
impl EDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDBG_A {
        match self.bits {
            false => EDBG_A::DISABLED,
            true => EDBG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EDBG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EDBG_A::ENABLED
    }
}
#[doc = "Field `EDBG` writer - Enable Debug"]
pub type EDBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EDBG_A, O>;
impl<'a, const O: u8> EDBG_W<'a, O> {
    #[doc = "When the chip is in Debug mode, the eDMA continues to operate."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EDBG_A::DISABLED)
    }
    #[doc = "When the chip is in debug mode, the DMA stalls the start of a new channel. Executing channels are allowed to complete."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EDBG_A::ENABLED)
    }
}
#[doc = "Field `ERCA` reader - Enable Round Robin Channel Arbitration"]
pub type ERCA_R = crate::BitReader<ERCA_A>;
#[doc = "Enable Round Robin Channel Arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERCA_A {
    #[doc = "0: Fixed priority arbitration within each group"]
    DISABLED = 0,
    #[doc = "1: Round robin arbitration within each group"]
    ENABLED = 1,
}
impl From<ERCA_A> for bool {
    #[inline(always)]
    fn from(variant: ERCA_A) -> Self {
        variant as u8 != 0
    }
}
impl ERCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERCA_A {
        match self.bits {
            false => ERCA_A::DISABLED,
            true => ERCA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERCA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERCA_A::ENABLED
    }
}
#[doc = "Field `ERCA` writer - Enable Round Robin Channel Arbitration"]
pub type ERCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERCA_A, O>;
impl<'a, const O: u8> ERCA_W<'a, O> {
    #[doc = "Fixed priority arbitration within each group"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERCA_A::DISABLED)
    }
    #[doc = "Round robin arbitration within each group"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERCA_A::ENABLED)
    }
}
#[doc = "Field `ERGA` reader - Enable Round Robin Group Arbitration"]
pub type ERGA_R = crate::BitReader<ERGA_A>;
#[doc = "Enable Round Robin Group Arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERGA_A {
    #[doc = "0: Fixed priority arbitration"]
    DISABLED = 0,
    #[doc = "1: Round robin arbitration"]
    ENABLED = 1,
}
impl From<ERGA_A> for bool {
    #[inline(always)]
    fn from(variant: ERGA_A) -> Self {
        variant as u8 != 0
    }
}
impl ERGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERGA_A {
        match self.bits {
            false => ERGA_A::DISABLED,
            true => ERGA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERGA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERGA_A::ENABLED
    }
}
#[doc = "Field `ERGA` writer - Enable Round Robin Group Arbitration"]
pub type ERGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ERGA_A, O>;
impl<'a, const O: u8> ERGA_W<'a, O> {
    #[doc = "Fixed priority arbitration"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERGA_A::DISABLED)
    }
    #[doc = "Round robin arbitration"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERGA_A::ENABLED)
    }
}
#[doc = "Field `HOE` reader - Halt On Error"]
pub type HOE_R = crate::BitReader<HOE_A>;
#[doc = "Halt On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOE_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: Error causes HALT field to be automatically set to 1"]
    HALT_ON_ERROR = 1,
}
impl From<HOE_A> for bool {
    #[inline(always)]
    fn from(variant: HOE_A) -> Self {
        variant as u8 != 0
    }
}
impl HOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOE_A {
        match self.bits {
            false => HOE_A::NORMAL_OPS,
            true => HOE_A::HALT_ON_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == HOE_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `HALT_ON_ERROR`"]
    #[inline(always)]
    pub fn is_halt_on_error(&self) -> bool {
        *self == HOE_A::HALT_ON_ERROR
    }
}
#[doc = "Field `HOE` writer - Halt On Error"]
pub type HOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HOE_A, O>;
impl<'a, const O: u8> HOE_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(HOE_A::NORMAL_OPS)
    }
    #[doc = "Error causes HALT field to be automatically set to 1"]
    #[inline(always)]
    pub fn halt_on_error(self) -> &'a mut W {
        self.variant(HOE_A::HALT_ON_ERROR)
    }
}
#[doc = "Field `HALT` reader - Halt eDMA Operations"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halt eDMA Operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: eDMA operations halted"]
    HALT_DMA = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::NORMAL_OPS,
            true => HALT_A::HALT_DMA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == HALT_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `HALT_DMA`"]
    #[inline(always)]
    pub fn is_halt_dma(&self) -> bool {
        *self == HALT_A::HALT_DMA
    }
}
#[doc = "Field `HALT` writer - Halt eDMA Operations"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(HALT_A::NORMAL_OPS)
    }
    #[doc = "eDMA operations halted"]
    #[inline(always)]
    pub fn halt_dma(self) -> &'a mut W {
        self.variant(HALT_A::HALT_DMA)
    }
}
#[doc = "Field `CLM` reader - Continuous Link Mode"]
pub type CLM_R = crate::BitReader<CLM_A>;
#[doc = "Continuous Link Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLM_A {
    #[doc = "0: Continuous link mode is off"]
    CLM_OFF = 0,
    #[doc = "1: Continuous link mode is on"]
    CLM_ON = 1,
}
impl From<CLM_A> for bool {
    #[inline(always)]
    fn from(variant: CLM_A) -> Self {
        variant as u8 != 0
    }
}
impl CLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLM_A {
        match self.bits {
            false => CLM_A::CLM_OFF,
            true => CLM_A::CLM_ON,
        }
    }
    #[doc = "Checks if the value of the field is `CLM_OFF`"]
    #[inline(always)]
    pub fn is_clm_off(&self) -> bool {
        *self == CLM_A::CLM_OFF
    }
    #[doc = "Checks if the value of the field is `CLM_ON`"]
    #[inline(always)]
    pub fn is_clm_on(&self) -> bool {
        *self == CLM_A::CLM_ON
    }
}
#[doc = "Field `CLM` writer - Continuous Link Mode"]
pub type CLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CLM_A, O>;
impl<'a, const O: u8> CLM_W<'a, O> {
    #[doc = "Continuous link mode is off"]
    #[inline(always)]
    pub fn clm_off(self) -> &'a mut W {
        self.variant(CLM_A::CLM_OFF)
    }
    #[doc = "Continuous link mode is on"]
    #[inline(always)]
    pub fn clm_on(self) -> &'a mut W {
        self.variant(CLM_A::CLM_ON)
    }
}
#[doc = "Field `EMLM` reader - Enable Minor Loop Mapping"]
pub type EMLM_R = crate::BitReader<EMLM_A>;
#[doc = "Enable Minor Loop Mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMLM_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<EMLM_A> for bool {
    #[inline(always)]
    fn from(variant: EMLM_A) -> Self {
        variant as u8 != 0
    }
}
impl EMLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMLM_A {
        match self.bits {
            false => EMLM_A::DISABLED,
            true => EMLM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EMLM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EMLM_A::ENABLED
    }
}
#[doc = "Field `EMLM` writer - Enable Minor Loop Mapping"]
pub type EMLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EMLM_A, O>;
impl<'a, const O: u8> EMLM_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EMLM_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EMLM_A::ENABLED)
    }
}
#[doc = "Field `GRP0PRI` reader - Channel Group 0 Priority"]
pub type GRP0PRI_R = crate::BitReader<bool>;
#[doc = "Field `GRP0PRI` writer - Channel Group 0 Priority"]
pub type GRP0PRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GRP1PRI` reader - Channel Group 1 Priority"]
pub type GRP1PRI_R = crate::BitReader<bool>;
#[doc = "Field `GRP1PRI` writer - Channel Group 1 Priority"]
pub type GRP1PRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ECX` reader - Error Cancel Transfer"]
pub type ECX_R = crate::BitReader<ECX_A>;
#[doc = "Error Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECX_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: Cancel the remaining data transfer"]
    CANCEL = 1,
}
impl From<ECX_A> for bool {
    #[inline(always)]
    fn from(variant: ECX_A) -> Self {
        variant as u8 != 0
    }
}
impl ECX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECX_A {
        match self.bits {
            false => ECX_A::NORMAL_OPS,
            true => ECX_A::CANCEL,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == ECX_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `CANCEL`"]
    #[inline(always)]
    pub fn is_cancel(&self) -> bool {
        *self == ECX_A::CANCEL
    }
}
#[doc = "Field `ECX` writer - Error Cancel Transfer"]
pub type ECX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ECX_A, O>;
impl<'a, const O: u8> ECX_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(ECX_A::NORMAL_OPS)
    }
    #[doc = "Cancel the remaining data transfer"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut W {
        self.variant(ECX_A::CANCEL)
    }
}
#[doc = "Field `CX` reader - Cancel Transfer"]
pub type CX_R = crate::BitReader<CX_A>;
#[doc = "Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CX_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: Cancel the remaining data transfer"]
    CANCEL = 1,
}
impl From<CX_A> for bool {
    #[inline(always)]
    fn from(variant: CX_A) -> Self {
        variant as u8 != 0
    }
}
impl CX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX_A {
        match self.bits {
            false => CX_A::NORMAL_OPS,
            true => CX_A::CANCEL,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == CX_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `CANCEL`"]
    #[inline(always)]
    pub fn is_cancel(&self) -> bool {
        *self == CX_A::CANCEL
    }
}
#[doc = "Field `CX` writer - Cancel Transfer"]
pub type CX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CX_A, O>;
impl<'a, const O: u8> CX_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(CX_A::NORMAL_OPS)
    }
    #[doc = "Cancel the remaining data transfer"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut W {
        self.variant(CX_A::CANCEL)
    }
}
#[doc = "Field `ACTIVE` reader - eDMA Active Status"]
pub type ACTIVE_R = crate::BitReader<ACTIVE_A>;
#[doc = "eDMA Active Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTIVE_A {
    #[doc = "0: eDMA is idle"]
    IDLE = 0,
    #[doc = "1: eDMA is executing a channel"]
    ACTIVE = 1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::IDLE,
            true => ACTIVE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ACTIVE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&self) -> EDBG_R {
        EDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&self) -> ERCA_R {
        ERCA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Round Robin Group Arbitration"]
    #[inline(always)]
    pub fn erga(&self) -> ERGA_R {
        ERGA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&self) -> HOE_R {
        HOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt eDMA Operations"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&self) -> CLM_R {
        CLM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&self) -> EMLM_R {
        EMLM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Group 0 Priority"]
    #[inline(always)]
    pub fn grp0pri(&self) -> GRP0PRI_R {
        GRP0PRI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Group 1 Priority"]
    #[inline(always)]
    pub fn grp1pri(&self) -> GRP1PRI_R {
        GRP1PRI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&self) -> CX_R {
        CX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - eDMA Active Status"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    #[must_use]
    pub fn edbg(&mut self) -> EDBG_W<1> {
        EDBG_W::new(self)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn erca(&mut self) -> ERCA_W<2> {
        ERCA_W::new(self)
    }
    #[doc = "Bit 3 - Enable Round Robin Group Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn erga(&mut self) -> ERGA_W<3> {
        ERGA_W::new(self)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    #[must_use]
    pub fn hoe(&mut self) -> HOE_W<4> {
        HOE_W::new(self)
    }
    #[doc = "Bit 5 - Halt eDMA Operations"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<5> {
        HALT_W::new(self)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clm(&mut self) -> CLM_W<6> {
        CLM_W::new(self)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    #[must_use]
    pub fn emlm(&mut self) -> EMLM_W<7> {
        EMLM_W::new(self)
    }
    #[doc = "Bit 8 - Channel Group 0 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn grp0pri(&mut self) -> GRP0PRI_W<8> {
        GRP0PRI_W::new(self)
    }
    #[doc = "Bit 10 - Channel Group 1 Priority"]
    #[inline(always)]
    #[must_use]
    pub fn grp1pri(&mut self) -> GRP1PRI_W<10> {
        GRP1PRI_W::new(self)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ecx(&mut self) -> ECX_W<16> {
        ECX_W::new(self)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cx(&mut self) -> CX_W<17> {
        CX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x0400"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
