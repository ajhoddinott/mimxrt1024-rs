#[doc = "Register `MISC1_CLR` reader"]
pub struct R(crate::R<MISC1_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC1_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC1_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC1_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC1_CLR` writer"]
pub struct W(crate::W<MISC1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC1_CLR_SPEC>;
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
impl From<crate::W<MISC1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFD_480_AUTOGATE_EN` reader - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
pub type PFD_480_AUTOGATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `PFD_480_AUTOGATE_EN` writer - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
pub type PFD_480_AUTOGATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `PFD_528_AUTOGATE_EN` reader - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
pub type PFD_528_AUTOGATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `PFD_528_AUTOGATE_EN` writer - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
pub type PFD_528_AUTOGATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `IRQ_TEMPPANIC` reader - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
pub type IRQ_TEMPPANIC_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_TEMPPANIC` writer - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
pub type IRQ_TEMPPANIC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `IRQ_TEMPLOW` reader - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
pub type IRQ_TEMPLOW_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_TEMPLOW` writer - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
pub type IRQ_TEMPLOW_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `IRQ_TEMPHIGH` reader - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
pub type IRQ_TEMPHIGH_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_TEMPHIGH` writer - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
pub type IRQ_TEMPHIGH_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `IRQ_ANA_BO` reader - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
pub type IRQ_ANA_BO_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_ANA_BO` writer - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
pub type IRQ_ANA_BO_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MISC1_CLR_SPEC, bool, O>;
#[doc = "Field `IRQ_DIG_BO` reader - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
pub type IRQ_DIG_BO_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_DIG_BO` writer - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
pub type IRQ_DIG_BO_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, MISC1_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_480_autogate_en(&self) -> PFD_480_AUTOGATE_EN_R {
        PFD_480_AUTOGATE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_528_autogate_en(&self) -> PFD_528_AUTOGATE_EN_R {
        PFD_528_AUTOGATE_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub fn irq_temppanic(&self) -> IRQ_TEMPPANIC_R {
        IRQ_TEMPPANIC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub fn irq_templow(&self) -> IRQ_TEMPLOW_R {
        IRQ_TEMPLOW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub fn irq_temphigh(&self) -> IRQ_TEMPHIGH_R {
        IRQ_TEMPHIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_ana_bo(&self) -> IRQ_ANA_BO_R {
        IRQ_ANA_BO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_dig_bo(&self) -> IRQ_DIG_BO_R {
        IRQ_DIG_BO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    #[must_use]
    pub fn pfd_480_autogate_en(&mut self) -> PFD_480_AUTOGATE_EN_W<16> {
        PFD_480_AUTOGATE_EN_W::new(self)
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    #[must_use]
    pub fn pfd_528_autogate_en(&mut self) -> PFD_528_AUTOGATE_EN_W<17> {
        PFD_528_AUTOGATE_EN_W::new(self)
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    #[must_use]
    pub fn irq_temppanic(&mut self) -> IRQ_TEMPPANIC_W<27> {
        IRQ_TEMPPANIC_W::new(self)
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    #[must_use]
    pub fn irq_templow(&mut self) -> IRQ_TEMPLOW_W<28> {
        IRQ_TEMPLOW_W::new(self)
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    #[must_use]
    pub fn irq_temphigh(&mut self) -> IRQ_TEMPHIGH_W<29> {
        IRQ_TEMPHIGH_W::new(self)
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    #[must_use]
    pub fn irq_ana_bo(&mut self) -> IRQ_ANA_BO_W<30> {
        IRQ_ANA_BO_W::new(self)
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    #[must_use]
    pub fn irq_dig_bo(&mut self) -> IRQ_DIG_BO_W<31> {
        IRQ_DIG_BO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc1_clr](index.html) module"]
pub struct MISC1_CLR_SPEC;
impl crate::RegisterSpec for MISC1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc1_clr::R](R) reader structure"]
impl crate::Readable for MISC1_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc1_clr::W](W) writer structure"]
impl crate::Writable for MISC1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf800_0000;
}
#[doc = "`reset()` method sets MISC1_CLR to value 0"]
impl crate::Resettable for MISC1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
