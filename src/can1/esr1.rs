#[doc = "Register `ESR1` reader"]
pub struct R(crate::R<ESR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR1` writer"]
pub struct W(crate::W<ESR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR1_SPEC>;
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
impl From<crate::W<ESR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKINT` reader - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
pub type WAKINT_R = crate::BitReader<WAKINT_A>;
#[doc = "When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKINT_A {
    #[doc = "0: No such occurrence"]
    WAKINT_0 = 0,
    #[doc = "1: Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
    WAKINT_1 = 1,
}
impl From<WAKINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKINT_A {
        match self.bits {
            false => WAKINT_A::WAKINT_0,
            true => WAKINT_A::WAKINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKINT_0`"]
    #[inline(always)]
    pub fn is_wakint_0(&self) -> bool {
        *self == WAKINT_A::WAKINT_0
    }
    #[doc = "Checks if the value of the field is `WAKINT_1`"]
    #[inline(always)]
    pub fn is_wakint_1(&self) -> bool {
        *self == WAKINT_A::WAKINT_1
    }
}
#[doc = "Field `WAKINT` writer - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
pub type WAKINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, WAKINT_A, O>;
impl<'a, const O: u8> WAKINT_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn wakint_0(self) -> &'a mut W {
        self.variant(WAKINT_A::WAKINT_0)
    }
    #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode"]
    #[inline(always)]
    pub fn wakint_1(self) -> &'a mut W {
        self.variant(WAKINT_A::WAKINT_1)
    }
}
#[doc = "Field `ERRINT` reader - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
pub type ERRINT_R = crate::BitReader<ERRINT_A>;
#[doc = "This bit indicates that at least one of the Error Bits (bits 15-10) is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRINT_A {
    #[doc = "0: No such occurrence"]
    ERRINT_0 = 0,
    #[doc = "1: Indicates setting of any Error Bit in the Error and Status Register"]
    ERRINT_1 = 1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::ERRINT_0,
            true => ERRINT_A::ERRINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERRINT_0`"]
    #[inline(always)]
    pub fn is_errint_0(&self) -> bool {
        *self == ERRINT_A::ERRINT_0
    }
    #[doc = "Checks if the value of the field is `ERRINT_1`"]
    #[inline(always)]
    pub fn is_errint_1(&self) -> bool {
        *self == ERRINT_A::ERRINT_1
    }
}
#[doc = "Field `ERRINT` writer - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
pub type ERRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, ERRINT_A, O>;
impl<'a, const O: u8> ERRINT_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn errint_0(self) -> &'a mut W {
        self.variant(ERRINT_A::ERRINT_0)
    }
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register"]
    #[inline(always)]
    pub fn errint_1(self) -> &'a mut W {
        self.variant(ERRINT_A::ERRINT_1)
    }
}
#[doc = "Field `BOFFINT` reader - This bit is set when FLEXCAN enters 'Bus Off' state"]
pub type BOFFINT_R = crate::BitReader<BOFFINT_A>;
#[doc = "This bit is set when FLEXCAN enters 'Bus Off' state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFFINT_A {
    #[doc = "0: No such occurrence"]
    BOFFINT_0 = 0,
    #[doc = "1: FLEXCAN module entered 'Bus Off' state"]
    BOFFINT_1 = 1,
}
impl From<BOFFINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFINT_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFFINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFINT_A {
        match self.bits {
            false => BOFFINT_A::BOFFINT_0,
            true => BOFFINT_A::BOFFINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOFFINT_0`"]
    #[inline(always)]
    pub fn is_boffint_0(&self) -> bool {
        *self == BOFFINT_A::BOFFINT_0
    }
    #[doc = "Checks if the value of the field is `BOFFINT_1`"]
    #[inline(always)]
    pub fn is_boffint_1(&self) -> bool {
        *self == BOFFINT_A::BOFFINT_1
    }
}
#[doc = "Field `BOFFINT` writer - This bit is set when FLEXCAN enters 'Bus Off' state"]
pub type BOFFINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, BOFFINT_A, O>;
impl<'a, const O: u8> BOFFINT_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn boffint_0(self) -> &'a mut W {
        self.variant(BOFFINT_A::BOFFINT_0)
    }
    #[doc = "FLEXCAN module entered 'Bus Off' state"]
    #[inline(always)]
    pub fn boffint_1(self) -> &'a mut W {
        self.variant(BOFFINT_A::BOFFINT_1)
    }
}
#[doc = "Field `RX` reader - This bit indicates if FlexCAN is receiving a message. Refer to ."]
pub type RX_R = crate::BitReader<RX_A>;
#[doc = "This bit indicates if FlexCAN is receiving a message. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "0: FLEXCAN is receiving a message"]
    RX_0 = 0,
    #[doc = "1: FLEXCAN is transmitting a message"]
    RX_1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::RX_0,
            true => RX_A::RX_1,
        }
    }
    #[doc = "Checks if the value of the field is `RX_0`"]
    #[inline(always)]
    pub fn is_rx_0(&self) -> bool {
        *self == RX_A::RX_0
    }
    #[doc = "Checks if the value of the field is `RX_1`"]
    #[inline(always)]
    pub fn is_rx_1(&self) -> bool {
        *self == RX_A::RX_1
    }
}
#[doc = "Field `FLTCONF` reader - If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
pub type FLTCONF_R = crate::FieldReader<u8, FLTCONF_A>;
#[doc = "If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLTCONF_A {
    #[doc = "0: Error Active"]
    FLTCONF_0 = 0,
    #[doc = "1: Error Passive"]
    FLTCONF_1 = 1,
    #[doc = "2: Bus off"]
    FLTCONF_2 = 2,
}
impl From<FLTCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTCONF_A) -> Self {
        variant as _
    }
}
impl FLTCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLTCONF_A> {
        match self.bits {
            0 => Some(FLTCONF_A::FLTCONF_0),
            1 => Some(FLTCONF_A::FLTCONF_1),
            2 => Some(FLTCONF_A::FLTCONF_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLTCONF_0`"]
    #[inline(always)]
    pub fn is_fltconf_0(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_0
    }
    #[doc = "Checks if the value of the field is `FLTCONF_1`"]
    #[inline(always)]
    pub fn is_fltconf_1(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_1
    }
    #[doc = "Checks if the value of the field is `FLTCONF_2`"]
    #[inline(always)]
    pub fn is_fltconf_2(&self) -> bool {
        *self == FLTCONF_A::FLTCONF_2
    }
}
#[doc = "Field `TX` reader - This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "This bit indicates if FLEXCAN is transmitting a message.Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "0: FLEXCAN is receiving a message"]
    TX_0 = 0,
    #[doc = "1: FLEXCAN is transmitting a message"]
    TX_1 = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::TX_0,
            true => TX_A::TX_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_0`"]
    #[inline(always)]
    pub fn is_tx_0(&self) -> bool {
        *self == TX_A::TX_0
    }
    #[doc = "Checks if the value of the field is `TX_1`"]
    #[inline(always)]
    pub fn is_tx_1(&self) -> bool {
        *self == TX_A::TX_1
    }
}
#[doc = "Field `IDLE` reader - This bit indicates when CAN bus is in IDLE state.Refer to ."]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "This bit indicates when CAN bus is in IDLE state.Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: No such occurrence"]
    IDLE_0 = 0,
    #[doc = "1: CAN bus is now IDLE"]
    IDLE_1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::IDLE_0,
            true => IDLE_A::IDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_0`"]
    #[inline(always)]
    pub fn is_idle_0(&self) -> bool {
        *self == IDLE_A::IDLE_0
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline(always)]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLE_A::IDLE_1
    }
}
#[doc = "Field `RXWRN` reader - This bit indicates when repetitive errors are occurring during message reception."]
pub type RXWRN_R = crate::BitReader<RXWRN_A>;
#[doc = "This bit indicates when repetitive errors are occurring during message reception.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXWRN_A {
    #[doc = "0: No such occurrence"]
    RXWRN_0 = 0,
    #[doc = "1: Rx_Err_Counter >= 96"]
    RXWRN_1 = 1,
}
impl From<RXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: RXWRN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXWRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWRN_A {
        match self.bits {
            false => RXWRN_A::RXWRN_0,
            true => RXWRN_A::RXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXWRN_0`"]
    #[inline(always)]
    pub fn is_rxwrn_0(&self) -> bool {
        *self == RXWRN_A::RXWRN_0
    }
    #[doc = "Checks if the value of the field is `RXWRN_1`"]
    #[inline(always)]
    pub fn is_rxwrn_1(&self) -> bool {
        *self == RXWRN_A::RXWRN_1
    }
}
#[doc = "Field `TXWRN` reader - This bit indicates when repetitive errors are occurring during message transmission."]
pub type TXWRN_R = crate::BitReader<TXWRN_A>;
#[doc = "This bit indicates when repetitive errors are occurring during message transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXWRN_A {
    #[doc = "0: No such occurrence"]
    TXWRN_0 = 0,
    #[doc = "1: TX_Err_Counter >= 96"]
    TXWRN_1 = 1,
}
impl From<TXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: TXWRN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXWRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWRN_A {
        match self.bits {
            false => TXWRN_A::TXWRN_0,
            true => TXWRN_A::TXWRN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXWRN_0`"]
    #[inline(always)]
    pub fn is_txwrn_0(&self) -> bool {
        *self == TXWRN_A::TXWRN_0
    }
    #[doc = "Checks if the value of the field is `TXWRN_1`"]
    #[inline(always)]
    pub fn is_txwrn_1(&self) -> bool {
        *self == TXWRN_A::TXWRN_1
    }
}
#[doc = "Field `STFERR` reader - This bit indicates that a Stuffing Error has been detected."]
pub type STFERR_R = crate::BitReader<STFERR_A>;
#[doc = "This bit indicates that a Stuffing Error has been detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STFERR_A {
    #[doc = "0: No such occurrence."]
    STFERR_0 = 0,
    #[doc = "1: A Stuffing Error occurred since last read of this register."]
    STFERR_1 = 1,
}
impl From<STFERR_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_A) -> Self {
        variant as u8 != 0
    }
}
impl STFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_A {
        match self.bits {
            false => STFERR_A::STFERR_0,
            true => STFERR_A::STFERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `STFERR_0`"]
    #[inline(always)]
    pub fn is_stferr_0(&self) -> bool {
        *self == STFERR_A::STFERR_0
    }
    #[doc = "Checks if the value of the field is `STFERR_1`"]
    #[inline(always)]
    pub fn is_stferr_1(&self) -> bool {
        *self == STFERR_A::STFERR_1
    }
}
#[doc = "Field `FRMERR` reader - This bit indicates that a Form Error has been detected by the receiver node, i"]
pub type FRMERR_R = crate::BitReader<FRMERR_A>;
#[doc = "This bit indicates that a Form Error has been detected by the receiver node, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRMERR_A {
    #[doc = "0: No such occurrence"]
    FRMERR_0 = 0,
    #[doc = "1: A Form Error occurred since last read of this register"]
    FRMERR_1 = 1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FRMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::FRMERR_0,
            true => FRMERR_A::FRMERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMERR_0`"]
    #[inline(always)]
    pub fn is_frmerr_0(&self) -> bool {
        *self == FRMERR_A::FRMERR_0
    }
    #[doc = "Checks if the value of the field is `FRMERR_1`"]
    #[inline(always)]
    pub fn is_frmerr_1(&self) -> bool {
        *self == FRMERR_A::FRMERR_1
    }
}
#[doc = "Field `CRCERR` reader - This bit indicates that a CRC Error has been detected by the receiver node, i"]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "This bit indicates that a CRC Error has been detected by the receiver node, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR_A {
    #[doc = "0: No such occurrence"]
    CRCERR_0 = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CRCERR_1 = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::CRCERR_0,
            true => CRCERR_A::CRCERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERR_0`"]
    #[inline(always)]
    pub fn is_crcerr_0(&self) -> bool {
        *self == CRCERR_A::CRCERR_0
    }
    #[doc = "Checks if the value of the field is `CRCERR_1`"]
    #[inline(always)]
    pub fn is_crcerr_1(&self) -> bool {
        *self == CRCERR_A::CRCERR_1
    }
}
#[doc = "Field `ACKERR` reader - This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
pub type ACKERR_R = crate::BitReader<ACKERR_A>;
#[doc = "This bit indicates that an Acknowledge Error has been detected by the transmitter node, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKERR_A {
    #[doc = "0: No such occurrence"]
    ACKERR_0 = 0,
    #[doc = "1: An ACK error occurred since last read of this register"]
    ACKERR_1 = 1,
}
impl From<ACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKERR_A {
        match self.bits {
            false => ACKERR_A::ACKERR_0,
            true => ACKERR_A::ACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACKERR_0`"]
    #[inline(always)]
    pub fn is_ackerr_0(&self) -> bool {
        *self == ACKERR_A::ACKERR_0
    }
    #[doc = "Checks if the value of the field is `ACKERR_1`"]
    #[inline(always)]
    pub fn is_ackerr_1(&self) -> bool {
        *self == ACKERR_A::ACKERR_1
    }
}
#[doc = "Field `BIT0ERR` reader - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
pub type BIT0ERR_R = crate::BitReader<BIT0ERR_A>;
#[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT0ERR_A {
    #[doc = "0: No such occurrence"]
    BIT0ERR_0 = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive"]
    BIT0ERR_1 = 1,
}
impl From<BIT0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT0ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_A {
        match self.bits {
            false => BIT0ERR_A::BIT0ERR_0,
            true => BIT0ERR_A::BIT0ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_0`"]
    #[inline(always)]
    pub fn is_bit0err_0(&self) -> bool {
        *self == BIT0ERR_A::BIT0ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT0ERR_1`"]
    #[inline(always)]
    pub fn is_bit0err_1(&self) -> bool {
        *self == BIT0ERR_A::BIT0ERR_1
    }
}
#[doc = "Field `BIT1ERR` reader - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
pub type BIT1ERR_R = crate::BitReader<BIT1ERR_A>;
#[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT1ERR_A {
    #[doc = "0: No such occurrence"]
    BIT1ERR_0 = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant"]
    BIT1ERR_1 = 1,
}
impl From<BIT1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT1ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_A {
        match self.bits {
            false => BIT1ERR_A::BIT1ERR_0,
            true => BIT1ERR_A::BIT1ERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_0`"]
    #[inline(always)]
    pub fn is_bit1err_0(&self) -> bool {
        *self == BIT1ERR_A::BIT1ERR_0
    }
    #[doc = "Checks if the value of the field is `BIT1ERR_1`"]
    #[inline(always)]
    pub fn is_bit1err_1(&self) -> bool {
        *self == BIT1ERR_A::BIT1ERR_1
    }
}
#[doc = "Field `RWRNINT` reader - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
pub type RWRNINT_R = crate::BitReader<RWRNINT_A>;
#[doc = "If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWRNINT_A {
    #[doc = "0: No such occurrence"]
    RWRNINT_0 = 0,
    #[doc = "1: The Rx error counter transition from < 96 to >= 96"]
    RWRNINT_1 = 1,
}
impl From<RWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
impl RWRNINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNINT_A {
        match self.bits {
            false => RWRNINT_A::RWRNINT_0,
            true => RWRNINT_A::RWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWRNINT_0`"]
    #[inline(always)]
    pub fn is_rwrnint_0(&self) -> bool {
        *self == RWRNINT_A::RWRNINT_0
    }
    #[doc = "Checks if the value of the field is `RWRNINT_1`"]
    #[inline(always)]
    pub fn is_rwrnint_1(&self) -> bool {
        *self == RWRNINT_A::RWRNINT_1
    }
}
#[doc = "Field `RWRNINT` writer - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
pub type RWRNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, RWRNINT_A, O>;
impl<'a, const O: u8> RWRNINT_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn rwrnint_0(self) -> &'a mut W {
        self.variant(RWRNINT_A::RWRNINT_0)
    }
    #[doc = "The Rx error counter transition from < 96 to >= 96"]
    #[inline(always)]
    pub fn rwrnint_1(self) -> &'a mut W {
        self.variant(RWRNINT_A::RWRNINT_1)
    }
}
#[doc = "Field `TWRNINT` reader - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
pub type TWRNINT_R = crate::BitReader<TWRNINT_A>;
#[doc = "If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWRNINT_A {
    #[doc = "0: No such occurrence"]
    TWRNINT_0 = 0,
    #[doc = "1: The Tx error counter transition from < 96 to >= 96"]
    TWRNINT_1 = 1,
}
impl From<TWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
impl TWRNINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNINT_A {
        match self.bits {
            false => TWRNINT_A::TWRNINT_0,
            true => TWRNINT_A::TWRNINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNINT_0`"]
    #[inline(always)]
    pub fn is_twrnint_0(&self) -> bool {
        *self == TWRNINT_A::TWRNINT_0
    }
    #[doc = "Checks if the value of the field is `TWRNINT_1`"]
    #[inline(always)]
    pub fn is_twrnint_1(&self) -> bool {
        *self == TWRNINT_A::TWRNINT_1
    }
}
#[doc = "Field `TWRNINT` writer - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
pub type TWRNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESR1_SPEC, TWRNINT_A, O>;
impl<'a, const O: u8> TWRNINT_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn twrnint_0(self) -> &'a mut W {
        self.variant(TWRNINT_A::TWRNINT_0)
    }
    #[doc = "The Tx error counter transition from < 96 to >= 96"]
    #[inline(always)]
    pub fn twrnint_1(self) -> &'a mut W {
        self.variant(TWRNINT_A::TWRNINT_1)
    }
}
#[doc = "Field `SYNCH` reader - This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
pub type SYNCH_R = crate::BitReader<SYNCH_A>;
#[doc = "This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCH_A {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus"]
    SYNCH_0 = 0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus"]
    SYNCH_1 = 1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::SYNCH_0,
            true => SYNCH_A::SYNCH_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCH_0`"]
    #[inline(always)]
    pub fn is_synch_0(&self) -> bool {
        *self == SYNCH_A::SYNCH_0
    }
    #[doc = "Checks if the value of the field is `SYNCH_1`"]
    #[inline(always)]
    pub fn is_synch_1(&self) -> bool {
        *self == SYNCH_A::SYNCH_1
    }
}
impl R {
    #[doc = "Bit 0 - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
    #[inline(always)]
    pub fn wakint(&self) -> WAKINT_R {
        WAKINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[inline(always)]
    pub fn boffint(&self) -> BOFFINT_R {
        BOFFINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates if FlexCAN is receiving a message. Refer to ."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
    #[inline(always)]
    pub fn fltconf(&self) -> FLTCONF_R {
        FLTCONF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates when CAN bus is in IDLE state.Refer to ."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit indicates when repetitive errors are occurring during message reception."]
    #[inline(always)]
    pub fn rxwrn(&self) -> RXWRN_R {
        RXWRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit indicates when repetitive errors are occurring during message transmission."]
    #[inline(always)]
    pub fn txwrn(&self) -> TXWRN_R {
        TXWRN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit indicates that a Stuffing Error has been detected."]
    #[inline(always)]
    pub fn stferr(&self) -> STFERR_R {
        STFERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates that a Form Error has been detected by the receiver node, i"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates that a CRC Error has been detected by the receiver node, i"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
    #[inline(always)]
    pub fn ackerr(&self) -> ACKERR_R {
        ACKERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline(always)]
    pub fn bit0err(&self) -> BIT0ERR_R {
        BIT0ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline(always)]
    pub fn bit1err(&self) -> BIT1ERR_R {
        BIT1ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RWRNINT_R {
        RWRNINT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[inline(always)]
    pub fn twrnint(&self) -> TWRNINT_R {
        TWRNINT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
    #[inline(always)]
    #[must_use]
    pub fn wakint(&mut self) -> WAKINT_W<0> {
        WAKINT_W::new(self)
    }
    #[doc = "Bit 1 - This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[inline(always)]
    #[must_use]
    pub fn errint(&mut self) -> ERRINT_W<1> {
        ERRINT_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[inline(always)]
    #[must_use]
    pub fn boffint(&mut self) -> BOFFINT_W<2> {
        BOFFINT_W::new(self)
    }
    #[doc = "Bit 16 - If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[inline(always)]
    #[must_use]
    pub fn rwrnint(&mut self) -> RWRNINT_W<16> {
        RWRNINT_W::new(self)
    }
    #[doc = "Bit 17 - If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[inline(always)]
    #[must_use]
    pub fn twrnint(&mut self) -> TWRNINT_W<17> {
        TWRNINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error and Status 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr1](index.html) module"]
pub struct ESR1_SPEC;
impl crate::RegisterSpec for ESR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr1::R](R) reader structure"]
impl crate::Readable for ESR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr1::W](W) writer structure"]
impl crate::Writable for ESR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESR1 to value 0"]
impl crate::Resettable for ESR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
