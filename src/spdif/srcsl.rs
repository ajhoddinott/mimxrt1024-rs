#[doc = "Register `SRCSL` reader"]
pub struct R(crate::R<SRCSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxCChannel_l` reader - SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
pub type RX_CCHANNEL_L_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive C channel register, contains next 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub fn rx_cchannel_l(&self) -> RX_CCHANNEL_L_R {
        RX_CCHANNEL_L_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "SPDIFRxCChannel_l Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcsl](index.html) module"]
pub struct SRCSL_SPEC;
impl crate::RegisterSpec for SRCSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcsl::R](R) reader structure"]
impl crate::Readable for SRCSL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRCSL to value 0"]
impl crate::Resettable for SRCSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
