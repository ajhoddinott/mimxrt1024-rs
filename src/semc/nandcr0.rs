#[doc = "Register `NANDCR0` reader"]
pub struct R(crate::R<NANDCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NANDCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NANDCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NANDCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NANDCR0` writer"]
pub struct W(crate::W<NANDCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NANDCR0_SPEC>;
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
impl From<crate::W<NANDCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NANDCR0_SPEC>) -> Self {
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
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCR0_SPEC, PS_A, O>;
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
pub type BL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NANDCR0_SPEC, u8, BL_A, 3, O>;
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
#[doc = "Field `EDO` reader - EDO mode enabled"]
pub type EDO_R = crate::BitReader<EDO_A>;
#[doc = "EDO mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDO_A {
    #[doc = "0: EDO mode disabled"]
    EDO_0 = 0,
    #[doc = "1: EDO mode enabled"]
    EDO_1 = 1,
}
impl From<EDO_A> for bool {
    #[inline(always)]
    fn from(variant: EDO_A) -> Self {
        variant as u8 != 0
    }
}
impl EDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDO_A {
        match self.bits {
            false => EDO_A::EDO_0,
            true => EDO_A::EDO_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDO_0`"]
    #[inline(always)]
    pub fn is_edo_0(&self) -> bool {
        *self == EDO_A::EDO_0
    }
    #[doc = "Checks if the value of the field is `EDO_1`"]
    #[inline(always)]
    pub fn is_edo_1(&self) -> bool {
        *self == EDO_A::EDO_1
    }
}
#[doc = "Field `EDO` writer - EDO mode enabled"]
pub type EDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCR0_SPEC, EDO_A, O>;
impl<'a, const O: u8> EDO_W<'a, O> {
    #[doc = "EDO mode disabled"]
    #[inline(always)]
    pub fn edo_0(self) -> &'a mut W {
        self.variant(EDO_A::EDO_0)
    }
    #[doc = "EDO mode enabled"]
    #[inline(always)]
    pub fn edo_1(self) -> &'a mut W {
        self.variant(EDO_A::EDO_1)
    }
}
#[doc = "Field `COL` reader - Column address bit number"]
pub type COL_R = crate::FieldReader<u8, COL_A>;
#[doc = "Column address bit number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: 16"]
    COL_0 = 0,
    #[doc = "1: 15"]
    COL_1 = 1,
    #[doc = "2: 14"]
    COL_2 = 2,
    #[doc = "3: 13"]
    COL_3 = 3,
    #[doc = "4: 12"]
    COL_4 = 4,
    #[doc = "5: 11"]
    COL_5 = 5,
    #[doc = "6: 10"]
    COL_6 = 6,
    #[doc = "7: 9"]
    COL_7 = 7,
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
}
#[doc = "Field `COL` writer - Column address bit number"]
pub type COL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NANDCR0_SPEC, u8, COL_A, 3, O>;
impl<'a, const O: u8> COL_W<'a, O> {
    #[doc = "16"]
    #[inline(always)]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COL_A::COL_0)
    }
    #[doc = "15"]
    #[inline(always)]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COL_A::COL_1)
    }
    #[doc = "14"]
    #[inline(always)]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COL_A::COL_2)
    }
    #[doc = "13"]
    #[inline(always)]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COL_A::COL_3)
    }
    #[doc = "12"]
    #[inline(always)]
    pub fn col_4(self) -> &'a mut W {
        self.variant(COL_A::COL_4)
    }
    #[doc = "11"]
    #[inline(always)]
    pub fn col_5(self) -> &'a mut W {
        self.variant(COL_A::COL_5)
    }
    #[doc = "10"]
    #[inline(always)]
    pub fn col_6(self) -> &'a mut W {
        self.variant(COL_A::COL_6)
    }
    #[doc = "9"]
    #[inline(always)]
    pub fn col_7(self) -> &'a mut W {
        self.variant(COL_A::COL_7)
    }
}
impl R {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EDO mode enabled"]
    #[inline(always)]
    pub fn edo(&self) -> EDO_R {
        EDO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Column address bit number"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<4> {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - EDO mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn edo(&mut self) -> EDO_W<7> {
        EDO_W::new(self)
    }
    #[doc = "Bits 8:10 - Column address bit number"]
    #[inline(always)]
    #[must_use]
    pub fn col(&mut self) -> COL_W<8> {
        COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr0](index.html) module"]
pub struct NANDCR0_SPEC;
impl crate::RegisterSpec for NANDCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nandcr0::R](R) reader structure"]
impl crate::Readable for NANDCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nandcr0::W](W) writer structure"]
impl crate::Writable for NANDCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NANDCR0 to value 0"]
impl crate::Resettable for NANDCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
