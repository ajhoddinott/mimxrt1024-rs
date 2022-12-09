#[doc = "Register `CAPABILITY1` reader"]
pub struct R(crate::R<CAPABILITY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPABILITY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPABILITY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPABILITY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIPHER_ALGORITHMS` reader - One-hot field indicating which cipher algorithms are available"]
pub type CIPHER_ALGORITHMS_R = crate::FieldReader<u16, CIPHER_ALGORITHMS_A>;
#[doc = "One-hot field indicating which cipher algorithms are available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CIPHER_ALGORITHMS_A {
    #[doc = "1: AES128"]
    AES128 = 1,
}
impl From<CIPHER_ALGORITHMS_A> for u16 {
    #[inline(always)]
    fn from(variant: CIPHER_ALGORITHMS_A) -> Self {
        variant as _
    }
}
impl CIPHER_ALGORITHMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIPHER_ALGORITHMS_A> {
        match self.bits {
            1 => Some(CIPHER_ALGORITHMS_A::AES128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_ALGORITHMS_A::AES128
    }
}
#[doc = "Field `HASH_ALGORITHMS` reader - One-hot field indicating which hashing features are implemented in the hardware"]
pub type HASH_ALGORITHMS_R = crate::FieldReader<u16, HASH_ALGORITHMS_A>;
#[doc = "One-hot field indicating which hashing features are implemented in the hardware\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HASH_ALGORITHMS_A {
    #[doc = "1: SHA1"]
    SHA1 = 1,
    #[doc = "2: CRC32"]
    CRC32 = 2,
    #[doc = "4: SHA256"]
    SHA256 = 4,
}
impl From<HASH_ALGORITHMS_A> for u16 {
    #[inline(always)]
    fn from(variant: HASH_ALGORITHMS_A) -> Self {
        variant as _
    }
}
impl HASH_ALGORITHMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HASH_ALGORITHMS_A> {
        match self.bits {
            1 => Some(HASH_ALGORITHMS_A::SHA1),
            2 => Some(HASH_ALGORITHMS_A::CRC32),
            4 => Some(HASH_ALGORITHMS_A::SHA256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_ALGORITHMS_A::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_ALGORITHMS_A::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_ALGORITHMS_A::SHA256
    }
}
impl R {
    #[doc = "Bits 0:15 - One-hot field indicating which cipher algorithms are available"]
    #[inline(always)]
    pub fn cipher_algorithms(&self) -> CIPHER_ALGORITHMS_R {
        CIPHER_ALGORITHMS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - One-hot field indicating which hashing features are implemented in the hardware"]
    #[inline(always)]
    pub fn hash_algorithms(&self) -> HASH_ALGORITHMS_R {
        HASH_ALGORITHMS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "DCP capability 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capability1](index.html) module"]
pub struct CAPABILITY1_SPEC;
impl crate::RegisterSpec for CAPABILITY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capability1::R](R) reader structure"]
impl crate::Readable for CAPABILITY1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPABILITY1 to value 0x0007_0001"]
impl crate::Resettable for CAPABILITY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0001;
}
