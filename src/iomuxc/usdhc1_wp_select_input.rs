#[doc = "Register `USDHC1_WP_SELECT_INPUT` reader"]
pub struct R(crate::R<USDHC1_WP_SELECT_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USDHC1_WP_SELECT_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USDHC1_WP_SELECT_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USDHC1_WP_SELECT_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USDHC1_WP_SELECT_INPUT` writer"]
pub struct W(crate::W<USDHC1_WP_SELECT_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USDHC1_WP_SELECT_INPUT_SPEC>;
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
impl From<crate::W<USDHC1_WP_SELECT_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USDHC1_WP_SELECT_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAISY` reader - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_R = crate::FieldReader<u8, DAISY_A>;
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT4"]
    GPIO_AD_B0_03_ALT4 = 0,
    #[doc = "1: Selecting Pad: GPIO_AD_B0_04 for Mode: ALT2"]
    GPIO_AD_B0_04_ALT2 = 1,
    #[doc = "3: Selecting Pad: GPIO_AD_B1_11 for Mode: ALT3"]
    GPIO_AD_B1_11_ALT3 = 3,
    #[doc = "4: Selecting Pad: GPIO_EMC_36 for Mode: ALT7"]
    GPIO_EMC_36_ALT7 = 4,
}
impl From<DAISY_A> for u8 {
    #[inline(always)]
    fn from(variant: DAISY_A) -> Self {
        variant as _
    }
}
impl DAISY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAISY_A> {
        match self.bits {
            0 => Some(DAISY_A::GPIO_AD_B0_03_ALT4),
            1 => Some(DAISY_A::GPIO_AD_B0_04_ALT2),
            3 => Some(DAISY_A::GPIO_AD_B1_11_ALT3),
            4 => Some(DAISY_A::GPIO_EMC_36_ALT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_03_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_03_alt4(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_03_ALT4
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_04_ALT2`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_04_alt2(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_04_ALT2
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B1_11_ALT3`"]
    #[inline(always)]
    pub fn is_gpio_ad_b1_11_alt3(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B1_11_ALT3
    }
    #[doc = "Checks if the value of the field is `GPIO_EMC_36_ALT7`"]
    #[inline(always)]
    pub fn is_gpio_emc_36_alt7(&self) -> bool {
        *self == DAISY_A::GPIO_EMC_36_ALT7
    }
}
#[doc = "Field `DAISY` writer - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USDHC1_WP_SELECT_INPUT_SPEC, u8, DAISY_A, 3, O>;
impl<'a, const O: u8> DAISY_W<'a, O> {
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT4"]
    #[inline(always)]
    pub fn gpio_ad_b0_03_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_03_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_04 for Mode: ALT2"]
    #[inline(always)]
    pub fn gpio_ad_b0_04_alt2(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_04_ALT2)
    }
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT3"]
    #[inline(always)]
    pub fn gpio_ad_b1_11_alt3(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B1_11_ALT3)
    }
    #[doc = "Selecting Pad: GPIO_EMC_36 for Mode: ALT7"]
    #[inline(always)]
    pub fn gpio_emc_36_alt7(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_EMC_36_ALT7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selecting Pads Involved in Daisy Chain."]
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
#[doc = "USDHC1_WP_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usdhc1_wp_select_input](index.html) module"]
pub struct USDHC1_WP_SELECT_INPUT_SPEC;
impl crate::RegisterSpec for USDHC1_WP_SELECT_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usdhc1_wp_select_input::R](R) reader structure"]
impl crate::Readable for USDHC1_WP_SELECT_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usdhc1_wp_select_input::W](W) writer structure"]
impl crate::Writable for USDHC1_WP_SELECT_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USDHC1_WP_SELECT_INPUT to value 0"]
impl crate::Resettable for USDHC1_WP_SELECT_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
