#[doc = "Register `SRU` reader"]
pub struct R(crate::R<SRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxUChannel` reader - SPDIF receive U channel register, contains next 3 U channel bytes"]
pub type RX_UCHANNEL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive U channel register, contains next 3 U channel bytes"]
    #[inline(always)]
    pub fn rx_uchannel(&self) -> RX_UCHANNEL_R {
        RX_UCHANNEL_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "UchannelRx Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sru](index.html) module"]
pub struct SRU_SPEC;
impl crate::RegisterSpec for SRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sru::R](R) reader structure"]
impl crate::Readable for SRU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRU to value 0"]
impl crate::Resettable for SRU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
