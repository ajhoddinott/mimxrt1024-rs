#[doc = "Register `FCTRL20` reader"]
pub struct R(crate::R<FCTRL20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRL20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRL20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRL20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTRL20` writer"]
pub struct W(crate::W<FCTRL20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRL20_SPEC>;
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
impl From<crate::W<FCTRL20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRL20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOCOMB` reader - No Combinational Path From Fault Input To PWM Output"]
pub type NOCOMB_R = crate::FieldReader<u8, NOCOMB_A>;
#[doc = "No Combinational Path From Fault Input To PWM Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NOCOMB_A {
    #[doc = "0: There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    ENABLED = 0,
    #[doc = "1: The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    DISABLED = 1,
}
impl From<NOCOMB_A> for u8 {
    #[inline(always)]
    fn from(variant: NOCOMB_A) -> Self {
        variant as _
    }
}
impl NOCOMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NOCOMB_A> {
        match self.bits {
            0 => Some(NOCOMB_A::ENABLED),
            1 => Some(NOCOMB_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOCOMB_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOCOMB_A::DISABLED
    }
}
#[doc = "Field `NOCOMB` writer - No Combinational Path From Fault Input To PWM Output"]
pub type NOCOMB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FCTRL20_SPEC, u8, NOCOMB_A, 4, O>;
impl<'a, const O: u8> NOCOMB_W<'a, O> {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOCOMB_A::ENABLED)
    }
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOCOMB_A::DISABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub fn nocomb(&self) -> NOCOMB_R {
        NOCOMB_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    #[must_use]
    pub fn nocomb(&mut self) -> NOCOMB_W<0> {
        NOCOMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl20](index.html) module"]
pub struct FCTRL20_SPEC;
impl crate::RegisterSpec for FCTRL20_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fctrl20::R](R) reader structure"]
impl crate::Readable for FCTRL20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrl20::W](W) writer structure"]
impl crate::Writable for FCTRL20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL20 to value 0"]
impl crate::Resettable for FCTRL20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
