#[doc = "Register `OUTEN` reader"]
pub struct R(crate::R<OUTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTEN` writer"]
pub struct W(crate::W<OUTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTEN_SPEC>;
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
impl From<crate::W<OUTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWMX_EN` reader - PWM_X Output Enables"]
pub type PWMX_EN_R = crate::FieldReader<u8, PWMX_EN_A>;
#[doc = "PWM_X Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMX_EN_A {
    #[doc = "0: PWM_X output disabled."]
    DISABLED = 0,
    #[doc = "1: PWM_X output enabled."]
    ENABLED = 1,
}
impl From<PWMX_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMX_EN_A) -> Self {
        variant as _
    }
}
impl PWMX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWMX_EN_A> {
        match self.bits {
            0 => Some(PWMX_EN_A::DISABLED),
            1 => Some(PWMX_EN_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMX_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMX_EN_A::ENABLED
    }
}
#[doc = "Field `PWMX_EN` writer - PWM_X Output Enables"]
pub type PWMX_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, OUTEN_SPEC, u8, PWMX_EN_A, 4, O>;
impl<'a, const O: u8> PWMX_EN_W<'a, O> {
    #[doc = "PWM_X output disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMX_EN_A::DISABLED)
    }
    #[doc = "PWM_X output enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMX_EN_A::ENABLED)
    }
}
#[doc = "Field `PWMB_EN` reader - PWM_B Output Enables"]
pub type PWMB_EN_R = crate::FieldReader<u8, PWMB_EN_A>;
#[doc = "PWM_B Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMB_EN_A {
    #[doc = "0: PWM_B output disabled."]
    DISABLED = 0,
    #[doc = "1: PWM_B output enabled."]
    ENABLED = 1,
}
impl From<PWMB_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMB_EN_A) -> Self {
        variant as _
    }
}
impl PWMB_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWMB_EN_A> {
        match self.bits {
            0 => Some(PWMB_EN_A::DISABLED),
            1 => Some(PWMB_EN_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMB_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMB_EN_A::ENABLED
    }
}
#[doc = "Field `PWMB_EN` writer - PWM_B Output Enables"]
pub type PWMB_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, OUTEN_SPEC, u8, PWMB_EN_A, 4, O>;
impl<'a, const O: u8> PWMB_EN_W<'a, O> {
    #[doc = "PWM_B output disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMB_EN_A::DISABLED)
    }
    #[doc = "PWM_B output enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMB_EN_A::ENABLED)
    }
}
#[doc = "Field `PWMA_EN` reader - PWM_A Output Enables"]
pub type PWMA_EN_R = crate::FieldReader<u8, PWMA_EN_A>;
#[doc = "PWM_A Output Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMA_EN_A {
    #[doc = "0: PWM_A output disabled."]
    DISABLED = 0,
    #[doc = "1: PWM_A output enabled."]
    ENABLED = 1,
}
impl From<PWMA_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: PWMA_EN_A) -> Self {
        variant as _
    }
}
impl PWMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWMA_EN_A> {
        match self.bits {
            0 => Some(PWMA_EN_A::DISABLED),
            1 => Some(PWMA_EN_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMA_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMA_EN_A::ENABLED
    }
}
#[doc = "Field `PWMA_EN` writer - PWM_A Output Enables"]
pub type PWMA_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, OUTEN_SPEC, u8, PWMA_EN_A, 4, O>;
impl<'a, const O: u8> PWMA_EN_W<'a, O> {
    #[doc = "PWM_A output disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMA_EN_A::DISABLED)
    }
    #[doc = "PWM_A output enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMA_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline(always)]
    pub fn pwmx_en(&self) -> PWMX_EN_R {
        PWMX_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline(always)]
    pub fn pwmb_en(&self) -> PWMB_EN_R {
        PWMB_EN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline(always)]
    pub fn pwma_en(&self) -> PWMA_EN_R {
        PWMA_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_X Output Enables"]
    #[inline(always)]
    #[must_use]
    pub fn pwmx_en(&mut self) -> PWMX_EN_W<0> {
        PWMX_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - PWM_B Output Enables"]
    #[inline(always)]
    #[must_use]
    pub fn pwmb_en(&mut self) -> PWMB_EN_W<4> {
        PWMB_EN_W::new(self)
    }
    #[doc = "Bits 8:11 - PWM_A Output Enables"]
    #[inline(always)]
    #[must_use]
    pub fn pwma_en(&mut self) -> PWMA_EN_W<8> {
        PWMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outen](index.html) module"]
pub struct OUTEN_SPEC;
impl crate::RegisterSpec for OUTEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [outen::R](R) reader structure"]
impl crate::Readable for OUTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outen::W](W) writer structure"]
impl crate::Writable for OUTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OUTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
