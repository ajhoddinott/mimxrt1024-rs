#[doc = "Register `FCTRL0` reader"]
pub struct R(crate::R<FCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTRL0` writer"]
pub struct W(crate::W<FCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRL0_SPEC>;
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
impl From<crate::W<FCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIE` reader - Fault Interrupt Enables"]
pub type FIE_R = crate::FieldReader<u8, FIE_A>;
#[doc = "Fault Interrupt Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIE_A {
    #[doc = "0: FAULTx CPU interrupt requests disabled."]
    DISABLED = 0,
    #[doc = "1: FAULTx CPU interrupt requests enabled."]
    ENABLED = 1,
}
impl From<FIE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIE_A) -> Self {
        variant as _
    }
}
impl FIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIE_A> {
        match self.bits {
            0 => Some(FIE_A::DISABLED),
            1 => Some(FIE_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIE_A::ENABLED
    }
}
#[doc = "Field `FIE` writer - Fault Interrupt Enables"]
pub type FIE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCTRL0_SPEC, u8, FIE_A, 4, O>;
impl<'a, const O: u8> FIE_W<'a, O> {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIE_A::DISABLED)
    }
    #[doc = "FAULTx CPU interrupt requests enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIE_A::ENABLED)
    }
}
#[doc = "Field `FSAFE` reader - Fault Safety Mode"]
pub type FSAFE_R = crate::FieldReader<u8, FSAFE_A>;
#[doc = "Fault Safety Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSAFE_A {
    #[doc = "0: Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]
without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    NORMAL = 0,
    #[doc = "1: Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear and FSTS\\[FFPINx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    SAFE = 1,
}
impl From<FSAFE_A> for u8 {
    #[inline(always)]
    fn from(variant: FSAFE_A) -> Self {
        variant as _
    }
}
impl FSAFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSAFE_A> {
        match self.bits {
            0 => Some(FSAFE_A::NORMAL),
            1 => Some(FSAFE_A::SAFE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FSAFE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SAFE`"]
    #[inline(always)]
    pub fn is_safe(&self) -> bool {
        *self == FSAFE_A::SAFE
    }
}
#[doc = "Field `FSAFE` writer - Fault Safety Mode"]
pub type FSAFE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCTRL0_SPEC, u8, FSAFE_A, 4, O>;
impl<'a, const O: u8> FSAFE_W<'a, O> {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]
without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FSAFE_A::NORMAL)
    }
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear and FSTS\\[FFPINx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    #[inline(always)]
    pub fn safe(self) -> &'a mut W {
        self.variant(FSAFE_A::SAFE)
    }
}
#[doc = "Field `FAUTO` reader - Automatic Fault Clearing"]
pub type FAUTO_R = crate::FieldReader<u8, FAUTO_A>;
#[doc = "Automatic Fault Clearing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAUTO_A {
    #[doc = "0: Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear at the start of a half cycle or full cycle depending the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    MANUAL = 0,
    #[doc = "1: Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]
without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    AUTOMATIC = 1,
}
impl From<FAUTO_A> for u8 {
    #[inline(always)]
    fn from(variant: FAUTO_A) -> Self {
        variant as _
    }
}
impl FAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAUTO_A> {
        match self.bits {
            0 => Some(FAUTO_A::MANUAL),
            1 => Some(FAUTO_A::AUTOMATIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == FAUTO_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == FAUTO_A::AUTOMATIC
    }
}
#[doc = "Field `FAUTO` writer - Automatic Fault Clearing"]
pub type FAUTO_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCTRL0_SPEC, u8, FAUTO_A, 4, O>;
impl<'a, const O: u8> FAUTO_W<'a, O> {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\]
is clear at the start of a half cycle or full cycle depending the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(FAUTO_A::MANUAL)
    }
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\]
is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\]
and FSTS\\[FFULL\\]
without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(FAUTO_A::AUTOMATIC)
    }
}
#[doc = "Field `FLVL` reader - Fault Level"]
pub type FLVL_R = crate::FieldReader<u8, FLVL_A>;
#[doc = "Fault Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLVL_A {
    #[doc = "0: A logic 0 on the fault input indicates a fault condition."]
    LOGIC_0 = 0,
    #[doc = "1: A logic 1 on the fault input indicates a fault condition."]
    LOGIC_1 = 1,
}
impl From<FLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLVL_A) -> Self {
        variant as _
    }
}
impl FLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLVL_A> {
        match self.bits {
            0 => Some(FLVL_A::LOGIC_0),
            1 => Some(FLVL_A::LOGIC_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOGIC_0`"]
    #[inline(always)]
    pub fn is_logic_0(&self) -> bool {
        *self == FLVL_A::LOGIC_0
    }
    #[doc = "Checks if the value of the field is `LOGIC_1`"]
    #[inline(always)]
    pub fn is_logic_1(&self) -> bool {
        *self == FLVL_A::LOGIC_1
    }
}
#[doc = "Field `FLVL` writer - Fault Level"]
pub type FLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCTRL0_SPEC, u8, FLVL_A, 4, O>;
impl<'a, const O: u8> FLVL_W<'a, O> {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    #[inline(always)]
    pub fn logic_0(self) -> &'a mut W {
        self.variant(FLVL_A::LOGIC_0)
    }
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    #[inline(always)]
    pub fn logic_1(self) -> &'a mut W {
        self.variant(FLVL_A::LOGIC_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline(always)]
    pub fn fie(&self) -> FIE_R {
        FIE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline(always)]
    pub fn fsafe(&self) -> FSAFE_R {
        FSAFE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline(always)]
    pub fn fauto(&self) -> FAUTO_R {
        FAUTO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline(always)]
    #[must_use]
    pub fn fie(&mut self) -> FIE_W<0> {
        FIE_W::new(self)
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fsafe(&mut self) -> FSAFE_W<4> {
        FSAFE_W::new(self)
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline(always)]
    #[must_use]
    pub fn fauto(&mut self) -> FAUTO_W<8> {
        FAUTO_W::new(self)
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline(always)]
    #[must_use]
    pub fn flvl(&mut self) -> FLVL_W<12> {
        FLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl0](index.html) module"]
pub struct FCTRL0_SPEC;
impl crate::RegisterSpec for FCTRL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctrl0::R](R) reader structure"]
impl crate::Readable for FCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrl0::W](W) writer structure"]
impl crate::Writable for FCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL0 to value 0"]
impl crate::Resettable for FCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
