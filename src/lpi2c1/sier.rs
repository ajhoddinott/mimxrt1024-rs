#[doc = "Register `SIER` reader"]
pub struct R(crate::R<SIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIER` writer"]
pub struct W(crate::W<SIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIER_SPEC>;
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
impl From<crate::W<SIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDIE` reader - Transmit Data Interrupt Enable"]
pub type TDIE_R = crate::BitReader<TDIE_A>;
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::DISABLED,
            true => TDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDIE_A::ENABLED
    }
}
#[doc = "Field `TDIE` writer - Transmit Data Interrupt Enable"]
pub type TDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, TDIE_A, O>;
impl<'a, const O: u8> TDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDIE_A::ENABLED)
    }
}
#[doc = "Field `RDIE` reader - Receive Data Interrupt Enable"]
pub type RDIE_R = crate::BitReader<RDIE_A>;
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::DISABLED,
            true => RDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDIE_A::ENABLED
    }
}
#[doc = "Field `RDIE` writer - Receive Data Interrupt Enable"]
pub type RDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, RDIE_A, O>;
impl<'a, const O: u8> RDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDIE_A::ENABLED)
    }
}
#[doc = "Field `AVIE` reader - Address Valid Interrupt Enable"]
pub type AVIE_R = crate::BitReader<AVIE_A>;
#[doc = "Address Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AVIE_A> for bool {
    #[inline(always)]
    fn from(variant: AVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVIE_A {
        match self.bits {
            false => AVIE_A::DISABLED,
            true => AVIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVIE_A::ENABLED
    }
}
#[doc = "Field `AVIE` writer - Address Valid Interrupt Enable"]
pub type AVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, AVIE_A, O>;
impl<'a, const O: u8> AVIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AVIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AVIE_A::ENABLED)
    }
}
#[doc = "Field `TAIE` reader - Transmit ACK Interrupt Enable"]
pub type TAIE_R = crate::BitReader<TAIE_A>;
#[doc = "Transmit ACK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::DISABLED,
            true => TAIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAIE_A::ENABLED
    }
}
#[doc = "Field `TAIE` writer - Transmit ACK Interrupt Enable"]
pub type TAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, TAIE_A, O>;
impl<'a, const O: u8> TAIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAIE_A::ENABLED)
    }
}
#[doc = "Field `RSIE` reader - Repeated Start Interrupt Enable"]
pub type RSIE_R = crate::BitReader<RSIE_A>;
#[doc = "Repeated Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<RSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIE_A {
        match self.bits {
            false => RSIE_A::DISABLED,
            true => RSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSIE_A::ENABLED
    }
}
#[doc = "Field `RSIE` writer - Repeated Start Interrupt Enable"]
pub type RSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, RSIE_A, O>;
impl<'a, const O: u8> RSIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSIE_A::ENABLED)
    }
}
#[doc = "Field `SDIE` reader - STOP Detect Interrupt Enable"]
pub type SDIE_R = crate::BitReader<SDIE_A>;
#[doc = "STOP Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIE_A {
        match self.bits {
            false => SDIE_A::DISABLED,
            true => SDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIE_A::ENABLED
    }
}
#[doc = "Field `SDIE` writer - STOP Detect Interrupt Enable"]
pub type SDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, SDIE_A, O>;
impl<'a, const O: u8> SDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIE_A::ENABLED)
    }
}
#[doc = "Field `BEIE` reader - Bit Error Interrupt Enable"]
pub type BEIE_R = crate::BitReader<BEIE_A>;
#[doc = "Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl BEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIE_A {
        match self.bits {
            false => BEIE_A::DISABLED,
            true => BEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BEIE_A::ENABLED
    }
}
#[doc = "Field `BEIE` writer - Bit Error Interrupt Enable"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, BEIE_A, O>;
impl<'a, const O: u8> BEIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BEIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BEIE_A::ENABLED)
    }
}
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::DISABLED,
            true => FEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::ENABLED
    }
}
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
    }
}
#[doc = "Field `AM0IE` reader - Address Match 0 Interrupt Enable"]
pub type AM0IE_R = crate::BitReader<AM0IE_A>;
#[doc = "Address Match 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM0IE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AM0IE_A> for bool {
    #[inline(always)]
    fn from(variant: AM0IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AM0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM0IE_A {
        match self.bits {
            false => AM0IE_A::DISABLED,
            true => AM0IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AM0IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AM0IE_A::ENABLED
    }
}
#[doc = "Field `AM0IE` writer - Address Match 0 Interrupt Enable"]
pub type AM0IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, AM0IE_A, O>;
impl<'a, const O: u8> AM0IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AM0IE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AM0IE_A::ENABLED)
    }
}
#[doc = "Field `AM1IE` reader - Address Match 1 Interrupt Enable"]
pub type AM1IE_R = crate::BitReader<AM1IE_A>;
#[doc = "Address Match 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM1IE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<AM1IE_A> for bool {
    #[inline(always)]
    fn from(variant: AM1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AM1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM1IE_A {
        match self.bits {
            false => AM1IE_A::DISABLED,
            true => AM1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AM1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AM1IE_A::ENABLED
    }
}
#[doc = "Field `AM1IE` writer - Address Match 1 Interrupt Enable"]
pub type AM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, AM1IE_A, O>;
impl<'a, const O: u8> AM1IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AM1IE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AM1IE_A::ENABLED)
    }
}
#[doc = "Field `GCIE` reader - General Call Interrupt Enable"]
pub type GCIE_R = crate::BitReader<GCIE_A>;
#[doc = "General Call Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<GCIE_A> for bool {
    #[inline(always)]
    fn from(variant: GCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl GCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCIE_A {
        match self.bits {
            false => GCIE_A::DISABLED,
            true => GCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCIE_A::ENABLED
    }
}
#[doc = "Field `GCIE` writer - General Call Interrupt Enable"]
pub type GCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, GCIE_A, O>;
impl<'a, const O: u8> GCIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCIE_A::ENABLED)
    }
}
#[doc = "Field `SARIE` reader - SMBus Alert Response Interrupt Enable"]
pub type SARIE_R = crate::BitReader<SARIE_A>;
#[doc = "SMBus Alert Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SARIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SARIE_A> for bool {
    #[inline(always)]
    fn from(variant: SARIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SARIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARIE_A {
        match self.bits {
            false => SARIE_A::DISABLED,
            true => SARIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SARIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SARIE_A::ENABLED
    }
}
#[doc = "Field `SARIE` writer - SMBus Alert Response Interrupt Enable"]
pub type SARIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIER_SPEC, SARIE_A, O>;
impl<'a, const O: u8> SARIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SARIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SARIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avie(&self) -> AVIE_R {
        AVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsie(&self) -> RSIE_R {
        RSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&self) -> SDIE_R {
        SDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn am0ie(&self) -> AM0IE_R {
        AM0IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn am1ie(&self) -> AM1IE_R {
        AM1IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline(always)]
    pub fn gcie(&self) -> GCIE_R {
        GCIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub fn sarie(&self) -> SARIE_R {
        SARIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdie(&mut self) -> TDIE_W<0> {
        TDIE_W::new(self)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<1> {
        RDIE_W::new(self)
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avie(&mut self) -> AVIE_W<2> {
        AVIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taie(&mut self) -> TAIE_W<3> {
        TAIE_W::new(self)
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsie(&mut self) -> RSIE_W<8> {
        RSIE_W::new(self)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdie(&mut self) -> SDIE_W<9> {
        SDIE_W::new(self)
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<10> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<11> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn am0ie(&mut self) -> AM0IE_W<12> {
        AM0IE_W::new(self)
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn am1ie(&mut self) -> AM1IE_W<13> {
        AM1IE_W::new(self)
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcie(&mut self) -> GCIE_W<14> {
        GCIE_W::new(self)
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sarie(&mut self) -> SARIE_W<15> {
        SARIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sier](index.html) module"]
pub struct SIER_SPEC;
impl crate::RegisterSpec for SIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sier::R](R) reader structure"]
impl crate::Readable for SIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sier::W](W) writer structure"]
impl crate::Writable for SIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIER to value 0"]
impl crate::Resettable for SIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
