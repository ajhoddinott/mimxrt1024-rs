#[doc = "Register `PROT_CTRL` reader"]
pub struct R(crate::R<PROT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROT_CTRL` writer"]
pub struct W(crate::W<PROT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROT_CTRL_SPEC>;
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
impl From<crate::W<PROT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTW` reader - Data transfer width"]
pub type DTW_R = crate::FieldReader<u8, DTW_A>;
#[doc = "Data transfer width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTW_A {
    #[doc = "0: 1-bit mode"]
    DTW_0 = 0,
    #[doc = "1: 4-bit mode"]
    DTW_1 = 1,
    #[doc = "2: 8-bit mode"]
    DTW_2 = 2,
}
impl From<DTW_A> for u8 {
    #[inline(always)]
    fn from(variant: DTW_A) -> Self {
        variant as _
    }
}
impl DTW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTW_A> {
        match self.bits {
            0 => Some(DTW_A::DTW_0),
            1 => Some(DTW_A::DTW_1),
            2 => Some(DTW_A::DTW_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DTW_0`"]
    #[inline(always)]
    pub fn is_dtw_0(&self) -> bool {
        *self == DTW_A::DTW_0
    }
    #[doc = "Checks if the value of the field is `DTW_1`"]
    #[inline(always)]
    pub fn is_dtw_1(&self) -> bool {
        *self == DTW_A::DTW_1
    }
    #[doc = "Checks if the value of the field is `DTW_2`"]
    #[inline(always)]
    pub fn is_dtw_2(&self) -> bool {
        *self == DTW_A::DTW_2
    }
}
#[doc = "Field `DTW` writer - Data transfer width"]
pub type DTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, DTW_A, 2, O>;
impl<'a, const O: u8> DTW_W<'a, O> {
    #[doc = "1-bit mode"]
    #[inline(always)]
    pub fn dtw_0(self) -> &'a mut W {
        self.variant(DTW_A::DTW_0)
    }
    #[doc = "4-bit mode"]
    #[inline(always)]
    pub fn dtw_1(self) -> &'a mut W {
        self.variant(DTW_A::DTW_1)
    }
    #[doc = "8-bit mode"]
    #[inline(always)]
    pub fn dtw_2(self) -> &'a mut W {
        self.variant(DTW_A::DTW_2)
    }
}
#[doc = "Field `D3CD` reader - DATA3 as card detection pin"]
pub type D3CD_R = crate::BitReader<D3CD_A>;
#[doc = "DATA3 as card detection pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D3CD_A {
    #[doc = "0: DATA3 does not monitor card insertion"]
    D3CD_0 = 0,
    #[doc = "1: DATA3 as card detection pin"]
    D3CD_1 = 1,
}
impl From<D3CD_A> for bool {
    #[inline(always)]
    fn from(variant: D3CD_A) -> Self {
        variant as u8 != 0
    }
}
impl D3CD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D3CD_A {
        match self.bits {
            false => D3CD_A::D3CD_0,
            true => D3CD_A::D3CD_1,
        }
    }
    #[doc = "Checks if the value of the field is `D3CD_0`"]
    #[inline(always)]
    pub fn is_d3cd_0(&self) -> bool {
        *self == D3CD_A::D3CD_0
    }
    #[doc = "Checks if the value of the field is `D3CD_1`"]
    #[inline(always)]
    pub fn is_d3cd_1(&self) -> bool {
        *self == D3CD_A::D3CD_1
    }
}
#[doc = "Field `D3CD` writer - DATA3 as card detection pin"]
pub type D3CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, D3CD_A, O>;
impl<'a, const O: u8> D3CD_W<'a, O> {
    #[doc = "DATA3 does not monitor card insertion"]
    #[inline(always)]
    pub fn d3cd_0(self) -> &'a mut W {
        self.variant(D3CD_A::D3CD_0)
    }
    #[doc = "DATA3 as card detection pin"]
    #[inline(always)]
    pub fn d3cd_1(self) -> &'a mut W {
        self.variant(D3CD_A::D3CD_1)
    }
}
#[doc = "Field `EMODE` reader - Endian mode"]
pub type EMODE_R = crate::FieldReader<u8, EMODE_A>;
#[doc = "Endian mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMODE_A {
    #[doc = "0: Big endian mode"]
    EMODE_0 = 0,
    #[doc = "1: Half word big endian mode"]
    EMODE_1 = 1,
    #[doc = "2: Little endian mode"]
    EMODE_2 = 2,
}
impl From<EMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as _
    }
}
impl EMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMODE_A> {
        match self.bits {
            0 => Some(EMODE_A::EMODE_0),
            1 => Some(EMODE_A::EMODE_1),
            2 => Some(EMODE_A::EMODE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMODE_0`"]
    #[inline(always)]
    pub fn is_emode_0(&self) -> bool {
        *self == EMODE_A::EMODE_0
    }
    #[doc = "Checks if the value of the field is `EMODE_1`"]
    #[inline(always)]
    pub fn is_emode_1(&self) -> bool {
        *self == EMODE_A::EMODE_1
    }
    #[doc = "Checks if the value of the field is `EMODE_2`"]
    #[inline(always)]
    pub fn is_emode_2(&self) -> bool {
        *self == EMODE_A::EMODE_2
    }
}
#[doc = "Field `EMODE` writer - Endian mode"]
pub type EMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, EMODE_A, 2, O>;
impl<'a, const O: u8> EMODE_W<'a, O> {
    #[doc = "Big endian mode"]
    #[inline(always)]
    pub fn emode_0(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_0)
    }
    #[doc = "Half word big endian mode"]
    #[inline(always)]
    pub fn emode_1(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_1)
    }
    #[doc = "Little endian mode"]
    #[inline(always)]
    pub fn emode_2(self) -> &'a mut W {
        self.variant(EMODE_A::EMODE_2)
    }
}
#[doc = "Field `DMASEL` reader - DMA select"]
pub type DMASEL_R = crate::FieldReader<u8, DMASEL_A>;
#[doc = "DMA select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMASEL_A {
    #[doc = "0: No DMA or simple DMA is selected."]
    DMASEL_0 = 0,
    #[doc = "1: ADMA1 is selected."]
    DMASEL_1 = 1,
    #[doc = "2: ADMA2 is selected."]
    DMASEL_2 = 2,
}
impl From<DMASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMASEL_A) -> Self {
        variant as _
    }
}
impl DMASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMASEL_A> {
        match self.bits {
            0 => Some(DMASEL_A::DMASEL_0),
            1 => Some(DMASEL_A::DMASEL_1),
            2 => Some(DMASEL_A::DMASEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMASEL_0`"]
    #[inline(always)]
    pub fn is_dmasel_0(&self) -> bool {
        *self == DMASEL_A::DMASEL_0
    }
    #[doc = "Checks if the value of the field is `DMASEL_1`"]
    #[inline(always)]
    pub fn is_dmasel_1(&self) -> bool {
        *self == DMASEL_A::DMASEL_1
    }
    #[doc = "Checks if the value of the field is `DMASEL_2`"]
    #[inline(always)]
    pub fn is_dmasel_2(&self) -> bool {
        *self == DMASEL_A::DMASEL_2
    }
}
#[doc = "Field `DMASEL` writer - DMA select"]
pub type DMASEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, DMASEL_A, 2, O>;
impl<'a, const O: u8> DMASEL_W<'a, O> {
    #[doc = "No DMA or simple DMA is selected."]
    #[inline(always)]
    pub fn dmasel_0(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_0)
    }
    #[doc = "ADMA1 is selected."]
    #[inline(always)]
    pub fn dmasel_1(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_1)
    }
    #[doc = "ADMA2 is selected."]
    #[inline(always)]
    pub fn dmasel_2(self) -> &'a mut W {
        self.variant(DMASEL_A::DMASEL_2)
    }
}
#[doc = "Field `SABGREQ` reader - Stop at block gap request"]
pub type SABGREQ_R = crate::BitReader<SABGREQ_A>;
#[doc = "Stop at block gap request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SABGREQ_A {
    #[doc = "0: Transfer"]
    SABGREQ_0 = 0,
    #[doc = "1: Stop"]
    SABGREQ_1 = 1,
}
impl From<SABGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SABGREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SABGREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SABGREQ_A {
        match self.bits {
            false => SABGREQ_A::SABGREQ_0,
            true => SABGREQ_A::SABGREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `SABGREQ_0`"]
    #[inline(always)]
    pub fn is_sabgreq_0(&self) -> bool {
        *self == SABGREQ_A::SABGREQ_0
    }
    #[doc = "Checks if the value of the field is `SABGREQ_1`"]
    #[inline(always)]
    pub fn is_sabgreq_1(&self) -> bool {
        *self == SABGREQ_A::SABGREQ_1
    }
}
#[doc = "Field `SABGREQ` writer - Stop at block gap request"]
pub type SABGREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, SABGREQ_A, O>;
impl<'a, const O: u8> SABGREQ_W<'a, O> {
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn sabgreq_0(self) -> &'a mut W {
        self.variant(SABGREQ_A::SABGREQ_0)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn sabgreq_1(self) -> &'a mut W {
        self.variant(SABGREQ_A::SABGREQ_1)
    }
}
#[doc = "Field `CREQ` reader - Continue request"]
pub type CREQ_R = crate::BitReader<CREQ_A>;
#[doc = "Continue request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREQ_A {
    #[doc = "0: No effect"]
    CREQ_0 = 0,
    #[doc = "1: Restart"]
    CREQ_1 = 1,
}
impl From<CREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CREQ_A {
        match self.bits {
            false => CREQ_A::CREQ_0,
            true => CREQ_A::CREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CREQ_0`"]
    #[inline(always)]
    pub fn is_creq_0(&self) -> bool {
        *self == CREQ_A::CREQ_0
    }
    #[doc = "Checks if the value of the field is `CREQ_1`"]
    #[inline(always)]
    pub fn is_creq_1(&self) -> bool {
        *self == CREQ_A::CREQ_1
    }
}
#[doc = "Field `CREQ` writer - Continue request"]
pub type CREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, CREQ_A, O>;
impl<'a, const O: u8> CREQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn creq_0(self) -> &'a mut W {
        self.variant(CREQ_A::CREQ_0)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn creq_1(self) -> &'a mut W {
        self.variant(CREQ_A::CREQ_1)
    }
}
#[doc = "Field `RWCTL` reader - Read wait control"]
pub type RWCTL_R = crate::BitReader<RWCTL_A>;
#[doc = "Read wait control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWCTL_A {
    #[doc = "0: Disables read wait control and stop SD clock at block gap when SABGREQ field is set"]
    RWCTL_0 = 0,
    #[doc = "1: Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set"]
    RWCTL_1 = 1,
}
impl From<RWCTL_A> for bool {
    #[inline(always)]
    fn from(variant: RWCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl RWCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWCTL_A {
        match self.bits {
            false => RWCTL_A::RWCTL_0,
            true => RWCTL_A::RWCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RWCTL_0`"]
    #[inline(always)]
    pub fn is_rwctl_0(&self) -> bool {
        *self == RWCTL_A::RWCTL_0
    }
    #[doc = "Checks if the value of the field is `RWCTL_1`"]
    #[inline(always)]
    pub fn is_rwctl_1(&self) -> bool {
        *self == RWCTL_A::RWCTL_1
    }
}
#[doc = "Field `RWCTL` writer - Read wait control"]
pub type RWCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, RWCTL_A, O>;
impl<'a, const O: u8> RWCTL_W<'a, O> {
    #[doc = "Disables read wait control and stop SD clock at block gap when SABGREQ field is set"]
    #[inline(always)]
    pub fn rwctl_0(self) -> &'a mut W {
        self.variant(RWCTL_A::RWCTL_0)
    }
    #[doc = "Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set"]
    #[inline(always)]
    pub fn rwctl_1(self) -> &'a mut W {
        self.variant(RWCTL_A::RWCTL_1)
    }
}
#[doc = "Field `IABG` reader - Interrupt at block gap"]
pub type IABG_R = crate::BitReader<IABG_A>;
#[doc = "Interrupt at block gap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IABG_A {
    #[doc = "0: Disables interrupt at block gap"]
    IABG_0 = 0,
    #[doc = "1: Enables interrupt at block gap"]
    IABG_1 = 1,
}
impl From<IABG_A> for bool {
    #[inline(always)]
    fn from(variant: IABG_A) -> Self {
        variant as u8 != 0
    }
}
impl IABG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IABG_A {
        match self.bits {
            false => IABG_A::IABG_0,
            true => IABG_A::IABG_1,
        }
    }
    #[doc = "Checks if the value of the field is `IABG_0`"]
    #[inline(always)]
    pub fn is_iabg_0(&self) -> bool {
        *self == IABG_A::IABG_0
    }
    #[doc = "Checks if the value of the field is `IABG_1`"]
    #[inline(always)]
    pub fn is_iabg_1(&self) -> bool {
        *self == IABG_A::IABG_1
    }
}
#[doc = "Field `IABG` writer - Interrupt at block gap"]
pub type IABG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, IABG_A, O>;
impl<'a, const O: u8> IABG_W<'a, O> {
    #[doc = "Disables interrupt at block gap"]
    #[inline(always)]
    pub fn iabg_0(self) -> &'a mut W {
        self.variant(IABG_A::IABG_0)
    }
    #[doc = "Enables interrupt at block gap"]
    #[inline(always)]
    pub fn iabg_1(self) -> &'a mut W {
        self.variant(IABG_A::IABG_1)
    }
}
#[doc = "Field `RD_DONE_NO_8CLK` reader - Read performed number 8 clock"]
pub type RD_DONE_NO_8CLK_R = crate::BitReader<bool>;
#[doc = "Field `RD_DONE_NO_8CLK` writer - Read performed number 8 clock"]
pub type RD_DONE_NO_8CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `WECINT` reader - Wakeup event enable on card interrupt"]
pub type WECINT_R = crate::BitReader<WECINT_A>;
#[doc = "Wakeup event enable on card interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECINT_A {
    #[doc = "0: Disables wakeup event enable on card interrupt"]
    WECINT_0 = 0,
    #[doc = "1: Enables wakeup event enable on card interrupt"]
    WECINT_1 = 1,
}
impl From<WECINT_A> for bool {
    #[inline(always)]
    fn from(variant: WECINT_A) -> Self {
        variant as u8 != 0
    }
}
impl WECINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINT_A {
        match self.bits {
            false => WECINT_A::WECINT_0,
            true => WECINT_A::WECINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINT_0`"]
    #[inline(always)]
    pub fn is_wecint_0(&self) -> bool {
        *self == WECINT_A::WECINT_0
    }
    #[doc = "Checks if the value of the field is `WECINT_1`"]
    #[inline(always)]
    pub fn is_wecint_1(&self) -> bool {
        *self == WECINT_A::WECINT_1
    }
}
#[doc = "Field `WECINT` writer - Wakeup event enable on card interrupt"]
pub type WECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, WECINT_A, O>;
impl<'a, const O: u8> WECINT_W<'a, O> {
    #[doc = "Disables wakeup event enable on card interrupt"]
    #[inline(always)]
    pub fn wecint_0(self) -> &'a mut W {
        self.variant(WECINT_A::WECINT_0)
    }
    #[doc = "Enables wakeup event enable on card interrupt"]
    #[inline(always)]
    pub fn wecint_1(self) -> &'a mut W {
        self.variant(WECINT_A::WECINT_1)
    }
}
#[doc = "Field `WECINS` reader - Wakeup event enable on SD card insertion"]
pub type WECINS_R = crate::BitReader<WECINS_A>;
#[doc = "Wakeup event enable on SD card insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECINS_A {
    #[doc = "0: Disable wakeup event enable on SD card insertion"]
    WECINS_0 = 0,
    #[doc = "1: Enable wakeup event enable on SD card insertion"]
    WECINS_1 = 1,
}
impl From<WECINS_A> for bool {
    #[inline(always)]
    fn from(variant: WECINS_A) -> Self {
        variant as u8 != 0
    }
}
impl WECINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECINS_A {
        match self.bits {
            false => WECINS_A::WECINS_0,
            true => WECINS_A::WECINS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECINS_0`"]
    #[inline(always)]
    pub fn is_wecins_0(&self) -> bool {
        *self == WECINS_A::WECINS_0
    }
    #[doc = "Checks if the value of the field is `WECINS_1`"]
    #[inline(always)]
    pub fn is_wecins_1(&self) -> bool {
        *self == WECINS_A::WECINS_1
    }
}
#[doc = "Field `WECINS` writer - Wakeup event enable on SD card insertion"]
pub type WECINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, WECINS_A, O>;
impl<'a, const O: u8> WECINS_W<'a, O> {
    #[doc = "Disable wakeup event enable on SD card insertion"]
    #[inline(always)]
    pub fn wecins_0(self) -> &'a mut W {
        self.variant(WECINS_A::WECINS_0)
    }
    #[doc = "Enable wakeup event enable on SD card insertion"]
    #[inline(always)]
    pub fn wecins_1(self) -> &'a mut W {
        self.variant(WECINS_A::WECINS_1)
    }
}
#[doc = "Field `WECRM` reader - Wakeup event enable on SD card removal"]
pub type WECRM_R = crate::BitReader<WECRM_A>;
#[doc = "Wakeup event enable on SD card removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WECRM_A {
    #[doc = "0: Disables wakeup event enable on SD card removal"]
    WECRM_0 = 0,
    #[doc = "1: Enables wakeup event enable on SD card removal"]
    WECRM_1 = 1,
}
impl From<WECRM_A> for bool {
    #[inline(always)]
    fn from(variant: WECRM_A) -> Self {
        variant as u8 != 0
    }
}
impl WECRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WECRM_A {
        match self.bits {
            false => WECRM_A::WECRM_0,
            true => WECRM_A::WECRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `WECRM_0`"]
    #[inline(always)]
    pub fn is_wecrm_0(&self) -> bool {
        *self == WECRM_A::WECRM_0
    }
    #[doc = "Checks if the value of the field is `WECRM_1`"]
    #[inline(always)]
    pub fn is_wecrm_1(&self) -> bool {
        *self == WECRM_A::WECRM_1
    }
}
#[doc = "Field `WECRM` writer - Wakeup event enable on SD card removal"]
pub type WECRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, WECRM_A, O>;
impl<'a, const O: u8> WECRM_W<'a, O> {
    #[doc = "Disables wakeup event enable on SD card removal"]
    #[inline(always)]
    pub fn wecrm_0(self) -> &'a mut W {
        self.variant(WECRM_A::WECRM_0)
    }
    #[doc = "Enables wakeup event enable on SD card removal"]
    #[inline(always)]
    pub fn wecrm_1(self) -> &'a mut W {
        self.variant(WECRM_A::WECRM_1)
    }
}
#[doc = "Field `BURST_LEN_EN` reader - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
pub type BURST_LEN_EN_R = crate::FieldReader<u8, BURST_LEN_EN_A>;
#[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURST_LEN_EN_A {
    #[doc = "1: Burst length is enabled for INCR."]
    BURST_LEN_EN_1 = 1,
}
impl From<BURST_LEN_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_LEN_EN_A) -> Self {
        variant as _
    }
}
impl BURST_LEN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BURST_LEN_EN_A> {
        match self.bits {
            1 => Some(BURST_LEN_EN_A::BURST_LEN_EN_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_LEN_EN_1`"]
    #[inline(always)]
    pub fn is_burst_len_en_1(&self) -> bool {
        *self == BURST_LEN_EN_A::BURST_LEN_EN_1
    }
}
#[doc = "Field `BURST_LEN_EN` writer - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
pub type BURST_LEN_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, BURST_LEN_EN_A, 3, O>;
impl<'a, const O: u8> BURST_LEN_EN_W<'a, O> {
    #[doc = "Burst length is enabled for INCR."]
    #[inline(always)]
    pub fn burst_len_en_1(self) -> &'a mut W {
        self.variant(BURST_LEN_EN_A::BURST_LEN_EN_1)
    }
}
#[doc = "Field `NON_EXACT_BLK_RD` reader - Non-exact block read"]
pub type NON_EXACT_BLK_RD_R = crate::BitReader<NON_EXACT_BLK_RD_A>;
#[doc = "Non-exact block read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NON_EXACT_BLK_RD_A {
    #[doc = "0: The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_0 = 0,
    #[doc = "1: The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_1 = 1,
}
impl From<NON_EXACT_BLK_RD_A> for bool {
    #[inline(always)]
    fn from(variant: NON_EXACT_BLK_RD_A) -> Self {
        variant as u8 != 0
    }
}
impl NON_EXACT_BLK_RD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NON_EXACT_BLK_RD_A {
        match self.bits {
            false => NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0,
            true => NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1,
        }
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_0`"]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_0(&self) -> bool {
        *self == NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0
    }
    #[doc = "Checks if the value of the field is `NON_EXACT_BLK_RD_1`"]
    #[inline(always)]
    pub fn is_non_exact_blk_rd_1(&self) -> bool {
        *self == NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1
    }
}
#[doc = "Field `NON_EXACT_BLK_RD` writer - Non-exact block read"]
pub type NON_EXACT_BLK_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PROT_CTRL_SPEC, NON_EXACT_BLK_RD_A, O>;
impl<'a, const O: u8> NON_EXACT_BLK_RD_W<'a, O> {
    #[doc = "The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_0(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_0)
    }
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    #[inline(always)]
    pub fn non_exact_blk_rd_1(self) -> &'a mut W {
        self.variant(NON_EXACT_BLK_RD_A::NON_EXACT_BLK_RD_1)
    }
}
impl R {
    #[doc = "Bits 1:2 - Data transfer width"]
    #[inline(always)]
    pub fn dtw(&self) -> DTW_R {
        DTW_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - DATA3 as card detection pin"]
    #[inline(always)]
    pub fn d3cd(&self) -> D3CD_R {
        D3CD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Endian mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DMA select"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Stop at block gap request"]
    #[inline(always)]
    pub fn sabgreq(&self) -> SABGREQ_R {
        SABGREQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue request"]
    #[inline(always)]
    pub fn creq(&self) -> CREQ_R {
        CREQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read wait control"]
    #[inline(always)]
    pub fn rwctl(&self) -> RWCTL_R {
        RWCTL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt at block gap"]
    #[inline(always)]
    pub fn iabg(&self) -> IABG_R {
        IABG_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Read performed number 8 clock"]
    #[inline(always)]
    pub fn rd_done_no_8clk(&self) -> RD_DONE_NO_8CLK_R {
        RD_DONE_NO_8CLK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup event enable on card interrupt"]
    #[inline(always)]
    pub fn wecint(&self) -> WECINT_R {
        WECINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup event enable on SD card insertion"]
    #[inline(always)]
    pub fn wecins(&self) -> WECINS_R {
        WECINS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup event enable on SD card removal"]
    #[inline(always)]
    pub fn wecrm(&self) -> WECRM_R {
        WECRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub fn burst_len_en(&self) -> BURST_LEN_EN_R {
        BURST_LEN_EN_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Non-exact block read"]
    #[inline(always)]
    pub fn non_exact_blk_rd(&self) -> NON_EXACT_BLK_RD_R {
        NON_EXACT_BLK_RD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - Data transfer width"]
    #[inline(always)]
    #[must_use]
    pub fn dtw(&mut self) -> DTW_W<1> {
        DTW_W::new(self)
    }
    #[doc = "Bit 3 - DATA3 as card detection pin"]
    #[inline(always)]
    #[must_use]
    pub fn d3cd(&mut self) -> D3CD_W<3> {
        D3CD_W::new(self)
    }
    #[doc = "Bits 4:5 - Endian mode"]
    #[inline(always)]
    #[must_use]
    pub fn emode(&mut self) -> EMODE_W<4> {
        EMODE_W::new(self)
    }
    #[doc = "Bits 8:9 - DMA select"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<8> {
        DMASEL_W::new(self)
    }
    #[doc = "Bit 16 - Stop at block gap request"]
    #[inline(always)]
    #[must_use]
    pub fn sabgreq(&mut self) -> SABGREQ_W<16> {
        SABGREQ_W::new(self)
    }
    #[doc = "Bit 17 - Continue request"]
    #[inline(always)]
    #[must_use]
    pub fn creq(&mut self) -> CREQ_W<17> {
        CREQ_W::new(self)
    }
    #[doc = "Bit 18 - Read wait control"]
    #[inline(always)]
    #[must_use]
    pub fn rwctl(&mut self) -> RWCTL_W<18> {
        RWCTL_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn iabg(&mut self) -> IABG_W<19> {
        IABG_W::new(self)
    }
    #[doc = "Bit 20 - Read performed number 8 clock"]
    #[inline(always)]
    #[must_use]
    pub fn rd_done_no_8clk(&mut self) -> RD_DONE_NO_8CLK_W<20> {
        RD_DONE_NO_8CLK_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup event enable on card interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wecint(&mut self) -> WECINT_W<24> {
        WECINT_W::new(self)
    }
    #[doc = "Bit 25 - Wakeup event enable on SD card insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wecins(&mut self) -> WECINS_W<25> {
        WECINS_W::new(self)
    }
    #[doc = "Bit 26 - Wakeup event enable on SD card removal"]
    #[inline(always)]
    #[must_use]
    pub fn wecrm(&mut self) -> WECRM_W<26> {
        WECRM_W::new(self)
    }
    #[doc = "Bits 27:29 - BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    #[must_use]
    pub fn burst_len_en(&mut self) -> BURST_LEN_EN_W<27> {
        BURST_LEN_EN_W::new(self)
    }
    #[doc = "Bit 30 - Non-exact block read"]
    #[inline(always)]
    #[must_use]
    pub fn non_exact_blk_rd(&mut self) -> NON_EXACT_BLK_RD_W<30> {
        NON_EXACT_BLK_RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protocol Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prot_ctrl](index.html) module"]
pub struct PROT_CTRL_SPEC;
impl crate::RegisterSpec for PROT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prot_ctrl::R](R) reader structure"]
impl crate::Readable for PROT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prot_ctrl::W](W) writer structure"]
impl crate::Writable for PROT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROT_CTRL to value 0x0880_0020"]
impl crate::Resettable for PROT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0880_0020;
}
