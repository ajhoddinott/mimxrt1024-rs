#[doc = "Register `SBMR2` reader"]
pub struct R(crate::R<SBMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_CONFIG` reader - SECONFIG\\[1\\]
shows the state of the SECONFIG\\[1\\]
fuse"]
pub type SEC_CONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BT_FUSE_SEL` reader - BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
pub type BT_FUSE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BMOD` reader - BMOD\\[1:0\\]
shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
pub type BMOD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - SECONFIG\\[1\\]
shows the state of the SECONFIG\\[1\\]
fuse"]
    #[inline(always)]
    pub fn sec_config(&self) -> SEC_CONFIG_R {
        SEC_CONFIG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    #[inline(always)]
    pub fn bt_fuse_sel(&self) -> BT_FUSE_SEL_R {
        BT_FUSE_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 24:25 - BMOD\\[1:0\\]
shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    #[inline(always)]
    pub fn bmod(&self) -> BMOD_R {
        BMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[doc = "SRC Boot Mode Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbmr2](index.html) module"]
pub struct SBMR2_SPEC;
impl crate::RegisterSpec for SBMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbmr2::R](R) reader structure"]
impl crate::Readable for SBMR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SBMR2 to value 0"]
impl crate::Resettable for SBMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
