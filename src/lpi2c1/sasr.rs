#[doc = "Register `SASR` reader"]
pub struct R(crate::R<SASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RADDR` reader - Received Address"]
pub type RADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ANV` reader - Address Not Valid"]
pub type ANV_R = crate::BitReader<ANV_A>;
#[doc = "Address Not Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANV_A {
    #[doc = "0: Received Address (RADDR) is valid"]
    VALID = 0,
    #[doc = "1: Received Address (RADDR) is not valid"]
    NOT_VALID = 1,
}
impl From<ANV_A> for bool {
    #[inline(always)]
    fn from(variant: ANV_A) -> Self {
        variant as u8 != 0
    }
}
impl ANV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANV_A {
        match self.bits {
            false => ANV_A::VALID,
            true => ANV_A::NOT_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == ANV_A::VALID
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == ANV_A::NOT_VALID
    }
}
impl R {
    #[doc = "Bits 0:10 - Received Address"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - Address Not Valid"]
    #[inline(always)]
    pub fn anv(&self) -> ANV_R {
        ANV_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Slave Address Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sasr](index.html) module"]
pub struct SASR_SPEC;
impl crate::RegisterSpec for SASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sasr::R](R) reader structure"]
impl crate::Readable for SASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SASR to value 0x4000"]
impl crate::Resettable for SASR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
