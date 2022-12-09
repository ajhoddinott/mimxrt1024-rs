#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Configuration number"]
pub type ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NID` reader - Complement version of ID"]
pub type NID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` reader - Revision number of the controller core."]
pub type REVISION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Configuration number"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Complement version of ID"]
    #[inline(always)]
    pub fn nid(&self) -> NID_R {
        NID_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Revision number of the controller core."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID to value 0xe4a1_fa05"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0xe4a1_fa05;
}
