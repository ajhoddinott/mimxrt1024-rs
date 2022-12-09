#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMINLINE` reader - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
pub type IMINLINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMINLINE` reader - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
pub type DMINLINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERG` reader - Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
pub type ERG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CWG` reader - Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
pub type CWG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORMAT` reader - Indicates the implemented CTR format."]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "Indicates the implemented CTR format.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "4: ARMv7 format."]
    FORMAT_4 = 4,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            4 => Some(FORMAT_A::FORMAT_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FORMAT_4`"]
    #[inline(always)]
    pub fn is_format_4(&self) -> bool {
        *self == FORMAT_A::FORMAT_4
    }
}
impl R {
    #[doc = "Bits 0:3 - Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by the processor."]
    #[inline(always)]
    pub fn iminline(&self) -> IMINLINE_R {
        IMINLINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are controlled by the processor."]
    #[inline(always)]
    pub fn dminline(&self) -> DMINLINE_R {
        DMINLINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Exclusives Reservation Granule. The maximum size of the reservation granule that has been implemented for the Load-Exclusive and Store-Exclusive instructions, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Cache Write-back Granule. The maximum size of memory that can be overwritten as a result of the eviction of a cache entry that has had a memory location in it modified, encoded as Log2 of the number of words."]
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Indicates the implemented CTR format."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[doc = "Cache Type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTR to value 0x8000_c000"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_c000;
}
