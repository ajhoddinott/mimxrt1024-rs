#[doc = "Register `SCFGR1` reader"]
pub struct R(crate::R<SCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGR1` writer"]
pub struct W(crate::W<SCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGR1_SPEC>;
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
impl From<crate::W<SCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRSTALL` reader - Address SCL Stall"]
pub type ADRSTALL_R = crate::BitReader<ADRSTALL_A>;
#[doc = "Address SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    DISABLED = 0,
    #[doc = "1: Clock stretching is enabled"]
    ENABLED = 1,
}
impl From<ADRSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ADRSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRSTALL_A {
        match self.bits {
            false => ADRSTALL_A::DISABLED,
            true => ADRSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRSTALL_A::ENABLED
    }
}
#[doc = "Field `ADRSTALL` writer - Address SCL Stall"]
pub type ADRSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, ADRSTALL_A, O>;
impl<'a, const O: u8> ADRSTALL_W<'a, O> {
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADRSTALL_A::DISABLED)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADRSTALL_A::ENABLED)
    }
}
#[doc = "Field `RXSTALL` reader - RX SCL Stall"]
pub type RXSTALL_R = crate::BitReader<RXSTALL_A>;
#[doc = "RX SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    DISABLED = 0,
    #[doc = "1: Clock stretching is enabled"]
    ENABLED = 1,
}
impl From<RXSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTALL_A {
        match self.bits {
            false => RXSTALL_A::DISABLED,
            true => RXSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTALL_A::ENABLED
    }
}
#[doc = "Field `RXSTALL` writer - RX SCL Stall"]
pub type RXSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, RXSTALL_A, O>;
impl<'a, const O: u8> RXSTALL_W<'a, O> {
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXSTALL_A::DISABLED)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXSTALL_A::ENABLED)
    }
}
#[doc = "Field `TXDSTALL` reader - TX Data SCL Stall"]
pub type TXDSTALL_R = crate::BitReader<TXDSTALL_A>;
#[doc = "TX Data SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    DISABLED = 0,
    #[doc = "1: Clock stretching is enabled"]
    ENABLED = 1,
}
impl From<TXDSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDSTALL_A {
        match self.bits {
            false => TXDSTALL_A::DISABLED,
            true => TXDSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDSTALL_A::ENABLED
    }
}
#[doc = "Field `TXDSTALL` writer - TX Data SCL Stall"]
pub type TXDSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, TXDSTALL_A, O>;
impl<'a, const O: u8> TXDSTALL_W<'a, O> {
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDSTALL_A::DISABLED)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDSTALL_A::ENABLED)
    }
}
#[doc = "Field `ACKSTALL` reader - ACK SCL Stall"]
pub type ACKSTALL_R = crate::BitReader<ACKSTALL_A>;
#[doc = "ACK SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    DISABLED = 0,
    #[doc = "1: Clock stretching is enabled"]
    ENABLED = 1,
}
impl From<ACKSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ACKSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKSTALL_A {
        match self.bits {
            false => ACKSTALL_A::DISABLED,
            true => ACKSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACKSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACKSTALL_A::ENABLED
    }
}
#[doc = "Field `ACKSTALL` writer - ACK SCL Stall"]
pub type ACKSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, ACKSTALL_A, O>;
impl<'a, const O: u8> ACKSTALL_W<'a, O> {
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACKSTALL_A::DISABLED)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACKSTALL_A::ENABLED)
    }
}
#[doc = "Field `GCEN` reader - General Call Enable"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
#[doc = "General Call Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    #[doc = "0: General Call address is disabled"]
    DISABLED = 0,
    #[doc = "1: General Call address is enabled"]
    ENABLED = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::DISABLED,
            true => GCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN_A::ENABLED
    }
}
#[doc = "Field `GCEN` writer - General Call Enable"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, GCEN_A, O>;
impl<'a, const O: u8> GCEN_W<'a, O> {
    #[doc = "General Call address is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCEN_A::DISABLED)
    }
    #[doc = "General Call address is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCEN_A::ENABLED)
    }
}
#[doc = "Field `SAEN` reader - SMBus Alert Enable"]
pub type SAEN_R = crate::BitReader<SAEN_A>;
#[doc = "SMBus Alert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAEN_A {
    #[doc = "0: Disables match on SMBus Alert"]
    DISABLE = 0,
    #[doc = "1: Enables match on SMBus Alert"]
    ENABLE = 1,
}
impl From<SAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEN_A {
        match self.bits {
            false => SAEN_A::DISABLE,
            true => SAEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SAEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SAEN_A::ENABLE
    }
}
#[doc = "Field `SAEN` writer - SMBus Alert Enable"]
pub type SAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, SAEN_A, O>;
impl<'a, const O: u8> SAEN_W<'a, O> {
    #[doc = "Disables match on SMBus Alert"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SAEN_A::DISABLE)
    }
    #[doc = "Enables match on SMBus Alert"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SAEN_A::ENABLE)
    }
}
#[doc = "Field `TXCFG` reader - Transmit Flag Configuration"]
pub type TXCFG_R = crate::BitReader<TXCFG_A>;
#[doc = "Transmit Flag Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXCFG_A {
    #[doc = "0: Transmit Data Flag only asserts during a slave-transmit transfer when the Transmit Data register is empty"]
    ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY = 0,
    #[doc = "1: Transmit Data Flag asserts whenever the Transmit Data register is empty"]
    ASSERTS_WHEN_TX_DATA_EMPTY = 1,
}
impl From<TXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: TXCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl TXCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCFG_A {
        match self.bits {
            false => TXCFG_A::ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY,
            true => TXCFG_A::ASSERTS_WHEN_TX_DATA_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY`"]
    #[inline(always)]
    pub fn is_asserts_during_slave_transmit_transfer_when_tx_data_empty(&self) -> bool {
        *self == TXCFG_A::ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY
    }
    #[doc = "Checks if the value of the field is `ASSERTS_WHEN_TX_DATA_EMPTY`"]
    #[inline(always)]
    pub fn is_asserts_when_tx_data_empty(&self) -> bool {
        *self == TXCFG_A::ASSERTS_WHEN_TX_DATA_EMPTY
    }
}
#[doc = "Field `TXCFG` writer - Transmit Flag Configuration"]
pub type TXCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, TXCFG_A, O>;
impl<'a, const O: u8> TXCFG_W<'a, O> {
    #[doc = "Transmit Data Flag only asserts during a slave-transmit transfer when the Transmit Data register is empty"]
    #[inline(always)]
    pub fn asserts_during_slave_transmit_transfer_when_tx_data_empty(self) -> &'a mut W {
        self.variant(TXCFG_A::ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY)
    }
    #[doc = "Transmit Data Flag asserts whenever the Transmit Data register is empty"]
    #[inline(always)]
    pub fn asserts_when_tx_data_empty(self) -> &'a mut W {
        self.variant(TXCFG_A::ASSERTS_WHEN_TX_DATA_EMPTY)
    }
}
#[doc = "Field `RXCFG` reader - Receive Data Configuration"]
pub type RXCFG_R = crate::BitReader<RXCFG_A>;
#[doc = "Receive Data Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXCFG_A {
    #[doc = "0: Reading the Receive Data register returns received data and clears the Receive Data flag (MSR\\[RDF\\])."]
    RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG = 0,
    #[doc = "1: Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\])is set, returns the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, returns received data and clears the Receive Data flag (MSR\\[RDF\\])."]
    WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG = 1,
}
impl From<RXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: RXCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RXCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCFG_A {
        match self . bits { false => RXCFG_A :: RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG , true => RXCFG_A :: WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG , }
    }
    #[doc = "Checks if the value of the field is `RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG`"]
    #[inline(always)]
    pub fn is_returns_received_data_and_clears_rx_data_flag(&self) -> bool {
        *self == RXCFG_A::RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG
    }
    #[doc = "Checks if the value of the field is `WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG`"]
    #[inline(always)]
    pub fn is_when_address_valid_flag_set_returns_address_status_and_clears_address_valid_flag(
        &self,
    ) -> bool {
        * self == RXCFG_A :: WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG
    }
}
#[doc = "Field `RXCFG` writer - Receive Data Configuration"]
pub type RXCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, RXCFG_A, O>;
impl<'a, const O: u8> RXCFG_W<'a, O> {
    #[doc = "Reading the Receive Data register returns received data and clears the Receive Data flag (MSR\\[RDF\\])."]
    #[inline(always)]
    pub fn returns_received_data_and_clears_rx_data_flag(self) -> &'a mut W {
        self.variant(RXCFG_A::RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG)
    }
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\])is set, returns the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, returns received data and clears the Receive Data flag (MSR\\[RDF\\])."]
    #[inline(always)]
    pub fn when_address_valid_flag_set_returns_address_status_and_clears_address_valid_flag(
        self,
    ) -> &'a mut W {
        self . variant (RXCFG_A :: WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG)
    }
}
#[doc = "Field `IGNACK` reader - Ignore NACK"]
pub type IGNACK_R = crate::BitReader<IGNACK_A>;
#[doc = "Ignore NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IGNACK_A {
    #[doc = "0: Slave ends transfer when NACK is detected"]
    ENDS_TRANSFER_ON_NACK = 0,
    #[doc = "1: Slave does not end transfer when NACK detected"]
    DOES_NOT_END_TRANSFER_ON_NACK = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl IGNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::ENDS_TRANSFER_ON_NACK,
            true => IGNACK_A::DOES_NOT_END_TRANSFER_ON_NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ENDS_TRANSFER_ON_NACK`"]
    #[inline(always)]
    pub fn is_ends_transfer_on_nack(&self) -> bool {
        *self == IGNACK_A::ENDS_TRANSFER_ON_NACK
    }
    #[doc = "Checks if the value of the field is `DOES_NOT_END_TRANSFER_ON_NACK`"]
    #[inline(always)]
    pub fn is_does_not_end_transfer_on_nack(&self) -> bool {
        *self == IGNACK_A::DOES_NOT_END_TRANSFER_ON_NACK
    }
}
#[doc = "Field `IGNACK` writer - Ignore NACK"]
pub type IGNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, IGNACK_A, O>;
impl<'a, const O: u8> IGNACK_W<'a, O> {
    #[doc = "Slave ends transfer when NACK is detected"]
    #[inline(always)]
    pub fn ends_transfer_on_nack(self) -> &'a mut W {
        self.variant(IGNACK_A::ENDS_TRANSFER_ON_NACK)
    }
    #[doc = "Slave does not end transfer when NACK detected"]
    #[inline(always)]
    pub fn does_not_end_transfer_on_nack(self) -> &'a mut W {
        self.variant(IGNACK_A::DOES_NOT_END_TRANSFER_ON_NACK)
    }
}
#[doc = "Field `HSMEN` reader - High Speed Mode Enable"]
pub type HSMEN_R = crate::BitReader<HSMEN_A>;
#[doc = "High Speed Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSMEN_A {
    #[doc = "0: Disables detection of HS-mode master code"]
    DISABLED = 0,
    #[doc = "1: Enables detection of HS-mode master code"]
    ENABLED = 1,
}
impl From<HSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMEN_A {
        match self.bits {
            false => HSMEN_A::DISABLED,
            true => HSMEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSMEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSMEN_A::ENABLED
    }
}
#[doc = "Field `HSMEN` writer - High Speed Mode Enable"]
pub type HSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFGR1_SPEC, HSMEN_A, O>;
impl<'a, const O: u8> HSMEN_W<'a, O> {
    #[doc = "Disables detection of HS-mode master code"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSMEN_A::DISABLED)
    }
    #[doc = "Enables detection of HS-mode master code"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSMEN_A::ENABLED)
    }
}
#[doc = "Field `ADDRCFG` reader - Address Configuration"]
pub type ADDRCFG_R = crate::FieldReader<u8, ADDRCFG_A>;
#[doc = "Address Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADDRCFG_A {
    #[doc = "0: Address match 0 (7-bit)"]
    ADDRESS_MATCH0_7_BIT = 0,
    #[doc = "1: Address match 0 (10-bit)"]
    ADDRESS_MATCH0_10_BIT = 1,
    #[doc = "2: Address match 0 (7-bit) or Address match 1 (7-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT = 2,
    #[doc = "3: Address match 0 (10-bit) or Address match 1 (10-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT = 3,
    #[doc = "4: Address match 0 (7-bit) or Address match 1 (10-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT = 4,
    #[doc = "5: Address match 0 (10-bit) or Address match 1 (7-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT = 5,
    #[doc = "6: From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT = 6,
    #[doc = "7: From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT = 7,
}
impl From<ADDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDRCFG_A) -> Self {
        variant as _
    }
}
impl ADDRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRCFG_A {
        match self.bits {
            0 => ADDRCFG_A::ADDRESS_MATCH0_7_BIT,
            1 => ADDRCFG_A::ADDRESS_MATCH0_10_BIT,
            2 => ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT,
            3 => ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT,
            4 => ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT,
            5 => ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT,
            6 => ADDRCFG_A::FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT,
            7 => ADDRCFG_A::FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_7_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_7_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_7_BIT
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_10_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_10_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_10_BIT
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_7_bit_or_address_match1_7_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_10_bit_or_address_match1_10_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_7_bit_or_address_match1_10_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT
    }
    #[doc = "Checks if the value of the field is `ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT`"]
    #[inline(always)]
    pub fn is_address_match0_10_bit_or_address_match1_7_bit(&self) -> bool {
        *self == ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT
    }
    #[doc = "Checks if the value of the field is `FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT`"]
    #[inline(always)]
    pub fn is_from_address_match0_7_bit_to_address_match1_7_bit(&self) -> bool {
        *self == ADDRCFG_A::FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT
    }
    #[doc = "Checks if the value of the field is `FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT`"]
    #[inline(always)]
    pub fn is_from_address_match0_10_bit_to_address_match1_10_bit(&self) -> bool {
        *self == ADDRCFG_A::FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT
    }
}
#[doc = "Field `ADDRCFG` writer - Address Configuration"]
pub type ADDRCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCFGR1_SPEC, u8, ADDRCFG_A, 3, O>;
impl<'a, const O: u8> ADDRCFG_W<'a, O> {
    #[doc = "Address match 0 (7-bit)"]
    #[inline(always)]
    pub fn address_match0_7_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_7_BIT)
    }
    #[doc = "Address match 0 (10-bit)"]
    #[inline(always)]
    pub fn address_match0_10_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_10_BIT)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn address_match0_7_bit_or_address_match1_7_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn address_match0_10_bit_or_address_match1_10_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn address_match0_7_bit_or_address_match1_10_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn address_match0_10_bit_or_address_match1_7_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT)
    }
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn from_address_match0_7_bit_to_address_match1_7_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT)
    }
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn from_address_match0_10_bit_to_address_match1_10_bit(self) -> &'a mut W {
        self.variant(ADDRCFG_A::FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT)
    }
}
impl R {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    pub fn adrstall(&self) -> ADRSTALL_R {
        ADRSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    pub fn rxstall(&self) -> RXSTALL_R {
        RXSTALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    pub fn txdstall(&self) -> TXDSTALL_R {
        TXDSTALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    pub fn ackstall(&self) -> ACKSTALL_R {
        ACKSTALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    pub fn saen(&self) -> SAEN_R {
        SAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    pub fn txcfg(&self) -> TXCFG_R {
        TXCFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    pub fn rxcfg(&self) -> RXCFG_R {
        RXCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsmen(&self) -> HSMEN_R {
        HSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    pub fn addrcfg(&self) -> ADDRCFG_R {
        ADDRCFG_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    #[must_use]
    pub fn adrstall(&mut self) -> ADRSTALL_W<0> {
        ADRSTALL_W::new(self)
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    #[must_use]
    pub fn rxstall(&mut self) -> RXSTALL_W<1> {
        RXSTALL_W::new(self)
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    #[must_use]
    pub fn txdstall(&mut self) -> TXDSTALL_W<2> {
        TXDSTALL_W::new(self)
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    #[must_use]
    pub fn ackstall(&mut self) -> ACKSTALL_W<3> {
        ACKSTALL_W::new(self)
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<8> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn saen(&mut self) -> SAEN_W<9> {
        SAEN_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn txcfg(&mut self) -> TXCFG_W<10> {
        TXCFG_W::new(self)
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rxcfg(&mut self) -> RXCFG_W<11> {
        RXCFG_W::new(self)
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    #[must_use]
    pub fn ignack(&mut self) -> IGNACK_W<12> {
        IGNACK_W::new(self)
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsmen(&mut self) -> HSMEN_W<13> {
        HSMEN_W::new(self)
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn addrcfg(&mut self) -> ADDRCFG_W<16> {
        ADDRCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr1](index.html) module"]
pub struct SCFGR1_SPEC;
impl crate::RegisterSpec for SCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgr1::R](R) reader structure"]
impl crate::Readable for SCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgr1::W](W) writer structure"]
impl crate::Writable for SCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFGR1 to value 0"]
impl crate::Resettable for SCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
