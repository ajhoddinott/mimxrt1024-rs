#[doc = "Register `USDHC2_CD_B_SELECT_INPUT` reader"]
pub struct R(crate::R<USDHC2_CD_B_SELECT_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USDHC2_CD_B_SELECT_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USDHC2_CD_B_SELECT_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USDHC2_CD_B_SELECT_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USDHC2_CD_B_SELECT_INPUT` writer"]
pub struct W(crate::W<USDHC2_CD_B_SELECT_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USDHC2_CD_B_SELECT_INPUT_SPEC>;
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
impl From<crate::W<USDHC2_CD_B_SELECT_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USDHC2_CD_B_SELECT_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAISY` reader - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_R = crate::FieldReader<u8, DAISY_A>;
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_SD_B1_06 for Mode: ALT0"]
    GPIO_SD_B1_06_ALT0 = 0,
    #[doc = "1: Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    GPIO_AD_B0_03_ALT1 = 1,
    #[doc = "2: Selecting Pad: GPIO_AD_B1_12 for Mode: ALT3"]
    GPIO_AD_B1_12_ALT3 = 2,
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
            0 => Some(DAISY_A::GPIO_SD_B1_06_ALT0),
            1 => Some(DAISY_A::GPIO_AD_B0_03_ALT1),
            2 => Some(DAISY_A::GPIO_AD_B1_12_ALT3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_SD_B1_06_ALT0`"]
    #[inline(always)]
    pub fn is_gpio_sd_b1_06_alt0(&self) -> bool {
        *self == DAISY_A::GPIO_SD_B1_06_ALT0
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B0_03_ALT1`"]
    #[inline(always)]
    pub fn is_gpio_ad_b0_03_alt1(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B0_03_ALT1
    }
    #[doc = "Checks if the value of the field is `GPIO_AD_B1_12_ALT3`"]
    #[inline(always)]
    pub fn is_gpio_ad_b1_12_alt3(&self) -> bool {
        *self == DAISY_A::GPIO_AD_B1_12_ALT3
    }
}
#[doc = "Field `DAISY` writer - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USDHC2_CD_B_SELECT_INPUT_SPEC, u8, DAISY_A, 2, O>;
impl<'a, const O: u8> DAISY_W<'a, O> {
    #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT0"]
    #[inline(always)]
    pub fn gpio_sd_b1_06_alt0(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_SD_B1_06_ALT0)
    }
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    #[inline(always)]
    pub fn gpio_ad_b0_03_alt1(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B0_03_ALT1)
    }
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT3"]
    #[inline(always)]
    pub fn gpio_ad_b1_12_alt3(self) -> &'a mut W {
        self.variant(DAISY_A::GPIO_AD_B1_12_ALT3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub fn daisy(&self) -> DAISY_R {
        DAISY_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selecting Pads Involved in Daisy Chain."]
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
#[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usdhc2_cd_b_select_input](index.html) module"]
pub struct USDHC2_CD_B_SELECT_INPUT_SPEC;
impl crate::RegisterSpec for USDHC2_CD_B_SELECT_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usdhc2_cd_b_select_input::R](R) reader structure"]
impl crate::Readable for USDHC2_CD_B_SELECT_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usdhc2_cd_b_select_input::W](W) writer structure"]
impl crate::Writable for USDHC2_CD_B_SELECT_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USDHC2_CD_B_SELECT_INPUT to value 0"]
impl crate::Resettable for USDHC2_CD_B_SELECT_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
