#[doc = "Register `FSTS0` reader"]
pub struct R(crate::R<FSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSTS0` writer"]
pub struct W(crate::W<FSTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSTS0_SPEC>;
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
impl From<crate::W<FSTS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFLAG` reader - Fault Flags"]
pub type FFLAG_R = crate::FieldReader<u8, FFLAG_A>;
#[doc = "Fault Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FFLAG_A {
    #[doc = "0: No fault on the FAULTx pin."]
    NO_FLAG = 0,
    #[doc = "1: Fault on the FAULTx pin."]
    FLAG = 1,
}
impl From<FFLAG_A> for u8 {
    #[inline(always)]
    fn from(variant: FFLAG_A) -> Self {
        variant as _
    }
}
impl FFLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFLAG_A> {
        match self.bits {
            0 => Some(FFLAG_A::NO_FLAG),
            1 => Some(FFLAG_A::FLAG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == FFLAG_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == FFLAG_A::FLAG
    }
}
#[doc = "Field `FFLAG` writer - Fault Flags"]
pub type FFLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSTS0_SPEC, u8, FFLAG_A, 4, O>;
impl<'a, const O: u8> FFLAG_W<'a, O> {
    #[doc = "No fault on the FAULTx pin."]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(FFLAG_A::NO_FLAG)
    }
    #[doc = "Fault on the FAULTx pin."]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(FFLAG_A::FLAG)
    }
}
#[doc = "Field `FFULL` reader - Full Cycle"]
pub type FFULL_R = crate::FieldReader<u8, FFULL_A>;
#[doc = "Full Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FFULL_A {
    #[doc = "0: PWM outputs are not re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_NOT_REENABLED = 0,
    #[doc = "1: PWM outputs are re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_REENABLED = 1,
}
impl From<FFULL_A> for u8 {
    #[inline(always)]
    fn from(variant: FFULL_A) -> Self {
        variant as _
    }
}
impl FFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFULL_A> {
        match self.bits {
            0 => Some(FFULL_A::PWM_OUTPUTS_NOT_REENABLED),
            1 => Some(FFULL_A::PWM_OUTPUTS_REENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_OUTPUTS_NOT_REENABLED`"]
    #[inline(always)]
    pub fn is_pwm_outputs_not_reenabled(&self) -> bool {
        *self == FFULL_A::PWM_OUTPUTS_NOT_REENABLED
    }
    #[doc = "Checks if the value of the field is `PWM_OUTPUTS_REENABLED`"]
    #[inline(always)]
    pub fn is_pwm_outputs_reenabled(&self) -> bool {
        *self == FFULL_A::PWM_OUTPUTS_REENABLED
    }
}
#[doc = "Field `FFULL` writer - Full Cycle"]
pub type FFULL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSTS0_SPEC, u8, FFULL_A, 4, O>;
impl<'a, const O: u8> FFULL_W<'a, O> {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    #[inline(always)]
    pub fn pwm_outputs_not_reenabled(self) -> &'a mut W {
        self.variant(FFULL_A::PWM_OUTPUTS_NOT_REENABLED)
    }
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    #[inline(always)]
    pub fn pwm_outputs_reenabled(self) -> &'a mut W {
        self.variant(FFULL_A::PWM_OUTPUTS_REENABLED)
    }
}
#[doc = "Field `FFPIN` reader - Filtered Fault Pins"]
pub type FFPIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FHALF` reader - Half Cycle Fault Recovery"]
pub type FHALF_R = crate::FieldReader<u8, FHALF_A>;
#[doc = "Half Cycle Fault Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FHALF_A {
    #[doc = "0: PWM outputs are not re-enabled at the start of a half cycle."]
    PWM_OUTPUTS_NOT_REENABLED = 0,
    #[doc = "1: PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    PWM_OUTPUTS_REENABLED = 1,
}
impl From<FHALF_A> for u8 {
    #[inline(always)]
    fn from(variant: FHALF_A) -> Self {
        variant as _
    }
}
impl FHALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FHALF_A> {
        match self.bits {
            0 => Some(FHALF_A::PWM_OUTPUTS_NOT_REENABLED),
            1 => Some(FHALF_A::PWM_OUTPUTS_REENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_OUTPUTS_NOT_REENABLED`"]
    #[inline(always)]
    pub fn is_pwm_outputs_not_reenabled(&self) -> bool {
        *self == FHALF_A::PWM_OUTPUTS_NOT_REENABLED
    }
    #[doc = "Checks if the value of the field is `PWM_OUTPUTS_REENABLED`"]
    #[inline(always)]
    pub fn is_pwm_outputs_reenabled(&self) -> bool {
        *self == FHALF_A::PWM_OUTPUTS_REENABLED
    }
}
#[doc = "Field `FHALF` writer - Half Cycle Fault Recovery"]
pub type FHALF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FSTS0_SPEC, u8, FHALF_A, 4, O>;
impl<'a, const O: u8> FHALF_W<'a, O> {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    #[inline(always)]
    pub fn pwm_outputs_not_reenabled(self) -> &'a mut W {
        self.variant(FHALF_A::PWM_OUTPUTS_NOT_REENABLED)
    }
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    #[inline(always)]
    pub fn pwm_outputs_reenabled(self) -> &'a mut W {
        self.variant(FHALF_A::PWM_OUTPUTS_REENABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline(always)]
    pub fn fflag(&self) -> FFLAG_R {
        FFLAG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline(always)]
    pub fn ffull(&self) -> FFULL_R {
        FFULL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Filtered Fault Pins"]
    #[inline(always)]
    pub fn ffpin(&self) -> FFPIN_R {
        FFPIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline(always)]
    pub fn fhalf(&self) -> FHALF_R {
        FHALF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline(always)]
    #[must_use]
    pub fn fflag(&mut self) -> FFLAG_W<0> {
        FFLAG_W::new(self)
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn ffull(&mut self) -> FFULL_W<4> {
        FFULL_W::new(self)
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline(always)]
    #[must_use]
    pub fn fhalf(&mut self) -> FHALF_W<12> {
        FHALF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsts0](index.html) module"]
pub struct FSTS0_SPEC;
impl crate::RegisterSpec for FSTS0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fsts0::R](R) reader structure"]
impl crate::Readable for FSTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsts0::W](W) writer structure"]
impl crate::Writable for FSTS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSTS0 to value 0"]
impl crate::Resettable for FSTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
