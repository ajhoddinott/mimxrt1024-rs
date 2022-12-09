#[doc = "Register `ID_MMFR1` reader"]
pub struct R(crate::R<ID_MMFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_MMFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_MMFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_MMFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID_MMFR1` reader - Gives information about the implemented memory model and memory management support."]
pub type ID_MMFR1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Gives information about the implemented memory model and memory management support."]
    #[inline(always)]
    pub fn id_mmfr1(&self) -> ID_MMFR1_R {
        ID_MMFR1_R::new(self.bits)
    }
}
#[doc = "Memory Model Feature Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr1](index.html) module"]
pub struct ID_MMFR1_SPEC;
impl crate::RegisterSpec for ID_MMFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_mmfr1::R](R) reader structure"]
impl crate::Readable for ID_MMFR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_MMFR1 to value 0"]
impl crate::Resettable for ID_MMFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
