#[doc = "Register `SYS_CTRL` reader"]
pub struct R(crate::R<SYS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTRL` writer"]
pub struct W(crate::W<SYS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTRL_SPEC>;
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
impl From<crate::W<SYS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVS` reader - Divisor"]
pub type DVS_R = crate::FieldReader<u8, DVS_A>;
#[doc = "Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVS_A {
    #[doc = "0: Divide-by-1"]
    DVS_0 = 0,
    #[doc = "1: Divide-by-2"]
    DVS_1 = 1,
    #[doc = "14: Divide-by-15"]
    DVS_14 = 14,
    #[doc = "15: Divide-by-16"]
    DVS_15 = 15,
}
impl From<DVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DVS_A) -> Self {
        variant as _
    }
}
impl DVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DVS_A> {
        match self.bits {
            0 => Some(DVS_A::DVS_0),
            1 => Some(DVS_A::DVS_1),
            14 => Some(DVS_A::DVS_14),
            15 => Some(DVS_A::DVS_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DVS_0`"]
    #[inline(always)]
    pub fn is_dvs_0(&self) -> bool {
        *self == DVS_A::DVS_0
    }
    #[doc = "Checks if the value of the field is `DVS_1`"]
    #[inline(always)]
    pub fn is_dvs_1(&self) -> bool {
        *self == DVS_A::DVS_1
    }
    #[doc = "Checks if the value of the field is `DVS_14`"]
    #[inline(always)]
    pub fn is_dvs_14(&self) -> bool {
        *self == DVS_A::DVS_14
    }
    #[doc = "Checks if the value of the field is `DVS_15`"]
    #[inline(always)]
    pub fn is_dvs_15(&self) -> bool {
        *self == DVS_A::DVS_15
    }
}
#[doc = "Field `DVS` writer - Divisor"]
pub type DVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CTRL_SPEC, u8, DVS_A, 4, O>;
impl<'a, const O: u8> DVS_W<'a, O> {
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn dvs_0(self) -> &'a mut W {
        self.variant(DVS_A::DVS_0)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn dvs_1(self) -> &'a mut W {
        self.variant(DVS_A::DVS_1)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn dvs_14(self) -> &'a mut W {
        self.variant(DVS_A::DVS_14)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn dvs_15(self) -> &'a mut W {
        self.variant(DVS_A::DVS_15)
    }
}
#[doc = "Field `SDCLKFS` reader - SDCLK frequency select"]
pub type SDCLKFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDCLKFS` writer - SDCLK frequency select"]
pub type SDCLKFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DTOCV` reader - Data timeout counter value"]
pub type DTOCV_R = crate::FieldReader<u8, DTOCV_A>;
#[doc = "Data timeout counter value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTOCV_A {
    #[doc = "0: SDCLK x 2 14"]
    DTOCV_0 = 0,
    #[doc = "1: SDCLK x 2 15"]
    DTOCV_1 = 1,
    #[doc = "2: SDCLK x 2 16"]
    DTOCV_2 = 2,
    #[doc = "3: SDCLK x 2 17"]
    DTOCV_3 = 3,
    #[doc = "4: SDCLK x 2 18"]
    DTOCV_4 = 4,
    #[doc = "5: SDCLK x 2 19"]
    DTOCV_5 = 5,
    #[doc = "6: SDCLK x 2 20"]
    DTOCV_6 = 6,
    #[doc = "7: SDCLK x 2 21"]
    DTOCV_7 = 7,
    #[doc = "8: SDCLK x 2 22"]
    DTOCV_8 = 8,
    #[doc = "9: SDCLK x 2 23"]
    DTOCV_9 = 9,
    #[doc = "10: SDCLK x 2 24"]
    DTOCV_10 = 10,
    #[doc = "11: SDCLK x 2 25"]
    DTOCV_11 = 11,
    #[doc = "12: SDCLK x 2 26"]
    DTOCV_12 = 12,
    #[doc = "13: SDCLK x 2 27"]
    DTOCV_13 = 13,
    #[doc = "14: SDCLK x 2 28"]
    DTOCV_14 = 14,
    #[doc = "15: SDCLK x 2 29"]
    DTOCV_15 = 15,
}
impl From<DTOCV_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_A) -> Self {
        variant as _
    }
}
impl DTOCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOCV_A {
        match self.bits {
            0 => DTOCV_A::DTOCV_0,
            1 => DTOCV_A::DTOCV_1,
            2 => DTOCV_A::DTOCV_2,
            3 => DTOCV_A::DTOCV_3,
            4 => DTOCV_A::DTOCV_4,
            5 => DTOCV_A::DTOCV_5,
            6 => DTOCV_A::DTOCV_6,
            7 => DTOCV_A::DTOCV_7,
            8 => DTOCV_A::DTOCV_8,
            9 => DTOCV_A::DTOCV_9,
            10 => DTOCV_A::DTOCV_10,
            11 => DTOCV_A::DTOCV_11,
            12 => DTOCV_A::DTOCV_12,
            13 => DTOCV_A::DTOCV_13,
            14 => DTOCV_A::DTOCV_14,
            15 => DTOCV_A::DTOCV_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_0`"]
    #[inline(always)]
    pub fn is_dtocv_0(&self) -> bool {
        *self == DTOCV_A::DTOCV_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_1`"]
    #[inline(always)]
    pub fn is_dtocv_1(&self) -> bool {
        *self == DTOCV_A::DTOCV_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_2`"]
    #[inline(always)]
    pub fn is_dtocv_2(&self) -> bool {
        *self == DTOCV_A::DTOCV_2
    }
    #[doc = "Checks if the value of the field is `DTOCV_3`"]
    #[inline(always)]
    pub fn is_dtocv_3(&self) -> bool {
        *self == DTOCV_A::DTOCV_3
    }
    #[doc = "Checks if the value of the field is `DTOCV_4`"]
    #[inline(always)]
    pub fn is_dtocv_4(&self) -> bool {
        *self == DTOCV_A::DTOCV_4
    }
    #[doc = "Checks if the value of the field is `DTOCV_5`"]
    #[inline(always)]
    pub fn is_dtocv_5(&self) -> bool {
        *self == DTOCV_A::DTOCV_5
    }
    #[doc = "Checks if the value of the field is `DTOCV_6`"]
    #[inline(always)]
    pub fn is_dtocv_6(&self) -> bool {
        *self == DTOCV_A::DTOCV_6
    }
    #[doc = "Checks if the value of the field is `DTOCV_7`"]
    #[inline(always)]
    pub fn is_dtocv_7(&self) -> bool {
        *self == DTOCV_A::DTOCV_7
    }
    #[doc = "Checks if the value of the field is `DTOCV_8`"]
    #[inline(always)]
    pub fn is_dtocv_8(&self) -> bool {
        *self == DTOCV_A::DTOCV_8
    }
    #[doc = "Checks if the value of the field is `DTOCV_9`"]
    #[inline(always)]
    pub fn is_dtocv_9(&self) -> bool {
        *self == DTOCV_A::DTOCV_9
    }
    #[doc = "Checks if the value of the field is `DTOCV_10`"]
    #[inline(always)]
    pub fn is_dtocv_10(&self) -> bool {
        *self == DTOCV_A::DTOCV_10
    }
    #[doc = "Checks if the value of the field is `DTOCV_11`"]
    #[inline(always)]
    pub fn is_dtocv_11(&self) -> bool {
        *self == DTOCV_A::DTOCV_11
    }
    #[doc = "Checks if the value of the field is `DTOCV_12`"]
    #[inline(always)]
    pub fn is_dtocv_12(&self) -> bool {
        *self == DTOCV_A::DTOCV_12
    }
    #[doc = "Checks if the value of the field is `DTOCV_13`"]
    #[inline(always)]
    pub fn is_dtocv_13(&self) -> bool {
        *self == DTOCV_A::DTOCV_13
    }
    #[doc = "Checks if the value of the field is `DTOCV_14`"]
    #[inline(always)]
    pub fn is_dtocv_14(&self) -> bool {
        *self == DTOCV_A::DTOCV_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_15`"]
    #[inline(always)]
    pub fn is_dtocv_15(&self) -> bool {
        *self == DTOCV_A::DTOCV_15
    }
}
#[doc = "Field `DTOCV` writer - Data timeout counter value"]
pub type DTOCV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SYS_CTRL_SPEC, u8, DTOCV_A, 4, O>;
impl<'a, const O: u8> DTOCV_W<'a, O> {
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn dtocv_0(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_0)
    }
    #[doc = "SDCLK x 2 15"]
    #[inline(always)]
    pub fn dtocv_1(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_1)
    }
    #[doc = "SDCLK x 2 16"]
    #[inline(always)]
    pub fn dtocv_2(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_2)
    }
    #[doc = "SDCLK x 2 17"]
    #[inline(always)]
    pub fn dtocv_3(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_3)
    }
    #[doc = "SDCLK x 2 18"]
    #[inline(always)]
    pub fn dtocv_4(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_4)
    }
    #[doc = "SDCLK x 2 19"]
    #[inline(always)]
    pub fn dtocv_5(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_5)
    }
    #[doc = "SDCLK x 2 20"]
    #[inline(always)]
    pub fn dtocv_6(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_6)
    }
    #[doc = "SDCLK x 2 21"]
    #[inline(always)]
    pub fn dtocv_7(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_7)
    }
    #[doc = "SDCLK x 2 22"]
    #[inline(always)]
    pub fn dtocv_8(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_8)
    }
    #[doc = "SDCLK x 2 23"]
    #[inline(always)]
    pub fn dtocv_9(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_9)
    }
    #[doc = "SDCLK x 2 24"]
    #[inline(always)]
    pub fn dtocv_10(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_10)
    }
    #[doc = "SDCLK x 2 25"]
    #[inline(always)]
    pub fn dtocv_11(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_11)
    }
    #[doc = "SDCLK x 2 26"]
    #[inline(always)]
    pub fn dtocv_12(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_12)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn dtocv_13(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_13)
    }
    #[doc = "SDCLK x 2 28"]
    #[inline(always)]
    pub fn dtocv_14(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_14)
    }
    #[doc = "SDCLK x 2 29"]
    #[inline(always)]
    pub fn dtocv_15(self) -> &'a mut W {
        self.variant(DTOCV_A::DTOCV_15)
    }
}
#[doc = "Field `IPP_RST_N` reader - Hardware reset"]
pub type IPP_RST_N_R = crate::BitReader<bool>;
#[doc = "Field `IPP_RST_N` writer - Hardware reset"]
pub type IPP_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `RSTA` reader - Software reset for all"]
pub type RSTA_R = crate::BitReader<RSTA_A>;
#[doc = "Software reset for all\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTA_A {
    #[doc = "0: No reset"]
    RSTA_0 = 0,
    #[doc = "1: Reset"]
    RSTA_1 = 1,
}
impl From<RSTA_A> for bool {
    #[inline(always)]
    fn from(variant: RSTA_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTA_A {
        match self.bits {
            false => RSTA_A::RSTA_0,
            true => RSTA_A::RSTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTA_0`"]
    #[inline(always)]
    pub fn is_rsta_0(&self) -> bool {
        *self == RSTA_A::RSTA_0
    }
    #[doc = "Checks if the value of the field is `RSTA_1`"]
    #[inline(always)]
    pub fn is_rsta_1(&self) -> bool {
        *self == RSTA_A::RSTA_1
    }
}
#[doc = "Field `RSTA` writer - Software reset for all"]
pub type RSTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, RSTA_A, O>;
impl<'a, const O: u8> RSTA_W<'a, O> {
    #[doc = "No reset"]
    #[inline(always)]
    pub fn rsta_0(self) -> &'a mut W {
        self.variant(RSTA_A::RSTA_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rsta_1(self) -> &'a mut W {
        self.variant(RSTA_A::RSTA_1)
    }
}
#[doc = "Field `RSTC` reader - Software reset for CMD line"]
pub type RSTC_R = crate::BitReader<RSTC_A>;
#[doc = "Software reset for CMD line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTC_A {
    #[doc = "0: No reset"]
    RSTC_0 = 0,
    #[doc = "1: Reset"]
    RSTC_1 = 1,
}
impl From<RSTC_A> for bool {
    #[inline(always)]
    fn from(variant: RSTC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTC_A {
        match self.bits {
            false => RSTC_A::RSTC_0,
            true => RSTC_A::RSTC_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTC_0`"]
    #[inline(always)]
    pub fn is_rstc_0(&self) -> bool {
        *self == RSTC_A::RSTC_0
    }
    #[doc = "Checks if the value of the field is `RSTC_1`"]
    #[inline(always)]
    pub fn is_rstc_1(&self) -> bool {
        *self == RSTC_A::RSTC_1
    }
}
#[doc = "Field `RSTC` writer - Software reset for CMD line"]
pub type RSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, RSTC_A, O>;
impl<'a, const O: u8> RSTC_W<'a, O> {
    #[doc = "No reset"]
    #[inline(always)]
    pub fn rstc_0(self) -> &'a mut W {
        self.variant(RSTC_A::RSTC_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstc_1(self) -> &'a mut W {
        self.variant(RSTC_A::RSTC_1)
    }
}
#[doc = "Field `RSTD` reader - Software reset for data line"]
pub type RSTD_R = crate::BitReader<RSTD_A>;
#[doc = "Software reset for data line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTD_A {
    #[doc = "0: No reset"]
    RSTD_0 = 0,
    #[doc = "1: Reset"]
    RSTD_1 = 1,
}
impl From<RSTD_A> for bool {
    #[inline(always)]
    fn from(variant: RSTD_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTD_A {
        match self.bits {
            false => RSTD_A::RSTD_0,
            true => RSTD_A::RSTD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTD_0`"]
    #[inline(always)]
    pub fn is_rstd_0(&self) -> bool {
        *self == RSTD_A::RSTD_0
    }
    #[doc = "Checks if the value of the field is `RSTD_1`"]
    #[inline(always)]
    pub fn is_rstd_1(&self) -> bool {
        *self == RSTD_A::RSTD_1
    }
}
#[doc = "Field `RSTD` writer - Software reset for data line"]
pub type RSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, RSTD_A, O>;
impl<'a, const O: u8> RSTD_W<'a, O> {
    #[doc = "No reset"]
    #[inline(always)]
    pub fn rstd_0(self) -> &'a mut W {
        self.variant(RSTD_A::RSTD_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn rstd_1(self) -> &'a mut W {
        self.variant(RSTD_A::RSTD_1)
    }
}
#[doc = "Field `INITA` reader - Initialization active"]
pub type INITA_R = crate::BitReader<bool>;
#[doc = "Field `INITA` writer - Initialization active"]
pub type INITA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `RSTT` reader - Reset tuning"]
pub type RSTT_R = crate::BitReader<bool>;
#[doc = "Field `RSTT` writer - Reset tuning"]
pub type RSTT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK frequency select"]
    #[inline(always)]
    pub fn sdclkfs(&self) -> SDCLKFS_R {
        SDCLKFS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data timeout counter value"]
    #[inline(always)]
    pub fn dtocv(&self) -> DTOCV_R {
        DTOCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Hardware reset"]
    #[inline(always)]
    pub fn ipp_rst_n(&self) -> IPP_RST_N_R {
        IPP_RST_N_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Software reset for all"]
    #[inline(always)]
    pub fn rsta(&self) -> RSTA_R {
        RSTA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software reset for CMD line"]
    #[inline(always)]
    pub fn rstc(&self) -> RSTC_R {
        RSTC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software reset for data line"]
    #[inline(always)]
    pub fn rstd(&self) -> RSTD_R {
        RSTD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Initialization active"]
    #[inline(always)]
    pub fn inita(&self) -> INITA_R {
        INITA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset tuning"]
    #[inline(always)]
    pub fn rstt(&self) -> RSTT_R {
        RSTT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn dvs(&mut self) -> DVS_W<4> {
        DVS_W::new(self)
    }
    #[doc = "Bits 8:15 - SDCLK frequency select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfs(&mut self) -> SDCLKFS_W<8> {
        SDCLKFS_W::new(self)
    }
    #[doc = "Bits 16:19 - Data timeout counter value"]
    #[inline(always)]
    #[must_use]
    pub fn dtocv(&mut self) -> DTOCV_W<16> {
        DTOCV_W::new(self)
    }
    #[doc = "Bit 23 - Hardware reset"]
    #[inline(always)]
    #[must_use]
    pub fn ipp_rst_n(&mut self) -> IPP_RST_N_W<23> {
        IPP_RST_N_W::new(self)
    }
    #[doc = "Bit 24 - Software reset for all"]
    #[inline(always)]
    #[must_use]
    pub fn rsta(&mut self) -> RSTA_W<24> {
        RSTA_W::new(self)
    }
    #[doc = "Bit 25 - Software reset for CMD line"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<25> {
        RSTC_W::new(self)
    }
    #[doc = "Bit 26 - Software reset for data line"]
    #[inline(always)]
    #[must_use]
    pub fn rstd(&mut self) -> RSTD_W<26> {
        RSTD_W::new(self)
    }
    #[doc = "Bit 27 - Initialization active"]
    #[inline(always)]
    #[must_use]
    pub fn inita(&mut self) -> INITA_W<27> {
        INITA_W::new(self)
    }
    #[doc = "Bit 28 - Reset tuning"]
    #[inline(always)]
    #[must_use]
    pub fn rstt(&mut self) -> RSTT_W<28> {
        RSTT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctrl](index.html) module"]
pub struct SYS_CTRL_SPEC;
impl crate::RegisterSpec for SYS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctrl::R](R) reader structure"]
impl crate::Readable for SYS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctrl::W](W) writer structure"]
impl crate::Writable for SYS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0x0080_800f"]
impl crate::Resettable for SYS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_800f;
}
