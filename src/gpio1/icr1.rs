#[doc = "Register `ICR1` reader"]
pub struct R(crate::R<ICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR1` writer"]
pub struct W(crate::W<ICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR1_SPEC>;
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
impl From<crate::W<ICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICR0` reader - Interrupt configuration field for GPIO interrupt 0"]
pub type ICR0_R = crate::FieldReader<u8, ICR0_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR0_A {
    #[doc = "0: Interrupt 0 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 0 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 0 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 0 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR0_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR0_A) -> Self {
        variant as _
    }
}
impl ICR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR0_A {
        match self.bits {
            0 => ICR0_A::LOW_LEVEL,
            1 => ICR0_A::HIGH_LEVEL,
            2 => ICR0_A::RISING_EDGE,
            3 => ICR0_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR0_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR0_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR0_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR0_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR0` writer - Interrupt configuration field for GPIO interrupt 0"]
pub type ICR0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR0_A, 2, O>;
impl<'a, const O: u8> ICR0_W<'a, O> {
    #[doc = "Interrupt 0 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR0_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 0 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR0_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 0 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR0_A::RISING_EDGE)
    }
    #[doc = "Interrupt 0 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR0_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR1` reader - Interrupt configuration field for GPIO interrupt 1"]
pub type ICR1_R = crate::FieldReader<u8, ICR1_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR1_A {
    #[doc = "0: Interrupt 1 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 1 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 1 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 1 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR1_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR1_A) -> Self {
        variant as _
    }
}
impl ICR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR1_A {
        match self.bits {
            0 => ICR1_A::LOW_LEVEL,
            1 => ICR1_A::HIGH_LEVEL,
            2 => ICR1_A::RISING_EDGE,
            3 => ICR1_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR1_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR1_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR1_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR1` writer - Interrupt configuration field for GPIO interrupt 1"]
pub type ICR1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR1_A, 2, O>;
impl<'a, const O: u8> ICR1_W<'a, O> {
    #[doc = "Interrupt 1 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR1_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 1 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR1_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 1 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR1_A::RISING_EDGE)
    }
    #[doc = "Interrupt 1 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR1_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR2` reader - Interrupt configuration field for GPIO interrupt 2"]
pub type ICR2_R = crate::FieldReader<u8, ICR2_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR2_A {
    #[doc = "0: Interrupt 2 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 2 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 2 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 2 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR2_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR2_A) -> Self {
        variant as _
    }
}
impl ICR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR2_A {
        match self.bits {
            0 => ICR2_A::LOW_LEVEL,
            1 => ICR2_A::HIGH_LEVEL,
            2 => ICR2_A::RISING_EDGE,
            3 => ICR2_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR2_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR2_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR2_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR2_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR2` writer - Interrupt configuration field for GPIO interrupt 2"]
pub type ICR2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR2_A, 2, O>;
impl<'a, const O: u8> ICR2_W<'a, O> {
    #[doc = "Interrupt 2 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR2_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 2 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR2_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 2 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR2_A::RISING_EDGE)
    }
    #[doc = "Interrupt 2 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR2_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR3` reader - Interrupt configuration field for GPIO interrupt 3"]
pub type ICR3_R = crate::FieldReader<u8, ICR3_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR3_A {
    #[doc = "0: Interrupt 3 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 3 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 3 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 3 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR3_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR3_A) -> Self {
        variant as _
    }
}
impl ICR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR3_A {
        match self.bits {
            0 => ICR3_A::LOW_LEVEL,
            1 => ICR3_A::HIGH_LEVEL,
            2 => ICR3_A::RISING_EDGE,
            3 => ICR3_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR3_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR3_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR3_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR3_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR3` writer - Interrupt configuration field for GPIO interrupt 3"]
pub type ICR3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR3_A, 2, O>;
impl<'a, const O: u8> ICR3_W<'a, O> {
    #[doc = "Interrupt 3 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR3_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 3 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR3_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 3 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR3_A::RISING_EDGE)
    }
    #[doc = "Interrupt 3 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR3_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR4` reader - Interrupt configuration field for GPIO interrupt 4"]
pub type ICR4_R = crate::FieldReader<u8, ICR4_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR4_A {
    #[doc = "0: Interrupt 4 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 4 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 4 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 4 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR4_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR4_A) -> Self {
        variant as _
    }
}
impl ICR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR4_A {
        match self.bits {
            0 => ICR4_A::LOW_LEVEL,
            1 => ICR4_A::HIGH_LEVEL,
            2 => ICR4_A::RISING_EDGE,
            3 => ICR4_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR4_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR4_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR4_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR4_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR4` writer - Interrupt configuration field for GPIO interrupt 4"]
pub type ICR4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR4_A, 2, O>;
impl<'a, const O: u8> ICR4_W<'a, O> {
    #[doc = "Interrupt 4 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR4_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 4 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR4_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 4 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR4_A::RISING_EDGE)
    }
    #[doc = "Interrupt 4 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR4_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR5` reader - Interrupt configuration field for GPIO interrupt 5"]
pub type ICR5_R = crate::FieldReader<u8, ICR5_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR5_A {
    #[doc = "0: Interrupt 5 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 5 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 5 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 5 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR5_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR5_A) -> Self {
        variant as _
    }
}
impl ICR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR5_A {
        match self.bits {
            0 => ICR5_A::LOW_LEVEL,
            1 => ICR5_A::HIGH_LEVEL,
            2 => ICR5_A::RISING_EDGE,
            3 => ICR5_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR5_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR5_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR5_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR5_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR5` writer - Interrupt configuration field for GPIO interrupt 5"]
pub type ICR5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR5_A, 2, O>;
impl<'a, const O: u8> ICR5_W<'a, O> {
    #[doc = "Interrupt 5 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR5_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 5 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR5_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 5 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR5_A::RISING_EDGE)
    }
    #[doc = "Interrupt 5 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR5_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR6` reader - Interrupt configuration field for GPIO interrupt 6"]
pub type ICR6_R = crate::FieldReader<u8, ICR6_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR6_A {
    #[doc = "0: Interrupt 6 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 6 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 6 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 6 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR6_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR6_A) -> Self {
        variant as _
    }
}
impl ICR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR6_A {
        match self.bits {
            0 => ICR6_A::LOW_LEVEL,
            1 => ICR6_A::HIGH_LEVEL,
            2 => ICR6_A::RISING_EDGE,
            3 => ICR6_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR6_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR6_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR6_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR6_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR6` writer - Interrupt configuration field for GPIO interrupt 6"]
pub type ICR6_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR6_A, 2, O>;
impl<'a, const O: u8> ICR6_W<'a, O> {
    #[doc = "Interrupt 6 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR6_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 6 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR6_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 6 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR6_A::RISING_EDGE)
    }
    #[doc = "Interrupt 6 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR6_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR7` reader - Interrupt configuration field for GPIO interrupt 7"]
pub type ICR7_R = crate::FieldReader<u8, ICR7_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR7_A {
    #[doc = "0: Interrupt 7 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 7 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 7 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 7 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR7_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR7_A) -> Self {
        variant as _
    }
}
impl ICR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR7_A {
        match self.bits {
            0 => ICR7_A::LOW_LEVEL,
            1 => ICR7_A::HIGH_LEVEL,
            2 => ICR7_A::RISING_EDGE,
            3 => ICR7_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR7_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR7_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR7_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR7_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR7` writer - Interrupt configuration field for GPIO interrupt 7"]
pub type ICR7_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR7_A, 2, O>;
impl<'a, const O: u8> ICR7_W<'a, O> {
    #[doc = "Interrupt 7 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR7_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 7 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR7_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 7 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR7_A::RISING_EDGE)
    }
    #[doc = "Interrupt 7 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR7_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR8` reader - Interrupt configuration field for GPIO interrupt 8"]
pub type ICR8_R = crate::FieldReader<u8, ICR8_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR8_A {
    #[doc = "0: Interrupt 8 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 8 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 8 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 8 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR8_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR8_A) -> Self {
        variant as _
    }
}
impl ICR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR8_A {
        match self.bits {
            0 => ICR8_A::LOW_LEVEL,
            1 => ICR8_A::HIGH_LEVEL,
            2 => ICR8_A::RISING_EDGE,
            3 => ICR8_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR8_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR8_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR8_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR8_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR8` writer - Interrupt configuration field for GPIO interrupt 8"]
pub type ICR8_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR8_A, 2, O>;
impl<'a, const O: u8> ICR8_W<'a, O> {
    #[doc = "Interrupt 8 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR8_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 8 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR8_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 8 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR8_A::RISING_EDGE)
    }
    #[doc = "Interrupt 8 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR8_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR9` reader - Interrupt configuration field for GPIO interrupt 9"]
pub type ICR9_R = crate::FieldReader<u8, ICR9_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR9_A {
    #[doc = "0: Interrupt 9 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 9 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 9 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 9 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR9_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR9_A) -> Self {
        variant as _
    }
}
impl ICR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR9_A {
        match self.bits {
            0 => ICR9_A::LOW_LEVEL,
            1 => ICR9_A::HIGH_LEVEL,
            2 => ICR9_A::RISING_EDGE,
            3 => ICR9_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR9_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR9_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR9_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR9_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR9` writer - Interrupt configuration field for GPIO interrupt 9"]
pub type ICR9_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR9_A, 2, O>;
impl<'a, const O: u8> ICR9_W<'a, O> {
    #[doc = "Interrupt 9 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR9_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 9 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR9_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 9 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR9_A::RISING_EDGE)
    }
    #[doc = "Interrupt 9 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR9_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR10` reader - Interrupt configuration field for GPIO interrupt 10"]
pub type ICR10_R = crate::FieldReader<u8, ICR10_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR10_A {
    #[doc = "0: Interrupt 10 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 10 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 10 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 10 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR10_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR10_A) -> Self {
        variant as _
    }
}
impl ICR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR10_A {
        match self.bits {
            0 => ICR10_A::LOW_LEVEL,
            1 => ICR10_A::HIGH_LEVEL,
            2 => ICR10_A::RISING_EDGE,
            3 => ICR10_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR10_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR10_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR10_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR10_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR10` writer - Interrupt configuration field for GPIO interrupt 10"]
pub type ICR10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR10_A, 2, O>;
impl<'a, const O: u8> ICR10_W<'a, O> {
    #[doc = "Interrupt 10 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR10_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 10 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR10_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 10 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR10_A::RISING_EDGE)
    }
    #[doc = "Interrupt 10 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR10_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR11` reader - Interrupt configuration field for GPIO interrupt 11"]
pub type ICR11_R = crate::FieldReader<u8, ICR11_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR11_A {
    #[doc = "0: Interrupt 11 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 11 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 11 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 11 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR11_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR11_A) -> Self {
        variant as _
    }
}
impl ICR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR11_A {
        match self.bits {
            0 => ICR11_A::LOW_LEVEL,
            1 => ICR11_A::HIGH_LEVEL,
            2 => ICR11_A::RISING_EDGE,
            3 => ICR11_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR11_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR11_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR11_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR11_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR11` writer - Interrupt configuration field for GPIO interrupt 11"]
pub type ICR11_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR11_A, 2, O>;
impl<'a, const O: u8> ICR11_W<'a, O> {
    #[doc = "Interrupt 11 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR11_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 11 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR11_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 11 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR11_A::RISING_EDGE)
    }
    #[doc = "Interrupt 11 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR11_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR12` reader - Interrupt configuration field for GPIO interrupt 12"]
pub type ICR12_R = crate::FieldReader<u8, ICR12_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR12_A {
    #[doc = "0: Interrupt 12 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 12 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 12 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 12 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR12_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR12_A) -> Self {
        variant as _
    }
}
impl ICR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR12_A {
        match self.bits {
            0 => ICR12_A::LOW_LEVEL,
            1 => ICR12_A::HIGH_LEVEL,
            2 => ICR12_A::RISING_EDGE,
            3 => ICR12_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR12_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR12_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR12_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR12_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR12` writer - Interrupt configuration field for GPIO interrupt 12"]
pub type ICR12_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR12_A, 2, O>;
impl<'a, const O: u8> ICR12_W<'a, O> {
    #[doc = "Interrupt 12 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR12_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 12 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR12_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 12 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR12_A::RISING_EDGE)
    }
    #[doc = "Interrupt 12 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR12_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR13` reader - Interrupt configuration field for GPIO interrupt 13"]
pub type ICR13_R = crate::FieldReader<u8, ICR13_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR13_A {
    #[doc = "0: Interrupt 13 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 13 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 13 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 13 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR13_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR13_A) -> Self {
        variant as _
    }
}
impl ICR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR13_A {
        match self.bits {
            0 => ICR13_A::LOW_LEVEL,
            1 => ICR13_A::HIGH_LEVEL,
            2 => ICR13_A::RISING_EDGE,
            3 => ICR13_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR13_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR13_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR13_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR13_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR13` writer - Interrupt configuration field for GPIO interrupt 13"]
pub type ICR13_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR13_A, 2, O>;
impl<'a, const O: u8> ICR13_W<'a, O> {
    #[doc = "Interrupt 13 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR13_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 13 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR13_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 13 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR13_A::RISING_EDGE)
    }
    #[doc = "Interrupt 13 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR13_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR14` reader - Interrupt configuration field for GPIO interrupt 14"]
pub type ICR14_R = crate::FieldReader<u8, ICR14_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR14_A {
    #[doc = "0: Interrupt 14 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 14 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 14 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 14 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR14_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR14_A) -> Self {
        variant as _
    }
}
impl ICR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR14_A {
        match self.bits {
            0 => ICR14_A::LOW_LEVEL,
            1 => ICR14_A::HIGH_LEVEL,
            2 => ICR14_A::RISING_EDGE,
            3 => ICR14_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR14_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR14_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR14_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR14_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR14` writer - Interrupt configuration field for GPIO interrupt 14"]
pub type ICR14_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR14_A, 2, O>;
impl<'a, const O: u8> ICR14_W<'a, O> {
    #[doc = "Interrupt 14 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR14_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 14 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR14_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 14 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR14_A::RISING_EDGE)
    }
    #[doc = "Interrupt 14 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR14_A::FALLING_EDGE)
    }
}
#[doc = "Field `ICR15` reader - Interrupt configuration field for GPIO interrupt 15"]
pub type ICR15_R = crate::FieldReader<u8, ICR15_A>;
#[doc = "Interrupt configuration field for GPIO interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICR15_A {
    #[doc = "0: Interrupt 15 is low-level sensitive."]
    LOW_LEVEL = 0,
    #[doc = "1: Interrupt 15 is high-level sensitive."]
    HIGH_LEVEL = 1,
    #[doc = "2: Interrupt 15 is rising-edge sensitive."]
    RISING_EDGE = 2,
    #[doc = "3: Interrupt 15 is falling-edge sensitive."]
    FALLING_EDGE = 3,
}
impl From<ICR15_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR15_A) -> Self {
        variant as _
    }
}
impl ICR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR15_A {
        match self.bits {
            0 => ICR15_A::LOW_LEVEL,
            1 => ICR15_A::HIGH_LEVEL,
            2 => ICR15_A::RISING_EDGE,
            3 => ICR15_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR15_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR15_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR15_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR15_A::FALLING_EDGE
    }
}
#[doc = "Field `ICR15` writer - Interrupt configuration field for GPIO interrupt 15"]
pub type ICR15_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ICR1_SPEC, u8, ICR15_A, 2, O>;
impl<'a, const O: u8> ICR15_W<'a, O> {
    #[doc = "Interrupt 15 is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR15_A::LOW_LEVEL)
    }
    #[doc = "Interrupt 15 is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR15_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt 15 is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR15_A::RISING_EDGE)
    }
    #[doc = "Interrupt 15 is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR15_A::FALLING_EDGE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt configuration field for GPIO interrupt 0"]
    #[inline(always)]
    pub fn icr0(&self) -> ICR0_R {
        ICR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt configuration field for GPIO interrupt 1"]
    #[inline(always)]
    pub fn icr1(&self) -> ICR1_R {
        ICR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt configuration field for GPIO interrupt 2"]
    #[inline(always)]
    pub fn icr2(&self) -> ICR2_R {
        ICR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt configuration field for GPIO interrupt 3"]
    #[inline(always)]
    pub fn icr3(&self) -> ICR3_R {
        ICR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Interrupt configuration field for GPIO interrupt 4"]
    #[inline(always)]
    pub fn icr4(&self) -> ICR4_R {
        ICR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Interrupt configuration field for GPIO interrupt 5"]
    #[inline(always)]
    pub fn icr5(&self) -> ICR5_R {
        ICR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Interrupt configuration field for GPIO interrupt 6"]
    #[inline(always)]
    pub fn icr6(&self) -> ICR6_R {
        ICR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Interrupt configuration field for GPIO interrupt 7"]
    #[inline(always)]
    pub fn icr7(&self) -> ICR7_R {
        ICR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Interrupt configuration field for GPIO interrupt 8"]
    #[inline(always)]
    pub fn icr8(&self) -> ICR8_R {
        ICR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Interrupt configuration field for GPIO interrupt 9"]
    #[inline(always)]
    pub fn icr9(&self) -> ICR9_R {
        ICR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Interrupt configuration field for GPIO interrupt 10"]
    #[inline(always)]
    pub fn icr10(&self) -> ICR10_R {
        ICR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Interrupt configuration field for GPIO interrupt 11"]
    #[inline(always)]
    pub fn icr11(&self) -> ICR11_R {
        ICR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Interrupt configuration field for GPIO interrupt 12"]
    #[inline(always)]
    pub fn icr12(&self) -> ICR12_R {
        ICR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Interrupt configuration field for GPIO interrupt 13"]
    #[inline(always)]
    pub fn icr13(&self) -> ICR13_R {
        ICR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Interrupt configuration field for GPIO interrupt 14"]
    #[inline(always)]
    pub fn icr14(&self) -> ICR14_R {
        ICR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Interrupt configuration field for GPIO interrupt 15"]
    #[inline(always)]
    pub fn icr15(&self) -> ICR15_R {
        ICR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt configuration field for GPIO interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn icr0(&mut self) -> ICR0_W<0> {
        ICR0_W::new(self)
    }
    #[doc = "Bits 2:3 - Interrupt configuration field for GPIO interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn icr1(&mut self) -> ICR1_W<2> {
        ICR1_W::new(self)
    }
    #[doc = "Bits 4:5 - Interrupt configuration field for GPIO interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn icr2(&mut self) -> ICR2_W<4> {
        ICR2_W::new(self)
    }
    #[doc = "Bits 6:7 - Interrupt configuration field for GPIO interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn icr3(&mut self) -> ICR3_W<6> {
        ICR3_W::new(self)
    }
    #[doc = "Bits 8:9 - Interrupt configuration field for GPIO interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn icr4(&mut self) -> ICR4_W<8> {
        ICR4_W::new(self)
    }
    #[doc = "Bits 10:11 - Interrupt configuration field for GPIO interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn icr5(&mut self) -> ICR5_W<10> {
        ICR5_W::new(self)
    }
    #[doc = "Bits 12:13 - Interrupt configuration field for GPIO interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn icr6(&mut self) -> ICR6_W<12> {
        ICR6_W::new(self)
    }
    #[doc = "Bits 14:15 - Interrupt configuration field for GPIO interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn icr7(&mut self) -> ICR7_W<14> {
        ICR7_W::new(self)
    }
    #[doc = "Bits 16:17 - Interrupt configuration field for GPIO interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn icr8(&mut self) -> ICR8_W<16> {
        ICR8_W::new(self)
    }
    #[doc = "Bits 18:19 - Interrupt configuration field for GPIO interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn icr9(&mut self) -> ICR9_W<18> {
        ICR9_W::new(self)
    }
    #[doc = "Bits 20:21 - Interrupt configuration field for GPIO interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn icr10(&mut self) -> ICR10_W<20> {
        ICR10_W::new(self)
    }
    #[doc = "Bits 22:23 - Interrupt configuration field for GPIO interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn icr11(&mut self) -> ICR11_W<22> {
        ICR11_W::new(self)
    }
    #[doc = "Bits 24:25 - Interrupt configuration field for GPIO interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn icr12(&mut self) -> ICR12_W<24> {
        ICR12_W::new(self)
    }
    #[doc = "Bits 26:27 - Interrupt configuration field for GPIO interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn icr13(&mut self) -> ICR13_W<26> {
        ICR13_W::new(self)
    }
    #[doc = "Bits 28:29 - Interrupt configuration field for GPIO interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn icr14(&mut self) -> ICR14_W<28> {
        ICR14_W::new(self)
    }
    #[doc = "Bits 30:31 - Interrupt configuration field for GPIO interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn icr15(&mut self) -> ICR15_W<30> {
        ICR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt configuration register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](index.html) module"]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr1::R](R) reader structure"]
impl crate::Readable for ICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr1::W](W) writer structure"]
impl crate::Writable for ICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
