#[doc = "Register `SRFM` reader"]
pub struct R(crate::R<SRFM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRFM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRFM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRFM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FreqMeas` reader - Frequency measurement data"]
pub type FREQ_MEAS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Frequency measurement data"]
    #[inline(always)]
    pub fn freq_meas(&self) -> FREQ_MEAS_R {
        FREQ_MEAS_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FreqMeas Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srfm](index.html) module"]
pub struct SRFM_SPEC;
impl crate::RegisterSpec for SRFM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srfm::R](R) reader structure"]
impl crate::Readable for SRFM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRFM to value 0"]
impl crate::Resettable for SRFM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
