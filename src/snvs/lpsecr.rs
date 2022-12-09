#[doc = "Register `LPSECR` reader"]
pub struct R(crate::R<LPSECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSECR` writer"]
pub struct W(crate::W<LPSECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSECR_SPEC>;
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
impl From<crate::W<LPSECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRTCR_EN` reader - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
pub type SRTCR_EN_R = crate::BitReader<SRTCR_EN_A>;
#[doc = "SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTCR_EN_A {
    #[doc = "0: SRTC rollover is disabled."]
    DISABLED = 0,
    #[doc = "1: SRTC rollover is enabled."]
    ENABLED = 1,
}
impl From<SRTCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRTCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTCR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTCR_EN_A {
        match self.bits {
            false => SRTCR_EN_A::DISABLED,
            true => SRTCR_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRTCR_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRTCR_EN_A::ENABLED
    }
}
#[doc = "Field `SRTCR_EN` writer - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
pub type SRTCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSECR_SPEC, SRTCR_EN_A, O>;
impl<'a, const O: u8> SRTCR_EN_W<'a, O> {
    #[doc = "SRTC rollover is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRTCR_EN_A::DISABLED)
    }
    #[doc = "SRTC rollover is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRTCR_EN_A::ENABLED)
    }
}
#[doc = "Field `MCR_EN` reader - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
pub type MCR_EN_R = crate::BitReader<MCR_EN_A>;
#[doc = "MC Rollover Enable When set, an MC Rollover event generates an LP security violation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCR_EN_A {
    #[doc = "0: MC rollover is disabled."]
    DISABLED = 0,
    #[doc = "1: MC rollover is enabled."]
    ENABLED = 1,
}
impl From<MCR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MCR_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCR_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCR_EN_A {
        match self.bits {
            false => MCR_EN_A::DISABLED,
            true => MCR_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCR_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCR_EN_A::ENABLED
    }
}
#[doc = "Field `MCR_EN` writer - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
pub type MCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSECR_SPEC, MCR_EN_A, O>;
impl<'a, const O: u8> MCR_EN_W<'a, O> {
    #[doc = "MC rollover is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCR_EN_A::DISABLED)
    }
    #[doc = "MC rollover is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCR_EN_A::ENABLED)
    }
}
#[doc = "Field `PFD_OBSERV` reader - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
pub type PFD_OBSERV_R = crate::BitReader<bool>;
#[doc = "Field `PFD_OBSERV` writer - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
pub type PFD_OBSERV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSECR_SPEC, bool, O>;
#[doc = "Field `POR_OBSERV` reader - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
pub type POR_OBSERV_R = crate::BitReader<bool>;
#[doc = "Field `POR_OBSERV` writer - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
pub type POR_OBSERV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSECR_SPEC, bool, O>;
#[doc = "Field `LTDC` reader - Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
pub type LTDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LTDC` writer - Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
pub type LTDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPSECR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HTDC` reader - High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
pub type HTDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HTDC` writer - High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
pub type HTDC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPSECR_SPEC, u8, u8, 3, O>;
#[doc = "Field `VRC` reader - Voltage Reference Configuration These configuration bits are wired as an output of the module."]
pub type VRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VRC` writer - Voltage Reference Configuration These configuration bits are wired as an output of the module."]
pub type VRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPSECR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OSCB` reader - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
pub type OSCB_R = crate::BitReader<OSCB_A>;
#[doc = "Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCB_A {
    #[doc = "0: Normal SRTC clock oscillator not bypassed."]
    NOT_BYPASSED = 0,
    #[doc = "1: Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    BYPASSED = 1,
}
impl From<OSCB_A> for bool {
    #[inline(always)]
    fn from(variant: OSCB_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCB_A {
        match self.bits {
            false => OSCB_A::NOT_BYPASSED,
            true => OSCB_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == OSCB_A::NOT_BYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == OSCB_A::BYPASSED
    }
}
#[doc = "Field `OSCB` writer - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
pub type OSCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSECR_SPEC, OSCB_A, O>;
impl<'a, const O: u8> OSCB_W<'a, O> {
    #[doc = "Normal SRTC clock oscillator not bypassed."]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(OSCB_A::NOT_BYPASSED)
    }
    #[doc = "Normal SRTC clock oscillator bypassed. Alternate clock can drive the SRTC clock source."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(OSCB_A::BYPASSED)
    }
}
impl R {
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn srtcr_en(&self) -> SRTCR_EN_R {
        SRTCR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    pub fn mcr_en(&self) -> MCR_EN_R {
        MCR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    pub fn pfd_observ(&self) -> PFD_OBSERV_R {
        PFD_OBSERV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    pub fn por_observ(&self) -> POR_OBSERV_R {
        POR_OBSERV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub fn ltdc(&self) -> LTDC_R {
        LTDC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[inline(always)]
    pub fn htdc(&self) -> HTDC_R {
        HTDC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    pub fn vrc(&self) -> VRC_R {
        VRC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    pub fn oscb(&self) -> OSCB_R {
        OSCB_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRTC Rollover Enable When set, an SRTC rollover event generates an LP security violation."]
    #[inline(always)]
    #[must_use]
    pub fn srtcr_en(&mut self) -> SRTCR_EN_W<1> {
        SRTCR_EN_W::new(self)
    }
    #[doc = "Bit 2 - MC Rollover Enable When set, an MC Rollover event generates an LP security violation."]
    #[inline(always)]
    #[must_use]
    pub fn mcr_en(&mut self) -> MCR_EN_W<2> {
        MCR_EN_W::new(self)
    }
    #[doc = "Bit 14 - System Power Fail Detector (PFD) Observability Flop The asynchronous reset input of this flop is connected directly to the inverted output of the PFD analog circuitry (external to the SNVS block)"]
    #[inline(always)]
    #[must_use]
    pub fn pfd_observ(&mut self) -> PFD_OBSERV_W<14> {
        PFD_OBSERV_W::new(self)
    }
    #[doc = "Bit 15 - Power On Reset (POR) Observability Flop The asynchronous reset input of this flop is connected directly to the output of the POR analog circuitry (external to the SNVS"]
    #[inline(always)]
    #[must_use]
    pub fn por_observ(&mut self) -> POR_OBSERV_W<15> {
        POR_OBSERV_W::new(self)
    }
    #[doc = "Bits 16:18 - Low Temp Detect Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    #[must_use]
    pub fn ltdc(&mut self) -> LTDC_W<16> {
        LTDC_W::new(self)
    }
    #[doc = "Bits 20:22 - High Temperature Detect Configuration These configuration bits are wired as an output of the module"]
    #[inline(always)]
    #[must_use]
    pub fn htdc(&mut self) -> HTDC_W<20> {
        HTDC_W::new(self)
    }
    #[doc = "Bits 24:26 - Voltage Reference Configuration These configuration bits are wired as an output of the module."]
    #[inline(always)]
    #[must_use]
    pub fn vrc(&mut self) -> VRC_W<24> {
        VRC_W::new(self)
    }
    #[doc = "Bit 28 - Oscillator Bypass When OSCB=1 the osc_bypass signal is asserted"]
    #[inline(always)]
    #[must_use]
    pub fn oscb(&mut self) -> OSCB_W<28> {
        OSCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Security Events Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsecr](index.html) module"]
pub struct LPSECR_SPEC;
impl crate::RegisterSpec for LPSECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsecr::R](R) reader structure"]
impl crate::Readable for LPSECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsecr::W](W) writer structure"]
impl crate::Writable for LPSECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSECR to value 0"]
impl crate::Resettable for LPSECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
