#[doc = "Register `FIFO` reader"]
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO` writer"]
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFOSIZE` reader - Receive FIFO Buffer Depth"]
pub type RXFIFOSIZE_R = crate::FieldReader<u8, RXFIFOSIZE_A>;
#[doc = "Receive FIFO Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFIFOSIZE_A {
    #[doc = "0: Receive FIFO/Buffer depth = 1 dataword."]
    FIFO_1 = 0,
    #[doc = "1: Receive FIFO/Buffer depth = 4 datawords."]
    FIFO_4 = 1,
    #[doc = "2: Receive FIFO/Buffer depth = 8 datawords."]
    FIFO_8 = 2,
    #[doc = "3: Receive FIFO/Buffer depth = 16 datawords."]
    FIFO_16 = 3,
    #[doc = "4: Receive FIFO/Buffer depth = 32 datawords."]
    FIFO_32 = 4,
    #[doc = "5: Receive FIFO/Buffer depth = 64 datawords."]
    FIFO_64 = 5,
    #[doc = "6: Receive FIFO/Buffer depth = 128 datawords."]
    FIFO_128 = 6,
    #[doc = "7: Receive FIFO/Buffer depth = 256 datawords."]
    FIFO_256 = 7,
}
impl From<RXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOSIZE_A) -> Self {
        variant as _
    }
}
impl RXFIFOSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOSIZE_A {
        match self.bits {
            0 => RXFIFOSIZE_A::FIFO_1,
            1 => RXFIFOSIZE_A::FIFO_4,
            2 => RXFIFOSIZE_A::FIFO_8,
            3 => RXFIFOSIZE_A::FIFO_16,
            4 => RXFIFOSIZE_A::FIFO_32,
            5 => RXFIFOSIZE_A::FIFO_64,
            6 => RXFIFOSIZE_A::FIFO_128,
            7 => RXFIFOSIZE_A::FIFO_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_1`"]
    #[inline(always)]
    pub fn is_fifo_1(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_1
    }
    #[doc = "Checks if the value of the field is `FIFO_4`"]
    #[inline(always)]
    pub fn is_fifo_4(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_4
    }
    #[doc = "Checks if the value of the field is `FIFO_8`"]
    #[inline(always)]
    pub fn is_fifo_8(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_8
    }
    #[doc = "Checks if the value of the field is `FIFO_16`"]
    #[inline(always)]
    pub fn is_fifo_16(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_16
    }
    #[doc = "Checks if the value of the field is `FIFO_32`"]
    #[inline(always)]
    pub fn is_fifo_32(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_32
    }
    #[doc = "Checks if the value of the field is `FIFO_64`"]
    #[inline(always)]
    pub fn is_fifo_64(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_64
    }
    #[doc = "Checks if the value of the field is `FIFO_128`"]
    #[inline(always)]
    pub fn is_fifo_128(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_128
    }
    #[doc = "Checks if the value of the field is `FIFO_256`"]
    #[inline(always)]
    pub fn is_fifo_256(&self) -> bool {
        *self == RXFIFOSIZE_A::FIFO_256
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO Enable"]
pub type RXFE_R = crate::BitReader<RXFE_A>;
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFE_A {
    #[doc = "0: Receive FIFO is not enabled. Buffer depth is 1."]
    DISABLED = 0,
    #[doc = "1: Receive FIFO is enabled. Buffer depth is indicted by RXFIFOSIZE."]
    ENABLED = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::DISABLED,
            true => RXFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFE_A::ENABLED
    }
}
#[doc = "Field `RXFE` writer - Receive FIFO Enable"]
pub type RXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, RXFE_A, O>;
impl<'a, const O: u8> RXFE_W<'a, O> {
    #[doc = "Receive FIFO is not enabled. Buffer depth is 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXFE_A::DISABLED)
    }
    #[doc = "Receive FIFO is enabled. Buffer depth is indicted by RXFIFOSIZE."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXFE_A::ENABLED)
    }
}
#[doc = "Field `TXFIFOSIZE` reader - Transmit FIFO Buffer Depth"]
pub type TXFIFOSIZE_R = crate::FieldReader<u8, TXFIFOSIZE_A>;
#[doc = "Transmit FIFO Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFIFOSIZE_A {
    #[doc = "0: Transmit FIFO/Buffer depth = 1 dataword."]
    FIFO_1 = 0,
    #[doc = "1: Transmit FIFO/Buffer depth = 4 datawords."]
    FIFO_4 = 1,
    #[doc = "2: Transmit FIFO/Buffer depth = 8 datawords."]
    FIFO_8 = 2,
    #[doc = "3: Transmit FIFO/Buffer depth = 16 datawords."]
    FIFO_16 = 3,
    #[doc = "4: Transmit FIFO/Buffer depth = 32 datawords."]
    FIFO_32 = 4,
    #[doc = "5: Transmit FIFO/Buffer depth = 64 datawords."]
    FIFO_64 = 5,
    #[doc = "6: Transmit FIFO/Buffer depth = 128 datawords."]
    FIFO_128 = 6,
    #[doc = "7: Transmit FIFO/Buffer depth = 256 datawords"]
    FIFO_256 = 7,
}
impl From<TXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOSIZE_A) -> Self {
        variant as _
    }
}
impl TXFIFOSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOSIZE_A {
        match self.bits {
            0 => TXFIFOSIZE_A::FIFO_1,
            1 => TXFIFOSIZE_A::FIFO_4,
            2 => TXFIFOSIZE_A::FIFO_8,
            3 => TXFIFOSIZE_A::FIFO_16,
            4 => TXFIFOSIZE_A::FIFO_32,
            5 => TXFIFOSIZE_A::FIFO_64,
            6 => TXFIFOSIZE_A::FIFO_128,
            7 => TXFIFOSIZE_A::FIFO_256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_1`"]
    #[inline(always)]
    pub fn is_fifo_1(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_1
    }
    #[doc = "Checks if the value of the field is `FIFO_4`"]
    #[inline(always)]
    pub fn is_fifo_4(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_4
    }
    #[doc = "Checks if the value of the field is `FIFO_8`"]
    #[inline(always)]
    pub fn is_fifo_8(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_8
    }
    #[doc = "Checks if the value of the field is `FIFO_16`"]
    #[inline(always)]
    pub fn is_fifo_16(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_16
    }
    #[doc = "Checks if the value of the field is `FIFO_32`"]
    #[inline(always)]
    pub fn is_fifo_32(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_32
    }
    #[doc = "Checks if the value of the field is `FIFO_64`"]
    #[inline(always)]
    pub fn is_fifo_64(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_64
    }
    #[doc = "Checks if the value of the field is `FIFO_128`"]
    #[inline(always)]
    pub fn is_fifo_128(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_128
    }
    #[doc = "Checks if the value of the field is `FIFO_256`"]
    #[inline(always)]
    pub fn is_fifo_256(&self) -> bool {
        *self == TXFIFOSIZE_A::FIFO_256
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO Enable"]
pub type TXFE_R = crate::BitReader<TXFE_A>;
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE_A {
    #[doc = "0: Transmit FIFO is not enabled. Buffer depth is 1."]
    DISABLED = 0,
    #[doc = "1: Transmit FIFO is enabled. Buffer depth is indicated by TXFIFOSIZE."]
    ENABLED = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::DISABLED,
            true => TXFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFE_A::ENABLED
    }
}
#[doc = "Field `TXFE` writer - Transmit FIFO Enable"]
pub type TXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, TXFE_A, O>;
impl<'a, const O: u8> TXFE_W<'a, O> {
    #[doc = "Transmit FIFO is not enabled. Buffer depth is 1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFE_A::DISABLED)
    }
    #[doc = "Transmit FIFO is enabled. Buffer depth is indicated by TXFIFOSIZE."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFE_A::ENABLED)
    }
}
#[doc = "Field `RXUFE` reader - Receive FIFO Underflow Interrupt Enable"]
pub type RXUFE_R = crate::BitReader<RXUFE_A>;
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUFE_A {
    #[doc = "0: RXUF flag does not generate an interrupt to the host."]
    DISABLED = 0,
    #[doc = "1: RXUF flag generates an interrupt to the host."]
    ENABLED = 1,
}
impl From<RXUFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXUFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXUFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUFE_A {
        match self.bits {
            false => RXUFE_A::DISABLED,
            true => RXUFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXUFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXUFE_A::ENABLED
    }
}
#[doc = "Field `RXUFE` writer - Receive FIFO Underflow Interrupt Enable"]
pub type RXUFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, RXUFE_A, O>;
impl<'a, const O: u8> RXUFE_W<'a, O> {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXUFE_A::DISABLED)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXUFE_A::ENABLED)
    }
}
#[doc = "Field `TXOFE` reader - Transmit FIFO Overflow Interrupt Enable"]
pub type TXOFE_R = crate::BitReader<TXOFE_A>;
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOFE_A {
    #[doc = "0: TXOF flag does not generate an interrupt to the host."]
    DISABLED = 0,
    #[doc = "1: TXOF flag generates an interrupt to the host."]
    ENABLED = 1,
}
impl From<TXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFE_A {
        match self.bits {
            false => TXOFE_A::DISABLED,
            true => TXOFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXOFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXOFE_A::ENABLED
    }
}
#[doc = "Field `TXOFE` writer - Transmit FIFO Overflow Interrupt Enable"]
pub type TXOFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, TXOFE_A, O>;
impl<'a, const O: u8> TXOFE_W<'a, O> {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXOFE_A::DISABLED)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXOFE_A::ENABLED)
    }
}
#[doc = "Field `RXIDEN` reader - Receiver Idle Empty Enable"]
pub type RXIDEN_R = crate::FieldReader<u8, RXIDEN_A>;
#[doc = "Receiver Idle Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXIDEN_A {
    #[doc = "0: Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    DISABLED = 0,
    #[doc = "1: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    IDLE_1 = 1,
    #[doc = "2: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    IDLE_2 = 2,
    #[doc = "3: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    IDLE_4 = 3,
    #[doc = "4: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    IDLE_8 = 4,
    #[doc = "5: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    IDLE_16 = 5,
    #[doc = "6: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    IDLE_32 = 6,
    #[doc = "7: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    IDLE_64 = 7,
}
impl From<RXIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIDEN_A) -> Self {
        variant as _
    }
}
impl RXIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDEN_A {
        match self.bits {
            0 => RXIDEN_A::DISABLED,
            1 => RXIDEN_A::IDLE_1,
            2 => RXIDEN_A::IDLE_2,
            3 => RXIDEN_A::IDLE_4,
            4 => RXIDEN_A::IDLE_8,
            5 => RXIDEN_A::IDLE_16,
            6 => RXIDEN_A::IDLE_32,
            7 => RXIDEN_A::IDLE_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXIDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline(always)]
    pub fn is_idle_1(&self) -> bool {
        *self == RXIDEN_A::IDLE_1
    }
    #[doc = "Checks if the value of the field is `IDLE_2`"]
    #[inline(always)]
    pub fn is_idle_2(&self) -> bool {
        *self == RXIDEN_A::IDLE_2
    }
    #[doc = "Checks if the value of the field is `IDLE_4`"]
    #[inline(always)]
    pub fn is_idle_4(&self) -> bool {
        *self == RXIDEN_A::IDLE_4
    }
    #[doc = "Checks if the value of the field is `IDLE_8`"]
    #[inline(always)]
    pub fn is_idle_8(&self) -> bool {
        *self == RXIDEN_A::IDLE_8
    }
    #[doc = "Checks if the value of the field is `IDLE_16`"]
    #[inline(always)]
    pub fn is_idle_16(&self) -> bool {
        *self == RXIDEN_A::IDLE_16
    }
    #[doc = "Checks if the value of the field is `IDLE_32`"]
    #[inline(always)]
    pub fn is_idle_32(&self) -> bool {
        *self == RXIDEN_A::IDLE_32
    }
    #[doc = "Checks if the value of the field is `IDLE_64`"]
    #[inline(always)]
    pub fn is_idle_64(&self) -> bool {
        *self == RXIDEN_A::IDLE_64
    }
}
#[doc = "Field `RXIDEN` writer - Receiver Idle Empty Enable"]
pub type RXIDEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FIFO_SPEC, u8, RXIDEN_A, 3, O>;
impl<'a, const O: u8> RXIDEN_W<'a, O> {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIDEN_A::DISABLED)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    #[inline(always)]
    pub fn idle_1(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_1)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    #[inline(always)]
    pub fn idle_2(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_2)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    #[inline(always)]
    pub fn idle_4(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_4)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    #[inline(always)]
    pub fn idle_8(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_8)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    #[inline(always)]
    pub fn idle_16(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_16)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    #[inline(always)]
    pub fn idle_32(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_32)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    #[inline(always)]
    pub fn idle_64(self) -> &'a mut W {
        self.variant(RXIDEN_A::IDLE_64)
    }
}
#[doc = "Field `RXFLUSH` reader - Receive FIFO Flush"]
pub type RXFLUSH_R = crate::BitReader<RXFLUSH_A>;
#[doc = "Receive FIFO Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFLUSH_A {
    #[doc = "0: No flush operation occurs."]
    NO_EFFECT = 0,
    #[doc = "1: All data in the receive FIFO/buffer is cleared out."]
    RXFIFO_RST = 1,
}
impl From<RXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFLUSH_A {
        match self.bits {
            false => RXFLUSH_A::NO_EFFECT,
            true => RXFLUSH_A::RXFIFO_RST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RXFLUSH_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RXFIFO_RST`"]
    #[inline(always)]
    pub fn is_rxfifo_rst(&self) -> bool {
        *self == RXFLUSH_A::RXFIFO_RST
    }
}
#[doc = "Field `RXFLUSH` writer - Receive FIFO Flush"]
pub type RXFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, RXFLUSH_A, O>;
impl<'a, const O: u8> RXFLUSH_W<'a, O> {
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RXFLUSH_A::NO_EFFECT)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline(always)]
    pub fn rxfifo_rst(self) -> &'a mut W {
        self.variant(RXFLUSH_A::RXFIFO_RST)
    }
}
#[doc = "Field `TXFLUSH` reader - Transmit FIFO Flush"]
pub type TXFLUSH_R = crate::BitReader<TXFLUSH_A>;
#[doc = "Transmit FIFO Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFLUSH_A {
    #[doc = "0: No flush operation occurs."]
    NO_EFFECT = 0,
    #[doc = "1: All data in the transmit FIFO is cleared out."]
    TXFIFO_RST = 1,
}
impl From<TXFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFLUSH_A {
        match self.bits {
            false => TXFLUSH_A::NO_EFFECT,
            true => TXFLUSH_A::TXFIFO_RST,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXFLUSH_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `TXFIFO_RST`"]
    #[inline(always)]
    pub fn is_txfifo_rst(&self) -> bool {
        *self == TXFLUSH_A::TXFIFO_RST
    }
}
#[doc = "Field `TXFLUSH` writer - Transmit FIFO Flush"]
pub type TXFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_SPEC, TXFLUSH_A, O>;
impl<'a, const O: u8> TXFLUSH_W<'a, O> {
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TXFLUSH_A::NO_EFFECT)
    }
    #[doc = "All data in the transmit FIFO is cleared out."]
    #[inline(always)]
    pub fn txfifo_rst(self) -> &'a mut W {
        self.variant(TXFLUSH_A::TXFIFO_RST)
    }
}
#[doc = "Field `RXUF` reader - Receiver FIFO Underflow Flag"]
pub type RXUF_R = crate::BitReader<RXUF_A>;
#[doc = "Receiver FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXUF_A {
    #[doc = "0: No receive FIFO underflow has occurred since the last time the flag was cleared."]
    NO_UNDERFLOW = 0,
    #[doc = "1: At least one receive FIFO underflow has occurred since the last time the flag was cleared."]
    UNDERFLOW = 1,
}
impl From<RXUF_A> for bool {
    #[inline(always)]
    fn from(variant: RXUF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUF_A {
        match self.bits {
            false => RXUF_A::NO_UNDERFLOW,
            true => RXUF_A::UNDERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NO_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_no_underflow(&self) -> bool {
        *self == RXUF_A::NO_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == RXUF_A::UNDERFLOW
    }
}
#[doc = "Field `RXUF` writer - Receiver FIFO Underflow Flag"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_SPEC, RXUF_A, O>;
impl<'a, const O: u8> RXUF_W<'a, O> {
    #[doc = "No receive FIFO underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn no_underflow(self) -> &'a mut W {
        self.variant(RXUF_A::NO_UNDERFLOW)
    }
    #[doc = "At least one receive FIFO underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(RXUF_A::UNDERFLOW)
    }
}
#[doc = "Field `TXOF` reader - Transmitter FIFO Overflow Flag"]
pub type TXOF_R = crate::BitReader<TXOF_A>;
#[doc = "Transmitter FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXOF_A {
    #[doc = "0: No transmit FIFO overflow has occurred since the last time the flag was cleared."]
    NO_OVERFLOW = 0,
    #[doc = "1: At least one transmit FIFO overflow has occurred since the last time the flag was cleared."]
    OVERFLOW = 1,
}
impl From<TXOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOF_A {
        match self.bits {
            false => TXOF_A::NO_OVERFLOW,
            true => TXOF_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == TXOF_A::NO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TXOF_A::OVERFLOW
    }
}
#[doc = "Field `TXOF` writer - Transmitter FIFO Overflow Flag"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FIFO_SPEC, TXOF_A, O>;
impl<'a, const O: u8> TXOF_W<'a, O> {
    #[doc = "No transmit FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut W {
        self.variant(TXOF_A::NO_OVERFLOW)
    }
    #[doc = "At least one transmit FIFO overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut W {
        self.variant(TXOF_A::OVERFLOW)
    }
}
#[doc = "Field `RXEMPT` reader - Receive FIFO/Buffer Empty"]
pub type RXEMPT_R = crate::BitReader<RXEMPT_A>;
#[doc = "Receive FIFO/Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer is not empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Receive buffer is empty."]
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
#[doc = "Field `TXEMPT` reader - Transmit FIFO/Buffer Empty"]
pub type TXEMPT_R = crate::BitReader<TXEMPT_A>;
#[doc = "Transmit FIFO/Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPT_A {
    #[doc = "0: Transmit buffer is not empty."]
    NOT_EMPTY = 0,
    #[doc = "1: Transmit buffer is empty."]
    EMPTY = 1,
}
impl From<TXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPT_A {
        match self.bits {
            false => TXEMPT_A::NOT_EMPTY,
            true => TXEMPT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXEMPT_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXEMPT_A::EMPTY
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Buffer Depth"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO Buffer Depth"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RXUFE_R {
        RXUFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TXOFE_R {
        TXOFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    pub fn rxiden(&self) -> RXIDEN_R {
        RXIDEN_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Receive FIFO Flush"]
    #[inline(always)]
    pub fn rxflush(&self) -> RXFLUSH_R {
        RXFLUSH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit FIFO Flush"]
    #[inline(always)]
    pub fn txflush(&self) -> TXFLUSH_R {
        TXFLUSH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Receiver FIFO Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmitter FIFO Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive FIFO/Buffer Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit FIFO/Buffer Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RXFE_W<3> {
        RXFE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TXFE_W<7> {
        TXFE_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxufe(&mut self) -> RXUFE_W<8> {
        RXUFE_W::new(self)
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txofe(&mut self) -> TXOFE_W<9> {
        TXOFE_W::new(self)
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxiden(&mut self) -> RXIDEN_W<10> {
        RXIDEN_W::new(self)
    }
    #[doc = "Bit 14 - Receive FIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxflush(&mut self) -> RXFLUSH_W<14> {
        RXFLUSH_W::new(self)
    }
    #[doc = "Bit 15 - Transmit FIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn txflush(&mut self) -> TXFLUSH_W<15> {
        TXFLUSH_W::new(self)
    }
    #[doc = "Bit 16 - Receiver FIFO Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<16> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 17 - Transmitter FIFO Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<17> {
        TXOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](index.html) module"]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo::R](R) reader structure"]
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo::W](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0000;
}
#[doc = "`reset()` method sets FIFO to value 0x00c0_0011"]
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c0_0011;
}
