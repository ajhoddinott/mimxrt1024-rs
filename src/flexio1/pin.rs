#[doc = "Register `PIN` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI` reader - Pin Data Input"]
pub type PDI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pin Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(self.bits)
    }
}
#[doc = "Pin State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIN to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
