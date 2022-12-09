#[doc = "Register `RDR0` reader"]
pub struct R(crate::R<RDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receive Data Register"]
pub type RDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Register"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new(self.bits)
    }
}
#[doc = "Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr0](index.html) module"]
pub struct RDR0_SPEC;
impl crate::RegisterSpec for RDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdr0::R](R) reader structure"]
impl crate::Readable for RDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDR0 to value 0"]
impl crate::Resettable for RDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
