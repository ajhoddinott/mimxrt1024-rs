#[doc = "Register `HW_OCOTP_VERSION` reader"]
pub struct R(crate::R<HW_OCOTP_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STEP` reader - RTL Version Steping"]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MINOR` reader - Minor RTL Version"]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` reader - Major RTL Version"]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - RTL Version Steping"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor RTL Version"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major RTL Version"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "OTP Controller Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_version](index.html) module"]
pub struct HW_OCOTP_VERSION_SPEC;
impl crate::RegisterSpec for HW_OCOTP_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_version::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HW_OCOTP_VERSION to value 0x0600_0000"]
impl crate::Resettable for HW_OCOTP_VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_0000;
}
