#[doc = "Register `CTRL_SET` reader"]
pub struct R(crate::R<CTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_SET` writer"]
pub struct W(crate::W<CTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SET_SPEC>;
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
impl From<crate::W<CTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL_INTERRUPT_ENABLE` reader - Per-channel interrupt enable bit"]
pub type CHANNEL_INTERRUPT_ENABLE_R = crate::FieldReader<u8, CHANNEL_INTERRUPT_ENABLE_A>;
#[doc = "Per-channel interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL_INTERRUPT_ENABLE_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<CHANNEL_INTERRUPT_ENABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_INTERRUPT_ENABLE_A) -> Self {
        variant as _
    }
}
impl CHANNEL_INTERRUPT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNEL_INTERRUPT_ENABLE_A> {
        match self.bits {
            1 => Some(CHANNEL_INTERRUPT_ENABLE_A::CH0),
            2 => Some(CHANNEL_INTERRUPT_ENABLE_A::CH1),
            4 => Some(CHANNEL_INTERRUPT_ENABLE_A::CH2),
            8 => Some(CHANNEL_INTERRUPT_ENABLE_A::CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == CHANNEL_INTERRUPT_ENABLE_A::CH3
    }
}
#[doc = "Field `CHANNEL_INTERRUPT_ENABLE` writer - Per-channel interrupt enable bit"]
pub type CHANNEL_INTERRUPT_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SET_SPEC, u8, CHANNEL_INTERRUPT_ENABLE_A, 8, O>;
impl<'a, const O: u8> CHANNEL_INTERRUPT_ENABLE_W<'a, O> {
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(CHANNEL_INTERRUPT_ENABLE_A::CH3)
    }
}
#[doc = "Field `ENABLE_CONTEXT_SWITCHING` reader - Enable automatic context switching for the channels"]
pub type ENABLE_CONTEXT_SWITCHING_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_CONTEXT_SWITCHING` writer - Enable automatic context switching for the channels"]
pub type ENABLE_CONTEXT_SWITCHING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENABLE_CONTEXT_CACHING` reader - The software must set this bit to enable the caching of contexts between the operations"]
pub type ENABLE_CONTEXT_CACHING_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_CONTEXT_CACHING` writer - The software must set this bit to enable the caching of contexts between the operations"]
pub type ENABLE_CONTEXT_CACHING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `GATHER_RESIDUAL_WRITES` reader - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
pub type GATHER_RESIDUAL_WRITES_R = crate::BitReader<bool>;
#[doc = "Field `GATHER_RESIDUAL_WRITES` writer - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
pub type GATHER_RESIDUAL_WRITES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `PRESENT_SHA` reader - Indicates whether the SHA1/SHA2 functions are present."]
pub type PRESENT_SHA_R = crate::BitReader<PRESENT_SHA_A>;
#[doc = "Indicates whether the SHA1/SHA2 functions are present.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRESENT_SHA_A {
    #[doc = "0: Absent"]
    ABSENT = 0,
    #[doc = "1: Present"]
    PRESENT = 1,
}
impl From<PRESENT_SHA_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_SHA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRESENT_SHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_SHA_A {
        match self.bits {
            false => PRESENT_SHA_A::ABSENT,
            true => PRESENT_SHA_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_SHA_A::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_SHA_A::PRESENT
    }
}
#[doc = "Field `PRESENT_CRYPTO` reader - Indicates whether the crypto (cipher/hash) functions are present."]
pub type PRESENT_CRYPTO_R = crate::BitReader<PRESENT_CRYPTO_A>;
#[doc = "Indicates whether the crypto (cipher/hash) functions are present.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRESENT_CRYPTO_A {
    #[doc = "0: Absent"]
    ABSENT = 0,
    #[doc = "1: Present"]
    PRESENT = 1,
}
impl From<PRESENT_CRYPTO_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_CRYPTO_A) -> Self {
        variant as u8 != 0
    }
}
impl PRESENT_CRYPTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_CRYPTO_A {
        match self.bits {
            false => PRESENT_CRYPTO_A::ABSENT,
            true => PRESENT_CRYPTO_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT`"]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == PRESENT_CRYPTO_A::ABSENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PRESENT_CRYPTO_A::PRESENT
    }
}
#[doc = "Field `CLKGATE` reader - This bit must be set to zero for a normal operation"]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - This bit must be set to zero for a normal operation"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `SFTRST` reader - Set this bit to zero to enable a normal DCP operation"]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - Set this bit to zero to enable a normal DCP operation"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline(always)]
    pub fn channel_interrupt_enable(&self) -> CHANNEL_INTERRUPT_ENABLE_R {
        CHANNEL_INTERRUPT_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline(always)]
    pub fn enable_context_switching(&self) -> ENABLE_CONTEXT_SWITCHING_R {
        ENABLE_CONTEXT_SWITCHING_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    pub fn enable_context_caching(&self) -> ENABLE_CONTEXT_CACHING_R {
        ENABLE_CONTEXT_CACHING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    pub fn gather_residual_writes(&self) -> GATHER_RESIDUAL_WRITES_R {
        GATHER_RESIDUAL_WRITES_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates whether the SHA1/SHA2 functions are present."]
    #[inline(always)]
    pub fn present_sha(&self) -> PRESENT_SHA_R {
        PRESENT_SHA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates whether the crypto (cipher/hash) functions are present."]
    #[inline(always)]
    pub fn present_crypto(&self) -> PRESENT_CRYPTO_R {
        PRESENT_CRYPTO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Per-channel interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn channel_interrupt_enable(&mut self) -> CHANNEL_INTERRUPT_ENABLE_W<0> {
        CHANNEL_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 21 - Enable automatic context switching for the channels"]
    #[inline(always)]
    #[must_use]
    pub fn enable_context_switching(&mut self) -> ENABLE_CONTEXT_SWITCHING_W<21> {
        ENABLE_CONTEXT_SWITCHING_W::new(self)
    }
    #[doc = "Bit 22 - The software must set this bit to enable the caching of contexts between the operations"]
    #[inline(always)]
    #[must_use]
    pub fn enable_context_caching(&mut self) -> ENABLE_CONTEXT_CACHING_W<22> {
        ENABLE_CONTEXT_CACHING_W::new(self)
    }
    #[doc = "Bit 23 - The software must set this bit to enable the ragged writes to the unaligned buffers to be gathered between multiple write operations"]
    #[inline(always)]
    #[must_use]
    pub fn gather_residual_writes(&mut self) -> GATHER_RESIDUAL_WRITES_W<23> {
        GATHER_RESIDUAL_WRITES_W::new(self)
    }
    #[doc = "Bit 30 - This bit must be set to zero for a normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to zero to enable a normal DCP operation"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<31> {
        SFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](index.html) module"]
pub struct CTRL_SET_SPEC;
impl crate::RegisterSpec for CTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_set::R](R) reader structure"]
impl crate::Readable for CTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](W) writer structure"]
impl crate::Writable for CTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_SET to value 0xf080_0000"]
impl crate::Resettable for CTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0xf080_0000;
}
