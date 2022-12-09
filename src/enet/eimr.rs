#[doc = "Register `EIMR` reader"]
pub struct R(crate::R<EIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMR` writer"]
pub struct W(crate::W<EIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMR_SPEC>;
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
impl From<crate::W<EIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_TIMER` reader - TS_TIMER Interrupt Mask"]
pub type TS_TIMER_R = crate::BitReader<TS_TIMER_A>;
#[doc = "TS_TIMER Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_TIMER_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<TS_TIMER_A> for bool {
    #[inline(always)]
    fn from(variant: TS_TIMER_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_TIMER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_TIMER_A {
        match self.bits {
            false => TS_TIMER_A::ZERO,
            true => TS_TIMER_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TS_TIMER_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TS_TIMER_A::ONE
    }
}
#[doc = "Field `TS_TIMER` writer - TS_TIMER Interrupt Mask"]
pub type TS_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TS_TIMER_A, O>;
impl<'a, const O: u8> TS_TIMER_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TS_TIMER_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TS_TIMER_A::ONE)
    }
}
#[doc = "Field `TS_AVAIL` reader - TS_AVAIL Interrupt Mask"]
pub type TS_AVAIL_R = crate::BitReader<TS_AVAIL_A>;
#[doc = "TS_AVAIL Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_AVAIL_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<TS_AVAIL_A> for bool {
    #[inline(always)]
    fn from(variant: TS_AVAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_AVAIL_A {
        match self.bits {
            false => TS_AVAIL_A::ZERO,
            true => TS_AVAIL_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TS_AVAIL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TS_AVAIL_A::ONE
    }
}
#[doc = "Field `TS_AVAIL` writer - TS_AVAIL Interrupt Mask"]
pub type TS_AVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TS_AVAIL_A, O>;
impl<'a, const O: u8> TS_AVAIL_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TS_AVAIL_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TS_AVAIL_A::ONE)
    }
}
#[doc = "Field `WAKEUP` reader - WAKEUP Interrupt Mask"]
pub type WAKEUP_R = crate::BitReader<WAKEUP_A>;
#[doc = "WAKEUP Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_A {
        match self.bits {
            false => WAKEUP_A::ZERO,
            true => WAKEUP_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WAKEUP_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == WAKEUP_A::ONE
    }
}
#[doc = "Field `WAKEUP` writer - WAKEUP Interrupt Mask"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, WAKEUP_A, O>;
impl<'a, const O: u8> WAKEUP_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(WAKEUP_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(WAKEUP_A::ONE)
    }
}
#[doc = "Field `PLR` reader - PLR Interrupt Mask"]
pub type PLR_R = crate::BitReader<PLR_A>;
#[doc = "PLR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<PLR_A> for bool {
    #[inline(always)]
    fn from(variant: PLR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLR_A {
        match self.bits {
            false => PLR_A::ZERO,
            true => PLR_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PLR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PLR_A::ONE
    }
}
#[doc = "Field `PLR` writer - PLR Interrupt Mask"]
pub type PLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, PLR_A, O>;
impl<'a, const O: u8> PLR_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PLR_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PLR_A::ONE)
    }
}
#[doc = "Field `UN` reader - UN Interrupt Mask"]
pub type UN_R = crate::BitReader<UN_A>;
#[doc = "UN Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UN_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<UN_A> for bool {
    #[inline(always)]
    fn from(variant: UN_A) -> Self {
        variant as u8 != 0
    }
}
impl UN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UN_A {
        match self.bits {
            false => UN_A::ZERO,
            true => UN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == UN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == UN_A::ONE
    }
}
#[doc = "Field `UN` writer - UN Interrupt Mask"]
pub type UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, UN_A, O>;
impl<'a, const O: u8> UN_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(UN_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(UN_A::ONE)
    }
}
#[doc = "Field `RL` reader - RL Interrupt Mask"]
pub type RL_R = crate::BitReader<RL_A>;
#[doc = "RL Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RL_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<RL_A> for bool {
    #[inline(always)]
    fn from(variant: RL_A) -> Self {
        variant as u8 != 0
    }
}
impl RL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RL_A {
        match self.bits {
            false => RL_A::ZERO,
            true => RL_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == RL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == RL_A::ONE
    }
}
#[doc = "Field `RL` writer - RL Interrupt Mask"]
pub type RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, RL_A, O>;
impl<'a, const O: u8> RL_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(RL_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(RL_A::ONE)
    }
}
#[doc = "Field `LC` reader - LC Interrupt Mask"]
pub type LC_R = crate::BitReader<LC_A>;
#[doc = "LC Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LC_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<LC_A> for bool {
    #[inline(always)]
    fn from(variant: LC_A) -> Self {
        variant as u8 != 0
    }
}
impl LC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LC_A {
        match self.bits {
            false => LC_A::ZERO,
            true => LC_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == LC_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == LC_A::ONE
    }
}
#[doc = "Field `LC` writer - LC Interrupt Mask"]
pub type LC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, LC_A, O>;
impl<'a, const O: u8> LC_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(LC_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(LC_A::ONE)
    }
}
#[doc = "Field `EBERR` reader - EBERR Interrupt Mask"]
pub type EBERR_R = crate::BitReader<EBERR_A>;
#[doc = "EBERR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBERR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<EBERR_A> for bool {
    #[inline(always)]
    fn from(variant: EBERR_A) -> Self {
        variant as u8 != 0
    }
}
impl EBERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBERR_A {
        match self.bits {
            false => EBERR_A::ZERO,
            true => EBERR_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EBERR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EBERR_A::ONE
    }
}
#[doc = "Field `EBERR` writer - EBERR Interrupt Mask"]
pub type EBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, EBERR_A, O>;
impl<'a, const O: u8> EBERR_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(EBERR_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(EBERR_A::ONE)
    }
}
#[doc = "Field `MII` reader - MII Interrupt Mask"]
pub type MII_R = crate::BitReader<MII_A>;
#[doc = "MII Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MII_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<MII_A> for bool {
    #[inline(always)]
    fn from(variant: MII_A) -> Self {
        variant as u8 != 0
    }
}
impl MII_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MII_A {
        match self.bits {
            false => MII_A::ZERO,
            true => MII_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == MII_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == MII_A::ONE
    }
}
#[doc = "Field `MII` writer - MII Interrupt Mask"]
pub type MII_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, MII_A, O>;
impl<'a, const O: u8> MII_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(MII_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(MII_A::ONE)
    }
}
#[doc = "Field `RXB` reader - RXB Interrupt Mask"]
pub type RXB_R = crate::BitReader<RXB_A>;
#[doc = "RXB Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXB_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<RXB_A> for bool {
    #[inline(always)]
    fn from(variant: RXB_A) -> Self {
        variant as u8 != 0
    }
}
impl RXB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXB_A {
        match self.bits {
            false => RXB_A::ZERO,
            true => RXB_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == RXB_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == RXB_A::ONE
    }
}
#[doc = "Field `RXB` writer - RXB Interrupt Mask"]
pub type RXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, RXB_A, O>;
impl<'a, const O: u8> RXB_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(RXB_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(RXB_A::ONE)
    }
}
#[doc = "Field `RXF` reader - RXF Interrupt Mask"]
pub type RXF_R = crate::BitReader<RXF_A>;
#[doc = "RXF Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXF_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<RXF_A> for bool {
    #[inline(always)]
    fn from(variant: RXF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXF_A {
        match self.bits {
            false => RXF_A::ZERO,
            true => RXF_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == RXF_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == RXF_A::ONE
    }
}
#[doc = "Field `RXF` writer - RXF Interrupt Mask"]
pub type RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, RXF_A, O>;
impl<'a, const O: u8> RXF_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(RXF_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(RXF_A::ONE)
    }
}
#[doc = "Field `TXB` reader - TXB Interrupt Mask"]
pub type TXB_R = crate::BitReader<TXB_A>;
#[doc = "TXB Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXB_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    MASKED = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    UNMASKED = 1,
}
impl From<TXB_A> for bool {
    #[inline(always)]
    fn from(variant: TXB_A) -> Self {
        variant as u8 != 0
    }
}
impl TXB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXB_A {
        match self.bits {
            false => TXB_A::MASKED,
            true => TXB_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXB_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TXB_A::UNMASKED
    }
}
#[doc = "Field `TXB` writer - TXB Interrupt Mask"]
pub type TXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TXB_A, O>;
impl<'a, const O: u8> TXB_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXB_A::MASKED)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TXB_A::UNMASKED)
    }
}
#[doc = "Field `TXF` reader - TXF Interrupt Mask"]
pub type TXF_R = crate::BitReader<TXF_A>;
#[doc = "TXF Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXF_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    MASKED = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    UNMASKED = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::MASKED,
            true => TXF_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXF_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TXF_A::UNMASKED
    }
}
#[doc = "Field `TXF` writer - TXF Interrupt Mask"]
pub type TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TXF_A, O>;
impl<'a, const O: u8> TXF_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXF_A::MASKED)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TXF_A::UNMASKED)
    }
}
#[doc = "Field `GRA` reader - GRA Interrupt Mask"]
pub type GRA_R = crate::BitReader<GRA_A>;
#[doc = "GRA Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRA_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    MASKED = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    UMASKED = 1,
}
impl From<GRA_A> for bool {
    #[inline(always)]
    fn from(variant: GRA_A) -> Self {
        variant as u8 != 0
    }
}
impl GRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRA_A {
        match self.bits {
            false => GRA_A::MASKED,
            true => GRA_A::UMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == GRA_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UMASKED`"]
    #[inline(always)]
    pub fn is_umasked(&self) -> bool {
        *self == GRA_A::UMASKED
    }
}
#[doc = "Field `GRA` writer - GRA Interrupt Mask"]
pub type GRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, GRA_A, O>;
impl<'a, const O: u8> GRA_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(GRA_A::MASKED)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn umasked(self) -> &'a mut W {
        self.variant(GRA_A::UMASKED)
    }
}
#[doc = "Field `BABT` reader - BABT Interrupt Mask"]
pub type BABT_R = crate::BitReader<BABT_A>;
#[doc = "BABT Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BABT_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<BABT_A> for bool {
    #[inline(always)]
    fn from(variant: BABT_A) -> Self {
        variant as u8 != 0
    }
}
impl BABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABT_A {
        match self.bits {
            false => BABT_A::ZERO,
            true => BABT_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == BABT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == BABT_A::ONE
    }
}
#[doc = "Field `BABT` writer - BABT Interrupt Mask"]
pub type BABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, BABT_A, O>;
impl<'a, const O: u8> BABT_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(BABT_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(BABT_A::ONE)
    }
}
#[doc = "Field `BABR` reader - BABR Interrupt Mask"]
pub type BABR_R = crate::BitReader<BABR_A>;
#[doc = "BABR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BABR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    ZERO = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    ONE = 1,
}
impl From<BABR_A> for bool {
    #[inline(always)]
    fn from(variant: BABR_A) -> Self {
        variant as u8 != 0
    }
}
impl BABR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABR_A {
        match self.bits {
            false => BABR_A::ZERO,
            true => BABR_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == BABR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == BABR_A::ONE
    }
}
#[doc = "Field `BABR` writer - BABR Interrupt Mask"]
pub type BABR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, BABR_A, O>;
impl<'a, const O: u8> BABR_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(BABR_A::ZERO)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(BABR_A::ONE)
    }
}
impl R {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ts_timer(&mut self) -> TS_TIMER_W<15> {
        TS_TIMER_W::new(self)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W<16> {
        TS_AVAIL_W::new(self)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<17> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn plr(&mut self) -> PLR_W<18> {
        PLR_W::new(self)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn un(&mut self) -> UN_W<19> {
        UN_W::new(self)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<20> {
        RL_W::new(self)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lc(&mut self) -> LC_W<21> {
        LC_W::new(self)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eberr(&mut self) -> EBERR_W<22> {
        EBERR_W::new(self)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MII_W<23> {
        MII_W::new(self)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxb(&mut self) -> RXB_W<24> {
        RXB_W::new(self)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<25> {
        RXF_W::new(self)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txb(&mut self) -> TXB_W<26> {
        TXB_W::new(self)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<27> {
        TXF_W::new(self)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn gra(&mut self) -> GRA_W<28> {
        GRA_W::new(self)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn babt(&mut self) -> BABT_W<29> {
        BABT_W::new(self)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn babr(&mut self) -> BABR_W<30> {
        BABR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimr](index.html) module"]
pub struct EIMR_SPEC;
impl crate::RegisterSpec for EIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eimr::R](R) reader structure"]
impl crate::Readable for EIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimr::W](W) writer structure"]
impl crate::Writable for EIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMR to value 0"]
impl crate::Resettable for EIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
