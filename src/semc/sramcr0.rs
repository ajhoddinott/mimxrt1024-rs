#[doc = "Register `SRAMCR0` reader"]
pub struct R(crate::R<SRAMCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMCR0` writer"]
pub struct W(crate::W<SRAMCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMCR0_SPEC>;
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
impl From<crate::W<SRAMCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Port Size"]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "Port Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: 8bit"]
    PS_0 = 0,
    #[doc = "1: 16bit"]
    PS_1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::PS_0,
            true => PS_A::PS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline(always)]
    pub fn is_ps_0(&self) -> bool {
        *self == PS_A::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline(always)]
    pub fn is_ps_1(&self) -> bool {
        *self == PS_A::PS_1
    }
}
#[doc = "Field `PS` writer - Port Size"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMCR0_SPEC, PS_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "8bit"]
    #[inline(always)]
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PS_A::PS_0)
    }
    #[doc = "16bit"]
    #[inline(always)]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PS_A::PS_1)
    }
}
#[doc = "Field `WAITEN` reader - Wait Enable"]
pub type WAITEN_R = crate::BitReader<WAITEN_A>;
#[doc = "Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN_A {
    #[doc = "0: The SEMC does not monitor wait pin."]
    WAITEN_0 = 0,
    #[doc = "1: The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
    WAITEN_1 = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::WAITEN_0,
            true => WAITEN_A::WAITEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITEN_0`"]
    #[inline(always)]
    pub fn is_waiten_0(&self) -> bool {
        *self == WAITEN_A::WAITEN_0
    }
    #[doc = "Checks if the value of the field is `WAITEN_1`"]
    #[inline(always)]
    pub fn is_waiten_1(&self) -> bool {
        *self == WAITEN_A::WAITEN_1
    }
}
#[doc = "Field `WAITEN` writer - Wait Enable"]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMCR0_SPEC, WAITEN_A, O>;
impl<'a, const O: u8> WAITEN_W<'a, O> {
    #[doc = "The SEMC does not monitor wait pin."]
    #[inline(always)]
    pub fn waiten_0(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_0)
    }
    #[doc = "The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
    #[inline(always)]
    pub fn waiten_1(self) -> &'a mut W {
        self.variant(WAITEN_A::WAITEN_1)
    }
}
#[doc = "Field `WAITSP` reader - Wait Sample"]
pub type WAITSP_R = crate::BitReader<WAITSP_A>;
#[doc = "Wait Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITSP_A {
    #[doc = "0: Wait pin is directly used by the SEMC."]
    WAITSP_0 = 0,
    #[doc = "1: Wait pin is sampled by internal clock before it is used."]
    WAITSP_1 = 1,
}
impl From<WAITSP_A> for bool {
    #[inline(always)]
    fn from(variant: WAITSP_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITSP_A {
        match self.bits {
            false => WAITSP_A::WAITSP_0,
            true => WAITSP_A::WAITSP_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAITSP_0`"]
    #[inline(always)]
    pub fn is_waitsp_0(&self) -> bool {
        *self == WAITSP_A::WAITSP_0
    }
    #[doc = "Checks if the value of the field is `WAITSP_1`"]
    #[inline(always)]
    pub fn is_waitsp_1(&self) -> bool {
        *self == WAITSP_A::WAITSP_1
    }
}
#[doc = "Field `WAITSP` writer - Wait Sample"]
pub type WAITSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMCR0_SPEC, WAITSP_A, O>;
impl<'a, const O: u8> WAITSP_W<'a, O> {
    #[doc = "Wait pin is directly used by the SEMC."]
    #[inline(always)]
    pub fn waitsp_0(self) -> &'a mut W {
        self.variant(WAITSP_A::WAITSP_0)
    }
    #[doc = "Wait pin is sampled by internal clock before it is used."]
    #[inline(always)]
    pub fn waitsp_1(self) -> &'a mut W {
        self.variant(WAITSP_A::WAITSP_1)
    }
}
#[doc = "Field `BL` reader - Burst Length"]
pub type BL_R = crate::FieldReader<u8, BL_A>;
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BL_A {
    #[doc = "0: 1"]
    BL_0 = 0,
    #[doc = "1: 2"]
    BL_1 = 1,
    #[doc = "2: 4"]
    BL_2 = 2,
    #[doc = "3: 8"]
    BL_3 = 3,
    #[doc = "4: 16"]
    BL_4 = 4,
    #[doc = "5: 32"]
    BL_5 = 5,
    #[doc = "6: 64"]
    BL_6 = 6,
    #[doc = "7: 64"]
    BL_7 = 7,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
impl BL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::BL_0,
            1 => BL_A::BL_1,
            2 => BL_A::BL_2,
            3 => BL_A::BL_3,
            4 => BL_A::BL_4,
            5 => BL_A::BL_5,
            6 => BL_A::BL_6,
            7 => BL_A::BL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BL_0`"]
    #[inline(always)]
    pub fn is_bl_0(&self) -> bool {
        *self == BL_A::BL_0
    }
    #[doc = "Checks if the value of the field is `BL_1`"]
    #[inline(always)]
    pub fn is_bl_1(&self) -> bool {
        *self == BL_A::BL_1
    }
    #[doc = "Checks if the value of the field is `BL_2`"]
    #[inline(always)]
    pub fn is_bl_2(&self) -> bool {
        *self == BL_A::BL_2
    }
    #[doc = "Checks if the value of the field is `BL_3`"]
    #[inline(always)]
    pub fn is_bl_3(&self) -> bool {
        *self == BL_A::BL_3
    }
    #[doc = "Checks if the value of the field is `BL_4`"]
    #[inline(always)]
    pub fn is_bl_4(&self) -> bool {
        *self == BL_A::BL_4
    }
    #[doc = "Checks if the value of the field is `BL_5`"]
    #[inline(always)]
    pub fn is_bl_5(&self) -> bool {
        *self == BL_A::BL_5
    }
    #[doc = "Checks if the value of the field is `BL_6`"]
    #[inline(always)]
    pub fn is_bl_6(&self) -> bool {
        *self == BL_A::BL_6
    }
    #[doc = "Checks if the value of the field is `BL_7`"]
    #[inline(always)]
    pub fn is_bl_7(&self) -> bool {
        *self == BL_A::BL_7
    }
}
#[doc = "Field `BL` writer - Burst Length"]
pub type BL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRAMCR0_SPEC, u8, BL_A, 3, O>;
impl<'a, const O: u8> BL_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn bl_0(self) -> &'a mut W {
        self.variant(BL_A::BL_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn bl_1(self) -> &'a mut W {
        self.variant(BL_A::BL_1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn bl_2(self) -> &'a mut W {
        self.variant(BL_A::BL_2)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_3(self) -> &'a mut W {
        self.variant(BL_A::BL_3)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BL_A::BL_4)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BL_A::BL_5)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BL_A::BL_6)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn bl_7(self) -> &'a mut W {
        self.variant(BL_A::BL_7)
    }
}
#[doc = "Field `AM` reader - Address Mode"]
pub type AM_R = crate::FieldReader<u8, AM_A>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM_A {
    #[doc = "0: Address/Data MUX mode (ADMUX)"]
    AM_0 = 0,
    #[doc = "1: Advanced Address/Data MUX mode (AADM)"]
    AM_1 = 1,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
impl AM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AM_A> {
        match self.bits {
            0 => Some(AM_A::AM_0),
            1 => Some(AM_A::AM_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AM_0`"]
    #[inline(always)]
    pub fn is_am_0(&self) -> bool {
        *self == AM_A::AM_0
    }
    #[doc = "Checks if the value of the field is `AM_1`"]
    #[inline(always)]
    pub fn is_am_1(&self) -> bool {
        *self == AM_A::AM_1
    }
}
#[doc = "Field `AM` writer - Address Mode"]
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAMCR0_SPEC, u8, AM_A, 2, O>;
impl<'a, const O: u8> AM_W<'a, O> {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    #[inline(always)]
    pub fn am_0(self) -> &'a mut W {
        self.variant(AM_A::AM_0)
    }
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    #[inline(always)]
    pub fn am_1(self) -> &'a mut W {
        self.variant(AM_A::AM_1)
    }
}
#[doc = "Field `ADVP` reader - ADV# polarity"]
pub type ADVP_R = crate::BitReader<ADVP_A>;
#[doc = "ADV# polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVP_A {
    #[doc = "0: ADV# is active low."]
    ADVP_0 = 0,
    #[doc = "1: ADV# is active high."]
    ADVP_1 = 1,
}
impl From<ADVP_A> for bool {
    #[inline(always)]
    fn from(variant: ADVP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADVP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVP_A {
        match self.bits {
            false => ADVP_A::ADVP_0,
            true => ADVP_A::ADVP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADVP_0`"]
    #[inline(always)]
    pub fn is_advp_0(&self) -> bool {
        *self == ADVP_A::ADVP_0
    }
    #[doc = "Checks if the value of the field is `ADVP_1`"]
    #[inline(always)]
    pub fn is_advp_1(&self) -> bool {
        *self == ADVP_A::ADVP_1
    }
}
#[doc = "Field `ADVP` writer - ADV# polarity"]
pub type ADVP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRAMCR0_SPEC, ADVP_A, O>;
impl<'a, const O: u8> ADVP_W<'a, O> {
    #[doc = "ADV# is active low."]
    #[inline(always)]
    pub fn advp_0(self) -> &'a mut W {
        self.variant(ADVP_A::ADVP_0)
    }
    #[doc = "ADV# is active high."]
    #[inline(always)]
    pub fn advp_1(self) -> &'a mut W {
        self.variant(ADVP_A::ADVP_1)
    }
}
#[doc = "Field `COL` reader - Column Address bit width"]
pub type COL_R = crate::FieldReader<u8, COL_A>;
#[doc = "Column Address bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: 12 Bits"]
    COL_0 = 0,
    #[doc = "1: 11 Bits"]
    COL_1 = 1,
    #[doc = "2: 10 Bits"]
    COL_2 = 2,
    #[doc = "3: 9 Bits"]
    COL_3 = 3,
    #[doc = "4: 8 Bits"]
    COL_4 = 4,
    #[doc = "5: 7 Bits"]
    COL_5 = 5,
    #[doc = "6: 6 Bits"]
    COL_6 = 6,
    #[doc = "7: 5 Bits"]
    COL_7 = 7,
    #[doc = "8: 4 Bits"]
    COL_8 = 8,
    #[doc = "9: 3 Bits"]
    COL_9 = 9,
    #[doc = "10: 2 Bits"]
    COL_10 = 10,
    #[doc = "11: 12 Bits"]
    COL_11 = 11,
    #[doc = "12: 12 Bits"]
    COL_12 = 12,
    #[doc = "13: 12 Bits"]
    COL_13 = 13,
    #[doc = "14: 12 Bits"]
    COL_14 = 14,
    #[doc = "15: 12 Bits"]
    COL_15 = 15,
}
impl From<COL_A> for u8 {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as _
    }
}
impl COL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            0 => COL_A::COL_0,
            1 => COL_A::COL_1,
            2 => COL_A::COL_2,
            3 => COL_A::COL_3,
            4 => COL_A::COL_4,
            5 => COL_A::COL_5,
            6 => COL_A::COL_6,
            7 => COL_A::COL_7,
            8 => COL_A::COL_8,
            9 => COL_A::COL_9,
            10 => COL_A::COL_10,
            11 => COL_A::COL_11,
            12 => COL_A::COL_12,
            13 => COL_A::COL_13,
            14 => COL_A::COL_14,
            15 => COL_A::COL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL_0`"]
    #[inline(always)]
    pub fn is_col_0(&self) -> bool {
        *self == COL_A::COL_0
    }
    #[doc = "Checks if the value of the field is `COL_1`"]
    #[inline(always)]
    pub fn is_col_1(&self) -> bool {
        *self == COL_A::COL_1
    }
    #[doc = "Checks if the value of the field is `COL_2`"]
    #[inline(always)]
    pub fn is_col_2(&self) -> bool {
        *self == COL_A::COL_2
    }
    #[doc = "Checks if the value of the field is `COL_3`"]
    #[inline(always)]
    pub fn is_col_3(&self) -> bool {
        *self == COL_A::COL_3
    }
    #[doc = "Checks if the value of the field is `COL_4`"]
    #[inline(always)]
    pub fn is_col_4(&self) -> bool {
        *self == COL_A::COL_4
    }
    #[doc = "Checks if the value of the field is `COL_5`"]
    #[inline(always)]
    pub fn is_col_5(&self) -> bool {
        *self == COL_A::COL_5
    }
    #[doc = "Checks if the value of the field is `COL_6`"]
    #[inline(always)]
    pub fn is_col_6(&self) -> bool {
        *self == COL_A::COL_6
    }
    #[doc = "Checks if the value of the field is `COL_7`"]
    #[inline(always)]
    pub fn is_col_7(&self) -> bool {
        *self == COL_A::COL_7
    }
    #[doc = "Checks if the value of the field is `COL_8`"]
    #[inline(always)]
    pub fn is_col_8(&self) -> bool {
        *self == COL_A::COL_8
    }
    #[doc = "Checks if the value of the field is `COL_9`"]
    #[inline(always)]
    pub fn is_col_9(&self) -> bool {
        *self == COL_A::COL_9
    }
    #[doc = "Checks if the value of the field is `COL_10`"]
    #[inline(always)]
    pub fn is_col_10(&self) -> bool {
        *self == COL_A::COL_10
    }
    #[doc = "Checks if the value of the field is `COL_11`"]
    #[inline(always)]
    pub fn is_col_11(&self) -> bool {
        *self == COL_A::COL_11
    }
    #[doc = "Checks if the value of the field is `COL_12`"]
    #[inline(always)]
    pub fn is_col_12(&self) -> bool {
        *self == COL_A::COL_12
    }
    #[doc = "Checks if the value of the field is `COL_13`"]
    #[inline(always)]
    pub fn is_col_13(&self) -> bool {
        *self == COL_A::COL_13
    }
    #[doc = "Checks if the value of the field is `COL_14`"]
    #[inline(always)]
    pub fn is_col_14(&self) -> bool {
        *self == COL_A::COL_14
    }
    #[doc = "Checks if the value of the field is `COL_15`"]
    #[inline(always)]
    pub fn is_col_15(&self) -> bool {
        *self == COL_A::COL_15
    }
}
#[doc = "Field `COL` writer - Column Address bit width"]
pub type COL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRAMCR0_SPEC, u8, COL_A, 4, O>;
impl<'a, const O: u8> COL_W<'a, O> {
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COL_A::COL_0)
    }
    #[doc = "11 Bits"]
    #[inline(always)]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COL_A::COL_1)
    }
    #[doc = "10 Bits"]
    #[inline(always)]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COL_A::COL_2)
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COL_A::COL_3)
    }
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn col_4(self) -> &'a mut W {
        self.variant(COL_A::COL_4)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn col_5(self) -> &'a mut W {
        self.variant(COL_A::COL_5)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn col_6(self) -> &'a mut W {
        self.variant(COL_A::COL_6)
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn col_7(self) -> &'a mut W {
        self.variant(COL_A::COL_7)
    }
    #[doc = "4 Bits"]
    #[inline(always)]
    pub fn col_8(self) -> &'a mut W {
        self.variant(COL_A::COL_8)
    }
    #[doc = "3 Bits"]
    #[inline(always)]
    pub fn col_9(self) -> &'a mut W {
        self.variant(COL_A::COL_9)
    }
    #[doc = "2 Bits"]
    #[inline(always)]
    pub fn col_10(self) -> &'a mut W {
        self.variant(COL_A::COL_10)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_11(self) -> &'a mut W {
        self.variant(COL_A::COL_11)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_12(self) -> &'a mut W {
        self.variant(COL_A::COL_12)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_13(self) -> &'a mut W {
        self.variant(COL_A::COL_13)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_14(self) -> &'a mut W {
        self.variant(COL_A::COL_14)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_15(self) -> &'a mut W {
        self.variant(COL_A::COL_15)
    }
}
impl R {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Enable"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wait Sample"]
    #[inline(always)]
    pub fn waitsp(&self) -> WAITSP_R {
        WAITSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Address Mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - ADV# polarity"]
    #[inline(always)]
    pub fn advp(&self) -> ADVP_R {
        ADVP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bit 2 - Wait Enable"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<2> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 3 - Wait Sample"]
    #[inline(always)]
    #[must_use]
    pub fn waitsp(&mut self) -> WAITSP_W<3> {
        WAITSP_W::new(self)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<4> {
        BL_W::new(self)
    }
    #[doc = "Bits 8:9 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<8> {
        AM_W::new(self)
    }
    #[doc = "Bit 10 - ADV# polarity"]
    #[inline(always)]
    #[must_use]
    pub fn advp(&mut self) -> ADVP_W<10> {
        ADVP_W::new(self)
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<12> {
        COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr0](index.html) module"]
pub struct SRAMCR0_SPEC;
impl crate::RegisterSpec for SRAMCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramcr0::R](R) reader structure"]
impl crate::Readable for SRAMCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramcr0::W](W) writer structure"]
impl crate::Writable for SRAMCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMCR0 to value 0"]
impl crate::Resettable for SRAMCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
