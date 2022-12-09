#[doc = "Register `RFR0` reader"]
pub struct R(crate::R<RFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFP` reader - Read FIFO Pointer"]
pub type RFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WFP` reader - Write FIFO Pointer"]
pub type WFP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr0](index.html) module"]
pub struct RFR0_SPEC;
impl crate::RegisterSpec for RFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfr0::R](R) reader structure"]
impl crate::Readable for RFR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFR0 to value 0"]
impl crate::Resettable for RFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
