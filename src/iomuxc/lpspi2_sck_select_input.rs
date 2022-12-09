#[doc = "Register `LPSPI2_SCK_SELECT_INPUT` reader"]
pub struct R(crate::R<LPSPI2_SCK_SELECT_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSPI2_SCK_SELECT_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSPI2_SCK_SELECT_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSPI2_SCK_SELECT_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSPI2_SCK_SELECT_INPUT` writer"]
pub struct W(crate::W<LPSPI2_SCK_SELECT_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSPI2_SCK_SELECT_INPUT_SPEC>;
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
impl From<crate::W<LPSPI2_SCK_SELECT_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSPI2_SCK_SELECT_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAISY` reader - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_R = crate::FieldReader<u8, DAISY_A>;
#[doc = "Selecting Pads Involved in Daisy Chain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAISY_A {
    #[doc = "0: Selecting Pad: GPIO_EMC_00 for Mode: ALT4"]
    _GPIO_EMC_00_ALT4 = 0,
    #[doc = "1: Selecting Pad: GPIO_EMC_10 for Mode: ALT4"]
    _GPIO_EMC_10_ALT4 = 1,
    #[doc = "2: Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4"]
    _GPIO_SD_B1_07_ALT4 = 2,
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
            0 => Some(DAISY_A::_GPIO_EMC_00_ALT4),
            1 => Some(DAISY_A::_GPIO_EMC_10_ALT4),
            2 => Some(DAISY_A::_GPIO_SD_B1_07_ALT4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_GPIO_EMC_00_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_emc_00_alt4(&self) -> bool {
        *self == DAISY_A::_GPIO_EMC_00_ALT4
    }
    #[doc = "Checks if the value of the field is `_GPIO_EMC_10_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_emc_10_alt4(&self) -> bool {
        *self == DAISY_A::_GPIO_EMC_10_ALT4
    }
    #[doc = "Checks if the value of the field is `_GPIO_SD_B1_07_ALT4`"]
    #[inline(always)]
    pub fn is_gpio_sd_b1_07_alt4(&self) -> bool {
        *self == DAISY_A::_GPIO_SD_B1_07_ALT4
    }
}
#[doc = "Field `DAISY` writer - Selecting Pads Involved in Daisy Chain."]
pub type DAISY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPSPI2_SCK_SELECT_INPUT_SPEC, u8, DAISY_A, 2, O>;
impl<'a, const O: u8> DAISY_W<'a, O> {
    #[doc = "Selecting Pad: GPIO_EMC_00 for Mode: ALT4"]
    #[inline(always)]
    pub fn _gpio_emc_00_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::_GPIO_EMC_00_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT4"]
    #[inline(always)]
    pub fn _gpio_emc_10_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::_GPIO_EMC_10_ALT4)
    }
    #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4"]
    #[inline(always)]
    pub fn _gpio_sd_b1_07_alt4(self) -> &'a mut W {
        self.variant(DAISY_A::_GPIO_SD_B1_07_ALT4)
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
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpspi2_sck_select_input](index.html) module"]
pub struct LPSPI2_SCK_SELECT_INPUT_SPEC;
impl crate::RegisterSpec for LPSPI2_SCK_SELECT_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpspi2_sck_select_input::R](R) reader structure"]
impl crate::Readable for LPSPI2_SCK_SELECT_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpspi2_sck_select_input::W](W) writer structure"]
impl crate::Writable for LPSPI2_SCK_SELECT_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSPI2_SCK_SELECT_INPUT to value 0"]
impl crate::Resettable for LPSPI2_SCK_SELECT_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
