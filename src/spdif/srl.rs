#[doc = "Register `SRL` reader"]
pub struct R(crate::R<SRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxDataLeft` reader - Processor receive SPDIF data left"]
pub type RX_DATA_LEFT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Processor receive SPDIF data left"]
    #[inline(always)]
    pub fn rx_data_left(&self) -> RX_DATA_LEFT_R {
        RX_DATA_LEFT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "SPDIFRxLeft Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srl](index.html) module"]
pub struct SRL_SPEC;
impl crate::RegisterSpec for SRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srl::R](R) reader structure"]
impl crate::Readable for SRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRL to value 0"]
impl crate::Resettable for SRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
