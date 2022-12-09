#[doc = "Register `FRQCNT` reader"]
pub struct R(crate::R<MAX_CNT_FRQCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_CNT_FRQCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_CNT_FRQCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_CNT_FRQCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRQ_CT` reader - Frequency Count"]
pub type FRQ_CT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn frq_ct(&self) -> FRQ_CT_R {
        FRQ_CT_R::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Frequency Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_cnt_frqcnt](index.html) module"]
pub struct MAX_CNT_FRQCNT_SPEC;
impl crate::RegisterSpec for MAX_CNT_FRQCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_cnt_frqcnt::R](R) reader structure"]
impl crate::Readable for MAX_CNT_FRQCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRQCNT to value 0"]
impl crate::Resettable for MAX_CNT_FRQCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
