#[doc = "Register `TRIG7_RESULT_3_2` reader"]
pub struct R(crate::R<TRIG7_RESULT_3_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG7_RESULT_3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG7_RESULT_3_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG7_RESULT_3_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA2` reader - Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA3` reader - Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig7_result_3_2](index.html) module"]
pub struct TRIG7_RESULT_3_2_SPEC;
impl crate::RegisterSpec for TRIG7_RESULT_3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig7_result_3_2::R](R) reader structure"]
impl crate::Readable for TRIG7_RESULT_3_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIG7_RESULT_3_2 to value 0"]
impl crate::Resettable for TRIG7_RESULT_3_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
