#[doc = "Register `RSR` reader"]
pub struct R(crate::R<RSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SOF_R = crate::BitReader<SOF_A>;
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF_A {
    #[doc = "0: Subsequent data word received after PCS assertion"]
    NEXT_DATAWORD = 0,
    #[doc = "1: First data word received after PCS assertion"]
    FIRST_DATAWORD = 1,
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
            false => SOF_A::NEXT_DATAWORD,
            true => SOF_A::FIRST_DATAWORD,
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_DATAWORD`"]
    #[inline(always)]
    pub fn is_next_dataword(&self) -> bool {
        *self == SOF_A::NEXT_DATAWORD
    }
    #[doc = "Checks if the value of the field is `FIRST_DATAWORD`"]
    #[inline(always)]
    pub fn is_first_dataword(&self) -> bool {
        *self == SOF_A::FIRST_DATAWORD
    }
}
#[doc = "Field `RXEMPTY` reader - RX FIFO Empty"]
pub type RXEMPTY_R = crate::BitReader<RXEMPTY_A>;
#[doc = "RX FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPTY_A {
    #[doc = "0: RX FIFO is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: RX FIFO is empty"]
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
    #[doc = "Bit 0 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Empty"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Receive Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](index.html) module"]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsr::R](R) reader structure"]
impl crate::Readable for RSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSR to value 0x02"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
