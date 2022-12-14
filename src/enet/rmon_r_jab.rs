#[doc = "Register `RMON_R_JAB` reader"]
pub struct R(crate::R<RMON_R_JAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_JAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_JAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_JAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of receive packets greater than MAX_FL and bad CRC"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of receive packets greater than MAX_FL and bad CRC"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_jab](index.html) module"]
pub struct RMON_R_JAB_SPEC;
impl crate::RegisterSpec for RMON_R_JAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_jab::R](R) reader structure"]
impl crate::Readable for RMON_R_JAB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_JAB to value 0"]
impl crate::Resettable for RMON_R_JAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
