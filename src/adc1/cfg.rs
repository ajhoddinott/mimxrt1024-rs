#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADICLK` reader - Input Clock Select"]
pub type ADICLK_R = crate::FieldReader<u8, ADICLK_A>;
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADICLK_A {
    #[doc = "0: IPG clock"]
    ADICLK_0 = 0,
    #[doc = "1: IPG clock divided by 2"]
    ADICLK_1 = 1,
    #[doc = "3: Asynchronous clock (ADACK)"]
    ADICLK_3 = 3,
}
impl From<ADICLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADICLK_A) -> Self {
        variant as _
    }
}
impl ADICLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADICLK_A> {
        match self.bits {
            0 => Some(ADICLK_A::ADICLK_0),
            1 => Some(ADICLK_A::ADICLK_1),
            3 => Some(ADICLK_A::ADICLK_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADICLK_0`"]
    #[inline(always)]
    pub fn is_adiclk_0(&self) -> bool {
        *self == ADICLK_A::ADICLK_0
    }
    #[doc = "Checks if the value of the field is `ADICLK_1`"]
    #[inline(always)]
    pub fn is_adiclk_1(&self) -> bool {
        *self == ADICLK_A::ADICLK_1
    }
    #[doc = "Checks if the value of the field is `ADICLK_3`"]
    #[inline(always)]
    pub fn is_adiclk_3(&self) -> bool {
        *self == ADICLK_A::ADICLK_3
    }
}
#[doc = "Field `ADICLK` writer - Input Clock Select"]
pub type ADICLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, ADICLK_A, 2, O>;
impl<'a, const O: u8> ADICLK_W<'a, O> {
    #[doc = "IPG clock"]
    #[inline(always)]
    pub fn adiclk_0(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_0)
    }
    #[doc = "IPG clock divided by 2"]
    #[inline(always)]
    pub fn adiclk_1(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_1)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn adiclk_3(self) -> &'a mut W {
        self.variant(ADICLK_A::ADICLK_3)
    }
}
#[doc = "Field `MODE` reader - Conversion Mode Selection"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Conversion Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 8-bit conversion"]
    MODE_0 = 0,
    #[doc = "1: 10-bit conversion"]
    MODE_1 = 1,
    #[doc = "2: 12-bit conversion"]
    MODE_2 = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::MODE_0),
            1 => Some(MODE_A::MODE_1),
            2 => Some(MODE_A::MODE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
    #[doc = "Checks if the value of the field is `MODE_2`"]
    #[inline(always)]
    pub fn is_mode_2(&self) -> bool {
        *self == MODE_A::MODE_2
    }
}
#[doc = "Field `MODE` writer - Conversion Mode Selection"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "8-bit conversion"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "10-bit conversion"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
    }
    #[doc = "12-bit conversion"]
    #[inline(always)]
    pub fn mode_2(self) -> &'a mut W {
        self.variant(MODE_A::MODE_2)
    }
}
#[doc = "Field `ADLSMP` reader - Long Sample Time Configuration"]
pub type ADLSMP_R = crate::BitReader<ADLSMP_A>;
#[doc = "Long Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADLSMP_A {
    #[doc = "0: Short sample mode."]
    ADLSMP_0 = 0,
    #[doc = "1: Long sample mode."]
    ADLSMP_1 = 1,
}
impl From<ADLSMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADLSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADLSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSMP_A {
        match self.bits {
            false => ADLSMP_A::ADLSMP_0,
            true => ADLSMP_A::ADLSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLSMP_0`"]
    #[inline(always)]
    pub fn is_adlsmp_0(&self) -> bool {
        *self == ADLSMP_A::ADLSMP_0
    }
    #[doc = "Checks if the value of the field is `ADLSMP_1`"]
    #[inline(always)]
    pub fn is_adlsmp_1(&self) -> bool {
        *self == ADLSMP_A::ADLSMP_1
    }
}
#[doc = "Field `ADLSMP` writer - Long Sample Time Configuration"]
pub type ADLSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ADLSMP_A, O>;
impl<'a, const O: u8> ADLSMP_W<'a, O> {
    #[doc = "Short sample mode."]
    #[inline(always)]
    pub fn adlsmp_0(self) -> &'a mut W {
        self.variant(ADLSMP_A::ADLSMP_0)
    }
    #[doc = "Long sample mode."]
    #[inline(always)]
    pub fn adlsmp_1(self) -> &'a mut W {
        self.variant(ADLSMP_A::ADLSMP_1)
    }
}
#[doc = "Field `ADIV` reader - Clock Divide Select"]
pub type ADIV_R = crate::FieldReader<u8, ADIV_A>;
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADIV_A {
    #[doc = "0: Input clock"]
    ADIV_0 = 0,
    #[doc = "1: Input clock / 2"]
    ADIV_1 = 1,
    #[doc = "2: Input clock / 4"]
    ADIV_2 = 2,
    #[doc = "3: Input clock / 8"]
    ADIV_3 = 3,
}
impl From<ADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIV_A) -> Self {
        variant as _
    }
}
impl ADIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIV_A {
        match self.bits {
            0 => ADIV_A::ADIV_0,
            1 => ADIV_A::ADIV_1,
            2 => ADIV_A::ADIV_2,
            3 => ADIV_A::ADIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADIV_0`"]
    #[inline(always)]
    pub fn is_adiv_0(&self) -> bool {
        *self == ADIV_A::ADIV_0
    }
    #[doc = "Checks if the value of the field is `ADIV_1`"]
    #[inline(always)]
    pub fn is_adiv_1(&self) -> bool {
        *self == ADIV_A::ADIV_1
    }
    #[doc = "Checks if the value of the field is `ADIV_2`"]
    #[inline(always)]
    pub fn is_adiv_2(&self) -> bool {
        *self == ADIV_A::ADIV_2
    }
    #[doc = "Checks if the value of the field is `ADIV_3`"]
    #[inline(always)]
    pub fn is_adiv_3(&self) -> bool {
        *self == ADIV_A::ADIV_3
    }
}
#[doc = "Field `ADIV` writer - Clock Divide Select"]
pub type ADIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, ADIV_A, 2, O>;
impl<'a, const O: u8> ADIV_W<'a, O> {
    #[doc = "Input clock"]
    #[inline(always)]
    pub fn adiv_0(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_0)
    }
    #[doc = "Input clock / 2"]
    #[inline(always)]
    pub fn adiv_1(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_1)
    }
    #[doc = "Input clock / 4"]
    #[inline(always)]
    pub fn adiv_2(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_2)
    }
    #[doc = "Input clock / 8"]
    #[inline(always)]
    pub fn adiv_3(self) -> &'a mut W {
        self.variant(ADIV_A::ADIV_3)
    }
}
#[doc = "Field `ADLPC` reader - Low-Power Configuration"]
pub type ADLPC_R = crate::BitReader<ADLPC_A>;
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADLPC_A {
    #[doc = "0: ADC hard block not in low power mode."]
    ADLPC_0 = 0,
    #[doc = "1: ADC hard block in low power mode."]
    ADLPC_1 = 1,
}
impl From<ADLPC_A> for bool {
    #[inline(always)]
    fn from(variant: ADLPC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADLPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLPC_A {
        match self.bits {
            false => ADLPC_A::ADLPC_0,
            true => ADLPC_A::ADLPC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADLPC_0`"]
    #[inline(always)]
    pub fn is_adlpc_0(&self) -> bool {
        *self == ADLPC_A::ADLPC_0
    }
    #[doc = "Checks if the value of the field is `ADLPC_1`"]
    #[inline(always)]
    pub fn is_adlpc_1(&self) -> bool {
        *self == ADLPC_A::ADLPC_1
    }
}
#[doc = "Field `ADLPC` writer - Low-Power Configuration"]
pub type ADLPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ADLPC_A, O>;
impl<'a, const O: u8> ADLPC_W<'a, O> {
    #[doc = "ADC hard block not in low power mode."]
    #[inline(always)]
    pub fn adlpc_0(self) -> &'a mut W {
        self.variant(ADLPC_A::ADLPC_0)
    }
    #[doc = "ADC hard block in low power mode."]
    #[inline(always)]
    pub fn adlpc_1(self) -> &'a mut W {
        self.variant(ADLPC_A::ADLPC_1)
    }
}
#[doc = "Field `ADSTS` reader - Defines the total sample time duration in number of full cycles"]
pub type ADSTS_R = crate::FieldReader<u8, ADSTS_A>;
#[doc = "Defines the total sample time duration in number of full cycles\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADSTS_A {
    #[doc = "0: Sample period (ADC clocks) = 3 if ADLSMP=0b Sample period (ADC clocks) = 13 if ADLSMP=1b"]
    ADSTS_0 = 0,
    #[doc = "1: Sample period (ADC clocks) = 5 if ADLSMP=0b Sample period (ADC clocks) = 17 if ADLSMP=1b"]
    ADSTS_1 = 1,
    #[doc = "2: Sample period (ADC clocks) = 7 if ADLSMP=0b Sample period (ADC clocks) = 21 if ADLSMP=1b"]
    ADSTS_2 = 2,
    #[doc = "3: Sample period (ADC clocks) = 9 if ADLSMP=0b Sample period (ADC clocks) = 25 if ADLSMP=1b"]
    ADSTS_3 = 3,
}
impl From<ADSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSTS_A) -> Self {
        variant as _
    }
}
impl ADSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTS_A {
        match self.bits {
            0 => ADSTS_A::ADSTS_0,
            1 => ADSTS_A::ADSTS_1,
            2 => ADSTS_A::ADSTS_2,
            3 => ADSTS_A::ADSTS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADSTS_0`"]
    #[inline(always)]
    pub fn is_adsts_0(&self) -> bool {
        *self == ADSTS_A::ADSTS_0
    }
    #[doc = "Checks if the value of the field is `ADSTS_1`"]
    #[inline(always)]
    pub fn is_adsts_1(&self) -> bool {
        *self == ADSTS_A::ADSTS_1
    }
    #[doc = "Checks if the value of the field is `ADSTS_2`"]
    #[inline(always)]
    pub fn is_adsts_2(&self) -> bool {
        *self == ADSTS_A::ADSTS_2
    }
    #[doc = "Checks if the value of the field is `ADSTS_3`"]
    #[inline(always)]
    pub fn is_adsts_3(&self) -> bool {
        *self == ADSTS_A::ADSTS_3
    }
}
#[doc = "Field `ADSTS` writer - Defines the total sample time duration in number of full cycles"]
pub type ADSTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, ADSTS_A, 2, O>;
impl<'a, const O: u8> ADSTS_W<'a, O> {
    #[doc = "Sample period (ADC clocks) = 3 if ADLSMP=0b Sample period (ADC clocks) = 13 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_0(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_0)
    }
    #[doc = "Sample period (ADC clocks) = 5 if ADLSMP=0b Sample period (ADC clocks) = 17 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_1(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_1)
    }
    #[doc = "Sample period (ADC clocks) = 7 if ADLSMP=0b Sample period (ADC clocks) = 21 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_2(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_2)
    }
    #[doc = "Sample period (ADC clocks) = 9 if ADLSMP=0b Sample period (ADC clocks) = 25 if ADLSMP=1b"]
    #[inline(always)]
    pub fn adsts_3(self) -> &'a mut W {
        self.variant(ADSTS_A::ADSTS_3)
    }
}
#[doc = "Field `ADHSC` reader - High Speed Configuration"]
pub type ADHSC_R = crate::BitReader<ADHSC_A>;
#[doc = "High Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADHSC_A {
    #[doc = "0: Normal conversion selected."]
    ADHSC_0 = 0,
    #[doc = "1: High speed conversion selected."]
    ADHSC_1 = 1,
}
impl From<ADHSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADHSC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADHSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHSC_A {
        match self.bits {
            false => ADHSC_A::ADHSC_0,
            true => ADHSC_A::ADHSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADHSC_0`"]
    #[inline(always)]
    pub fn is_adhsc_0(&self) -> bool {
        *self == ADHSC_A::ADHSC_0
    }
    #[doc = "Checks if the value of the field is `ADHSC_1`"]
    #[inline(always)]
    pub fn is_adhsc_1(&self) -> bool {
        *self == ADHSC_A::ADHSC_1
    }
}
#[doc = "Field `ADHSC` writer - High Speed Configuration"]
pub type ADHSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ADHSC_A, O>;
impl<'a, const O: u8> ADHSC_W<'a, O> {
    #[doc = "Normal conversion selected."]
    #[inline(always)]
    pub fn adhsc_0(self) -> &'a mut W {
        self.variant(ADHSC_A::ADHSC_0)
    }
    #[doc = "High speed conversion selected."]
    #[inline(always)]
    pub fn adhsc_1(self) -> &'a mut W {
        self.variant(ADHSC_A::ADHSC_1)
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Selects VREFH/VREFL as reference voltage."]
    REFSEL_0 = 0,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::REFSEL_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        *self == REFSEL_A::REFSEL_0
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_0)
    }
}
#[doc = "Field `ADTRG` reader - Conversion Trigger Select"]
pub type ADTRG_R = crate::BitReader<ADTRG_A>;
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected"]
    ADTRG_0 = 0,
    #[doc = "1: Hardware trigger selected"]
    ADTRG_1 = 1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::ADTRG_0,
            true => ADTRG_A::ADTRG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADTRG_0`"]
    #[inline(always)]
    pub fn is_adtrg_0(&self) -> bool {
        *self == ADTRG_A::ADTRG_0
    }
    #[doc = "Checks if the value of the field is `ADTRG_1`"]
    #[inline(always)]
    pub fn is_adtrg_1(&self) -> bool {
        *self == ADTRG_A::ADTRG_1
    }
}
#[doc = "Field `ADTRG` writer - Conversion Trigger Select"]
pub type ADTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ADTRG_A, O>;
impl<'a, const O: u8> ADTRG_W<'a, O> {
    #[doc = "Software trigger selected"]
    #[inline(always)]
    pub fn adtrg_0(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_0)
    }
    #[doc = "Hardware trigger selected"]
    #[inline(always)]
    pub fn adtrg_1(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_1)
    }
}
#[doc = "Field `AVGS` reader - Hardware Average select"]
pub type AVGS_R = crate::FieldReader<u8, AVGS_A>;
#[doc = "Hardware Average select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: 4 samples averaged"]
    AVGS_0 = 0,
    #[doc = "1: 8 samples averaged"]
    AVGS_1 = 1,
    #[doc = "2: 16 samples averaged"]
    AVGS_2 = 2,
    #[doc = "3: 32 samples averaged"]
    AVGS_3 = 3,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
impl AVGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::AVGS_0,
            1 => AVGS_A::AVGS_1,
            2 => AVGS_A::AVGS_2,
            3 => AVGS_A::AVGS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVGS_0`"]
    #[inline(always)]
    pub fn is_avgs_0(&self) -> bool {
        *self == AVGS_A::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline(always)]
    pub fn is_avgs_1(&self) -> bool {
        *self == AVGS_A::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline(always)]
    pub fn is_avgs_2(&self) -> bool {
        *self == AVGS_A::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline(always)]
    pub fn is_avgs_3(&self) -> bool {
        *self == AVGS_A::AVGS_3
    }
}
#[doc = "Field `AVGS` writer - Hardware Average select"]
pub type AVGS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, AVGS_A, 2, O>;
impl<'a, const O: u8> AVGS_W<'a, O> {
    #[doc = "4 samples averaged"]
    #[inline(always)]
    pub fn avgs_0(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_0)
    }
    #[doc = "8 samples averaged"]
    #[inline(always)]
    pub fn avgs_1(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_1)
    }
    #[doc = "16 samples averaged"]
    #[inline(always)]
    pub fn avgs_2(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_2)
    }
    #[doc = "32 samples averaged"]
    #[inline(always)]
    pub fn avgs_3(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_3)
    }
}
#[doc = "Field `OVWREN` reader - Data Overwrite Enable"]
pub type OVWREN_R = crate::BitReader<OVWREN_A>;
#[doc = "Data Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVWREN_A {
    #[doc = "0: Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    OVWREN_0 = 0,
    #[doc = "1: Enable the overwriting."]
    OVWREN_1 = 1,
}
impl From<OVWREN_A> for bool {
    #[inline(always)]
    fn from(variant: OVWREN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVWREN_A {
        match self.bits {
            false => OVWREN_A::OVWREN_0,
            true => OVWREN_A::OVWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVWREN_0`"]
    #[inline(always)]
    pub fn is_ovwren_0(&self) -> bool {
        *self == OVWREN_A::OVWREN_0
    }
    #[doc = "Checks if the value of the field is `OVWREN_1`"]
    #[inline(always)]
    pub fn is_ovwren_1(&self) -> bool {
        *self == OVWREN_A::OVWREN_1
    }
}
#[doc = "Field `OVWREN` writer - Data Overwrite Enable"]
pub type OVWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, OVWREN_A, O>;
impl<'a, const O: u8> OVWREN_W<'a, O> {
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    #[inline(always)]
    pub fn ovwren_0(self) -> &'a mut W {
        self.variant(OVWREN_A::OVWREN_0)
    }
    #[doc = "Enable the overwriting."]
    #[inline(always)]
    pub fn ovwren_1(self) -> &'a mut W {
        self.variant(OVWREN_A::OVWREN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> ADICLK_R {
        ADICLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> ADLSMP_R {
        ADLSMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> ADIV_R {
        ADIV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> ADLPC_R {
        ADLPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Defines the total sample time duration in number of full cycles"]
    #[inline(always)]
    pub fn adsts(&self) -> ADSTS_R {
        ADSTS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline(always)]
    pub fn ovwren(&self) -> OVWREN_R {
        OVWREN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiclk(&mut self) -> ADICLK_W<0> {
        ADICLK_W::new(self)
    }
    #[doc = "Bits 2:3 - Conversion Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Long Sample Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlsmp(&mut self) -> ADLSMP_W<4> {
        ADLSMP_W::new(self)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiv(&mut self) -> ADIV_W<5> {
        ADIV_W::new(self)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlpc(&mut self) -> ADLPC_W<7> {
        ADLPC_W::new(self)
    }
    #[doc = "Bits 8:9 - Defines the total sample time duration in number of full cycles"]
    #[inline(always)]
    #[must_use]
    pub fn adsts(&mut self) -> ADSTS_W<8> {
        ADSTS_W::new(self)
    }
    #[doc = "Bit 10 - High Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> ADHSC_W<10> {
        ADHSC_W::new(self)
    }
    #[doc = "Bits 11:12 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<11> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 13 - Conversion Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg(&mut self) -> ADTRG_W<13> {
        ADTRG_W::new(self)
    }
    #[doc = "Bits 14:15 - Hardware Average select"]
    #[inline(always)]
    #[must_use]
    pub fn avgs(&mut self) -> AVGS_W<14> {
        AVGS_W::new(self)
    }
    #[doc = "Bit 16 - Data Overwrite Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovwren(&mut self) -> OVWREN_W<16> {
        OVWREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0200"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
