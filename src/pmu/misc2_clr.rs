#[doc = "Register `MISC2_CLR` reader"]
pub struct R(crate::R<MISC2_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC2_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC2_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC2_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC2_CLR` writer"]
pub struct W(crate::W<MISC2_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC2_CLR_SPEC>;
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
impl From<crate::W<MISC2_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC2_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG0_BO_OFFSET` reader - This field defines the brown out voltage offset for the CORE power domain"]
pub type REG0_BO_OFFSET_R = crate::FieldReader<u8, REG0_BO_OFFSET_A>;
#[doc = "This field defines the brown out voltage offset for the CORE power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG0_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4 = 4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7 = 7,
}
impl From<REG0_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_BO_OFFSET_A) -> Self {
        variant as _
    }
}
impl REG0_BO_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG0_BO_OFFSET_A> {
        match self.bits {
            4 => Some(REG0_BO_OFFSET_A::REG0_BO_OFFSET_4),
            7 => Some(REG0_BO_OFFSET_A::REG0_BO_OFFSET_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg0_bo_offset_4(&self) -> bool {
        *self == REG0_BO_OFFSET_A::REG0_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg0_bo_offset_7(&self) -> bool {
        *self == REG0_BO_OFFSET_A::REG0_BO_OFFSET_7
    }
}
#[doc = "Field `REG0_BO_STATUS` reader - Reg0 brownout status bit."]
pub type REG0_BO_STATUS_R = crate::BitReader<REG0_BO_STATUS_A>;
#[doc = "Reg0 brownout status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REG0_BO_STATUS_A {
    #[doc = "1: Brownout, supply is below target minus brownout offset."]
    REG0_BO_STATUS_1 = 1,
}
impl From<REG0_BO_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG0_BO_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl REG0_BO_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG0_BO_STATUS_A> {
        match self.bits {
            true => Some(REG0_BO_STATUS_A::REG0_BO_STATUS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_STATUS_1`"]
    #[inline(always)]
    pub fn is_reg0_bo_status_1(&self) -> bool {
        *self == REG0_BO_STATUS_A::REG0_BO_STATUS_1
    }
}
#[doc = "Field `REG0_ENABLE_BO` reader - Enables the brownout detection."]
pub type REG0_ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `REG0_ENABLE_BO` writer - Enables the brownout detection."]
pub type REG0_ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC2_CLR_SPEC, bool, O>;
#[doc = "Field `PLL3_disable` reader - Default value of \"0\""]
pub type PLL3_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL3_disable` writer - Default value of \"0\""]
pub type PLL3_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC2_CLR_SPEC, bool, O>;
#[doc = "Field `REG1_BO_OFFSET` reader - This field defines the brown out voltage offset for the xPU power domain"]
pub type REG1_BO_OFFSET_R = crate::FieldReader<u8, REG1_BO_OFFSET_A>;
#[doc = "This field defines the brown out voltage offset for the xPU power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG1_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4 = 4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7 = 7,
}
impl From<REG1_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_BO_OFFSET_A) -> Self {
        variant as _
    }
}
impl REG1_BO_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG1_BO_OFFSET_A> {
        match self.bits {
            4 => Some(REG1_BO_OFFSET_A::REG1_BO_OFFSET_4),
            7 => Some(REG1_BO_OFFSET_A::REG1_BO_OFFSET_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG1_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg1_bo_offset_4(&self) -> bool {
        *self == REG1_BO_OFFSET_A::REG1_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG1_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg1_bo_offset_7(&self) -> bool {
        *self == REG1_BO_OFFSET_A::REG1_BO_OFFSET_7
    }
}
#[doc = "Field `REG1_BO_STATUS` reader - Reg1 brownout status bit."]
pub type REG1_BO_STATUS_R = crate::BitReader<REG1_BO_STATUS_A>;
#[doc = "Reg1 brownout status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REG1_BO_STATUS_A {
    #[doc = "1: Brownout, supply is below target minus brownout offset."]
    REG1_BO_STATUS_1 = 1,
}
impl From<REG1_BO_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG1_BO_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl REG1_BO_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG1_BO_STATUS_A> {
        match self.bits {
            true => Some(REG1_BO_STATUS_A::REG1_BO_STATUS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG1_BO_STATUS_1`"]
    #[inline(always)]
    pub fn is_reg1_bo_status_1(&self) -> bool {
        *self == REG1_BO_STATUS_A::REG1_BO_STATUS_1
    }
}
#[doc = "Field `REG1_ENABLE_BO` reader - Enables the brownout detection."]
pub type REG1_ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `REG1_ENABLE_BO` writer - Enables the brownout detection."]
pub type REG1_ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC2_CLR_SPEC, bool, O>;
#[doc = "Field `AUDIO_DIV_LSB` reader - LSB of Post-divider for Audio PLL"]
pub type AUDIO_DIV_LSB_R = crate::BitReader<AUDIO_DIV_LSB_A>;
#[doc = "LSB of Post-divider for Audio PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDIO_DIV_LSB_A {
    #[doc = "0: divide by 1 (Default)"]
    AUDIO_DIV_LSB_0 = 0,
    #[doc = "1: divide by 2"]
    AUDIO_DIV_LSB_1 = 1,
}
impl From<AUDIO_DIV_LSB_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_DIV_LSB_A) -> Self {
        variant as u8 != 0
    }
}
impl AUDIO_DIV_LSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_DIV_LSB_A {
        match self.bits {
            false => AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0,
            true => AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_0`"]
    #[inline(always)]
    pub fn is_audio_div_lsb_0(&self) -> bool {
        *self == AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_1`"]
    #[inline(always)]
    pub fn is_audio_div_lsb_1(&self) -> bool {
        *self == AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1
    }
}
#[doc = "Field `AUDIO_DIV_LSB` writer - LSB of Post-divider for Audio PLL"]
pub type AUDIO_DIV_LSB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC2_CLR_SPEC, AUDIO_DIV_LSB_A, O>;
impl<'a, const O: u8> AUDIO_DIV_LSB_W<'a, O> {
    #[doc = "divide by 1 (Default)"]
    #[inline(always)]
    pub fn audio_div_lsb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn audio_div_lsb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1)
    }
}
#[doc = "Field `REG2_BO_OFFSET` reader - This field defines the brown out voltage offset for the xPU power domain"]
pub type REG2_BO_OFFSET_R = crate::FieldReader<u8, REG2_BO_OFFSET_A>;
#[doc = "This field defines the brown out voltage offset for the xPU power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG2_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4 = 4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7 = 7,
}
impl From<REG2_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_BO_OFFSET_A) -> Self {
        variant as _
    }
}
impl REG2_BO_OFFSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG2_BO_OFFSET_A> {
        match self.bits {
            4 => Some(REG2_BO_OFFSET_A::REG2_BO_OFFSET_4),
            7 => Some(REG2_BO_OFFSET_A::REG2_BO_OFFSET_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg2_bo_offset_4(&self) -> bool {
        *self == REG2_BO_OFFSET_A::REG2_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg2_bo_offset_7(&self) -> bool {
        *self == REG2_BO_OFFSET_A::REG2_BO_OFFSET_7
    }
}
#[doc = "Field `REG2_BO_STATUS` reader - Reg2 brownout status bit."]
pub type REG2_BO_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `REG2_ENABLE_BO` reader - Enables the brownout detection."]
pub type REG2_ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `REG2_ENABLE_BO` writer - Enables the brownout detection."]
pub type REG2_ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC2_CLR_SPEC, bool, O>;
#[doc = "Field `REG2_OK` reader - Signals that the voltage is above the brownout level for the SOC supply"]
pub type REG2_OK_R = crate::BitReader<bool>;
#[doc = "Field `AUDIO_DIV_MSB` reader - MSB of Post-divider for Audio PLL"]
pub type AUDIO_DIV_MSB_R = crate::BitReader<AUDIO_DIV_MSB_A>;
#[doc = "MSB of Post-divider for Audio PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUDIO_DIV_MSB_A {
    #[doc = "0: divide by 1 (Default)"]
    AUDIO_DIV_MSB_0 = 0,
    #[doc = "1: divide by 2"]
    AUDIO_DIV_MSB_1 = 1,
}
impl From<AUDIO_DIV_MSB_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_DIV_MSB_A) -> Self {
        variant as u8 != 0
    }
}
impl AUDIO_DIV_MSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_DIV_MSB_A {
        match self.bits {
            false => AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0,
            true => AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_0`"]
    #[inline(always)]
    pub fn is_audio_div_msb_0(&self) -> bool {
        *self == AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_1`"]
    #[inline(always)]
    pub fn is_audio_div_msb_1(&self) -> bool {
        *self == AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1
    }
}
#[doc = "Field `AUDIO_DIV_MSB` writer - MSB of Post-divider for Audio PLL"]
pub type AUDIO_DIV_MSB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC2_CLR_SPEC, AUDIO_DIV_MSB_A, O>;
impl<'a, const O: u8> AUDIO_DIV_MSB_W<'a, O> {
    #[doc = "divide by 1 (Default)"]
    #[inline(always)]
    pub fn audio_div_msb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn audio_div_msb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1)
    }
}
#[doc = "Field `REG0_STEP_TIME` reader - Number of clock periods (24MHz clock)."]
pub type REG0_STEP_TIME_R = crate::FieldReader<u8, REG0_STEP_TIME_A>;
#[doc = "Number of clock periods (24MHz clock).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG0_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS = 0,
    #[doc = "1: 128"]
    _128_CLOCKS = 1,
    #[doc = "2: 256"]
    _256_CLOCKS = 2,
    #[doc = "3: 512"]
    _512_CLOCKS = 3,
}
impl From<REG0_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_STEP_TIME_A) -> Self {
        variant as _
    }
}
impl REG0_STEP_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG0_STEP_TIME_A {
        match self.bits {
            0 => REG0_STEP_TIME_A::_64_CLOCKS,
            1 => REG0_STEP_TIME_A::_128_CLOCKS,
            2 => REG0_STEP_TIME_A::_256_CLOCKS,
            3 => REG0_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Field `REG0_STEP_TIME` writer - Number of clock periods (24MHz clock)."]
pub type REG0_STEP_TIME_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC2_CLR_SPEC, u8, REG0_STEP_TIME_A, 2, O>;
impl<'a, const O: u8> REG0_STEP_TIME_W<'a, O> {
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_512_CLOCKS)
    }
}
#[doc = "Field `REG1_STEP_TIME` reader - Number of clock periods (24MHz clock)."]
pub type REG1_STEP_TIME_R = crate::FieldReader<u8, REG1_STEP_TIME_A>;
#[doc = "Number of clock periods (24MHz clock).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG1_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS = 0,
    #[doc = "1: 128"]
    _128_CLOCKS = 1,
    #[doc = "2: 256"]
    _256_CLOCKS = 2,
    #[doc = "3: 512"]
    _512_CLOCKS = 3,
}
impl From<REG1_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_STEP_TIME_A) -> Self {
        variant as _
    }
}
impl REG1_STEP_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG1_STEP_TIME_A {
        match self.bits {
            0 => REG1_STEP_TIME_A::_64_CLOCKS,
            1 => REG1_STEP_TIME_A::_128_CLOCKS,
            2 => REG1_STEP_TIME_A::_256_CLOCKS,
            3 => REG1_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Field `REG1_STEP_TIME` writer - Number of clock periods (24MHz clock)."]
pub type REG1_STEP_TIME_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC2_CLR_SPEC, u8, REG1_STEP_TIME_A, 2, O>;
impl<'a, const O: u8> REG1_STEP_TIME_W<'a, O> {
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_512_CLOCKS)
    }
}
#[doc = "Field `REG2_STEP_TIME` reader - Number of clock periods (24MHz clock)."]
pub type REG2_STEP_TIME_R = crate::FieldReader<u8, REG2_STEP_TIME_A>;
#[doc = "Number of clock periods (24MHz clock).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG2_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS = 0,
    #[doc = "1: 128"]
    _128_CLOCKS = 1,
    #[doc = "2: 256"]
    _256_CLOCKS = 2,
    #[doc = "3: 512"]
    _512_CLOCKS = 3,
}
impl From<REG2_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_STEP_TIME_A) -> Self {
        variant as _
    }
}
impl REG2_STEP_TIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG2_STEP_TIME_A {
        match self.bits {
            0 => REG2_STEP_TIME_A::_64_CLOCKS,
            1 => REG2_STEP_TIME_A::_128_CLOCKS,
            2 => REG2_STEP_TIME_A::_256_CLOCKS,
            3 => REG2_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Field `REG2_STEP_TIME` writer - Number of clock periods (24MHz clock)."]
pub type REG2_STEP_TIME_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MISC2_CLR_SPEC, u8, REG2_STEP_TIME_A, 2, O>;
impl<'a, const O: u8> REG2_STEP_TIME_W<'a, O> {
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_512_CLOCKS)
    }
}
impl R {
    #[doc = "Bits 0:2 - This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub fn reg0_bo_offset(&self) -> REG0_BO_OFFSET_R {
        REG0_BO_OFFSET_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Reg0 brownout status bit."]
    #[inline(always)]
    pub fn reg0_bo_status(&self) -> REG0_BO_STATUS_R {
        REG0_BO_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the brownout detection."]
    #[inline(always)]
    pub fn reg0_enable_bo(&self) -> REG0_ENABLE_BO_R {
        REG0_ENABLE_BO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Default value of \"0\""]
    #[inline(always)]
    pub fn pll3_disable(&self) -> PLL3_DISABLE_R {
        PLL3_DISABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub fn reg1_bo_offset(&self) -> REG1_BO_OFFSET_R {
        REG1_BO_OFFSET_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Reg1 brownout status bit."]
    #[inline(always)]
    pub fn reg1_bo_status(&self) -> REG1_BO_STATUS_R {
        REG1_BO_STATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the brownout detection."]
    #[inline(always)]
    pub fn reg1_enable_bo(&self) -> REG1_ENABLE_BO_R {
        REG1_ENABLE_BO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_lsb(&self) -> AUDIO_DIV_LSB_R {
        AUDIO_DIV_LSB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub fn reg2_bo_offset(&self) -> REG2_BO_OFFSET_R {
        REG2_BO_OFFSET_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Reg2 brownout status bit."]
    #[inline(always)]
    pub fn reg2_bo_status(&self) -> REG2_BO_STATUS_R {
        REG2_BO_STATUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the brownout detection."]
    #[inline(always)]
    pub fn reg2_enable_bo(&self) -> REG2_ENABLE_BO_R {
        REG2_ENABLE_BO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub fn reg2_ok(&self) -> REG2_OK_R {
        REG2_OK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_msb(&self) -> AUDIO_DIV_MSB_R {
        AUDIO_DIV_MSB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub fn reg0_step_time(&self) -> REG0_STEP_TIME_R {
        REG0_STEP_TIME_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub fn reg1_step_time(&self) -> REG1_STEP_TIME_R {
        REG1_STEP_TIME_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    pub fn reg2_step_time(&self) -> REG2_STEP_TIME_R {
        REG2_STEP_TIME_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Enables the brownout detection."]
    #[inline(always)]
    #[must_use]
    pub fn reg0_enable_bo(&mut self) -> REG0_ENABLE_BO_W<5> {
        REG0_ENABLE_BO_W::new(self)
    }
    #[doc = "Bit 7 - Default value of \"0\""]
    #[inline(always)]
    #[must_use]
    pub fn pll3_disable(&mut self) -> PLL3_DISABLE_W<7> {
        PLL3_DISABLE_W::new(self)
    }
    #[doc = "Bit 13 - Enables the brownout detection."]
    #[inline(always)]
    #[must_use]
    pub fn reg1_enable_bo(&mut self) -> REG1_ENABLE_BO_W<13> {
        REG1_ENABLE_BO_W::new(self)
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    #[must_use]
    pub fn audio_div_lsb(&mut self) -> AUDIO_DIV_LSB_W<15> {
        AUDIO_DIV_LSB_W::new(self)
    }
    #[doc = "Bit 21 - Enables the brownout detection."]
    #[inline(always)]
    #[must_use]
    pub fn reg2_enable_bo(&mut self) -> REG2_ENABLE_BO_W<21> {
        REG2_ENABLE_BO_W::new(self)
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    #[must_use]
    pub fn audio_div_msb(&mut self) -> AUDIO_DIV_MSB_W<23> {
        AUDIO_DIV_MSB_W::new(self)
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    #[must_use]
    pub fn reg0_step_time(&mut self) -> REG0_STEP_TIME_W<24> {
        REG0_STEP_TIME_W::new(self)
    }
    #[doc = "Bits 26:27 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    #[must_use]
    pub fn reg1_step_time(&mut self) -> REG1_STEP_TIME_W<26> {
        REG1_STEP_TIME_W::new(self)
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock)."]
    #[inline(always)]
    #[must_use]
    pub fn reg2_step_time(&mut self) -> REG2_STEP_TIME_W<28> {
        REG2_STEP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc2_clr](index.html) module"]
pub struct MISC2_CLR_SPEC;
impl crate::RegisterSpec for MISC2_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc2_clr::R](R) reader structure"]
impl crate::Readable for MISC2_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc2_clr::W](W) writer structure"]
impl crate::Writable for MISC2_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC2_CLR to value 0x0027_2727"]
impl crate::Resettable for MISC2_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0027_2727;
}
