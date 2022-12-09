#[doc = "Register `ERQ` reader"]
pub struct R(crate::R<ERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERQ` writer"]
pub struct W(crate::W<ERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERQ_SPEC>;
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
impl From<crate::W<ERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERQ0` reader - Enable DMA Request 0"]
pub type ERQ0_R = crate::BitReader<ERQ0_A>;
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for channel 0 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 0 is enabled"]
    ENABLE = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::DISABLE,
            true => ERQ0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ0_A::ENABLE
    }
}
#[doc = "Field `ERQ0` writer - Enable DMA Request 0"]
pub type ERQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ0_A, O>;
impl<'a, const O: u8> ERQ0_W<'a, O> {
    #[doc = "The DMA request signal for channel 0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ0_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 0 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ0_A::ENABLE)
    }
}
#[doc = "Field `ERQ1` reader - Enable DMA Request 1"]
pub type ERQ1_R = crate::BitReader<ERQ1_A>;
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for channel 1 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 1 is enabled"]
    ENABLE = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::DISABLE,
            true => ERQ1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ1_A::ENABLE
    }
}
#[doc = "Field `ERQ1` writer - Enable DMA Request 1"]
pub type ERQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ1_A, O>;
impl<'a, const O: u8> ERQ1_W<'a, O> {
    #[doc = "The DMA request signal for channel 1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ1_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 1 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ1_A::ENABLE)
    }
}
#[doc = "Field `ERQ2` reader - Enable DMA Request 2"]
pub type ERQ2_R = crate::BitReader<ERQ2_A>;
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for channel 2 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 2 is enabled"]
    ENABLE = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::DISABLE,
            true => ERQ2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ2_A::ENABLE
    }
}
#[doc = "Field `ERQ2` writer - Enable DMA Request 2"]
pub type ERQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ2_A, O>;
impl<'a, const O: u8> ERQ2_W<'a, O> {
    #[doc = "The DMA request signal for channel 2 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ2_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 2 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ2_A::ENABLE)
    }
}
#[doc = "Field `ERQ3` reader - Enable DMA Request 3"]
pub type ERQ3_R = crate::BitReader<ERQ3_A>;
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for channel 3 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 3 is enabled"]
    ENABLE = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::DISABLE,
            true => ERQ3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ3_A::ENABLE
    }
}
#[doc = "Field `ERQ3` writer - Enable DMA Request 3"]
pub type ERQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ3_A, O>;
impl<'a, const O: u8> ERQ3_W<'a, O> {
    #[doc = "The DMA request signal for channel 3 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ3_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 3 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ3_A::ENABLE)
    }
}
#[doc = "Field `ERQ4` reader - Enable DMA Request 4"]
pub type ERQ4_R = crate::BitReader<ERQ4_A>;
#[doc = "Enable DMA Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ4_A {
    #[doc = "0: The DMA request signal for channel 4 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 4 is enabled"]
    ENABLE = 1,
}
impl From<ERQ4_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ4_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ4_A {
        match self.bits {
            false => ERQ4_A::DISABLE,
            true => ERQ4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ4_A::ENABLE
    }
}
#[doc = "Field `ERQ4` writer - Enable DMA Request 4"]
pub type ERQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ4_A, O>;
impl<'a, const O: u8> ERQ4_W<'a, O> {
    #[doc = "The DMA request signal for channel 4 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ4_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 4 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ4_A::ENABLE)
    }
}
#[doc = "Field `ERQ5` reader - Enable DMA Request 5"]
pub type ERQ5_R = crate::BitReader<ERQ5_A>;
#[doc = "Enable DMA Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ5_A {
    #[doc = "0: The DMA request signal for channel 5 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 5 is enabled"]
    ENABLE = 1,
}
impl From<ERQ5_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ5_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ5_A {
        match self.bits {
            false => ERQ5_A::DISABLE,
            true => ERQ5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ5_A::ENABLE
    }
}
#[doc = "Field `ERQ5` writer - Enable DMA Request 5"]
pub type ERQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ5_A, O>;
impl<'a, const O: u8> ERQ5_W<'a, O> {
    #[doc = "The DMA request signal for channel 5 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ5_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 5 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ5_A::ENABLE)
    }
}
#[doc = "Field `ERQ6` reader - Enable DMA Request 6"]
pub type ERQ6_R = crate::BitReader<ERQ6_A>;
#[doc = "Enable DMA Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ6_A {
    #[doc = "0: The DMA request signal for channel 6 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 6 is enabled"]
    ENABLE = 1,
}
impl From<ERQ6_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ6_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ6_A {
        match self.bits {
            false => ERQ6_A::DISABLE,
            true => ERQ6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ6_A::ENABLE
    }
}
#[doc = "Field `ERQ6` writer - Enable DMA Request 6"]
pub type ERQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ6_A, O>;
impl<'a, const O: u8> ERQ6_W<'a, O> {
    #[doc = "The DMA request signal for channel 6 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ6_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 6 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ6_A::ENABLE)
    }
}
#[doc = "Field `ERQ7` reader - Enable DMA Request 7"]
pub type ERQ7_R = crate::BitReader<ERQ7_A>;
#[doc = "Enable DMA Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ7_A {
    #[doc = "0: The DMA request signal for channel 7 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 7 is enabled"]
    ENABLE = 1,
}
impl From<ERQ7_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ7_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ7_A {
        match self.bits {
            false => ERQ7_A::DISABLE,
            true => ERQ7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ7_A::ENABLE
    }
}
#[doc = "Field `ERQ7` writer - Enable DMA Request 7"]
pub type ERQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ7_A, O>;
impl<'a, const O: u8> ERQ7_W<'a, O> {
    #[doc = "The DMA request signal for channel 7 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ7_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 7 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ7_A::ENABLE)
    }
}
#[doc = "Field `ERQ8` reader - Enable DMA Request 8"]
pub type ERQ8_R = crate::BitReader<ERQ8_A>;
#[doc = "Enable DMA Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ8_A {
    #[doc = "0: The DMA request signal for channel 8 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 8 is enabled"]
    ENABLE = 1,
}
impl From<ERQ8_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ8_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ8_A {
        match self.bits {
            false => ERQ8_A::DISABLE,
            true => ERQ8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ8_A::ENABLE
    }
}
#[doc = "Field `ERQ8` writer - Enable DMA Request 8"]
pub type ERQ8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ8_A, O>;
impl<'a, const O: u8> ERQ8_W<'a, O> {
    #[doc = "The DMA request signal for channel 8 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ8_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 8 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ8_A::ENABLE)
    }
}
#[doc = "Field `ERQ9` reader - Enable DMA Request 9"]
pub type ERQ9_R = crate::BitReader<ERQ9_A>;
#[doc = "Enable DMA Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ9_A {
    #[doc = "0: The DMA request signal for channel 9 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 9 is enabled"]
    ENABLE = 1,
}
impl From<ERQ9_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ9_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ9_A {
        match self.bits {
            false => ERQ9_A::DISABLE,
            true => ERQ9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ9_A::ENABLE
    }
}
#[doc = "Field `ERQ9` writer - Enable DMA Request 9"]
pub type ERQ9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ9_A, O>;
impl<'a, const O: u8> ERQ9_W<'a, O> {
    #[doc = "The DMA request signal for channel 9 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ9_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 9 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ9_A::ENABLE)
    }
}
#[doc = "Field `ERQ10` reader - Enable DMA Request 10"]
pub type ERQ10_R = crate::BitReader<ERQ10_A>;
#[doc = "Enable DMA Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ10_A {
    #[doc = "0: The DMA request signal for channel 10 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 10 is enabled"]
    ENABLE = 1,
}
impl From<ERQ10_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ10_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ10_A {
        match self.bits {
            false => ERQ10_A::DISABLE,
            true => ERQ10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ10_A::ENABLE
    }
}
#[doc = "Field `ERQ10` writer - Enable DMA Request 10"]
pub type ERQ10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ10_A, O>;
impl<'a, const O: u8> ERQ10_W<'a, O> {
    #[doc = "The DMA request signal for channel 10 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ10_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 10 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ10_A::ENABLE)
    }
}
#[doc = "Field `ERQ11` reader - Enable DMA Request 11"]
pub type ERQ11_R = crate::BitReader<ERQ11_A>;
#[doc = "Enable DMA Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ11_A {
    #[doc = "0: The DMA request signal for channel 11 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 11 is enabled"]
    ENABLE = 1,
}
impl From<ERQ11_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ11_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ11_A {
        match self.bits {
            false => ERQ11_A::DISABLE,
            true => ERQ11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ11_A::ENABLE
    }
}
#[doc = "Field `ERQ11` writer - Enable DMA Request 11"]
pub type ERQ11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ11_A, O>;
impl<'a, const O: u8> ERQ11_W<'a, O> {
    #[doc = "The DMA request signal for channel 11 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ11_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 11 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ11_A::ENABLE)
    }
}
#[doc = "Field `ERQ12` reader - Enable DMA Request 12"]
pub type ERQ12_R = crate::BitReader<ERQ12_A>;
#[doc = "Enable DMA Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ12_A {
    #[doc = "0: The DMA request signal for channel 12 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 12 is enabled"]
    ENABLE = 1,
}
impl From<ERQ12_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ12_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ12_A {
        match self.bits {
            false => ERQ12_A::DISABLE,
            true => ERQ12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ12_A::ENABLE
    }
}
#[doc = "Field `ERQ12` writer - Enable DMA Request 12"]
pub type ERQ12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ12_A, O>;
impl<'a, const O: u8> ERQ12_W<'a, O> {
    #[doc = "The DMA request signal for channel 12 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ12_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 12 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ12_A::ENABLE)
    }
}
#[doc = "Field `ERQ13` reader - Enable DMA Request 13"]
pub type ERQ13_R = crate::BitReader<ERQ13_A>;
#[doc = "Enable DMA Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ13_A {
    #[doc = "0: The DMA request signal for channel 13 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 13 is enabled"]
    ENABLE = 1,
}
impl From<ERQ13_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ13_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ13_A {
        match self.bits {
            false => ERQ13_A::DISABLE,
            true => ERQ13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ13_A::ENABLE
    }
}
#[doc = "Field `ERQ13` writer - Enable DMA Request 13"]
pub type ERQ13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ13_A, O>;
impl<'a, const O: u8> ERQ13_W<'a, O> {
    #[doc = "The DMA request signal for channel 13 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ13_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 13 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ13_A::ENABLE)
    }
}
#[doc = "Field `ERQ14` reader - Enable DMA Request 14"]
pub type ERQ14_R = crate::BitReader<ERQ14_A>;
#[doc = "Enable DMA Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ14_A {
    #[doc = "0: The DMA request signal for channel 14 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 14 is enabled"]
    ENABLE = 1,
}
impl From<ERQ14_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ14_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ14_A {
        match self.bits {
            false => ERQ14_A::DISABLE,
            true => ERQ14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ14_A::ENABLE
    }
}
#[doc = "Field `ERQ14` writer - Enable DMA Request 14"]
pub type ERQ14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ14_A, O>;
impl<'a, const O: u8> ERQ14_W<'a, O> {
    #[doc = "The DMA request signal for channel 14 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ14_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 14 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ14_A::ENABLE)
    }
}
#[doc = "Field `ERQ15` reader - Enable DMA Request 15"]
pub type ERQ15_R = crate::BitReader<ERQ15_A>;
#[doc = "Enable DMA Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ15_A {
    #[doc = "0: The DMA request signal for channel 15 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 15 is enabled"]
    ENABLE = 1,
}
impl From<ERQ15_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ15_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ15_A {
        match self.bits {
            false => ERQ15_A::DISABLE,
            true => ERQ15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ15_A::ENABLE
    }
}
#[doc = "Field `ERQ15` writer - Enable DMA Request 15"]
pub type ERQ15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ15_A, O>;
impl<'a, const O: u8> ERQ15_W<'a, O> {
    #[doc = "The DMA request signal for channel 15 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ15_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 15 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ15_A::ENABLE)
    }
}
#[doc = "Field `ERQ16` reader - Enable DMA Request 16"]
pub type ERQ16_R = crate::BitReader<ERQ16_A>;
#[doc = "Enable DMA Request 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ16_A {
    #[doc = "0: The DMA request signal for channel 16 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 16 is enabled"]
    ENABLE = 1,
}
impl From<ERQ16_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ16_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ16_A {
        match self.bits {
            false => ERQ16_A::DISABLE,
            true => ERQ16_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ16_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ16_A::ENABLE
    }
}
#[doc = "Field `ERQ16` writer - Enable DMA Request 16"]
pub type ERQ16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ16_A, O>;
impl<'a, const O: u8> ERQ16_W<'a, O> {
    #[doc = "The DMA request signal for channel 16 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ16_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 16 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ16_A::ENABLE)
    }
}
#[doc = "Field `ERQ17` reader - Enable DMA Request 17"]
pub type ERQ17_R = crate::BitReader<ERQ17_A>;
#[doc = "Enable DMA Request 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ17_A {
    #[doc = "0: The DMA request signal for channel 17 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 17 is enabled"]
    ENABLE = 1,
}
impl From<ERQ17_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ17_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ17_A {
        match self.bits {
            false => ERQ17_A::DISABLE,
            true => ERQ17_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ17_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ17_A::ENABLE
    }
}
#[doc = "Field `ERQ17` writer - Enable DMA Request 17"]
pub type ERQ17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ17_A, O>;
impl<'a, const O: u8> ERQ17_W<'a, O> {
    #[doc = "The DMA request signal for channel 17 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ17_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 17 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ17_A::ENABLE)
    }
}
#[doc = "Field `ERQ18` reader - Enable DMA Request 18"]
pub type ERQ18_R = crate::BitReader<ERQ18_A>;
#[doc = "Enable DMA Request 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ18_A {
    #[doc = "0: The DMA request signal for channel 18 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 18 is enabled"]
    ENABLE = 1,
}
impl From<ERQ18_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ18_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ18_A {
        match self.bits {
            false => ERQ18_A::DISABLE,
            true => ERQ18_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ18_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ18_A::ENABLE
    }
}
#[doc = "Field `ERQ18` writer - Enable DMA Request 18"]
pub type ERQ18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ18_A, O>;
impl<'a, const O: u8> ERQ18_W<'a, O> {
    #[doc = "The DMA request signal for channel 18 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ18_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 18 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ18_A::ENABLE)
    }
}
#[doc = "Field `ERQ19` reader - Enable DMA Request 19"]
pub type ERQ19_R = crate::BitReader<ERQ19_A>;
#[doc = "Enable DMA Request 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ19_A {
    #[doc = "0: The DMA request signal for channel 19 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 19 is enabled"]
    ENABLE = 1,
}
impl From<ERQ19_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ19_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ19_A {
        match self.bits {
            false => ERQ19_A::DISABLE,
            true => ERQ19_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ19_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ19_A::ENABLE
    }
}
#[doc = "Field `ERQ19` writer - Enable DMA Request 19"]
pub type ERQ19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ19_A, O>;
impl<'a, const O: u8> ERQ19_W<'a, O> {
    #[doc = "The DMA request signal for channel 19 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ19_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 19 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ19_A::ENABLE)
    }
}
#[doc = "Field `ERQ20` reader - Enable DMA Request 20"]
pub type ERQ20_R = crate::BitReader<ERQ20_A>;
#[doc = "Enable DMA Request 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ20_A {
    #[doc = "0: The DMA request signal for channel 20 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 20 is enabled"]
    ENABLE = 1,
}
impl From<ERQ20_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ20_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ20_A {
        match self.bits {
            false => ERQ20_A::DISABLE,
            true => ERQ20_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ20_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ20_A::ENABLE
    }
}
#[doc = "Field `ERQ20` writer - Enable DMA Request 20"]
pub type ERQ20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ20_A, O>;
impl<'a, const O: u8> ERQ20_W<'a, O> {
    #[doc = "The DMA request signal for channel 20 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ20_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 20 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ20_A::ENABLE)
    }
}
#[doc = "Field `ERQ21` reader - Enable DMA Request 21"]
pub type ERQ21_R = crate::BitReader<ERQ21_A>;
#[doc = "Enable DMA Request 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ21_A {
    #[doc = "0: The DMA request signal for channel 21 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 21 is enabled"]
    ENABLE = 1,
}
impl From<ERQ21_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ21_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ21_A {
        match self.bits {
            false => ERQ21_A::DISABLE,
            true => ERQ21_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ21_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ21_A::ENABLE
    }
}
#[doc = "Field `ERQ21` writer - Enable DMA Request 21"]
pub type ERQ21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ21_A, O>;
impl<'a, const O: u8> ERQ21_W<'a, O> {
    #[doc = "The DMA request signal for channel 21 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ21_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 21 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ21_A::ENABLE)
    }
}
#[doc = "Field `ERQ22` reader - Enable DMA Request 22"]
pub type ERQ22_R = crate::BitReader<ERQ22_A>;
#[doc = "Enable DMA Request 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ22_A {
    #[doc = "0: The DMA request signal for channel 22 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 22 is enabled"]
    ENABLE = 1,
}
impl From<ERQ22_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ22_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ22_A {
        match self.bits {
            false => ERQ22_A::DISABLE,
            true => ERQ22_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ22_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ22_A::ENABLE
    }
}
#[doc = "Field `ERQ22` writer - Enable DMA Request 22"]
pub type ERQ22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ22_A, O>;
impl<'a, const O: u8> ERQ22_W<'a, O> {
    #[doc = "The DMA request signal for channel 22 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ22_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 22 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ22_A::ENABLE)
    }
}
#[doc = "Field `ERQ23` reader - Enable DMA Request 23"]
pub type ERQ23_R = crate::BitReader<ERQ23_A>;
#[doc = "Enable DMA Request 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ23_A {
    #[doc = "0: The DMA request signal for channel 23 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 23 is enabled"]
    ENABLE = 1,
}
impl From<ERQ23_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ23_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ23_A {
        match self.bits {
            false => ERQ23_A::DISABLE,
            true => ERQ23_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ23_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ23_A::ENABLE
    }
}
#[doc = "Field `ERQ23` writer - Enable DMA Request 23"]
pub type ERQ23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ23_A, O>;
impl<'a, const O: u8> ERQ23_W<'a, O> {
    #[doc = "The DMA request signal for channel 23 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ23_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 23 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ23_A::ENABLE)
    }
}
#[doc = "Field `ERQ24` reader - Enable DMA Request 24"]
pub type ERQ24_R = crate::BitReader<ERQ24_A>;
#[doc = "Enable DMA Request 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ24_A {
    #[doc = "0: The DMA request signal for channel 24 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 24 is enabled"]
    ENABLE = 1,
}
impl From<ERQ24_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ24_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ24_A {
        match self.bits {
            false => ERQ24_A::DISABLE,
            true => ERQ24_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ24_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ24_A::ENABLE
    }
}
#[doc = "Field `ERQ24` writer - Enable DMA Request 24"]
pub type ERQ24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ24_A, O>;
impl<'a, const O: u8> ERQ24_W<'a, O> {
    #[doc = "The DMA request signal for channel 24 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ24_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 24 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ24_A::ENABLE)
    }
}
#[doc = "Field `ERQ25` reader - Enable DMA Request 25"]
pub type ERQ25_R = crate::BitReader<ERQ25_A>;
#[doc = "Enable DMA Request 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ25_A {
    #[doc = "0: The DMA request signal for channel 25 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 25 is enabled"]
    ENABLE = 1,
}
impl From<ERQ25_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ25_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ25_A {
        match self.bits {
            false => ERQ25_A::DISABLE,
            true => ERQ25_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ25_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ25_A::ENABLE
    }
}
#[doc = "Field `ERQ25` writer - Enable DMA Request 25"]
pub type ERQ25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ25_A, O>;
impl<'a, const O: u8> ERQ25_W<'a, O> {
    #[doc = "The DMA request signal for channel 25 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ25_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 25 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ25_A::ENABLE)
    }
}
#[doc = "Field `ERQ26` reader - Enable DMA Request 26"]
pub type ERQ26_R = crate::BitReader<ERQ26_A>;
#[doc = "Enable DMA Request 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ26_A {
    #[doc = "0: The DMA request signal for channel 26 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 26 is enabled"]
    ENABLE = 1,
}
impl From<ERQ26_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ26_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ26_A {
        match self.bits {
            false => ERQ26_A::DISABLE,
            true => ERQ26_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ26_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ26_A::ENABLE
    }
}
#[doc = "Field `ERQ26` writer - Enable DMA Request 26"]
pub type ERQ26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ26_A, O>;
impl<'a, const O: u8> ERQ26_W<'a, O> {
    #[doc = "The DMA request signal for channel 26 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ26_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 26 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ26_A::ENABLE)
    }
}
#[doc = "Field `ERQ27` reader - Enable DMA Request 27"]
pub type ERQ27_R = crate::BitReader<ERQ27_A>;
#[doc = "Enable DMA Request 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ27_A {
    #[doc = "0: The DMA request signal for channel 27 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 27 is enabled"]
    ENABLE = 1,
}
impl From<ERQ27_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ27_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ27_A {
        match self.bits {
            false => ERQ27_A::DISABLE,
            true => ERQ27_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ27_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ27_A::ENABLE
    }
}
#[doc = "Field `ERQ27` writer - Enable DMA Request 27"]
pub type ERQ27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ27_A, O>;
impl<'a, const O: u8> ERQ27_W<'a, O> {
    #[doc = "The DMA request signal for channel 27 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ27_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 27 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ27_A::ENABLE)
    }
}
#[doc = "Field `ERQ28` reader - Enable DMA Request 28"]
pub type ERQ28_R = crate::BitReader<ERQ28_A>;
#[doc = "Enable DMA Request 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ28_A {
    #[doc = "0: The DMA request signal for channel 28 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 28 is enabled"]
    ENABLE = 1,
}
impl From<ERQ28_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ28_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ28_A {
        match self.bits {
            false => ERQ28_A::DISABLE,
            true => ERQ28_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ28_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ28_A::ENABLE
    }
}
#[doc = "Field `ERQ28` writer - Enable DMA Request 28"]
pub type ERQ28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ28_A, O>;
impl<'a, const O: u8> ERQ28_W<'a, O> {
    #[doc = "The DMA request signal for channel 28 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ28_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 28 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ28_A::ENABLE)
    }
}
#[doc = "Field `ERQ29` reader - Enable DMA Request 29"]
pub type ERQ29_R = crate::BitReader<ERQ29_A>;
#[doc = "Enable DMA Request 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ29_A {
    #[doc = "0: The DMA request signal for channel 29 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 29 is enabled"]
    ENABLE = 1,
}
impl From<ERQ29_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ29_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ29_A {
        match self.bits {
            false => ERQ29_A::DISABLE,
            true => ERQ29_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ29_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ29_A::ENABLE
    }
}
#[doc = "Field `ERQ29` writer - Enable DMA Request 29"]
pub type ERQ29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ29_A, O>;
impl<'a, const O: u8> ERQ29_W<'a, O> {
    #[doc = "The DMA request signal for channel 29 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ29_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 29 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ29_A::ENABLE)
    }
}
#[doc = "Field `ERQ30` reader - Enable DMA Request 30"]
pub type ERQ30_R = crate::BitReader<ERQ30_A>;
#[doc = "Enable DMA Request 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ30_A {
    #[doc = "0: The DMA request signal for channel 30 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 30 is enabled"]
    ENABLE = 1,
}
impl From<ERQ30_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ30_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ30_A {
        match self.bits {
            false => ERQ30_A::DISABLE,
            true => ERQ30_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ30_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ30_A::ENABLE
    }
}
#[doc = "Field `ERQ30` writer - Enable DMA Request 30"]
pub type ERQ30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ30_A, O>;
impl<'a, const O: u8> ERQ30_W<'a, O> {
    #[doc = "The DMA request signal for channel 30 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ30_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 30 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ30_A::ENABLE)
    }
}
#[doc = "Field `ERQ31` reader - Enable DMA Request 31"]
pub type ERQ31_R = crate::BitReader<ERQ31_A>;
#[doc = "Enable DMA Request 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ31_A {
    #[doc = "0: The DMA request signal for channel 31 is disabled"]
    DISABLE = 0,
    #[doc = "1: The DMA request signal for channel 31 is enabled"]
    ENABLE = 1,
}
impl From<ERQ31_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ31_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ31_A {
        match self.bits {
            false => ERQ31_A::DISABLE,
            true => ERQ31_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERQ31_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERQ31_A::ENABLE
    }
}
#[doc = "Field `ERQ31` writer - Enable DMA Request 31"]
pub type ERQ31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ31_A, O>;
impl<'a, const O: u8> ERQ31_W<'a, O> {
    #[doc = "The DMA request signal for channel 31 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERQ31_A::DISABLE)
    }
    #[doc = "The DMA request signal for channel 31 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERQ31_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&self) -> ERQ4_R {
        ERQ4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&self) -> ERQ5_R {
        ERQ5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&self) -> ERQ6_R {
        ERQ6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&self) -> ERQ7_R {
        ERQ7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&self) -> ERQ8_R {
        ERQ8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&self) -> ERQ9_R {
        ERQ9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&self) -> ERQ10_R {
        ERQ10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&self) -> ERQ11_R {
        ERQ11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&self) -> ERQ12_R {
        ERQ12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&self) -> ERQ13_R {
        ERQ13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&self) -> ERQ14_R {
        ERQ14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&self) -> ERQ15_R {
        ERQ15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline(always)]
    pub fn erq16(&self) -> ERQ16_R {
        ERQ16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline(always)]
    pub fn erq17(&self) -> ERQ17_R {
        ERQ17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline(always)]
    pub fn erq18(&self) -> ERQ18_R {
        ERQ18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline(always)]
    pub fn erq19(&self) -> ERQ19_R {
        ERQ19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline(always)]
    pub fn erq20(&self) -> ERQ20_R {
        ERQ20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline(always)]
    pub fn erq21(&self) -> ERQ21_R {
        ERQ21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline(always)]
    pub fn erq22(&self) -> ERQ22_R {
        ERQ22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline(always)]
    pub fn erq23(&self) -> ERQ23_R {
        ERQ23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline(always)]
    pub fn erq24(&self) -> ERQ24_R {
        ERQ24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline(always)]
    pub fn erq25(&self) -> ERQ25_R {
        ERQ25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline(always)]
    pub fn erq26(&self) -> ERQ26_R {
        ERQ26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline(always)]
    pub fn erq27(&self) -> ERQ27_R {
        ERQ27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline(always)]
    pub fn erq28(&self) -> ERQ28_R {
        ERQ28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline(always)]
    pub fn erq29(&self) -> ERQ29_R {
        ERQ29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline(always)]
    pub fn erq30(&self) -> ERQ30_R {
        ERQ30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline(always)]
    pub fn erq31(&self) -> ERQ31_R {
        ERQ31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn erq0(&mut self) -> ERQ0_W<0> {
        ERQ0_W::new(self)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn erq1(&mut self) -> ERQ1_W<1> {
        ERQ1_W::new(self)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn erq2(&mut self) -> ERQ2_W<2> {
        ERQ2_W::new(self)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn erq3(&mut self) -> ERQ3_W<3> {
        ERQ3_W::new(self)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn erq4(&mut self) -> ERQ4_W<4> {
        ERQ4_W::new(self)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn erq5(&mut self) -> ERQ5_W<5> {
        ERQ5_W::new(self)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn erq6(&mut self) -> ERQ6_W<6> {
        ERQ6_W::new(self)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn erq7(&mut self) -> ERQ7_W<7> {
        ERQ7_W::new(self)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn erq8(&mut self) -> ERQ8_W<8> {
        ERQ8_W::new(self)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn erq9(&mut self) -> ERQ9_W<9> {
        ERQ9_W::new(self)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn erq10(&mut self) -> ERQ10_W<10> {
        ERQ10_W::new(self)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn erq11(&mut self) -> ERQ11_W<11> {
        ERQ11_W::new(self)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn erq12(&mut self) -> ERQ12_W<12> {
        ERQ12_W::new(self)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn erq13(&mut self) -> ERQ13_W<13> {
        ERQ13_W::new(self)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn erq14(&mut self) -> ERQ14_W<14> {
        ERQ14_W::new(self)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn erq15(&mut self) -> ERQ15_W<15> {
        ERQ15_W::new(self)
    }
    #[doc = "Bit 16 - Enable DMA Request 16"]
    #[inline(always)]
    #[must_use]
    pub fn erq16(&mut self) -> ERQ16_W<16> {
        ERQ16_W::new(self)
    }
    #[doc = "Bit 17 - Enable DMA Request 17"]
    #[inline(always)]
    #[must_use]
    pub fn erq17(&mut self) -> ERQ17_W<17> {
        ERQ17_W::new(self)
    }
    #[doc = "Bit 18 - Enable DMA Request 18"]
    #[inline(always)]
    #[must_use]
    pub fn erq18(&mut self) -> ERQ18_W<18> {
        ERQ18_W::new(self)
    }
    #[doc = "Bit 19 - Enable DMA Request 19"]
    #[inline(always)]
    #[must_use]
    pub fn erq19(&mut self) -> ERQ19_W<19> {
        ERQ19_W::new(self)
    }
    #[doc = "Bit 20 - Enable DMA Request 20"]
    #[inline(always)]
    #[must_use]
    pub fn erq20(&mut self) -> ERQ20_W<20> {
        ERQ20_W::new(self)
    }
    #[doc = "Bit 21 - Enable DMA Request 21"]
    #[inline(always)]
    #[must_use]
    pub fn erq21(&mut self) -> ERQ21_W<21> {
        ERQ21_W::new(self)
    }
    #[doc = "Bit 22 - Enable DMA Request 22"]
    #[inline(always)]
    #[must_use]
    pub fn erq22(&mut self) -> ERQ22_W<22> {
        ERQ22_W::new(self)
    }
    #[doc = "Bit 23 - Enable DMA Request 23"]
    #[inline(always)]
    #[must_use]
    pub fn erq23(&mut self) -> ERQ23_W<23> {
        ERQ23_W::new(self)
    }
    #[doc = "Bit 24 - Enable DMA Request 24"]
    #[inline(always)]
    #[must_use]
    pub fn erq24(&mut self) -> ERQ24_W<24> {
        ERQ24_W::new(self)
    }
    #[doc = "Bit 25 - Enable DMA Request 25"]
    #[inline(always)]
    #[must_use]
    pub fn erq25(&mut self) -> ERQ25_W<25> {
        ERQ25_W::new(self)
    }
    #[doc = "Bit 26 - Enable DMA Request 26"]
    #[inline(always)]
    #[must_use]
    pub fn erq26(&mut self) -> ERQ26_W<26> {
        ERQ26_W::new(self)
    }
    #[doc = "Bit 27 - Enable DMA Request 27"]
    #[inline(always)]
    #[must_use]
    pub fn erq27(&mut self) -> ERQ27_W<27> {
        ERQ27_W::new(self)
    }
    #[doc = "Bit 28 - Enable DMA Request 28"]
    #[inline(always)]
    #[must_use]
    pub fn erq28(&mut self) -> ERQ28_W<28> {
        ERQ28_W::new(self)
    }
    #[doc = "Bit 29 - Enable DMA Request 29"]
    #[inline(always)]
    #[must_use]
    pub fn erq29(&mut self) -> ERQ29_W<29> {
        ERQ29_W::new(self)
    }
    #[doc = "Bit 30 - Enable DMA Request 30"]
    #[inline(always)]
    #[must_use]
    pub fn erq30(&mut self) -> ERQ30_W<30> {
        ERQ30_W::new(self)
    }
    #[doc = "Bit 31 - Enable DMA Request 31"]
    #[inline(always)]
    #[must_use]
    pub fn erq31(&mut self) -> ERQ31_W<31> {
        ERQ31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](index.html) module"]
pub struct ERQ_SPEC;
impl crate::RegisterSpec for ERQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erq::R](R) reader structure"]
impl crate::Readable for ERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erq::W](W) writer structure"]
impl crate::Writable for ERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERQ to value 0"]
impl crate::Resettable for ERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
