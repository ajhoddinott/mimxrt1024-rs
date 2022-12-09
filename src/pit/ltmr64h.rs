#[doc = "Register `LTMR64H` reader"]
pub struct R(crate::R<LTMR64H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTMR64H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTMR64H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTMR64H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LTH` reader - Life Timer value"]
pub type LTH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn lth(&self) -> LTH_R {
        LTH_R::new(self.bits)
    }
}
#[doc = "PIT Upper Lifetime Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltmr64h](index.html) module"]
pub struct LTMR64H_SPEC;
impl crate::RegisterSpec for LTMR64H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltmr64h::R](R) reader structure"]
impl crate::Readable for LTMR64H_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTMR64H to value 0"]
impl crate::Resettable for LTMR64H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
