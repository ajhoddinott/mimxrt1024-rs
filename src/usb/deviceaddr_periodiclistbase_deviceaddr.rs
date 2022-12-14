#[doc = "Register `DEVICEADDR` reader"]
pub struct R(crate::R<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICEADDR` writer"]
pub struct W(crate::W<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>;
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
impl From<crate::W<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBADRA` reader - Device Address Advance"]
pub type USBADRA_R = crate::BitReader<bool>;
#[doc = "Field `USBADRA` writer - Device Address Advance"]
pub type USBADRA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC, bool, O>;
#[doc = "Field `USBADR` reader - Device Address. These bits correspond to the USB device address"]
pub type USBADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADR` writer - Device Address. These bits correspond to the USB device address"]
pub type USBADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    pub fn usbadra(&self) -> USBADRA_R {
        USBADRA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    pub fn usbadr(&self) -> USBADR_R {
        USBADR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Device Address Advance"]
    #[inline(always)]
    #[must_use]
    pub fn usbadra(&mut self) -> USBADRA_W<24> {
        USBADRA_W::new(self)
    }
    #[doc = "Bits 25:31 - Device Address. These bits correspond to the USB device address"]
    #[inline(always)]
    #[must_use]
    pub fn usbadr(&mut self) -> USBADR_W<25> {
        USBADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr_periodiclistbase_deviceaddr](index.html) module"]
pub struct DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC;
impl crate::RegisterSpec for DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddr_periodiclistbase_deviceaddr::R](R) reader structure"]
impl crate::Readable for DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deviceaddr_periodiclistbase_deviceaddr::W](W) writer structure"]
impl crate::Writable for DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVICEADDR to value 0"]
impl crate::Resettable for DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
