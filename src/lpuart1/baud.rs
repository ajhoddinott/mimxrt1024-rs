#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBR` reader - Baud Rate Modulo Divisor."]
pub type SBR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SBR` writer - Baud Rate Modulo Divisor."]
pub type SBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u16, u16, 13, O>;
#[doc = "Field `SBNS` reader - Stop Bit Number Select"]
pub type SBNS_R = crate::BitReader<SBNS_A>;
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBNS_A {
    #[doc = "0: One stop bit."]
    ONE = 0,
    #[doc = "1: Two stop bits."]
    TWO = 1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::ONE,
            true => SBNS_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBNS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SBNS_A::TWO
    }
}
#[doc = "Field `SBNS` writer - Stop Bit Number Select"]
pub type SBNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, SBNS_A, O>;
impl<'a, const O: u8> SBNS_W<'a, O> {
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBNS_A::ONE)
    }
    #[doc = "Two stop bits."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SBNS_A::TWO)
    }
}
#[doc = "Field `RXEDGIE` reader - RX Input Active Edge Interrupt Enable"]
pub type RXEDGIE_R = crate::BitReader<RXEDGIE_A>;
#[doc = "RX Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from STAT\\[RXEDGIF\\]
are disabled."]
    DISABLE = 0,
    #[doc = "1: Hardware interrupt is requested when STAT\\[RXEDGIF\\]
flag is 1."]
    ENABLE = 1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEDGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::DISABLE,
            true => RXEDGIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXEDGIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXEDGIE_A::ENABLE
    }
}
#[doc = "Field `RXEDGIE` writer - RX Input Active Edge Interrupt Enable"]
pub type RXEDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, RXEDGIE_A, O>;
impl<'a, const O: u8> RXEDGIE_W<'a, O> {
    #[doc = "Hardware interrupts from STAT\\[RXEDGIF\\]
are disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXEDGIE_A::DISABLE)
    }
    #[doc = "Hardware interrupt is requested when STAT\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXEDGIE_A::ENABLE)
    }
}
#[doc = "Field `LBKDIE` reader - LIN Break Detect Interrupt Enable"]
pub type LBKDIE_R = crate::BitReader<LBKDIE_A>;
#[doc = "LIN Break Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDIE_A {
    #[doc = "0: Hardware interrupts from STAT\\[LBKDIF\\]
flag are disabled (use polling)."]
    DISABLE = 0,
    #[doc = "1: Hardware interrupt is requested when STAT\\[LBKDIF\\]
flag is 1."]
    ENABLE = 1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::DISABLE,
            true => LBKDIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LBKDIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LBKDIE_A::ENABLE
    }
}
#[doc = "Field `LBKDIE` writer - LIN Break Detect Interrupt Enable"]
pub type LBKDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, LBKDIE_A, O>;
impl<'a, const O: u8> LBKDIE_W<'a, O> {
    #[doc = "Hardware interrupts from STAT\\[LBKDIF\\]
flag are disabled (use polling)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LBKDIE_A::DISABLE)
    }
    #[doc = "Hardware interrupt is requested when STAT\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LBKDIE_A::ENABLE)
    }
}
#[doc = "Field `RESYNCDIS` reader - Resynchronization Disable"]
pub type RESYNCDIS_R = crate::BitReader<RESYNCDIS_A>;
#[doc = "Resynchronization Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESYNCDIS_A {
    #[doc = "0: Resynchronization during received data word is supported."]
    RESYNC = 0,
    #[doc = "1: Resynchronization during received data word is disabled."]
    NO_RESYNC = 1,
}
impl From<RESYNCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNCDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RESYNCDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNCDIS_A {
        match self.bits {
            false => RESYNCDIS_A::RESYNC,
            true => RESYNCDIS_A::NO_RESYNC,
        }
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == RESYNCDIS_A::RESYNC
    }
    #[doc = "Checks if the value of the field is `NO_RESYNC`"]
    #[inline(always)]
    pub fn is_no_resync(&self) -> bool {
        *self == RESYNCDIS_A::NO_RESYNC
    }
}
#[doc = "Field `RESYNCDIS` writer - Resynchronization Disable"]
pub type RESYNCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, RESYNCDIS_A, O>;
impl<'a, const O: u8> RESYNCDIS_W<'a, O> {
    #[doc = "Resynchronization during received data word is supported."]
    #[inline(always)]
    pub fn resync(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::RESYNC)
    }
    #[doc = "Resynchronization during received data word is disabled."]
    #[inline(always)]
    pub fn no_resync(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::NO_RESYNC)
    }
}
#[doc = "Field `BOTHEDGE` reader - Both Edge Sampling"]
pub type BOTHEDGE_R = crate::BitReader<BOTHEDGE_A>;
#[doc = "Both Edge Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOTHEDGE_A {
    #[doc = "0: Receiver samples input data using the rising edge of the baud rate clock."]
    DISABLED = 0,
    #[doc = "1: Receiver samples input data using the rising and falling edge of the baud rate clock."]
    ENABLED = 1,
}
impl From<BOTHEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: BOTHEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOTHEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOTHEDGE_A {
        match self.bits {
            false => BOTHEDGE_A::DISABLED,
            true => BOTHEDGE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOTHEDGE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOTHEDGE_A::ENABLED
    }
}
#[doc = "Field `BOTHEDGE` writer - Both Edge Sampling"]
pub type BOTHEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, BOTHEDGE_A, O>;
impl<'a, const O: u8> BOTHEDGE_W<'a, O> {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::DISABLED)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::ENABLED)
    }
}
#[doc = "Field `MATCFG` reader - Match Configuration"]
pub type MATCFG_R = crate::FieldReader<u8, MATCFG_A>;
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Address Match Wakeup"]
    ADDR_MATCH = 0,
    #[doc = "1: Idle Match Wakeup"]
    IDLE_MATCH = 1,
    #[doc = "2: Match On and Match Off"]
    ONOFF_MATCH = 2,
    #[doc = "3: Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    RWU_MATCH = 3,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
impl MATCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCFG_A {
        match self.bits {
            0 => MATCFG_A::ADDR_MATCH,
            1 => MATCFG_A::IDLE_MATCH,
            2 => MATCFG_A::ONOFF_MATCH,
            3 => MATCFG_A::RWU_MATCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDR_MATCH`"]
    #[inline(always)]
    pub fn is_addr_match(&self) -> bool {
        *self == MATCFG_A::ADDR_MATCH
    }
    #[doc = "Checks if the value of the field is `IDLE_MATCH`"]
    #[inline(always)]
    pub fn is_idle_match(&self) -> bool {
        *self == MATCFG_A::IDLE_MATCH
    }
    #[doc = "Checks if the value of the field is `ONOFF_MATCH`"]
    #[inline(always)]
    pub fn is_onoff_match(&self) -> bool {
        *self == MATCFG_A::ONOFF_MATCH
    }
    #[doc = "Checks if the value of the field is `RWU_MATCH`"]
    #[inline(always)]
    pub fn is_rwu_match(&self) -> bool {
        *self == MATCFG_A::RWU_MATCH
    }
}
#[doc = "Field `MATCFG` writer - Match Configuration"]
pub type MATCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BAUD_SPEC, u8, MATCFG_A, 2, O>;
impl<'a, const O: u8> MATCFG_W<'a, O> {
    #[doc = "Address Match Wakeup"]
    #[inline(always)]
    pub fn addr_match(self) -> &'a mut W {
        self.variant(MATCFG_A::ADDR_MATCH)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline(always)]
    pub fn idle_match(self) -> &'a mut W {
        self.variant(MATCFG_A::IDLE_MATCH)
    }
    #[doc = "Match On and Match Off"]
    #[inline(always)]
    pub fn onoff_match(self) -> &'a mut W {
        self.variant(MATCFG_A::ONOFF_MATCH)
    }
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    #[inline(always)]
    pub fn rwu_match(self) -> &'a mut W {
        self.variant(MATCFG_A::RWU_MATCH)
    }
}
#[doc = "Field `RDMAE` reader - Receiver Full DMA Enable"]
pub type RDMAE_R = crate::BitReader<RDMAE_A>;
#[doc = "Receiver Full DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAE_A {
    #[doc = "0: DMA request disabled."]
    DISABLED = 0,
    #[doc = "1: DMA request enabled."]
    ENABLED = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::DISABLED,
            true => RDMAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMAE_A::ENABLED
    }
}
#[doc = "Field `RDMAE` writer - Receiver Full DMA Enable"]
pub type RDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, RDMAE_A, O>;
impl<'a, const O: u8> RDMAE_W<'a, O> {
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDMAE_A::DISABLED)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDMAE_A::ENABLED)
    }
}
#[doc = "Field `TDMAE` reader - Transmitter DMA Enable"]
pub type TDMAE_R = crate::BitReader<TDMAE_A>;
#[doc = "Transmitter DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDMAE_A {
    #[doc = "0: DMA request disabled."]
    DISABLED = 0,
    #[doc = "1: DMA request enabled."]
    ENABLED = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::DISABLED,
            true => TDMAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDMAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDMAE_A::ENABLED
    }
}
#[doc = "Field `TDMAE` writer - Transmitter DMA Enable"]
pub type TDMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, TDMAE_A, O>;
impl<'a, const O: u8> TDMAE_W<'a, O> {
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDMAE_A::DISABLED)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDMAE_A::ENABLED)
    }
}
#[doc = "Field `OSR` reader - Oversampling Ratio"]
pub type OSR_R = crate::FieldReader<u8, OSR_A>;
#[doc = "Oversampling Ratio\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: Writing 0 to this field results in an oversampling ratio of 16"]
    DEFAULT = 0,
    #[doc = "3: Oversampling ratio of 4, requires BOTHEDGE to be set."]
    OSR_4 = 3,
    #[doc = "4: Oversampling ratio of 5, requires BOTHEDGE to be set."]
    OSR_5 = 4,
    #[doc = "5: Oversampling ratio of 6, requires BOTHEDGE to be set."]
    OSR_6 = 5,
    #[doc = "6: Oversampling ratio of 7, requires BOTHEDGE to be set."]
    OSR_7 = 6,
    #[doc = "7: Oversampling ratio of 8."]
    OSR_8 = 7,
    #[doc = "8: Oversampling ratio of 9."]
    OSR_9 = 8,
    #[doc = "9: Oversampling ratio of 10."]
    OSR_10 = 9,
    #[doc = "10: Oversampling ratio of 11."]
    OSR_11 = 10,
    #[doc = "11: Oversampling ratio of 12."]
    OSR_12 = 11,
    #[doc = "12: Oversampling ratio of 13."]
    OSR_13 = 12,
    #[doc = "13: Oversampling ratio of 14."]
    OSR_14 = 13,
    #[doc = "14: Oversampling ratio of 15."]
    OSR_15 = 14,
    #[doc = "15: Oversampling ratio of 16."]
    OSR_16 = 15,
    #[doc = "16: Oversampling ratio of 17."]
    OSR_17 = 16,
    #[doc = "17: Oversampling ratio of 18."]
    OSR_18 = 17,
    #[doc = "18: Oversampling ratio of 19."]
    OSR_19 = 18,
    #[doc = "19: Oversampling ratio of 20."]
    OSR_20 = 19,
    #[doc = "20: Oversampling ratio of 21."]
    OSR_21 = 20,
    #[doc = "21: Oversampling ratio of 22."]
    OSR_22 = 21,
    #[doc = "22: Oversampling ratio of 23."]
    OSR_23 = 22,
    #[doc = "23: Oversampling ratio of 24."]
    OSR_24 = 23,
    #[doc = "24: Oversampling ratio of 25."]
    OSR_25 = 24,
    #[doc = "25: Oversampling ratio of 26."]
    OSR_26 = 25,
    #[doc = "26: Oversampling ratio of 27."]
    OSR_27 = 26,
    #[doc = "27: Oversampling ratio of 28."]
    OSR_28 = 27,
    #[doc = "28: Oversampling ratio of 29."]
    OSR_29 = 28,
    #[doc = "29: Oversampling ratio of 30."]
    OSR_30 = 29,
    #[doc = "30: Oversampling ratio of 31."]
    OSR_31 = 30,
    #[doc = "31: Oversampling ratio of 32."]
    OSR_32 = 31,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
impl OSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSR_A> {
        match self.bits {
            0 => Some(OSR_A::DEFAULT),
            3 => Some(OSR_A::OSR_4),
            4 => Some(OSR_A::OSR_5),
            5 => Some(OSR_A::OSR_6),
            6 => Some(OSR_A::OSR_7),
            7 => Some(OSR_A::OSR_8),
            8 => Some(OSR_A::OSR_9),
            9 => Some(OSR_A::OSR_10),
            10 => Some(OSR_A::OSR_11),
            11 => Some(OSR_A::OSR_12),
            12 => Some(OSR_A::OSR_13),
            13 => Some(OSR_A::OSR_14),
            14 => Some(OSR_A::OSR_15),
            15 => Some(OSR_A::OSR_16),
            16 => Some(OSR_A::OSR_17),
            17 => Some(OSR_A::OSR_18),
            18 => Some(OSR_A::OSR_19),
            19 => Some(OSR_A::OSR_20),
            20 => Some(OSR_A::OSR_21),
            21 => Some(OSR_A::OSR_22),
            22 => Some(OSR_A::OSR_23),
            23 => Some(OSR_A::OSR_24),
            24 => Some(OSR_A::OSR_25),
            25 => Some(OSR_A::OSR_26),
            26 => Some(OSR_A::OSR_27),
            27 => Some(OSR_A::OSR_28),
            28 => Some(OSR_A::OSR_29),
            29 => Some(OSR_A::OSR_30),
            30 => Some(OSR_A::OSR_31),
            31 => Some(OSR_A::OSR_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == OSR_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR_A::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_5`"]
    #[inline(always)]
    pub fn is_osr_5(&self) -> bool {
        *self == OSR_A::OSR_5
    }
    #[doc = "Checks if the value of the field is `OSR_6`"]
    #[inline(always)]
    pub fn is_osr_6(&self) -> bool {
        *self == OSR_A::OSR_6
    }
    #[doc = "Checks if the value of the field is `OSR_7`"]
    #[inline(always)]
    pub fn is_osr_7(&self) -> bool {
        *self == OSR_A::OSR_7
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR_A::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_9`"]
    #[inline(always)]
    pub fn is_osr_9(&self) -> bool {
        *self == OSR_A::OSR_9
    }
    #[doc = "Checks if the value of the field is `OSR_10`"]
    #[inline(always)]
    pub fn is_osr_10(&self) -> bool {
        *self == OSR_A::OSR_10
    }
    #[doc = "Checks if the value of the field is `OSR_11`"]
    #[inline(always)]
    pub fn is_osr_11(&self) -> bool {
        *self == OSR_A::OSR_11
    }
    #[doc = "Checks if the value of the field is `OSR_12`"]
    #[inline(always)]
    pub fn is_osr_12(&self) -> bool {
        *self == OSR_A::OSR_12
    }
    #[doc = "Checks if the value of the field is `OSR_13`"]
    #[inline(always)]
    pub fn is_osr_13(&self) -> bool {
        *self == OSR_A::OSR_13
    }
    #[doc = "Checks if the value of the field is `OSR_14`"]
    #[inline(always)]
    pub fn is_osr_14(&self) -> bool {
        *self == OSR_A::OSR_14
    }
    #[doc = "Checks if the value of the field is `OSR_15`"]
    #[inline(always)]
    pub fn is_osr_15(&self) -> bool {
        *self == OSR_A::OSR_15
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR_A::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_17`"]
    #[inline(always)]
    pub fn is_osr_17(&self) -> bool {
        *self == OSR_A::OSR_17
    }
    #[doc = "Checks if the value of the field is `OSR_18`"]
    #[inline(always)]
    pub fn is_osr_18(&self) -> bool {
        *self == OSR_A::OSR_18
    }
    #[doc = "Checks if the value of the field is `OSR_19`"]
    #[inline(always)]
    pub fn is_osr_19(&self) -> bool {
        *self == OSR_A::OSR_19
    }
    #[doc = "Checks if the value of the field is `OSR_20`"]
    #[inline(always)]
    pub fn is_osr_20(&self) -> bool {
        *self == OSR_A::OSR_20
    }
    #[doc = "Checks if the value of the field is `OSR_21`"]
    #[inline(always)]
    pub fn is_osr_21(&self) -> bool {
        *self == OSR_A::OSR_21
    }
    #[doc = "Checks if the value of the field is `OSR_22`"]
    #[inline(always)]
    pub fn is_osr_22(&self) -> bool {
        *self == OSR_A::OSR_22
    }
    #[doc = "Checks if the value of the field is `OSR_23`"]
    #[inline(always)]
    pub fn is_osr_23(&self) -> bool {
        *self == OSR_A::OSR_23
    }
    #[doc = "Checks if the value of the field is `OSR_24`"]
    #[inline(always)]
    pub fn is_osr_24(&self) -> bool {
        *self == OSR_A::OSR_24
    }
    #[doc = "Checks if the value of the field is `OSR_25`"]
    #[inline(always)]
    pub fn is_osr_25(&self) -> bool {
        *self == OSR_A::OSR_25
    }
    #[doc = "Checks if the value of the field is `OSR_26`"]
    #[inline(always)]
    pub fn is_osr_26(&self) -> bool {
        *self == OSR_A::OSR_26
    }
    #[doc = "Checks if the value of the field is `OSR_27`"]
    #[inline(always)]
    pub fn is_osr_27(&self) -> bool {
        *self == OSR_A::OSR_27
    }
    #[doc = "Checks if the value of the field is `OSR_28`"]
    #[inline(always)]
    pub fn is_osr_28(&self) -> bool {
        *self == OSR_A::OSR_28
    }
    #[doc = "Checks if the value of the field is `OSR_29`"]
    #[inline(always)]
    pub fn is_osr_29(&self) -> bool {
        *self == OSR_A::OSR_29
    }
    #[doc = "Checks if the value of the field is `OSR_30`"]
    #[inline(always)]
    pub fn is_osr_30(&self) -> bool {
        *self == OSR_A::OSR_30
    }
    #[doc = "Checks if the value of the field is `OSR_31`"]
    #[inline(always)]
    pub fn is_osr_31(&self) -> bool {
        *self == OSR_A::OSR_31
    }
    #[doc = "Checks if the value of the field is `OSR_32`"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == OSR_A::OSR_32
    }
}
#[doc = "Field `OSR` writer - Oversampling Ratio"]
pub type OSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, OSR_A, 5, O>;
impl<'a, const O: u8> OSR_W<'a, O> {
    #[doc = "Writing 0 to this field results in an oversampling ratio of 16"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(OSR_A::DEFAULT)
    }
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR_A::OSR_4)
    }
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_5(self) -> &'a mut W {
        self.variant(OSR_A::OSR_5)
    }
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_6(self) -> &'a mut W {
        self.variant(OSR_A::OSR_6)
    }
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_7(self) -> &'a mut W {
        self.variant(OSR_A::OSR_7)
    }
    #[doc = "Oversampling ratio of 8."]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR_A::OSR_8)
    }
    #[doc = "Oversampling ratio of 9."]
    #[inline(always)]
    pub fn osr_9(self) -> &'a mut W {
        self.variant(OSR_A::OSR_9)
    }
    #[doc = "Oversampling ratio of 10."]
    #[inline(always)]
    pub fn osr_10(self) -> &'a mut W {
        self.variant(OSR_A::OSR_10)
    }
    #[doc = "Oversampling ratio of 11."]
    #[inline(always)]
    pub fn osr_11(self) -> &'a mut W {
        self.variant(OSR_A::OSR_11)
    }
    #[doc = "Oversampling ratio of 12."]
    #[inline(always)]
    pub fn osr_12(self) -> &'a mut W {
        self.variant(OSR_A::OSR_12)
    }
    #[doc = "Oversampling ratio of 13."]
    #[inline(always)]
    pub fn osr_13(self) -> &'a mut W {
        self.variant(OSR_A::OSR_13)
    }
    #[doc = "Oversampling ratio of 14."]
    #[inline(always)]
    pub fn osr_14(self) -> &'a mut W {
        self.variant(OSR_A::OSR_14)
    }
    #[doc = "Oversampling ratio of 15."]
    #[inline(always)]
    pub fn osr_15(self) -> &'a mut W {
        self.variant(OSR_A::OSR_15)
    }
    #[doc = "Oversampling ratio of 16."]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR_A::OSR_16)
    }
    #[doc = "Oversampling ratio of 17."]
    #[inline(always)]
    pub fn osr_17(self) -> &'a mut W {
        self.variant(OSR_A::OSR_17)
    }
    #[doc = "Oversampling ratio of 18."]
    #[inline(always)]
    pub fn osr_18(self) -> &'a mut W {
        self.variant(OSR_A::OSR_18)
    }
    #[doc = "Oversampling ratio of 19."]
    #[inline(always)]
    pub fn osr_19(self) -> &'a mut W {
        self.variant(OSR_A::OSR_19)
    }
    #[doc = "Oversampling ratio of 20."]
    #[inline(always)]
    pub fn osr_20(self) -> &'a mut W {
        self.variant(OSR_A::OSR_20)
    }
    #[doc = "Oversampling ratio of 21."]
    #[inline(always)]
    pub fn osr_21(self) -> &'a mut W {
        self.variant(OSR_A::OSR_21)
    }
    #[doc = "Oversampling ratio of 22."]
    #[inline(always)]
    pub fn osr_22(self) -> &'a mut W {
        self.variant(OSR_A::OSR_22)
    }
    #[doc = "Oversampling ratio of 23."]
    #[inline(always)]
    pub fn osr_23(self) -> &'a mut W {
        self.variant(OSR_A::OSR_23)
    }
    #[doc = "Oversampling ratio of 24."]
    #[inline(always)]
    pub fn osr_24(self) -> &'a mut W {
        self.variant(OSR_A::OSR_24)
    }
    #[doc = "Oversampling ratio of 25."]
    #[inline(always)]
    pub fn osr_25(self) -> &'a mut W {
        self.variant(OSR_A::OSR_25)
    }
    #[doc = "Oversampling ratio of 26."]
    #[inline(always)]
    pub fn osr_26(self) -> &'a mut W {
        self.variant(OSR_A::OSR_26)
    }
    #[doc = "Oversampling ratio of 27."]
    #[inline(always)]
    pub fn osr_27(self) -> &'a mut W {
        self.variant(OSR_A::OSR_27)
    }
    #[doc = "Oversampling ratio of 28."]
    #[inline(always)]
    pub fn osr_28(self) -> &'a mut W {
        self.variant(OSR_A::OSR_28)
    }
    #[doc = "Oversampling ratio of 29."]
    #[inline(always)]
    pub fn osr_29(self) -> &'a mut W {
        self.variant(OSR_A::OSR_29)
    }
    #[doc = "Oversampling ratio of 30."]
    #[inline(always)]
    pub fn osr_30(self) -> &'a mut W {
        self.variant(OSR_A::OSR_30)
    }
    #[doc = "Oversampling ratio of 31."]
    #[inline(always)]
    pub fn osr_31(self) -> &'a mut W {
        self.variant(OSR_A::OSR_31)
    }
    #[doc = "Oversampling ratio of 32."]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut W {
        self.variant(OSR_A::OSR_32)
    }
}
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub type M10_R = crate::BitReader<M10_A>;
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M10_A {
    #[doc = "0: Receiver and transmitter use 7-bit to 9-bit data characters."]
    DISABLED = 0,
    #[doc = "1: Receiver and transmitter use 10-bit data characters."]
    ENABLED = 1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        variant as u8 != 0
    }
}
impl M10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::DISABLED,
            true => M10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M10_A::ENABLED
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub type M10_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, M10_A, O>;
impl<'a, const O: u8> M10_W<'a, O> {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M10_A::DISABLED)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M10_A::ENABLED)
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub type MAEN2_R = crate::BitReader<MAEN2_A>;
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAEN2_A {
    #[doc = "0: Normal operation."]
    DISABLED = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    ENABLED = 1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl MAEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::DISABLED,
            true => MAEN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MAEN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MAEN2_A::ENABLED
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub type MAEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, MAEN2_A, O>;
impl<'a, const O: u8> MAEN2_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAEN2_A::DISABLED)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAEN2_A::ENABLED)
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub type MAEN1_R = crate::BitReader<MAEN1_A>;
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAEN1_A {
    #[doc = "0: Normal operation."]
    DISABLED = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    ENABLED = 1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl MAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::DISABLED,
            true => MAEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MAEN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MAEN1_A::ENABLED
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub type MAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BAUD_SPEC, MAEN1_A, O>;
impl<'a, const O: u8> MAEN1_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MAEN1_A::DISABLED)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MAEN1_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&self) -> RESYNCDIS_R {
        RESYNCDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&self) -> BOTHEDGE_R {
        BOTHEDGE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SBR_W<0> {
        SBR_W::new(self)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbns(&mut self) -> SBNS_W<13> {
        SBNS_W::new(self)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgie(&mut self) -> RXEDGIE_W<14> {
        RXEDGIE_W::new(self)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdie(&mut self) -> LBKDIE_W<15> {
        LBKDIE_W::new(self)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn resyncdis(&mut self) -> RESYNCDIS_W<16> {
        RESYNCDIS_W::new(self)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn bothedge(&mut self) -> BOTHEDGE_W<17> {
        BOTHEDGE_W::new(self)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn matcfg(&mut self) -> MATCFG_W<18> {
        MATCFG_W::new(self)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdmae(&mut self) -> RDMAE_W<21> {
        RDMAE_W::new(self)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdmae(&mut self) -> TDMAE_W<23> {
        TDMAE_W::new(self)
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<24> {
        OSR_W::new(self)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn m10(&mut self) -> M10_W<29> {
        M10_W::new(self)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn maen2(&mut self) -> MAEN2_W<30> {
        MAEN2_W::new(self)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn maen1(&mut self) -> MAEN1_W<31> {
        MAEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD to value 0x0f00_0004"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_0004;
}
