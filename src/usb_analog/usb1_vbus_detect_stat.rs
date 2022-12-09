#[doc = "Register `USB1_VBUS_DETECT_STAT` reader"]
pub struct R(crate::R<USB1_VBUS_DETECT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DETECT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DETECT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DETECT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SESSEND` reader - Session End for USB OTG"]
pub type SESSEND_R = crate::BitReader<bool>;
#[doc = "Field `BVALID` reader - Indicates VBus is valid for a B-peripheral"]
pub type BVALID_R = crate::BitReader<bool>;
#[doc = "Field `AVALID` reader - Indicates VBus is valid for a A-peripheral"]
pub type AVALID_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_VALID` reader - VBus valid for USB OTG"]
pub type VBUS_VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Session End for USB OTG"]
    #[inline(always)]
    pub fn sessend(&self) -> SESSEND_R {
        SESSEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub fn bvalid(&self) -> BVALID_R {
        BVALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub fn avalid(&self) -> AVALID_R {
        AVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBus valid for USB OTG"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "USB VBUS Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect_stat](index.html) module"]
pub struct USB1_VBUS_DETECT_STAT_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DETECT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_detect_stat::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT_STAT to value 0"]
impl crate::Resettable for USB1_VBUS_DETECT_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
