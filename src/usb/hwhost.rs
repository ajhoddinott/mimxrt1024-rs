#[doc = "Register `HWHOST` reader"]
pub struct R(crate::R<HWHOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWHOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWHOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWHOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HC` reader - Host Capable. Indicating whether host operation mode is supported or not."]
pub type HC_R = crate::BitReader<HC_A>;
#[doc = "Host Capable. Indicating whether host operation mode is supported or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HC_A {
    #[doc = "0: Not supported"]
    HC_0 = 0,
    #[doc = "1: Supported"]
    HC_1 = 1,
}
impl From<HC_A> for bool {
    #[inline(always)]
    fn from(variant: HC_A) -> Self {
        variant as u8 != 0
    }
}
impl HC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HC_A {
        match self.bits {
            false => HC_A::HC_0,
            true => HC_A::HC_1,
        }
    }
    #[doc = "Checks if the value of the field is `HC_0`"]
    #[inline(always)]
    pub fn is_hc_0(&self) -> bool {
        *self == HC_A::HC_0
    }
    #[doc = "Checks if the value of the field is `HC_1`"]
    #[inline(always)]
    pub fn is_hc_1(&self) -> bool {
        *self == HC_A::HC_1
    }
}
#[doc = "Field `NPORT` reader - The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
pub type NPORT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Host Capable. Indicating whether host operation mode is supported or not."]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
    #[inline(always)]
    pub fn nport(&self) -> NPORT_R {
        NPORT_R::new(((self.bits >> 1) & 7) as u8)
    }
}
#[doc = "Host Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwhost](index.html) module"]
pub struct HWHOST_SPEC;
impl crate::RegisterSpec for HWHOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwhost::R](R) reader structure"]
impl crate::Readable for HWHOST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWHOST to value 0x1002_0001"]
impl crate::Resettable for HWHOST_SPEC {
    const RESET_VALUE: Self::Ux = 0x1002_0001;
}
