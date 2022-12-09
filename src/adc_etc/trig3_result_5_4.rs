#[doc = "Register `TRIG3_RESULT_5_4` reader"]
pub struct R(crate::R<TRIG3_RESULT_5_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG3_RESULT_5_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG3_RESULT_5_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG3_RESULT_5_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA4` reader - Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA5` reader - Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
pub type DATA5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig3_result_5_4](index.html) module"]
pub struct TRIG3_RESULT_5_4_SPEC;
impl crate::RegisterSpec for TRIG3_RESULT_5_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig3_result_5_4::R](R) reader structure"]
impl crate::Readable for TRIG3_RESULT_5_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIG3_RESULT_5_4 to value 0"]
impl crate::Resettable for TRIG3_RESULT_5_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
