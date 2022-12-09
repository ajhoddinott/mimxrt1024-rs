#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - Master Mode"]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "Master Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode"]
    SLAVE_MODE = 0,
    #[doc = "1: Master mode"]
    MASTER_MODE = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE_MODE,
            true => MASTER_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MASTER_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MASTER_A::MASTER_MODE
    }
}
#[doc = "Field `MASTER` writer - Master Mode"]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE_MODE)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_MODE)
    }
}
#[doc = "Field `SAMPLE` reader - Sample Point"]
pub type SAMPLE_R = crate::BitReader<SAMPLE_A>;
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPLE_A {
    #[doc = "0: Input data is sampled on SCK edge"]
    ON_SCK_EDGE = 0,
    #[doc = "1: Input data is sampled on delayed SCK edge"]
    ON_DELAYED_SCK_EDGE = 1,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLE_A {
        match self.bits {
            false => SAMPLE_A::ON_SCK_EDGE,
            true => SAMPLE_A::ON_DELAYED_SCK_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `ON_SCK_EDGE`"]
    #[inline(always)]
    pub fn is_on_sck_edge(&self) -> bool {
        *self == SAMPLE_A::ON_SCK_EDGE
    }
    #[doc = "Checks if the value of the field is `ON_DELAYED_SCK_EDGE`"]
    #[inline(always)]
    pub fn is_on_delayed_sck_edge(&self) -> bool {
        *self == SAMPLE_A::ON_DELAYED_SCK_EDGE
    }
}
#[doc = "Field `SAMPLE` writer - Sample Point"]
pub type SAMPLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SAMPLE_A, O>;
impl<'a, const O: u8> SAMPLE_W<'a, O> {
    #[doc = "Input data is sampled on SCK edge"]
    #[inline(always)]
    pub fn on_sck_edge(self) -> &'a mut W {
        self.variant(SAMPLE_A::ON_SCK_EDGE)
    }
    #[doc = "Input data is sampled on delayed SCK edge"]
    #[inline(always)]
    pub fn on_delayed_sck_edge(self) -> &'a mut W {
        self.variant(SAMPLE_A::ON_DELAYED_SCK_EDGE)
    }
}
#[doc = "Field `AUTOPCS` reader - Automatic PCS"]
pub type AUTOPCS_R = crate::BitReader<AUTOPCS_A>;
#[doc = "Automatic PCS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOPCS_A {
    #[doc = "0: Automatic PCS generation is disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic PCS generation is enabled"]
    ENABLED = 1,
}
impl From<AUTOPCS_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOPCS_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOPCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOPCS_A {
        match self.bits {
            false => AUTOPCS_A::DISABLED,
            true => AUTOPCS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOPCS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOPCS_A::ENABLED
    }
}
#[doc = "Field `AUTOPCS` writer - Automatic PCS"]
pub type AUTOPCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AUTOPCS_A, O>;
impl<'a, const O: u8> AUTOPCS_W<'a, O> {
    #[doc = "Automatic PCS generation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOPCS_A::DISABLED)
    }
    #[doc = "Automatic PCS generation is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOPCS_A::ENABLED)
    }
}
#[doc = "Field `NOSTALL` reader - No Stall"]
pub type NOSTALL_R = crate::BitReader<NOSTALL_A>;
#[doc = "No Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTALL_A {
    #[doc = "0: Transfers stall when the transmit FIFO is empty"]
    DISABLED = 0,
    #[doc = "1: Transfers do not stall, allowing transmit FIFO underruns to occur"]
    ENABLED = 1,
}
impl From<NOSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTALL_A) -> Self {
        variant as u8 != 0
    }
}
impl NOSTALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOSTALL_A {
        match self.bits {
            false => NOSTALL_A::DISABLED,
            true => NOSTALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTALL_A::ENABLED
    }
}
#[doc = "Field `NOSTALL` writer - No Stall"]
pub type NOSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, NOSTALL_A, O>;
impl<'a, const O: u8> NOSTALL_W<'a, O> {
    #[doc = "Transfers stall when the transmit FIFO is empty"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTALL_A::DISABLED)
    }
    #[doc = "Transfers do not stall, allowing transmit FIFO underruns to occur"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTALL_A::ENABLED)
    }
}
#[doc = "Field `PCSPOL` reader - Peripheral Chip Select Polarity"]
pub type PCSPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSPOL` writer - Peripheral Chip Select Polarity"]
pub type PCSPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `MATCFG` reader - Match Configuration"]
pub type MATCFG_R = crate::FieldReader<u8, MATCFG_A>;
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match is disabled"]
    DISABLED = 0,
    #[doc = "2: Match is enabled is 1st data word is MATCH0 or MATCH1"]
    ENABLED_FIRSTDATAMATCH = 2,
    #[doc = "3: Match is enabled on any data word equal MATCH0 or MATCH1"]
    ENABLED_ANYDATAMATCH = 3,
    #[doc = "4: Match is enabled on data match sequence"]
    ENABLED_DATAMATCH_100 = 4,
    #[doc = "5: Match is enabled on data match sequence"]
    ENABLED_DATAMATCH_101 = 5,
    #[doc = "6: Match is enabled"]
    ENABLED_DATAMATCH_110 = 6,
    #[doc = "7: Match is enabled"]
    ENABLED_DATAMATCH_111 = 7,
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
    pub fn variant(&self) -> Option<MATCFG_A> {
        match self.bits {
            0 => Some(MATCFG_A::DISABLED),
            2 => Some(MATCFG_A::ENABLED_FIRSTDATAMATCH),
            3 => Some(MATCFG_A::ENABLED_ANYDATAMATCH),
            4 => Some(MATCFG_A::ENABLED_DATAMATCH_100),
            5 => Some(MATCFG_A::ENABLED_DATAMATCH_101),
            6 => Some(MATCFG_A::ENABLED_DATAMATCH_110),
            7 => Some(MATCFG_A::ENABLED_DATAMATCH_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MATCFG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED_FIRSTDATAMATCH`"]
    #[inline(always)]
    pub fn is_enabled_firstdatamatch(&self) -> bool {
        *self == MATCFG_A::ENABLED_FIRSTDATAMATCH
    }
    #[doc = "Checks if the value of the field is `ENABLED_ANYDATAMATCH`"]
    #[inline(always)]
    pub fn is_enabled_anydatamatch(&self) -> bool {
        *self == MATCFG_A::ENABLED_ANYDATAMATCH
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_100`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_100(&self) -> bool {
        *self == MATCFG_A::ENABLED_DATAMATCH_100
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_101`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_101(&self) -> bool {
        *self == MATCFG_A::ENABLED_DATAMATCH_101
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_110`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_110(&self) -> bool {
        *self == MATCFG_A::ENABLED_DATAMATCH_110
    }
    #[doc = "Checks if the value of the field is `ENABLED_DATAMATCH_111`"]
    #[inline(always)]
    pub fn is_enabled_datamatch_111(&self) -> bool {
        *self == MATCFG_A::ENABLED_DATAMATCH_111
    }
}
#[doc = "Field `MATCFG` writer - Match Configuration"]
pub type MATCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, MATCFG_A, 3, O>;
impl<'a, const O: u8> MATCFG_W<'a, O> {
    #[doc = "Match is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MATCFG_A::DISABLED)
    }
    #[doc = "Match is enabled is 1st data word is MATCH0 or MATCH1"]
    #[inline(always)]
    pub fn enabled_firstdatamatch(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_FIRSTDATAMATCH)
    }
    #[doc = "Match is enabled on any data word equal MATCH0 or MATCH1"]
    #[inline(always)]
    pub fn enabled_anydatamatch(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_ANYDATAMATCH)
    }
    #[doc = "Match is enabled on data match sequence"]
    #[inline(always)]
    pub fn enabled_datamatch_100(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_100)
    }
    #[doc = "Match is enabled on data match sequence"]
    #[inline(always)]
    pub fn enabled_datamatch_101(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_101)
    }
    #[doc = "Match is enabled"]
    #[inline(always)]
    pub fn enabled_datamatch_110(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_110)
    }
    #[doc = "Match is enabled"]
    #[inline(always)]
    pub fn enabled_datamatch_111(self) -> &'a mut W {
        self.variant(MATCFG_A::ENABLED_DATAMATCH_111)
    }
}
#[doc = "Field `PINCFG` reader - Pin Configuration"]
pub type PINCFG_R = crate::FieldReader<u8, PINCFG_A>;
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: SIN is used for input data and SOUT is used for output data"]
    SIN_IN_SOUT_OUT = 0,
    #[doc = "1: SIN is used for both input and output data, only half-duplex serial transfers are supported"]
    SIN_BOTH_IN_OUT = 1,
    #[doc = "2: SOUT is used for both input and output data, only half-duplex serial transfers are supported"]
    SOUT_BOTH_IN_OUT = 2,
    #[doc = "3: SOUT is used for input data and SIN is used for output data"]
    SOUT_IN_SIN_OUT = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
impl PINCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::SIN_IN_SOUT_OUT,
            1 => PINCFG_A::SIN_BOTH_IN_OUT,
            2 => PINCFG_A::SOUT_BOTH_IN_OUT,
            3 => PINCFG_A::SOUT_IN_SIN_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIN_IN_SOUT_OUT`"]
    #[inline(always)]
    pub fn is_sin_in_sout_out(&self) -> bool {
        *self == PINCFG_A::SIN_IN_SOUT_OUT
    }
    #[doc = "Checks if the value of the field is `SIN_BOTH_IN_OUT`"]
    #[inline(always)]
    pub fn is_sin_both_in_out(&self) -> bool {
        *self == PINCFG_A::SIN_BOTH_IN_OUT
    }
    #[doc = "Checks if the value of the field is `SOUT_BOTH_IN_OUT`"]
    #[inline(always)]
    pub fn is_sout_both_in_out(&self) -> bool {
        *self == PINCFG_A::SOUT_BOTH_IN_OUT
    }
    #[doc = "Checks if the value of the field is `SOUT_IN_SIN_OUT`"]
    #[inline(always)]
    pub fn is_sout_in_sin_out(&self) -> bool {
        *self == PINCFG_A::SOUT_IN_SIN_OUT
    }
}
#[doc = "Field `PINCFG` writer - Pin Configuration"]
pub type PINCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, PINCFG_A, 2, O>;
impl<'a, const O: u8> PINCFG_W<'a, O> {
    #[doc = "SIN is used for input data and SOUT is used for output data"]
    #[inline(always)]
    pub fn sin_in_sout_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SIN_IN_SOUT_OUT)
    }
    #[doc = "SIN is used for both input and output data, only half-duplex serial transfers are supported"]
    #[inline(always)]
    pub fn sin_both_in_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SIN_BOTH_IN_OUT)
    }
    #[doc = "SOUT is used for both input and output data, only half-duplex serial transfers are supported"]
    #[inline(always)]
    pub fn sout_both_in_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SOUT_BOTH_IN_OUT)
    }
    #[doc = "SOUT is used for input data and SIN is used for output data"]
    #[inline(always)]
    pub fn sout_in_sin_out(self) -> &'a mut W {
        self.variant(PINCFG_A::SOUT_IN_SIN_OUT)
    }
}
#[doc = "Field `OUTCFG` reader - Output Configuration"]
pub type OUTCFG_R = crate::BitReader<OUTCFG_A>;
#[doc = "Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTCFG_A {
    #[doc = "0: Output data retains last value when chip select is negated"]
    RETAIN_LASTVALUE = 0,
    #[doc = "1: Output data is tristated when chip select is negated"]
    TRISTATED = 1,
}
impl From<OUTCFG_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCFG_A {
        match self.bits {
            false => OUTCFG_A::RETAIN_LASTVALUE,
            true => OUTCFG_A::TRISTATED,
        }
    }
    #[doc = "Checks if the value of the field is `RETAIN_LASTVALUE`"]
    #[inline(always)]
    pub fn is_retain_lastvalue(&self) -> bool {
        *self == OUTCFG_A::RETAIN_LASTVALUE
    }
    #[doc = "Checks if the value of the field is `TRISTATED`"]
    #[inline(always)]
    pub fn is_tristated(&self) -> bool {
        *self == OUTCFG_A::TRISTATED
    }
}
#[doc = "Field `OUTCFG` writer - Output Configuration"]
pub type OUTCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, OUTCFG_A, O>;
impl<'a, const O: u8> OUTCFG_W<'a, O> {
    #[doc = "Output data retains last value when chip select is negated"]
    #[inline(always)]
    pub fn retain_lastvalue(self) -> &'a mut W {
        self.variant(OUTCFG_A::RETAIN_LASTVALUE)
    }
    #[doc = "Output data is tristated when chip select is negated"]
    #[inline(always)]
    pub fn tristated(self) -> &'a mut W {
        self.variant(OUTCFG_A::TRISTATED)
    }
}
#[doc = "Field `PCSCFG` reader - Peripheral Chip Select Configuration"]
pub type PCSCFG_R = crate::BitReader<PCSCFG_A>;
#[doc = "Peripheral Chip Select Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSCFG_A {
    #[doc = "0: PCS\\[3:2\\]
are configured for chip select function"]
    CHIP_SELECT = 0,
    #[doc = "1: PCS\\[3:2\\]
are configured for half-duplex 4-bit transfers (PCS\\[3:2\\]
= DATA\\[3:2\\])"]
    HALFDUPLEX4BIT = 1,
}
impl From<PCSCFG_A> for bool {
    #[inline(always)]
    fn from(variant: PCSCFG_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSCFG_A {
        match self.bits {
            false => PCSCFG_A::CHIP_SELECT,
            true => PCSCFG_A::HALFDUPLEX4BIT,
        }
    }
    #[doc = "Checks if the value of the field is `CHIP_SELECT`"]
    #[inline(always)]
    pub fn is_chip_select(&self) -> bool {
        *self == PCSCFG_A::CHIP_SELECT
    }
    #[doc = "Checks if the value of the field is `HALFDUPLEX4BIT`"]
    #[inline(always)]
    pub fn is_halfduplex4bit(&self) -> bool {
        *self == PCSCFG_A::HALFDUPLEX4BIT
    }
}
#[doc = "Field `PCSCFG` writer - Peripheral Chip Select Configuration"]
pub type PCSCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, PCSCFG_A, O>;
impl<'a, const O: u8> PCSCFG_W<'a, O> {
    #[doc = "PCS\\[3:2\\]
are configured for chip select function"]
    #[inline(always)]
    pub fn chip_select(self) -> &'a mut W {
        self.variant(PCSCFG_A::CHIP_SELECT)
    }
    #[doc = "PCS\\[3:2\\]
are configured for half-duplex 4-bit transfers (PCS\\[3:2\\]
= DATA\\[3:2\\])"]
    #[inline(always)]
    pub fn halfduplex4bit(self) -> &'a mut W {
        self.variant(PCSCFG_A::HALFDUPLEX4BIT)
    }
}
impl R {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    pub fn autopcs(&self) -> AUTOPCS_R {
        AUTOPCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    pub fn nostall(&self) -> NOSTALL_R {
        NOSTALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub fn pcspol(&self) -> PCSPOL_R {
        PCSPOL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Output Configuration"]
    #[inline(always)]
    pub fn outcfg(&self) -> OUTCFG_R {
        OUTCFG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub fn pcscfg(&self) -> PCSCFG_R {
        PCSCFG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SAMPLE_W<1> {
        SAMPLE_W::new(self)
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline(always)]
    #[must_use]
    pub fn autopcs(&mut self) -> AUTOPCS_W<2> {
        AUTOPCS_W::new(self)
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline(always)]
    #[must_use]
    pub fn nostall(&mut self) -> NOSTALL_W<3> {
        NOSTALL_W::new(self)
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pcspol(&mut self) -> PCSPOL_W<8> {
        PCSPOL_W::new(self)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn matcfg(&mut self) -> MATCFG_W<16> {
        MATCFG_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PINCFG_W<24> {
        PINCFG_W::new(self)
    }
    #[doc = "Bit 26 - Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn outcfg(&mut self) -> OUTCFG_W<26> {
        OUTCFG_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pcscfg(&mut self) -> PCSCFG_W<27> {
        PCSCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
