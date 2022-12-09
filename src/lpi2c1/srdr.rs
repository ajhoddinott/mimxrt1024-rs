#[doc = "Register `SRDR` reader"]
pub struct R(crate::R<SRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDR_SPEC>) -> Self {
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
    #[doc = "0: The Receive Data Register is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: The Receive Data Register is empty"]
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
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SOF_R = crate::BitReader<SOF_A>;
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF_A {
    #[doc = "0: Indicates this is not the first data word since a (repeated) START or STOP condition"]
    NOT_FIRST_DATA_WORD = 0,
    #[doc = "1: Indicates this is the first data word since a (repeated) START or STOP condition"]
    FIRST_DATA_WORD = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::NOT_FIRST_DATA_WORD,
            true => SOF_A::FIRST_DATA_WORD,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FIRST_DATA_WORD`"]
    #[inline(always)]
    pub fn is_not_first_data_word(&self) -> bool {
        *self == SOF_A::NOT_FIRST_DATA_WORD
    }
    #[doc = "Checks if the value of the field is `FIRST_DATA_WORD`"]
    #[inline(always)]
    pub fn is_first_data_word(&self) -> bool {
        *self == SOF_A::FIRST_DATA_WORD
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
    #[doc = "Bit 15 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Slave Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdr](index.html) module"]
pub struct SRDR_SPEC;
impl crate::RegisterSpec for SRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdr::R](R) reader structure"]
impl crate::Readable for SRDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRDR to value 0x4000"]
impl crate::Resettable for SRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
