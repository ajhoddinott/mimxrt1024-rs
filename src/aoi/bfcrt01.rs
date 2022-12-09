#[doc = "Register `BFCRT01%s` reader"]
pub struct R(crate::R<BFCRT01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFCRT01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFCRT01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFCRT01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFCRT01%s` writer"]
pub struct W(crate::W<BFCRT01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFCRT01_SPEC>;
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
impl From<crate::W<BFCRT01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFCRT01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT1_DC` reader - Product term 1, D input configuration"]
pub type PT1_DC_R = crate::FieldReader<u8, PT1_DC_A>;
#[doc = "Product term 1, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT1_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT1_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT1_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT1_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT1_DC_3 = 3,
}
impl From<PT1_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_DC_A) -> Self {
        variant as _
    }
}
impl PT1_DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_DC_A {
        match self.bits {
            0 => PT1_DC_A::PT1_DC_0,
            1 => PT1_DC_A::PT1_DC_1,
            2 => PT1_DC_A::PT1_DC_2,
            3 => PT1_DC_A::PT1_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_DC_0`"]
    #[inline(always)]
    pub fn is_pt1_dc_0(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_0
    }
    #[doc = "Checks if the value of the field is `PT1_DC_1`"]
    #[inline(always)]
    pub fn is_pt1_dc_1(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_1
    }
    #[doc = "Checks if the value of the field is `PT1_DC_2`"]
    #[inline(always)]
    pub fn is_pt1_dc_2(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_2
    }
    #[doc = "Checks if the value of the field is `PT1_DC_3`"]
    #[inline(always)]
    pub fn is_pt1_dc_3(&self) -> bool {
        *self == PT1_DC_A::PT1_DC_3
    }
}
#[doc = "Field `PT1_DC` writer - Product term 1, D input configuration"]
pub type PT1_DC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT1_DC_A, 2, O>;
impl<'a, const O: u8> PT1_DC_W<'a, O> {
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_dc_0(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt1_dc_1(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt1_dc_2(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_dc_3(self) -> &'a mut W {
        self.variant(PT1_DC_A::PT1_DC_3)
    }
}
#[doc = "Field `PT1_CC` reader - Product term 1, C input configuration"]
pub type PT1_CC_R = crate::FieldReader<u8, PT1_CC_A>;
#[doc = "Product term 1, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT1_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT1_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT1_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT1_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT1_CC_3 = 3,
}
impl From<PT1_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_CC_A) -> Self {
        variant as _
    }
}
impl PT1_CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_CC_A {
        match self.bits {
            0 => PT1_CC_A::PT1_CC_0,
            1 => PT1_CC_A::PT1_CC_1,
            2 => PT1_CC_A::PT1_CC_2,
            3 => PT1_CC_A::PT1_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_CC_0`"]
    #[inline(always)]
    pub fn is_pt1_cc_0(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_0
    }
    #[doc = "Checks if the value of the field is `PT1_CC_1`"]
    #[inline(always)]
    pub fn is_pt1_cc_1(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_1
    }
    #[doc = "Checks if the value of the field is `PT1_CC_2`"]
    #[inline(always)]
    pub fn is_pt1_cc_2(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_2
    }
    #[doc = "Checks if the value of the field is `PT1_CC_3`"]
    #[inline(always)]
    pub fn is_pt1_cc_3(&self) -> bool {
        *self == PT1_CC_A::PT1_CC_3
    }
}
#[doc = "Field `PT1_CC` writer - Product term 1, C input configuration"]
pub type PT1_CC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT1_CC_A, 2, O>;
impl<'a, const O: u8> PT1_CC_W<'a, O> {
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_cc_0(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt1_cc_1(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt1_cc_2(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_cc_3(self) -> &'a mut W {
        self.variant(PT1_CC_A::PT1_CC_3)
    }
}
#[doc = "Field `PT1_BC` reader - Product term 1, B input configuration"]
pub type PT1_BC_R = crate::FieldReader<u8, PT1_BC_A>;
#[doc = "Product term 1, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT1_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT1_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT1_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT1_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT1_BC_3 = 3,
}
impl From<PT1_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_BC_A) -> Self {
        variant as _
    }
}
impl PT1_BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_BC_A {
        match self.bits {
            0 => PT1_BC_A::PT1_BC_0,
            1 => PT1_BC_A::PT1_BC_1,
            2 => PT1_BC_A::PT1_BC_2,
            3 => PT1_BC_A::PT1_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_BC_0`"]
    #[inline(always)]
    pub fn is_pt1_bc_0(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_0
    }
    #[doc = "Checks if the value of the field is `PT1_BC_1`"]
    #[inline(always)]
    pub fn is_pt1_bc_1(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_1
    }
    #[doc = "Checks if the value of the field is `PT1_BC_2`"]
    #[inline(always)]
    pub fn is_pt1_bc_2(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_2
    }
    #[doc = "Checks if the value of the field is `PT1_BC_3`"]
    #[inline(always)]
    pub fn is_pt1_bc_3(&self) -> bool {
        *self == PT1_BC_A::PT1_BC_3
    }
}
#[doc = "Field `PT1_BC` writer - Product term 1, B input configuration"]
pub type PT1_BC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT1_BC_A, 2, O>;
impl<'a, const O: u8> PT1_BC_W<'a, O> {
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_bc_0(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt1_bc_1(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt1_bc_2(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_bc_3(self) -> &'a mut W {
        self.variant(PT1_BC_A::PT1_BC_3)
    }
}
#[doc = "Field `PT1_AC` reader - Product term 1, A input configuration"]
pub type PT1_AC_R = crate::FieldReader<u8, PT1_AC_A>;
#[doc = "Product term 1, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT1_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT1_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT1_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT1_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT1_AC_3 = 3,
}
impl From<PT1_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT1_AC_A) -> Self {
        variant as _
    }
}
impl PT1_AC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT1_AC_A {
        match self.bits {
            0 => PT1_AC_A::PT1_AC_0,
            1 => PT1_AC_A::PT1_AC_1,
            2 => PT1_AC_A::PT1_AC_2,
            3 => PT1_AC_A::PT1_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT1_AC_0`"]
    #[inline(always)]
    pub fn is_pt1_ac_0(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_0
    }
    #[doc = "Checks if the value of the field is `PT1_AC_1`"]
    #[inline(always)]
    pub fn is_pt1_ac_1(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_1
    }
    #[doc = "Checks if the value of the field is `PT1_AC_2`"]
    #[inline(always)]
    pub fn is_pt1_ac_2(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_2
    }
    #[doc = "Checks if the value of the field is `PT1_AC_3`"]
    #[inline(always)]
    pub fn is_pt1_ac_3(&self) -> bool {
        *self == PT1_AC_A::PT1_AC_3
    }
}
#[doc = "Field `PT1_AC` writer - Product term 1, A input configuration"]
pub type PT1_AC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT1_AC_A, 2, O>;
impl<'a, const O: u8> PT1_AC_W<'a, O> {
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt1_ac_0(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt1_ac_1(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt1_ac_2(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt1_ac_3(self) -> &'a mut W {
        self.variant(PT1_AC_A::PT1_AC_3)
    }
}
#[doc = "Field `PT0_DC` reader - Product term 0, D input configuration"]
pub type PT0_DC_R = crate::FieldReader<u8, PT0_DC_A>;
#[doc = "Product term 0, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT0_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT0_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT0_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT0_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT0_DC_3 = 3,
}
impl From<PT0_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_DC_A) -> Self {
        variant as _
    }
}
impl PT0_DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_DC_A {
        match self.bits {
            0 => PT0_DC_A::PT0_DC_0,
            1 => PT0_DC_A::PT0_DC_1,
            2 => PT0_DC_A::PT0_DC_2,
            3 => PT0_DC_A::PT0_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_DC_0`"]
    #[inline(always)]
    pub fn is_pt0_dc_0(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_0
    }
    #[doc = "Checks if the value of the field is `PT0_DC_1`"]
    #[inline(always)]
    pub fn is_pt0_dc_1(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_1
    }
    #[doc = "Checks if the value of the field is `PT0_DC_2`"]
    #[inline(always)]
    pub fn is_pt0_dc_2(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_2
    }
    #[doc = "Checks if the value of the field is `PT0_DC_3`"]
    #[inline(always)]
    pub fn is_pt0_dc_3(&self) -> bool {
        *self == PT0_DC_A::PT0_DC_3
    }
}
#[doc = "Field `PT0_DC` writer - Product term 0, D input configuration"]
pub type PT0_DC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT0_DC_A, 2, O>;
impl<'a, const O: u8> PT0_DC_W<'a, O> {
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_dc_0(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt0_dc_1(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt0_dc_2(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_dc_3(self) -> &'a mut W {
        self.variant(PT0_DC_A::PT0_DC_3)
    }
}
#[doc = "Field `PT0_CC` reader - Product term 0, C input configuration"]
pub type PT0_CC_R = crate::FieldReader<u8, PT0_CC_A>;
#[doc = "Product term 0, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT0_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT0_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT0_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT0_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT0_CC_3 = 3,
}
impl From<PT0_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_CC_A) -> Self {
        variant as _
    }
}
impl PT0_CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_CC_A {
        match self.bits {
            0 => PT0_CC_A::PT0_CC_0,
            1 => PT0_CC_A::PT0_CC_1,
            2 => PT0_CC_A::PT0_CC_2,
            3 => PT0_CC_A::PT0_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_CC_0`"]
    #[inline(always)]
    pub fn is_pt0_cc_0(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_0
    }
    #[doc = "Checks if the value of the field is `PT0_CC_1`"]
    #[inline(always)]
    pub fn is_pt0_cc_1(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_1
    }
    #[doc = "Checks if the value of the field is `PT0_CC_2`"]
    #[inline(always)]
    pub fn is_pt0_cc_2(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_2
    }
    #[doc = "Checks if the value of the field is `PT0_CC_3`"]
    #[inline(always)]
    pub fn is_pt0_cc_3(&self) -> bool {
        *self == PT0_CC_A::PT0_CC_3
    }
}
#[doc = "Field `PT0_CC` writer - Product term 0, C input configuration"]
pub type PT0_CC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT0_CC_A, 2, O>;
impl<'a, const O: u8> PT0_CC_W<'a, O> {
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_cc_0(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt0_cc_1(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt0_cc_2(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_cc_3(self) -> &'a mut W {
        self.variant(PT0_CC_A::PT0_CC_3)
    }
}
#[doc = "Field `PT0_BC` reader - Product term 0, B input configuration"]
pub type PT0_BC_R = crate::FieldReader<u8, PT0_BC_A>;
#[doc = "Product term 0, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT0_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT0_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT0_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT0_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT0_BC_3 = 3,
}
impl From<PT0_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_BC_A) -> Self {
        variant as _
    }
}
impl PT0_BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_BC_A {
        match self.bits {
            0 => PT0_BC_A::PT0_BC_0,
            1 => PT0_BC_A::PT0_BC_1,
            2 => PT0_BC_A::PT0_BC_2,
            3 => PT0_BC_A::PT0_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_BC_0`"]
    #[inline(always)]
    pub fn is_pt0_bc_0(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_0
    }
    #[doc = "Checks if the value of the field is `PT0_BC_1`"]
    #[inline(always)]
    pub fn is_pt0_bc_1(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_1
    }
    #[doc = "Checks if the value of the field is `PT0_BC_2`"]
    #[inline(always)]
    pub fn is_pt0_bc_2(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_2
    }
    #[doc = "Checks if the value of the field is `PT0_BC_3`"]
    #[inline(always)]
    pub fn is_pt0_bc_3(&self) -> bool {
        *self == PT0_BC_A::PT0_BC_3
    }
}
#[doc = "Field `PT0_BC` writer - Product term 0, B input configuration"]
pub type PT0_BC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT0_BC_A, 2, O>;
impl<'a, const O: u8> PT0_BC_W<'a, O> {
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_bc_0(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt0_bc_1(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt0_bc_2(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_bc_3(self) -> &'a mut W {
        self.variant(PT0_BC_A::PT0_BC_3)
    }
}
#[doc = "Field `PT0_AC` reader - Product term 0, A input configuration"]
pub type PT0_AC_R = crate::FieldReader<u8, PT0_AC_A>;
#[doc = "Product term 0, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT0_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT0_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT0_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT0_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT0_AC_3 = 3,
}
impl From<PT0_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT0_AC_A) -> Self {
        variant as _
    }
}
impl PT0_AC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT0_AC_A {
        match self.bits {
            0 => PT0_AC_A::PT0_AC_0,
            1 => PT0_AC_A::PT0_AC_1,
            2 => PT0_AC_A::PT0_AC_2,
            3 => PT0_AC_A::PT0_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT0_AC_0`"]
    #[inline(always)]
    pub fn is_pt0_ac_0(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_0
    }
    #[doc = "Checks if the value of the field is `PT0_AC_1`"]
    #[inline(always)]
    pub fn is_pt0_ac_1(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_1
    }
    #[doc = "Checks if the value of the field is `PT0_AC_2`"]
    #[inline(always)]
    pub fn is_pt0_ac_2(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_2
    }
    #[doc = "Checks if the value of the field is `PT0_AC_3`"]
    #[inline(always)]
    pub fn is_pt0_ac_3(&self) -> bool {
        *self == PT0_AC_A::PT0_AC_3
    }
}
#[doc = "Field `PT0_AC` writer - Product term 0, A input configuration"]
pub type PT0_AC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT01_SPEC, u8, PT0_AC_A, 2, O>;
impl<'a, const O: u8> PT0_AC_W<'a, O> {
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt0_ac_0(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt0_ac_1(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt0_ac_2(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt0_ac_3(self) -> &'a mut W {
        self.variant(PT0_AC_A::PT0_AC_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline(always)]
    pub fn pt1_dc(&self) -> PT1_DC_R {
        PT1_DC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline(always)]
    pub fn pt1_cc(&self) -> PT1_CC_R {
        PT1_CC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline(always)]
    pub fn pt1_bc(&self) -> PT1_BC_R {
        PT1_BC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline(always)]
    pub fn pt1_ac(&self) -> PT1_AC_R {
        PT1_AC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline(always)]
    pub fn pt0_dc(&self) -> PT0_DC_R {
        PT0_DC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline(always)]
    pub fn pt0_cc(&self) -> PT0_CC_R {
        PT0_CC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline(always)]
    pub fn pt0_bc(&self) -> PT0_BC_R {
        PT0_BC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline(always)]
    pub fn pt0_ac(&self) -> PT0_AC_R {
        PT0_AC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Product term 1, D input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt1_dc(&mut self) -> PT1_DC_W<0> {
        PT1_DC_W::new(self)
    }
    #[doc = "Bits 2:3 - Product term 1, C input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt1_cc(&mut self) -> PT1_CC_W<2> {
        PT1_CC_W::new(self)
    }
    #[doc = "Bits 4:5 - Product term 1, B input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt1_bc(&mut self) -> PT1_BC_W<4> {
        PT1_BC_W::new(self)
    }
    #[doc = "Bits 6:7 - Product term 1, A input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt1_ac(&mut self) -> PT1_AC_W<6> {
        PT1_AC_W::new(self)
    }
    #[doc = "Bits 8:9 - Product term 0, D input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt0_dc(&mut self) -> PT0_DC_W<8> {
        PT0_DC_W::new(self)
    }
    #[doc = "Bits 10:11 - Product term 0, C input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt0_cc(&mut self) -> PT0_CC_W<10> {
        PT0_CC_W::new(self)
    }
    #[doc = "Bits 12:13 - Product term 0, B input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt0_bc(&mut self) -> PT0_BC_W<12> {
        PT0_BC_W::new(self)
    }
    #[doc = "Bits 14:15 - Product term 0, A input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt0_ac(&mut self) -> PT0_AC_W<14> {
        PT0_AC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration Register for EVENTn\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcrt01](index.html) module"]
pub struct BFCRT01_SPEC;
impl crate::RegisterSpec for BFCRT01_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bfcrt01::R](R) reader structure"]
impl crate::Readable for BFCRT01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfcrt01::W](W) writer structure"]
impl crate::Writable for BFCRT01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFCRT01%s to value 0"]
impl crate::Resettable for BFCRT01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
