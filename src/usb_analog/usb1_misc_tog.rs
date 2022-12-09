#[doc = "Register `USB1_MISC_TOG` reader"]
pub struct R(crate::R<USB1_MISC_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_MISC_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_MISC_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_MISC_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_MISC_TOG` writer"]
pub struct W(crate::W<USB1_MISC_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_MISC_TOG_SPEC>;
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
impl From<crate::W<USB1_MISC_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_MISC_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HS_USE_EXTERNAL_R` reader - Use external resistor to generate the current bias for the high speed transmitter"]
pub type HS_USE_EXTERNAL_R_R = crate::BitReader<bool>;
#[doc = "Field `HS_USE_EXTERNAL_R` writer - Use external resistor to generate the current bias for the high speed transmitter"]
pub type HS_USE_EXTERNAL_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_MISC_TOG_SPEC, bool, O>;
#[doc = "Field `EN_DEGLITCH` reader - Enable the deglitching circuit of the USB PLL output."]
pub type EN_DEGLITCH_R = crate::BitReader<bool>;
#[doc = "Field `EN_DEGLITCH` writer - Enable the deglitching circuit of the USB PLL output."]
pub type EN_DEGLITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_MISC_TOG_SPEC, bool, O>;
#[doc = "Field `EN_CLK_UTMI` reader - Enables the clk to the UTMI block."]
pub type EN_CLK_UTMI_R = crate::BitReader<bool>;
#[doc = "Field `EN_CLK_UTMI` writer - Enables the clk to the UTMI block."]
pub type EN_CLK_UTMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_MISC_TOG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    pub fn hs_use_external_r(&self) -> HS_USE_EXTERNAL_R_R {
        HS_USE_EXTERNAL_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    pub fn en_deglitch(&self) -> EN_DEGLITCH_R {
        EN_DEGLITCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Enables the clk to the UTMI block."]
    #[inline(always)]
    pub fn en_clk_utmi(&self) -> EN_CLK_UTMI_R {
        EN_CLK_UTMI_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use external resistor to generate the current bias for the high speed transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn hs_use_external_r(&mut self) -> HS_USE_EXTERNAL_R_W<0> {
        HS_USE_EXTERNAL_R_W::new(self)
    }
    #[doc = "Bit 1 - Enable the deglitching circuit of the USB PLL output."]
    #[inline(always)]
    #[must_use]
    pub fn en_deglitch(&mut self) -> EN_DEGLITCH_W<1> {
        EN_DEGLITCH_W::new(self)
    }
    #[doc = "Bit 30 - Enables the clk to the UTMI block."]
    #[inline(always)]
    #[must_use]
    pub fn en_clk_utmi(&mut self) -> EN_CLK_UTMI_W<30> {
        EN_CLK_UTMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Misc Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_misc_tog](index.html) module"]
pub struct USB1_MISC_TOG_SPEC;
impl crate::RegisterSpec for USB1_MISC_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_misc_tog::R](R) reader structure"]
impl crate::Readable for USB1_MISC_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_misc_tog::W](W) writer structure"]
impl crate::Writable for USB1_MISC_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_MISC_TOG to value 0x02"]
impl crate::Resettable for USB1_MISC_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
