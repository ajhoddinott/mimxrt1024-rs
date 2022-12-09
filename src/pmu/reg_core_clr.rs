#[doc = "Register `REG_CORE_CLR` reader"]
pub struct R(crate::R<REG_CORE_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_CORE_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_CORE_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_CORE_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_CORE_CLR` writer"]
pub struct W(crate::W<REG_CORE_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_CORE_CLR_SPEC>;
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
impl From<crate::W<REG_CORE_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_CORE_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG0_TARG` reader - This field defines the target voltage for the Arm core power domain"]
pub type REG0_TARG_R = crate::FieldReader<u8, REG0_TARG_A>;
#[doc = "This field defines the target voltage for the Arm core power domain\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG0_TARG_A {
    #[doc = "0: Power gated off"]
    REG0_TARG_0 = 0,
    #[doc = "1: Target core voltage = 0.725V"]
    REG0_TARG_1 = 1,
    #[doc = "2: Target core voltage = 0.750V"]
    REG0_TARG_2 = 2,
    #[doc = "3: Target core voltage = 0.775V"]
    REG0_TARG_3 = 3,
    #[doc = "16: Target core voltage = 1.100V"]
    REG0_TARG_16 = 16,
    #[doc = "30: Target core voltage = 1.450V"]
    REG0_TARG_30 = 30,
    #[doc = "31: Power FET switched full on. No regulation."]
    REG0_TARG_31 = 31,
}
impl From<REG0_TARG_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_TARG_A) -> Self {
        variant as _
    }
}
impl REG0_TARG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG0_TARG_A> {
        match self.bits {
            0 => Some(REG0_TARG_A::REG0_TARG_0),
            1 => Some(REG0_TARG_A::REG0_TARG_1),
            2 => Some(REG0_TARG_A::REG0_TARG_2),
            3 => Some(REG0_TARG_A::REG0_TARG_3),
            16 => Some(REG0_TARG_A::REG0_TARG_16),
            30 => Some(REG0_TARG_A::REG0_TARG_30),
            31 => Some(REG0_TARG_A::REG0_TARG_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_0`"]
    #[inline(always)]
    pub fn is_reg0_targ_0(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_1`"]
    #[inline(always)]
    pub fn is_reg0_targ_1(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_2`"]
    #[inline(always)]
    pub fn is_reg0_targ_2(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_3`"]
    #[inline(always)]
    pub fn is_reg0_targ_3(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_16`"]
    #[inline(always)]
    pub fn is_reg0_targ_16(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_30`"]
    #[inline(always)]
    pub fn is_reg0_targ_30(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG0_TARG_31`"]
    #[inline(always)]
    pub fn is_reg0_targ_31(&self) -> bool {
        *self == REG0_TARG_A::REG0_TARG_31
    }
}
#[doc = "Field `REG0_TARG` writer - This field defines the target voltage for the Arm core power domain"]
pub type REG0_TARG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_CORE_CLR_SPEC, u8, REG0_TARG_A, 5, O>;
impl<'a, const O: u8> REG0_TARG_W<'a, O> {
    #[doc = "Power gated off"]
    #[inline(always)]
    pub fn reg0_targ_0(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline(always)]
    pub fn reg0_targ_1(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline(always)]
    pub fn reg0_targ_2(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline(always)]
    pub fn reg0_targ_3(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline(always)]
    pub fn reg0_targ_16(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline(always)]
    pub fn reg0_targ_30(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline(always)]
    pub fn reg0_targ_31(self) -> &'a mut W {
        self.variant(REG0_TARG_A::REG0_TARG_31)
    }
}
#[doc = "Field `REG0_ADJ` reader - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG0_ADJ_R = crate::FieldReader<u8, REG0_ADJ_A>;
#[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG0_ADJ_A {
    #[doc = "0: No adjustment"]
    REG0_ADJ_0 = 0,
    #[doc = "1: + 0.25%"]
    REG0_ADJ_1 = 1,
    #[doc = "2: + 0.50%"]
    REG0_ADJ_2 = 2,
    #[doc = "3: + 0.75%"]
    REG0_ADJ_3 = 3,
    #[doc = "4: + 1.00%"]
    REG0_ADJ_4 = 4,
    #[doc = "5: + 1.25%"]
    REG0_ADJ_5 = 5,
    #[doc = "6: + 1.50%"]
    REG0_ADJ_6 = 6,
    #[doc = "7: + 1.75%"]
    REG0_ADJ_7 = 7,
    #[doc = "8: - 0.25%"]
    REG0_ADJ_8 = 8,
    #[doc = "9: - 0.50%"]
    REG0_ADJ_9 = 9,
    #[doc = "10: - 0.75%"]
    REG0_ADJ_10 = 10,
    #[doc = "11: - 1.00%"]
    REG0_ADJ_11 = 11,
    #[doc = "12: - 1.25%"]
    REG0_ADJ_12 = 12,
    #[doc = "13: - 1.50%"]
    REG0_ADJ_13 = 13,
    #[doc = "14: - 1.75%"]
    REG0_ADJ_14 = 14,
    #[doc = "15: - 2.00%"]
    REG0_ADJ_15 = 15,
}
impl From<REG0_ADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_ADJ_A) -> Self {
        variant as _
    }
}
impl REG0_ADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG0_ADJ_A {
        match self.bits {
            0 => REG0_ADJ_A::REG0_ADJ_0,
            1 => REG0_ADJ_A::REG0_ADJ_1,
            2 => REG0_ADJ_A::REG0_ADJ_2,
            3 => REG0_ADJ_A::REG0_ADJ_3,
            4 => REG0_ADJ_A::REG0_ADJ_4,
            5 => REG0_ADJ_A::REG0_ADJ_5,
            6 => REG0_ADJ_A::REG0_ADJ_6,
            7 => REG0_ADJ_A::REG0_ADJ_7,
            8 => REG0_ADJ_A::REG0_ADJ_8,
            9 => REG0_ADJ_A::REG0_ADJ_9,
            10 => REG0_ADJ_A::REG0_ADJ_10,
            11 => REG0_ADJ_A::REG0_ADJ_11,
            12 => REG0_ADJ_A::REG0_ADJ_12,
            13 => REG0_ADJ_A::REG0_ADJ_13,
            14 => REG0_ADJ_A::REG0_ADJ_14,
            15 => REG0_ADJ_A::REG0_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_0`"]
    #[inline(always)]
    pub fn is_reg0_adj_0(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_1`"]
    #[inline(always)]
    pub fn is_reg0_adj_1(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_2`"]
    #[inline(always)]
    pub fn is_reg0_adj_2(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_3`"]
    #[inline(always)]
    pub fn is_reg0_adj_3(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_4`"]
    #[inline(always)]
    pub fn is_reg0_adj_4(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_5`"]
    #[inline(always)]
    pub fn is_reg0_adj_5(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_6`"]
    #[inline(always)]
    pub fn is_reg0_adj_6(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_7`"]
    #[inline(always)]
    pub fn is_reg0_adj_7(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_8`"]
    #[inline(always)]
    pub fn is_reg0_adj_8(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_9`"]
    #[inline(always)]
    pub fn is_reg0_adj_9(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_10`"]
    #[inline(always)]
    pub fn is_reg0_adj_10(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_11`"]
    #[inline(always)]
    pub fn is_reg0_adj_11(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_12`"]
    #[inline(always)]
    pub fn is_reg0_adj_12(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_13`"]
    #[inline(always)]
    pub fn is_reg0_adj_13(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_14`"]
    #[inline(always)]
    pub fn is_reg0_adj_14(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG0_ADJ_15`"]
    #[inline(always)]
    pub fn is_reg0_adj_15(&self) -> bool {
        *self == REG0_ADJ_A::REG0_ADJ_15
    }
}
#[doc = "Field `REG0_ADJ` writer - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG0_ADJ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG_CORE_CLR_SPEC, u8, REG0_ADJ_A, 4, O>;
impl<'a, const O: u8> REG0_ADJ_W<'a, O> {
    #[doc = "No adjustment"]
    #[inline(always)]
    pub fn reg0_adj_0(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline(always)]
    pub fn reg0_adj_1(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline(always)]
    pub fn reg0_adj_2(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline(always)]
    pub fn reg0_adj_3(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline(always)]
    pub fn reg0_adj_4(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline(always)]
    pub fn reg0_adj_5(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline(always)]
    pub fn reg0_adj_6(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline(always)]
    pub fn reg0_adj_7(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline(always)]
    pub fn reg0_adj_8(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline(always)]
    pub fn reg0_adj_9(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline(always)]
    pub fn reg0_adj_10(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline(always)]
    pub fn reg0_adj_11(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline(always)]
    pub fn reg0_adj_12(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline(always)]
    pub fn reg0_adj_13(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline(always)]
    pub fn reg0_adj_14(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline(always)]
    pub fn reg0_adj_15(self) -> &'a mut W {
        self.variant(REG0_ADJ_A::REG0_ADJ_15)
    }
}
#[doc = "Field `REG1_TARG` reader - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
pub type REG1_TARG_R = crate::FieldReader<u8, REG1_TARG_A>;
#[doc = "This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG1_TARG_A {
    #[doc = "0: Power gated off"]
    REG1_TARG_0 = 0,
    #[doc = "1: Target core voltage = 0.725V"]
    REG1_TARG_1 = 1,
    #[doc = "2: Target core voltage = 0.750V"]
    REG1_TARG_2 = 2,
    #[doc = "3: Target core voltage = 0.775V"]
    REG1_TARG_3 = 3,
    #[doc = "16: Target core voltage = 1.100V"]
    REG1_TARG_16 = 16,
    #[doc = "30: Target core voltage = 1.450V"]
    REG1_TARG_30 = 30,
    #[doc = "31: Power FET switched full on. No regulation."]
    REG1_TARG_31 = 31,
}
impl From<REG1_TARG_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_TARG_A) -> Self {
        variant as _
    }
}
impl REG1_TARG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG1_TARG_A> {
        match self.bits {
            0 => Some(REG1_TARG_A::REG1_TARG_0),
            1 => Some(REG1_TARG_A::REG1_TARG_1),
            2 => Some(REG1_TARG_A::REG1_TARG_2),
            3 => Some(REG1_TARG_A::REG1_TARG_3),
            16 => Some(REG1_TARG_A::REG1_TARG_16),
            30 => Some(REG1_TARG_A::REG1_TARG_30),
            31 => Some(REG1_TARG_A::REG1_TARG_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_0`"]
    #[inline(always)]
    pub fn is_reg1_targ_0(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_1`"]
    #[inline(always)]
    pub fn is_reg1_targ_1(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_2`"]
    #[inline(always)]
    pub fn is_reg1_targ_2(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_3`"]
    #[inline(always)]
    pub fn is_reg1_targ_3(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_16`"]
    #[inline(always)]
    pub fn is_reg1_targ_16(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_30`"]
    #[inline(always)]
    pub fn is_reg1_targ_30(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG1_TARG_31`"]
    #[inline(always)]
    pub fn is_reg1_targ_31(&self) -> bool {
        *self == REG1_TARG_A::REG1_TARG_31
    }
}
#[doc = "Field `REG1_TARG` writer - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
pub type REG1_TARG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_CORE_CLR_SPEC, u8, REG1_TARG_A, 5, O>;
impl<'a, const O: u8> REG1_TARG_W<'a, O> {
    #[doc = "Power gated off"]
    #[inline(always)]
    pub fn reg1_targ_0(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline(always)]
    pub fn reg1_targ_1(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline(always)]
    pub fn reg1_targ_2(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline(always)]
    pub fn reg1_targ_3(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline(always)]
    pub fn reg1_targ_16(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline(always)]
    pub fn reg1_targ_30(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline(always)]
    pub fn reg1_targ_31(self) -> &'a mut W {
        self.variant(REG1_TARG_A::REG1_TARG_31)
    }
}
#[doc = "Field `REG1_ADJ` reader - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG1_ADJ_R = crate::FieldReader<u8, REG1_ADJ_A>;
#[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG1_ADJ_A {
    #[doc = "0: No adjustment"]
    REG1_ADJ_0 = 0,
    #[doc = "1: + 0.25%"]
    REG1_ADJ_1 = 1,
    #[doc = "2: + 0.50%"]
    REG1_ADJ_2 = 2,
    #[doc = "3: + 0.75%"]
    REG1_ADJ_3 = 3,
    #[doc = "4: + 1.00%"]
    REG1_ADJ_4 = 4,
    #[doc = "5: + 1.25%"]
    REG1_ADJ_5 = 5,
    #[doc = "6: + 1.50%"]
    REG1_ADJ_6 = 6,
    #[doc = "7: + 1.75%"]
    REG1_ADJ_7 = 7,
    #[doc = "8: - 0.25%"]
    REG1_ADJ_8 = 8,
    #[doc = "9: - 0.50%"]
    REG1_ADJ_9 = 9,
    #[doc = "10: - 0.75%"]
    REG1_ADJ_10 = 10,
    #[doc = "11: - 1.00%"]
    REG1_ADJ_11 = 11,
    #[doc = "12: - 1.25%"]
    REG1_ADJ_12 = 12,
    #[doc = "13: - 1.50%"]
    REG1_ADJ_13 = 13,
    #[doc = "14: - 1.75%"]
    REG1_ADJ_14 = 14,
    #[doc = "15: - 2.00%"]
    REG1_ADJ_15 = 15,
}
impl From<REG1_ADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_ADJ_A) -> Self {
        variant as _
    }
}
impl REG1_ADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG1_ADJ_A {
        match self.bits {
            0 => REG1_ADJ_A::REG1_ADJ_0,
            1 => REG1_ADJ_A::REG1_ADJ_1,
            2 => REG1_ADJ_A::REG1_ADJ_2,
            3 => REG1_ADJ_A::REG1_ADJ_3,
            4 => REG1_ADJ_A::REG1_ADJ_4,
            5 => REG1_ADJ_A::REG1_ADJ_5,
            6 => REG1_ADJ_A::REG1_ADJ_6,
            7 => REG1_ADJ_A::REG1_ADJ_7,
            8 => REG1_ADJ_A::REG1_ADJ_8,
            9 => REG1_ADJ_A::REG1_ADJ_9,
            10 => REG1_ADJ_A::REG1_ADJ_10,
            11 => REG1_ADJ_A::REG1_ADJ_11,
            12 => REG1_ADJ_A::REG1_ADJ_12,
            13 => REG1_ADJ_A::REG1_ADJ_13,
            14 => REG1_ADJ_A::REG1_ADJ_14,
            15 => REG1_ADJ_A::REG1_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_0`"]
    #[inline(always)]
    pub fn is_reg1_adj_0(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_1`"]
    #[inline(always)]
    pub fn is_reg1_adj_1(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_2`"]
    #[inline(always)]
    pub fn is_reg1_adj_2(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_3`"]
    #[inline(always)]
    pub fn is_reg1_adj_3(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_4`"]
    #[inline(always)]
    pub fn is_reg1_adj_4(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_5`"]
    #[inline(always)]
    pub fn is_reg1_adj_5(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_6`"]
    #[inline(always)]
    pub fn is_reg1_adj_6(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_7`"]
    #[inline(always)]
    pub fn is_reg1_adj_7(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_8`"]
    #[inline(always)]
    pub fn is_reg1_adj_8(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_9`"]
    #[inline(always)]
    pub fn is_reg1_adj_9(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_10`"]
    #[inline(always)]
    pub fn is_reg1_adj_10(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_11`"]
    #[inline(always)]
    pub fn is_reg1_adj_11(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_12`"]
    #[inline(always)]
    pub fn is_reg1_adj_12(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_13`"]
    #[inline(always)]
    pub fn is_reg1_adj_13(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_14`"]
    #[inline(always)]
    pub fn is_reg1_adj_14(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG1_ADJ_15`"]
    #[inline(always)]
    pub fn is_reg1_adj_15(&self) -> bool {
        *self == REG1_ADJ_A::REG1_ADJ_15
    }
}
#[doc = "Field `REG1_ADJ` writer - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG1_ADJ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG_CORE_CLR_SPEC, u8, REG1_ADJ_A, 4, O>;
impl<'a, const O: u8> REG1_ADJ_W<'a, O> {
    #[doc = "No adjustment"]
    #[inline(always)]
    pub fn reg1_adj_0(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline(always)]
    pub fn reg1_adj_1(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline(always)]
    pub fn reg1_adj_2(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline(always)]
    pub fn reg1_adj_3(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline(always)]
    pub fn reg1_adj_4(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline(always)]
    pub fn reg1_adj_5(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline(always)]
    pub fn reg1_adj_6(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline(always)]
    pub fn reg1_adj_7(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline(always)]
    pub fn reg1_adj_8(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline(always)]
    pub fn reg1_adj_9(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline(always)]
    pub fn reg1_adj_10(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline(always)]
    pub fn reg1_adj_11(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline(always)]
    pub fn reg1_adj_12(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline(always)]
    pub fn reg1_adj_13(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline(always)]
    pub fn reg1_adj_14(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline(always)]
    pub fn reg1_adj_15(self) -> &'a mut W {
        self.variant(REG1_ADJ_A::REG1_ADJ_15)
    }
}
#[doc = "Field `REG2_TARG` reader - This field defines the target voltage for the SOC power domain"]
pub type REG2_TARG_R = crate::FieldReader<u8, REG2_TARG_A>;
#[doc = "This field defines the target voltage for the SOC power domain\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG2_TARG_A {
    #[doc = "0: Power gated off"]
    REG2_TARG_0 = 0,
    #[doc = "1: Target core voltage = 0.725V"]
    REG2_TARG_1 = 1,
    #[doc = "2: Target core voltage = 0.750V"]
    REG2_TARG_2 = 2,
    #[doc = "3: Target core voltage = 0.775V"]
    REG2_TARG_3 = 3,
    #[doc = "16: Target core voltage = 1.100V"]
    REG2_TARG_16 = 16,
    #[doc = "30: Target core voltage = 1.450V"]
    REG2_TARG_30 = 30,
    #[doc = "31: Power FET switched full on. No regulation."]
    REG2_TARG_31 = 31,
}
impl From<REG2_TARG_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_TARG_A) -> Self {
        variant as _
    }
}
impl REG2_TARG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG2_TARG_A> {
        match self.bits {
            0 => Some(REG2_TARG_A::REG2_TARG_0),
            1 => Some(REG2_TARG_A::REG2_TARG_1),
            2 => Some(REG2_TARG_A::REG2_TARG_2),
            3 => Some(REG2_TARG_A::REG2_TARG_3),
            16 => Some(REG2_TARG_A::REG2_TARG_16),
            30 => Some(REG2_TARG_A::REG2_TARG_30),
            31 => Some(REG2_TARG_A::REG2_TARG_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_0`"]
    #[inline(always)]
    pub fn is_reg2_targ_0(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_0
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_1`"]
    #[inline(always)]
    pub fn is_reg2_targ_1(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_1
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_2`"]
    #[inline(always)]
    pub fn is_reg2_targ_2(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_2
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_3`"]
    #[inline(always)]
    pub fn is_reg2_targ_3(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_3
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_16`"]
    #[inline(always)]
    pub fn is_reg2_targ_16(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_16
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_30`"]
    #[inline(always)]
    pub fn is_reg2_targ_30(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_30
    }
    #[doc = "Checks if the value of the field is `REG2_TARG_31`"]
    #[inline(always)]
    pub fn is_reg2_targ_31(&self) -> bool {
        *self == REG2_TARG_A::REG2_TARG_31
    }
}
#[doc = "Field `REG2_TARG` writer - This field defines the target voltage for the SOC power domain"]
pub type REG2_TARG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_CORE_CLR_SPEC, u8, REG2_TARG_A, 5, O>;
impl<'a, const O: u8> REG2_TARG_W<'a, O> {
    #[doc = "Power gated off"]
    #[inline(always)]
    pub fn reg2_targ_0(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_0)
    }
    #[doc = "Target core voltage = 0.725V"]
    #[inline(always)]
    pub fn reg2_targ_1(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_1)
    }
    #[doc = "Target core voltage = 0.750V"]
    #[inline(always)]
    pub fn reg2_targ_2(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_2)
    }
    #[doc = "Target core voltage = 0.775V"]
    #[inline(always)]
    pub fn reg2_targ_3(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_3)
    }
    #[doc = "Target core voltage = 1.100V"]
    #[inline(always)]
    pub fn reg2_targ_16(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_16)
    }
    #[doc = "Target core voltage = 1.450V"]
    #[inline(always)]
    pub fn reg2_targ_30(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_30)
    }
    #[doc = "Power FET switched full on. No regulation."]
    #[inline(always)]
    pub fn reg2_targ_31(self) -> &'a mut W {
        self.variant(REG2_TARG_A::REG2_TARG_31)
    }
}
#[doc = "Field `REG2_ADJ` reader - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG2_ADJ_R = crate::FieldReader<u8, REG2_ADJ_A>;
#[doc = "This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REG2_ADJ_A {
    #[doc = "0: No adjustment"]
    REG2_ADJ_0 = 0,
    #[doc = "1: + 0.25%"]
    REG2_ADJ_1 = 1,
    #[doc = "2: + 0.50%"]
    REG2_ADJ_2 = 2,
    #[doc = "3: + 0.75%"]
    REG2_ADJ_3 = 3,
    #[doc = "4: + 1.00%"]
    REG2_ADJ_4 = 4,
    #[doc = "5: + 1.25%"]
    REG2_ADJ_5 = 5,
    #[doc = "6: + 1.50%"]
    REG2_ADJ_6 = 6,
    #[doc = "7: + 1.75%"]
    REG2_ADJ_7 = 7,
    #[doc = "8: - 0.25%"]
    REG2_ADJ_8 = 8,
    #[doc = "9: - 0.50%"]
    REG2_ADJ_9 = 9,
    #[doc = "10: - 0.75%"]
    REG2_ADJ_10 = 10,
    #[doc = "11: - 1.00%"]
    REG2_ADJ_11 = 11,
    #[doc = "12: - 1.25%"]
    REG2_ADJ_12 = 12,
    #[doc = "13: - 1.50%"]
    REG2_ADJ_13 = 13,
    #[doc = "14: - 1.75%"]
    REG2_ADJ_14 = 14,
    #[doc = "15: - 2.00%"]
    REG2_ADJ_15 = 15,
}
impl From<REG2_ADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_ADJ_A) -> Self {
        variant as _
    }
}
impl REG2_ADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG2_ADJ_A {
        match self.bits {
            0 => REG2_ADJ_A::REG2_ADJ_0,
            1 => REG2_ADJ_A::REG2_ADJ_1,
            2 => REG2_ADJ_A::REG2_ADJ_2,
            3 => REG2_ADJ_A::REG2_ADJ_3,
            4 => REG2_ADJ_A::REG2_ADJ_4,
            5 => REG2_ADJ_A::REG2_ADJ_5,
            6 => REG2_ADJ_A::REG2_ADJ_6,
            7 => REG2_ADJ_A::REG2_ADJ_7,
            8 => REG2_ADJ_A::REG2_ADJ_8,
            9 => REG2_ADJ_A::REG2_ADJ_9,
            10 => REG2_ADJ_A::REG2_ADJ_10,
            11 => REG2_ADJ_A::REG2_ADJ_11,
            12 => REG2_ADJ_A::REG2_ADJ_12,
            13 => REG2_ADJ_A::REG2_ADJ_13,
            14 => REG2_ADJ_A::REG2_ADJ_14,
            15 => REG2_ADJ_A::REG2_ADJ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_0`"]
    #[inline(always)]
    pub fn is_reg2_adj_0(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_0
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_1`"]
    #[inline(always)]
    pub fn is_reg2_adj_1(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_1
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_2`"]
    #[inline(always)]
    pub fn is_reg2_adj_2(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_2
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_3`"]
    #[inline(always)]
    pub fn is_reg2_adj_3(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_3
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_4`"]
    #[inline(always)]
    pub fn is_reg2_adj_4(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_4
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_5`"]
    #[inline(always)]
    pub fn is_reg2_adj_5(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_5
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_6`"]
    #[inline(always)]
    pub fn is_reg2_adj_6(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_6
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_7`"]
    #[inline(always)]
    pub fn is_reg2_adj_7(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_7
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_8`"]
    #[inline(always)]
    pub fn is_reg2_adj_8(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_8
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_9`"]
    #[inline(always)]
    pub fn is_reg2_adj_9(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_9
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_10`"]
    #[inline(always)]
    pub fn is_reg2_adj_10(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_10
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_11`"]
    #[inline(always)]
    pub fn is_reg2_adj_11(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_11
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_12`"]
    #[inline(always)]
    pub fn is_reg2_adj_12(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_12
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_13`"]
    #[inline(always)]
    pub fn is_reg2_adj_13(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_13
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_14`"]
    #[inline(always)]
    pub fn is_reg2_adj_14(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_14
    }
    #[doc = "Checks if the value of the field is `REG2_ADJ_15`"]
    #[inline(always)]
    pub fn is_reg2_adj_15(&self) -> bool {
        *self == REG2_ADJ_A::REG2_ADJ_15
    }
}
#[doc = "Field `REG2_ADJ` writer - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
pub type REG2_ADJ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG_CORE_CLR_SPEC, u8, REG2_ADJ_A, 4, O>;
impl<'a, const O: u8> REG2_ADJ_W<'a, O> {
    #[doc = "No adjustment"]
    #[inline(always)]
    pub fn reg2_adj_0(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_0)
    }
    #[doc = "+ 0.25%"]
    #[inline(always)]
    pub fn reg2_adj_1(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_1)
    }
    #[doc = "+ 0.50%"]
    #[inline(always)]
    pub fn reg2_adj_2(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_2)
    }
    #[doc = "+ 0.75%"]
    #[inline(always)]
    pub fn reg2_adj_3(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_3)
    }
    #[doc = "+ 1.00%"]
    #[inline(always)]
    pub fn reg2_adj_4(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_4)
    }
    #[doc = "+ 1.25%"]
    #[inline(always)]
    pub fn reg2_adj_5(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_5)
    }
    #[doc = "+ 1.50%"]
    #[inline(always)]
    pub fn reg2_adj_6(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_6)
    }
    #[doc = "+ 1.75%"]
    #[inline(always)]
    pub fn reg2_adj_7(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_7)
    }
    #[doc = "- 0.25%"]
    #[inline(always)]
    pub fn reg2_adj_8(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_8)
    }
    #[doc = "- 0.50%"]
    #[inline(always)]
    pub fn reg2_adj_9(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_9)
    }
    #[doc = "- 0.75%"]
    #[inline(always)]
    pub fn reg2_adj_10(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_10)
    }
    #[doc = "- 1.00%"]
    #[inline(always)]
    pub fn reg2_adj_11(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_11)
    }
    #[doc = "- 1.25%"]
    #[inline(always)]
    pub fn reg2_adj_12(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_12)
    }
    #[doc = "- 1.50%"]
    #[inline(always)]
    pub fn reg2_adj_13(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_13)
    }
    #[doc = "- 1.75%"]
    #[inline(always)]
    pub fn reg2_adj_14(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_14)
    }
    #[doc = "- 2.00%"]
    #[inline(always)]
    pub fn reg2_adj_15(self) -> &'a mut W {
        self.variant(REG2_ADJ_A::REG2_ADJ_15)
    }
}
#[doc = "Field `RAMP_RATE` reader - Regulator voltage ramp rate."]
pub type RAMP_RATE_R = crate::FieldReader<u8, RAMP_RATE_A>;
#[doc = "Regulator voltage ramp rate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMP_RATE_A {
    #[doc = "0: Fast"]
    RAMP_RATE_0 = 0,
    #[doc = "1: Medium Fast"]
    RAMP_RATE_1 = 1,
    #[doc = "2: Medium Slow"]
    RAMP_RATE_2 = 2,
    #[doc = "3: Slow"]
    RAMP_RATE_3 = 3,
}
impl From<RAMP_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMP_RATE_A) -> Self {
        variant as _
    }
}
impl RAMP_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMP_RATE_A {
        match self.bits {
            0 => RAMP_RATE_A::RAMP_RATE_0,
            1 => RAMP_RATE_A::RAMP_RATE_1,
            2 => RAMP_RATE_A::RAMP_RATE_2,
            3 => RAMP_RATE_A::RAMP_RATE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_0`"]
    #[inline(always)]
    pub fn is_ramp_rate_0(&self) -> bool {
        *self == RAMP_RATE_A::RAMP_RATE_0
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_1`"]
    #[inline(always)]
    pub fn is_ramp_rate_1(&self) -> bool {
        *self == RAMP_RATE_A::RAMP_RATE_1
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_2`"]
    #[inline(always)]
    pub fn is_ramp_rate_2(&self) -> bool {
        *self == RAMP_RATE_A::RAMP_RATE_2
    }
    #[doc = "Checks if the value of the field is `RAMP_RATE_3`"]
    #[inline(always)]
    pub fn is_ramp_rate_3(&self) -> bool {
        *self == RAMP_RATE_A::RAMP_RATE_3
    }
}
#[doc = "Field `RAMP_RATE` writer - Regulator voltage ramp rate."]
pub type RAMP_RATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG_CORE_CLR_SPEC, u8, RAMP_RATE_A, 2, O>;
impl<'a, const O: u8> RAMP_RATE_W<'a, O> {
    #[doc = "Fast"]
    #[inline(always)]
    pub fn ramp_rate_0(self) -> &'a mut W {
        self.variant(RAMP_RATE_A::RAMP_RATE_0)
    }
    #[doc = "Medium Fast"]
    #[inline(always)]
    pub fn ramp_rate_1(self) -> &'a mut W {
        self.variant(RAMP_RATE_A::RAMP_RATE_1)
    }
    #[doc = "Medium Slow"]
    #[inline(always)]
    pub fn ramp_rate_2(self) -> &'a mut W {
        self.variant(RAMP_RATE_A::RAMP_RATE_2)
    }
    #[doc = "Slow"]
    #[inline(always)]
    pub fn ramp_rate_3(self) -> &'a mut W {
        self.variant(RAMP_RATE_A::RAMP_RATE_3)
    }
}
#[doc = "Field `FET_ODRIVE` reader - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
pub type FET_ODRIVE_R = crate::BitReader<bool>;
#[doc = "Field `FET_ODRIVE` writer - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
pub type FET_ODRIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_CORE_CLR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    pub fn reg0_targ(&self) -> REG0_TARG_R {
        REG0_TARG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub fn reg0_adj(&self) -> REG0_ADJ_R {
        REG0_ADJ_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:13 - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    pub fn reg1_targ(&self) -> REG1_TARG_R {
        REG1_TARG_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:17 - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub fn reg1_adj(&self) -> REG1_ADJ_R {
        REG1_ADJ_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:22 - This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    pub fn reg2_targ(&self) -> REG2_TARG_R {
        REG2_TARG_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:26 - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    pub fn reg2_adj(&self) -> REG2_ADJ_R {
        REG2_ADJ_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:28 - Regulator voltage ramp rate."]
    #[inline(always)]
    pub fn ramp_rate(&self) -> RAMP_RATE_R {
        RAMP_RATE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    pub fn fet_odrive(&self) -> FET_ODRIVE_R {
        FET_ODRIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field defines the target voltage for the Arm core power domain"]
    #[inline(always)]
    #[must_use]
    pub fn reg0_targ(&mut self) -> REG0_TARG_W<0> {
        REG0_TARG_W::new(self)
    }
    #[doc = "Bits 5:8 - This bit field defines the adjustment bits to calibrate the target value of Reg0. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    #[must_use]
    pub fn reg0_adj(&mut self) -> REG0_ADJ_W<5> {
        REG0_ADJ_W::new(self)
    }
    #[doc = "Bits 9:13 - This bit field defines the target voltage for the vpu/gpu power domain. Single bit increments reflect 25mV core voltage steps. Not all steps will make sense to use either because of input supply limitations or load operation."]
    #[inline(always)]
    #[must_use]
    pub fn reg1_targ(&mut self) -> REG1_TARG_W<9> {
        REG1_TARG_W::new(self)
    }
    #[doc = "Bits 14:17 - This bit field defines the adjustment bits to calibrate the target value of Reg1. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    #[must_use]
    pub fn reg1_adj(&mut self) -> REG1_ADJ_W<14> {
        REG1_ADJ_W::new(self)
    }
    #[doc = "Bits 18:22 - This field defines the target voltage for the SOC power domain"]
    #[inline(always)]
    #[must_use]
    pub fn reg2_targ(&mut self) -> REG2_TARG_W<18> {
        REG2_TARG_W::new(self)
    }
    #[doc = "Bits 23:26 - This bit field defines the adjustment bits to calibrate the target value of Reg2. The adjustment is applied on top on any adjustment applied to the global reference in the misc0 register."]
    #[inline(always)]
    #[must_use]
    pub fn reg2_adj(&mut self) -> REG2_ADJ_W<23> {
        REG2_ADJ_W::new(self)
    }
    #[doc = "Bits 27:28 - Regulator voltage ramp rate."]
    #[inline(always)]
    #[must_use]
    pub fn ramp_rate(&mut self) -> RAMP_RATE_W<27> {
        RAMP_RATE_W::new(self)
    }
    #[doc = "Bit 29 - If set, increases the gate drive on power gating FETs to reduce leakage in the off state"]
    #[inline(always)]
    #[must_use]
    pub fn fet_odrive(&mut self) -> FET_ODRIVE_W<29> {
        FET_ODRIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Regulator Core Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_core_clr](index.html) module"]
pub struct REG_CORE_CLR_SPEC;
impl crate::RegisterSpec for REG_CORE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_core_clr::R](R) reader structure"]
impl crate::Readable for REG_CORE_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_core_clr::W](W) writer structure"]
impl crate::Writable for REG_CORE_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_CORE_CLR to value 0x0048_2012"]
impl crate::Resettable for REG_CORE_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0048_2012;
}
