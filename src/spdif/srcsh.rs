#[doc = "Register `SRCSH` reader"]
pub struct R(crate::R<SRCSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxCChannel_h` reader - SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
pub type RX_CCHANNEL_H_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - SPDIF receive C channel register, contains first 24 bits of C channel without interpretation"]
    #[inline(always)]
    pub fn rx_cchannel_h(&self) -> RX_CCHANNEL_H_R {
        RX_CCHANNEL_H_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "SPDIFRxCChannel_h Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcsh](index.html) module"]
pub struct SRCSH_SPEC;
impl crate::RegisterSpec for SRCSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcsh::R](R) reader structure"]
impl crate::Readable for SRCSH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRCSH to value 0"]
impl crate::Resettable for SRCSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
