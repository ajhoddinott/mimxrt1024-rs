#[doc = "Register `CPACR` reader"]
pub struct R(crate::R<CPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPACR` writer"]
pub struct W(crate::W<CPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPACR_SPEC>;
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
impl From<crate::W<CPACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP0` reader - Access privileges for coprocessor 0."]
pub type CP0_R = crate::FieldReader<u8, CP0_A>;
#[doc = "Access privileges for coprocessor 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP0_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP0_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP0_1 = 1,
    #[doc = "3: Full access."]
    CP0_3 = 3,
}
impl From<CP0_A> for u8 {
    #[inline(always)]
    fn from(variant: CP0_A) -> Self {
        variant as _
    }
}
impl CP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP0_A> {
        match self.bits {
            0 => Some(CP0_A::CP0_0),
            1 => Some(CP0_A::CP0_1),
            3 => Some(CP0_A::CP0_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP0_0`"]
    #[inline(always)]
    pub fn is_cp0_0(&self) -> bool {
        *self == CP0_A::CP0_0
    }
    #[doc = "Checks if the value of the field is `CP0_1`"]
    #[inline(always)]
    pub fn is_cp0_1(&self) -> bool {
        *self == CP0_A::CP0_1
    }
    #[doc = "Checks if the value of the field is `CP0_3`"]
    #[inline(always)]
    pub fn is_cp0_3(&self) -> bool {
        *self == CP0_A::CP0_3
    }
}
#[doc = "Field `CP0` writer - Access privileges for coprocessor 0."]
pub type CP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP0_A, 2, O>;
impl<'a, const O: u8> CP0_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp0_0(self) -> &'a mut W {
        self.variant(CP0_A::CP0_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp0_1(self) -> &'a mut W {
        self.variant(CP0_A::CP0_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp0_3(self) -> &'a mut W {
        self.variant(CP0_A::CP0_3)
    }
}
#[doc = "Field `CP1` reader - Access privileges for coprocessor 1."]
pub type CP1_R = crate::FieldReader<u8, CP1_A>;
#[doc = "Access privileges for coprocessor 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP1_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP1_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP1_1 = 1,
    #[doc = "3: Full access."]
    CP1_3 = 3,
}
impl From<CP1_A> for u8 {
    #[inline(always)]
    fn from(variant: CP1_A) -> Self {
        variant as _
    }
}
impl CP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP1_A> {
        match self.bits {
            0 => Some(CP1_A::CP1_0),
            1 => Some(CP1_A::CP1_1),
            3 => Some(CP1_A::CP1_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP1_0`"]
    #[inline(always)]
    pub fn is_cp1_0(&self) -> bool {
        *self == CP1_A::CP1_0
    }
    #[doc = "Checks if the value of the field is `CP1_1`"]
    #[inline(always)]
    pub fn is_cp1_1(&self) -> bool {
        *self == CP1_A::CP1_1
    }
    #[doc = "Checks if the value of the field is `CP1_3`"]
    #[inline(always)]
    pub fn is_cp1_3(&self) -> bool {
        *self == CP1_A::CP1_3
    }
}
#[doc = "Field `CP1` writer - Access privileges for coprocessor 1."]
pub type CP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP1_A, 2, O>;
impl<'a, const O: u8> CP1_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp1_0(self) -> &'a mut W {
        self.variant(CP1_A::CP1_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp1_1(self) -> &'a mut W {
        self.variant(CP1_A::CP1_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp1_3(self) -> &'a mut W {
        self.variant(CP1_A::CP1_3)
    }
}
#[doc = "Field `CP2` reader - Access privileges for coprocessor 2."]
pub type CP2_R = crate::FieldReader<u8, CP2_A>;
#[doc = "Access privileges for coprocessor 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP2_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP2_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP2_1 = 1,
    #[doc = "3: Full access."]
    CP2_3 = 3,
}
impl From<CP2_A> for u8 {
    #[inline(always)]
    fn from(variant: CP2_A) -> Self {
        variant as _
    }
}
impl CP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP2_A> {
        match self.bits {
            0 => Some(CP2_A::CP2_0),
            1 => Some(CP2_A::CP2_1),
            3 => Some(CP2_A::CP2_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP2_0`"]
    #[inline(always)]
    pub fn is_cp2_0(&self) -> bool {
        *self == CP2_A::CP2_0
    }
    #[doc = "Checks if the value of the field is `CP2_1`"]
    #[inline(always)]
    pub fn is_cp2_1(&self) -> bool {
        *self == CP2_A::CP2_1
    }
    #[doc = "Checks if the value of the field is `CP2_3`"]
    #[inline(always)]
    pub fn is_cp2_3(&self) -> bool {
        *self == CP2_A::CP2_3
    }
}
#[doc = "Field `CP2` writer - Access privileges for coprocessor 2."]
pub type CP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP2_A, 2, O>;
impl<'a, const O: u8> CP2_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp2_0(self) -> &'a mut W {
        self.variant(CP2_A::CP2_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp2_1(self) -> &'a mut W {
        self.variant(CP2_A::CP2_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp2_3(self) -> &'a mut W {
        self.variant(CP2_A::CP2_3)
    }
}
#[doc = "Field `CP3` reader - Access privileges for coprocessor 3."]
pub type CP3_R = crate::FieldReader<u8, CP3_A>;
#[doc = "Access privileges for coprocessor 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP3_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP3_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP3_1 = 1,
    #[doc = "3: Full access."]
    CP3_3 = 3,
}
impl From<CP3_A> for u8 {
    #[inline(always)]
    fn from(variant: CP3_A) -> Self {
        variant as _
    }
}
impl CP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP3_A> {
        match self.bits {
            0 => Some(CP3_A::CP3_0),
            1 => Some(CP3_A::CP3_1),
            3 => Some(CP3_A::CP3_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP3_0`"]
    #[inline(always)]
    pub fn is_cp3_0(&self) -> bool {
        *self == CP3_A::CP3_0
    }
    #[doc = "Checks if the value of the field is `CP3_1`"]
    #[inline(always)]
    pub fn is_cp3_1(&self) -> bool {
        *self == CP3_A::CP3_1
    }
    #[doc = "Checks if the value of the field is `CP3_3`"]
    #[inline(always)]
    pub fn is_cp3_3(&self) -> bool {
        *self == CP3_A::CP3_3
    }
}
#[doc = "Field `CP3` writer - Access privileges for coprocessor 3."]
pub type CP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP3_A, 2, O>;
impl<'a, const O: u8> CP3_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp3_0(self) -> &'a mut W {
        self.variant(CP3_A::CP3_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp3_1(self) -> &'a mut W {
        self.variant(CP3_A::CP3_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp3_3(self) -> &'a mut W {
        self.variant(CP3_A::CP3_3)
    }
}
#[doc = "Field `CP4` reader - Access privileges for coprocessor 4."]
pub type CP4_R = crate::FieldReader<u8, CP4_A>;
#[doc = "Access privileges for coprocessor 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP4_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP4_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP4_1 = 1,
    #[doc = "3: Full access."]
    CP4_3 = 3,
}
impl From<CP4_A> for u8 {
    #[inline(always)]
    fn from(variant: CP4_A) -> Self {
        variant as _
    }
}
impl CP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP4_A> {
        match self.bits {
            0 => Some(CP4_A::CP4_0),
            1 => Some(CP4_A::CP4_1),
            3 => Some(CP4_A::CP4_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP4_0`"]
    #[inline(always)]
    pub fn is_cp4_0(&self) -> bool {
        *self == CP4_A::CP4_0
    }
    #[doc = "Checks if the value of the field is `CP4_1`"]
    #[inline(always)]
    pub fn is_cp4_1(&self) -> bool {
        *self == CP4_A::CP4_1
    }
    #[doc = "Checks if the value of the field is `CP4_3`"]
    #[inline(always)]
    pub fn is_cp4_3(&self) -> bool {
        *self == CP4_A::CP4_3
    }
}
#[doc = "Field `CP4` writer - Access privileges for coprocessor 4."]
pub type CP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP4_A, 2, O>;
impl<'a, const O: u8> CP4_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp4_0(self) -> &'a mut W {
        self.variant(CP4_A::CP4_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp4_1(self) -> &'a mut W {
        self.variant(CP4_A::CP4_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp4_3(self) -> &'a mut W {
        self.variant(CP4_A::CP4_3)
    }
}
#[doc = "Field `CP5` reader - Access privileges for coprocessor 5."]
pub type CP5_R = crate::FieldReader<u8, CP5_A>;
#[doc = "Access privileges for coprocessor 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP5_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP5_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP5_1 = 1,
    #[doc = "3: Full access."]
    CP5_3 = 3,
}
impl From<CP5_A> for u8 {
    #[inline(always)]
    fn from(variant: CP5_A) -> Self {
        variant as _
    }
}
impl CP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP5_A> {
        match self.bits {
            0 => Some(CP5_A::CP5_0),
            1 => Some(CP5_A::CP5_1),
            3 => Some(CP5_A::CP5_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP5_0`"]
    #[inline(always)]
    pub fn is_cp5_0(&self) -> bool {
        *self == CP5_A::CP5_0
    }
    #[doc = "Checks if the value of the field is `CP5_1`"]
    #[inline(always)]
    pub fn is_cp5_1(&self) -> bool {
        *self == CP5_A::CP5_1
    }
    #[doc = "Checks if the value of the field is `CP5_3`"]
    #[inline(always)]
    pub fn is_cp5_3(&self) -> bool {
        *self == CP5_A::CP5_3
    }
}
#[doc = "Field `CP5` writer - Access privileges for coprocessor 5."]
pub type CP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP5_A, 2, O>;
impl<'a, const O: u8> CP5_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp5_0(self) -> &'a mut W {
        self.variant(CP5_A::CP5_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp5_1(self) -> &'a mut W {
        self.variant(CP5_A::CP5_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp5_3(self) -> &'a mut W {
        self.variant(CP5_A::CP5_3)
    }
}
#[doc = "Field `CP6` reader - Access privileges for coprocessor 6."]
pub type CP6_R = crate::FieldReader<u8, CP6_A>;
#[doc = "Access privileges for coprocessor 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP6_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP6_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP6_1 = 1,
    #[doc = "3: Full access."]
    CP6_3 = 3,
}
impl From<CP6_A> for u8 {
    #[inline(always)]
    fn from(variant: CP6_A) -> Self {
        variant as _
    }
}
impl CP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP6_A> {
        match self.bits {
            0 => Some(CP6_A::CP6_0),
            1 => Some(CP6_A::CP6_1),
            3 => Some(CP6_A::CP6_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP6_0`"]
    #[inline(always)]
    pub fn is_cp6_0(&self) -> bool {
        *self == CP6_A::CP6_0
    }
    #[doc = "Checks if the value of the field is `CP6_1`"]
    #[inline(always)]
    pub fn is_cp6_1(&self) -> bool {
        *self == CP6_A::CP6_1
    }
    #[doc = "Checks if the value of the field is `CP6_3`"]
    #[inline(always)]
    pub fn is_cp6_3(&self) -> bool {
        *self == CP6_A::CP6_3
    }
}
#[doc = "Field `CP6` writer - Access privileges for coprocessor 6."]
pub type CP6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP6_A, 2, O>;
impl<'a, const O: u8> CP6_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp6_0(self) -> &'a mut W {
        self.variant(CP6_A::CP6_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp6_1(self) -> &'a mut W {
        self.variant(CP6_A::CP6_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp6_3(self) -> &'a mut W {
        self.variant(CP6_A::CP6_3)
    }
}
#[doc = "Field `CP7` reader - Access privileges for coprocessor 7."]
pub type CP7_R = crate::FieldReader<u8, CP7_A>;
#[doc = "Access privileges for coprocessor 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP7_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP7_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP7_1 = 1,
    #[doc = "3: Full access."]
    CP7_3 = 3,
}
impl From<CP7_A> for u8 {
    #[inline(always)]
    fn from(variant: CP7_A) -> Self {
        variant as _
    }
}
impl CP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP7_A> {
        match self.bits {
            0 => Some(CP7_A::CP7_0),
            1 => Some(CP7_A::CP7_1),
            3 => Some(CP7_A::CP7_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP7_0`"]
    #[inline(always)]
    pub fn is_cp7_0(&self) -> bool {
        *self == CP7_A::CP7_0
    }
    #[doc = "Checks if the value of the field is `CP7_1`"]
    #[inline(always)]
    pub fn is_cp7_1(&self) -> bool {
        *self == CP7_A::CP7_1
    }
    #[doc = "Checks if the value of the field is `CP7_3`"]
    #[inline(always)]
    pub fn is_cp7_3(&self) -> bool {
        *self == CP7_A::CP7_3
    }
}
#[doc = "Field `CP7` writer - Access privileges for coprocessor 7."]
pub type CP7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP7_A, 2, O>;
impl<'a, const O: u8> CP7_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp7_0(self) -> &'a mut W {
        self.variant(CP7_A::CP7_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp7_1(self) -> &'a mut W {
        self.variant(CP7_A::CP7_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp7_3(self) -> &'a mut W {
        self.variant(CP7_A::CP7_3)
    }
}
#[doc = "Field `CP10` reader - Access privileges for coprocessor 10."]
pub type CP10_R = crate::FieldReader<u8, CP10_A>;
#[doc = "Access privileges for coprocessor 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP10_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP10_1 = 1,
    #[doc = "3: Full access."]
    CP10_3 = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
impl CP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP10_A> {
        match self.bits {
            0 => Some(CP10_A::CP10_0),
            1 => Some(CP10_A::CP10_1),
            3 => Some(CP10_A::CP10_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP10_0`"]
    #[inline(always)]
    pub fn is_cp10_0(&self) -> bool {
        *self == CP10_A::CP10_0
    }
    #[doc = "Checks if the value of the field is `CP10_1`"]
    #[inline(always)]
    pub fn is_cp10_1(&self) -> bool {
        *self == CP10_A::CP10_1
    }
    #[doc = "Checks if the value of the field is `CP10_3`"]
    #[inline(always)]
    pub fn is_cp10_3(&self) -> bool {
        *self == CP10_A::CP10_3
    }
}
#[doc = "Field `CP10` writer - Access privileges for coprocessor 10."]
pub type CP10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP10_A, 2, O>;
impl<'a, const O: u8> CP10_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp10_0(self) -> &'a mut W {
        self.variant(CP10_A::CP10_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp10_1(self) -> &'a mut W {
        self.variant(CP10_A::CP10_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp10_3(self) -> &'a mut W {
        self.variant(CP10_A::CP10_3)
    }
}
#[doc = "Field `CP11` reader - Access privileges for coprocessor 11."]
pub type CP11_R = crate::FieldReader<u8, CP11_A>;
#[doc = "Access privileges for coprocessor 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP11_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP11_1 = 1,
    #[doc = "3: Full access."]
    CP11_3 = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
impl CP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CP11_A> {
        match self.bits {
            0 => Some(CP11_A::CP11_0),
            1 => Some(CP11_A::CP11_1),
            3 => Some(CP11_A::CP11_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CP11_0`"]
    #[inline(always)]
    pub fn is_cp11_0(&self) -> bool {
        *self == CP11_A::CP11_0
    }
    #[doc = "Checks if the value of the field is `CP11_1`"]
    #[inline(always)]
    pub fn is_cp11_1(&self) -> bool {
        *self == CP11_A::CP11_1
    }
    #[doc = "Checks if the value of the field is `CP11_3`"]
    #[inline(always)]
    pub fn is_cp11_3(&self) -> bool {
        *self == CP11_A::CP11_3
    }
}
#[doc = "Field `CP11` writer - Access privileges for coprocessor 11."]
pub type CP11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, CP11_A, 2, O>;
impl<'a, const O: u8> CP11_W<'a, O> {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp11_0(self) -> &'a mut W {
        self.variant(CP11_A::CP11_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp11_1(self) -> &'a mut W {
        self.variant(CP11_A::CP11_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp11_3(self) -> &'a mut W {
        self.variant(CP11_A::CP11_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline(always)]
    pub fn cp2(&self) -> CP2_R {
        CP2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline(always)]
    pub fn cp3(&self) -> CP3_R {
        CP3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline(always)]
    pub fn cp4(&self) -> CP4_R {
        CP4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline(always)]
    pub fn cp5(&self) -> CP5_R {
        CP5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline(always)]
    pub fn cp6(&self) -> CP6_R {
        CP6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline(always)]
    pub fn cp7(&self) -> CP7_R {
        CP7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline(always)]
    #[must_use]
    pub fn cp0(&mut self) -> CP0_W<0> {
        CP0_W::new(self)
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline(always)]
    #[must_use]
    pub fn cp1(&mut self) -> CP1_W<2> {
        CP1_W::new(self)
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline(always)]
    #[must_use]
    pub fn cp2(&mut self) -> CP2_W<4> {
        CP2_W::new(self)
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline(always)]
    #[must_use]
    pub fn cp3(&mut self) -> CP3_W<6> {
        CP3_W::new(self)
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline(always)]
    #[must_use]
    pub fn cp4(&mut self) -> CP4_W<8> {
        CP4_W::new(self)
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline(always)]
    #[must_use]
    pub fn cp5(&mut self) -> CP5_W<10> {
        CP5_W::new(self)
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline(always)]
    #[must_use]
    pub fn cp6(&mut self) -> CP6_W<12> {
        CP6_W::new(self)
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline(always)]
    #[must_use]
    pub fn cp7(&mut self) -> CP7_W<14> {
        CP7_W::new(self)
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    #[must_use]
    pub fn cp10(&mut self) -> CP10_W<20> {
        CP10_W::new(self)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    #[must_use]
    pub fn cp11(&mut self) -> CP11_W<22> {
        CP11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coprocessor Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpacr](index.html) module"]
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpacr::R](R) reader structure"]
impl crate::Readable for CPACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpacr::W](W) writer structure"]
impl crate::Writable for CPACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPACR to value 0"]
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
