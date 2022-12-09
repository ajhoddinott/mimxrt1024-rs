#[doc = "Register `SCMC` reader"]
pub struct R(crate::R<SCML_MC_SCMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCML_MC_SCMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCML_MC_SCMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCML_MC_SCMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MONO_CT` reader - Monobit Count"]
pub type MONO_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monobit Count"]
    #[inline(always)]
    pub fn mono_ct(&self) -> MONO_CT_R {
        MONO_CT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Monobit Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scml_mc_scmc](index.html) module"]
pub struct SCML_MC_SCMC_SPEC;
impl crate::RegisterSpec for SCML_MC_SCMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scml_mc_scmc::R](R) reader structure"]
impl crate::Readable for SCML_MC_SCMC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCMC to value 0"]
impl crate::Resettable for SCML_MC_SCMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
