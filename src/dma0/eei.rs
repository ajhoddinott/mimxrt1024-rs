#[doc = "Register `EEI` reader"]
pub struct R(crate::R<EEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEI` writer"]
pub struct W(crate::W<EEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEI_SPEC>;
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
impl From<crate::W<EEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub type EEI0_R = crate::BitReader<EEI0_A>;
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI0_A {
    #[doc = "0: An error on channel 0 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 0 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::NO_INTERRUPT,
            true => EEI0_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI0_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI0_A::INTERRUPT
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub type EEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI0_A, O>;
impl<'a, const O: u8> EEI0_W<'a, O> {
    #[doc = "An error on channel 0 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI0_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 0 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI0_A::INTERRUPT)
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub type EEI1_R = crate::BitReader<EEI1_A>;
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI1_A {
    #[doc = "0: An error on channel 1 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 1 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::NO_INTERRUPT,
            true => EEI1_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI1_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI1_A::INTERRUPT
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub type EEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI1_A, O>;
impl<'a, const O: u8> EEI1_W<'a, O> {
    #[doc = "An error on channel 1 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI1_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 1 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI1_A::INTERRUPT)
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub type EEI2_R = crate::BitReader<EEI2_A>;
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI2_A {
    #[doc = "0: An error on channel 2 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 2 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::NO_INTERRUPT,
            true => EEI2_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI2_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI2_A::INTERRUPT
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub type EEI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI2_A, O>;
impl<'a, const O: u8> EEI2_W<'a, O> {
    #[doc = "An error on channel 2 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI2_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 2 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI2_A::INTERRUPT)
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub type EEI3_R = crate::BitReader<EEI3_A>;
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI3_A {
    #[doc = "0: An error on channel 3 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 3 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::NO_INTERRUPT,
            true => EEI3_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI3_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI3_A::INTERRUPT
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub type EEI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI3_A, O>;
impl<'a, const O: u8> EEI3_W<'a, O> {
    #[doc = "An error on channel 3 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI3_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 3 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI3_A::INTERRUPT)
    }
}
#[doc = "Field `EEI4` reader - Enable Error Interrupt 4"]
pub type EEI4_R = crate::BitReader<EEI4_A>;
#[doc = "Enable Error Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI4_A {
    #[doc = "0: An error on channel 4 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 4 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI4_A> for bool {
    #[inline(always)]
    fn from(variant: EEI4_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI4_A {
        match self.bits {
            false => EEI4_A::NO_INTERRUPT,
            true => EEI4_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI4_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI4_A::INTERRUPT
    }
}
#[doc = "Field `EEI4` writer - Enable Error Interrupt 4"]
pub type EEI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI4_A, O>;
impl<'a, const O: u8> EEI4_W<'a, O> {
    #[doc = "An error on channel 4 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI4_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 4 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI4_A::INTERRUPT)
    }
}
#[doc = "Field `EEI5` reader - Enable Error Interrupt 5"]
pub type EEI5_R = crate::BitReader<EEI5_A>;
#[doc = "Enable Error Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI5_A {
    #[doc = "0: An error on channel 5 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 5 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI5_A> for bool {
    #[inline(always)]
    fn from(variant: EEI5_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI5_A {
        match self.bits {
            false => EEI5_A::NO_INTERRUPT,
            true => EEI5_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI5_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI5_A::INTERRUPT
    }
}
#[doc = "Field `EEI5` writer - Enable Error Interrupt 5"]
pub type EEI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI5_A, O>;
impl<'a, const O: u8> EEI5_W<'a, O> {
    #[doc = "An error on channel 5 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI5_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 5 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI5_A::INTERRUPT)
    }
}
#[doc = "Field `EEI6` reader - Enable Error Interrupt 6"]
pub type EEI6_R = crate::BitReader<EEI6_A>;
#[doc = "Enable Error Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI6_A {
    #[doc = "0: An error on channel 6 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 6 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI6_A> for bool {
    #[inline(always)]
    fn from(variant: EEI6_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI6_A {
        match self.bits {
            false => EEI6_A::NO_INTERRUPT,
            true => EEI6_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI6_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI6_A::INTERRUPT
    }
}
#[doc = "Field `EEI6` writer - Enable Error Interrupt 6"]
pub type EEI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI6_A, O>;
impl<'a, const O: u8> EEI6_W<'a, O> {
    #[doc = "An error on channel 6 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI6_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 6 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI6_A::INTERRUPT)
    }
}
#[doc = "Field `EEI7` reader - Enable Error Interrupt 7"]
pub type EEI7_R = crate::BitReader<EEI7_A>;
#[doc = "Enable Error Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI7_A {
    #[doc = "0: An error on channel 7 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 7 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI7_A> for bool {
    #[inline(always)]
    fn from(variant: EEI7_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI7_A {
        match self.bits {
            false => EEI7_A::NO_INTERRUPT,
            true => EEI7_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI7_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI7_A::INTERRUPT
    }
}
#[doc = "Field `EEI7` writer - Enable Error Interrupt 7"]
pub type EEI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI7_A, O>;
impl<'a, const O: u8> EEI7_W<'a, O> {
    #[doc = "An error on channel 7 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI7_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 7 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI7_A::INTERRUPT)
    }
}
#[doc = "Field `EEI8` reader - Enable Error Interrupt 8"]
pub type EEI8_R = crate::BitReader<EEI8_A>;
#[doc = "Enable Error Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI8_A {
    #[doc = "0: An error on channel 8 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 8 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI8_A> for bool {
    #[inline(always)]
    fn from(variant: EEI8_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI8_A {
        match self.bits {
            false => EEI8_A::NO_INTERRUPT,
            true => EEI8_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI8_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI8_A::INTERRUPT
    }
}
#[doc = "Field `EEI8` writer - Enable Error Interrupt 8"]
pub type EEI8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI8_A, O>;
impl<'a, const O: u8> EEI8_W<'a, O> {
    #[doc = "An error on channel 8 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI8_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 8 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI8_A::INTERRUPT)
    }
}
#[doc = "Field `EEI9` reader - Enable Error Interrupt 9"]
pub type EEI9_R = crate::BitReader<EEI9_A>;
#[doc = "Enable Error Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI9_A {
    #[doc = "0: An error on channel 9 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 9 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI9_A> for bool {
    #[inline(always)]
    fn from(variant: EEI9_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI9_A {
        match self.bits {
            false => EEI9_A::NO_INTERRUPT,
            true => EEI9_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI9_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI9_A::INTERRUPT
    }
}
#[doc = "Field `EEI9` writer - Enable Error Interrupt 9"]
pub type EEI9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI9_A, O>;
impl<'a, const O: u8> EEI9_W<'a, O> {
    #[doc = "An error on channel 9 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI9_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 9 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI9_A::INTERRUPT)
    }
}
#[doc = "Field `EEI10` reader - Enable Error Interrupt 10"]
pub type EEI10_R = crate::BitReader<EEI10_A>;
#[doc = "Enable Error Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI10_A {
    #[doc = "0: An error on channel 10 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 10 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI10_A> for bool {
    #[inline(always)]
    fn from(variant: EEI10_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI10_A {
        match self.bits {
            false => EEI10_A::NO_INTERRUPT,
            true => EEI10_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI10_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI10_A::INTERRUPT
    }
}
#[doc = "Field `EEI10` writer - Enable Error Interrupt 10"]
pub type EEI10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI10_A, O>;
impl<'a, const O: u8> EEI10_W<'a, O> {
    #[doc = "An error on channel 10 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI10_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 10 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI10_A::INTERRUPT)
    }
}
#[doc = "Field `EEI11` reader - Enable Error Interrupt 11"]
pub type EEI11_R = crate::BitReader<EEI11_A>;
#[doc = "Enable Error Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI11_A {
    #[doc = "0: An error on channel 11 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 11 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI11_A> for bool {
    #[inline(always)]
    fn from(variant: EEI11_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI11_A {
        match self.bits {
            false => EEI11_A::NO_INTERRUPT,
            true => EEI11_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI11_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI11_A::INTERRUPT
    }
}
#[doc = "Field `EEI11` writer - Enable Error Interrupt 11"]
pub type EEI11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI11_A, O>;
impl<'a, const O: u8> EEI11_W<'a, O> {
    #[doc = "An error on channel 11 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI11_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 11 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI11_A::INTERRUPT)
    }
}
#[doc = "Field `EEI12` reader - Enable Error Interrupt 12"]
pub type EEI12_R = crate::BitReader<EEI12_A>;
#[doc = "Enable Error Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI12_A {
    #[doc = "0: An error on channel 12 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 12 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI12_A> for bool {
    #[inline(always)]
    fn from(variant: EEI12_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI12_A {
        match self.bits {
            false => EEI12_A::NO_INTERRUPT,
            true => EEI12_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI12_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI12_A::INTERRUPT
    }
}
#[doc = "Field `EEI12` writer - Enable Error Interrupt 12"]
pub type EEI12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI12_A, O>;
impl<'a, const O: u8> EEI12_W<'a, O> {
    #[doc = "An error on channel 12 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI12_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 12 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI12_A::INTERRUPT)
    }
}
#[doc = "Field `EEI13` reader - Enable Error Interrupt 13"]
pub type EEI13_R = crate::BitReader<EEI13_A>;
#[doc = "Enable Error Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI13_A {
    #[doc = "0: An error on channel 13 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 13 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI13_A> for bool {
    #[inline(always)]
    fn from(variant: EEI13_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI13_A {
        match self.bits {
            false => EEI13_A::NO_INTERRUPT,
            true => EEI13_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI13_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI13_A::INTERRUPT
    }
}
#[doc = "Field `EEI13` writer - Enable Error Interrupt 13"]
pub type EEI13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI13_A, O>;
impl<'a, const O: u8> EEI13_W<'a, O> {
    #[doc = "An error on channel 13 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI13_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 13 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI13_A::INTERRUPT)
    }
}
#[doc = "Field `EEI14` reader - Enable Error Interrupt 14"]
pub type EEI14_R = crate::BitReader<EEI14_A>;
#[doc = "Enable Error Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI14_A {
    #[doc = "0: An error on channel 14 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 14 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI14_A> for bool {
    #[inline(always)]
    fn from(variant: EEI14_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI14_A {
        match self.bits {
            false => EEI14_A::NO_INTERRUPT,
            true => EEI14_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI14_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI14_A::INTERRUPT
    }
}
#[doc = "Field `EEI14` writer - Enable Error Interrupt 14"]
pub type EEI14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI14_A, O>;
impl<'a, const O: u8> EEI14_W<'a, O> {
    #[doc = "An error on channel 14 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI14_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 14 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI14_A::INTERRUPT)
    }
}
#[doc = "Field `EEI15` reader - Enable Error Interrupt 15"]
pub type EEI15_R = crate::BitReader<EEI15_A>;
#[doc = "Enable Error Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI15_A {
    #[doc = "0: An error on channel 15 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 15 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI15_A> for bool {
    #[inline(always)]
    fn from(variant: EEI15_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI15_A {
        match self.bits {
            false => EEI15_A::NO_INTERRUPT,
            true => EEI15_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI15_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI15_A::INTERRUPT
    }
}
#[doc = "Field `EEI15` writer - Enable Error Interrupt 15"]
pub type EEI15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI15_A, O>;
impl<'a, const O: u8> EEI15_W<'a, O> {
    #[doc = "An error on channel 15 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI15_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 15 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI15_A::INTERRUPT)
    }
}
#[doc = "Field `EEI16` reader - Enable Error Interrupt 16"]
pub type EEI16_R = crate::BitReader<EEI16_A>;
#[doc = "Enable Error Interrupt 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI16_A {
    #[doc = "0: An error on channel 16 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 16 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI16_A> for bool {
    #[inline(always)]
    fn from(variant: EEI16_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI16_A {
        match self.bits {
            false => EEI16_A::NO_INTERRUPT,
            true => EEI16_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI16_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI16_A::INTERRUPT
    }
}
#[doc = "Field `EEI16` writer - Enable Error Interrupt 16"]
pub type EEI16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI16_A, O>;
impl<'a, const O: u8> EEI16_W<'a, O> {
    #[doc = "An error on channel 16 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI16_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 16 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI16_A::INTERRUPT)
    }
}
#[doc = "Field `EEI17` reader - Enable Error Interrupt 17"]
pub type EEI17_R = crate::BitReader<EEI17_A>;
#[doc = "Enable Error Interrupt 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI17_A {
    #[doc = "0: An error on channel 17 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 17 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI17_A> for bool {
    #[inline(always)]
    fn from(variant: EEI17_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI17_A {
        match self.bits {
            false => EEI17_A::NO_INTERRUPT,
            true => EEI17_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI17_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI17_A::INTERRUPT
    }
}
#[doc = "Field `EEI17` writer - Enable Error Interrupt 17"]
pub type EEI17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI17_A, O>;
impl<'a, const O: u8> EEI17_W<'a, O> {
    #[doc = "An error on channel 17 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI17_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 17 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI17_A::INTERRUPT)
    }
}
#[doc = "Field `EEI18` reader - Enable Error Interrupt 18"]
pub type EEI18_R = crate::BitReader<EEI18_A>;
#[doc = "Enable Error Interrupt 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI18_A {
    #[doc = "0: An error on channel 18 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 18 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI18_A> for bool {
    #[inline(always)]
    fn from(variant: EEI18_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI18_A {
        match self.bits {
            false => EEI18_A::NO_INTERRUPT,
            true => EEI18_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI18_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI18_A::INTERRUPT
    }
}
#[doc = "Field `EEI18` writer - Enable Error Interrupt 18"]
pub type EEI18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI18_A, O>;
impl<'a, const O: u8> EEI18_W<'a, O> {
    #[doc = "An error on channel 18 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI18_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 18 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI18_A::INTERRUPT)
    }
}
#[doc = "Field `EEI19` reader - Enable Error Interrupt 19"]
pub type EEI19_R = crate::BitReader<EEI19_A>;
#[doc = "Enable Error Interrupt 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI19_A {
    #[doc = "0: An error on channel 19 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 19 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI19_A> for bool {
    #[inline(always)]
    fn from(variant: EEI19_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI19_A {
        match self.bits {
            false => EEI19_A::NO_INTERRUPT,
            true => EEI19_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI19_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI19_A::INTERRUPT
    }
}
#[doc = "Field `EEI19` writer - Enable Error Interrupt 19"]
pub type EEI19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI19_A, O>;
impl<'a, const O: u8> EEI19_W<'a, O> {
    #[doc = "An error on channel 19 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI19_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 19 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI19_A::INTERRUPT)
    }
}
#[doc = "Field `EEI20` reader - Enable Error Interrupt 20"]
pub type EEI20_R = crate::BitReader<EEI20_A>;
#[doc = "Enable Error Interrupt 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI20_A {
    #[doc = "0: An error on channel 20 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 20 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI20_A> for bool {
    #[inline(always)]
    fn from(variant: EEI20_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI20_A {
        match self.bits {
            false => EEI20_A::NO_INTERRUPT,
            true => EEI20_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI20_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI20_A::INTERRUPT
    }
}
#[doc = "Field `EEI20` writer - Enable Error Interrupt 20"]
pub type EEI20_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI20_A, O>;
impl<'a, const O: u8> EEI20_W<'a, O> {
    #[doc = "An error on channel 20 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI20_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 20 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI20_A::INTERRUPT)
    }
}
#[doc = "Field `EEI21` reader - Enable Error Interrupt 21"]
pub type EEI21_R = crate::BitReader<EEI21_A>;
#[doc = "Enable Error Interrupt 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI21_A {
    #[doc = "0: An error on channel 21 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 21 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI21_A> for bool {
    #[inline(always)]
    fn from(variant: EEI21_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI21_A {
        match self.bits {
            false => EEI21_A::NO_INTERRUPT,
            true => EEI21_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI21_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI21_A::INTERRUPT
    }
}
#[doc = "Field `EEI21` writer - Enable Error Interrupt 21"]
pub type EEI21_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI21_A, O>;
impl<'a, const O: u8> EEI21_W<'a, O> {
    #[doc = "An error on channel 21 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI21_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 21 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI21_A::INTERRUPT)
    }
}
#[doc = "Field `EEI22` reader - Enable Error Interrupt 22"]
pub type EEI22_R = crate::BitReader<EEI22_A>;
#[doc = "Enable Error Interrupt 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI22_A {
    #[doc = "0: An error on channel 22 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 22 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI22_A> for bool {
    #[inline(always)]
    fn from(variant: EEI22_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI22_A {
        match self.bits {
            false => EEI22_A::NO_INTERRUPT,
            true => EEI22_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI22_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI22_A::INTERRUPT
    }
}
#[doc = "Field `EEI22` writer - Enable Error Interrupt 22"]
pub type EEI22_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI22_A, O>;
impl<'a, const O: u8> EEI22_W<'a, O> {
    #[doc = "An error on channel 22 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI22_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 22 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI22_A::INTERRUPT)
    }
}
#[doc = "Field `EEI23` reader - Enable Error Interrupt 23"]
pub type EEI23_R = crate::BitReader<EEI23_A>;
#[doc = "Enable Error Interrupt 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI23_A {
    #[doc = "0: An error on channel 23 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 23 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI23_A> for bool {
    #[inline(always)]
    fn from(variant: EEI23_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI23_A {
        match self.bits {
            false => EEI23_A::NO_INTERRUPT,
            true => EEI23_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI23_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI23_A::INTERRUPT
    }
}
#[doc = "Field `EEI23` writer - Enable Error Interrupt 23"]
pub type EEI23_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI23_A, O>;
impl<'a, const O: u8> EEI23_W<'a, O> {
    #[doc = "An error on channel 23 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI23_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 23 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI23_A::INTERRUPT)
    }
}
#[doc = "Field `EEI24` reader - Enable Error Interrupt 24"]
pub type EEI24_R = crate::BitReader<EEI24_A>;
#[doc = "Enable Error Interrupt 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI24_A {
    #[doc = "0: An error on channel 24 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 24 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI24_A> for bool {
    #[inline(always)]
    fn from(variant: EEI24_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI24_A {
        match self.bits {
            false => EEI24_A::NO_INTERRUPT,
            true => EEI24_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI24_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI24_A::INTERRUPT
    }
}
#[doc = "Field `EEI24` writer - Enable Error Interrupt 24"]
pub type EEI24_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI24_A, O>;
impl<'a, const O: u8> EEI24_W<'a, O> {
    #[doc = "An error on channel 24 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI24_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 24 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI24_A::INTERRUPT)
    }
}
#[doc = "Field `EEI25` reader - Enable Error Interrupt 25"]
pub type EEI25_R = crate::BitReader<EEI25_A>;
#[doc = "Enable Error Interrupt 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI25_A {
    #[doc = "0: An error on channel 25 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 25 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI25_A> for bool {
    #[inline(always)]
    fn from(variant: EEI25_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI25_A {
        match self.bits {
            false => EEI25_A::NO_INTERRUPT,
            true => EEI25_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI25_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI25_A::INTERRUPT
    }
}
#[doc = "Field `EEI25` writer - Enable Error Interrupt 25"]
pub type EEI25_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI25_A, O>;
impl<'a, const O: u8> EEI25_W<'a, O> {
    #[doc = "An error on channel 25 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI25_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 25 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI25_A::INTERRUPT)
    }
}
#[doc = "Field `EEI26` reader - Enable Error Interrupt 26"]
pub type EEI26_R = crate::BitReader<EEI26_A>;
#[doc = "Enable Error Interrupt 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI26_A {
    #[doc = "0: An error on channel 26 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 26 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI26_A> for bool {
    #[inline(always)]
    fn from(variant: EEI26_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI26_A {
        match self.bits {
            false => EEI26_A::NO_INTERRUPT,
            true => EEI26_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI26_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI26_A::INTERRUPT
    }
}
#[doc = "Field `EEI26` writer - Enable Error Interrupt 26"]
pub type EEI26_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI26_A, O>;
impl<'a, const O: u8> EEI26_W<'a, O> {
    #[doc = "An error on channel 26 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI26_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 26 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI26_A::INTERRUPT)
    }
}
#[doc = "Field `EEI27` reader - Enable Error Interrupt 27"]
pub type EEI27_R = crate::BitReader<EEI27_A>;
#[doc = "Enable Error Interrupt 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI27_A {
    #[doc = "0: An error on channel 27 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 27 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI27_A> for bool {
    #[inline(always)]
    fn from(variant: EEI27_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI27_A {
        match self.bits {
            false => EEI27_A::NO_INTERRUPT,
            true => EEI27_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI27_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI27_A::INTERRUPT
    }
}
#[doc = "Field `EEI27` writer - Enable Error Interrupt 27"]
pub type EEI27_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI27_A, O>;
impl<'a, const O: u8> EEI27_W<'a, O> {
    #[doc = "An error on channel 27 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI27_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 27 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI27_A::INTERRUPT)
    }
}
#[doc = "Field `EEI28` reader - Enable Error Interrupt 28"]
pub type EEI28_R = crate::BitReader<EEI28_A>;
#[doc = "Enable Error Interrupt 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI28_A {
    #[doc = "0: An error on channel 28 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 28 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI28_A> for bool {
    #[inline(always)]
    fn from(variant: EEI28_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI28_A {
        match self.bits {
            false => EEI28_A::NO_INTERRUPT,
            true => EEI28_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI28_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI28_A::INTERRUPT
    }
}
#[doc = "Field `EEI28` writer - Enable Error Interrupt 28"]
pub type EEI28_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI28_A, O>;
impl<'a, const O: u8> EEI28_W<'a, O> {
    #[doc = "An error on channel 28 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI28_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 28 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI28_A::INTERRUPT)
    }
}
#[doc = "Field `EEI29` reader - Enable Error Interrupt 29"]
pub type EEI29_R = crate::BitReader<EEI29_A>;
#[doc = "Enable Error Interrupt 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI29_A {
    #[doc = "0: An error on channel 29 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 29 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI29_A> for bool {
    #[inline(always)]
    fn from(variant: EEI29_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI29_A {
        match self.bits {
            false => EEI29_A::NO_INTERRUPT,
            true => EEI29_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI29_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI29_A::INTERRUPT
    }
}
#[doc = "Field `EEI29` writer - Enable Error Interrupt 29"]
pub type EEI29_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI29_A, O>;
impl<'a, const O: u8> EEI29_W<'a, O> {
    #[doc = "An error on channel 29 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI29_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 29 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI29_A::INTERRUPT)
    }
}
#[doc = "Field `EEI30` reader - Enable Error Interrupt 30"]
pub type EEI30_R = crate::BitReader<EEI30_A>;
#[doc = "Enable Error Interrupt 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI30_A {
    #[doc = "0: An error on channel 30 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 30 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI30_A> for bool {
    #[inline(always)]
    fn from(variant: EEI30_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI30_A {
        match self.bits {
            false => EEI30_A::NO_INTERRUPT,
            true => EEI30_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI30_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI30_A::INTERRUPT
    }
}
#[doc = "Field `EEI30` writer - Enable Error Interrupt 30"]
pub type EEI30_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI30_A, O>;
impl<'a, const O: u8> EEI30_W<'a, O> {
    #[doc = "An error on channel 30 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI30_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 30 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI30_A::INTERRUPT)
    }
}
#[doc = "Field `EEI31` reader - Enable Error Interrupt 31"]
pub type EEI31_R = crate::BitReader<EEI31_A>;
#[doc = "Enable Error Interrupt 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI31_A {
    #[doc = "0: An error on channel 31 does not generate an error interrupt"]
    NO_INTERRUPT = 0,
    #[doc = "1: An error on channel 31 generates an error interrupt request"]
    INTERRUPT = 1,
}
impl From<EEI31_A> for bool {
    #[inline(always)]
    fn from(variant: EEI31_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI31_A {
        match self.bits {
            false => EEI31_A::NO_INTERRUPT,
            true => EEI31_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == EEI31_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EEI31_A::INTERRUPT
    }
}
#[doc = "Field `EEI31` writer - Enable Error Interrupt 31"]
pub type EEI31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI31_A, O>;
impl<'a, const O: u8> EEI31_W<'a, O> {
    #[doc = "An error on channel 31 does not generate an error interrupt"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(EEI31_A::NO_INTERRUPT)
    }
    #[doc = "An error on channel 31 generates an error interrupt request"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EEI31_A::INTERRUPT)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&self) -> EEI4_R {
        EEI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&self) -> EEI5_R {
        EEI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&self) -> EEI6_R {
        EEI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&self) -> EEI7_R {
        EEI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&self) -> EEI8_R {
        EEI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&self) -> EEI9_R {
        EEI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&self) -> EEI10_R {
        EEI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&self) -> EEI11_R {
        EEI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&self) -> EEI12_R {
        EEI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&self) -> EEI13_R {
        EEI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&self) -> EEI14_R {
        EEI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&self) -> EEI15_R {
        EEI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline(always)]
    pub fn eei16(&self) -> EEI16_R {
        EEI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline(always)]
    pub fn eei17(&self) -> EEI17_R {
        EEI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline(always)]
    pub fn eei18(&self) -> EEI18_R {
        EEI18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline(always)]
    pub fn eei19(&self) -> EEI19_R {
        EEI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline(always)]
    pub fn eei20(&self) -> EEI20_R {
        EEI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline(always)]
    pub fn eei21(&self) -> EEI21_R {
        EEI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline(always)]
    pub fn eei22(&self) -> EEI22_R {
        EEI22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline(always)]
    pub fn eei23(&self) -> EEI23_R {
        EEI23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline(always)]
    pub fn eei24(&self) -> EEI24_R {
        EEI24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline(always)]
    pub fn eei25(&self) -> EEI25_R {
        EEI25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline(always)]
    pub fn eei26(&self) -> EEI26_R {
        EEI26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline(always)]
    pub fn eei27(&self) -> EEI27_R {
        EEI27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline(always)]
    pub fn eei28(&self) -> EEI28_R {
        EEI28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline(always)]
    pub fn eei29(&self) -> EEI29_R {
        EEI29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline(always)]
    pub fn eei30(&self) -> EEI30_R {
        EEI30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline(always)]
    pub fn eei31(&self) -> EEI31_R {
        EEI31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn eei0(&mut self) -> EEI0_W<0> {
        EEI0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn eei1(&mut self) -> EEI1_W<1> {
        EEI1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn eei2(&mut self) -> EEI2_W<2> {
        EEI2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn eei3(&mut self) -> EEI3_W<3> {
        EEI3_W::new(self)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn eei4(&mut self) -> EEI4_W<4> {
        EEI4_W::new(self)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn eei5(&mut self) -> EEI5_W<5> {
        EEI5_W::new(self)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn eei6(&mut self) -> EEI6_W<6> {
        EEI6_W::new(self)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn eei7(&mut self) -> EEI7_W<7> {
        EEI7_W::new(self)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn eei8(&mut self) -> EEI8_W<8> {
        EEI8_W::new(self)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn eei9(&mut self) -> EEI9_W<9> {
        EEI9_W::new(self)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn eei10(&mut self) -> EEI10_W<10> {
        EEI10_W::new(self)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn eei11(&mut self) -> EEI11_W<11> {
        EEI11_W::new(self)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn eei12(&mut self) -> EEI12_W<12> {
        EEI12_W::new(self)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn eei13(&mut self) -> EEI13_W<13> {
        EEI13_W::new(self)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn eei14(&mut self) -> EEI14_W<14> {
        EEI14_W::new(self)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn eei15(&mut self) -> EEI15_W<15> {
        EEI15_W::new(self)
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn eei16(&mut self) -> EEI16_W<16> {
        EEI16_W::new(self)
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn eei17(&mut self) -> EEI17_W<17> {
        EEI17_W::new(self)
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn eei18(&mut self) -> EEI18_W<18> {
        EEI18_W::new(self)
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn eei19(&mut self) -> EEI19_W<19> {
        EEI19_W::new(self)
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn eei20(&mut self) -> EEI20_W<20> {
        EEI20_W::new(self)
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn eei21(&mut self) -> EEI21_W<21> {
        EEI21_W::new(self)
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn eei22(&mut self) -> EEI22_W<22> {
        EEI22_W::new(self)
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn eei23(&mut self) -> EEI23_W<23> {
        EEI23_W::new(self)
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn eei24(&mut self) -> EEI24_W<24> {
        EEI24_W::new(self)
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn eei25(&mut self) -> EEI25_W<25> {
        EEI25_W::new(self)
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn eei26(&mut self) -> EEI26_W<26> {
        EEI26_W::new(self)
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn eei27(&mut self) -> EEI27_W<27> {
        EEI27_W::new(self)
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn eei28(&mut self) -> EEI28_W<28> {
        EEI28_W::new(self)
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn eei29(&mut self) -> EEI29_W<29> {
        EEI29_W::new(self)
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn eei30(&mut self) -> EEI30_W<30> {
        EEI30_W::new(self)
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn eei31(&mut self) -> EEI31_W<31> {
        EEI31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Error Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](index.html) module"]
pub struct EEI_SPEC;
impl crate::RegisterSpec for EEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eei::R](R) reader structure"]
impl crate::Readable for EEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eei::W](W) writer structure"]
impl crate::Writable for EEI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
