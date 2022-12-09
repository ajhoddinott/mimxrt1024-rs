#[doc = "Register `HPVIDR1` reader"]
pub struct R(crate::R<HPVIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPVIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPVIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPVIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MINOR_REV` reader - SNVS block minor version number"]
pub type MINOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR_REV` reader - SNVS block major version number"]
pub type MAJOR_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_ID` reader - SNVS block ID"]
pub type IP_ID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - SNVS block minor version number"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SNVS block major version number"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - SNVS block ID"]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SNVS_HP Version ID Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpvidr1](index.html) module"]
pub struct HPVIDR1_SPEC;
impl crate::RegisterSpec for HPVIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpvidr1::R](R) reader structure"]
impl crate::Readable for HPVIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPVIDR1 to value 0x003e_0104"]
impl crate::Resettable for HPVIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x003e_0104;
}
