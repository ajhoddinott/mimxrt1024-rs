#[doc = "Register `HWDEVICE` reader"]
pub struct R(crate::R<HWDEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWDEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWDEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWDEVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC` reader - Device Capable. Indicating whether device operation mode is supported or not."]
pub type DC_R = crate::BitReader<DC_A>;
#[doc = "Device Capable. Indicating whether device operation mode is supported or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_A {
    #[doc = "0: Not supported"]
    DC_0 = 0,
    #[doc = "1: Supported"]
    DC_1 = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
impl DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::DC_0,
            true => DC_A::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline(always)]
    pub fn is_dc_0(&self) -> bool {
        *self == DC_A::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline(always)]
    pub fn is_dc_1(&self) -> bool {
        *self == DC_A::DC_1
    }
}
#[doc = "Field `DEVEP` reader - Device Endpoint Number"]
pub type DEVEP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Device Capable. Indicating whether device operation mode is supported or not."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Device Endpoint Number"]
    #[inline(always)]
    pub fn devep(&self) -> DEVEP_R {
        DEVEP_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
#[doc = "Device Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwdevice](index.html) module"]
pub struct HWDEVICE_SPEC;
impl crate::RegisterSpec for HWDEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwdevice::R](R) reader structure"]
impl crate::Readable for HWDEVICE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWDEVICE to value 0x11"]
impl crate::Resettable for HWDEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
