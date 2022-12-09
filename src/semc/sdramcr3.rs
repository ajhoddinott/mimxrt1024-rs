#[doc = "Register `SDRAMCR3` reader"]
pub struct R(crate::R<SDRAMCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMCR3` writer"]
pub struct W(crate::W<SDRAMCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMCR3_SPEC>;
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
impl From<crate::W<SDRAMCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REN` reader - Refresh enable"]
pub type REN_R = crate::BitReader<REN_A>;
#[doc = "Refresh enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REN_A {
    #[doc = "0: The SEMC does not send AUTO REFRESH command automatically"]
    REN_0 = 0,
    #[doc = "1: The SEMC sends AUTO REFRESH command automatically"]
    REN_1 = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
impl REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::REN_0,
            true => REN_A::REN_1,
        }
    }
    #[doc = "Checks if the value of the field is `REN_0`"]
    #[inline(always)]
    pub fn is_ren_0(&self) -> bool {
        *self == REN_A::REN_0
    }
    #[doc = "Checks if the value of the field is `REN_1`"]
    #[inline(always)]
    pub fn is_ren_1(&self) -> bool {
        *self == REN_A::REN_1
    }
}
#[doc = "Field `REN` writer - Refresh enable"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRAMCR3_SPEC, REN_A, O>;
impl<'a, const O: u8> REN_W<'a, O> {
    #[doc = "The SEMC does not send AUTO REFRESH command automatically"]
    #[inline(always)]
    pub fn ren_0(self) -> &'a mut W {
        self.variant(REN_A::REN_0)
    }
    #[doc = "The SEMC sends AUTO REFRESH command automatically"]
    #[inline(always)]
    pub fn ren_1(self) -> &'a mut W {
        self.variant(REN_A::REN_1)
    }
}
#[doc = "Field `REBL` reader - Refresh burst length"]
pub type REBL_R = crate::FieldReader<u8, REBL_A>;
#[doc = "Refresh burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REBL_A {
    #[doc = "0: 1"]
    REBL_0 = 0,
    #[doc = "1: 2"]
    REBL_1 = 1,
    #[doc = "2: 3"]
    REBL_2 = 2,
    #[doc = "3: 4"]
    REBL_3 = 3,
    #[doc = "4: 5"]
    REBL_4 = 4,
    #[doc = "5: 6"]
    REBL_5 = 5,
    #[doc = "6: 7"]
    REBL_6 = 6,
    #[doc = "7: 8"]
    REBL_7 = 7,
}
impl From<REBL_A> for u8 {
    #[inline(always)]
    fn from(variant: REBL_A) -> Self {
        variant as _
    }
}
impl REBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REBL_A {
        match self.bits {
            0 => REBL_A::REBL_0,
            1 => REBL_A::REBL_1,
            2 => REBL_A::REBL_2,
            3 => REBL_A::REBL_3,
            4 => REBL_A::REBL_4,
            5 => REBL_A::REBL_5,
            6 => REBL_A::REBL_6,
            7 => REBL_A::REBL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REBL_0`"]
    #[inline(always)]
    pub fn is_rebl_0(&self) -> bool {
        *self == REBL_A::REBL_0
    }
    #[doc = "Checks if the value of the field is `REBL_1`"]
    #[inline(always)]
    pub fn is_rebl_1(&self) -> bool {
        *self == REBL_A::REBL_1
    }
    #[doc = "Checks if the value of the field is `REBL_2`"]
    #[inline(always)]
    pub fn is_rebl_2(&self) -> bool {
        *self == REBL_A::REBL_2
    }
    #[doc = "Checks if the value of the field is `REBL_3`"]
    #[inline(always)]
    pub fn is_rebl_3(&self) -> bool {
        *self == REBL_A::REBL_3
    }
    #[doc = "Checks if the value of the field is `REBL_4`"]
    #[inline(always)]
    pub fn is_rebl_4(&self) -> bool {
        *self == REBL_A::REBL_4
    }
    #[doc = "Checks if the value of the field is `REBL_5`"]
    #[inline(always)]
    pub fn is_rebl_5(&self) -> bool {
        *self == REBL_A::REBL_5
    }
    #[doc = "Checks if the value of the field is `REBL_6`"]
    #[inline(always)]
    pub fn is_rebl_6(&self) -> bool {
        *self == REBL_A::REBL_6
    }
    #[doc = "Checks if the value of the field is `REBL_7`"]
    #[inline(always)]
    pub fn is_rebl_7(&self) -> bool {
        *self == REBL_A::REBL_7
    }
}
#[doc = "Field `REBL` writer - Refresh burst length"]
pub type REBL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRAMCR3_SPEC, u8, REBL_A, 3, O>;
impl<'a, const O: u8> REBL_W<'a, O> {
    #[doc = "1"]
    #[inline(always)]
    pub fn rebl_0(self) -> &'a mut W {
        self.variant(REBL_A::REBL_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn rebl_1(self) -> &'a mut W {
        self.variant(REBL_A::REBL_1)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn rebl_2(self) -> &'a mut W {
        self.variant(REBL_A::REBL_2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn rebl_3(self) -> &'a mut W {
        self.variant(REBL_A::REBL_3)
    }
    #[doc = "5"]
    #[inline(always)]
    pub fn rebl_4(self) -> &'a mut W {
        self.variant(REBL_A::REBL_4)
    }
    #[doc = "6"]
    #[inline(always)]
    pub fn rebl_5(self) -> &'a mut W {
        self.variant(REBL_A::REBL_5)
    }
    #[doc = "7"]
    #[inline(always)]
    pub fn rebl_6(self) -> &'a mut W {
        self.variant(REBL_A::REBL_6)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn rebl_7(self) -> &'a mut W {
        self.variant(REBL_A::REBL_7)
    }
}
#[doc = "Field `PRESCALE` reader - Prescaler period"]
pub type PRESCALE_R = crate::FieldReader<u8, PRESCALE_A>;
#[doc = "Prescaler period\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: (256*16+1) clock cycles"]
    PRESCALE_0 = 0,
    #[doc = "1: (PRESCALE*16+1) clock cycles"]
    PRESCALE_1 = 1,
    #[doc = "2: (PRESCALE*16+1) clock cycles"]
    PRESCALE_2 = 2,
    #[doc = "3: (PRESCALE*16+1) clock cycles"]
    PRESCALE_3 = 3,
    #[doc = "4: (PRESCALE*16+1) clock cycles"]
    PRESCALE_4 = 4,
    #[doc = "5: (PRESCALE*16+1) clock cycles"]
    PRESCALE_5 = 5,
    #[doc = "6: (PRESCALE*16+1) clock cycles"]
    PRESCALE_6 = 6,
    #[doc = "7: (PRESCALE*16+1) clock cycles"]
    PRESCALE_7 = 7,
    #[doc = "8: (PRESCALE*16+1) clock cycles"]
    PRESCALE_8 = 8,
    #[doc = "9: (PRESCALE*16+1) clock cycles"]
    PRESCALE_9 = 9,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
impl PRESCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALE_A> {
        match self.bits {
            0 => Some(PRESCALE_A::PRESCALE_0),
            1 => Some(PRESCALE_A::PRESCALE_1),
            2 => Some(PRESCALE_A::PRESCALE_2),
            3 => Some(PRESCALE_A::PRESCALE_3),
            4 => Some(PRESCALE_A::PRESCALE_4),
            5 => Some(PRESCALE_A::PRESCALE_5),
            6 => Some(PRESCALE_A::PRESCALE_6),
            7 => Some(PRESCALE_A::PRESCALE_7),
            8 => Some(PRESCALE_A::PRESCALE_8),
            9 => Some(PRESCALE_A::PRESCALE_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_2
    }
    #[doc = "Checks if the value of the field is `PRESCALE_3`"]
    #[inline(always)]
    pub fn is_prescale_3(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_3
    }
    #[doc = "Checks if the value of the field is `PRESCALE_4`"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_4
    }
    #[doc = "Checks if the value of the field is `PRESCALE_5`"]
    #[inline(always)]
    pub fn is_prescale_5(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_5
    }
    #[doc = "Checks if the value of the field is `PRESCALE_6`"]
    #[inline(always)]
    pub fn is_prescale_6(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_6
    }
    #[doc = "Checks if the value of the field is `PRESCALE_7`"]
    #[inline(always)]
    pub fn is_prescale_7(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_7
    }
    #[doc = "Checks if the value of the field is `PRESCALE_8`"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_8
    }
    #[doc = "Checks if the value of the field is `PRESCALE_9`"]
    #[inline(always)]
    pub fn is_prescale_9(&self) -> bool {
        *self == PRESCALE_A::PRESCALE_9
    }
}
#[doc = "Field `PRESCALE` writer - Prescaler period"]
pub type PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDRAMCR3_SPEC, u8, PRESCALE_A, 8, O>;
impl<'a, const O: u8> PRESCALE_W<'a, O> {
    #[doc = "(256*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_3(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_3)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_4)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_5(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_5)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_6(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_6)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_7(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_7)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_8)
    }
    #[doc = "(PRESCALE*16+1) clock cycles"]
    #[inline(always)]
    pub fn prescale_9(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_9)
    }
}
#[doc = "Field `RT` reader - Refresh timer period"]
pub type RT_R = crate::FieldReader<u8, RT_A>;
#[doc = "Refresh timer period\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RT_A {
    #[doc = "0: (256+1)*(Prescaler period)"]
    RT_0 = 0,
    #[doc = "1: (RT+1)*(Prescaler period)"]
    RT_1 = 1,
    #[doc = "2: (RT+1)*(Prescaler period)"]
    RT_2 = 2,
    #[doc = "3: (RT+1)*(Prescaler period)"]
    RT_3 = 3,
    #[doc = "4: (RT+1)*(Prescaler period)"]
    RT_4 = 4,
    #[doc = "5: (RT+1)*(Prescaler period)"]
    RT_5 = 5,
    #[doc = "6: (RT+1)*(Prescaler period)"]
    RT_6 = 6,
    #[doc = "7: (RT+1)*(Prescaler period)"]
    RT_7 = 7,
    #[doc = "8: (RT+1)*(Prescaler period)"]
    RT_8 = 8,
    #[doc = "9: (RT+1)*(Prescaler period)"]
    RT_9 = 9,
}
impl From<RT_A> for u8 {
    #[inline(always)]
    fn from(variant: RT_A) -> Self {
        variant as _
    }
}
impl RT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RT_A> {
        match self.bits {
            0 => Some(RT_A::RT_0),
            1 => Some(RT_A::RT_1),
            2 => Some(RT_A::RT_2),
            3 => Some(RT_A::RT_3),
            4 => Some(RT_A::RT_4),
            5 => Some(RT_A::RT_5),
            6 => Some(RT_A::RT_6),
            7 => Some(RT_A::RT_7),
            8 => Some(RT_A::RT_8),
            9 => Some(RT_A::RT_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RT_0`"]
    #[inline(always)]
    pub fn is_rt_0(&self) -> bool {
        *self == RT_A::RT_0
    }
    #[doc = "Checks if the value of the field is `RT_1`"]
    #[inline(always)]
    pub fn is_rt_1(&self) -> bool {
        *self == RT_A::RT_1
    }
    #[doc = "Checks if the value of the field is `RT_2`"]
    #[inline(always)]
    pub fn is_rt_2(&self) -> bool {
        *self == RT_A::RT_2
    }
    #[doc = "Checks if the value of the field is `RT_3`"]
    #[inline(always)]
    pub fn is_rt_3(&self) -> bool {
        *self == RT_A::RT_3
    }
    #[doc = "Checks if the value of the field is `RT_4`"]
    #[inline(always)]
    pub fn is_rt_4(&self) -> bool {
        *self == RT_A::RT_4
    }
    #[doc = "Checks if the value of the field is `RT_5`"]
    #[inline(always)]
    pub fn is_rt_5(&self) -> bool {
        *self == RT_A::RT_5
    }
    #[doc = "Checks if the value of the field is `RT_6`"]
    #[inline(always)]
    pub fn is_rt_6(&self) -> bool {
        *self == RT_A::RT_6
    }
    #[doc = "Checks if the value of the field is `RT_7`"]
    #[inline(always)]
    pub fn is_rt_7(&self) -> bool {
        *self == RT_A::RT_7
    }
    #[doc = "Checks if the value of the field is `RT_8`"]
    #[inline(always)]
    pub fn is_rt_8(&self) -> bool {
        *self == RT_A::RT_8
    }
    #[doc = "Checks if the value of the field is `RT_9`"]
    #[inline(always)]
    pub fn is_rt_9(&self) -> bool {
        *self == RT_A::RT_9
    }
}
#[doc = "Field `RT` writer - Refresh timer period"]
pub type RT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR3_SPEC, u8, RT_A, 8, O>;
impl<'a, const O: u8> RT_W<'a, O> {
    #[doc = "(256+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_0(self) -> &'a mut W {
        self.variant(RT_A::RT_0)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_1(self) -> &'a mut W {
        self.variant(RT_A::RT_1)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_2(self) -> &'a mut W {
        self.variant(RT_A::RT_2)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_3(self) -> &'a mut W {
        self.variant(RT_A::RT_3)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_4(self) -> &'a mut W {
        self.variant(RT_A::RT_4)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_5(self) -> &'a mut W {
        self.variant(RT_A::RT_5)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_6(self) -> &'a mut W {
        self.variant(RT_A::RT_6)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_7(self) -> &'a mut W {
        self.variant(RT_A::RT_7)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_8(self) -> &'a mut W {
        self.variant(RT_A::RT_8)
    }
    #[doc = "(RT+1)*(Prescaler period)"]
    #[inline(always)]
    pub fn rt_9(self) -> &'a mut W {
        self.variant(RT_A::RT_9)
    }
}
#[doc = "Field `UT` reader - Urgent refresh threshold"]
pub type UT_R = crate::FieldReader<u8, UT_A>;
#[doc = "Urgent refresh threshold\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UT_A {
    #[doc = "0: 256*(Prescaler period)"]
    UT_0 = 0,
    #[doc = "1: UT*(Prescaler period)"]
    UT_1 = 1,
    #[doc = "2: UT*(Prescaler period)"]
    UT_2 = 2,
    #[doc = "3: UT*(Prescaler period)"]
    UT_3 = 3,
    #[doc = "4: UT*(Prescaler period)"]
    UT_4 = 4,
    #[doc = "5: UT*(Prescaler period)"]
    UT_5 = 5,
    #[doc = "6: UT*(Prescaler period)"]
    UT_6 = 6,
    #[doc = "7: UT*(Prescaler period)"]
    UT_7 = 7,
    #[doc = "8: UT*(Prescaler period)"]
    UT_8 = 8,
    #[doc = "9: UT*(Prescaler period)"]
    UT_9 = 9,
}
impl From<UT_A> for u8 {
    #[inline(always)]
    fn from(variant: UT_A) -> Self {
        variant as _
    }
}
impl UT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UT_A> {
        match self.bits {
            0 => Some(UT_A::UT_0),
            1 => Some(UT_A::UT_1),
            2 => Some(UT_A::UT_2),
            3 => Some(UT_A::UT_3),
            4 => Some(UT_A::UT_4),
            5 => Some(UT_A::UT_5),
            6 => Some(UT_A::UT_6),
            7 => Some(UT_A::UT_7),
            8 => Some(UT_A::UT_8),
            9 => Some(UT_A::UT_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UT_0`"]
    #[inline(always)]
    pub fn is_ut_0(&self) -> bool {
        *self == UT_A::UT_0
    }
    #[doc = "Checks if the value of the field is `UT_1`"]
    #[inline(always)]
    pub fn is_ut_1(&self) -> bool {
        *self == UT_A::UT_1
    }
    #[doc = "Checks if the value of the field is `UT_2`"]
    #[inline(always)]
    pub fn is_ut_2(&self) -> bool {
        *self == UT_A::UT_2
    }
    #[doc = "Checks if the value of the field is `UT_3`"]
    #[inline(always)]
    pub fn is_ut_3(&self) -> bool {
        *self == UT_A::UT_3
    }
    #[doc = "Checks if the value of the field is `UT_4`"]
    #[inline(always)]
    pub fn is_ut_4(&self) -> bool {
        *self == UT_A::UT_4
    }
    #[doc = "Checks if the value of the field is `UT_5`"]
    #[inline(always)]
    pub fn is_ut_5(&self) -> bool {
        *self == UT_A::UT_5
    }
    #[doc = "Checks if the value of the field is `UT_6`"]
    #[inline(always)]
    pub fn is_ut_6(&self) -> bool {
        *self == UT_A::UT_6
    }
    #[doc = "Checks if the value of the field is `UT_7`"]
    #[inline(always)]
    pub fn is_ut_7(&self) -> bool {
        *self == UT_A::UT_7
    }
    #[doc = "Checks if the value of the field is `UT_8`"]
    #[inline(always)]
    pub fn is_ut_8(&self) -> bool {
        *self == UT_A::UT_8
    }
    #[doc = "Checks if the value of the field is `UT_9`"]
    #[inline(always)]
    pub fn is_ut_9(&self) -> bool {
        *self == UT_A::UT_9
    }
}
#[doc = "Field `UT` writer - Urgent refresh threshold"]
pub type UT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRAMCR3_SPEC, u8, UT_A, 8, O>;
impl<'a, const O: u8> UT_W<'a, O> {
    #[doc = "256*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_0(self) -> &'a mut W {
        self.variant(UT_A::UT_0)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_1(self) -> &'a mut W {
        self.variant(UT_A::UT_1)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_2(self) -> &'a mut W {
        self.variant(UT_A::UT_2)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_3(self) -> &'a mut W {
        self.variant(UT_A::UT_3)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_4(self) -> &'a mut W {
        self.variant(UT_A::UT_4)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_5(self) -> &'a mut W {
        self.variant(UT_A::UT_5)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_6(self) -> &'a mut W {
        self.variant(UT_A::UT_6)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_7(self) -> &'a mut W {
        self.variant(UT_A::UT_7)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_8(self) -> &'a mut W {
        self.variant(UT_A::UT_8)
    }
    #[doc = "UT*(Prescaler period)"]
    #[inline(always)]
    pub fn ut_9(self) -> &'a mut W {
        self.variant(UT_A::UT_9)
    }
}
impl R {
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline(always)]
    pub fn rebl(&self) -> REBL_R {
        REBL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Prescaler period"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Urgent refresh threshold"]
    #[inline(always)]
    pub fn ut(&self) -> UT_R {
        UT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<0> {
        REN_W::new(self)
    }
    #[doc = "Bits 1:3 - Refresh burst length"]
    #[inline(always)]
    #[must_use]
    pub fn rebl(&mut self) -> REBL_W<1> {
        REBL_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescaler period"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<8> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bits 16:23 - Refresh timer period"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RT_W<16> {
        RT_W::new(self)
    }
    #[doc = "Bits 24:31 - Urgent refresh threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ut(&mut self) -> UT_W<24> {
        UT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramcr3](index.html) module"]
pub struct SDRAMCR3_SPEC;
impl crate::RegisterSpec for SDRAMCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramcr3::R](R) reader structure"]
impl crate::Readable for SDRAMCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramcr3::W](W) writer structure"]
impl crate::Writable for SDRAMCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRAMCR3 to value 0x4080_8000"]
impl crate::Resettable for SDRAMCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x4080_8000;
}
