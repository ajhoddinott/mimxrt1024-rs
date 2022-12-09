#[doc = "Register `SRR` reader"]
pub struct R(crate::R<SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxDataRight` reader - Processor receive SPDIF data right"]
pub type RX_DATA_RIGHT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Processor receive SPDIF data right"]
    #[inline(always)]
    pub fn rx_data_right(&self) -> RX_DATA_RIGHT_R {
        RX_DATA_RIGHT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "SPDIFRxRight Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](index.html) module"]
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srr::R](R) reader structure"]
impl crate::Readable for SRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
