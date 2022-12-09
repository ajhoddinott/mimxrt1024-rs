#[doc = "Register `USB1_LOOPBACK_CLR` reader"]
pub struct R(crate::R<USB1_LOOPBACK_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_LOOPBACK_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_LOOPBACK_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_LOOPBACK_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_LOOPBACK_CLR` writer"]
pub struct W(crate::W<USB1_LOOPBACK_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_LOOPBACK_CLR_SPEC>;
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
impl From<crate::W<USB1_LOOPBACK_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_LOOPBACK_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTMI_TESTSTART` reader - Setting this bit can enable 1"]
pub type UTMI_TESTSTART_R = crate::BitReader<bool>;
#[doc = "Field `UTMI_TESTSTART` writer - Setting this bit can enable 1"]
pub type UTMI_TESTSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_LOOPBACK_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit can enable 1"]
    #[inline(always)]
    pub fn utmi_teststart(&self) -> UTMI_TESTSTART_R {
        UTMI_TESTSTART_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit can enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_teststart(&mut self) -> UTMI_TESTSTART_W<0> {
        UTMI_TESTSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Loopback Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_loopback_clr](index.html) module"]
pub struct USB1_LOOPBACK_CLR_SPEC;
impl crate::RegisterSpec for USB1_LOOPBACK_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_loopback_clr::R](R) reader structure"]
impl crate::Readable for USB1_LOOPBACK_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_loopback_clr::W](W) writer structure"]
impl crate::Writable for USB1_LOOPBACK_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK_CLR to value 0"]
impl crate::Resettable for USB1_LOOPBACK_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
