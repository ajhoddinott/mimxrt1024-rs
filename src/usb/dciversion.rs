#[doc = "Register `DCIVERSION` reader"]
pub struct R(crate::R<DCIVERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCIVERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCIVERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCIVERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCIVERSION` reader - Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
pub type DCIVERSION_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
    #[inline(always)]
    pub fn dciversion(&self) -> DCIVERSION_R {
        DCIVERSION_R::new(self.bits)
    }
}
#[doc = "Device Controller Interface Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dciversion](index.html) module"]
pub struct DCIVERSION_SPEC;
impl crate::RegisterSpec for DCIVERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dciversion::R](R) reader structure"]
impl crate::Readable for DCIVERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCIVERSION to value 0x01"]
impl crate::Resettable for DCIVERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
