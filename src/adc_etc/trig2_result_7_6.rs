#[doc = "Register `TRIG2_RESULT_7_6` reader"]
pub struct R(crate::R<TRIG2_RESULT_7_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG2_RESULT_7_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG2_RESULT_7_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG2_RESULT_7_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA6` reader - Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA7` reader - Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA7_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig2_result_7_6](index.html) module"]
pub struct TRIG2_RESULT_7_6_SPEC;
impl crate::RegisterSpec for TRIG2_RESULT_7_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig2_result_7_6::R](R) reader structure"]
impl crate::Readable for TRIG2_RESULT_7_6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIG2_RESULT_7_6 to value 0"]
impl crate::Resettable for TRIG2_RESULT_7_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
