#[doc = "Register `GPR14` reader"]
pub struct R(crate::R<GPR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR14` writer"]
pub struct W(crate::W<GPR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR14_SPEC>;
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
impl From<crate::W<GPR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP1_CMP_IGEN_TRIM_DN` reader - reduces ACMP1 internal bias current by 30%"]
pub type ACMP1_CMP_IGEN_TRIM_DN_R = crate::BitReader<ACMP1_CMP_IGEN_TRIM_DN_A>;
#[doc = "reduces ACMP1 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP1_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP1_CMP_IGEN_TRIM_DN_0 = 0,
    #[doc = "1: reduces"]
    ACMP1_CMP_IGEN_TRIM_DN_1 = 1,
}
impl From<ACMP1_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_CMP_IGEN_TRIM_DN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP1_CMP_IGEN_TRIM_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0,
            true => ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Field `ACMP1_CMP_IGEN_TRIM_DN` writer - reduces ACMP1 internal bias current by 30%"]
pub type ACMP1_CMP_IGEN_TRIM_DN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP1_CMP_IGEN_TRIM_DN_A, O>;
impl<'a, const O: u8> ACMP1_CMP_IGEN_TRIM_DN_W<'a, O> {
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_DN_A::ACMP1_CMP_IGEN_TRIM_DN_1)
    }
}
#[doc = "Field `ACMP2_CMP_IGEN_TRIM_DN` reader - reduces ACMP2 internal bias current by 30%"]
pub type ACMP2_CMP_IGEN_TRIM_DN_R = crate::BitReader<ACMP2_CMP_IGEN_TRIM_DN_A>;
#[doc = "reduces ACMP2 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP2_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP2_CMP_IGEN_TRIM_DN_0 = 0,
    #[doc = "1: reduces"]
    ACMP2_CMP_IGEN_TRIM_DN_1 = 1,
}
impl From<ACMP2_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_CMP_IGEN_TRIM_DN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP2_CMP_IGEN_TRIM_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0,
            true => ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Field `ACMP2_CMP_IGEN_TRIM_DN` writer - reduces ACMP2 internal bias current by 30%"]
pub type ACMP2_CMP_IGEN_TRIM_DN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP2_CMP_IGEN_TRIM_DN_A, O>;
impl<'a, const O: u8> ACMP2_CMP_IGEN_TRIM_DN_W<'a, O> {
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_DN_A::ACMP2_CMP_IGEN_TRIM_DN_1)
    }
}
#[doc = "Field `ACMP3_CMP_IGEN_TRIM_DN` reader - reduces ACMP3 internal bias current by 30%"]
pub type ACMP3_CMP_IGEN_TRIM_DN_R = crate::BitReader<ACMP3_CMP_IGEN_TRIM_DN_A>;
#[doc = "reduces ACMP3 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP3_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP3_CMP_IGEN_TRIM_DN_0 = 0,
    #[doc = "1: reduces"]
    ACMP3_CMP_IGEN_TRIM_DN_1 = 1,
}
impl From<ACMP3_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_CMP_IGEN_TRIM_DN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP3_CMP_IGEN_TRIM_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0,
            true => ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Field `ACMP3_CMP_IGEN_TRIM_DN` writer - reduces ACMP3 internal bias current by 30%"]
pub type ACMP3_CMP_IGEN_TRIM_DN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP3_CMP_IGEN_TRIM_DN_A, O>;
impl<'a, const O: u8> ACMP3_CMP_IGEN_TRIM_DN_W<'a, O> {
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_DN_A::ACMP3_CMP_IGEN_TRIM_DN_1)
    }
}
#[doc = "Field `ACMP4_CMP_IGEN_TRIM_DN` reader - reduces ACMP4 internal bias current by 30%"]
pub type ACMP4_CMP_IGEN_TRIM_DN_R = crate::BitReader<ACMP4_CMP_IGEN_TRIM_DN_A>;
#[doc = "reduces ACMP4 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP4_CMP_IGEN_TRIM_DN_A {
    #[doc = "0: no reduce"]
    ACMP4_CMP_IGEN_TRIM_DN_0 = 0,
    #[doc = "1: reduces"]
    ACMP4_CMP_IGEN_TRIM_DN_1 = 1,
}
impl From<ACMP4_CMP_IGEN_TRIM_DN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_CMP_IGEN_TRIM_DN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP4_CMP_IGEN_TRIM_DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_CMP_IGEN_TRIM_DN_A {
        match self.bits {
            false => ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0,
            true => ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_0`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_dn_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_DN_1`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_dn_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1
    }
}
#[doc = "Field `ACMP4_CMP_IGEN_TRIM_DN` writer - reduces ACMP4 internal bias current by 30%"]
pub type ACMP4_CMP_IGEN_TRIM_DN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP4_CMP_IGEN_TRIM_DN_A, O>;
impl<'a, const O: u8> ACMP4_CMP_IGEN_TRIM_DN_W<'a, O> {
    #[doc = "no reduce"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_0)
    }
    #[doc = "reduces"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_DN_A::ACMP4_CMP_IGEN_TRIM_DN_1)
    }
}
#[doc = "Field `ACMP1_CMP_IGEN_TRIM_UP` reader - increases ACMP1 internal bias current by 30%"]
pub type ACMP1_CMP_IGEN_TRIM_UP_R = crate::BitReader<ACMP1_CMP_IGEN_TRIM_UP_A>;
#[doc = "increases ACMP1 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP1_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP1_CMP_IGEN_TRIM_UP_0 = 0,
    #[doc = "1: increases"]
    ACMP1_CMP_IGEN_TRIM_UP_1 = 1,
}
impl From<ACMP1_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_CMP_IGEN_TRIM_UP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP1_CMP_IGEN_TRIM_UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0,
            true => ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp1_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Field `ACMP1_CMP_IGEN_TRIM_UP` writer - increases ACMP1 internal bias current by 30%"]
pub type ACMP1_CMP_IGEN_TRIM_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP1_CMP_IGEN_TRIM_UP_A, O>;
impl<'a, const O: u8> ACMP1_CMP_IGEN_TRIM_UP_W<'a, O> {
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP1_CMP_IGEN_TRIM_UP_A::ACMP1_CMP_IGEN_TRIM_UP_1)
    }
}
#[doc = "Field `ACMP2_CMP_IGEN_TRIM_UP` reader - increases ACMP2 internal bias current by 30%"]
pub type ACMP2_CMP_IGEN_TRIM_UP_R = crate::BitReader<ACMP2_CMP_IGEN_TRIM_UP_A>;
#[doc = "increases ACMP2 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP2_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP2_CMP_IGEN_TRIM_UP_0 = 0,
    #[doc = "1: increases"]
    ACMP2_CMP_IGEN_TRIM_UP_1 = 1,
}
impl From<ACMP2_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_CMP_IGEN_TRIM_UP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP2_CMP_IGEN_TRIM_UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0,
            true => ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp2_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Field `ACMP2_CMP_IGEN_TRIM_UP` writer - increases ACMP2 internal bias current by 30%"]
pub type ACMP2_CMP_IGEN_TRIM_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP2_CMP_IGEN_TRIM_UP_A, O>;
impl<'a, const O: u8> ACMP2_CMP_IGEN_TRIM_UP_W<'a, O> {
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP2_CMP_IGEN_TRIM_UP_A::ACMP2_CMP_IGEN_TRIM_UP_1)
    }
}
#[doc = "Field `ACMP3_CMP_IGEN_TRIM_UP` reader - increases ACMP3 internal bias current by 30%"]
pub type ACMP3_CMP_IGEN_TRIM_UP_R = crate::BitReader<ACMP3_CMP_IGEN_TRIM_UP_A>;
#[doc = "increases ACMP3 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP3_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP3_CMP_IGEN_TRIM_UP_0 = 0,
    #[doc = "1: increases"]
    ACMP3_CMP_IGEN_TRIM_UP_1 = 1,
}
impl From<ACMP3_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_CMP_IGEN_TRIM_UP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP3_CMP_IGEN_TRIM_UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0,
            true => ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp3_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Field `ACMP3_CMP_IGEN_TRIM_UP` writer - increases ACMP3 internal bias current by 30%"]
pub type ACMP3_CMP_IGEN_TRIM_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP3_CMP_IGEN_TRIM_UP_A, O>;
impl<'a, const O: u8> ACMP3_CMP_IGEN_TRIM_UP_W<'a, O> {
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP3_CMP_IGEN_TRIM_UP_A::ACMP3_CMP_IGEN_TRIM_UP_1)
    }
}
#[doc = "Field `ACMP4_CMP_IGEN_TRIM_UP` reader - increases ACMP4 internal bias current by 30%"]
pub type ACMP4_CMP_IGEN_TRIM_UP_R = crate::BitReader<ACMP4_CMP_IGEN_TRIM_UP_A>;
#[doc = "increases ACMP4 internal bias current by 30%\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP4_CMP_IGEN_TRIM_UP_A {
    #[doc = "0: no increase"]
    ACMP4_CMP_IGEN_TRIM_UP_0 = 0,
    #[doc = "1: increases"]
    ACMP4_CMP_IGEN_TRIM_UP_1 = 1,
}
impl From<ACMP4_CMP_IGEN_TRIM_UP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_CMP_IGEN_TRIM_UP_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP4_CMP_IGEN_TRIM_UP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_CMP_IGEN_TRIM_UP_A {
        match self.bits {
            false => ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0,
            true => ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_0`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_up_0(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_CMP_IGEN_TRIM_UP_1`"]
    #[inline(always)]
    pub fn is_acmp4_cmp_igen_trim_up_1(&self) -> bool {
        *self == ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1
    }
}
#[doc = "Field `ACMP4_CMP_IGEN_TRIM_UP` writer - increases ACMP4 internal bias current by 30%"]
pub type ACMP4_CMP_IGEN_TRIM_UP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP4_CMP_IGEN_TRIM_UP_A, O>;
impl<'a, const O: u8> ACMP4_CMP_IGEN_TRIM_UP_W<'a, O> {
    #[doc = "no increase"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up_0(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_0)
    }
    #[doc = "increases"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up_1(self) -> &'a mut W {
        self.variant(ACMP4_CMP_IGEN_TRIM_UP_A::ACMP4_CMP_IGEN_TRIM_UP_1)
    }
}
#[doc = "Field `ACMP1_SAMPLE_SYNC_EN` reader - ACMP1 sample_lv source select"]
pub type ACMP1_SAMPLE_SYNC_EN_R = crate::BitReader<ACMP1_SAMPLE_SYNC_EN_A>;
#[doc = "ACMP1 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP1_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP1_SAMPLE_SYNC_EN_0 = 0,
    #[doc = "1: select synced sample_lv"]
    ACMP1_SAMPLE_SYNC_EN_1 = 1,
}
impl From<ACMP1_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP1_SAMPLE_SYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP1_SAMPLE_SYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP1_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0,
            true => ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp1_sample_sync_en_0(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP1_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp1_sample_sync_en_1(&self) -> bool {
        *self == ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Field `ACMP1_SAMPLE_SYNC_EN` writer - ACMP1 sample_lv source select"]
pub type ACMP1_SAMPLE_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP1_SAMPLE_SYNC_EN_A, O>;
impl<'a, const O: u8> ACMP1_SAMPLE_SYNC_EN_W<'a, O> {
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP1_SAMPLE_SYNC_EN_A::ACMP1_SAMPLE_SYNC_EN_1)
    }
}
#[doc = "Field `ACMP2_SAMPLE_SYNC_EN` reader - ACMP2 sample_lv source select"]
pub type ACMP2_SAMPLE_SYNC_EN_R = crate::BitReader<ACMP2_SAMPLE_SYNC_EN_A>;
#[doc = "ACMP2 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP2_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP2_SAMPLE_SYNC_EN_0 = 0,
    #[doc = "1: select synced sample_lv"]
    ACMP2_SAMPLE_SYNC_EN_1 = 1,
}
impl From<ACMP2_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP2_SAMPLE_SYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP2_SAMPLE_SYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP2_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0,
            true => ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp2_sample_sync_en_0(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP2_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp2_sample_sync_en_1(&self) -> bool {
        *self == ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Field `ACMP2_SAMPLE_SYNC_EN` writer - ACMP2 sample_lv source select"]
pub type ACMP2_SAMPLE_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP2_SAMPLE_SYNC_EN_A, O>;
impl<'a, const O: u8> ACMP2_SAMPLE_SYNC_EN_W<'a, O> {
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP2_SAMPLE_SYNC_EN_A::ACMP2_SAMPLE_SYNC_EN_1)
    }
}
#[doc = "Field `ACMP3_SAMPLE_SYNC_EN` reader - ACMP3 sample_lv source select"]
pub type ACMP3_SAMPLE_SYNC_EN_R = crate::BitReader<ACMP3_SAMPLE_SYNC_EN_A>;
#[doc = "ACMP3 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP3_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP3_SAMPLE_SYNC_EN_0 = 0,
    #[doc = "1: select synced sample_lv"]
    ACMP3_SAMPLE_SYNC_EN_1 = 1,
}
impl From<ACMP3_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP3_SAMPLE_SYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP3_SAMPLE_SYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP3_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0,
            true => ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp3_sample_sync_en_0(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP3_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp3_sample_sync_en_1(&self) -> bool {
        *self == ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Field `ACMP3_SAMPLE_SYNC_EN` writer - ACMP3 sample_lv source select"]
pub type ACMP3_SAMPLE_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP3_SAMPLE_SYNC_EN_A, O>;
impl<'a, const O: u8> ACMP3_SAMPLE_SYNC_EN_W<'a, O> {
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP3_SAMPLE_SYNC_EN_A::ACMP3_SAMPLE_SYNC_EN_1)
    }
}
#[doc = "Field `ACMP4_SAMPLE_SYNC_EN` reader - ACMP4 sample_lv source select"]
pub type ACMP4_SAMPLE_SYNC_EN_R = crate::BitReader<ACMP4_SAMPLE_SYNC_EN_A>;
#[doc = "ACMP4 sample_lv source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMP4_SAMPLE_SYNC_EN_A {
    #[doc = "0: select XBAR output"]
    ACMP4_SAMPLE_SYNC_EN_0 = 0,
    #[doc = "1: select synced sample_lv"]
    ACMP4_SAMPLE_SYNC_EN_1 = 1,
}
impl From<ACMP4_SAMPLE_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP4_SAMPLE_SYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMP4_SAMPLE_SYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP4_SAMPLE_SYNC_EN_A {
        match self.bits {
            false => ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0,
            true => ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_0`"]
    #[inline(always)]
    pub fn is_acmp4_sample_sync_en_0(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMP4_SAMPLE_SYNC_EN_1`"]
    #[inline(always)]
    pub fn is_acmp4_sample_sync_en_1(&self) -> bool {
        *self == ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1
    }
}
#[doc = "Field `ACMP4_SAMPLE_SYNC_EN` writer - ACMP4 sample_lv source select"]
pub type ACMP4_SAMPLE_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR14_SPEC, ACMP4_SAMPLE_SYNC_EN_A, O>;
impl<'a, const O: u8> ACMP4_SAMPLE_SYNC_EN_W<'a, O> {
    #[doc = "select XBAR output"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en_0(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_0)
    }
    #[doc = "select synced sample_lv"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en_1(self) -> &'a mut W {
        self.variant(ACMP4_SAMPLE_SYNC_EN_A::ACMP4_SAMPLE_SYNC_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_dn(&self) -> ACMP1_CMP_IGEN_TRIM_DN_R {
        ACMP1_CMP_IGEN_TRIM_DN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_dn(&self) -> ACMP2_CMP_IGEN_TRIM_DN_R {
        ACMP2_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_dn(&self) -> ACMP3_CMP_IGEN_TRIM_DN_R {
        ACMP3_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_dn(&self) -> ACMP4_CMP_IGEN_TRIM_DN_R {
        ACMP4_CMP_IGEN_TRIM_DN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp1_cmp_igen_trim_up(&self) -> ACMP1_CMP_IGEN_TRIM_UP_R {
        ACMP1_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp2_cmp_igen_trim_up(&self) -> ACMP2_CMP_IGEN_TRIM_UP_R {
        ACMP2_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp3_cmp_igen_trim_up(&self) -> ACMP3_CMP_IGEN_TRIM_UP_R {
        ACMP3_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub fn acmp4_cmp_igen_trim_up(&self) -> ACMP4_CMP_IGEN_TRIM_UP_R {
        ACMP4_CMP_IGEN_TRIM_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline(always)]
    pub fn acmp1_sample_sync_en(&self) -> ACMP1_SAMPLE_SYNC_EN_R {
        ACMP1_SAMPLE_SYNC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline(always)]
    pub fn acmp2_sample_sync_en(&self) -> ACMP2_SAMPLE_SYNC_EN_R {
        ACMP2_SAMPLE_SYNC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline(always)]
    pub fn acmp3_sample_sync_en(&self) -> ACMP3_SAMPLE_SYNC_EN_R {
        ACMP3_SAMPLE_SYNC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline(always)]
    pub fn acmp4_sample_sync_en(&self) -> ACMP4_SAMPLE_SYNC_EN_R {
        ACMP4_SAMPLE_SYNC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reduces ACMP1 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1_cmp_igen_trim_dn(&mut self) -> ACMP1_CMP_IGEN_TRIM_DN_W<0> {
        ACMP1_CMP_IGEN_TRIM_DN_W::new(self)
    }
    #[doc = "Bit 1 - reduces ACMP2 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2_cmp_igen_trim_dn(&mut self) -> ACMP2_CMP_IGEN_TRIM_DN_W<1> {
        ACMP2_CMP_IGEN_TRIM_DN_W::new(self)
    }
    #[doc = "Bit 2 - reduces ACMP3 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3_cmp_igen_trim_dn(&mut self) -> ACMP3_CMP_IGEN_TRIM_DN_W<2> {
        ACMP3_CMP_IGEN_TRIM_DN_W::new(self)
    }
    #[doc = "Bit 3 - reduces ACMP4 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp4_cmp_igen_trim_dn(&mut self) -> ACMP4_CMP_IGEN_TRIM_DN_W<3> {
        ACMP4_CMP_IGEN_TRIM_DN_W::new(self)
    }
    #[doc = "Bit 4 - increases ACMP1 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1_cmp_igen_trim_up(&mut self) -> ACMP1_CMP_IGEN_TRIM_UP_W<4> {
        ACMP1_CMP_IGEN_TRIM_UP_W::new(self)
    }
    #[doc = "Bit 5 - increases ACMP2 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2_cmp_igen_trim_up(&mut self) -> ACMP2_CMP_IGEN_TRIM_UP_W<5> {
        ACMP2_CMP_IGEN_TRIM_UP_W::new(self)
    }
    #[doc = "Bit 6 - increases ACMP3 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3_cmp_igen_trim_up(&mut self) -> ACMP3_CMP_IGEN_TRIM_UP_W<6> {
        ACMP3_CMP_IGEN_TRIM_UP_W::new(self)
    }
    #[doc = "Bit 7 - increases ACMP4 internal bias current by 30%"]
    #[inline(always)]
    #[must_use]
    pub fn acmp4_cmp_igen_trim_up(&mut self) -> ACMP4_CMP_IGEN_TRIM_UP_W<7> {
        ACMP4_CMP_IGEN_TRIM_UP_W::new(self)
    }
    #[doc = "Bit 8 - ACMP1 sample_lv source select"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1_sample_sync_en(&mut self) -> ACMP1_SAMPLE_SYNC_EN_W<8> {
        ACMP1_SAMPLE_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 9 - ACMP2 sample_lv source select"]
    #[inline(always)]
    #[must_use]
    pub fn acmp2_sample_sync_en(&mut self) -> ACMP2_SAMPLE_SYNC_EN_W<9> {
        ACMP2_SAMPLE_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 10 - ACMP3 sample_lv source select"]
    #[inline(always)]
    #[must_use]
    pub fn acmp3_sample_sync_en(&mut self) -> ACMP3_SAMPLE_SYNC_EN_W<10> {
        ACMP3_SAMPLE_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 11 - ACMP4 sample_lv source select"]
    #[inline(always)]
    #[must_use]
    pub fn acmp4_sample_sync_en(&mut self) -> ACMP4_SAMPLE_SYNC_EN_W<11> {
        ACMP4_SAMPLE_SYNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR14 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr14](index.html) module"]
pub struct GPR14_SPEC;
impl crate::RegisterSpec for GPR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr14::R](R) reader structure"]
impl crate::Readable for GPR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr14::W](W) writer structure"]
impl crate::Writable for GPR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR14 to value 0"]
impl crate::Resettable for GPR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
