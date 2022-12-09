#[doc = "Register `SMFRCTRL` reader"]
pub struct R(crate::R<SMFRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMFRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMFRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMFRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMFRCTRL` writer"]
pub struct W(crate::W<SMFRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMFRCTRL_SPEC>;
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
impl From<crate::W<SMFRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMFRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC1_EN` reader - Fractional Cycle PWM Period Enable"]
pub type FRAC1_EN_R = crate::BitReader<FRAC1_EN_A>;
#[doc = "Fractional Cycle PWM Period Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAC1_EN_A {
    #[doc = "0: Disable fractional cycle length for the PWM period."]
    DISABLED = 0,
    #[doc = "1: Enable fractional cycle length for the PWM period."]
    ENABLED = 1,
}
impl From<FRAC1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAC1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC1_EN_A {
        match self.bits {
            false => FRAC1_EN_A::DISABLED,
            true => FRAC1_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAC1_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAC1_EN_A::ENABLED
    }
}
#[doc = "Field `FRAC1_EN` writer - Fractional Cycle PWM Period Enable"]
pub type FRAC1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMFRCTRL_SPEC, FRAC1_EN_A, O>;
impl<'a, const O: u8> FRAC1_EN_W<'a, O> {
    #[doc = "Disable fractional cycle length for the PWM period."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRAC1_EN_A::DISABLED)
    }
    #[doc = "Enable fractional cycle length for the PWM period."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRAC1_EN_A::ENABLED)
    }
}
#[doc = "Field `FRAC23_EN` reader - Fractional Cycle Placement Enable for PWM_A"]
pub type FRAC23_EN_R = crate::BitReader<FRAC23_EN_A>;
#[doc = "Fractional Cycle Placement Enable for PWM_A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAC23_EN_A {
    #[doc = "0: Disable fractional cycle placement for PWM_A."]
    DISABLED = 0,
    #[doc = "1: Enable fractional cycle placement for PWM_A."]
    ENABLED = 1,
}
impl From<FRAC23_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC23_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAC23_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC23_EN_A {
        match self.bits {
            false => FRAC23_EN_A::DISABLED,
            true => FRAC23_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAC23_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAC23_EN_A::ENABLED
    }
}
#[doc = "Field `FRAC23_EN` writer - Fractional Cycle Placement Enable for PWM_A"]
pub type FRAC23_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMFRCTRL_SPEC, FRAC23_EN_A, O>;
impl<'a, const O: u8> FRAC23_EN_W<'a, O> {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRAC23_EN_A::DISABLED)
    }
    #[doc = "Enable fractional cycle placement for PWM_A."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRAC23_EN_A::ENABLED)
    }
}
#[doc = "Field `FRAC45_EN` reader - Fractional Cycle Placement Enable for PWM_B"]
pub type FRAC45_EN_R = crate::BitReader<FRAC45_EN_A>;
#[doc = "Fractional Cycle Placement Enable for PWM_B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAC45_EN_A {
    #[doc = "0: Disable fractional cycle placement for PWM_B."]
    DISABLED = 0,
    #[doc = "1: Enable fractional cycle placement for PWM_B."]
    ENABLED = 1,
}
impl From<FRAC45_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC45_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAC45_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC45_EN_A {
        match self.bits {
            false => FRAC45_EN_A::DISABLED,
            true => FRAC45_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAC45_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAC45_EN_A::ENABLED
    }
}
#[doc = "Field `FRAC45_EN` writer - Fractional Cycle Placement Enable for PWM_B"]
pub type FRAC45_EN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMFRCTRL_SPEC, FRAC45_EN_A, O>;
impl<'a, const O: u8> FRAC45_EN_W<'a, O> {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRAC45_EN_A::DISABLED)
    }
    #[doc = "Enable fractional cycle placement for PWM_B."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRAC45_EN_A::ENABLED)
    }
}
#[doc = "Field `FRAC_PU` reader - Fractional Delay Circuit Power Up"]
pub type FRAC_PU_R = crate::BitReader<FRAC_PU_A>;
#[doc = "Fractional Delay Circuit Power Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAC_PU_A {
    #[doc = "0: Turn off fractional delay logic."]
    TURN_OFF = 0,
    #[doc = "1: Power up fractional delay logic."]
    POWER_UP = 1,
}
impl From<FRAC_PU_A> for bool {
    #[inline(always)]
    fn from(variant: FRAC_PU_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAC_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAC_PU_A {
        match self.bits {
            false => FRAC_PU_A::TURN_OFF,
            true => FRAC_PU_A::POWER_UP,
        }
    }
    #[doc = "Checks if the value of the field is `TURN_OFF`"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == FRAC_PU_A::TURN_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == FRAC_PU_A::POWER_UP
    }
}
#[doc = "Field `FRAC_PU` writer - Fractional Delay Circuit Power Up"]
pub type FRAC_PU_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMFRCTRL_SPEC, FRAC_PU_A, O>;
impl<'a, const O: u8> FRAC_PU_W<'a, O> {
    #[doc = "Turn off fractional delay logic."]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut W {
        self.variant(FRAC_PU_A::TURN_OFF)
    }
    #[doc = "Power up fractional delay logic."]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(FRAC_PU_A::POWER_UP)
    }
}
#[doc = "Field `TEST` reader - Test Status Bit"]
pub type TEST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    pub fn frac1_en(&self) -> FRAC1_EN_R {
        FRAC1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    pub fn frac23_en(&self) -> FRAC23_EN_R {
        FRAC23_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    pub fn frac45_en(&self) -> FRAC45_EN_R {
        FRAC45_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline(always)]
    pub fn frac_pu(&self) -> FRAC_PU_R {
        FRAC_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Test Status Bit"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Fractional Cycle PWM Period Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frac1_en(&mut self) -> FRAC1_EN_W<1> {
        FRAC1_EN_W::new(self)
    }
    #[doc = "Bit 2 - Fractional Cycle Placement Enable for PWM_A"]
    #[inline(always)]
    #[must_use]
    pub fn frac23_en(&mut self) -> FRAC23_EN_W<2> {
        FRAC23_EN_W::new(self)
    }
    #[doc = "Bit 4 - Fractional Cycle Placement Enable for PWM_B"]
    #[inline(always)]
    #[must_use]
    pub fn frac45_en(&mut self) -> FRAC45_EN_W<4> {
        FRAC45_EN_W::new(self)
    }
    #[doc = "Bit 8 - Fractional Delay Circuit Power Up"]
    #[inline(always)]
    #[must_use]
    pub fn frac_pu(&mut self) -> FRAC_PU_W<8> {
        FRAC_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfrctrl](index.html) module"]
pub struct SMFRCTRL_SPEC;
impl crate::RegisterSpec for SMFRCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smfrctrl::R](R) reader structure"]
impl crate::Readable for SMFRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smfrctrl::W](W) writer structure"]
impl crate::Writable for SMFRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMFRCTRL to value 0"]
impl crate::Resettable for SMFRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
