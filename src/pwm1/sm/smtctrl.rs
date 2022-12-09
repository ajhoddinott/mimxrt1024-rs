#[doc = "Register `SMTCTRL` reader"]
pub struct R(crate::R<SMTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMTCTRL` writer"]
pub struct W(crate::W<SMTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMTCTRL_SPEC>;
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
impl From<crate::W<SMTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_TRIG_EN` reader - Output Trigger Enables"]
pub type OUT_TRIG_EN_R = crate::FieldReader<u8, OUT_TRIG_EN_A>;
#[doc = "Output Trigger Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUT_TRIG_EN_A {
    #[doc = "1: PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 1,
}
impl From<OUT_TRIG_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT_TRIG_EN_A) -> Self {
        variant as _
    }
}
impl OUT_TRIG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT_TRIG_EN_A> {
        match self.bits {
            1 => Some(OUT_TRIG_EN_A::VAL0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == OUT_TRIG_EN_A::VAL0
    }
}
#[doc = "Field `OUT_TRIG_EN` writer - Output Trigger Enables"]
pub type OUT_TRIG_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SMTCTRL_SPEC, u8, OUT_TRIG_EN_A, 6, O>;
impl<'a, const O: u8> OUT_TRIG_EN_W<'a, O> {
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(OUT_TRIG_EN_A::VAL0)
    }
}
#[doc = "Field `TRGFRQ` reader - Trigger frequency"]
pub type TRGFRQ_R = crate::BitReader<TRGFRQ_A>;
#[doc = "Trigger frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGFRQ_A {
    #[doc = "0: Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\]
being non-zero."]
    EVERYPWM = 0,
    #[doc = "1: Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\]
being non-zero."]
    FINALPWM = 1,
}
impl From<TRGFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TRGFRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGFRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGFRQ_A {
        match self.bits {
            false => TRGFRQ_A::EVERYPWM,
            true => TRGFRQ_A::FINALPWM,
        }
    }
    #[doc = "Checks if the value of the field is `EVERYPWM`"]
    #[inline(always)]
    pub fn is_everypwm(&self) -> bool {
        *self == TRGFRQ_A::EVERYPWM
    }
    #[doc = "Checks if the value of the field is `FINALPWM`"]
    #[inline(always)]
    pub fn is_finalpwm(&self) -> bool {
        *self == TRGFRQ_A::FINALPWM
    }
}
#[doc = "Field `TRGFRQ` writer - Trigger frequency"]
pub type TRGFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMTCTRL_SPEC, TRGFRQ_A, O>;
impl<'a, const O: u8> TRGFRQ_W<'a, O> {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\]
being non-zero."]
    #[inline(always)]
    pub fn everypwm(self) -> &'a mut W {
        self.variant(TRGFRQ_A::EVERYPWM)
    }
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\]
being non-zero."]
    #[inline(always)]
    pub fn finalpwm(self) -> &'a mut W {
        self.variant(TRGFRQ_A::FINALPWM)
    }
}
#[doc = "Field `PWBOT1` reader - Output Trigger 1 Source Select"]
pub type PWBOT1_R = crate::BitReader<PWBOT1_A>;
#[doc = "Output Trigger 1 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWBOT1_A {
    #[doc = "0: Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0,
    #[doc = "1: Route the PWMB output to the PWM_OUT_TRIG1 port."]
    PWMB_OUTPUT = 1,
}
impl From<PWBOT1_A> for bool {
    #[inline(always)]
    fn from(variant: PWBOT1_A) -> Self {
        variant as u8 != 0
    }
}
impl PWBOT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWBOT1_A {
        match self.bits {
            false => PWBOT1_A::PWM_OUT_TRIG1_SIGNAL,
            true => PWBOT1_A::PWMB_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_OUT_TRIG1_SIGNAL`"]
    #[inline(always)]
    pub fn is_pwm_out_trig1_signal(&self) -> bool {
        *self == PWBOT1_A::PWM_OUT_TRIG1_SIGNAL
    }
    #[doc = "Checks if the value of the field is `PWMB_OUTPUT`"]
    #[inline(always)]
    pub fn is_pwmb_output(&self) -> bool {
        *self == PWBOT1_A::PWMB_OUTPUT
    }
}
#[doc = "Field `PWBOT1` writer - Output Trigger 1 Source Select"]
pub type PWBOT1_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMTCTRL_SPEC, PWBOT1_A, O>;
impl<'a, const O: u8> PWBOT1_W<'a, O> {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_OUT_TRIG1 port."]
    #[inline(always)]
    pub fn pwm_out_trig1_signal(self) -> &'a mut W {
        self.variant(PWBOT1_A::PWM_OUT_TRIG1_SIGNAL)
    }
    #[doc = "Route the PWMB output to the PWM_OUT_TRIG1 port."]
    #[inline(always)]
    pub fn pwmb_output(self) -> &'a mut W {
        self.variant(PWBOT1_A::PWMB_OUTPUT)
    }
}
#[doc = "Field `PWAOT0` reader - Output Trigger 0 Source Select"]
pub type PWAOT0_R = crate::BitReader<PWAOT0_A>;
#[doc = "Output Trigger 0 Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWAOT0_A {
    #[doc = "0: Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0,
    #[doc = "1: Route the PWMA output to the PWM_OUT_TRIG0 port."]
    PWMA_OUTPUT = 1,
}
impl From<PWAOT0_A> for bool {
    #[inline(always)]
    fn from(variant: PWAOT0_A) -> Self {
        variant as u8 != 0
    }
}
impl PWAOT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWAOT0_A {
        match self.bits {
            false => PWAOT0_A::PWM_OUT_TRIG0_SIGNAL,
            true => PWAOT0_A::PWMA_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_OUT_TRIG0_SIGNAL`"]
    #[inline(always)]
    pub fn is_pwm_out_trig0_signal(&self) -> bool {
        *self == PWAOT0_A::PWM_OUT_TRIG0_SIGNAL
    }
    #[doc = "Checks if the value of the field is `PWMA_OUTPUT`"]
    #[inline(always)]
    pub fn is_pwma_output(&self) -> bool {
        *self == PWAOT0_A::PWMA_OUTPUT
    }
}
#[doc = "Field `PWAOT0` writer - Output Trigger 0 Source Select"]
pub type PWAOT0_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMTCTRL_SPEC, PWAOT0_A, O>;
impl<'a, const O: u8> PWAOT0_W<'a, O> {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_OUT_TRIG0 port."]
    #[inline(always)]
    pub fn pwm_out_trig0_signal(self) -> &'a mut W {
        self.variant(PWAOT0_A::PWM_OUT_TRIG0_SIGNAL)
    }
    #[doc = "Route the PWMA output to the PWM_OUT_TRIG0 port."]
    #[inline(always)]
    pub fn pwma_output(self) -> &'a mut W {
        self.variant(PWAOT0_A::PWMA_OUTPUT)
    }
}
impl R {
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline(always)]
    pub fn out_trig_en(&self) -> OUT_TRIG_EN_R {
        OUT_TRIG_EN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline(always)]
    pub fn trgfrq(&self) -> TRGFRQ_R {
        TRGFRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline(always)]
    pub fn pwbot1(&self) -> PWBOT1_R {
        PWBOT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline(always)]
    pub fn pwaot0(&self) -> PWAOT0_R {
        PWAOT0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Output Trigger Enables"]
    #[inline(always)]
    #[must_use]
    pub fn out_trig_en(&mut self) -> OUT_TRIG_EN_W<0> {
        OUT_TRIG_EN_W::new(self)
    }
    #[doc = "Bit 12 - Trigger frequency"]
    #[inline(always)]
    #[must_use]
    pub fn trgfrq(&mut self) -> TRGFRQ_W<12> {
        TRGFRQ_W::new(self)
    }
    #[doc = "Bit 14 - Output Trigger 1 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwbot1(&mut self) -> PWBOT1_W<14> {
        PWBOT1_W::new(self)
    }
    #[doc = "Bit 15 - Output Trigger 0 Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwaot0(&mut self) -> PWAOT0_W<15> {
        PWAOT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smtctrl](index.html) module"]
pub struct SMTCTRL_SPEC;
impl crate::RegisterSpec for SMTCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smtctrl::R](R) reader structure"]
impl crate::Readable for SMTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smtctrl::W](W) writer structure"]
impl crate::Writable for SMTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMTCTRL to value 0"]
impl crate::Resettable for SMTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
