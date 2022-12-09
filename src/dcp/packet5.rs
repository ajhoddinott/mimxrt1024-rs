#[doc = "Register `PACKET5` reader"]
pub struct R(crate::R<PACKET5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Byte count register. This value is the working value and updates as the operation proceeds."]
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Byte count register. This value is the working value and updates as the operation proceeds."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "DCP work packet 5 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet5](index.html) module"]
pub struct PACKET5_SPEC;
impl crate::RegisterSpec for PACKET5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet5::R](R) reader structure"]
impl crate::Readable for PACKET5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET5 to value 0"]
impl crate::Resettable for PACKET5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
