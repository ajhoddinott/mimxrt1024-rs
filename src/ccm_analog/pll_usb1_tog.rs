#[doc = "Register `PLL_USB1_TOG` reader"]
pub struct R(crate::R<PLL_USB1_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_USB1_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_USB1_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_USB1_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_USB1_TOG` writer"]
pub struct W(crate::W<PLL_USB1_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_USB1_TOG_SPEC>;
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
impl From<crate::W<PLL_USB1_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_USB1_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_SELECT` reader - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
pub type DIV_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `DIV_SELECT` writer - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
pub type DIV_SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_USB1_TOG_SPEC, bool, O>;
#[doc = "Field `EN_USB_CLKS` reader - Powers the 9-phase PLL outputs for USBPHYn"]
pub type EN_USB_CLKS_R = crate::BitReader<EN_USB_CLKS_A>;
#[doc = "Powers the 9-phase PLL outputs for USBPHYn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_USB_CLKS_A {
    #[doc = "0: PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0 = 0,
    #[doc = "1: PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1 = 1,
}
impl From<EN_USB_CLKS_A> for bool {
    #[inline(always)]
    fn from(variant: EN_USB_CLKS_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_USB_CLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_USB_CLKS_A {
        match self.bits {
            false => EN_USB_CLKS_A::EN_USB_CLKS_0,
            true => EN_USB_CLKS_A::EN_USB_CLKS_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_0`"]
    #[inline(always)]
    pub fn is_en_usb_clks_0(&self) -> bool {
        *self == EN_USB_CLKS_A::EN_USB_CLKS_0
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_1`"]
    #[inline(always)]
    pub fn is_en_usb_clks_1(&self) -> bool {
        *self == EN_USB_CLKS_A::EN_USB_CLKS_1
    }
}
#[doc = "Field `EN_USB_CLKS` writer - Powers the 9-phase PLL outputs for USBPHYn"]
pub type EN_USB_CLKS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_USB1_TOG_SPEC, EN_USB_CLKS_A, O>;
impl<'a, const O: u8> EN_USB_CLKS_W<'a, O> {
    #[doc = "PLL outputs for USBPHYn off."]
    #[inline(always)]
    pub fn en_usb_clks_0(self) -> &'a mut W {
        self.variant(EN_USB_CLKS_A::EN_USB_CLKS_0)
    }
    #[doc = "PLL outputs for USBPHYn on."]
    #[inline(always)]
    pub fn en_usb_clks_1(self) -> &'a mut W {
        self.variant(EN_USB_CLKS_A::EN_USB_CLKS_1)
    }
}
#[doc = "Field `POWER` reader - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
pub type POWER_R = crate::BitReader<bool>;
#[doc = "Field `POWER` writer - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
pub type POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_USB1_TOG_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable the PLL clock output."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable the PLL clock output."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_USB1_TOG_SPEC, bool, O>;
#[doc = "Field `BYPASS_CLK_SRC` reader - Determines the bypass source."]
pub type BYPASS_CLK_SRC_R = crate::FieldReader<u8, BYPASS_CLK_SRC_A>;
#[doc = "Determines the bypass source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYPASS_CLK_SRC_A {
    #[doc = "0: Select the 24MHz oscillator as source."]
    REF_CLK_24M = 0,
}
impl From<BYPASS_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_CLK_SRC_A) -> Self {
        variant as _
    }
}
impl BYPASS_CLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BYPASS_CLK_SRC_A> {
        match self.bits {
            0 => Some(BYPASS_CLK_SRC_A::REF_CLK_24M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REF_CLK_24M`"]
    #[inline(always)]
    pub fn is_ref_clk_24m(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::REF_CLK_24M
    }
}
#[doc = "Field `BYPASS_CLK_SRC` writer - Determines the bypass source."]
pub type BYPASS_CLK_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_USB1_TOG_SPEC, u8, BYPASS_CLK_SRC_A, 2, O>;
impl<'a, const O: u8> BYPASS_CLK_SRC_W<'a, O> {
    #[doc = "Select the 24MHz oscillator as source."]
    #[inline(always)]
    pub fn ref_clk_24m(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::REF_CLK_24M)
    }
}
#[doc = "Field `BYPASS` reader - Bypass the PLL."]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Bypass the PLL."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_USB1_TOG_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
pub type LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub fn div_select(&self) -> DIV_SELECT_R {
        DIV_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub fn en_usb_clks(&self) -> EN_USB_CLKS_R {
        EN_USB_CLKS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&self) -> BYPASS_CLK_SRC_R {
        BYPASS_CLK_SRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    #[must_use]
    pub fn div_select(&mut self) -> DIV_SELECT_W<1> {
        DIV_SELECT_W::new(self)
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    #[must_use]
    pub fn en_usb_clks(&mut self) -> EN_USB_CLKS_W<6> {
        EN_USB_CLKS_W::new(self)
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    #[must_use]
    pub fn power(&mut self) -> POWER_W<12> {
        POWER_W::new(self)
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<13> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_clk_src(&mut self) -> BYPASS_CLK_SRC_W<14> {
        BYPASS_CLK_SRC_W::new(self)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<16> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog USB1 480MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_usb1_tog](index.html) module"]
pub struct PLL_USB1_TOG_SPEC;
impl crate::RegisterSpec for PLL_USB1_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_usb1_tog::R](R) reader structure"]
impl crate::Readable for PLL_USB1_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_usb1_tog::W](W) writer structure"]
impl crate::Writable for PLL_USB1_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_USB1_TOG to value 0x0001_2000"]
impl crate::Resettable for PLL_USB1_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_2000;
}
