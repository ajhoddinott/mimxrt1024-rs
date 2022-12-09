#[doc = "Register `PACKET3` reader"]
pub struct R(crate::R<PACKET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Source buffer address pointer"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source buffer address pointer"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "DCP work packet 3 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet3](index.html) module"]
pub struct PACKET3_SPEC;
impl crate::RegisterSpec for PACKET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet3::R](R) reader structure"]
impl crate::Readable for PACKET3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET3 to value 0"]
impl crate::Resettable for PACKET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
