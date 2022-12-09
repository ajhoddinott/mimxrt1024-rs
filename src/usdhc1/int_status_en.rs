#[doc = "Register `INT_STATUS_EN` reader"]
pub struct R(crate::R<INT_STATUS_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS_EN` writer"]
pub struct W(crate::W<INT_STATUS_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_EN_SPEC>;
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
impl From<crate::W<INT_STATUS_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCSEN` reader - Command complete status enable"]
pub type CCSEN_R = crate::BitReader<CCSEN_A>;
#[doc = "Command complete status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCSEN_A {
    #[doc = "0: Masked"]
    CCSEN_0 = 0,
    #[doc = "1: Enabled"]
    CCSEN_1 = 1,
}
impl From<CCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCSEN_A {
        match self.bits {
            false => CCSEN_A::CCSEN_0,
            true => CCSEN_A::CCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCSEN_0`"]
    #[inline(always)]
    pub fn is_ccsen_0(&self) -> bool {
        *self == CCSEN_A::CCSEN_0
    }
    #[doc = "Checks if the value of the field is `CCSEN_1`"]
    #[inline(always)]
    pub fn is_ccsen_1(&self) -> bool {
        *self == CCSEN_A::CCSEN_1
    }
}
#[doc = "Field `CCSEN` writer - Command complete status enable"]
pub type CCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CCSEN_A, O>;
impl<'a, const O: u8> CCSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccsen_0(self) -> &'a mut W {
        self.variant(CCSEN_A::CCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccsen_1(self) -> &'a mut W {
        self.variant(CCSEN_A::CCSEN_1)
    }
}
#[doc = "Field `TCSEN` reader - Transfer complete status enable"]
pub type TCSEN_R = crate::BitReader<TCSEN_A>;
#[doc = "Transfer complete status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSEN_A {
    #[doc = "0: Masked"]
    TCSEN_0 = 0,
    #[doc = "1: Enabled"]
    TCSEN_1 = 1,
}
impl From<TCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSEN_A {
        match self.bits {
            false => TCSEN_A::TCSEN_0,
            true => TCSEN_A::TCSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCSEN_0`"]
    #[inline(always)]
    pub fn is_tcsen_0(&self) -> bool {
        *self == TCSEN_A::TCSEN_0
    }
    #[doc = "Checks if the value of the field is `TCSEN_1`"]
    #[inline(always)]
    pub fn is_tcsen_1(&self) -> bool {
        *self == TCSEN_A::TCSEN_1
    }
}
#[doc = "Field `TCSEN` writer - Transfer complete status enable"]
pub type TCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, TCSEN_A, O>;
impl<'a, const O: u8> TCSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tcsen_0(self) -> &'a mut W {
        self.variant(TCSEN_A::TCSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tcsen_1(self) -> &'a mut W {
        self.variant(TCSEN_A::TCSEN_1)
    }
}
#[doc = "Field `BGESEN` reader - Block gap event status enable"]
pub type BGESEN_R = crate::BitReader<BGESEN_A>;
#[doc = "Block gap event status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGESEN_A {
    #[doc = "0: Masked"]
    BGESEN_0 = 0,
    #[doc = "1: Enabled"]
    BGESEN_1 = 1,
}
impl From<BGESEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BGESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGESEN_A {
        match self.bits {
            false => BGESEN_A::BGESEN_0,
            true => BGESEN_A::BGESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGESEN_0`"]
    #[inline(always)]
    pub fn is_bgesen_0(&self) -> bool {
        *self == BGESEN_A::BGESEN_0
    }
    #[doc = "Checks if the value of the field is `BGESEN_1`"]
    #[inline(always)]
    pub fn is_bgesen_1(&self) -> bool {
        *self == BGESEN_A::BGESEN_1
    }
}
#[doc = "Field `BGESEN` writer - Block gap event status enable"]
pub type BGESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, BGESEN_A, O>;
impl<'a, const O: u8> BGESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bgesen_0(self) -> &'a mut W {
        self.variant(BGESEN_A::BGESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bgesen_1(self) -> &'a mut W {
        self.variant(BGESEN_A::BGESEN_1)
    }
}
#[doc = "Field `DINTSEN` reader - DMA interrupt status enable"]
pub type DINTSEN_R = crate::BitReader<DINTSEN_A>;
#[doc = "DMA interrupt status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINTSEN_A {
    #[doc = "0: Masked"]
    DINTSEN_0 = 0,
    #[doc = "1: Enabled"]
    DINTSEN_1 = 1,
}
impl From<DINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DINTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTSEN_A {
        match self.bits {
            false => DINTSEN_A::DINTSEN_0,
            true => DINTSEN_A::DINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINTSEN_0`"]
    #[inline(always)]
    pub fn is_dintsen_0(&self) -> bool {
        *self == DINTSEN_A::DINTSEN_0
    }
    #[doc = "Checks if the value of the field is `DINTSEN_1`"]
    #[inline(always)]
    pub fn is_dintsen_1(&self) -> bool {
        *self == DINTSEN_A::DINTSEN_1
    }
}
#[doc = "Field `DINTSEN` writer - DMA interrupt status enable"]
pub type DINTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, DINTSEN_A, O>;
impl<'a, const O: u8> DINTSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dintsen_0(self) -> &'a mut W {
        self.variant(DINTSEN_A::DINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dintsen_1(self) -> &'a mut W {
        self.variant(DINTSEN_A::DINTSEN_1)
    }
}
#[doc = "Field `BWRSEN` reader - Buffer write ready status enable"]
pub type BWRSEN_R = crate::BitReader<BWRSEN_A>;
#[doc = "Buffer write ready status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRSEN_A {
    #[doc = "0: Masked"]
    BWRSEN_0 = 0,
    #[doc = "1: Enabled"]
    BWRSEN_1 = 1,
}
impl From<BWRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRSEN_A {
        match self.bits {
            false => BWRSEN_A::BWRSEN_0,
            true => BWRSEN_A::BWRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWRSEN_0`"]
    #[inline(always)]
    pub fn is_bwrsen_0(&self) -> bool {
        *self == BWRSEN_A::BWRSEN_0
    }
    #[doc = "Checks if the value of the field is `BWRSEN_1`"]
    #[inline(always)]
    pub fn is_bwrsen_1(&self) -> bool {
        *self == BWRSEN_A::BWRSEN_1
    }
}
#[doc = "Field `BWRSEN` writer - Buffer write ready status enable"]
pub type BWRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, BWRSEN_A, O>;
impl<'a, const O: u8> BWRSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn bwrsen_0(self) -> &'a mut W {
        self.variant(BWRSEN_A::BWRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn bwrsen_1(self) -> &'a mut W {
        self.variant(BWRSEN_A::BWRSEN_1)
    }
}
#[doc = "Field `BRRSEN` reader - Buffer read ready status enable"]
pub type BRRSEN_R = crate::BitReader<BRRSEN_A>;
#[doc = "Buffer read ready status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRSEN_A {
    #[doc = "0: Masked"]
    BRRSEN_0 = 0,
    #[doc = "1: Enabled"]
    BRRSEN_1 = 1,
}
impl From<BRRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRSEN_A {
        match self.bits {
            false => BRRSEN_A::BRRSEN_0,
            true => BRRSEN_A::BRRSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRRSEN_0`"]
    #[inline(always)]
    pub fn is_brrsen_0(&self) -> bool {
        *self == BRRSEN_A::BRRSEN_0
    }
    #[doc = "Checks if the value of the field is `BRRSEN_1`"]
    #[inline(always)]
    pub fn is_brrsen_1(&self) -> bool {
        *self == BRRSEN_A::BRRSEN_1
    }
}
#[doc = "Field `BRRSEN` writer - Buffer read ready status enable"]
pub type BRRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, BRRSEN_A, O>;
impl<'a, const O: u8> BRRSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn brrsen_0(self) -> &'a mut W {
        self.variant(BRRSEN_A::BRRSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn brrsen_1(self) -> &'a mut W {
        self.variant(BRRSEN_A::BRRSEN_1)
    }
}
#[doc = "Field `CINSSEN` reader - Card insertion status enable"]
pub type CINSSEN_R = crate::BitReader<CINSSEN_A>;
#[doc = "Card insertion status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSSEN_A {
    #[doc = "0: Masked"]
    CINSSEN_0 = 0,
    #[doc = "1: Enabled"]
    CINSSEN_1 = 1,
}
impl From<CINSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINSSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSSEN_A {
        match self.bits {
            false => CINSSEN_A::CINSSEN_0,
            true => CINSSEN_A::CINSSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINSSEN_0`"]
    #[inline(always)]
    pub fn is_cinssen_0(&self) -> bool {
        *self == CINSSEN_A::CINSSEN_0
    }
    #[doc = "Checks if the value of the field is `CINSSEN_1`"]
    #[inline(always)]
    pub fn is_cinssen_1(&self) -> bool {
        *self == CINSSEN_A::CINSSEN_1
    }
}
#[doc = "Field `CINSSEN` writer - Card insertion status enable"]
pub type CINSSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CINSSEN_A, O>;
impl<'a, const O: u8> CINSSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cinssen_0(self) -> &'a mut W {
        self.variant(CINSSEN_A::CINSSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cinssen_1(self) -> &'a mut W {
        self.variant(CINSSEN_A::CINSSEN_1)
    }
}
#[doc = "Field `CRMSEN` reader - Card removal status enable"]
pub type CRMSEN_R = crate::BitReader<CRMSEN_A>;
#[doc = "Card removal status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMSEN_A {
    #[doc = "0: Masked"]
    CRMSEN_0 = 0,
    #[doc = "1: Enabled"]
    CRMSEN_1 = 1,
}
impl From<CRMSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRMSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMSEN_A {
        match self.bits {
            false => CRMSEN_A::CRMSEN_0,
            true => CRMSEN_A::CRMSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRMSEN_0`"]
    #[inline(always)]
    pub fn is_crmsen_0(&self) -> bool {
        *self == CRMSEN_A::CRMSEN_0
    }
    #[doc = "Checks if the value of the field is `CRMSEN_1`"]
    #[inline(always)]
    pub fn is_crmsen_1(&self) -> bool {
        *self == CRMSEN_A::CRMSEN_1
    }
}
#[doc = "Field `CRMSEN` writer - Card removal status enable"]
pub type CRMSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CRMSEN_A, O>;
impl<'a, const O: u8> CRMSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn crmsen_0(self) -> &'a mut W {
        self.variant(CRMSEN_A::CRMSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn crmsen_1(self) -> &'a mut W {
        self.variant(CRMSEN_A::CRMSEN_1)
    }
}
#[doc = "Field `CINTSEN` reader - Card interrupt status enable"]
pub type CINTSEN_R = crate::BitReader<CINTSEN_A>;
#[doc = "Card interrupt status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTSEN_A {
    #[doc = "0: Masked"]
    CINTSEN_0 = 0,
    #[doc = "1: Enabled"]
    CINTSEN_1 = 1,
}
impl From<CINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTSEN_A {
        match self.bits {
            false => CINTSEN_A::CINTSEN_0,
            true => CINTSEN_A::CINTSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINTSEN_0`"]
    #[inline(always)]
    pub fn is_cintsen_0(&self) -> bool {
        *self == CINTSEN_A::CINTSEN_0
    }
    #[doc = "Checks if the value of the field is `CINTSEN_1`"]
    #[inline(always)]
    pub fn is_cintsen_1(&self) -> bool {
        *self == CINTSEN_A::CINTSEN_1
    }
}
#[doc = "Field `CINTSEN` writer - Card interrupt status enable"]
pub type CINTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CINTSEN_A, O>;
impl<'a, const O: u8> CINTSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cintsen_0(self) -> &'a mut W {
        self.variant(CINTSEN_A::CINTSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cintsen_1(self) -> &'a mut W {
        self.variant(CINTSEN_A::CINTSEN_1)
    }
}
#[doc = "Field `RTESEN` reader - Re-tuning event status enable"]
pub type RTESEN_R = crate::BitReader<RTESEN_A>;
#[doc = "Re-tuning event status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTESEN_A {
    #[doc = "0: Masked"]
    RTESEN_0 = 0,
    #[doc = "1: Enabled"]
    RTESEN_1 = 1,
}
impl From<RTESEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTESEN_A {
        match self.bits {
            false => RTESEN_A::RTESEN_0,
            true => RTESEN_A::RTESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTESEN_0`"]
    #[inline(always)]
    pub fn is_rtesen_0(&self) -> bool {
        *self == RTESEN_A::RTESEN_0
    }
    #[doc = "Checks if the value of the field is `RTESEN_1`"]
    #[inline(always)]
    pub fn is_rtesen_1(&self) -> bool {
        *self == RTESEN_A::RTESEN_1
    }
}
#[doc = "Field `RTESEN` writer - Re-tuning event status enable"]
pub type RTESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, RTESEN_A, O>;
impl<'a, const O: u8> RTESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn rtesen_0(self) -> &'a mut W {
        self.variant(RTESEN_A::RTESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rtesen_1(self) -> &'a mut W {
        self.variant(RTESEN_A::RTESEN_1)
    }
}
#[doc = "Field `TPSEN` reader - Tuning pass status enable"]
pub type TPSEN_R = crate::BitReader<TPSEN_A>;
#[doc = "Tuning pass status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPSEN_A {
    #[doc = "0: Masked"]
    TPSEN_0 = 0,
    #[doc = "1: Enabled"]
    TPSEN_1 = 1,
}
impl From<TPSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TPSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TPSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPSEN_A {
        match self.bits {
            false => TPSEN_A::TPSEN_0,
            true => TPSEN_A::TPSEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TPSEN_0`"]
    #[inline(always)]
    pub fn is_tpsen_0(&self) -> bool {
        *self == TPSEN_A::TPSEN_0
    }
    #[doc = "Checks if the value of the field is `TPSEN_1`"]
    #[inline(always)]
    pub fn is_tpsen_1(&self) -> bool {
        *self == TPSEN_A::TPSEN_1
    }
}
#[doc = "Field `TPSEN` writer - Tuning pass status enable"]
pub type TPSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, TPSEN_A, O>;
impl<'a, const O: u8> TPSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tpsen_0(self) -> &'a mut W {
        self.variant(TPSEN_A::TPSEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tpsen_1(self) -> &'a mut W {
        self.variant(TPSEN_A::TPSEN_1)
    }
}
#[doc = "Field `CTOESEN` reader - Command timeout error status enable"]
pub type CTOESEN_R = crate::BitReader<CTOESEN_A>;
#[doc = "Command timeout error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOESEN_A {
    #[doc = "0: Masked"]
    CTOESEN_0 = 0,
    #[doc = "1: Enabled"]
    CTOESEN_1 = 1,
}
impl From<CTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOESEN_A {
        match self.bits {
            false => CTOESEN_A::CTOESEN_0,
            true => CTOESEN_A::CTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOESEN_0`"]
    #[inline(always)]
    pub fn is_ctoesen_0(&self) -> bool {
        *self == CTOESEN_A::CTOESEN_0
    }
    #[doc = "Checks if the value of the field is `CTOESEN_1`"]
    #[inline(always)]
    pub fn is_ctoesen_1(&self) -> bool {
        *self == CTOESEN_A::CTOESEN_1
    }
}
#[doc = "Field `CTOESEN` writer - Command timeout error status enable"]
pub type CTOESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CTOESEN_A, O>;
impl<'a, const O: u8> CTOESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ctoesen_0(self) -> &'a mut W {
        self.variant(CTOESEN_A::CTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ctoesen_1(self) -> &'a mut W {
        self.variant(CTOESEN_A::CTOESEN_1)
    }
}
#[doc = "Field `CCESEN` reader - Command CRC error status enable"]
pub type CCESEN_R = crate::BitReader<CCESEN_A>;
#[doc = "Command CRC error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCESEN_A {
    #[doc = "0: Masked"]
    CCESEN_0 = 0,
    #[doc = "1: Enabled"]
    CCESEN_1 = 1,
}
impl From<CCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCESEN_A {
        match self.bits {
            false => CCESEN_A::CCESEN_0,
            true => CCESEN_A::CCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCESEN_0`"]
    #[inline(always)]
    pub fn is_ccesen_0(&self) -> bool {
        *self == CCESEN_A::CCESEN_0
    }
    #[doc = "Checks if the value of the field is `CCESEN_1`"]
    #[inline(always)]
    pub fn is_ccesen_1(&self) -> bool {
        *self == CCESEN_A::CCESEN_1
    }
}
#[doc = "Field `CCESEN` writer - Command CRC error status enable"]
pub type CCESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CCESEN_A, O>;
impl<'a, const O: u8> CCESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ccesen_0(self) -> &'a mut W {
        self.variant(CCESEN_A::CCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ccesen_1(self) -> &'a mut W {
        self.variant(CCESEN_A::CCESEN_1)
    }
}
#[doc = "Field `CEBESEN` reader - Command end bit error status enable"]
pub type CEBESEN_R = crate::BitReader<CEBESEN_A>;
#[doc = "Command end bit error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBESEN_A {
    #[doc = "0: Masked"]
    CEBESEN_0 = 0,
    #[doc = "1: Enabled"]
    CEBESEN_1 = 1,
}
impl From<CEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBESEN_A {
        match self.bits {
            false => CEBESEN_A::CEBESEN_0,
            true => CEBESEN_A::CEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBESEN_0`"]
    #[inline(always)]
    pub fn is_cebesen_0(&self) -> bool {
        *self == CEBESEN_A::CEBESEN_0
    }
    #[doc = "Checks if the value of the field is `CEBESEN_1`"]
    #[inline(always)]
    pub fn is_cebesen_1(&self) -> bool {
        *self == CEBESEN_A::CEBESEN_1
    }
}
#[doc = "Field `CEBESEN` writer - Command end bit error status enable"]
pub type CEBESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CEBESEN_A, O>;
impl<'a, const O: u8> CEBESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn cebesen_0(self) -> &'a mut W {
        self.variant(CEBESEN_A::CEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cebesen_1(self) -> &'a mut W {
        self.variant(CEBESEN_A::CEBESEN_1)
    }
}
#[doc = "Field `CIESEN` reader - Command index error status enable"]
pub type CIESEN_R = crate::BitReader<CIESEN_A>;
#[doc = "Command index error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIESEN_A {
    #[doc = "0: Masked"]
    CIESEN_0 = 0,
    #[doc = "1: Enabled"]
    CIESEN_1 = 1,
}
impl From<CIESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CIESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIESEN_A {
        match self.bits {
            false => CIESEN_A::CIESEN_0,
            true => CIESEN_A::CIESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIESEN_0`"]
    #[inline(always)]
    pub fn is_ciesen_0(&self) -> bool {
        *self == CIESEN_A::CIESEN_0
    }
    #[doc = "Checks if the value of the field is `CIESEN_1`"]
    #[inline(always)]
    pub fn is_ciesen_1(&self) -> bool {
        *self == CIESEN_A::CIESEN_1
    }
}
#[doc = "Field `CIESEN` writer - Command index error status enable"]
pub type CIESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, CIESEN_A, O>;
impl<'a, const O: u8> CIESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ciesen_0(self) -> &'a mut W {
        self.variant(CIESEN_A::CIESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ciesen_1(self) -> &'a mut W {
        self.variant(CIESEN_A::CIESEN_1)
    }
}
#[doc = "Field `DTOESEN` reader - Data timeout error status enable"]
pub type DTOESEN_R = crate::BitReader<DTOESEN_A>;
#[doc = "Data timeout error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOESEN_A {
    #[doc = "0: Masked"]
    DTOESEN_0 = 0,
    #[doc = "1: Enabled"]
    DTOESEN_1 = 1,
}
impl From<DTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOESEN_A {
        match self.bits {
            false => DTOESEN_A::DTOESEN_0,
            true => DTOESEN_A::DTOESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOESEN_0`"]
    #[inline(always)]
    pub fn is_dtoesen_0(&self) -> bool {
        *self == DTOESEN_A::DTOESEN_0
    }
    #[doc = "Checks if the value of the field is `DTOESEN_1`"]
    #[inline(always)]
    pub fn is_dtoesen_1(&self) -> bool {
        *self == DTOESEN_A::DTOESEN_1
    }
}
#[doc = "Field `DTOESEN` writer - Data timeout error status enable"]
pub type DTOESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, DTOESEN_A, O>;
impl<'a, const O: u8> DTOESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dtoesen_0(self) -> &'a mut W {
        self.variant(DTOESEN_A::DTOESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dtoesen_1(self) -> &'a mut W {
        self.variant(DTOESEN_A::DTOESEN_1)
    }
}
#[doc = "Field `DCESEN` reader - Data CRC error status enable"]
pub type DCESEN_R = crate::BitReader<DCESEN_A>;
#[doc = "Data CRC error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCESEN_A {
    #[doc = "0: Masked"]
    DCESEN_0 = 0,
    #[doc = "1: Enabled"]
    DCESEN_1 = 1,
}
impl From<DCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCESEN_A {
        match self.bits {
            false => DCESEN_A::DCESEN_0,
            true => DCESEN_A::DCESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCESEN_0`"]
    #[inline(always)]
    pub fn is_dcesen_0(&self) -> bool {
        *self == DCESEN_A::DCESEN_0
    }
    #[doc = "Checks if the value of the field is `DCESEN_1`"]
    #[inline(always)]
    pub fn is_dcesen_1(&self) -> bool {
        *self == DCESEN_A::DCESEN_1
    }
}
#[doc = "Field `DCESEN` writer - Data CRC error status enable"]
pub type DCESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, DCESEN_A, O>;
impl<'a, const O: u8> DCESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dcesen_0(self) -> &'a mut W {
        self.variant(DCESEN_A::DCESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcesen_1(self) -> &'a mut W {
        self.variant(DCESEN_A::DCESEN_1)
    }
}
#[doc = "Field `DEBESEN` reader - Data end bit error status enable"]
pub type DEBESEN_R = crate::BitReader<DEBESEN_A>;
#[doc = "Data end bit error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBESEN_A {
    #[doc = "0: Masked"]
    DEBESEN_0 = 0,
    #[doc = "1: Enabled"]
    DEBESEN_1 = 1,
}
impl From<DEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBESEN_A {
        match self.bits {
            false => DEBESEN_A::DEBESEN_0,
            true => DEBESEN_A::DEBESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBESEN_0`"]
    #[inline(always)]
    pub fn is_debesen_0(&self) -> bool {
        *self == DEBESEN_A::DEBESEN_0
    }
    #[doc = "Checks if the value of the field is `DEBESEN_1`"]
    #[inline(always)]
    pub fn is_debesen_1(&self) -> bool {
        *self == DEBESEN_A::DEBESEN_1
    }
}
#[doc = "Field `DEBESEN` writer - Data end bit error status enable"]
pub type DEBESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, DEBESEN_A, O>;
impl<'a, const O: u8> DEBESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn debesen_0(self) -> &'a mut W {
        self.variant(DEBESEN_A::DEBESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn debesen_1(self) -> &'a mut W {
        self.variant(DEBESEN_A::DEBESEN_1)
    }
}
#[doc = "Field `AC12ESEN` reader - Auto CMD12 error status enable"]
pub type AC12ESEN_R = crate::BitReader<AC12ESEN_A>;
#[doc = "Auto CMD12 error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12ESEN_A {
    #[doc = "0: Masked"]
    AC12ESEN_0 = 0,
    #[doc = "1: Enabled"]
    AC12ESEN_1 = 1,
}
impl From<AC12ESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12ESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12ESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12ESEN_A {
        match self.bits {
            false => AC12ESEN_A::AC12ESEN_0,
            true => AC12ESEN_A::AC12ESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_0`"]
    #[inline(always)]
    pub fn is_ac12esen_0(&self) -> bool {
        *self == AC12ESEN_A::AC12ESEN_0
    }
    #[doc = "Checks if the value of the field is `AC12ESEN_1`"]
    #[inline(always)]
    pub fn is_ac12esen_1(&self) -> bool {
        *self == AC12ESEN_A::AC12ESEN_1
    }
}
#[doc = "Field `AC12ESEN` writer - Auto CMD12 error status enable"]
pub type AC12ESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, AC12ESEN_A, O>;
impl<'a, const O: u8> AC12ESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn ac12esen_0(self) -> &'a mut W {
        self.variant(AC12ESEN_A::AC12ESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ac12esen_1(self) -> &'a mut W {
        self.variant(AC12ESEN_A::AC12ESEN_1)
    }
}
#[doc = "Field `TNESEN` reader - Tuning error status enable"]
pub type TNESEN_R = crate::BitReader<TNESEN_A>;
#[doc = "Tuning error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNESEN_A {
    #[doc = "0: Masked"]
    TNESEN_0 = 0,
    #[doc = "1: Enabled"]
    TNESEN_1 = 1,
}
impl From<TNESEN_A> for bool {
    #[inline(always)]
    fn from(variant: TNESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TNESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNESEN_A {
        match self.bits {
            false => TNESEN_A::TNESEN_0,
            true => TNESEN_A::TNESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TNESEN_0`"]
    #[inline(always)]
    pub fn is_tnesen_0(&self) -> bool {
        *self == TNESEN_A::TNESEN_0
    }
    #[doc = "Checks if the value of the field is `TNESEN_1`"]
    #[inline(always)]
    pub fn is_tnesen_1(&self) -> bool {
        *self == TNESEN_A::TNESEN_1
    }
}
#[doc = "Field `TNESEN` writer - Tuning error status enable"]
pub type TNESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, TNESEN_A, O>;
impl<'a, const O: u8> TNESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn tnesen_0(self) -> &'a mut W {
        self.variant(TNESEN_A::TNESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tnesen_1(self) -> &'a mut W {
        self.variant(TNESEN_A::TNESEN_1)
    }
}
#[doc = "Field `DMAESEN` reader - DMA error status enable"]
pub type DMAESEN_R = crate::BitReader<DMAESEN_A>;
#[doc = "DMA error status enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAESEN_A {
    #[doc = "0: Masked"]
    DMAESEN_0 = 0,
    #[doc = "1: Enabled"]
    DMAESEN_1 = 1,
}
impl From<DMAESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAESEN_A {
        match self.bits {
            false => DMAESEN_A::DMAESEN_0,
            true => DMAESEN_A::DMAESEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAESEN_0`"]
    #[inline(always)]
    pub fn is_dmaesen_0(&self) -> bool {
        *self == DMAESEN_A::DMAESEN_0
    }
    #[doc = "Checks if the value of the field is `DMAESEN_1`"]
    #[inline(always)]
    pub fn is_dmaesen_1(&self) -> bool {
        *self == DMAESEN_A::DMAESEN_1
    }
}
#[doc = "Field `DMAESEN` writer - DMA error status enable"]
pub type DMAESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STATUS_EN_SPEC, DMAESEN_A, O>;
impl<'a, const O: u8> DMAESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn dmaesen_0(self) -> &'a mut W {
        self.variant(DMAESEN_A::DMAESEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dmaesen_1(self) -> &'a mut W {
        self.variant(DMAESEN_A::DMAESEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command complete status enable"]
    #[inline(always)]
    pub fn ccsen(&self) -> CCSEN_R {
        CCSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete status enable"]
    #[inline(always)]
    pub fn tcsen(&self) -> TCSEN_R {
        TCSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block gap event status enable"]
    #[inline(always)]
    pub fn bgesen(&self) -> BGESEN_R {
        BGESEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt status enable"]
    #[inline(always)]
    pub fn dintsen(&self) -> DINTSEN_R {
        DINTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer write ready status enable"]
    #[inline(always)]
    pub fn bwrsen(&self) -> BWRSEN_R {
        BWRSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer read ready status enable"]
    #[inline(always)]
    pub fn brrsen(&self) -> BRRSEN_R {
        BRRSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card insertion status enable"]
    #[inline(always)]
    pub fn cinssen(&self) -> CINSSEN_R {
        CINSSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card removal status enable"]
    #[inline(always)]
    pub fn crmsen(&self) -> CRMSEN_R {
        CRMSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card interrupt status enable"]
    #[inline(always)]
    pub fn cintsen(&self) -> CINTSEN_R {
        CINTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-tuning event status enable"]
    #[inline(always)]
    pub fn rtesen(&self) -> RTESEN_R {
        RTESEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning pass status enable"]
    #[inline(always)]
    pub fn tpsen(&self) -> TPSEN_R {
        TPSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout error status enable"]
    #[inline(always)]
    pub fn ctoesen(&self) -> CTOESEN_R {
        CTOESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error status enable"]
    #[inline(always)]
    pub fn ccesen(&self) -> CCESEN_R {
        CCESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error status enable"]
    #[inline(always)]
    pub fn cebesen(&self) -> CEBESEN_R {
        CEBESEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command index error status enable"]
    #[inline(always)]
    pub fn ciesen(&self) -> CIESEN_R {
        CIESEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout error status enable"]
    #[inline(always)]
    pub fn dtoesen(&self) -> DTOESEN_R {
        DTOESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error status enable"]
    #[inline(always)]
    pub fn dcesen(&self) -> DCESEN_R {
        DCESEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error status enable"]
    #[inline(always)]
    pub fn debesen(&self) -> DEBESEN_R {
        DEBESEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 error status enable"]
    #[inline(always)]
    pub fn ac12esen(&self) -> AC12ESEN_R {
        AC12ESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning error status enable"]
    #[inline(always)]
    pub fn tnesen(&self) -> TNESEN_R {
        TNESEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA error status enable"]
    #[inline(always)]
    pub fn dmaesen(&self) -> DMAESEN_R {
        DMAESEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command complete status enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccsen(&mut self) -> CCSEN_W<0> {
        CCSEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete status enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcsen(&mut self) -> TCSEN_W<1> {
        TCSEN_W::new(self)
    }
    #[doc = "Bit 2 - Block gap event status enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgesen(&mut self) -> BGESEN_W<2> {
        BGESEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA interrupt status enable"]
    #[inline(always)]
    #[must_use]
    pub fn dintsen(&mut self) -> DINTSEN_W<3> {
        DINTSEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer write ready status enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrsen(&mut self) -> BWRSEN_W<4> {
        BWRSEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer read ready status enable"]
    #[inline(always)]
    #[must_use]
    pub fn brrsen(&mut self) -> BRRSEN_W<5> {
        BRRSEN_W::new(self)
    }
    #[doc = "Bit 6 - Card insertion status enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinssen(&mut self) -> CINSSEN_W<6> {
        CINSSEN_W::new(self)
    }
    #[doc = "Bit 7 - Card removal status enable"]
    #[inline(always)]
    #[must_use]
    pub fn crmsen(&mut self) -> CRMSEN_W<7> {
        CRMSEN_W::new(self)
    }
    #[doc = "Bit 8 - Card interrupt status enable"]
    #[inline(always)]
    #[must_use]
    pub fn cintsen(&mut self) -> CINTSEN_W<8> {
        CINTSEN_W::new(self)
    }
    #[doc = "Bit 12 - Re-tuning event status enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtesen(&mut self) -> RTESEN_W<12> {
        RTESEN_W::new(self)
    }
    #[doc = "Bit 14 - Tuning pass status enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpsen(&mut self) -> TPSEN_W<14> {
        TPSEN_W::new(self)
    }
    #[doc = "Bit 16 - Command timeout error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctoesen(&mut self) -> CTOESEN_W<16> {
        CTOESEN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccesen(&mut self) -> CCESEN_W<17> {
        CCESEN_W::new(self)
    }
    #[doc = "Bit 18 - Command end bit error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn cebesen(&mut self) -> CEBESEN_W<18> {
        CEBESEN_W::new(self)
    }
    #[doc = "Bit 19 - Command index error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciesen(&mut self) -> CIESEN_W<19> {
        CIESEN_W::new(self)
    }
    #[doc = "Bit 20 - Data timeout error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoesen(&mut self) -> DTOESEN_W<20> {
        DTOESEN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcesen(&mut self) -> DCESEN_W<21> {
        DCESEN_W::new(self)
    }
    #[doc = "Bit 22 - Data end bit error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn debesen(&mut self) -> DEBESEN_W<22> {
        DEBESEN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12esen(&mut self) -> AC12ESEN_W<24> {
        AC12ESEN_W::new(self)
    }
    #[doc = "Bit 26 - Tuning error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn tnesen(&mut self) -> TNESEN_W<26> {
        TNESEN_W::new(self)
    }
    #[doc = "Bit 28 - DMA error status enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaesen(&mut self) -> DMAESEN_W<28> {
        DMAESEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status_en](index.html) module"]
pub struct INT_STATUS_EN_SPEC;
impl crate::RegisterSpec for INT_STATUS_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status_en::R](R) reader structure"]
impl crate::Readable for INT_STATUS_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status_en::W](W) writer structure"]
impl crate::Writable for INT_STATUS_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_STATUS_EN to value 0"]
impl crate::Resettable for INT_STATUS_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
