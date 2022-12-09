#[doc = "Register `HCIVERSION` reader"]
pub struct R(crate::R<HCIVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCIVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCIVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCIVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCIVERSION` reader - Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
pub type HCIVERSION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
    #[inline(always)]
    pub fn hciversion(&self) -> HCIVERSION_R {
        HCIVERSION_R::new(self.bits)
    }
}
#[doc = "Host Controller Interface Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hciversion](index.html) module"]
pub struct HCIVERSION_SPEC;
impl crate::RegisterSpec for HCIVERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hciversion::R](R) reader structure"]
impl crate::Readable for HCIVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCIVERSION to value 0x0100"]
impl crate::Resettable for HCIVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
