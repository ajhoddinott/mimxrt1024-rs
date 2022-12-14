#[doc = "Register `DBGDATA` reader"]
pub struct R(crate::R<DBGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Debug data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Debug data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "DCP debug data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgdata](index.html) module"]
pub struct DBGDATA_SPEC;
impl crate::RegisterSpec for DBGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgdata::R](R) reader structure"]
impl crate::Readable for DBGDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBGDATA to value 0"]
impl crate::Resettable for DBGDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
