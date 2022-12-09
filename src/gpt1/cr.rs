#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - GPT Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "GPT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: GPT is disabled."]
    EN_0 = 0,
    #[doc = "1: GPT is enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Field `EN` writer - GPT Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "GPT is disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "GPT is enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
}
#[doc = "Field `ENMOD` reader - GPT Enable mode"]
pub type ENMOD_R = crate::BitReader<ENMOD_A>;
#[doc = "GPT Enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENMOD_A {
    #[doc = "0: GPT counter will retain its value when it is disabled."]
    ENMOD_0 = 0,
    #[doc = "1: GPT counter value is reset to 0 when it is disabled."]
    ENMOD_1 = 1,
}
impl From<ENMOD_A> for bool {
    #[inline(always)]
    fn from(variant: ENMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl ENMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENMOD_A {
        match self.bits {
            false => ENMOD_A::ENMOD_0,
            true => ENMOD_A::ENMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENMOD_0`"]
    #[inline(always)]
    pub fn is_enmod_0(&self) -> bool {
        *self == ENMOD_A::ENMOD_0
    }
    #[doc = "Checks if the value of the field is `ENMOD_1`"]
    #[inline(always)]
    pub fn is_enmod_1(&self) -> bool {
        *self == ENMOD_A::ENMOD_1
    }
}
#[doc = "Field `ENMOD` writer - GPT Enable mode"]
pub type ENMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENMOD_A, O>;
impl<'a, const O: u8> ENMOD_W<'a, O> {
    #[doc = "GPT counter will retain its value when it is disabled."]
    #[inline(always)]
    pub fn enmod_0(self) -> &'a mut W {
        self.variant(ENMOD_A::ENMOD_0)
    }
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    #[inline(always)]
    pub fn enmod_1(self) -> &'a mut W {
        self.variant(ENMOD_A::ENMOD_1)
    }
}
#[doc = "Field `DBGEN` reader - GPT debug mode enable"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "GPT debug mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: GPT is disabled in debug mode."]
    DBGEN_0 = 0,
    #[doc = "1: GPT is enabled in debug mode."]
    DBGEN_1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DBGEN_0,
            true => DBGEN_A::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline(always)]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGEN_A::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline(always)]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGEN_A::DBGEN_1
    }
}
#[doc = "Field `DBGEN` writer - GPT debug mode enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "GPT is disabled in debug mode."]
    #[inline(always)]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_0)
    }
    #[doc = "GPT is enabled in debug mode."]
    #[inline(always)]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_1)
    }
}
#[doc = "Field `WAITEN` reader - GPT Wait Mode enable"]
pub type WAITEN_R = crate::BitReader<WAITEN_A>;
#[doc = "GPT Wait Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN_A {
    #[doc = "0: GPT is disabled in wait mode."]
    WAITEN_0 = 0,
    #[doc = "1: GPT is enabled in wait mode."]
    WAITEN_1 = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::WAITEN_0,
            true => WAITEN_A::WAITEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITEN_0`"]
    #[inline(always)]
    pub fn is_waiten_0(&self) -> bool {
        *self == WAITEN_A::WAITEN_0
    }
    #[doc = "Checks if the value of the field is `WAITEN_1`"]
    #[inline(always)]
    pub fn is_waiten_1(&self) -> bool {
        *self == WAITEN_A::WAITEN_1
    }
}
#[doc = "Field `WAITEN` writer - GPT Wait Mode enable"]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, WAITEN_A, O>;
impl<'a, const O: u8> WAITEN_W<'a, O> {
    #[doc = "GPT is disabled in wait mode."]
    #[inline(always)]
    pub fn waiten_0(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_0)
    }
    #[doc = "GPT is enabled in wait mode."]
    #[inline(always)]
    pub fn waiten_1(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_1)
    }
}
#[doc = "Field `DOZEEN` reader - GPT Doze Mode Enable"]
pub type DOZEEN_R = crate::BitReader<DOZEEN_A>;
#[doc = "GPT Doze Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEEN_A {
    #[doc = "0: GPT is disabled in doze mode."]
    DOZEEN_0 = 0,
    #[doc = "1: GPT is enabled in doze mode."]
    DOZEEN_1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::DOZEEN_0,
            true => DOZEEN_A::DOZEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEEN_0`"]
    #[inline(always)]
    pub fn is_dozeen_0(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEEN_1`"]
    #[inline(always)]
    pub fn is_dozeen_1(&self) -> bool {
        *self == DOZEEN_A::DOZEEN_1
    }
}
#[doc = "Field `DOZEEN` writer - GPT Doze Mode Enable"]
pub type DOZEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DOZEEN_A, O>;
impl<'a, const O: u8> DOZEEN_W<'a, O> {
    #[doc = "GPT is disabled in doze mode."]
    #[inline(always)]
    pub fn dozeen_0(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_0)
    }
    #[doc = "GPT is enabled in doze mode."]
    #[inline(always)]
    pub fn dozeen_1(self) -> &'a mut W {
        self.variant(DOZEEN_A::DOZEEN_1)
    }
}
#[doc = "Field `STOPEN` reader - GPT Stop Mode enable"]
pub type STOPEN_R = crate::BitReader<STOPEN_A>;
#[doc = "GPT Stop Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPEN_A {
    #[doc = "0: GPT is disabled in Stop mode."]
    STOPEN_0 = 0,
    #[doc = "1: GPT is enabled in Stop mode."]
    STOPEN_1 = 1,
}
impl From<STOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPEN_A {
        match self.bits {
            false => STOPEN_A::STOPEN_0,
            true => STOPEN_A::STOPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPEN_0`"]
    #[inline(always)]
    pub fn is_stopen_0(&self) -> bool {
        *self == STOPEN_A::STOPEN_0
    }
    #[doc = "Checks if the value of the field is `STOPEN_1`"]
    #[inline(always)]
    pub fn is_stopen_1(&self) -> bool {
        *self == STOPEN_A::STOPEN_1
    }
}
#[doc = "Field `STOPEN` writer - GPT Stop Mode enable"]
pub type STOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, STOPEN_A, O>;
impl<'a, const O: u8> STOPEN_W<'a, O> {
    #[doc = "GPT is disabled in Stop mode."]
    #[inline(always)]
    pub fn stopen_0(self) -> &'a mut W {
        self.variant(STOPEN_A::STOPEN_0)
    }
    #[doc = "GPT is enabled in Stop mode."]
    #[inline(always)]
    pub fn stopen_1(self) -> &'a mut W {
        self.variant(STOPEN_A::STOPEN_1)
    }
}
#[doc = "Field `CLKSRC` reader - Clock Source select"]
pub type CLKSRC_R = crate::FieldReader<u8, CLKSRC_A>;
#[doc = "Clock Source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSRC_A {
    #[doc = "0: No clock"]
    CLKSRC_0 = 0,
    #[doc = "1: Peripheral Clock (ipg_clk)"]
    CLKSRC_1 = 1,
    #[doc = "2: High Frequency Reference Clock (ipg_clk_highfreq)"]
    CLKSRC_2 = 2,
    #[doc = "3: External Clock"]
    CLKSRC_3 = 3,
    #[doc = "4: Low Frequency Reference Clock (ipg_clk_32k)"]
    CLKSRC_4 = 4,
    #[doc = "5: Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    CLKSRC_5 = 5,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as _
    }
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSRC_A> {
        match self.bits {
            0 => Some(CLKSRC_A::CLKSRC_0),
            1 => Some(CLKSRC_A::CLKSRC_1),
            2 => Some(CLKSRC_A::CLKSRC_2),
            3 => Some(CLKSRC_A::CLKSRC_3),
            4 => Some(CLKSRC_A::CLKSRC_4),
            5 => Some(CLKSRC_A::CLKSRC_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLKSRC_0`"]
    #[inline(always)]
    pub fn is_clksrc_0(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_0
    }
    #[doc = "Checks if the value of the field is `CLKSRC_1`"]
    #[inline(always)]
    pub fn is_clksrc_1(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_1
    }
    #[doc = "Checks if the value of the field is `CLKSRC_2`"]
    #[inline(always)]
    pub fn is_clksrc_2(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_2
    }
    #[doc = "Checks if the value of the field is `CLKSRC_3`"]
    #[inline(always)]
    pub fn is_clksrc_3(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_3
    }
    #[doc = "Checks if the value of the field is `CLKSRC_4`"]
    #[inline(always)]
    pub fn is_clksrc_4(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_4
    }
    #[doc = "Checks if the value of the field is `CLKSRC_5`"]
    #[inline(always)]
    pub fn is_clksrc_5(&self) -> bool {
        *self == CLKSRC_A::CLKSRC_5
    }
}
#[doc = "Field `CLKSRC` writer - Clock Source select"]
pub type CLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, CLKSRC_A, 3, O>;
impl<'a, const O: u8> CLKSRC_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn clksrc_0(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_0)
    }
    #[doc = "Peripheral Clock (ipg_clk)"]
    #[inline(always)]
    pub fn clksrc_1(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_1)
    }
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)"]
    #[inline(always)]
    pub fn clksrc_2(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_2)
    }
    #[doc = "External Clock"]
    #[inline(always)]
    pub fn clksrc_3(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_3)
    }
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)"]
    #[inline(always)]
    pub fn clksrc_4(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_4)
    }
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)"]
    #[inline(always)]
    pub fn clksrc_5(self) -> &'a mut W {
        self.variant(CLKSRC_A::CLKSRC_5)
    }
}
#[doc = "Field `FRR` reader - Free-Run or Restart mode"]
pub type FRR_R = crate::BitReader<FRR_A>;
#[doc = "Free-Run or Restart mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRR_A {
    #[doc = "0: Restart mode"]
    FRR_0 = 0,
    #[doc = "1: Free-Run mode"]
    FRR_1 = 1,
}
impl From<FRR_A> for bool {
    #[inline(always)]
    fn from(variant: FRR_A) -> Self {
        variant as u8 != 0
    }
}
impl FRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRR_A {
        match self.bits {
            false => FRR_A::FRR_0,
            true => FRR_A::FRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRR_0`"]
    #[inline(always)]
    pub fn is_frr_0(&self) -> bool {
        *self == FRR_A::FRR_0
    }
    #[doc = "Checks if the value of the field is `FRR_1`"]
    #[inline(always)]
    pub fn is_frr_1(&self) -> bool {
        *self == FRR_A::FRR_1
    }
}
#[doc = "Field `FRR` writer - Free-Run or Restart mode"]
pub type FRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FRR_A, O>;
impl<'a, const O: u8> FRR_W<'a, O> {
    #[doc = "Restart mode"]
    #[inline(always)]
    pub fn frr_0(self) -> &'a mut W {
        self.variant(FRR_A::FRR_0)
    }
    #[doc = "Free-Run mode"]
    #[inline(always)]
    pub fn frr_1(self) -> &'a mut W {
        self.variant(FRR_A::FRR_1)
    }
}
#[doc = "Field `EN_24M` reader - Enable 24 MHz clock input from crystal"]
pub type EN_24M_R = crate::BitReader<EN_24M_A>;
#[doc = "Enable 24 MHz clock input from crystal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_24M_A {
    #[doc = "0: 24M clock disabled"]
    EN_24M_0 = 0,
    #[doc = "1: 24M clock enabled"]
    EN_24M_1 = 1,
}
impl From<EN_24M_A> for bool {
    #[inline(always)]
    fn from(variant: EN_24M_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_24M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_24M_A {
        match self.bits {
            false => EN_24M_A::EN_24M_0,
            true => EN_24M_A::EN_24M_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_24M_0`"]
    #[inline(always)]
    pub fn is_en_24m_0(&self) -> bool {
        *self == EN_24M_A::EN_24M_0
    }
    #[doc = "Checks if the value of the field is `EN_24M_1`"]
    #[inline(always)]
    pub fn is_en_24m_1(&self) -> bool {
        *self == EN_24M_A::EN_24M_1
    }
}
#[doc = "Field `EN_24M` writer - Enable 24 MHz clock input from crystal"]
pub type EN_24M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN_24M_A, O>;
impl<'a, const O: u8> EN_24M_W<'a, O> {
    #[doc = "24M clock disabled"]
    #[inline(always)]
    pub fn en_24m_0(self) -> &'a mut W {
        self.variant(EN_24M_A::EN_24M_0)
    }
    #[doc = "24M clock enabled"]
    #[inline(always)]
    pub fn en_24m_1(self) -> &'a mut W {
        self.variant(EN_24M_A::EN_24M_1)
    }
}
#[doc = "Field `SWR` reader - Software reset"]
pub type SWR_R = crate::BitReader<SWR_A>;
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWR_A {
    #[doc = "0: GPT is not in reset state"]
    SWR_0 = 0,
    #[doc = "1: GPT is in reset state"]
    SWR_1 = 1,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWR_A {
        match self.bits {
            false => SWR_A::SWR_0,
            true => SWR_A::SWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWR_0`"]
    #[inline(always)]
    pub fn is_swr_0(&self) -> bool {
        *self == SWR_A::SWR_0
    }
    #[doc = "Checks if the value of the field is `SWR_1`"]
    #[inline(always)]
    pub fn is_swr_1(&self) -> bool {
        *self == SWR_A::SWR_1
    }
}
#[doc = "Field `SWR` writer - Software reset"]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SWR_A, O>;
impl<'a, const O: u8> SWR_W<'a, O> {
    #[doc = "GPT is not in reset state"]
    #[inline(always)]
    pub fn swr_0(self) -> &'a mut W {
        self.variant(SWR_A::SWR_0)
    }
    #[doc = "GPT is in reset state"]
    #[inline(always)]
    pub fn swr_1(self) -> &'a mut W {
        self.variant(SWR_A::SWR_1)
    }
}
#[doc = "Field `IM1` reader - See IM2"]
pub type IM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IM1` writer - See IM2"]
pub type IM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `IM2` reader - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
pub type IM2_R = crate::FieldReader<u8, IM2_A>;
#[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IM2_A {
    #[doc = "0: capture disabled"]
    IM2_0 = 0,
    #[doc = "1: capture on rising edge only"]
    IM2_1 = 1,
    #[doc = "2: capture on falling edge only"]
    IM2_2 = 2,
    #[doc = "3: capture on both edges"]
    IM2_3 = 3,
}
impl From<IM2_A> for u8 {
    #[inline(always)]
    fn from(variant: IM2_A) -> Self {
        variant as _
    }
}
impl IM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM2_A {
        match self.bits {
            0 => IM2_A::IM2_0,
            1 => IM2_A::IM2_1,
            2 => IM2_A::IM2_2,
            3 => IM2_A::IM2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IM2_0`"]
    #[inline(always)]
    pub fn is_im2_0(&self) -> bool {
        *self == IM2_A::IM2_0
    }
    #[doc = "Checks if the value of the field is `IM2_1`"]
    #[inline(always)]
    pub fn is_im2_1(&self) -> bool {
        *self == IM2_A::IM2_1
    }
    #[doc = "Checks if the value of the field is `IM2_2`"]
    #[inline(always)]
    pub fn is_im2_2(&self) -> bool {
        *self == IM2_A::IM2_2
    }
    #[doc = "Checks if the value of the field is `IM2_3`"]
    #[inline(always)]
    pub fn is_im2_3(&self) -> bool {
        *self == IM2_A::IM2_3
    }
}
#[doc = "Field `IM2` writer - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
pub type IM2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, IM2_A, 2, O>;
impl<'a, const O: u8> IM2_W<'a, O> {
    #[doc = "capture disabled"]
    #[inline(always)]
    pub fn im2_0(self) -> &'a mut W {
        self.variant(IM2_A::IM2_0)
    }
    #[doc = "capture on rising edge only"]
    #[inline(always)]
    pub fn im2_1(self) -> &'a mut W {
        self.variant(IM2_A::IM2_1)
    }
    #[doc = "capture on falling edge only"]
    #[inline(always)]
    pub fn im2_2(self) -> &'a mut W {
        self.variant(IM2_A::IM2_2)
    }
    #[doc = "capture on both edges"]
    #[inline(always)]
    pub fn im2_3(self) -> &'a mut W {
        self.variant(IM2_A::IM2_3)
    }
}
#[doc = "Field `OM1` reader - See OM3"]
pub type OM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OM1` writer - See OM3"]
pub type OM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OM2` reader - See OM3"]
pub type OM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OM2` writer - See OM3"]
pub type OM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OM3` reader - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
pub type OM3_R = crate::FieldReader<u8, OM3_A>;
#[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OM3_A {
    #[doc = "0: Output disconnected. No response on pin."]
    OM3_0 = 0,
    #[doc = "1: Toggle output pin"]
    OM3_1 = 1,
    #[doc = "2: Clear output pin"]
    OM3_2 = 2,
    #[doc = "3: Set output pin"]
    OM3_3 = 3,
    #[doc = "4: Generate an active low pulse (that is one input clock wide) on the output pin."]
    OM3_4 = 4,
}
impl From<OM3_A> for u8 {
    #[inline(always)]
    fn from(variant: OM3_A) -> Self {
        variant as _
    }
}
impl OM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OM3_A> {
        match self.bits {
            0 => Some(OM3_A::OM3_0),
            1 => Some(OM3_A::OM3_1),
            2 => Some(OM3_A::OM3_2),
            3 => Some(OM3_A::OM3_3),
            4 => Some(OM3_A::OM3_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OM3_0`"]
    #[inline(always)]
    pub fn is_om3_0(&self) -> bool {
        *self == OM3_A::OM3_0
    }
    #[doc = "Checks if the value of the field is `OM3_1`"]
    #[inline(always)]
    pub fn is_om3_1(&self) -> bool {
        *self == OM3_A::OM3_1
    }
    #[doc = "Checks if the value of the field is `OM3_2`"]
    #[inline(always)]
    pub fn is_om3_2(&self) -> bool {
        *self == OM3_A::OM3_2
    }
    #[doc = "Checks if the value of the field is `OM3_3`"]
    #[inline(always)]
    pub fn is_om3_3(&self) -> bool {
        *self == OM3_A::OM3_3
    }
    #[doc = "Checks if the value of the field is `OM3_4`"]
    #[inline(always)]
    pub fn is_om3_4(&self) -> bool {
        *self == OM3_A::OM3_4
    }
}
#[doc = "Field `OM3` writer - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
pub type OM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, OM3_A, 3, O>;
impl<'a, const O: u8> OM3_W<'a, O> {
    #[doc = "Output disconnected. No response on pin."]
    #[inline(always)]
    pub fn om3_0(self) -> &'a mut W {
        self.variant(OM3_A::OM3_0)
    }
    #[doc = "Toggle output pin"]
    #[inline(always)]
    pub fn om3_1(self) -> &'a mut W {
        self.variant(OM3_A::OM3_1)
    }
    #[doc = "Clear output pin"]
    #[inline(always)]
    pub fn om3_2(self) -> &'a mut W {
        self.variant(OM3_A::OM3_2)
    }
    #[doc = "Set output pin"]
    #[inline(always)]
    pub fn om3_3(self) -> &'a mut W {
        self.variant(OM3_A::OM3_3)
    }
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    #[inline(always)]
    pub fn om3_4(self) -> &'a mut W {
        self.variant(OM3_A::OM3_4)
    }
}
#[doc = "Field `FO1` reader - See F03"]
pub type FO1_R = crate::BitReader<bool>;
#[doc = "Field `FO1` writer - See F03"]
pub type FO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FO2` reader - See F03"]
pub type FO2_R = crate::BitReader<bool>;
#[doc = "Field `FO2` writer - See F03"]
pub type FO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FO3` reader - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
pub type FO3_R = crate::BitReader<FO3_A>;
#[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FO3_A {
    #[doc = "0: Writing a 0 has no effect."]
    FO3_0 = 0,
    #[doc = "1: Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    FO3_1 = 1,
}
impl From<FO3_A> for bool {
    #[inline(always)]
    fn from(variant: FO3_A) -> Self {
        variant as u8 != 0
    }
}
impl FO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FO3_A {
        match self.bits {
            false => FO3_A::FO3_0,
            true => FO3_A::FO3_1,
        }
    }
    #[doc = "Checks if the value of the field is `FO3_0`"]
    #[inline(always)]
    pub fn is_fo3_0(&self) -> bool {
        *self == FO3_A::FO3_0
    }
    #[doc = "Checks if the value of the field is `FO3_1`"]
    #[inline(always)]
    pub fn is_fo3_1(&self) -> bool {
        *self == FO3_A::FO3_1
    }
}
#[doc = "Field `FO3` writer - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
pub type FO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FO3_A, O>;
impl<'a, const O: u8> FO3_W<'a, O> {
    #[doc = "Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fo3_0(self) -> &'a mut W {
        self.variant(FO3_A::FO3_0)
    }
    #[doc = "Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    #[inline(always)]
    pub fn fo3_1(self) -> &'a mut W {
        self.variant(FO3_A::FO3_1)
    }
}
impl R {
    #[doc = "Bit 0 - GPT Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline(always)]
    pub fn enmod(&self) -> ENMOD_R {
        ENMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline(always)]
    pub fn frr(&self) -> FRR_R {
        FRR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub fn en_24m(&self) -> EN_24M_R {
        EN_24M_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline(always)]
    pub fn om1(&self) -> OM1_R {
        OM1_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline(always)]
    pub fn om2(&self) -> OM2_R {
        OM2_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub fn om3(&self) -> OM3_R {
        OM3_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - See F03"]
    #[inline(always)]
    pub fn fo1(&self) -> FO1_R {
        FO1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - See F03"]
    #[inline(always)]
    pub fn fo2(&self) -> FO2_R {
        FO2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub fn fo3(&self) -> FO3_R {
        FO3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - GPT Enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn enmod(&mut self) -> ENMOD_W<1> {
        ENMOD_W::new(self)
    }
    #[doc = "Bit 2 - GPT debug mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<2> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 3 - GPT Wait Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<3> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 4 - GPT Doze Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozeen(&mut self) -> DOZEEN_W<4> {
        DOZEEN_W::new(self)
    }
    #[doc = "Bit 5 - GPT Stop Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopen(&mut self) -> STOPEN_W<5> {
        STOPEN_W::new(self)
    }
    #[doc = "Bits 6:8 - Clock Source select"]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<6> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bit 9 - Free-Run or Restart mode"]
    #[inline(always)]
    #[must_use]
    pub fn frr(&mut self) -> FRR_W<9> {
        FRR_W::new(self)
    }
    #[doc = "Bit 10 - Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    #[must_use]
    pub fn en_24m(&mut self) -> EN_24M_W<10> {
        EN_24M_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<15> {
        SWR_W::new(self)
    }
    #[doc = "Bits 16:17 - See IM2"]
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> IM1_W<16> {
        IM1_W::new(self)
    }
    #[doc = "Bits 18:19 - IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> IM2_W<18> {
        IM2_W::new(self)
    }
    #[doc = "Bits 20:22 - See OM3"]
    #[inline(always)]
    #[must_use]
    pub fn om1(&mut self) -> OM1_W<20> {
        OM1_W::new(self)
    }
    #[doc = "Bits 23:25 - See OM3"]
    #[inline(always)]
    #[must_use]
    pub fn om2(&mut self) -> OM2_W<23> {
        OM2_W::new(self)
    }
    #[doc = "Bits 26:28 - OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    #[must_use]
    pub fn om3(&mut self) -> OM3_W<26> {
        OM3_W::new(self)
    }
    #[doc = "Bit 29 - See F03"]
    #[inline(always)]
    #[must_use]
    pub fn fo1(&mut self) -> FO1_W<29> {
        FO1_W::new(self)
    }
    #[doc = "Bit 30 - See F03"]
    #[inline(always)]
    #[must_use]
    pub fn fo2(&mut self) -> FO2_W<30> {
        FO2_W::new(self)
    }
    #[doc = "Bit 31 - FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    #[must_use]
    pub fn fo3(&mut self) -> FO3_W<31> {
        FO3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
