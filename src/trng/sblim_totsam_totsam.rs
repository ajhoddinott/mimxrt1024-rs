#[doc = "Register `TOTSAM` reader"]
pub struct R(crate::R<SBLIM_TOTSAM_TOTSAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBLIM_TOTSAM_TOTSAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBLIM_TOTSAM_TOTSAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBLIM_TOTSAM_TOTSAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOT_SAM` reader - Total Samples"]
pub type TOT_SAM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Total Samples"]
    #[inline(always)]
    pub fn tot_sam(&self) -> TOT_SAM_R {
        TOT_SAM_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "Total Samples Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sblim_totsam_totsam](index.html) module"]
pub struct SBLIM_TOTSAM_TOTSAM_SPEC;
impl crate::RegisterSpec for SBLIM_TOTSAM_TOTSAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sblim_totsam_totsam::R](R) reader structure"]
impl crate::Readable for SBLIM_TOTSAM_TOTSAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TOTSAM to value 0"]
impl crate::Resettable for SBLIM_TOTSAM_TOTSAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
