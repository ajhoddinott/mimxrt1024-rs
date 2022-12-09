#[doc = "Register `MCTRL` reader"]
pub struct R(crate::R<MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL` writer"]
pub struct W(crate::W<MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL_SPEC>;
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
impl From<crate::W<MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOK` reader - Load Okay"]
pub type LDOK_R = crate::FieldReader<u8, LDOK_A>;
#[doc = "Load Okay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDOK_A {
    #[doc = "0: Do not load new values."]
    DISABLED = 0,
    #[doc = "1: Load prescaler, modulus, and PWM values of the corresponding submodule."]
    ENABLED = 1,
}
impl From<LDOK_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOK_A) -> Self {
        variant as _
    }
}
impl LDOK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LDOK_A> {
        match self.bits {
            0 => Some(LDOK_A::DISABLED),
            1 => Some(LDOK_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LDOK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LDOK_A::ENABLED
    }
}
#[doc = "Field `LDOK` writer - Load Okay"]
pub type LDOK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MCTRL_SPEC, u8, LDOK_A, 4, O>;
impl<'a, const O: u8> LDOK_W<'a, O> {
    #[doc = "Do not load new values."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LDOK_A::DISABLED)
    }
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LDOK_A::ENABLED)
    }
}
#[doc = "Field `CLDOK` reader - Clear Load Okay"]
pub type CLDOK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLDOK` writer - Clear Load Okay"]
pub type CLDOK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RUN` reader - Run"]
pub type RUN_R = crate::FieldReader<u8, RUN_A>;
#[doc = "Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RUN_A {
    #[doc = "0: PWM counter is stopped, but PWM outputs will hold the current state."]
    DISABLED = 0,
    #[doc = "1: PWM counter is started in the corresponding submodule."]
    ENABLED = 1,
}
impl From<RUN_A> for u8 {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as _
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUN_A> {
        match self.bits {
            0 => Some(RUN_A::DISABLED),
            1 => Some(RUN_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RUN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RUN_A::ENABLED
    }
}
#[doc = "Field `RUN` writer - Run"]
pub type RUN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MCTRL_SPEC, u8, RUN_A, 4, O>;
impl<'a, const O: u8> RUN_W<'a, O> {
    #[doc = "PWM counter is stopped, but PWM outputs will hold the current state."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RUN_A::DISABLED)
    }
    #[doc = "PWM counter is started in the corresponding submodule."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RUN_A::ENABLED)
    }
}
#[doc = "Field `IPOL` reader - Current Polarity"]
pub type IPOL_R = crate::FieldReader<u8, IPOL_A>;
#[doc = "Current Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPOL_A {
    #[doc = "0: PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM23 = 0,
    #[doc = "1: PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM45 = 1,
}
impl From<IPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: IPOL_A) -> Self {
        variant as _
    }
}
impl IPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPOL_A> {
        match self.bits {
            0 => Some(IPOL_A::PWM23),
            1 => Some(IPOL_A::PWM45),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM23`"]
    #[inline(always)]
    pub fn is_pwm23(&self) -> bool {
        *self == IPOL_A::PWM23
    }
    #[doc = "Checks if the value of the field is `PWM45`"]
    #[inline(always)]
    pub fn is_pwm45(&self) -> bool {
        *self == IPOL_A::PWM45
    }
}
#[doc = "Field `IPOL` writer - Current Polarity"]
pub type IPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MCTRL_SPEC, u8, IPOL_A, 4, O>;
impl<'a, const O: u8> IPOL_W<'a, O> {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline(always)]
    pub fn pwm23(self) -> &'a mut W {
        self.variant(IPOL_A::PWM23)
    }
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline(always)]
    pub fn pwm45(self) -> &'a mut W {
        self.variant(IPOL_A::PWM45)
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Clear Load Okay"]
    #[inline(always)]
    pub fn cldok(&self) -> CLDOK_R {
        CLDOK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline(always)]
    pub fn ipol(&self) -> IPOL_R {
        IPOL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline(always)]
    #[must_use]
    pub fn ldok(&mut self) -> LDOK_W<0> {
        LDOK_W::new(self)
    }
    #[doc = "Bits 4:7 - Clear Load Okay"]
    #[inline(always)]
    #[must_use]
    pub fn cldok(&mut self) -> CLDOK_W<4> {
        CLDOK_W::new(self)
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RUN_W<8> {
        RUN_W::new(self)
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ipol(&mut self) -> IPOL_W<12> {
        IPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](index.html) module"]
pub struct MCTRL_SPEC;
impl crate::RegisterSpec for MCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mctrl::R](R) reader structure"]
impl crate::Readable for MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl::W](W) writer structure"]
impl crate::Writable for MCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0"]
impl crate::Resettable for MCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
