#[doc = "Register `PRES_STATE` reader"]
pub struct R(crate::R<PRES_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRES_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRES_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRES_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIHB` reader - Command inhibit (CMD)"]
pub type CIHB_R = crate::BitReader<CIHB_A>;
#[doc = "Command inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIHB_A {
    #[doc = "0: Can issue command using only CMD line"]
    CIHB_0 = 0,
    #[doc = "1: Cannot issue command"]
    CIHB_1 = 1,
}
impl From<CIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CIHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CIHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIHB_A {
        match self.bits {
            false => CIHB_A::CIHB_0,
            true => CIHB_A::CIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIHB_0`"]
    #[inline(always)]
    pub fn is_cihb_0(&self) -> bool {
        *self == CIHB_A::CIHB_0
    }
    #[doc = "Checks if the value of the field is `CIHB_1`"]
    #[inline(always)]
    pub fn is_cihb_1(&self) -> bool {
        *self == CIHB_A::CIHB_1
    }
}
#[doc = "Field `CDIHB` reader - Command Inhibit Data (DATA)"]
pub type CDIHB_R = crate::BitReader<CDIHB_A>;
#[doc = "Command Inhibit Data (DATA)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDIHB_A {
    #[doc = "0: Can issue command that uses the DATA line"]
    CDIHB_0 = 0,
    #[doc = "1: Cannot issue command that uses the DATA line"]
    CDIHB_1 = 1,
}
impl From<CDIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CDIHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CDIHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIHB_A {
        match self.bits {
            false => CDIHB_A::CDIHB_0,
            true => CDIHB_A::CDIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDIHB_0`"]
    #[inline(always)]
    pub fn is_cdihb_0(&self) -> bool {
        *self == CDIHB_A::CDIHB_0
    }
    #[doc = "Checks if the value of the field is `CDIHB_1`"]
    #[inline(always)]
    pub fn is_cdihb_1(&self) -> bool {
        *self == CDIHB_A::CDIHB_1
    }
}
#[doc = "Field `DLA` reader - Data line active"]
pub type DLA_R = crate::BitReader<DLA_A>;
#[doc = "Data line active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLA_A {
    #[doc = "0: DATA line inactive"]
    DLA_0 = 0,
    #[doc = "1: DATA line active"]
    DLA_1 = 1,
}
impl From<DLA_A> for bool {
    #[inline(always)]
    fn from(variant: DLA_A) -> Self {
        variant as u8 != 0
    }
}
impl DLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLA_A {
        match self.bits {
            false => DLA_A::DLA_0,
            true => DLA_A::DLA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DLA_0`"]
    #[inline(always)]
    pub fn is_dla_0(&self) -> bool {
        *self == DLA_A::DLA_0
    }
    #[doc = "Checks if the value of the field is `DLA_1`"]
    #[inline(always)]
    pub fn is_dla_1(&self) -> bool {
        *self == DLA_A::DLA_1
    }
}
#[doc = "Field `SDSTB` reader - SD clock stable"]
pub type SDSTB_R = crate::BitReader<SDSTB_A>;
#[doc = "SD clock stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDSTB_A {
    #[doc = "0: Clock is changing frequency and not stable."]
    SDSTB_0 = 0,
    #[doc = "1: Clock is stable."]
    SDSTB_1 = 1,
}
impl From<SDSTB_A> for bool {
    #[inline(always)]
    fn from(variant: SDSTB_A) -> Self {
        variant as u8 != 0
    }
}
impl SDSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDSTB_A {
        match self.bits {
            false => SDSTB_A::SDSTB_0,
            true => SDSTB_A::SDSTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDSTB_0`"]
    #[inline(always)]
    pub fn is_sdstb_0(&self) -> bool {
        *self == SDSTB_A::SDSTB_0
    }
    #[doc = "Checks if the value of the field is `SDSTB_1`"]
    #[inline(always)]
    pub fn is_sdstb_1(&self) -> bool {
        *self == SDSTB_A::SDSTB_1
    }
}
#[doc = "Field `IPGOFF` reader - Peripheral clock gated off internally"]
pub type IPGOFF_R = crate::BitReader<IPGOFF_A>;
#[doc = "Peripheral clock gated off internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPGOFF_A {
    #[doc = "0: Peripheral clock is active."]
    IPGOFF_0 = 0,
    #[doc = "1: Peripheral clock is gated off."]
    IPGOFF_1 = 1,
}
impl From<IPGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: IPGOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl IPGOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGOFF_A {
        match self.bits {
            false => IPGOFF_A::IPGOFF_0,
            true => IPGOFF_A::IPGOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPGOFF_0`"]
    #[inline(always)]
    pub fn is_ipgoff_0(&self) -> bool {
        *self == IPGOFF_A::IPGOFF_0
    }
    #[doc = "Checks if the value of the field is `IPGOFF_1`"]
    #[inline(always)]
    pub fn is_ipgoff_1(&self) -> bool {
        *self == IPGOFF_A::IPGOFF_1
    }
}
#[doc = "Field `HCKOFF` reader - HCLK gated off internally"]
pub type HCKOFF_R = crate::BitReader<HCKOFF_A>;
#[doc = "HCLK gated off internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCKOFF_A {
    #[doc = "0: HCLK is active."]
    HCKOFF_0 = 0,
    #[doc = "1: HCLK is gated off."]
    HCKOFF_1 = 1,
}
impl From<HCKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: HCKOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl HCKOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKOFF_A {
        match self.bits {
            false => HCKOFF_A::HCKOFF_0,
            true => HCKOFF_A::HCKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCKOFF_0`"]
    #[inline(always)]
    pub fn is_hckoff_0(&self) -> bool {
        *self == HCKOFF_A::HCKOFF_0
    }
    #[doc = "Checks if the value of the field is `HCKOFF_1`"]
    #[inline(always)]
    pub fn is_hckoff_1(&self) -> bool {
        *self == HCKOFF_A::HCKOFF_1
    }
}
#[doc = "Field `PEROFF` reader - IPG_PERCLK gated off internally"]
pub type PEROFF_R = crate::BitReader<PEROFF_A>;
#[doc = "IPG_PERCLK gated off internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEROFF_A {
    #[doc = "0: IPG_PERCLK is active."]
    PEROFF_0 = 0,
    #[doc = "1: IPG_PERCLK is gated off."]
    PEROFF_1 = 1,
}
impl From<PEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: PEROFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PEROFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEROFF_A {
        match self.bits {
            false => PEROFF_A::PEROFF_0,
            true => PEROFF_A::PEROFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEROFF_0`"]
    #[inline(always)]
    pub fn is_peroff_0(&self) -> bool {
        *self == PEROFF_A::PEROFF_0
    }
    #[doc = "Checks if the value of the field is `PEROFF_1`"]
    #[inline(always)]
    pub fn is_peroff_1(&self) -> bool {
        *self == PEROFF_A::PEROFF_1
    }
}
#[doc = "Field `SDOFF` reader - SD clock gated off internally"]
pub type SDOFF_R = crate::BitReader<SDOFF_A>;
#[doc = "SD clock gated off internally\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDOFF_A {
    #[doc = "0: SD clock is active."]
    SDOFF_0 = 0,
    #[doc = "1: SD clock is gated off."]
    SDOFF_1 = 1,
}
impl From<SDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SDOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SDOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOFF_A {
        match self.bits {
            false => SDOFF_A::SDOFF_0,
            true => SDOFF_A::SDOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDOFF_0`"]
    #[inline(always)]
    pub fn is_sdoff_0(&self) -> bool {
        *self == SDOFF_A::SDOFF_0
    }
    #[doc = "Checks if the value of the field is `SDOFF_1`"]
    #[inline(always)]
    pub fn is_sdoff_1(&self) -> bool {
        *self == SDOFF_A::SDOFF_1
    }
}
#[doc = "Field `WTA` reader - Write transfer active"]
pub type WTA_R = crate::BitReader<WTA_A>;
#[doc = "Write transfer active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WTA_A {
    #[doc = "0: No valid data"]
    WTA_0 = 0,
    #[doc = "1: Transferring data"]
    WTA_1 = 1,
}
impl From<WTA_A> for bool {
    #[inline(always)]
    fn from(variant: WTA_A) -> Self {
        variant as u8 != 0
    }
}
impl WTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTA_A {
        match self.bits {
            false => WTA_A::WTA_0,
            true => WTA_A::WTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTA_0`"]
    #[inline(always)]
    pub fn is_wta_0(&self) -> bool {
        *self == WTA_A::WTA_0
    }
    #[doc = "Checks if the value of the field is `WTA_1`"]
    #[inline(always)]
    pub fn is_wta_1(&self) -> bool {
        *self == WTA_A::WTA_1
    }
}
#[doc = "Field `RTA` reader - Read transfer active"]
pub type RTA_R = crate::BitReader<RTA_A>;
#[doc = "Read transfer active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTA_A {
    #[doc = "0: No valid data"]
    RTA_0 = 0,
    #[doc = "1: Transferring data"]
    RTA_1 = 1,
}
impl From<RTA_A> for bool {
    #[inline(always)]
    fn from(variant: RTA_A) -> Self {
        variant as u8 != 0
    }
}
impl RTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTA_A {
        match self.bits {
            false => RTA_A::RTA_0,
            true => RTA_A::RTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTA_0`"]
    #[inline(always)]
    pub fn is_rta_0(&self) -> bool {
        *self == RTA_A::RTA_0
    }
    #[doc = "Checks if the value of the field is `RTA_1`"]
    #[inline(always)]
    pub fn is_rta_1(&self) -> bool {
        *self == RTA_A::RTA_1
    }
}
#[doc = "Field `BWEN` reader - Buffer write enable"]
pub type BWEN_R = crate::BitReader<BWEN_A>;
#[doc = "Buffer write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWEN_A {
    #[doc = "0: Write disable"]
    BWEN_0 = 0,
    #[doc = "1: Write enable"]
    BWEN_1 = 1,
}
impl From<BWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWEN_A {
        match self.bits {
            false => BWEN_A::BWEN_0,
            true => BWEN_A::BWEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWEN_0`"]
    #[inline(always)]
    pub fn is_bwen_0(&self) -> bool {
        *self == BWEN_A::BWEN_0
    }
    #[doc = "Checks if the value of the field is `BWEN_1`"]
    #[inline(always)]
    pub fn is_bwen_1(&self) -> bool {
        *self == BWEN_A::BWEN_1
    }
}
#[doc = "Field `BREN` reader - Buffer read enable"]
pub type BREN_R = crate::BitReader<BREN_A>;
#[doc = "Buffer read enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN_A {
    #[doc = "0: Read disable"]
    BREN_0 = 0,
    #[doc = "1: Read enable"]
    BREN_1 = 1,
}
impl From<BREN_A> for bool {
    #[inline(always)]
    fn from(variant: BREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREN_A {
        match self.bits {
            false => BREN_A::BREN_0,
            true => BREN_A::BREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BREN_0`"]
    #[inline(always)]
    pub fn is_bren_0(&self) -> bool {
        *self == BREN_A::BREN_0
    }
    #[doc = "Checks if the value of the field is `BREN_1`"]
    #[inline(always)]
    pub fn is_bren_1(&self) -> bool {
        *self == BREN_A::BREN_1
    }
}
#[doc = "Field `RTR` reader - Re-Tuning Request (only for SD3.0 SDR104 mode,and EMMC HS200 mode)"]
pub type RTR_R = crate::BitReader<RTR_A>;
#[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode,and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTR_A {
    #[doc = "0: Fixed or well tuned sampling clock"]
    RTR_0 = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    RTR_1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::RTR_0,
            true => RTR_A::RTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTR_0`"]
    #[inline(always)]
    pub fn is_rtr_0(&self) -> bool {
        *self == RTR_A::RTR_0
    }
    #[doc = "Checks if the value of the field is `RTR_1`"]
    #[inline(always)]
    pub fn is_rtr_1(&self) -> bool {
        *self == RTR_A::RTR_1
    }
}
#[doc = "Field `TSCD` reader - Tap select change done"]
pub type TSCD_R = crate::BitReader<TSCD_A>;
#[doc = "Tap select change done\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCD_A {
    #[doc = "0: Delay cell select change is not finished."]
    TSCD_0 = 0,
    #[doc = "1: Delay cell select change is finished."]
    TSCD_1 = 1,
}
impl From<TSCD_A> for bool {
    #[inline(always)]
    fn from(variant: TSCD_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCD_A {
        match self.bits {
            false => TSCD_A::TSCD_0,
            true => TSCD_A::TSCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSCD_0`"]
    #[inline(always)]
    pub fn is_tscd_0(&self) -> bool {
        *self == TSCD_A::TSCD_0
    }
    #[doc = "Checks if the value of the field is `TSCD_1`"]
    #[inline(always)]
    pub fn is_tscd_1(&self) -> bool {
        *self == TSCD_A::TSCD_1
    }
}
#[doc = "Field `CINST` reader - Card inserted"]
pub type CINST_R = crate::BitReader<CINST_A>;
#[doc = "Card inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINST_A {
    #[doc = "0: Power on reset or no card"]
    CINST_0 = 0,
    #[doc = "1: Card inserted"]
    CINST_1 = 1,
}
impl From<CINST_A> for bool {
    #[inline(always)]
    fn from(variant: CINST_A) -> Self {
        variant as u8 != 0
    }
}
impl CINST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINST_A {
        match self.bits {
            false => CINST_A::CINST_0,
            true => CINST_A::CINST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINST_0`"]
    #[inline(always)]
    pub fn is_cinst_0(&self) -> bool {
        *self == CINST_A::CINST_0
    }
    #[doc = "Checks if the value of the field is `CINST_1`"]
    #[inline(always)]
    pub fn is_cinst_1(&self) -> bool {
        *self == CINST_A::CINST_1
    }
}
#[doc = "Field `CLSL` reader - CMD line signal level"]
pub type CLSL_R = crate::BitReader<bool>;
#[doc = "Field `DLSL` reader - DATA\\[7:0\\]
line signal level"]
pub type DLSL_R = crate::FieldReader<u8, DLSL_A>;
#[doc = "DATA\\[7:0\\]
line signal level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLSL_A {
    #[doc = "0: Data 0 line signal level"]
    DATA0 = 0,
    #[doc = "1: Data 1 line signal level"]
    DATA1 = 1,
    #[doc = "2: Data 2 line signal level"]
    DATA2 = 2,
    #[doc = "3: Data 3 line signal level"]
    DATA3 = 3,
    #[doc = "4: Data 4 line signal level"]
    DATA4 = 4,
    #[doc = "5: Data 5 line signal level"]
    DATA5 = 5,
    #[doc = "6: Data 6 line signal level"]
    DATA6 = 6,
    #[doc = "7: Data 7 line signal level"]
    DATA7 = 7,
}
impl From<DLSL_A> for u8 {
    #[inline(always)]
    fn from(variant: DLSL_A) -> Self {
        variant as _
    }
}
impl DLSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DLSL_A> {
        match self.bits {
            0 => Some(DLSL_A::DATA0),
            1 => Some(DLSL_A::DATA1),
            2 => Some(DLSL_A::DATA2),
            3 => Some(DLSL_A::DATA3),
            4 => Some(DLSL_A::DATA4),
            5 => Some(DLSL_A::DATA5),
            6 => Some(DLSL_A::DATA6),
            7 => Some(DLSL_A::DATA7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DLSL_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DLSL_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DLSL_A::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA3`"]
    #[inline(always)]
    pub fn is_data3(&self) -> bool {
        *self == DLSL_A::DATA3
    }
    #[doc = "Checks if the value of the field is `DATA4`"]
    #[inline(always)]
    pub fn is_data4(&self) -> bool {
        *self == DLSL_A::DATA4
    }
    #[doc = "Checks if the value of the field is `DATA5`"]
    #[inline(always)]
    pub fn is_data5(&self) -> bool {
        *self == DLSL_A::DATA5
    }
    #[doc = "Checks if the value of the field is `DATA6`"]
    #[inline(always)]
    pub fn is_data6(&self) -> bool {
        *self == DLSL_A::DATA6
    }
    #[doc = "Checks if the value of the field is `DATA7`"]
    #[inline(always)]
    pub fn is_data7(&self) -> bool {
        *self == DLSL_A::DATA7
    }
}
impl R {
    #[doc = "Bit 0 - Command inhibit (CMD)"]
    #[inline(always)]
    pub fn cihb(&self) -> CIHB_R {
        CIHB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit Data (DATA)"]
    #[inline(always)]
    pub fn cdihb(&self) -> CDIHB_R {
        CDIHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data line active"]
    #[inline(always)]
    pub fn dla(&self) -> DLA_R {
        DLA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD clock stable"]
    #[inline(always)]
    pub fn sdstb(&self) -> SDSTB_R {
        SDSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral clock gated off internally"]
    #[inline(always)]
    pub fn ipgoff(&self) -> IPGOFF_R {
        IPGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HCLK gated off internally"]
    #[inline(always)]
    pub fn hckoff(&self) -> HCKOFF_R {
        HCKOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IPG_PERCLK gated off internally"]
    #[inline(always)]
    pub fn peroff(&self) -> PEROFF_R {
        PEROFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD clock gated off internally"]
    #[inline(always)]
    pub fn sdoff(&self) -> SDOFF_R {
        SDOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write transfer active"]
    #[inline(always)]
    pub fn wta(&self) -> WTA_R {
        WTA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read transfer active"]
    #[inline(always)]
    pub fn rta(&self) -> RTA_R {
        RTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer write enable"]
    #[inline(always)]
    pub fn bwen(&self) -> BWEN_R {
        BWEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer read enable"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Request (only for SD3.0 SDR104 mode,and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Tap select change done"]
    #[inline(always)]
    pub fn tscd(&self) -> TSCD_R {
        TSCD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Card inserted"]
    #[inline(always)]
    pub fn cinst(&self) -> CINST_R {
        CINST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - CMD line signal level"]
    #[inline(always)]
    pub fn clsl(&self) -> CLSL_R {
        CLSL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - DATA\\[7:0\\]
line signal level"]
    #[inline(always)]
    pub fn dlsl(&self) -> DLSL_R {
        DLSL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Present State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pres_state](index.html) module"]
pub struct PRES_STATE_SPEC;
impl crate::RegisterSpec for PRES_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pres_state::R](R) reader structure"]
impl crate::Readable for PRES_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRES_STATE to value 0x8080"]
impl crate::Resettable for PRES_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x8080;
}
