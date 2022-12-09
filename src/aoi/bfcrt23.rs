#[doc = "Register `BFCRT23%s` reader"]
pub struct R(crate::R<BFCRT23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFCRT23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFCRT23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFCRT23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFCRT23%s` writer"]
pub struct W(crate::W<BFCRT23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFCRT23_SPEC>;
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
impl From<crate::W<BFCRT23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFCRT23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT3_DC` reader - Product term 3, D input configuration"]
pub type PT3_DC_R = crate::FieldReader<u8, PT3_DC_A>;
#[doc = "Product term 3, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT3_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT3_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT3_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT3_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT3_DC_3 = 3,
}
impl From<PT3_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_DC_A) -> Self {
        variant as _
    }
}
impl PT3_DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_DC_A {
        match self.bits {
            0 => PT3_DC_A::PT3_DC_0,
            1 => PT3_DC_A::PT3_DC_1,
            2 => PT3_DC_A::PT3_DC_2,
            3 => PT3_DC_A::PT3_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_DC_0`"]
    #[inline(always)]
    pub fn is_pt3_dc_0(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_0
    }
    #[doc = "Checks if the value of the field is `PT3_DC_1`"]
    #[inline(always)]
    pub fn is_pt3_dc_1(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_1
    }
    #[doc = "Checks if the value of the field is `PT3_DC_2`"]
    #[inline(always)]
    pub fn is_pt3_dc_2(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_2
    }
    #[doc = "Checks if the value of the field is `PT3_DC_3`"]
    #[inline(always)]
    pub fn is_pt3_dc_3(&self) -> bool {
        *self == PT3_DC_A::PT3_DC_3
    }
}
#[doc = "Field `PT3_DC` writer - Product term 3, D input configuration"]
pub type PT3_DC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT3_DC_A, 2, O>;
impl<'a, const O: u8> PT3_DC_W<'a, O> {
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_dc_0(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt3_dc_1(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt3_dc_2(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_dc_3(self) -> &'a mut W {
        self.variant(PT3_DC_A::PT3_DC_3)
    }
}
#[doc = "Field `PT3_CC` reader - Product term 3, C input configuration"]
pub type PT3_CC_R = crate::FieldReader<u8, PT3_CC_A>;
#[doc = "Product term 3, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT3_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT3_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT3_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT3_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT3_CC_3 = 3,
}
impl From<PT3_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_CC_A) -> Self {
        variant as _
    }
}
impl PT3_CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_CC_A {
        match self.bits {
            0 => PT3_CC_A::PT3_CC_0,
            1 => PT3_CC_A::PT3_CC_1,
            2 => PT3_CC_A::PT3_CC_2,
            3 => PT3_CC_A::PT3_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_CC_0`"]
    #[inline(always)]
    pub fn is_pt3_cc_0(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_0
    }
    #[doc = "Checks if the value of the field is `PT3_CC_1`"]
    #[inline(always)]
    pub fn is_pt3_cc_1(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_1
    }
    #[doc = "Checks if the value of the field is `PT3_CC_2`"]
    #[inline(always)]
    pub fn is_pt3_cc_2(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_2
    }
    #[doc = "Checks if the value of the field is `PT3_CC_3`"]
    #[inline(always)]
    pub fn is_pt3_cc_3(&self) -> bool {
        *self == PT3_CC_A::PT3_CC_3
    }
}
#[doc = "Field `PT3_CC` writer - Product term 3, C input configuration"]
pub type PT3_CC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT3_CC_A, 2, O>;
impl<'a, const O: u8> PT3_CC_W<'a, O> {
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_cc_0(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt3_cc_1(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt3_cc_2(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_cc_3(self) -> &'a mut W {
        self.variant(PT3_CC_A::PT3_CC_3)
    }
}
#[doc = "Field `PT3_BC` reader - Product term 3, B input configuration"]
pub type PT3_BC_R = crate::FieldReader<u8, PT3_BC_A>;
#[doc = "Product term 3, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT3_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT3_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT3_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT3_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT3_BC_3 = 3,
}
impl From<PT3_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_BC_A) -> Self {
        variant as _
    }
}
impl PT3_BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_BC_A {
        match self.bits {
            0 => PT3_BC_A::PT3_BC_0,
            1 => PT3_BC_A::PT3_BC_1,
            2 => PT3_BC_A::PT3_BC_2,
            3 => PT3_BC_A::PT3_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_BC_0`"]
    #[inline(always)]
    pub fn is_pt3_bc_0(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_0
    }
    #[doc = "Checks if the value of the field is `PT3_BC_1`"]
    #[inline(always)]
    pub fn is_pt3_bc_1(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_1
    }
    #[doc = "Checks if the value of the field is `PT3_BC_2`"]
    #[inline(always)]
    pub fn is_pt3_bc_2(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_2
    }
    #[doc = "Checks if the value of the field is `PT3_BC_3`"]
    #[inline(always)]
    pub fn is_pt3_bc_3(&self) -> bool {
        *self == PT3_BC_A::PT3_BC_3
    }
}
#[doc = "Field `PT3_BC` writer - Product term 3, B input configuration"]
pub type PT3_BC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT3_BC_A, 2, O>;
impl<'a, const O: u8> PT3_BC_W<'a, O> {
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_bc_0(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt3_bc_1(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt3_bc_2(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_bc_3(self) -> &'a mut W {
        self.variant(PT3_BC_A::PT3_BC_3)
    }
}
#[doc = "Field `PT3_AC` reader - Product term 3, A input configuration"]
pub type PT3_AC_R = crate::FieldReader<u8, PT3_AC_A>;
#[doc = "Product term 3, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT3_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT3_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT3_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT3_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT3_AC_3 = 3,
}
impl From<PT3_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT3_AC_A) -> Self {
        variant as _
    }
}
impl PT3_AC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT3_AC_A {
        match self.bits {
            0 => PT3_AC_A::PT3_AC_0,
            1 => PT3_AC_A::PT3_AC_1,
            2 => PT3_AC_A::PT3_AC_2,
            3 => PT3_AC_A::PT3_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT3_AC_0`"]
    #[inline(always)]
    pub fn is_pt3_ac_0(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_0
    }
    #[doc = "Checks if the value of the field is `PT3_AC_1`"]
    #[inline(always)]
    pub fn is_pt3_ac_1(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_1
    }
    #[doc = "Checks if the value of the field is `PT3_AC_2`"]
    #[inline(always)]
    pub fn is_pt3_ac_2(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_2
    }
    #[doc = "Checks if the value of the field is `PT3_AC_3`"]
    #[inline(always)]
    pub fn is_pt3_ac_3(&self) -> bool {
        *self == PT3_AC_A::PT3_AC_3
    }
}
#[doc = "Field `PT3_AC` writer - Product term 3, A input configuration"]
pub type PT3_AC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT3_AC_A, 2, O>;
impl<'a, const O: u8> PT3_AC_W<'a, O> {
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt3_ac_0(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt3_ac_1(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt3_ac_2(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt3_ac_3(self) -> &'a mut W {
        self.variant(PT3_AC_A::PT3_AC_3)
    }
}
#[doc = "Field `PT2_DC` reader - Product term 2, D input configuration"]
pub type PT2_DC_R = crate::FieldReader<u8, PT2_DC_A>;
#[doc = "Product term 2, D input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT2_DC_A {
    #[doc = "0: Force the D input in this product term to a logical zero"]
    PT2_DC_0 = 0,
    #[doc = "1: Pass the D input in this product term"]
    PT2_DC_1 = 1,
    #[doc = "2: Complement the D input in this product term"]
    PT2_DC_2 = 2,
    #[doc = "3: Force the D input in this product term to a logical one"]
    PT2_DC_3 = 3,
}
impl From<PT2_DC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_DC_A) -> Self {
        variant as _
    }
}
impl PT2_DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_DC_A {
        match self.bits {
            0 => PT2_DC_A::PT2_DC_0,
            1 => PT2_DC_A::PT2_DC_1,
            2 => PT2_DC_A::PT2_DC_2,
            3 => PT2_DC_A::PT2_DC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_DC_0`"]
    #[inline(always)]
    pub fn is_pt2_dc_0(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_0
    }
    #[doc = "Checks if the value of the field is `PT2_DC_1`"]
    #[inline(always)]
    pub fn is_pt2_dc_1(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_1
    }
    #[doc = "Checks if the value of the field is `PT2_DC_2`"]
    #[inline(always)]
    pub fn is_pt2_dc_2(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_2
    }
    #[doc = "Checks if the value of the field is `PT2_DC_3`"]
    #[inline(always)]
    pub fn is_pt2_dc_3(&self) -> bool {
        *self == PT2_DC_A::PT2_DC_3
    }
}
#[doc = "Field `PT2_DC` writer - Product term 2, D input configuration"]
pub type PT2_DC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT2_DC_A, 2, O>;
impl<'a, const O: u8> PT2_DC_W<'a, O> {
    #[doc = "Force the D input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_dc_0(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_0)
    }
    #[doc = "Pass the D input in this product term"]
    #[inline(always)]
    pub fn pt2_dc_1(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_1)
    }
    #[doc = "Complement the D input in this product term"]
    #[inline(always)]
    pub fn pt2_dc_2(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_2)
    }
    #[doc = "Force the D input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_dc_3(self) -> &'a mut W {
        self.variant(PT2_DC_A::PT2_DC_3)
    }
}
#[doc = "Field `PT2_CC` reader - Product term 2, C input configuration"]
pub type PT2_CC_R = crate::FieldReader<u8, PT2_CC_A>;
#[doc = "Product term 2, C input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT2_CC_A {
    #[doc = "0: Force the C input in this product term to a logical zero"]
    PT2_CC_0 = 0,
    #[doc = "1: Pass the C input in this product term"]
    PT2_CC_1 = 1,
    #[doc = "2: Complement the C input in this product term"]
    PT2_CC_2 = 2,
    #[doc = "3: Force the C input in this product term to a logical one"]
    PT2_CC_3 = 3,
}
impl From<PT2_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_CC_A) -> Self {
        variant as _
    }
}
impl PT2_CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_CC_A {
        match self.bits {
            0 => PT2_CC_A::PT2_CC_0,
            1 => PT2_CC_A::PT2_CC_1,
            2 => PT2_CC_A::PT2_CC_2,
            3 => PT2_CC_A::PT2_CC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_CC_0`"]
    #[inline(always)]
    pub fn is_pt2_cc_0(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_0
    }
    #[doc = "Checks if the value of the field is `PT2_CC_1`"]
    #[inline(always)]
    pub fn is_pt2_cc_1(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_1
    }
    #[doc = "Checks if the value of the field is `PT2_CC_2`"]
    #[inline(always)]
    pub fn is_pt2_cc_2(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_2
    }
    #[doc = "Checks if the value of the field is `PT2_CC_3`"]
    #[inline(always)]
    pub fn is_pt2_cc_3(&self) -> bool {
        *self == PT2_CC_A::PT2_CC_3
    }
}
#[doc = "Field `PT2_CC` writer - Product term 2, C input configuration"]
pub type PT2_CC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT2_CC_A, 2, O>;
impl<'a, const O: u8> PT2_CC_W<'a, O> {
    #[doc = "Force the C input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_cc_0(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_0)
    }
    #[doc = "Pass the C input in this product term"]
    #[inline(always)]
    pub fn pt2_cc_1(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_1)
    }
    #[doc = "Complement the C input in this product term"]
    #[inline(always)]
    pub fn pt2_cc_2(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_2)
    }
    #[doc = "Force the C input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_cc_3(self) -> &'a mut W {
        self.variant(PT2_CC_A::PT2_CC_3)
    }
}
#[doc = "Field `PT2_BC` reader - Product term 2, B input configuration"]
pub type PT2_BC_R = crate::FieldReader<u8, PT2_BC_A>;
#[doc = "Product term 2, B input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT2_BC_A {
    #[doc = "0: Force the B input in this product term to a logical zero"]
    PT2_BC_0 = 0,
    #[doc = "1: Pass the B input in this product term"]
    PT2_BC_1 = 1,
    #[doc = "2: Complement the B input in this product term"]
    PT2_BC_2 = 2,
    #[doc = "3: Force the B input in this product term to a logical one"]
    PT2_BC_3 = 3,
}
impl From<PT2_BC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_BC_A) -> Self {
        variant as _
    }
}
impl PT2_BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_BC_A {
        match self.bits {
            0 => PT2_BC_A::PT2_BC_0,
            1 => PT2_BC_A::PT2_BC_1,
            2 => PT2_BC_A::PT2_BC_2,
            3 => PT2_BC_A::PT2_BC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_BC_0`"]
    #[inline(always)]
    pub fn is_pt2_bc_0(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_0
    }
    #[doc = "Checks if the value of the field is `PT2_BC_1`"]
    #[inline(always)]
    pub fn is_pt2_bc_1(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_1
    }
    #[doc = "Checks if the value of the field is `PT2_BC_2`"]
    #[inline(always)]
    pub fn is_pt2_bc_2(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_2
    }
    #[doc = "Checks if the value of the field is `PT2_BC_3`"]
    #[inline(always)]
    pub fn is_pt2_bc_3(&self) -> bool {
        *self == PT2_BC_A::PT2_BC_3
    }
}
#[doc = "Field `PT2_BC` writer - Product term 2, B input configuration"]
pub type PT2_BC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT2_BC_A, 2, O>;
impl<'a, const O: u8> PT2_BC_W<'a, O> {
    #[doc = "Force the B input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_bc_0(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_0)
    }
    #[doc = "Pass the B input in this product term"]
    #[inline(always)]
    pub fn pt2_bc_1(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_1)
    }
    #[doc = "Complement the B input in this product term"]
    #[inline(always)]
    pub fn pt2_bc_2(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_2)
    }
    #[doc = "Force the B input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_bc_3(self) -> &'a mut W {
        self.variant(PT2_BC_A::PT2_BC_3)
    }
}
#[doc = "Field `PT2_AC` reader - Product term 2, A input configuration"]
pub type PT2_AC_R = crate::FieldReader<u8, PT2_AC_A>;
#[doc = "Product term 2, A input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PT2_AC_A {
    #[doc = "0: Force the A input in this product term to a logical zero"]
    PT2_AC_0 = 0,
    #[doc = "1: Pass the A input in this product term"]
    PT2_AC_1 = 1,
    #[doc = "2: Complement the A input in this product term"]
    PT2_AC_2 = 2,
    #[doc = "3: Force the A input in this product term to a logical one"]
    PT2_AC_3 = 3,
}
impl From<PT2_AC_A> for u8 {
    #[inline(always)]
    fn from(variant: PT2_AC_A) -> Self {
        variant as _
    }
}
impl PT2_AC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT2_AC_A {
        match self.bits {
            0 => PT2_AC_A::PT2_AC_0,
            1 => PT2_AC_A::PT2_AC_1,
            2 => PT2_AC_A::PT2_AC_2,
            3 => PT2_AC_A::PT2_AC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PT2_AC_0`"]
    #[inline(always)]
    pub fn is_pt2_ac_0(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_0
    }
    #[doc = "Checks if the value of the field is `PT2_AC_1`"]
    #[inline(always)]
    pub fn is_pt2_ac_1(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_1
    }
    #[doc = "Checks if the value of the field is `PT2_AC_2`"]
    #[inline(always)]
    pub fn is_pt2_ac_2(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_2
    }
    #[doc = "Checks if the value of the field is `PT2_AC_3`"]
    #[inline(always)]
    pub fn is_pt2_ac_3(&self) -> bool {
        *self == PT2_AC_A::PT2_AC_3
    }
}
#[doc = "Field `PT2_AC` writer - Product term 2, A input configuration"]
pub type PT2_AC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, BFCRT23_SPEC, u8, PT2_AC_A, 2, O>;
impl<'a, const O: u8> PT2_AC_W<'a, O> {
    #[doc = "Force the A input in this product term to a logical zero"]
    #[inline(always)]
    pub fn pt2_ac_0(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_0)
    }
    #[doc = "Pass the A input in this product term"]
    #[inline(always)]
    pub fn pt2_ac_1(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_1)
    }
    #[doc = "Complement the A input in this product term"]
    #[inline(always)]
    pub fn pt2_ac_2(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_2)
    }
    #[doc = "Force the A input in this product term to a logical one"]
    #[inline(always)]
    pub fn pt2_ac_3(self) -> &'a mut W {
        self.variant(PT2_AC_A::PT2_AC_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline(always)]
    pub fn pt3_dc(&self) -> PT3_DC_R {
        PT3_DC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline(always)]
    pub fn pt3_cc(&self) -> PT3_CC_R {
        PT3_CC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline(always)]
    pub fn pt3_bc(&self) -> PT3_BC_R {
        PT3_BC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline(always)]
    pub fn pt3_ac(&self) -> PT3_AC_R {
        PT3_AC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline(always)]
    pub fn pt2_dc(&self) -> PT2_DC_R {
        PT2_DC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline(always)]
    pub fn pt2_cc(&self) -> PT2_CC_R {
        PT2_CC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline(always)]
    pub fn pt2_bc(&self) -> PT2_BC_R {
        PT2_BC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline(always)]
    pub fn pt2_ac(&self) -> PT2_AC_R {
        PT2_AC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Product term 3, D input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt3_dc(&mut self) -> PT3_DC_W<0> {
        PT3_DC_W::new(self)
    }
    #[doc = "Bits 2:3 - Product term 3, C input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt3_cc(&mut self) -> PT3_CC_W<2> {
        PT3_CC_W::new(self)
    }
    #[doc = "Bits 4:5 - Product term 3, B input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt3_bc(&mut self) -> PT3_BC_W<4> {
        PT3_BC_W::new(self)
    }
    #[doc = "Bits 6:7 - Product term 3, A input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt3_ac(&mut self) -> PT3_AC_W<6> {
        PT3_AC_W::new(self)
    }
    #[doc = "Bits 8:9 - Product term 2, D input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt2_dc(&mut self) -> PT2_DC_W<8> {
        PT2_DC_W::new(self)
    }
    #[doc = "Bits 10:11 - Product term 2, C input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt2_cc(&mut self) -> PT2_CC_W<10> {
        PT2_CC_W::new(self)
    }
    #[doc = "Bits 12:13 - Product term 2, B input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt2_bc(&mut self) -> PT2_BC_W<12> {
        PT2_BC_W::new(self)
    }
    #[doc = "Bits 14:15 - Product term 2, A input configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pt2_ac(&mut self) -> PT2_AC_W<14> {
        PT2_AC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration Register for EVENTn\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcrt23](index.html) module"]
pub struct BFCRT23_SPEC;
impl crate::RegisterSpec for BFCRT23_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bfcrt23::R](R) reader structure"]
impl crate::Readable for BFCRT23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfcrt23::W](W) writer structure"]
impl crate::Writable for BFCRT23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFCRT23%s to value 0"]
impl crate::Resettable for BFCRT23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
