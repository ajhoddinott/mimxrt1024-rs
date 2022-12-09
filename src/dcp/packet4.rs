#[doc = "Register `PACKET4` reader"]
pub struct R(crate::R<PACKET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Destination buffer address pointer"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination buffer address pointer"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "DCP work packet 4 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet4](index.html) module"]
pub struct PACKET4_SPEC;
impl crate::RegisterSpec for PACKET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet4::R](R) reader structure"]
impl crate::Readable for PACKET4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET4 to value 0"]
impl crate::Resettable for PACKET4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
