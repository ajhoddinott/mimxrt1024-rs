#[doc = "Register `PKRSQ` reader"]
pub struct R(crate::R<MAX_SQ_PKRSQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_SQ_PKRSQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_SQ_PKRSQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_SQ_PKRSQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_SQ` reader - Poker Square Calculation Result."]
pub type PKR_SQ_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Square Calculation Result."]
    #[inline(always)]
    pub fn pkr_sq(&self) -> PKR_SQ_R {
        PKR_SQ_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Poker Square Calculation Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_sq_pkrsq](index.html) module"]
pub struct MAX_SQ_PKRSQ_SPEC;
impl crate::RegisterSpec for MAX_SQ_PKRSQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_sq_pkrsq::R](R) reader structure"]
impl crate::Readable for MAX_SQ_PKRSQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRSQ to value 0"]
impl crate::Resettable for MAX_SQ_PKRSQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
