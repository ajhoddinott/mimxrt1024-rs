#[doc = "Register `USB_OTG1_PHY_CTRL_0` reader"]
pub struct R(crate::R<USB_OTG1_PHY_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_OTG1_PHY_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_OTG1_PHY_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_OTG1_PHY_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_OTG1_PHY_CTRL_0` writer"]
pub struct W(crate::W<USB_OTG1_PHY_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_OTG1_PHY_CTRL_0_SPEC>;
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
impl From<crate::W<USB_OTG1_PHY_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_OTG1_PHY_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_CLK_VLD` reader - Indicating whether OTG1 UTMI PHY clock is valid"]
pub type UTMI_CLK_VLD_R = crate::BitReader<UTMI_CLK_VLD_A>;
#[doc = "Indicating whether OTG1 UTMI PHY clock is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UTMI_CLK_VLD_A {
    #[doc = "0: Invalid"]
    UTMI_CLK_VLD_0 = 0,
    #[doc = "1: Valid"]
    UTMI_CLK_VLD_1 = 1,
}
impl From<UTMI_CLK_VLD_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_CLK_VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl UTMI_CLK_VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_CLK_VLD_A {
        match self.bits {
            false => UTMI_CLK_VLD_A::UTMI_CLK_VLD_0,
            true => UTMI_CLK_VLD_A::UTMI_CLK_VLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `UTMI_CLK_VLD_0`"]
    #[inline(always)]
    pub fn is_utmi_clk_vld_0(&self) -> bool {
        *self == UTMI_CLK_VLD_A::UTMI_CLK_VLD_0
    }
    #[doc = "Checks if the value of the field is `UTMI_CLK_VLD_1`"]
    #[inline(always)]
    pub fn is_utmi_clk_vld_1(&self) -> bool {
        *self == UTMI_CLK_VLD_A::UTMI_CLK_VLD_1
    }
}
#[doc = "Field `UTMI_CLK_VLD` writer - Indicating whether OTG1 UTMI PHY clock is valid"]
pub type UTMI_CLK_VLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_OTG1_PHY_CTRL_0_SPEC, UTMI_CLK_VLD_A, O>;
impl<'a, const O: u8> UTMI_CLK_VLD_W<'a, O> {
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn utmi_clk_vld_0(self) -> &'a mut W {
        self.variant(UTMI_CLK_VLD_A::UTMI_CLK_VLD_0)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn utmi_clk_vld_1(self) -> &'a mut W {
        self.variant(UTMI_CLK_VLD_A::UTMI_CLK_VLD_1)
    }
}
impl R {
    #[doc = "Bit 31 - Indicating whether OTG1 UTMI PHY clock is valid"]
    #[inline(always)]
    pub fn utmi_clk_vld(&self) -> UTMI_CLK_VLD_R {
        UTMI_CLK_VLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Indicating whether OTG1 UTMI PHY clock is valid"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_clk_vld(&mut self) -> UTMI_CLK_VLD_W<31> {
        UTMI_CLK_VLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG1 UTMI PHY Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg1_phy_ctrl_0](index.html) module"]
pub struct USB_OTG1_PHY_CTRL_0_SPEC;
impl crate::RegisterSpec for USB_OTG1_PHY_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_otg1_phy_ctrl_0::R](R) reader structure"]
impl crate::Readable for USB_OTG1_PHY_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_otg1_phy_ctrl_0::W](W) writer structure"]
impl crate::Writable for USB_OTG1_PHY_CTRL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_OTG1_PHY_CTRL_0 to value 0"]
impl crate::Resettable for USB_OTG1_PHY_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
