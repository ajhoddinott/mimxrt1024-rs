#[doc = "Register `HPHACR` reader"]
pub struct R(crate::R<HPHACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPHACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPHACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPHACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HAC_COUNTER` reader - High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
pub type HAC_COUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - High Assurance Counter When the HAC_EN bit is set and the SSM is in the soft fail state, this counter starts to count down with the system clock"]
    #[inline(always)]
    pub fn hac_counter(&self) -> HAC_COUNTER_R {
        HAC_COUNTER_R::new(self.bits)
    }
}
#[doc = "SNVS_HP High Assurance Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hphacr](index.html) module"]
pub struct HPHACR_SPEC;
impl crate::RegisterSpec for HPHACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hphacr::R](R) reader structure"]
impl crate::Readable for HPHACR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPHACR to value 0"]
impl crate::Resettable for HPHACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
