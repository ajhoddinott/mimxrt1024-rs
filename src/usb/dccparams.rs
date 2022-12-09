#[doc = "Register `DCCPARAMS` reader"]
pub struct R(crate::R<DCCPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEN` reader - Device Endpoint Number This field indicates the number of endpoints built into the device controller"]
pub type DEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC` reader - Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device."]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `HC` reader - Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2"]
pub type HC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:4 - Device Endpoint Number This field indicates the number of endpoints built into the device controller"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2"]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Device Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccparams](index.html) module"]
pub struct DCCPARAMS_SPEC;
impl crate::RegisterSpec for DCCPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dccparams::R](R) reader structure"]
impl crate::Readable for DCCPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCCPARAMS to value 0x0188"]
impl crate::Resettable for DCCPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0188;
}
