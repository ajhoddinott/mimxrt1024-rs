#[doc = "Register `RMON_R_MC_PKT` reader"]
pub struct R(crate::R<RMON_R_MC_PKT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_MC_PKT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_MC_PKT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_MC_PKT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of receive multicast packets"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of receive multicast packets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Rx Multicast Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_mc_pkt](index.html) module"]
pub struct RMON_R_MC_PKT_SPEC;
impl crate::RegisterSpec for RMON_R_MC_PKT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_mc_pkt::R](R) reader structure"]
impl crate::Readable for RMON_R_MC_PKT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_MC_PKT to value 0"]
impl crate::Resettable for RMON_R_MC_PKT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
