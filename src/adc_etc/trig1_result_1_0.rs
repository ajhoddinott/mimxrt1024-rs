#[doc = "Register `TRIG1_RESULT_1_0` reader"]
pub struct R(crate::R<TRIG1_RESULT_1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG1_RESULT_1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG1_RESULT_1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG1_RESULT_1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA1` reader - Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig1_result_1_0](index.html) module"]
pub struct TRIG1_RESULT_1_0_SPEC;
impl crate::RegisterSpec for TRIG1_RESULT_1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig1_result_1_0::R](R) reader structure"]
impl crate::Readable for TRIG1_RESULT_1_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIG1_RESULT_1_0 to value 0"]
impl crate::Resettable for TRIG1_RESULT_1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
