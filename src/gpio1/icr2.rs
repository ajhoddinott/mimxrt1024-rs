#[doc = "Register `ICR2` reader"]
pub struct R(crate::R<ICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR2` writer"]
pub struct W(crate::W<ICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR2_SPEC>;
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
impl From<crate::W<ICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR16` reader - Interrupt configuration field for GPIO interrupt 16"]
pub type ICR16_R = crate::FieldReader<u8, ICR16_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR16_A {
    #[doc = "0: Interrupt 16 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 16 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 16 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 16 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR16_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR16_A) -> Self {
        variant as _
    }
}
impl ICR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR16_A {
        match self.bits {
            0 => ICR16_A::LOW_LEVEL,
            1 => ICR16_A::HIGH_LEVEL,
            2 => ICR16_A::RISING_EDGE,
            3 => ICR16_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR16_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR16_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR16_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR16_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR16` writer - Interrupt configuration field for GPIO interrupt 16"]
pub type ICR16_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR16_A, 2, O>;
impl<'a, const O: u8> ICR16_W<'a, O> {
    #[doc = "Interrupt 16 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR16_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 16 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR16_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 16 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR16_A::RISING_EDGE)
    }
    #[doc = "Interrupt 16 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR16_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR17` reader - Interrupt configuration field for GPIO interrupt 17"]
pub type ICR17_R = crate::FieldReader<u8, ICR17_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR17_A {
    #[doc = "0: Interrupt 17 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 17 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 17 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 17 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR17_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR17_A) -> Self {
        variant as _
    }
}
impl ICR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR17_A {
        match self.bits {
            0 => ICR17_A::LOW_LEVEL,
            1 => ICR17_A::HIGH_LEVEL,
            2 => ICR17_A::RISING_EDGE,
            3 => ICR17_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR17_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR17_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR17_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR17_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR17` writer - Interrupt configuration field for GPIO interrupt 17"]
pub type ICR17_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR17_A, 2, O>;
impl<'a, const O: u8> ICR17_W<'a, O> {
    #[doc = "Interrupt 17 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR17_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 17 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR17_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 17 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR17_A::RISING_EDGE)
    }
    #[doc = "Interrupt 17 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR17_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR18` reader - Interrupt configuration field for GPIO interrupt 18"]
pub type ICR18_R = crate::FieldReader<u8, ICR18_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR18_A {
    #[doc = "0: Interrupt 18 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 18 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 18 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 18 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR18_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR18_A) -> Self {
        variant as _
    }
}
impl ICR18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR18_A {
        match self.bits {
            0 => ICR18_A::LOW_LEVEL,
            1 => ICR18_A::HIGH_LEVEL,
            2 => ICR18_A::RISING_EDGE,
            3 => ICR18_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR18_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR18_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR18_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR18_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR18` writer - Interrupt configuration field for GPIO interrupt 18"]
pub type ICR18_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR18_A, 2, O>;
impl<'a, const O: u8> ICR18_W<'a, O> {
    #[doc = "Interrupt 18 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR18_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 18 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR18_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 18 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR18_A::RISING_EDGE)
    }
    #[doc = "Interrupt 18 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR18_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR19` reader - Interrupt configuration field for GPIO interrupt 19"]
pub type ICR19_R = crate::FieldReader<u8, ICR19_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR19_A {
    #[doc = "0: Interrupt 19 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 19 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 19 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 19 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR19_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR19_A) -> Self {
        variant as _
    }
}
impl ICR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR19_A {
        match self.bits {
            0 => ICR19_A::LOW_LEVEL,
            1 => ICR19_A::HIGH_LEVEL,
            2 => ICR19_A::RISING_EDGE,
            3 => ICR19_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR19_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR19_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR19_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR19_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR19` writer - Interrupt configuration field for GPIO interrupt 19"]
pub type ICR19_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR19_A, 2, O>;
impl<'a, const O: u8> ICR19_W<'a, O> {
    #[doc = "Interrupt 19 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR19_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 19 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR19_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 19 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR19_A::RISING_EDGE)
    }
    #[doc = "Interrupt 19 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR19_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR20` reader - Interrupt configuration field for GPIO interrupt 20"]
pub type ICR20_R = crate::FieldReader<u8, ICR20_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR20_A {
    #[doc = "0: Interrupt 20 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 20 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 20 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 20 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR20_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR20_A) -> Self {
        variant as _
    }
}
impl ICR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR20_A {
        match self.bits {
            0 => ICR20_A::LOW_LEVEL,
            1 => ICR20_A::HIGH_LEVEL,
            2 => ICR20_A::RISING_EDGE,
            3 => ICR20_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR20_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR20_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR20_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR20_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR20` writer - Interrupt configuration field for GPIO interrupt 20"]
pub type ICR20_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR20_A, 2, O>;
impl<'a, const O: u8> ICR20_W<'a, O> {
    #[doc = "Interrupt 20 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR20_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 20 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR20_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 20 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR20_A::RISING_EDGE)
    }
    #[doc = "Interrupt 20 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR20_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR21` reader - Interrupt configuration field for GPIO interrupt 21"]
pub type ICR21_R = crate::FieldReader<u8, ICR21_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR21_A {
    #[doc = "0: Interrupt 21 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 21 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 21 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 21 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR21_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR21_A) -> Self {
        variant as _
    }
}
impl ICR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR21_A {
        match self.bits {
            0 => ICR21_A::LOW_LEVEL,
            1 => ICR21_A::HIGH_LEVEL,
            2 => ICR21_A::RISING_EDGE,
            3 => ICR21_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR21_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR21_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR21_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR21_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR21` writer - Interrupt configuration field for GPIO interrupt 21"]
pub type ICR21_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR21_A, 2, O>;
impl<'a, const O: u8> ICR21_W<'a, O> {
    #[doc = "Interrupt 21 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR21_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 21 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR21_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 21 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR21_A::RISING_EDGE)
    }
    #[doc = "Interrupt 21 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR21_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR22` reader - Interrupt configuration field for GPIO interrupt 22"]
pub type ICR22_R = crate::FieldReader<u8, ICR22_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR22_A {
    #[doc = "0: Interrupt 22 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 22 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 22 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 22 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR22_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR22_A) -> Self {
        variant as _
    }
}
impl ICR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR22_A {
        match self.bits {
            0 => ICR22_A::LOW_LEVEL,
            1 => ICR22_A::HIGH_LEVEL,
            2 => ICR22_A::RISING_EDGE,
            3 => ICR22_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR22_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR22_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR22_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR22_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR22` writer - Interrupt configuration field for GPIO interrupt 22"]
pub type ICR22_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR22_A, 2, O>;
impl<'a, const O: u8> ICR22_W<'a, O> {
    #[doc = "Interrupt 22 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR22_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 22 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR22_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 22 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR22_A::RISING_EDGE)
    }
    #[doc = "Interrupt 22 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR22_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR23` reader - Interrupt configuration field for GPIO interrupt 23"]
pub type ICR23_R = crate::FieldReader<u8, ICR23_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR23_A {
    #[doc = "0: Interrupt 23 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 23 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 23 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 23 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR23_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR23_A) -> Self {
        variant as _
    }
}
impl ICR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR23_A {
        match self.bits {
            0 => ICR23_A::LOW_LEVEL,
            1 => ICR23_A::HIGH_LEVEL,
            2 => ICR23_A::RISING_EDGE,
            3 => ICR23_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR23_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR23_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR23_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR23_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR23` writer - Interrupt configuration field for GPIO interrupt 23"]
pub type ICR23_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR23_A, 2, O>;
impl<'a, const O: u8> ICR23_W<'a, O> {
    #[doc = "Interrupt 23 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR23_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 23 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR23_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 23 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR23_A::RISING_EDGE)
    }
    #[doc = "Interrupt 23 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR23_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR24` reader - Interrupt configuration field for GPIO interrupt 24"]
pub type ICR24_R = crate::FieldReader<u8, ICR24_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR24_A {
    #[doc = "0: Interrupt 24 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 24 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 24 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 24 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR24_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR24_A) -> Self {
        variant as _
    }
}
impl ICR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR24_A {
        match self.bits {
            0 => ICR24_A::LOW_LEVEL,
            1 => ICR24_A::HIGH_LEVEL,
            2 => ICR24_A::RISING_EDGE,
            3 => ICR24_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR24_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR24_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR24_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR24_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR24` writer - Interrupt configuration field for GPIO interrupt 24"]
pub type ICR24_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR24_A, 2, O>;
impl<'a, const O: u8> ICR24_W<'a, O> {
    #[doc = "Interrupt 24 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR24_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 24 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR24_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 24 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR24_A::RISING_EDGE)
    }
    #[doc = "Interrupt 24 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR24_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR25` reader - Interrupt configuration field for GPIO interrupt 25"]
pub type ICR25_R = crate::FieldReader<u8, ICR25_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR25_A {
    #[doc = "0: Interrupt 25 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 25 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 25 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 25 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR25_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR25_A) -> Self {
        variant as _
    }
}
impl ICR25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR25_A {
        match self.bits {
            0 => ICR25_A::LOW_LEVEL,
            1 => ICR25_A::HIGH_LEVEL,
            2 => ICR25_A::RISING_EDGE,
            3 => ICR25_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR25_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR25_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR25_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR25_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR25` writer - Interrupt configuration field for GPIO interrupt 25"]
pub type ICR25_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR25_A, 2, O>;
impl<'a, const O: u8> ICR25_W<'a, O> {
    #[doc = "Interrupt 25 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR25_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 25 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR25_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 25 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR25_A::RISING_EDGE)
    }
    #[doc = "Interrupt 25 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR25_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR26` reader - Interrupt configuration field for GPIO interrupt 26"]
pub type ICR26_R = crate::FieldReader<u8, ICR26_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR26_A {
    #[doc = "0: Interrupt 26 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 26 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 26 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 26 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR26_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR26_A) -> Self {
        variant as _
    }
}
impl ICR26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR26_A {
        match self.bits {
            0 => ICR26_A::LOW_LEVEL,
            1 => ICR26_A::HIGH_LEVEL,
            2 => ICR26_A::RISING_EDGE,
            3 => ICR26_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR26_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR26_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR26_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR26_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR26` writer - Interrupt configuration field for GPIO interrupt 26"]
pub type ICR26_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR26_A, 2, O>;
impl<'a, const O: u8> ICR26_W<'a, O> {
    #[doc = "Interrupt 26 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR26_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 26 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR26_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 26 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR26_A::RISING_EDGE)
    }
    #[doc = "Interrupt 26 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR26_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR27` reader - Interrupt configuration field for GPIO interrupt 27"]
pub type ICR27_R = crate::FieldReader<u8, ICR27_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR27_A {
    #[doc = "0: Interrupt 27 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 27 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 27 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 27 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR27_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR27_A) -> Self {
        variant as _
    }
}
impl ICR27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR27_A {
        match self.bits {
            0 => ICR27_A::LOW_LEVEL,
            1 => ICR27_A::HIGH_LEVEL,
            2 => ICR27_A::RISING_EDGE,
            3 => ICR27_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR27_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR27_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR27_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR27_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR27` writer - Interrupt configuration field for GPIO interrupt 27"]
pub type ICR27_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR27_A, 2, O>;
impl<'a, const O: u8> ICR27_W<'a, O> {
    #[doc = "Interrupt 27 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR27_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 27 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR27_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 27 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR27_A::RISING_EDGE)
    }
    #[doc = "Interrupt 27 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR27_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR28` reader - Interrupt configuration field for GPIO interrupt 28"]
pub type ICR28_R = crate::FieldReader<u8, ICR28_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR28_A {
    #[doc = "0: Interrupt 28 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 28 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 28 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 28 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR28_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR28_A) -> Self {
        variant as _
    }
}
impl ICR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR28_A {
        match self.bits {
            0 => ICR28_A::LOW_LEVEL,
            1 => ICR28_A::HIGH_LEVEL,
            2 => ICR28_A::RISING_EDGE,
            3 => ICR28_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR28_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR28_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR28_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR28_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR28` writer - Interrupt configuration field for GPIO interrupt 28"]
pub type ICR28_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR28_A, 2, O>;
impl<'a, const O: u8> ICR28_W<'a, O> {
    #[doc = "Interrupt 28 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR28_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 28 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR28_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 28 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR28_A::RISING_EDGE)
    }
    #[doc = "Interrupt 28 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR28_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR29` reader - Interrupt configuration field for GPIO interrupt 29"]
pub type ICR29_R = crate::FieldReader<u8, ICR29_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR29_A {
    #[doc = "0: Interrupt 29 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 29 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 29 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 29 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR29_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR29_A) -> Self {
        variant as _
    }
}
impl ICR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR29_A {
        match self.bits {
            0 => ICR29_A::LOW_LEVEL,
            1 => ICR29_A::HIGH_LEVEL,
            2 => ICR29_A::RISING_EDGE,
            3 => ICR29_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR29_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR29_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR29_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR29_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR29` writer - Interrupt configuration field for GPIO interrupt 29"]
pub type ICR29_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR29_A, 2, O>;
impl<'a, const O: u8> ICR29_W<'a, O> {
    #[doc = "Interrupt 29 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR29_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 29 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR29_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 29 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR29_A::RISING_EDGE)
    }
    #[doc = "Interrupt 29 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR29_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR30` reader - Interrupt configuration field for GPIO interrupt 30"]
pub type ICR30_R = crate::FieldReader<u8, ICR30_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR30_A {
    #[doc = "0: Interrupt 30 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 30 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 30 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 30 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR30_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR30_A) -> Self {
        variant as _
    }
}
impl ICR30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR30_A {
        match self.bits {
            0 => ICR30_A::LOW_LEVEL,
            1 => ICR30_A::HIGH_LEVEL,
            2 => ICR30_A::RISING_EDGE,
            3 => ICR30_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR30_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR30_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR30_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR30_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR30` writer - Interrupt configuration field for GPIO interrupt 30"]
pub type ICR30_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR30_A, 2, O>;
impl<'a, const O: u8> ICR30_W<'a, O> {
    #[doc = "Interrupt 30 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR30_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 30 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR30_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 30 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR30_A::RISING_EDGE)
    }
    #[doc = "Interrupt 30 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR30_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR31` reader - Interrupt configuration field for GPIO interrupt 31"]
pub type ICR31_R = crate::FieldReader<u8, ICR31_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR31_A {
    #[doc = "0: Interrupt 31 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 31 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 31 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 31 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR31_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR31_A) -> Self {
        variant as _
    }
}
impl ICR31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR31_A {
        match self.bits {
            0 => ICR31_A::LOW_LEVEL,
            1 => ICR31_A::HIGH_LEVEL,
            2 => ICR31_A::RISING_EDGE,
            3 => ICR31_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR31_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR31_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR31_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR31_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR31` writer - Interrupt configuration field for GPIO interrupt 31"]
pub type ICR31_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR2_SPEC, u8, ICR31_A, 2, O>;
impl<'a, const O: u8> ICR31_W<'a, O> {
    #[doc = "Interrupt 31 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR31_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 31 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR31_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 31 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR31_A::RISING_EDGE)
    }
    #[doc = "Interrupt 31 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR31_A::FALLING_EDGE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt configuration field for GPIO interrupt 16"]
    #[inline(always)]
    pub fn icr16(&self) -> ICR16_R {
        ICR16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt configuration field for GPIO interrupt 17"]
    #[inline(always)]
    pub fn icr17(&self) -> ICR17_R {
        ICR17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt configuration field for GPIO interrupt 18"]
    #[inline(always)]
    pub fn icr18(&self) -> ICR18_R {
        ICR18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt configuration field for GPIO interrupt 19"]
    #[inline(always)]
    pub fn icr19(&self) -> ICR19_R {
        ICR19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Interrupt configuration field for GPIO interrupt 20"]
    #[inline(always)]
    pub fn icr20(&self) -> ICR20_R {
        ICR20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Interrupt configuration field for GPIO interrupt 21"]
    #[inline(always)]
    pub fn icr21(&self) -> ICR21_R {
        ICR21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Interrupt configuration field for GPIO interrupt 22"]
    #[inline(always)]
    pub fn icr22(&self) -> ICR22_R {
        ICR22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Interrupt configuration field for GPIO interrupt 23"]
    #[inline(always)]
    pub fn icr23(&self) -> ICR23_R {
        ICR23_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Interrupt configuration field for GPIO interrupt 24"]
    #[inline(always)]
    pub fn icr24(&self) -> ICR24_R {
        ICR24_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Interrupt configuration field for GPIO interrupt 25"]
    #[inline(always)]
    pub fn icr25(&self) -> ICR25_R {
        ICR25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Interrupt configuration field for GPIO interrupt 26"]
    #[inline(always)]
    pub fn icr26(&self) -> ICR26_R {
        ICR26_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Interrupt configuration field for GPIO interrupt 27"]
    #[inline(always)]
    pub fn icr27(&self) -> ICR27_R {
        ICR27_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Interrupt configuration field for GPIO interrupt 28"]
    #[inline(always)]
    pub fn icr28(&self) -> ICR28_R {
        ICR28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Interrupt configuration field for GPIO interrupt 29"]
    #[inline(always)]
    pub fn icr29(&self) -> ICR29_R {
        ICR29_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Interrupt configuration field for GPIO interrupt 30"]
    #[inline(always)]
    pub fn icr30(&self) -> ICR30_R {
        ICR30_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Interrupt configuration field for GPIO interrupt 31"]
    #[inline(always)]
    pub fn icr31(&self) -> ICR31_R {
        ICR31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt configuration field for GPIO interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn icr16(&mut self) -> ICR16_W<0> {
        ICR16_W::new(self)
    }
    #[doc = "Bits 2:3 - Interrupt configuration field for GPIO interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn icr17(&mut self) -> ICR17_W<2> {
        ICR17_W::new(self)
    }
    #[doc = "Bits 4:5 - Interrupt configuration field for GPIO interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn icr18(&mut self) -> ICR18_W<4> {
        ICR18_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt configuration field for GPIO interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn icr19(&mut self) -> ICR19_W<6> {
        ICR19_W::new(self)
    }
    #[doc = "Bits 8:9 - Interrupt configuration field for GPIO interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn icr20(&mut self) -> ICR20_W<8> {
        ICR20_W::new(self)
    }
    #[doc = "Bits 10:11 - Interrupt configuration field for GPIO interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn icr21(&mut self) -> ICR21_W<10> {
        ICR21_W::new(self)
    }
    #[doc = "Bits 12:13 - Interrupt configuration field for GPIO interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn icr22(&mut self) -> ICR22_W<12> {
        ICR22_W::new(self)
    }
    #[doc = "Bits 14:15 - Interrupt configuration field for GPIO interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn icr23(&mut self) -> ICR23_W<14> {
        ICR23_W::new(self)
    }
    #[doc = "Bits 16:17 - Interrupt configuration field for GPIO interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn icr24(&mut self) -> ICR24_W<16> {
        ICR24_W::new(self)
    }
    #[doc = "Bits 18:19 - Interrupt configuration field for GPIO interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn icr25(&mut self) -> ICR25_W<18> {
        ICR25_W::new(self)
    }
    #[doc = "Bits 20:21 - Interrupt configuration field for GPIO interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn icr26(&mut self) -> ICR26_W<20> {
        ICR26_W::new(self)
    }
    #[doc = "Bits 22:23 - Interrupt configuration field for GPIO interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn icr27(&mut self) -> ICR27_W<22> {
        ICR27_W::new(self)
    }
    #[doc = "Bits 24:25 - Interrupt configuration field for GPIO interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn icr28(&mut self) -> ICR28_W<24> {
        ICR28_W::new(self)
    }
    #[doc = "Bits 26:27 - Interrupt configuration field for GPIO interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn icr29(&mut self) -> ICR29_W<26> {
        ICR29_W::new(self)
    }
    #[doc = "Bits 28:29 - Interrupt configuration field for GPIO interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn icr30(&mut self) -> ICR30_W<28> {
        ICR30_W::new(self)
    }
    #[doc = "Bits 30:31 - Interrupt configuration field for GPIO interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn icr31(&mut self) -> ICR31_W<30> {
        ICR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt configuration register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr2](index.html) module"]
pub struct ICR2_SPEC;
impl crate::RegisterSpec for ICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr2::R](R) reader structure"]
impl crate::Readable for ICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr2::W](W) writer structure"]
impl crate::Writable for ICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR2 to value 0"]
impl crate::Resettable for ICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
