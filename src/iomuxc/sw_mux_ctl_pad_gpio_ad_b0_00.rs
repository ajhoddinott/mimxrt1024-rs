#[doc = "Register `SW_MUX_CTL_PAD_GPIO_AD_B0_00` reader"]
pub struct R(crate::R<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_MUX_CTL_PAD_GPIO_AD_B0_00` writer"]
pub struct W(crate::W<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>;
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
impl From<crate::W<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_MODE` reader - MUX Mode Select Field."]
pub type MUX_MODE_R = crate::FieldReader<u8, MUX_MODE_A>;
#[doc = "MUX Mode Select Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_MODE_A {
    #[doc = "0: Select mux mode: ALT0 mux port: JTAG_TMS of instance: jtag_mux"]
    ALT0 = 0,
    #[doc = "5: Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
    ALT5 = 5,
    #[doc = "7: Select mux mode: ALT7 mux port: GPT1_COMPARE1 of instance: gpt1"]
    ALT7 = 7,
}
impl From<MUX_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_MODE_A) -> Self {
        variant as _
    }
}
impl MUX_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_MODE_A> {
        match self.bits {
            0 => Some(MUX_MODE_A::ALT0),
            5 => Some(MUX_MODE_A::ALT5),
            7 => Some(MUX_MODE_A::ALT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == MUX_MODE_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == MUX_MODE_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == MUX_MODE_A::ALT7
    }
}
#[doc = "Field `MUX_MODE` writer - MUX Mode Select Field."]
pub type MUX_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC, u8, MUX_MODE_A, 3, O>;
impl<'a, const O: u8> MUX_MODE_W<'a, O> {
    #[doc = "Select mux mode: ALT0 mux port: JTAG_TMS of instance: jtag_mux"]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT0)
    }
    #[doc = "Select mux mode: ALT5 mux port: GPIO1_IO00 of instance: gpio1"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT5)
    }
    #[doc = "Select mux mode: ALT7 mux port: GPT1_COMPARE1 of instance: gpt1"]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(MUX_MODE_A::ALT7)
    }
}
#[doc = "Field `SION` reader - Software Input On Field."]
pub type SION_R = crate::BitReader<SION_A>;
#[doc = "Software Input On Field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SION_A {
    #[doc = "0: Input Path is determined by functionality"]
    DISABLED = 0,
    #[doc = "1: Force input path of pad GPIO_AD_B0_00"]
    ENABLED = 1,
}
impl From<SION_A> for bool {
    #[inline(always)]
    fn from(variant: SION_A) -> Self {
        variant as u8 != 0
    }
}
impl SION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SION_A {
        match self.bits {
            false => SION_A::DISABLED,
            true => SION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SION_A::ENABLED
    }
}
#[doc = "Field `SION` writer - Software Input On Field."]
pub type SION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC, SION_A, O>;
impl<'a, const O: u8> SION_W<'a, O> {
    #[doc = "Input Path is determined by functionality"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SION_A::DISABLED)
    }
    #[doc = "Force input path of pad GPIO_AD_B0_00"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SION_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    pub fn mux_mode(&self) -> MUX_MODE_R {
        MUX_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    pub fn sion(&self) -> SION_R {
        SION_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MUX Mode Select Field."]
    #[inline(always)]
    #[must_use]
    pub fn mux_mode(&mut self) -> MUX_MODE_W<0> {
        MUX_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Software Input On Field."]
    #[inline(always)]
    #[must_use]
    pub fn sion(&mut self) -> SION_W<4> {
        SION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_mux_ctl_pad_gpio_ad_b0_00](index.html) module"]
pub struct SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC;
impl crate::RegisterSpec for SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_mux_ctl_pad_gpio_ad_b0_00::R](R) reader structure"]
impl crate::Readable for SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_mux_ctl_pad_gpio_ad_b0_00::W](W) writer structure"]
impl crate::Writable for SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_MUX_CTL_PAD_GPIO_AD_B0_00 to value 0"]
impl crate::Resettable for SW_MUX_CTL_PAD_GPIO_AD_B0_00_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
