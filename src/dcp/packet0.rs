#[doc = "Register `PACKET0` reader"]
pub struct R(crate::R<PACKET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Next pointer register"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Next pointer register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "DCP work packet 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet0](index.html) module"]
pub struct PACKET0_SPEC;
impl crate::RegisterSpec for PACKET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet0::R](R) reader structure"]
impl crate::Readable for PACKET0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET0 to value 0"]
impl crate::Resettable for PACKET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
