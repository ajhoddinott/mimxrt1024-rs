#[doc = "Register `PACKET6` reader"]
pub struct R(crate::R<PACKET6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - This regiser reflects the payload pointer for the current control packet."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This regiser reflects the payload pointer for the current control packet."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "DCP work packet 6 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet6](index.html) module"]
pub struct PACKET6_SPEC;
impl crate::RegisterSpec for PACKET6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet6::R](R) reader structure"]
impl crate::Readable for PACKET6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET6 to value 0"]
impl crate::Resettable for PACKET6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
