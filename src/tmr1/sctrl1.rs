#[doc = "Register `SCTRL1` reader"]
pub struct R(crate::R<SCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTRL1` writer"]
pub struct W(crate::W<SCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTRL1_SPEC>;
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
impl From<crate::W<SCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEN` reader - Output Enable"]
pub type OEN_R = crate::BitReader<OEN_A>;
#[doc = "Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OEN_A {
    #[doc = "0: The external pin is configured as an input."]
    INPUT = 0,
    #[doc = "1: The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OFLAG_OUT = 1,
}
impl From<OEN_A> for bool {
    #[inline(always)]
    fn from(variant: OEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEN_A {
        match self.bits {
            false => OEN_A::INPUT,
            true => OEN_A::OFLAG_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == OEN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OFLAG_OUT`"]
    #[inline(always)]
    pub fn is_oflag_out(&self) -> bool {
        *self == OEN_A::OFLAG_OUT
    }
}
#[doc = "Field `OEN` writer - Output Enable"]
pub type OEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, OEN_A, O>;
impl<'a, const O: u8> OEN_W<'a, O> {
    #[doc = "The external pin is configured as an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(OEN_A::INPUT)
    }
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    #[inline(always)]
    pub fn oflag_out(self) -> &'a mut W {
        self.variant(OEN_A::OFLAG_OUT)
    }
}
#[doc = "Field `OPS` reader - Output Polarity Select"]
pub type OPS_R = crate::BitReader<OPS_A>;
#[doc = "Output Polarity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPS_A {
    #[doc = "0: True polarity."]
    TRUE = 0,
    #[doc = "1: Inverted polarity."]
    INVERTED = 1,
}
impl From<OPS_A> for bool {
    #[inline(always)]
    fn from(variant: OPS_A) -> Self {
        variant as u8 != 0
    }
}
impl OPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPS_A {
        match self.bits {
            false => OPS_A::TRUE,
            true => OPS_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == OPS_A::TRUE
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == OPS_A::INVERTED
    }
}
#[doc = "Field `OPS` writer - Output Polarity Select"]
pub type OPS_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, OPS_A, O>;
impl<'a, const O: u8> OPS_W<'a, O> {
    #[doc = "True polarity."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(OPS_A::TRUE)
    }
    #[doc = "Inverted polarity."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(OPS_A::INVERTED)
    }
}
#[doc = "Field `FORCE` reader - Force OFLAG Output"]
pub type FORCE_R = crate::BitReader<bool>;
#[doc = "Field `FORCE` writer - Force OFLAG Output"]
pub type FORCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `VAL` reader - Forced OFLAG Value"]
pub type VAL_R = crate::BitReader<bool>;
#[doc = "Field `VAL` writer - Forced OFLAG Value"]
pub type VAL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `EEOF` reader - Enable External OFLAG Force"]
pub type EEOF_R = crate::BitReader<bool>;
#[doc = "Field `EEOF` writer - Enable External OFLAG Force"]
pub type EEOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `MSTR` reader - Master Mode"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - Master Mode"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `CAPTURE_MODE` reader - Input Capture Mode"]
pub type CAPTURE_MODE_R = crate::FieldReader<u8, CAPTURE_MODE_A>;
#[doc = "Input Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTURE_MODE_A {
    #[doc = "0: Capture function is disabled"]
    DISABLED = 0,
    #[doc = "1: Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 1,
    #[doc = "2: Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 2,
    #[doc = "3: Load capture register on both edges of input"]
    ENABLE_BOTH = 3,
}
impl From<CAPTURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE_MODE_A) -> Self {
        variant as _
    }
}
impl CAPTURE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_MODE_A {
        match self.bits {
            0 => CAPTURE_MODE_A::DISABLED,
            1 => CAPTURE_MODE_A::ENABLE_RISING,
            2 => CAPTURE_MODE_A::ENABLE_FALLING,
            3 => CAPTURE_MODE_A::ENABLE_BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPTURE_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLE_RISING`"]
    #[inline(always)]
    pub fn is_enable_rising(&self) -> bool {
        *self == CAPTURE_MODE_A::ENABLE_RISING
    }
    #[doc = "Checks if the value of the field is `ENABLE_FALLING`"]
    #[inline(always)]
    pub fn is_enable_falling(&self) -> bool {
        *self == CAPTURE_MODE_A::ENABLE_FALLING
    }
    #[doc = "Checks if the value of the field is `ENABLE_BOTH`"]
    #[inline(always)]
    pub fn is_enable_both(&self) -> bool {
        *self == CAPTURE_MODE_A::ENABLE_BOTH
    }
}
#[doc = "Field `CAPTURE_MODE` writer - Input Capture Mode"]
pub type CAPTURE_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, SCTRL1_SPEC, u8, CAPTURE_MODE_A, 2, O>;
impl<'a, const O: u8> CAPTURE_MODE_W<'a, O> {
    #[doc = "Capture function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::DISABLED)
    }
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    #[inline(always)]
    pub fn enable_rising(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::ENABLE_RISING)
    }
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    #[inline(always)]
    pub fn enable_falling(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::ENABLE_FALLING)
    }
    #[doc = "Load capture register on both edges of input"]
    #[inline(always)]
    pub fn enable_both(self) -> &'a mut W {
        self.variant(CAPTURE_MODE_A::ENABLE_BOTH)
    }
}
#[doc = "Field `INPUT` reader - External Input Signal"]
pub type INPUT_R = crate::BitReader<bool>;
#[doc = "Field `IPS` reader - Input Polarity Select"]
pub type IPS_R = crate::BitReader<bool>;
#[doc = "Field `IPS` writer - Input Polarity Select"]
pub type IPS_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `IEFIE` reader - Input Edge Flag Interrupt Enable"]
pub type IEFIE_R = crate::BitReader<bool>;
#[doc = "Field `IEFIE` writer - Input Edge Flag Interrupt Enable"]
pub type IEFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `IEF` reader - Input Edge Flag"]
pub type IEF_R = crate::BitReader<bool>;
#[doc = "Field `IEF` writer - Input Edge Flag"]
pub type IEF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `TOFIE` reader - Timer Overflow Flag Interrupt Enable"]
pub type TOFIE_R = crate::BitReader<bool>;
#[doc = "Field `TOFIE` writer - Timer Overflow Flag Interrupt Enable"]
pub type TOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub type TOF_R = crate::BitReader<bool>;
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub type TOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `TCFIE` reader - Timer Compare Flag Interrupt Enable"]
pub type TCFIE_R = crate::BitReader<bool>;
#[doc = "Field `TCFIE` writer - Timer Compare Flag Interrupt Enable"]
pub type TCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Timer Compare Flag"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Timer Compare Flag"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u16, SCTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Output Enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline(always)]
    pub fn ops(&self) -> OPS_R {
        OPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force OFLAG Output"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline(always)]
    pub fn eeof(&self) -> EEOF_R {
        EEOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline(always)]
    pub fn capture_mode(&self) -> CAPTURE_MODE_R {
        CAPTURE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External Input Signal"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline(always)]
    pub fn ips(&self) -> IPS_R {
        IPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    pub fn iefie(&self) -> IEFIE_R {
        IEFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tofie(&self) -> TOFIE_R {
        TOFIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    pub fn tcfie(&self) -> TCFIE_R {
        TCFIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<0> {
        OEN_W::new(self)
    }
    #[doc = "Bit 1 - Output Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn ops(&mut self) -> OPS_W<1> {
        OPS_W::new(self)
    }
    #[doc = "Bit 2 - Force OFLAG Output"]
    #[inline(always)]
    #[must_use]
    pub fn force(&mut self) -> FORCE_W<2> {
        FORCE_W::new(self)
    }
    #[doc = "Bit 3 - Forced OFLAG Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<3> {
        VAL_W::new(self)
    }
    #[doc = "Bit 4 - Enable External OFLAG Force"]
    #[inline(always)]
    #[must_use]
    pub fn eeof(&mut self) -> EEOF_W<4> {
        EEOF_W::new(self)
    }
    #[doc = "Bit 5 - Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<5> {
        MSTR_W::new(self)
    }
    #[doc = "Bits 6:7 - Input Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn capture_mode(&mut self) -> CAPTURE_MODE_W<6> {
        CAPTURE_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Input Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn ips(&mut self) -> IPS_W<9> {
        IPS_W::new(self)
    }
    #[doc = "Bit 10 - Input Edge Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iefie(&mut self) -> IEFIE_W<10> {
        IEFIE_W::new(self)
    }
    #[doc = "Bit 11 - Input Edge Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ief(&mut self) -> IEF_W<11> {
        IEF_W::new(self)
    }
    #[doc = "Bit 12 - Timer Overflow Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tofie(&mut self) -> TOFIE_W<12> {
        TOFIE_W::new(self)
    }
    #[doc = "Bit 13 - Timer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<13> {
        TOF_W::new(self)
    }
    #[doc = "Bit 14 - Timer Compare Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcfie(&mut self) -> TCFIE_W<14> {
        TCFIE_W::new(self)
    }
    #[doc = "Bit 15 - Timer Compare Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<15> {
        TCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrl1](index.html) module"]
pub struct SCTRL1_SPEC;
impl crate::RegisterSpec for SCTRL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sctrl1::R](R) reader structure"]
impl crate::Readable for SCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctrl1::W](W) writer structure"]
impl crate::Writable for SCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTRL1 to value 0"]
impl crate::Resettable for SCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
