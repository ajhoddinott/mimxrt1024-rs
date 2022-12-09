#[doc = "Register `PACKET2` reader"]
pub struct R(crate::R<PACKET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIPHER_SELECT` reader - Cipher selection field"]
pub type CIPHER_SELECT_R = crate::FieldReader<u8, CIPHER_SELECT_A>;
#[doc = "Cipher selection field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CIPHER_SELECT_A {
    #[doc = "0: AES128"]
    AES128 = 0,
}
impl From<CIPHER_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_SELECT_A) -> Self {
        variant as _
    }
}
impl CIPHER_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIPHER_SELECT_A> {
        match self.bits {
            0 => Some(CIPHER_SELECT_A::AES128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == CIPHER_SELECT_A::AES128
    }
}
#[doc = "Field `CIPHER_MODE` reader - Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
pub type CIPHER_MODE_R = crate::FieldReader<u8, CIPHER_MODE_A>;
#[doc = "Cipher mode selection field. Reflects the mode of operation for the cipher operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CIPHER_MODE_A {
    #[doc = "0: ECB"]
    ECB = 0,
    #[doc = "1: CBC"]
    CBC = 1,
}
impl From<CIPHER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_MODE_A) -> Self {
        variant as _
    }
}
impl CIPHER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIPHER_MODE_A> {
        match self.bits {
            0 => Some(CIPHER_MODE_A::ECB),
            1 => Some(CIPHER_MODE_A::CBC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CIPHER_MODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CIPHER_MODE_A::CBC
    }
}
#[doc = "Field `KEY_SELECT` reader - Key selection field"]
pub type KEY_SELECT_R = crate::FieldReader<u8, KEY_SELECT_A>;
#[doc = "Key selection field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_SELECT_A {
    #[doc = "0: KEY0"]
    KEY0 = 0,
    #[doc = "1: KEY1"]
    KEY1 = 1,
    #[doc = "2: KEY2"]
    KEY2 = 2,
    #[doc = "3: KEY3"]
    KEY3 = 3,
    #[doc = "254: UNIQUE_KEY"]
    UNIQUE_KEY = 254,
    #[doc = "255: OTP_KEY"]
    OTP_KEY = 255,
}
impl From<KEY_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SELECT_A) -> Self {
        variant as _
    }
}
impl KEY_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_SELECT_A> {
        match self.bits {
            0 => Some(KEY_SELECT_A::KEY0),
            1 => Some(KEY_SELECT_A::KEY1),
            2 => Some(KEY_SELECT_A::KEY2),
            3 => Some(KEY_SELECT_A::KEY3),
            254 => Some(KEY_SELECT_A::UNIQUE_KEY),
            255 => Some(KEY_SELECT_A::OTP_KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY0`"]
    #[inline(always)]
    pub fn is_key0(&self) -> bool {
        *self == KEY_SELECT_A::KEY0
    }
    #[doc = "Checks if the value of the field is `KEY1`"]
    #[inline(always)]
    pub fn is_key1(&self) -> bool {
        *self == KEY_SELECT_A::KEY1
    }
    #[doc = "Checks if the value of the field is `KEY2`"]
    #[inline(always)]
    pub fn is_key2(&self) -> bool {
        *self == KEY_SELECT_A::KEY2
    }
    #[doc = "Checks if the value of the field is `KEY3`"]
    #[inline(always)]
    pub fn is_key3(&self) -> bool {
        *self == KEY_SELECT_A::KEY3
    }
    #[doc = "Checks if the value of the field is `UNIQUE_KEY`"]
    #[inline(always)]
    pub fn is_unique_key(&self) -> bool {
        *self == KEY_SELECT_A::UNIQUE_KEY
    }
    #[doc = "Checks if the value of the field is `OTP_KEY`"]
    #[inline(always)]
    pub fn is_otp_key(&self) -> bool {
        *self == KEY_SELECT_A::OTP_KEY
    }
}
#[doc = "Field `HASH_SELECT` reader - Hash Selection Field"]
pub type HASH_SELECT_R = crate::FieldReader<u8, HASH_SELECT_A>;
#[doc = "Hash Selection Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HASH_SELECT_A {
    #[doc = "0: SHA1"]
    SHA1 = 0,
    #[doc = "1: CRC32"]
    CRC32 = 1,
    #[doc = "2: SHA256"]
    SHA256 = 2,
}
impl From<HASH_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_SELECT_A) -> Self {
        variant as _
    }
}
impl HASH_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HASH_SELECT_A> {
        match self.bits {
            0 => Some(HASH_SELECT_A::SHA1),
            1 => Some(HASH_SELECT_A::CRC32),
            2 => Some(HASH_SELECT_A::SHA256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == HASH_SELECT_A::SHA1
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == HASH_SELECT_A::CRC32
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == HASH_SELECT_A::SHA256
    }
}
#[doc = "Field `CIPHER_CFG` reader - Cipher configuration bits. Optional configuration bits are required for the ciphers."]
pub type CIPHER_CFG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Cipher selection field"]
    #[inline(always)]
    pub fn cipher_select(&self) -> CIPHER_SELECT_R {
        CIPHER_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Cipher mode selection field. Reflects the mode of operation for the cipher operations."]
    #[inline(always)]
    pub fn cipher_mode(&self) -> CIPHER_MODE_R {
        CIPHER_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Key selection field"]
    #[inline(always)]
    pub fn key_select(&self) -> KEY_SELECT_R {
        KEY_SELECT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Hash Selection Field"]
    #[inline(always)]
    pub fn hash_select(&self) -> HASH_SELECT_R {
        HASH_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Cipher configuration bits. Optional configuration bits are required for the ciphers."]
    #[inline(always)]
    pub fn cipher_cfg(&self) -> CIPHER_CFG_R {
        CIPHER_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DCP work packet 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet2](index.html) module"]
pub struct PACKET2_SPEC;
impl crate::RegisterSpec for PACKET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet2::R](R) reader structure"]
impl crate::Readable for PACKET2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKET2 to value 0"]
impl crate::Resettable for PACKET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
