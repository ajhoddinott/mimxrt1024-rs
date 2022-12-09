#[doc = "Register `DLL_STATUS` reader"]
pub struct R(crate::R<DLL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLL_STS_SLV_LOCK` reader - Slave delay-line lock status"]
pub type DLL_STS_SLV_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `DLL_STS_REF_LOCK` reader - Reference DLL lock status"]
pub type DLL_STS_REF_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `DLL_STS_SLV_SEL` reader - Slave delay line select status"]
pub type DLL_STS_SLV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_STS_REF_SEL` reader - Reference delay line select taps"]
pub type DLL_STS_REF_SEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Slave delay-line lock status"]
    #[inline(always)]
    pub fn dll_sts_slv_lock(&self) -> DLL_STS_SLV_LOCK_R {
        DLL_STS_SLV_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference DLL lock status"]
    #[inline(always)]
    pub fn dll_sts_ref_lock(&self) -> DLL_STS_REF_LOCK_R {
        DLL_STS_REF_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - Slave delay line select status"]
    #[inline(always)]
    pub fn dll_sts_slv_sel(&self) -> DLL_STS_SLV_SEL_R {
        DLL_STS_SLV_SEL_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Reference delay line select taps"]
    #[inline(always)]
    pub fn dll_sts_ref_sel(&self) -> DLL_STS_REF_SEL_R {
        DLL_STS_REF_SEL_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "DLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll_status](index.html) module"]
pub struct DLL_STATUS_SPEC;
impl crate::RegisterSpec for DLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll_status::R](R) reader structure"]
impl crate::Readable for DLL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLL_STATUS to value 0x0200"]
impl crate::Resettable for DLL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
