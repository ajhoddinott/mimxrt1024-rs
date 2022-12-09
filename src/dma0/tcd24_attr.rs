#[doc = "Register `TCD24_ATTR` reader"]
pub struct R(crate::R<TCD24_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD24_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD24_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD24_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD24_ATTR` writer"]
pub struct W(crate::W<TCD24_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD24_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TCD24_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD24_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - Destination data transfer size"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Destination data transfer size"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD24_ATTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMOD` reader - Destination Address Modulo"]
pub type DMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMOD` writer - Destination Address Modulo"]
pub type DMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD24_ATTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSIZE` reader - Source data transfer size"]
pub type SSIZE_R = crate::FieldReader<u8, SSIZE_A>;
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 8-bit"]
    EIGHT = 0,
    #[doc = "1: 16-bit"]
    SIXTEEN_BIT = 1,
    #[doc = "2: 32-bit"]
    THIRTYTWO_BIT = 2,
    #[doc = "3: 64-bit"]
    SIXTYFOUR = 3,
    #[doc = "5: 32-byte burst (4 beats of 64 bits)"]
    THIRTYTWO_BYTE = 5,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
impl SSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSIZE_A> {
        match self.bits {
            0 => Some(SSIZE_A::EIGHT),
            1 => Some(SSIZE_A::SIXTEEN_BIT),
            2 => Some(SSIZE_A::THIRTYTWO_BIT),
            3 => Some(SSIZE_A::SIXTYFOUR),
            5 => Some(SSIZE_A::THIRTYTWO_BYTE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == SSIZE_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == SSIZE_A::SIXTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `THIRTYTWO_BIT`"]
    #[inline(always)]
    pub fn is_thirtytwo_bit(&self) -> bool {
        *self == SSIZE_A::THIRTYTWO_BIT
    }
    #[doc = "Checks if the value of the field is `SIXTYFOUR`"]
    #[inline(always)]
    pub fn is_sixtyfour(&self) -> bool {
        *self == SSIZE_A::SIXTYFOUR
    }
    #[doc = "Checks if the value of the field is `THIRTYTWO_BYTE`"]
    #[inline(always)]
    pub fn is_thirtytwo_byte(&self) -> bool {
        *self == SSIZE_A::THIRTYTWO_BYTE
    }
}
#[doc = "Field `SSIZE` writer - Source data transfer size"]
pub type SSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD24_ATTR_SPEC, u8, SSIZE_A, 3, O>;
impl<'a, const O: u8> SSIZE_W<'a, O> {
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(SSIZE_A::EIGHT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(SSIZE_A::SIXTEEN_BIT)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn thirtytwo_bit(self) -> &'a mut W {
        self.variant(SSIZE_A::THIRTYTWO_BIT)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn sixtyfour(self) -> &'a mut W {
        self.variant(SSIZE_A::SIXTYFOUR)
    }
    #[doc = "32-byte burst (4 beats of 64 bits)"]
    #[inline(always)]
    pub fn thirtytwo_byte(self) -> &'a mut W {
        self.variant(SSIZE_A::THIRTYTWO_BYTE)
    }
}
#[doc = "Field `SMOD` reader - Source Address Modulo"]
pub type SMOD_R = crate::FieldReader<u8, SMOD_A>;
#[doc = "Source Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Source address modulo feature is disabled"]
    DISABLED = 0,
    #[doc = "1: Value defines address range used to set up circular data queue"]
    ENABLED = 1,
    #[doc = "2: Value defines address range used to set up circular data queue"]
    ENABLED_2 = 2,
    #[doc = "3: Value defines address range used to set up circular data queue"]
    ENABLED_3 = 3,
    #[doc = "4: Value defines address range used to set up circular data queue"]
    ENABLED_4 = 4,
    #[doc = "5: Value defines address range used to set up circular data queue"]
    ENABLED_5 = 5,
    #[doc = "6: Value defines address range used to set up circular data queue"]
    ENABLED_6 = 6,
    #[doc = "7: Value defines address range used to set up circular data queue"]
    ENABLED_7 = 7,
    #[doc = "8: Value defines address range used to set up circular data queue"]
    ENABLED_8 = 8,
    #[doc = "9: Value defines address range used to set up circular data queue"]
    ENABLED_9 = 9,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
impl SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::DISABLED),
            1 => Some(SMOD_A::ENABLED),
            2 => Some(SMOD_A::ENABLED_2),
            3 => Some(SMOD_A::ENABLED_3),
            4 => Some(SMOD_A::ENABLED_4),
            5 => Some(SMOD_A::ENABLED_5),
            6 => Some(SMOD_A::ENABLED_6),
            7 => Some(SMOD_A::ENABLED_7),
            8 => Some(SMOD_A::ENABLED_8),
            9 => Some(SMOD_A::ENABLED_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMOD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED_2`"]
    #[inline(always)]
    pub fn is_enabled_2(&self) -> bool {
        *self == SMOD_A::ENABLED_2
    }
    #[doc = "Checks if the value of the field is `ENABLED_3`"]
    #[inline(always)]
    pub fn is_enabled_3(&self) -> bool {
        *self == SMOD_A::ENABLED_3
    }
    #[doc = "Checks if the value of the field is `ENABLED_4`"]
    #[inline(always)]
    pub fn is_enabled_4(&self) -> bool {
        *self == SMOD_A::ENABLED_4
    }
    #[doc = "Checks if the value of the field is `ENABLED_5`"]
    #[inline(always)]
    pub fn is_enabled_5(&self) -> bool {
        *self == SMOD_A::ENABLED_5
    }
    #[doc = "Checks if the value of the field is `ENABLED_6`"]
    #[inline(always)]
    pub fn is_enabled_6(&self) -> bool {
        *self == SMOD_A::ENABLED_6
    }
    #[doc = "Checks if the value of the field is `ENABLED_7`"]
    #[inline(always)]
    pub fn is_enabled_7(&self) -> bool {
        *self == SMOD_A::ENABLED_7
    }
    #[doc = "Checks if the value of the field is `ENABLED_8`"]
    #[inline(always)]
    pub fn is_enabled_8(&self) -> bool {
        *self == SMOD_A::ENABLED_8
    }
    #[doc = "Checks if the value of the field is `ENABLED_9`"]
    #[inline(always)]
    pub fn is_enabled_9(&self) -> bool {
        *self == SMOD_A::ENABLED_9
    }
}
#[doc = "Field `SMOD` writer - Source Address Modulo"]
pub type SMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD24_ATTR_SPEC, u8, SMOD_A, 5, O>;
impl<'a, const O: u8> SMOD_W<'a, O> {
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMOD_A::DISABLED)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_2(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_2)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_3(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_3)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_4(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_4)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_5(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_5)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_6(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_6)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_7(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_7)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_8(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_8)
    }
    #[doc = "Value defines address range used to set up circular data queue"]
    #[inline(always)]
    pub fn enabled_9(self) -> &'a mut W {
        self.variant(SMOD_A::ENABLED_9)
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn dmod(&mut self) -> DMOD_W<3> {
        DMOD_W::new(self)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<8> {
        SSIZE_W::new(self)
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SMOD_W<11> {
        SMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd24_attr](index.html) module"]
pub struct TCD24_ATTR_SPEC;
impl crate::RegisterSpec for TCD24_ATTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd24_attr::R](R) reader structure"]
impl crate::Readable for TCD24_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd24_attr::W](W) writer structure"]
impl crate::Writable for TCD24_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD24_ATTR to value 0"]
impl crate::Resettable for TCD24_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
