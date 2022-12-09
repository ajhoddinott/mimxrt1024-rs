#[doc = "Register `MRDR` reader"]
pub struct R(crate::R<MRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Receive Data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXEMPTY` reader - RX Empty"]
pub type RXEMPTY_R = crate::BitReader<RXEMPTY_A>;
#[doc = "RX Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPTY_A {
    #[doc = "0: Receive FIFO is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Receive FIFO is empty"]
    EMPTY = 1,
}
impl From<RXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPTY_A {
        match self.bits {
            false => RXEMPTY_A::NOT_EMPTY,
            true => RXEMPTY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXEMPTY_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXEMPTY_A::EMPTY
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - RX Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Master Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdr](index.html) module"]
pub struct MRDR_SPEC;
impl crate::RegisterSpec for MRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrdr::R](R) reader structure"]
impl crate::Readable for MRDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MRDR to value 0x4000"]
impl crate::Resettable for MRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
