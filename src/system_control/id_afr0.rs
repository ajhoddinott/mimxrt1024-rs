#[doc = "Register `ID_AFR0` reader"]
pub struct R(crate::R<ID_AFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_AFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_AFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_AFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMPLEMENTATION_DEFINED0` reader - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
pub type IMPLEMENTATION_DEFINED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTATION_DEFINED1` reader - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
pub type IMPLEMENTATION_DEFINED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTATION_DEFINED2` reader - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
pub type IMPLEMENTATION_DEFINED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTATION_DEFINED3` reader - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
pub type IMPLEMENTATION_DEFINED3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined0(&self) -> IMPLEMENTATION_DEFINED0_R {
        IMPLEMENTATION_DEFINED0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined1(&self) -> IMPLEMENTATION_DEFINED1_R {
        IMPLEMENTATION_DEFINED1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined2(&self) -> IMPLEMENTATION_DEFINED2_R {
        IMPLEMENTATION_DEFINED2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Gives information about the IMPLEMENTATION DEFINED features of a processor implementation."]
    #[inline(always)]
    pub fn implementation_defined3(&self) -> IMPLEMENTATION_DEFINED3_R {
        IMPLEMENTATION_DEFINED3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Auxiliary Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_afr0](index.html) module"]
pub struct ID_AFR0_SPEC;
impl crate::RegisterSpec for ID_AFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_afr0::R](R) reader structure"]
impl crate::Readable for ID_AFR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_AFR0 to value 0"]
impl crate::Resettable for ID_AFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
