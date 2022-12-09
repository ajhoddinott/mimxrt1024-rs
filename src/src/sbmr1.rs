#[doc = "Register `SBMR1` reader"]
pub struct R(crate::R<SBMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOOT_CFG1` reader - Refer to fusemap."]
pub type BOOT_CFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_CFG2` reader - Refer to fusemap."]
pub type BOOT_CFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_CFG3` reader - Refer to fusemap."]
pub type BOOT_CFG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_CFG4` reader - Refer to fusemap."]
pub type BOOT_CFG4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg1(&self) -> BOOT_CFG1_R {
        BOOT_CFG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg2(&self) -> BOOT_CFG2_R {
        BOOT_CFG2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg3(&self) -> BOOT_CFG3_R {
        BOOT_CFG3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Refer to fusemap."]
    #[inline(always)]
    pub fn boot_cfg4(&self) -> BOOT_CFG4_R {
        BOOT_CFG4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "SRC Boot Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbmr1](index.html) module"]
pub struct SBMR1_SPEC;
impl crate::RegisterSpec for SBMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbmr1::R](R) reader structure"]
impl crate::Readable for SBMR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SBMR1 to value 0"]
impl crate::Resettable for SBMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
