#[doc = "Register `HPVIDR2` reader"]
pub struct R(crate::R<HPVIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPVIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPVIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPVIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CONFIG_OPT` reader - SNVS Configuration Options"]
pub type CONFIG_OPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECO_REV` reader - SNVS ECO Revision"]
pub type ECO_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTG_OPT` reader - SNVS Integration Options"]
pub type INTG_OPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_ERA` reader - IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
pub type IP_ERA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SNVS Configuration Options"]
    #[inline(always)]
    pub fn config_opt(&self) -> CONFIG_OPT_R {
        CONFIG_OPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SNVS ECO Revision"]
    #[inline(always)]
    pub fn eco_rev(&self) -> ECO_REV_R {
        ECO_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SNVS Integration Options"]
    #[inline(always)]
    pub fn intg_opt(&self) -> INTG_OPT_R {
        INTG_OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - IP Era 00h - Era 1 or 2 03h - Era 3 04h - Era 4 05h - Era 5 06h - Era 6"]
    #[inline(always)]
    pub fn ip_era(&self) -> IP_ERA_R {
        IP_ERA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "SNVS_HP Version ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpvidr2](index.html) module"]
pub struct HPVIDR2_SPEC;
impl crate::RegisterSpec for HPVIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpvidr2::R](R) reader structure"]
impl crate::Readable for HPVIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HPVIDR2 to value 0x0600_0000"]
impl crate::Resettable for HPVIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0000;
}
