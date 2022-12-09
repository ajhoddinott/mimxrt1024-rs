#[doc = "Register `RCR` reader"]
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR` writer"]
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOP` reader - Internal Loopback"]
pub type LOOP_R = crate::BitReader<LOOP_A>;
#[doc = "Internal Loopback\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOP_A {
    #[doc = "0: Loopback disabled."]
    ZERO = 0,
    #[doc = "1: Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    ONE = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::ZERO,
            true => LOOP_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == LOOP_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == LOOP_A::ONE
    }
}
#[doc = "Field `LOOP` writer - Internal Loopback"]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, LOOP_A, O>;
impl<'a, const O: u8> LOOP_W<'a, O> {
    #[doc = "Loopback disabled."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(LOOP_A::ZERO)
    }
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(LOOP_A::ONE)
    }
}
#[doc = "Field `DRT` reader - Disable Receive On Transmit"]
pub type DRT_R = crate::BitReader<DRT_A>;
#[doc = "Disable Receive On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRT_A {
    #[doc = "0: Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    ZERO = 0,
    #[doc = "1: Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    ONE = 1,
}
impl From<DRT_A> for bool {
    #[inline(always)]
    fn from(variant: DRT_A) -> Self {
        variant as u8 != 0
    }
}
impl DRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRT_A {
        match self.bits {
            false => DRT_A::ZERO,
            true => DRT_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == DRT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DRT_A::ONE
    }
}
#[doc = "Field `DRT` writer - Disable Receive On Transmit"]
pub type DRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, DRT_A, O>;
impl<'a, const O: u8> DRT_W<'a, O> {
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(DRT_A::ZERO)
    }
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(DRT_A::ONE)
    }
}
#[doc = "Field `MII_MODE` reader - Media Independent Interface Mode"]
pub type MII_MODE_R = crate::BitReader<MII_MODE_A>;
#[doc = "Media Independent Interface Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MII_MODE_A {
    #[doc = "1: MII or RMII mode, as indicated by the RMII_MODE field."]
    ONE = 1,
}
impl From<MII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MII_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MII_MODE_A> {
        match self.bits {
            true => Some(MII_MODE_A::ONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MII_MODE_A::ONE
    }
}
#[doc = "Field `MII_MODE` writer - Media Independent Interface Mode"]
pub type MII_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, MII_MODE_A, O>;
impl<'a, const O: u8> MII_MODE_W<'a, O> {
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(MII_MODE_A::ONE)
    }
}
#[doc = "Field `PROM` reader - Promiscuous Mode"]
pub type PROM_R = crate::BitReader<PROM_A>;
#[doc = "Promiscuous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROM_A {
    #[doc = "0: Disabled."]
    ZERO = 0,
    #[doc = "1: Enabled."]
    ONE = 1,
}
impl From<PROM_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_A) -> Self {
        variant as u8 != 0
    }
}
impl PROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_A {
        match self.bits {
            false => PROM_A::ZERO,
            true => PROM_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PROM_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PROM_A::ONE
    }
}
#[doc = "Field `PROM` writer - Promiscuous Mode"]
pub type PROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, PROM_A, O>;
impl<'a, const O: u8> PROM_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PROM_A::ZERO)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PROM_A::ONE)
    }
}
#[doc = "Field `BC_REJ` reader - Broadcast Frame Reject"]
pub type BC_REJ_R = crate::BitReader<BC_REJ_A>;
#[doc = "Broadcast Frame Reject\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC_REJ_A {
    #[doc = "0: Will not reject frames as described above"]
    ZERO = 0,
    #[doc = "1: Will reject frames as described above"]
    ONE = 1,
}
impl From<BC_REJ_A> for bool {
    #[inline(always)]
    fn from(variant: BC_REJ_A) -> Self {
        variant as u8 != 0
    }
}
impl BC_REJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC_REJ_A {
        match self.bits {
            false => BC_REJ_A::ZERO,
            true => BC_REJ_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == BC_REJ_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == BC_REJ_A::ONE
    }
}
#[doc = "Field `BC_REJ` writer - Broadcast Frame Reject"]
pub type BC_REJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, BC_REJ_A, O>;
impl<'a, const O: u8> BC_REJ_W<'a, O> {
    #[doc = "Will not reject frames as described above"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(BC_REJ_A::ZERO)
    }
    #[doc = "Will reject frames as described above"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(BC_REJ_A::ONE)
    }
}
#[doc = "Field `FCE` reader - Flow Control Enable"]
pub type FCE_R = crate::BitReader<FCE_A>;
#[doc = "Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_A {
    #[doc = "0: Disable flow control"]
    ZERO = 0,
    #[doc = "1: Enable flow control"]
    ONE = 1,
}
impl From<FCE_A> for bool {
    #[inline(always)]
    fn from(variant: FCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCE_A {
        match self.bits {
            false => FCE_A::ZERO,
            true => FCE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == FCE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == FCE_A::ONE
    }
}
#[doc = "Field `FCE` writer - Flow Control Enable"]
pub type FCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, FCE_A, O>;
impl<'a, const O: u8> FCE_W<'a, O> {
    #[doc = "Disable flow control"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(FCE_A::ZERO)
    }
    #[doc = "Enable flow control"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(FCE_A::ONE)
    }
}
#[doc = "Field `RMII_MODE` reader - RMII Mode Enable"]
pub type RMII_MODE_R = crate::BitReader<RMII_MODE_A>;
#[doc = "RMII Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMII_MODE_A {
    #[doc = "0: MAC configured for MII mode."]
    ZERO = 0,
    #[doc = "1: MAC configured for RMII operation."]
    ONE = 1,
}
impl From<RMII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RMII_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_MODE_A {
        match self.bits {
            false => RMII_MODE_A::ZERO,
            true => RMII_MODE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == RMII_MODE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == RMII_MODE_A::ONE
    }
}
#[doc = "Field `RMII_MODE` writer - RMII Mode Enable"]
pub type RMII_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, RMII_MODE_A, O>;
impl<'a, const O: u8> RMII_MODE_W<'a, O> {
    #[doc = "MAC configured for MII mode."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(RMII_MODE_A::ZERO)
    }
    #[doc = "MAC configured for RMII operation."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(RMII_MODE_A::ONE)
    }
}
#[doc = "Field `RMII_10T` reader - Enables 10-Mbit/s mode of the RMII ."]
pub type RMII_10T_R = crate::BitReader<RMII_10T_A>;
#[doc = "Enables 10-Mbit/s mode of the RMII .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMII_10T_A {
    #[doc = "0: 100-Mbit/s operation."]
    ZERO = 0,
    #[doc = "1: 10-Mbit/s operation."]
    ONE = 1,
}
impl From<RMII_10T_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_10T_A) -> Self {
        variant as u8 != 0
    }
}
impl RMII_10T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_10T_A {
        match self.bits {
            false => RMII_10T_A::ZERO,
            true => RMII_10T_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == RMII_10T_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == RMII_10T_A::ONE
    }
}
#[doc = "Field `RMII_10T` writer - Enables 10-Mbit/s mode of the RMII ."]
pub type RMII_10T_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, RMII_10T_A, O>;
impl<'a, const O: u8> RMII_10T_W<'a, O> {
    #[doc = "100-Mbit/s operation."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(RMII_10T_A::ZERO)
    }
    #[doc = "10-Mbit/s operation."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(RMII_10T_A::ONE)
    }
}
#[doc = "Field `PADEN` reader - Enable Frame Padding Remove On Receive"]
pub type PADEN_R = crate::BitReader<PADEN_A>;
#[doc = "Enable Frame Padding Remove On Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PADEN_A {
    #[doc = "0: No padding is removed on receive by the MAC."]
    ZERO = 0,
    #[doc = "1: Padding is removed from received frames."]
    ONE = 1,
}
impl From<PADEN_A> for bool {
    #[inline(always)]
    fn from(variant: PADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADEN_A {
        match self.bits {
            false => PADEN_A::ZERO,
            true => PADEN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PADEN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PADEN_A::ONE
    }
}
#[doc = "Field `PADEN` writer - Enable Frame Padding Remove On Receive"]
pub type PADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, PADEN_A, O>;
impl<'a, const O: u8> PADEN_W<'a, O> {
    #[doc = "No padding is removed on receive by the MAC."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PADEN_A::ZERO)
    }
    #[doc = "Padding is removed from received frames."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PADEN_A::ONE)
    }
}
#[doc = "Field `PAUFWD` reader - Terminate/Forward Pause Frames"]
pub type PAUFWD_R = crate::BitReader<PAUFWD_A>;
#[doc = "Terminate/Forward Pause Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAUFWD_A {
    #[doc = "0: Pause frames are terminated and discarded in the MAC."]
    ZERO = 0,
    #[doc = "1: Pause frames are forwarded to the user application."]
    ONE = 1,
}
impl From<PAUFWD_A> for bool {
    #[inline(always)]
    fn from(variant: PAUFWD_A) -> Self {
        variant as u8 != 0
    }
}
impl PAUFWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUFWD_A {
        match self.bits {
            false => PAUFWD_A::ZERO,
            true => PAUFWD_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PAUFWD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PAUFWD_A::ONE
    }
}
#[doc = "Field `PAUFWD` writer - Terminate/Forward Pause Frames"]
pub type PAUFWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, PAUFWD_A, O>;
impl<'a, const O: u8> PAUFWD_W<'a, O> {
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PAUFWD_A::ZERO)
    }
    #[doc = "Pause frames are forwarded to the user application."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PAUFWD_A::ONE)
    }
}
#[doc = "Field `CRCFWD` reader - Terminate/Forward Received CRC"]
pub type CRCFWD_R = crate::BitReader<CRCFWD_A>;
#[doc = "Terminate/Forward Received CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCFWD_A {
    #[doc = "0: The CRC field of received frames is transmitted to the user application."]
    ZERO = 0,
    #[doc = "1: The CRC field is stripped from the frame."]
    ONE = 1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCFWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCFWD_A {
        match self.bits {
            false => CRCFWD_A::ZERO,
            true => CRCFWD_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CRCFWD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CRCFWD_A::ONE
    }
}
#[doc = "Field `CRCFWD` writer - Terminate/Forward Received CRC"]
pub type CRCFWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, CRCFWD_A, O>;
impl<'a, const O: u8> CRCFWD_W<'a, O> {
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CRCFWD_A::ZERO)
    }
    #[doc = "The CRC field is stripped from the frame."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CRCFWD_A::ONE)
    }
}
#[doc = "Field `CFEN` reader - MAC Control Frame Enable"]
pub type CFEN_R = crate::BitReader<CFEN_A>;
#[doc = "MAC Control Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEN_A {
    #[doc = "0: MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    ZERO = 0,
    #[doc = "1: MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    ONE = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::ZERO,
            true => CFEN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFEN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFEN_A::ONE
    }
}
#[doc = "Field `CFEN` writer - MAC Control Frame Enable"]
pub type CFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, CFEN_A, O>;
impl<'a, const O: u8> CFEN_W<'a, O> {
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFEN_A::ZERO)
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFEN_A::ONE)
    }
}
#[doc = "Field `MAX_FL` reader - Maximum Frame Length"]
pub type MAX_FL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_FL` writer - Maximum Frame Length"]
pub type MAX_FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR_SPEC, u16, u16, 14, O>;
#[doc = "Field `NLC` reader - Payload Length Check Disable"]
pub type NLC_R = crate::BitReader<NLC_A>;
#[doc = "Payload Length Check Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NLC_A {
    #[doc = "0: The payload length check is disabled."]
    ZERO = 0,
    #[doc = "1: The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    ONE = 1,
}
impl From<NLC_A> for bool {
    #[inline(always)]
    fn from(variant: NLC_A) -> Self {
        variant as u8 != 0
    }
}
impl NLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NLC_A {
        match self.bits {
            false => NLC_A::ZERO,
            true => NLC_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == NLC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == NLC_A::ONE
    }
}
#[doc = "Field `NLC` writer - Payload Length Check Disable"]
pub type NLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, NLC_A, O>;
impl<'a, const O: u8> NLC_W<'a, O> {
    #[doc = "The payload length check is disabled."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(NLC_A::ZERO)
    }
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(NLC_A::ONE)
    }
}
#[doc = "Field `GRS` reader - Graceful Receive Stopped"]
pub type GRS_R = crate::BitReader<GRS_A>;
#[doc = "Graceful Receive Stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRS_A {
    #[doc = "0: Receive not stopped"]
    ZERO = 0,
    #[doc = "1: Receive stopped"]
    ONE = 1,
}
impl From<GRS_A> for bool {
    #[inline(always)]
    fn from(variant: GRS_A) -> Self {
        variant as u8 != 0
    }
}
impl GRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRS_A {
        match self.bits {
            false => GRS_A::ZERO,
            true => GRS_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == GRS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == GRS_A::ONE
    }
}
impl R {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    pub fn drt(&self) -> DRT_R {
        DRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    pub fn mii_mode(&self) -> MII_MODE_R {
        MII_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prom(&self) -> PROM_R {
        PROM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    pub fn bc_rej(&self) -> BC_REJ_R {
        BC_REJ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    pub fn rmii_mode(&self) -> RMII_MODE_R {
        RMII_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub fn rmii_10t(&self) -> RMII_10T_R {
        RMII_10T_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub fn paden(&self) -> PADEN_R {
        PADEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub fn paufwd(&self) -> PAUFWD_R {
        PAUFWD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    pub fn max_fl(&self) -> MAX_FL_R {
        MAX_FL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    pub fn nlc(&self) -> NLC_R {
        NLC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Graceful Receive Stopped"]
    #[inline(always)]
    pub fn grs(&self) -> GRS_R {
        GRS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<0> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    #[must_use]
    pub fn drt(&mut self) -> DRT_W<1> {
        DRT_W::new(self)
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mii_mode(&mut self) -> MII_MODE_W<2> {
        MII_MODE_W::new(self)
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prom(&mut self) -> PROM_W<3> {
        PROM_W::new(self)
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    #[must_use]
    pub fn bc_rej(&mut self) -> BC_REJ_W<4> {
        BC_REJ_W::new(self)
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FCE_W<5> {
        FCE_W::new(self)
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_mode(&mut self) -> RMII_MODE_W<8> {
        RMII_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    #[must_use]
    pub fn rmii_10t(&mut self) -> RMII_10T_W<9> {
        RMII_10T_W::new(self)
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    #[must_use]
    pub fn paden(&mut self) -> PADEN_W<12> {
        PADEN_W::new(self)
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    #[must_use]
    pub fn paufwd(&mut self) -> PAUFWD_W<13> {
        PAUFWD_W::new(self)
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcfwd(&mut self) -> CRCFWD_W<14> {
        CRCFWD_W::new(self)
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfen(&mut self) -> CFEN_W<15> {
        CFEN_W::new(self)
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn max_fl(&mut self) -> MAX_FL_W<16> {
        MAX_FL_W::new(self)
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nlc(&mut self) -> NLC_W<30> {
        NLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](index.html) module"]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr::R](R) reader structure"]
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr::W](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR to value 0x05ee_0001"]
impl crate::Resettable for RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05ee_0001;
}
