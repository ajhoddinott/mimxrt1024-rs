#[doc = "Register `MODIR` reader"]
pub struct R(crate::R<MODIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODIR` writer"]
pub struct W(crate::W<MODIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODIR_SPEC>;
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
impl From<crate::W<MODIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCTSE` reader - Transmitter clear-to-send enable"]
pub type TXCTSE_R = crate::BitReader<TXCTSE_A>;
#[doc = "Transmitter clear-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCTSE_A {
    #[doc = "0: CTS has no effect on the transmitter."]
    DISABLED = 0,
    #[doc = "1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    ENABLED = 1,
}
impl From<TXCTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSE_A {
        match self.bits {
            false => TXCTSE_A::DISABLED,
            true => TXCTSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXCTSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXCTSE_A::ENABLED
    }
}
#[doc = "Field `TXCTSE` writer - Transmitter clear-to-send enable"]
pub type TXCTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, TXCTSE_A, O>;
impl<'a, const O: u8> TXCTSE_W<'a, O> {
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXCTSE_A::DISABLED)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXCTSE_A::ENABLED)
    }
}
#[doc = "Field `TXRTSE` reader - Transmitter request-to-send enable"]
pub type TXRTSE_R = crate::BitReader<TXRTSE_A>;
#[doc = "Transmitter request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRTSE_A {
    #[doc = "0: The transmitter has no effect on RTS."]
    DISABLED = 0,
    #[doc = "1: When a character is placed into an empty transmit shift register, RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter FIFO and shift register are completely sent, including the last stop bit."]
    ENABLED = 1,
}
impl From<TXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSE_A {
        match self.bits {
            false => TXRTSE_A::DISABLED,
            true => TXRTSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXRTSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXRTSE_A::ENABLED
    }
}
#[doc = "Field `TXRTSE` writer - Transmitter request-to-send enable"]
pub type TXRTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, TXRTSE_A, O>;
impl<'a, const O: u8> TXRTSE_W<'a, O> {
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXRTSE_A::DISABLED)
    }
    #[doc = "When a character is placed into an empty transmit shift register, RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter FIFO and shift register are completely sent, including the last stop bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXRTSE_A::ENABLED)
    }
}
#[doc = "Field `TXRTSPOL` reader - Transmitter request-to-send polarity"]
pub type TXRTSPOL_R = crate::BitReader<TXRTSPOL_A>;
#[doc = "Transmitter request-to-send polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRTSPOL_A {
    #[doc = "0: Transmitter RTS is active low."]
    LOW = 0,
    #[doc = "1: Transmitter RTS is active high."]
    HIGH = 1,
}
impl From<TXRTSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRTSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSPOL_A {
        match self.bits {
            false => TXRTSPOL_A::LOW,
            true => TXRTSPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TXRTSPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TXRTSPOL_A::HIGH
    }
}
#[doc = "Field `TXRTSPOL` writer - Transmitter request-to-send polarity"]
pub type TXRTSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, TXRTSPOL_A, O>;
impl<'a, const O: u8> TXRTSPOL_W<'a, O> {
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::LOW)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::HIGH)
    }
}
#[doc = "Field `RXRTSE` reader - Receiver request-to-send enable"]
pub type RXRTSE_R = crate::BitReader<RXRTSE_A>;
#[doc = "Receiver request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRTSE_A {
    #[doc = "0: The receiver has no effect on RTS."]
    DISABLED = 0,
    #[doc = "1: RTS is deasserted if the receiver data register is full or a start bit has been detected that would cause the receiver data register to become full. RTS is asserted if the receiver data register is not full and has not detected a start bit that would cause the receiver data register to become full."]
    ENABLED = 1,
}
impl From<RXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRTSE_A {
        match self.bits {
            false => RXRTSE_A::DISABLED,
            true => RXRTSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXRTSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXRTSE_A::ENABLED
    }
}
#[doc = "Field `RXRTSE` writer - Receiver request-to-send enable"]
pub type RXRTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, RXRTSE_A, O>;
impl<'a, const O: u8> RXRTSE_W<'a, O> {
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXRTSE_A::DISABLED)
    }
    #[doc = "RTS is deasserted if the receiver data register is full or a start bit has been detected that would cause the receiver data register to become full. RTS is asserted if the receiver data register is not full and has not detected a start bit that would cause the receiver data register to become full."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXRTSE_A::ENABLED)
    }
}
#[doc = "Field `TXCTSC` reader - Transmit CTS Configuration"]
pub type TXCTSC_R = crate::BitReader<TXCTSC_A>;
#[doc = "Transmit CTS Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCTSC_A {
    #[doc = "0: CTS input is sampled at the start of each character."]
    START = 0,
    #[doc = "1: CTS input is sampled when the transmitter is idle."]
    IDLE = 1,
}
impl From<TXCTSC_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSC_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCTSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSC_A {
        match self.bits {
            false => TXCTSC_A::START,
            true => TXCTSC_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == TXCTSC_A::START
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TXCTSC_A::IDLE
    }
}
#[doc = "Field `TXCTSC` writer - Transmit CTS Configuration"]
pub type TXCTSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, TXCTSC_A, O>;
impl<'a, const O: u8> TXCTSC_W<'a, O> {
    #[doc = "CTS input is sampled at the start of each character."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TXCTSC_A::START)
    }
    #[doc = "CTS input is sampled when the transmitter is idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(TXCTSC_A::IDLE)
    }
}
#[doc = "Field `TXCTSSRC` reader - Transmit CTS Source"]
pub type TXCTSSRC_R = crate::BitReader<TXCTSSRC_A>;
#[doc = "Transmit CTS Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCTSSRC_A {
    #[doc = "0: CTS input is the CTS_B pin."]
    CTS = 0,
    #[doc = "1: CTS input is an internal connection to the receiver address match result."]
    MATCH = 1,
}
impl From<TXCTSSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCTSSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSSRC_A {
        match self.bits {
            false => TXCTSSRC_A::CTS,
            true => TXCTSSRC_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `CTS`"]
    #[inline(always)]
    pub fn is_cts(&self) -> bool {
        *self == TXCTSSRC_A::CTS
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == TXCTSSRC_A::MATCH
    }
}
#[doc = "Field `TXCTSSRC` writer - Transmit CTS Source"]
pub type TXCTSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, TXCTSSRC_A, O>;
impl<'a, const O: u8> TXCTSSRC_W<'a, O> {
    #[doc = "CTS input is the CTS_B pin."]
    #[inline(always)]
    pub fn cts(self) -> &'a mut W {
        self.variant(TXCTSSRC_A::CTS)
    }
    #[doc = "CTS input is an internal connection to the receiver address match result."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(TXCTSSRC_A::MATCH)
    }
}
#[doc = "Field `RTSWATER` reader - Receive RTS Configuration"]
pub type RTSWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTSWATER` writer - Receive RTS Configuration"]
pub type RTSWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODIR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TNP` reader - Transmitter narrow pulse"]
pub type TNP_R = crate::FieldReader<u8, TNP_A>;
#[doc = "Transmitter narrow pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TNP_A {
    #[doc = "0: 1/OSR."]
    ONE_SAMPLE = 0,
    #[doc = "1: 2/OSR."]
    TWO_SAMPLE = 1,
    #[doc = "2: 3/OSR."]
    THREE_SAMPLE = 2,
    #[doc = "3: 4/OSR."]
    FOUR_SAMPLE = 3,
}
impl From<TNP_A> for u8 {
    #[inline(always)]
    fn from(variant: TNP_A) -> Self {
        variant as _
    }
}
impl TNP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNP_A {
        match self.bits {
            0 => TNP_A::ONE_SAMPLE,
            1 => TNP_A::TWO_SAMPLE,
            2 => TNP_A::THREE_SAMPLE,
            3 => TNP_A::FOUR_SAMPLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SAMPLE`"]
    #[inline(always)]
    pub fn is_one_sample(&self) -> bool {
        *self == TNP_A::ONE_SAMPLE
    }
    #[doc = "Checks if the value of the field is `TWO_SAMPLE`"]
    #[inline(always)]
    pub fn is_two_sample(&self) -> bool {
        *self == TNP_A::TWO_SAMPLE
    }
    #[doc = "Checks if the value of the field is `THREE_SAMPLE`"]
    #[inline(always)]
    pub fn is_three_sample(&self) -> bool {
        *self == TNP_A::THREE_SAMPLE
    }
    #[doc = "Checks if the value of the field is `FOUR_SAMPLE`"]
    #[inline(always)]
    pub fn is_four_sample(&self) -> bool {
        *self == TNP_A::FOUR_SAMPLE
    }
}
#[doc = "Field `TNP` writer - Transmitter narrow pulse"]
pub type TNP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MODIR_SPEC, u8, TNP_A, 2, O>;
impl<'a, const O: u8> TNP_W<'a, O> {
    #[doc = "1/OSR."]
    #[inline(always)]
    pub fn one_sample(self) -> &'a mut W {
        self.variant(TNP_A::ONE_SAMPLE)
    }
    #[doc = "2/OSR."]
    #[inline(always)]
    pub fn two_sample(self) -> &'a mut W {
        self.variant(TNP_A::TWO_SAMPLE)
    }
    #[doc = "3/OSR."]
    #[inline(always)]
    pub fn three_sample(self) -> &'a mut W {
        self.variant(TNP_A::THREE_SAMPLE)
    }
    #[doc = "4/OSR."]
    #[inline(always)]
    pub fn four_sample(self) -> &'a mut W {
        self.variant(TNP_A::FOUR_SAMPLE)
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: IR disabled."]
    DISABLED = 0,
    #[doc = "1: IR enabled."]
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::ENABLED
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODIR_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&self) -> TXCTSE_R {
        TXCTSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&self) -> TXRTSE_R {
        TXRTSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&self) -> TXRTSPOL_R {
        TXRTSPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&self) -> RXRTSE_R {
        RXRTSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline(always)]
    pub fn txctsc(&self) -> TXCTSC_R {
        TXCTSC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline(always)]
    pub fn txctssrc(&self) -> TXCTSSRC_R {
        TXCTSSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline(always)]
    pub fn rtswater(&self) -> RTSWATER_R {
        RTSWATER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&self) -> TNP_R {
        TNP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txctse(&mut self) -> TXCTSE_W<0> {
        TXCTSE_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrtse(&mut self) -> TXRTSE_W<1> {
        TXRTSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    #[must_use]
    pub fn txrtspol(&mut self) -> TXRTSPOL_W<2> {
        TXRTSPOL_W::new(self)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrtse(&mut self) -> RXRTSE_W<3> {
        RXRTSE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn txctsc(&mut self) -> TXCTSC_W<4> {
        TXCTSC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline(always)]
    #[must_use]
    pub fn txctssrc(&mut self) -> TXCTSSRC_W<5> {
        TXCTSSRC_W::new(self)
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rtswater(&mut self) -> RTSWATER_W<8> {
        RTSWATER_W::new(self)
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tnp(&mut self) -> TNP_W<16> {
        TNP_W::new(self)
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<18> {
        IREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Modem IrDA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modir](index.html) module"]
pub struct MODIR_SPEC;
impl crate::RegisterSpec for MODIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modir::R](R) reader structure"]
impl crate::Readable for MODIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modir::W](W) writer structure"]
impl crate::Writable for MODIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODIR to value 0"]
impl crate::Resettable for MODIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
