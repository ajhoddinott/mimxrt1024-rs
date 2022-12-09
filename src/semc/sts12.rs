#[doc = "Register `STS12` reader"]
pub struct R(crate::R<STS12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NDADDR` reader - This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
pub type NDADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    #[inline(always)]
    pub fn ndaddr(&self) -> NDADDR_R {
        NDADDR_R::new(self.bits)
    }
}
#[doc = "Status Register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts12](index.html) module"]
pub struct STS12_SPEC;
impl crate::RegisterSpec for STS12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts12::R](R) reader structure"]
impl crate::Readable for STS12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS12 to value 0"]
impl crate::Resettable for STS12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
