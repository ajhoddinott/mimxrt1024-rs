#[doc = "Register `SRQ` reader"]
pub struct R(crate::R<SRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxQChannel` reader - SPDIF receive Q channel register, contains next 3 Q channel bytes"]
pub type RX_QCHANNEL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive Q channel register, contains next 3 Q channel bytes"]
    #[inline(always)]
    pub fn rx_qchannel(&self) -> RX_QCHANNEL_R {
        RX_QCHANNEL_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "QchannelRx Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srq](index.html) module"]
pub struct SRQ_SPEC;
impl crate::RegisterSpec for SRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srq::R](R) reader structure"]
impl crate::Readable for SRQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRQ to value 0"]
impl crate::Resettable for SRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
