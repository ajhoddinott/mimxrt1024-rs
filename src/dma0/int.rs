#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0` reader - Interrupt Request 0"]
pub type INT0_R = crate::BitReader<INT0_A>;
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT0_A {
    #[doc = "0: The interrupt request for channel 0 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 0 is active"]
    ACTIVE = 1,
}
impl From<INT0_A> for bool {
    #[inline(always)]
    fn from(variant: INT0_A) -> Self {
        variant as u8 != 0
    }
}
impl INT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT0_A {
        match self.bits {
            false => INT0_A::NOT_ACTIVE,
            true => INT0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT0_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT0_A::ACTIVE
    }
}
#[doc = "Field `INT0` writer - Interrupt Request 0"]
pub type INT0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT0_A, O>;
impl<'a, const O: u8> INT0_W<'a, O> {
    #[doc = "The interrupt request for channel 0 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT0_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 0 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT0_A::ACTIVE)
    }
}
#[doc = "Field `INT1` reader - Interrupt Request 1"]
pub type INT1_R = crate::BitReader<INT1_A>;
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT1_A {
    #[doc = "0: The interrupt request for channel 1 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 1 is active"]
    ACTIVE = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
impl INT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::NOT_ACTIVE,
            true => INT1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT1_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT1_A::ACTIVE
    }
}
#[doc = "Field `INT1` writer - Interrupt Request 1"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT1_A, O>;
impl<'a, const O: u8> INT1_W<'a, O> {
    #[doc = "The interrupt request for channel 1 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT1_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 1 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT1_A::ACTIVE)
    }
}
#[doc = "Field `INT2` reader - Interrupt Request 2"]
pub type INT2_R = crate::BitReader<INT2_A>;
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT2_A {
    #[doc = "0: The interrupt request for channel 2 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 2 is active"]
    ACTIVE = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
impl INT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::NOT_ACTIVE,
            true => INT2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT2_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT2_A::ACTIVE
    }
}
#[doc = "Field `INT2` writer - Interrupt Request 2"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT2_A, O>;
impl<'a, const O: u8> INT2_W<'a, O> {
    #[doc = "The interrupt request for channel 2 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT2_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 2 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT2_A::ACTIVE)
    }
}
#[doc = "Field `INT3` reader - Interrupt Request 3"]
pub type INT3_R = crate::BitReader<INT3_A>;
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3_A {
    #[doc = "0: The interrupt request for channel 3 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 3 is active"]
    ACTIVE = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
impl INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::NOT_ACTIVE,
            true => INT3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT3_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT3_A::ACTIVE
    }
}
#[doc = "Field `INT3` writer - Interrupt Request 3"]
pub type INT3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT3_A, O>;
impl<'a, const O: u8> INT3_W<'a, O> {
    #[doc = "The interrupt request for channel 3 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT3_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 3 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT3_A::ACTIVE)
    }
}
#[doc = "Field `INT4` reader - Interrupt Request 4"]
pub type INT4_R = crate::BitReader<INT4_A>;
#[doc = "Interrupt Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT4_A {
    #[doc = "0: The interrupt request for channel 4 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 4 is active"]
    ACTIVE = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
impl INT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::NOT_ACTIVE,
            true => INT4_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT4_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT4_A::ACTIVE
    }
}
#[doc = "Field `INT4` writer - Interrupt Request 4"]
pub type INT4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT4_A, O>;
impl<'a, const O: u8> INT4_W<'a, O> {
    #[doc = "The interrupt request for channel 4 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT4_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 4 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT4_A::ACTIVE)
    }
}
#[doc = "Field `INT5` reader - Interrupt Request 5"]
pub type INT5_R = crate::BitReader<INT5_A>;
#[doc = "Interrupt Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT5_A {
    #[doc = "0: The interrupt request for channel 5 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 5 is active"]
    ACTIVE = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
impl INT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::NOT_ACTIVE,
            true => INT5_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT5_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT5_A::ACTIVE
    }
}
#[doc = "Field `INT5` writer - Interrupt Request 5"]
pub type INT5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT5_A, O>;
impl<'a, const O: u8> INT5_W<'a, O> {
    #[doc = "The interrupt request for channel 5 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT5_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 5 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT5_A::ACTIVE)
    }
}
#[doc = "Field `INT6` reader - Interrupt Request 6"]
pub type INT6_R = crate::BitReader<INT6_A>;
#[doc = "Interrupt Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT6_A {
    #[doc = "0: The interrupt request for channel 6 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 6 is active"]
    CTIVE = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
impl INT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::NOT_ACTIVE,
            true => INT6_A::CTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT6_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CTIVE`"]
    #[inline(always)]
    pub fn is_ctive(&self) -> bool {
        *self == INT6_A::CTIVE
    }
}
#[doc = "Field `INT6` writer - Interrupt Request 6"]
pub type INT6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT6_A, O>;
impl<'a, const O: u8> INT6_W<'a, O> {
    #[doc = "The interrupt request for channel 6 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT6_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 6 is active"]
    #[inline(always)]
    pub fn ctive(self) -> &'a mut W {
        self.variant(INT6_A::CTIVE)
    }
}
#[doc = "Field `INT7` reader - Interrupt Request 7"]
pub type INT7_R = crate::BitReader<INT7_A>;
#[doc = "Interrupt Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT7_A {
    #[doc = "0: The interrupt request for channel 7 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 7 is active"]
    ACTIVE = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
impl INT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::NOT_ACTIVE,
            true => INT7_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT7_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT7_A::ACTIVE
    }
}
#[doc = "Field `INT7` writer - Interrupt Request 7"]
pub type INT7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT7_A, O>;
impl<'a, const O: u8> INT7_W<'a, O> {
    #[doc = "The interrupt request for channel 7 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT7_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 7 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT7_A::ACTIVE)
    }
}
#[doc = "Field `INT8` reader - Interrupt Request 8"]
pub type INT8_R = crate::BitReader<INT8_A>;
#[doc = "Interrupt Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT8_A {
    #[doc = "0: The interrupt request for channel 8 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 8 is active"]
    ACTIVE = 1,
}
impl From<INT8_A> for bool {
    #[inline(always)]
    fn from(variant: INT8_A) -> Self {
        variant as u8 != 0
    }
}
impl INT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT8_A {
        match self.bits {
            false => INT8_A::NOT_ACTIVE,
            true => INT8_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT8_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT8_A::ACTIVE
    }
}
#[doc = "Field `INT8` writer - Interrupt Request 8"]
pub type INT8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT8_A, O>;
impl<'a, const O: u8> INT8_W<'a, O> {
    #[doc = "The interrupt request for channel 8 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT8_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 8 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT8_A::ACTIVE)
    }
}
#[doc = "Field `INT9` reader - Interrupt Request 9"]
pub type INT9_R = crate::BitReader<INT9_A>;
#[doc = "Interrupt Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT9_A {
    #[doc = "0: The interrupt request for channel 9 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 9 is active"]
    ACTIVE = 1,
}
impl From<INT9_A> for bool {
    #[inline(always)]
    fn from(variant: INT9_A) -> Self {
        variant as u8 != 0
    }
}
impl INT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT9_A {
        match self.bits {
            false => INT9_A::NOT_ACTIVE,
            true => INT9_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT9_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT9_A::ACTIVE
    }
}
#[doc = "Field `INT9` writer - Interrupt Request 9"]
pub type INT9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT9_A, O>;
impl<'a, const O: u8> INT9_W<'a, O> {
    #[doc = "The interrupt request for channel 9 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT9_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 9 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT9_A::ACTIVE)
    }
}
#[doc = "Field `INT10` reader - Interrupt Request 10"]
pub type INT10_R = crate::BitReader<INT10_A>;
#[doc = "Interrupt Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT10_A {
    #[doc = "0: The interrupt request for channel 10 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 10 is active"]
    ACTIVE = 1,
}
impl From<INT10_A> for bool {
    #[inline(always)]
    fn from(variant: INT10_A) -> Self {
        variant as u8 != 0
    }
}
impl INT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT10_A {
        match self.bits {
            false => INT10_A::NOT_ACTIVE,
            true => INT10_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT10_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT10_A::ACTIVE
    }
}
#[doc = "Field `INT10` writer - Interrupt Request 10"]
pub type INT10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT10_A, O>;
impl<'a, const O: u8> INT10_W<'a, O> {
    #[doc = "The interrupt request for channel 10 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT10_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 10 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT10_A::ACTIVE)
    }
}
#[doc = "Field `INT11` reader - Interrupt Request 11"]
pub type INT11_R = crate::BitReader<INT11_A>;
#[doc = "Interrupt Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT11_A {
    #[doc = "0: The interrupt request for channel 11 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 11 is active"]
    ACTIVE = 1,
}
impl From<INT11_A> for bool {
    #[inline(always)]
    fn from(variant: INT11_A) -> Self {
        variant as u8 != 0
    }
}
impl INT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT11_A {
        match self.bits {
            false => INT11_A::NOT_ACTIVE,
            true => INT11_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT11_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT11_A::ACTIVE
    }
}
#[doc = "Field `INT11` writer - Interrupt Request 11"]
pub type INT11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT11_A, O>;
impl<'a, const O: u8> INT11_W<'a, O> {
    #[doc = "The interrupt request for channel 11 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT11_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 11 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT11_A::ACTIVE)
    }
}
#[doc = "Field `INT12` reader - Interrupt Request 12"]
pub type INT12_R = crate::BitReader<INT12_A>;
#[doc = "Interrupt Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT12_A {
    #[doc = "0: The interrupt request for channel 12 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 12 is active"]
    ACTIVE = 1,
}
impl From<INT12_A> for bool {
    #[inline(always)]
    fn from(variant: INT12_A) -> Self {
        variant as u8 != 0
    }
}
impl INT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT12_A {
        match self.bits {
            false => INT12_A::NOT_ACTIVE,
            true => INT12_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT12_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT12_A::ACTIVE
    }
}
#[doc = "Field `INT12` writer - Interrupt Request 12"]
pub type INT12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT12_A, O>;
impl<'a, const O: u8> INT12_W<'a, O> {
    #[doc = "The interrupt request for channel 12 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT12_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 12 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT12_A::ACTIVE)
    }
}
#[doc = "Field `INT13` reader - Interrupt Request 13"]
pub type INT13_R = crate::BitReader<INT13_A>;
#[doc = "Interrupt Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT13_A {
    #[doc = "0: The interrupt request for channel 13 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 13 is active"]
    ACTIVE = 1,
}
impl From<INT13_A> for bool {
    #[inline(always)]
    fn from(variant: INT13_A) -> Self {
        variant as u8 != 0
    }
}
impl INT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT13_A {
        match self.bits {
            false => INT13_A::NOT_ACTIVE,
            true => INT13_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT13_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT13_A::ACTIVE
    }
}
#[doc = "Field `INT13` writer - Interrupt Request 13"]
pub type INT13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT13_A, O>;
impl<'a, const O: u8> INT13_W<'a, O> {
    #[doc = "The interrupt request for channel 13 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT13_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 13 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT13_A::ACTIVE)
    }
}
#[doc = "Field `INT14` reader - Interrupt Request 14"]
pub type INT14_R = crate::BitReader<INT14_A>;
#[doc = "Interrupt Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT14_A {
    #[doc = "0: The interrupt request for channel 14 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 14 is active"]
    ACTIVE = 1,
}
impl From<INT14_A> for bool {
    #[inline(always)]
    fn from(variant: INT14_A) -> Self {
        variant as u8 != 0
    }
}
impl INT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT14_A {
        match self.bits {
            false => INT14_A::NOT_ACTIVE,
            true => INT14_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT14_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT14_A::ACTIVE
    }
}
#[doc = "Field `INT14` writer - Interrupt Request 14"]
pub type INT14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT14_A, O>;
impl<'a, const O: u8> INT14_W<'a, O> {
    #[doc = "The interrupt request for channel 14 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT14_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 14 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT14_A::ACTIVE)
    }
}
#[doc = "Field `INT15` reader - Interrupt Request 15"]
pub type INT15_R = crate::BitReader<INT15_A>;
#[doc = "Interrupt Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT15_A {
    #[doc = "0: The interrupt request for channel 15 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 15 is active"]
    ACTIVE = 1,
}
impl From<INT15_A> for bool {
    #[inline(always)]
    fn from(variant: INT15_A) -> Self {
        variant as u8 != 0
    }
}
impl INT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT15_A {
        match self.bits {
            false => INT15_A::NOT_ACTIVE,
            true => INT15_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT15_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT15_A::ACTIVE
    }
}
#[doc = "Field `INT15` writer - Interrupt Request 15"]
pub type INT15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT15_A, O>;
impl<'a, const O: u8> INT15_W<'a, O> {
    #[doc = "The interrupt request for channel 15 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT15_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 15 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT15_A::ACTIVE)
    }
}
#[doc = "Field `INT16` reader - Interrupt Request 16"]
pub type INT16_R = crate::BitReader<INT16_A>;
#[doc = "Interrupt Request 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT16_A {
    #[doc = "0: The interrupt request for channel 16 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 16 is active"]
    ACTIVE = 1,
}
impl From<INT16_A> for bool {
    #[inline(always)]
    fn from(variant: INT16_A) -> Self {
        variant as u8 != 0
    }
}
impl INT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT16_A {
        match self.bits {
            false => INT16_A::NOT_ACTIVE,
            true => INT16_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT16_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT16_A::ACTIVE
    }
}
#[doc = "Field `INT16` writer - Interrupt Request 16"]
pub type INT16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT16_A, O>;
impl<'a, const O: u8> INT16_W<'a, O> {
    #[doc = "The interrupt request for channel 16 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT16_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 16 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT16_A::ACTIVE)
    }
}
#[doc = "Field `INT17` reader - Interrupt Request 17"]
pub type INT17_R = crate::BitReader<INT17_A>;
#[doc = "Interrupt Request 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT17_A {
    #[doc = "0: The interrupt request for channel 17 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 17 is active"]
    ACTIVE = 1,
}
impl From<INT17_A> for bool {
    #[inline(always)]
    fn from(variant: INT17_A) -> Self {
        variant as u8 != 0
    }
}
impl INT17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT17_A {
        match self.bits {
            false => INT17_A::NOT_ACTIVE,
            true => INT17_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT17_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT17_A::ACTIVE
    }
}
#[doc = "Field `INT17` writer - Interrupt Request 17"]
pub type INT17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT17_A, O>;
impl<'a, const O: u8> INT17_W<'a, O> {
    #[doc = "The interrupt request for channel 17 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT17_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 17 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT17_A::ACTIVE)
    }
}
#[doc = "Field `INT18` reader - Interrupt Request 18"]
pub type INT18_R = crate::BitReader<INT18_A>;
#[doc = "Interrupt Request 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT18_A {
    #[doc = "0: The interrupt request for channel 18 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 18 is active"]
    ACTIVE = 1,
}
impl From<INT18_A> for bool {
    #[inline(always)]
    fn from(variant: INT18_A) -> Self {
        variant as u8 != 0
    }
}
impl INT18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT18_A {
        match self.bits {
            false => INT18_A::NOT_ACTIVE,
            true => INT18_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT18_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT18_A::ACTIVE
    }
}
#[doc = "Field `INT18` writer - Interrupt Request 18"]
pub type INT18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT18_A, O>;
impl<'a, const O: u8> INT18_W<'a, O> {
    #[doc = "The interrupt request for channel 18 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT18_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 18 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT18_A::ACTIVE)
    }
}
#[doc = "Field `INT19` reader - Interrupt Request 19"]
pub type INT19_R = crate::BitReader<INT19_A>;
#[doc = "Interrupt Request 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT19_A {
    #[doc = "0: The interrupt request for channel 19 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 19 is active"]
    ACTIVE = 1,
}
impl From<INT19_A> for bool {
    #[inline(always)]
    fn from(variant: INT19_A) -> Self {
        variant as u8 != 0
    }
}
impl INT19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT19_A {
        match self.bits {
            false => INT19_A::NOT_ACTIVE,
            true => INT19_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT19_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT19_A::ACTIVE
    }
}
#[doc = "Field `INT19` writer - Interrupt Request 19"]
pub type INT19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT19_A, O>;
impl<'a, const O: u8> INT19_W<'a, O> {
    #[doc = "The interrupt request for channel 19 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT19_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 19 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT19_A::ACTIVE)
    }
}
#[doc = "Field `INT20` reader - Interrupt Request 20"]
pub type INT20_R = crate::BitReader<INT20_A>;
#[doc = "Interrupt Request 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT20_A {
    #[doc = "0: The interrupt request for channel 20 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 20 is active"]
    ACTIVE = 1,
}
impl From<INT20_A> for bool {
    #[inline(always)]
    fn from(variant: INT20_A) -> Self {
        variant as u8 != 0
    }
}
impl INT20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT20_A {
        match self.bits {
            false => INT20_A::NOT_ACTIVE,
            true => INT20_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT20_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT20_A::ACTIVE
    }
}
#[doc = "Field `INT20` writer - Interrupt Request 20"]
pub type INT20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT20_A, O>;
impl<'a, const O: u8> INT20_W<'a, O> {
    #[doc = "The interrupt request for channel 20 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT20_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 20 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT20_A::ACTIVE)
    }
}
#[doc = "Field `INT21` reader - Interrupt Request 21"]
pub type INT21_R = crate::BitReader<INT21_A>;
#[doc = "Interrupt Request 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT21_A {
    #[doc = "0: The interrupt request for channel 21 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 21 is active"]
    ACTIVE = 1,
}
impl From<INT21_A> for bool {
    #[inline(always)]
    fn from(variant: INT21_A) -> Self {
        variant as u8 != 0
    }
}
impl INT21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT21_A {
        match self.bits {
            false => INT21_A::NOT_ACTIVE,
            true => INT21_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT21_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT21_A::ACTIVE
    }
}
#[doc = "Field `INT21` writer - Interrupt Request 21"]
pub type INT21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT21_A, O>;
impl<'a, const O: u8> INT21_W<'a, O> {
    #[doc = "The interrupt request for channel 21 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT21_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 21 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT21_A::ACTIVE)
    }
}
#[doc = "Field `INT22` reader - Interrupt Request 22"]
pub type INT22_R = crate::BitReader<INT22_A>;
#[doc = "Interrupt Request 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT22_A {
    #[doc = "0: The interrupt request for channel 22 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 22 is active"]
    ACTIVE = 1,
}
impl From<INT22_A> for bool {
    #[inline(always)]
    fn from(variant: INT22_A) -> Self {
        variant as u8 != 0
    }
}
impl INT22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT22_A {
        match self.bits {
            false => INT22_A::NOT_ACTIVE,
            true => INT22_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT22_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT22_A::ACTIVE
    }
}
#[doc = "Field `INT22` writer - Interrupt Request 22"]
pub type INT22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT22_A, O>;
impl<'a, const O: u8> INT22_W<'a, O> {
    #[doc = "The interrupt request for channel 22 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT22_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 22 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT22_A::ACTIVE)
    }
}
#[doc = "Field `INT23` reader - Interrupt Request 23"]
pub type INT23_R = crate::BitReader<INT23_A>;
#[doc = "Interrupt Request 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT23_A {
    #[doc = "0: The interrupt request for channel 23 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 23 is active"]
    ACTIVE = 1,
}
impl From<INT23_A> for bool {
    #[inline(always)]
    fn from(variant: INT23_A) -> Self {
        variant as u8 != 0
    }
}
impl INT23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT23_A {
        match self.bits {
            false => INT23_A::NOT_ACTIVE,
            true => INT23_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT23_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT23_A::ACTIVE
    }
}
#[doc = "Field `INT23` writer - Interrupt Request 23"]
pub type INT23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT23_A, O>;
impl<'a, const O: u8> INT23_W<'a, O> {
    #[doc = "The interrupt request for channel 23 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT23_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 23 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT23_A::ACTIVE)
    }
}
#[doc = "Field `INT24` reader - Interrupt Request 24"]
pub type INT24_R = crate::BitReader<INT24_A>;
#[doc = "Interrupt Request 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT24_A {
    #[doc = "0: The interrupt request for channel 24 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 24 is active"]
    ACTIVE = 1,
}
impl From<INT24_A> for bool {
    #[inline(always)]
    fn from(variant: INT24_A) -> Self {
        variant as u8 != 0
    }
}
impl INT24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT24_A {
        match self.bits {
            false => INT24_A::NOT_ACTIVE,
            true => INT24_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT24_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT24_A::ACTIVE
    }
}
#[doc = "Field `INT24` writer - Interrupt Request 24"]
pub type INT24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT24_A, O>;
impl<'a, const O: u8> INT24_W<'a, O> {
    #[doc = "The interrupt request for channel 24 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT24_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 24 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT24_A::ACTIVE)
    }
}
#[doc = "Field `INT25` reader - Interrupt Request 25"]
pub type INT25_R = crate::BitReader<INT25_A>;
#[doc = "Interrupt Request 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT25_A {
    #[doc = "0: The interrupt request for channel 25 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 25 is active"]
    ACTIVE = 1,
}
impl From<INT25_A> for bool {
    #[inline(always)]
    fn from(variant: INT25_A) -> Self {
        variant as u8 != 0
    }
}
impl INT25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT25_A {
        match self.bits {
            false => INT25_A::NOT_ACTIVE,
            true => INT25_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT25_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT25_A::ACTIVE
    }
}
#[doc = "Field `INT25` writer - Interrupt Request 25"]
pub type INT25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT25_A, O>;
impl<'a, const O: u8> INT25_W<'a, O> {
    #[doc = "The interrupt request for channel 25 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT25_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 25 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT25_A::ACTIVE)
    }
}
#[doc = "Field `INT26` reader - Interrupt Request 26"]
pub type INT26_R = crate::BitReader<INT26_A>;
#[doc = "Interrupt Request 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT26_A {
    #[doc = "0: The interrupt request for channel 26 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 26 is active"]
    ACTIVE = 1,
}
impl From<INT26_A> for bool {
    #[inline(always)]
    fn from(variant: INT26_A) -> Self {
        variant as u8 != 0
    }
}
impl INT26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT26_A {
        match self.bits {
            false => INT26_A::NOT_ACTIVE,
            true => INT26_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT26_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT26_A::ACTIVE
    }
}
#[doc = "Field `INT26` writer - Interrupt Request 26"]
pub type INT26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT26_A, O>;
impl<'a, const O: u8> INT26_W<'a, O> {
    #[doc = "The interrupt request for channel 26 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT26_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 26 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT26_A::ACTIVE)
    }
}
#[doc = "Field `INT27` reader - Interrupt Request 27"]
pub type INT27_R = crate::BitReader<INT27_A>;
#[doc = "Interrupt Request 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT27_A {
    #[doc = "0: The interrupt request for channel 27 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 27 is active"]
    ACTIVE = 1,
}
impl From<INT27_A> for bool {
    #[inline(always)]
    fn from(variant: INT27_A) -> Self {
        variant as u8 != 0
    }
}
impl INT27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT27_A {
        match self.bits {
            false => INT27_A::NOT_ACTIVE,
            true => INT27_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT27_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT27_A::ACTIVE
    }
}
#[doc = "Field `INT27` writer - Interrupt Request 27"]
pub type INT27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT27_A, O>;
impl<'a, const O: u8> INT27_W<'a, O> {
    #[doc = "The interrupt request for channel 27 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT27_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 27 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT27_A::ACTIVE)
    }
}
#[doc = "Field `INT28` reader - Interrupt Request 28"]
pub type INT28_R = crate::BitReader<INT28_A>;
#[doc = "Interrupt Request 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT28_A {
    #[doc = "0: The interrupt request for channel 28 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 28 is active"]
    ACTIVE = 1,
}
impl From<INT28_A> for bool {
    #[inline(always)]
    fn from(variant: INT28_A) -> Self {
        variant as u8 != 0
    }
}
impl INT28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT28_A {
        match self.bits {
            false => INT28_A::NOT_ACTIVE,
            true => INT28_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT28_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT28_A::ACTIVE
    }
}
#[doc = "Field `INT28` writer - Interrupt Request 28"]
pub type INT28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT28_A, O>;
impl<'a, const O: u8> INT28_W<'a, O> {
    #[doc = "The interrupt request for channel 28 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT28_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 28 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT28_A::ACTIVE)
    }
}
#[doc = "Field `INT29` reader - Interrupt Request 29"]
pub type INT29_R = crate::BitReader<INT29_A>;
#[doc = "Interrupt Request 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT29_A {
    #[doc = "0: The interrupt request for channel 29 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 29 is active"]
    ACTIVE = 1,
}
impl From<INT29_A> for bool {
    #[inline(always)]
    fn from(variant: INT29_A) -> Self {
        variant as u8 != 0
    }
}
impl INT29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT29_A {
        match self.bits {
            false => INT29_A::NOT_ACTIVE,
            true => INT29_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT29_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT29_A::ACTIVE
    }
}
#[doc = "Field `INT29` writer - Interrupt Request 29"]
pub type INT29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT29_A, O>;
impl<'a, const O: u8> INT29_W<'a, O> {
    #[doc = "The interrupt request for channel 29 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT29_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 29 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT29_A::ACTIVE)
    }
}
#[doc = "Field `INT30` reader - Interrupt Request 30"]
pub type INT30_R = crate::BitReader<INT30_A>;
#[doc = "Interrupt Request 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT30_A {
    #[doc = "0: The interrupt request for channel 30 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 30 is active"]
    ACTIVE = 1,
}
impl From<INT30_A> for bool {
    #[inline(always)]
    fn from(variant: INT30_A) -> Self {
        variant as u8 != 0
    }
}
impl INT30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT30_A {
        match self.bits {
            false => INT30_A::NOT_ACTIVE,
            true => INT30_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT30_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT30_A::ACTIVE
    }
}
#[doc = "Field `INT30` writer - Interrupt Request 30"]
pub type INT30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT30_A, O>;
impl<'a, const O: u8> INT30_W<'a, O> {
    #[doc = "The interrupt request for channel 30 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT30_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 30 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT30_A::ACTIVE)
    }
}
#[doc = "Field `INT31` reader - Interrupt Request 31"]
pub type INT31_R = crate::BitReader<INT31_A>;
#[doc = "Interrupt Request 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT31_A {
    #[doc = "0: The interrupt request for channel 31 is cleared"]
    NOT_ACTIVE = 0,
    #[doc = "1: The interrupt request for channel 31 is active"]
    ACTIVE = 1,
}
impl From<INT31_A> for bool {
    #[inline(always)]
    fn from(variant: INT31_A) -> Self {
        variant as u8 != 0
    }
}
impl INT31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT31_A {
        match self.bits {
            false => INT31_A::NOT_ACTIVE,
            true => INT31_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == INT31_A::NOT_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == INT31_A::ACTIVE
    }
}
#[doc = "Field `INT31` writer - Interrupt Request 31"]
pub type INT31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_SPEC, INT31_A, O>;
impl<'a, const O: u8> INT31_W<'a, O> {
    #[doc = "The interrupt request for channel 31 is cleared"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(INT31_A::NOT_ACTIVE)
    }
    #[doc = "The interrupt request for channel 31 is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(INT31_A::ACTIVE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline(always)]
    pub fn int16(&self) -> INT16_R {
        INT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline(always)]
    pub fn int17(&self) -> INT17_R {
        INT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline(always)]
    pub fn int18(&self) -> INT18_R {
        INT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline(always)]
    pub fn int19(&self) -> INT19_R {
        INT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline(always)]
    pub fn int20(&self) -> INT20_R {
        INT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline(always)]
    pub fn int21(&self) -> INT21_R {
        INT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline(always)]
    pub fn int22(&self) -> INT22_R {
        INT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline(always)]
    pub fn int23(&self) -> INT23_R {
        INT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline(always)]
    pub fn int24(&self) -> INT24_R {
        INT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline(always)]
    pub fn int25(&self) -> INT25_R {
        INT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline(always)]
    pub fn int26(&self) -> INT26_R {
        INT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline(always)]
    pub fn int27(&self) -> INT27_R {
        INT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline(always)]
    pub fn int28(&self) -> INT28_R {
        INT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline(always)]
    pub fn int29(&self) -> INT29_R {
        INT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline(always)]
    pub fn int30(&self) -> INT30_R {
        INT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline(always)]
    pub fn int31(&self) -> INT31_R {
        INT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> INT0_W<0> {
        INT0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<1> {
        INT1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<2> {
        INT2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<3> {
        INT3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<4> {
        INT4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<5> {
        INT5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<6> {
        INT6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<7> {
        INT7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<8> {
        INT8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<9> {
        INT9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<10> {
        INT10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<11> {
        INT11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<12> {
        INT12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<13> {
        INT13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<14> {
        INT14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<15> {
        INT15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline(always)]
    #[must_use]
    pub fn int16(&mut self) -> INT16_W<16> {
        INT16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline(always)]
    #[must_use]
    pub fn int17(&mut self) -> INT17_W<17> {
        INT17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline(always)]
    #[must_use]
    pub fn int18(&mut self) -> INT18_W<18> {
        INT18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline(always)]
    #[must_use]
    pub fn int19(&mut self) -> INT19_W<19> {
        INT19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline(always)]
    #[must_use]
    pub fn int20(&mut self) -> INT20_W<20> {
        INT20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline(always)]
    #[must_use]
    pub fn int21(&mut self) -> INT21_W<21> {
        INT21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline(always)]
    #[must_use]
    pub fn int22(&mut self) -> INT22_W<22> {
        INT22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline(always)]
    #[must_use]
    pub fn int23(&mut self) -> INT23_W<23> {
        INT23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline(always)]
    #[must_use]
    pub fn int24(&mut self) -> INT24_W<24> {
        INT24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline(always)]
    #[must_use]
    pub fn int25(&mut self) -> INT25_W<25> {
        INT25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline(always)]
    #[must_use]
    pub fn int26(&mut self) -> INT26_W<26> {
        INT26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline(always)]
    #[must_use]
    pub fn int27(&mut self) -> INT27_W<27> {
        INT27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> INT28_W<28> {
        INT28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> INT29_W<29> {
        INT29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> INT30_W<30> {
        INT30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> INT31_W<31> {
        INT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
