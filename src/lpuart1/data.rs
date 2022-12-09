#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0T0` reader - R0T0"]
pub type R0T0_R = crate::BitReader<bool>;
#[doc = "Field `R0T0` writer - R0T0"]
pub type R0T0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R1T1` reader - R1T1"]
pub type R1T1_R = crate::BitReader<bool>;
#[doc = "Field `R1T1` writer - R1T1"]
pub type R1T1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R2T2` reader - R2T2"]
pub type R2T2_R = crate::BitReader<bool>;
#[doc = "Field `R2T2` writer - R2T2"]
pub type R2T2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R3T3` reader - R3T3"]
pub type R3T3_R = crate::BitReader<bool>;
#[doc = "Field `R3T3` writer - R3T3"]
pub type R3T3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R4T4` reader - R4T4"]
pub type R4T4_R = crate::BitReader<bool>;
#[doc = "Field `R4T4` writer - R4T4"]
pub type R4T4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R5T5` reader - R5T5"]
pub type R5T5_R = crate::BitReader<bool>;
#[doc = "Field `R5T5` writer - R5T5"]
pub type R5T5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R6T6` reader - R6T6"]
pub type R6T6_R = crate::BitReader<bool>;
#[doc = "Field `R6T6` writer - R6T6"]
pub type R6T6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R7T7` reader - R7T7"]
pub type R7T7_R = crate::BitReader<bool>;
#[doc = "Field `R7T7` writer - R7T7"]
pub type R7T7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R8T8` reader - R8T8"]
pub type R8T8_R = crate::BitReader<bool>;
#[doc = "Field `R8T8` writer - R8T8"]
pub type R8T8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `R9T9` reader - R9T9"]
pub type R9T9_R = crate::BitReader<bool>;
#[doc = "Field `R9T9` writer - R9T9"]
pub type R9T9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, bool, O>;
#[doc = "Field `IDLINE` reader - Idle Line"]
pub type IDLINE_R = crate::BitReader<IDLINE_A>;
#[doc = "Idle Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLINE_A {
    #[doc = "0: Receiver was not idle before receiving this character."]
    NO_IDLE = 0,
    #[doc = "1: Receiver was idle before receiving this character."]
    IDLE = 1,
}
impl From<IDLINE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLINE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLINE_A {
        match self.bits {
            false => IDLINE_A::NO_IDLE,
            true => IDLINE_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IDLE`"]
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLINE_A::NO_IDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLINE_A::IDLE
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer Empty"]
pub type RXEMPT_R = crate::BitReader<RXEMPT_A>;
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer contains valid data."]
    NOT_EMPTY = 0,
    #[doc = "1: Receive buffer is empty, data returned on read is not valid."]
    EMPTY = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::NOT_EMPTY,
            true => RXEMPT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXEMPT_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXEMPT_A::EMPTY
    }
}
#[doc = "Field `FRETSC` reader - Frame Error / Transmit Special Character"]
pub type FRETSC_R = crate::BitReader<FRETSC_A>;
#[doc = "Frame Error / Transmit Special Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRETSC_A {
    #[doc = "0: The dataword is received without a frame error on read, or transmit a normal character on write."]
    NO_ERROR = 0,
    #[doc = "1: The dataword is received with a frame error, or transmit an idle or break character on transmit."]
    ERROR = 1,
}
impl From<FRETSC_A> for bool {
    #[inline(always)]
    fn from(variant: FRETSC_A) -> Self {
        variant as u8 != 0
    }
}
impl FRETSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRETSC_A {
        match self.bits {
            false => FRETSC_A::NO_ERROR,
            true => FRETSC_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FRETSC_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FRETSC_A::ERROR
    }
}
#[doc = "Field `FRETSC` writer - Frame Error / Transmit Special Character"]
pub type FRETSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SPEC, FRETSC_A, O>;
impl<'a, const O: u8> FRETSC_W<'a, O> {
    #[doc = "The dataword is received without a frame error on read, or transmit a normal character on write."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(FRETSC_A::NO_ERROR)
    }
    #[doc = "The dataword is received with a frame error, or transmit an idle or break character on transmit."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(FRETSC_A::ERROR)
    }
}
#[doc = "Field `PARITYE` reader - Parity Error"]
pub type PARITYE_R = crate::BitReader<PARITYE_A>;
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITYE_A {
    #[doc = "0: The dataword is received without a parity error."]
    NO_PARITY = 0,
    #[doc = "1: The dataword is received with a parity error."]
    PARITY = 1,
}
impl From<PARITYE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYE_A {
        match self.bits {
            false => PARITYE_A::NO_PARITY,
            true => PARITYE_A::PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY`"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITYE_A::NO_PARITY
    }
    #[doc = "Checks if the value of the field is `PARITY`"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == PARITYE_A::PARITY
    }
}
#[doc = "Field `NOISY` reader - Noisy Data Received"]
pub type NOISY_R = crate::BitReader<NOISY_A>;
#[doc = "Noisy Data Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOISY_A {
    #[doc = "0: The dataword is received without noise."]
    NO_NOISE = 0,
    #[doc = "1: The data is received with noise."]
    NOISE = 1,
}
impl From<NOISY_A> for bool {
    #[inline(always)]
    fn from(variant: NOISY_A) -> Self {
        variant as u8 != 0
    }
}
impl NOISY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISY_A {
        match self.bits {
            false => NOISY_A::NO_NOISE,
            true => NOISY_A::NOISE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NOISE`"]
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NOISY_A::NO_NOISE
    }
    #[doc = "Checks if the value of the field is `NOISE`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NOISY_A::NOISE
    }
}
impl R {
    #[doc = "Bit 0 - R0T0"]
    #[inline(always)]
    pub fn r0t0(&self) -> R0T0_R {
        R0T0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline(always)]
    pub fn r1t1(&self) -> R1T1_R {
        R1T1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline(always)]
    pub fn r2t2(&self) -> R2T2_R {
        R2T2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline(always)]
    pub fn r3t3(&self) -> R3T3_R {
        R3T3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline(always)]
    pub fn r4t4(&self) -> R4T4_R {
        R4T4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline(always)]
    pub fn r5t5(&self) -> R5T5_R {
        R5T5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline(always)]
    pub fn r6t6(&self) -> R6T6_R {
        R6T6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline(always)]
    pub fn r7t7(&self) -> R7T7_R {
        R7T7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline(always)]
    pub fn r8t8(&self) -> R8T8_R {
        R8T8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline(always)]
    pub fn r9t9(&self) -> R9T9_R {
        R9T9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle Line"]
    #[inline(always)]
    pub fn idline(&self) -> IDLINE_R {
        IDLINE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&self) -> FRETSC_R {
        FRETSC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity Error"]
    #[inline(always)]
    pub fn paritye(&self) -> PARITYE_R {
        PARITYE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Noisy Data Received"]
    #[inline(always)]
    pub fn noisy(&self) -> NOISY_R {
        NOISY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - R0T0"]
    #[inline(always)]
    #[must_use]
    pub fn r0t0(&mut self) -> R0T0_W<0> {
        R0T0_W::new(self)
    }
    #[doc = "Bit 1 - R1T1"]
    #[inline(always)]
    #[must_use]
    pub fn r1t1(&mut self) -> R1T1_W<1> {
        R1T1_W::new(self)
    }
    #[doc = "Bit 2 - R2T2"]
    #[inline(always)]
    #[must_use]
    pub fn r2t2(&mut self) -> R2T2_W<2> {
        R2T2_W::new(self)
    }
    #[doc = "Bit 3 - R3T3"]
    #[inline(always)]
    #[must_use]
    pub fn r3t3(&mut self) -> R3T3_W<3> {
        R3T3_W::new(self)
    }
    #[doc = "Bit 4 - R4T4"]
    #[inline(always)]
    #[must_use]
    pub fn r4t4(&mut self) -> R4T4_W<4> {
        R4T4_W::new(self)
    }
    #[doc = "Bit 5 - R5T5"]
    #[inline(always)]
    #[must_use]
    pub fn r5t5(&mut self) -> R5T5_W<5> {
        R5T5_W::new(self)
    }
    #[doc = "Bit 6 - R6T6"]
    #[inline(always)]
    #[must_use]
    pub fn r6t6(&mut self) -> R6T6_W<6> {
        R6T6_W::new(self)
    }
    #[doc = "Bit 7 - R7T7"]
    #[inline(always)]
    #[must_use]
    pub fn r7t7(&mut self) -> R7T7_W<7> {
        R7T7_W::new(self)
    }
    #[doc = "Bit 8 - R8T8"]
    #[inline(always)]
    #[must_use]
    pub fn r8t8(&mut self) -> R8T8_W<8> {
        R8T8_W::new(self)
    }
    #[doc = "Bit 9 - R9T9"]
    #[inline(always)]
    #[must_use]
    pub fn r9t9(&mut self) -> R9T9_W<9> {
        R9T9_W::new(self)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    #[must_use]
    pub fn fretsc(&mut self) -> FRETSC_W<13> {
        FRETSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA to value 0x1000"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
