#[doc = "Register `ID_MMFR3` reader"]
pub struct R(crate::R<ID_MMFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_MMFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_MMFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_MMFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID_MMFR3` reader - Gives information about the implemented memory model and memory management support."]
pub type ID_MMFR3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Gives information about the implemented memory model and memory management support."]
    #[inline(always)]
    pub fn id_mmfr3(&self) -> ID_MMFR3_R {
        ID_MMFR3_R::new(self.bits)
    }
}
#[doc = "Memory Model Feature Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr3](index.html) module"]
pub struct ID_MMFR3_SPEC;
impl crate::RegisterSpec for ID_MMFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_mmfr3::R](R) reader structure"]
impl crate::Readable for ID_MMFR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_MMFR3 to value 0"]
impl crate::Resettable for ID_MMFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
