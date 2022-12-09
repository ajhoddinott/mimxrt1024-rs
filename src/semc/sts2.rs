#[doc = "Register `STS2` reader"]
pub struct R(crate::R<STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NDWRPEND` reader - This field indicating whether there is pending AXI command (write) to NAND device."]
pub type NDWRPEND_R = crate::BitReader<NDWRPEND_A>;
#[doc = "This field indicating whether there is pending AXI command (write) to NAND device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDWRPEND_A {
    #[doc = "0: No pending"]
    NDWRPEND_0 = 0,
    #[doc = "1: Pending"]
    NDWRPEND_1 = 1,
}
impl From<NDWRPEND_A> for bool {
    #[inline(always)]
    fn from(variant: NDWRPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl NDWRPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDWRPEND_A {
        match self.bits {
            false => NDWRPEND_A::NDWRPEND_0,
            true => NDWRPEND_A::NDWRPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_0`"]
    #[inline(always)]
    pub fn is_ndwrpend_0(&self) -> bool {
        *self == NDWRPEND_A::NDWRPEND_0
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_1`"]
    #[inline(always)]
    pub fn is_ndwrpend_1(&self) -> bool {
        *self == NDWRPEND_A::NDWRPEND_1
    }
}
impl R {
    #[doc = "Bit 3 - This field indicating whether there is pending AXI command (write) to NAND device."]
    #[inline(always)]
    pub fn ndwrpend(&self) -> NDWRPEND_R {
        NDWRPEND_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts2](index.html) module"]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts2::R](R) reader structure"]
impl crate::Readable for STS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
