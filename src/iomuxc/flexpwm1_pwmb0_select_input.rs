#[doc = "Register `FLEXPWM1_PWMB0_SELECT_INPUT` reader"]
pub struct R(crate::R<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXPWM1_PWMB0_SELECT_INPUT` writer"]
pub struct W(crate::W<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>;
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
impl From<crate::W<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXPWM1_PWMB0_SELECT_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAISY` reader - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_R = crate::BitReader<DAISY_A>;
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_AD_B1_07 for Mode: ALT1"]
    GPIO_AD_B1_07_ALT1 = 0,
    #[doc = "1: Selecting Pad: GPIO_EMC_27 for Mode: ALT1"]
    GPIO_EMC_27_ALT1 = 1,
}
impl From<DAISY_A> for bool {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        variant as u8 != 0
    }
}
impl DAISY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAISY_A {
        match self.bits {
            false => DAISY_A::GPIO_AD_B1_07_ALT1,
            true => DAISY_A::GPIO_EMC_27_ALT1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B1_07_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_ad_b1_07_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B1_07_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_27_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_emc_27_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_27_ALT1
    }
}
#[doc = "Field `DAISY` writer - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLEXPWM1_PWMB0_SELECT_INPUT_SPEC, DAISY_A, O>;
impl<'a, const O: u8> DAISY_W<'a, O> {
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_ad_b1_07_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B1_07_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_emc_27_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_27_ALT1)
    }
}
impl R {
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    #[must_use]
    pub fn daisy(&mut self) -> DAISY_W<0> {
        DAISY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexpwm1_pwmb0_select_input](index.html) module"]
pub struct FLEXPWM1_PWMB0_SELECT_INPUT_SPEC;
impl crate::RegisterSpec for FLEXPWM1_PWMB0_SELECT_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexpwm1_pwmb0_select_input::R](R) reader structure"]
impl crate::Readable for FLEXPWM1_PWMB0_SELECT_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexpwm1_pwmb0_select_input::W](W) writer structure"]
impl crate::Writable for FLEXPWM1_PWMB0_SELECT_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLEXPWM1_PWMB0_SELECT_INPUT to value 0"]
impl crate::Resettable for FLEXPWM1_PWMB0_SELECT_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
