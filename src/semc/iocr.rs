#[doc = "Register `IOCR` reader"]
pub struct R(crate::R<IOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR` writer"]
pub struct W(crate::W<IOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR_SPEC>;
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
impl From<crate::W<IOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_A8` reader - SEMC_ADDR08 output selection"]
pub type MUX_A8_R = crate::FieldReader<u8, MUX_A8_A>;
#[doc = "SEMC_ADDR08 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_A8_A {
    #[doc = "0: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_0 = 0,
    #[doc = "1: NAND CE#"]
    MUX_A8_1 = 1,
    #[doc = "2: NOR CE#"]
    MUX_A8_2 = 2,
    #[doc = "3: SRAM CE#"]
    MUX_A8_3 = 3,
    #[doc = "4: DBI CSX"]
    MUX_A8_4 = 4,
    #[doc = "5: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_5 = 5,
    #[doc = "6: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_6 = 6,
    #[doc = "7: SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_7 = 7,
}
impl From<MUX_A8_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A8_A) -> Self {
        variant as _
    }
}
impl MUX_A8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A8_A {
        match self.bits {
            0 => MUX_A8_A::MUX_A8_0,
            1 => MUX_A8_A::MUX_A8_1,
            2 => MUX_A8_A::MUX_A8_2,
            3 => MUX_A8_A::MUX_A8_3,
            4 => MUX_A8_A::MUX_A8_4,
            5 => MUX_A8_A::MUX_A8_5,
            6 => MUX_A8_A::MUX_A8_6,
            7 => MUX_A8_A::MUX_A8_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_A8_0`"]
    #[inline(always)]
    pub fn is_mux_a8_0(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_0
    }
    #[doc = "Checks if the value of the field is `MUX_A8_1`"]
    #[inline(always)]
    pub fn is_mux_a8_1(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_1
    }
    #[doc = "Checks if the value of the field is `MUX_A8_2`"]
    #[inline(always)]
    pub fn is_mux_a8_2(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_2
    }
    #[doc = "Checks if the value of the field is `MUX_A8_3`"]
    #[inline(always)]
    pub fn is_mux_a8_3(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_3
    }
    #[doc = "Checks if the value of the field is `MUX_A8_4`"]
    #[inline(always)]
    pub fn is_mux_a8_4(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_4
    }
    #[doc = "Checks if the value of the field is `MUX_A8_5`"]
    #[inline(always)]
    pub fn is_mux_a8_5(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_5
    }
    #[doc = "Checks if the value of the field is `MUX_A8_6`"]
    #[inline(always)]
    pub fn is_mux_a8_6(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_6
    }
    #[doc = "Checks if the value of the field is `MUX_A8_7`"]
    #[inline(always)]
    pub fn is_mux_a8_7(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_7
    }
}
#[doc = "Field `MUX_A8` writer - SEMC_ADDR08 output selection"]
pub type MUX_A8_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IOCR_SPEC, u8, MUX_A8_A, 3, O>;
impl<'a, const O: u8> MUX_A8_W<'a, O> {
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    #[inline(always)]
    pub fn mux_a8_0(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_0)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_a8_1(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_1)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_a8_2(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_2)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_a8_3(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_3)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_a8_4(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_4)
    }
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    #[inline(always)]
    pub fn mux_a8_5(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_5)
    }
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    #[inline(always)]
    pub fn mux_a8_6(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_6)
    }
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    #[inline(always)]
    pub fn mux_a8_7(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_7)
    }
}
#[doc = "Field `MUX_CSX0` reader - SEMC_CSX0 output selection"]
pub type MUX_CSX0_R = crate::FieldReader<u8, MUX_CSX0_A>;
#[doc = "SEMC_CSX0 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_CSX0_A {
    #[doc = "1: SDRAM CS1"]
    MUX_CSX0_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX0_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX0_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX0_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX0_5 = 5,
    #[doc = "6: SRAM CE#"]
    MUX_CSX0_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX0_7 = 7,
}
impl From<MUX_CSX0_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX0_A) -> Self {
        variant as _
    }
}
impl MUX_CSX0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_CSX0_A> {
        match self.bits {
            1 => Some(MUX_CSX0_A::MUX_CSX0_1),
            2 => Some(MUX_CSX0_A::MUX_CSX0_2),
            3 => Some(MUX_CSX0_A::MUX_CSX0_3),
            4 => Some(MUX_CSX0_A::MUX_CSX0_4),
            5 => Some(MUX_CSX0_A::MUX_CSX0_5),
            6 => Some(MUX_CSX0_A::MUX_CSX0_6),
            7 => Some(MUX_CSX0_A::MUX_CSX0_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_1`"]
    #[inline(always)]
    pub fn is_mux_csx0_1(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_2`"]
    #[inline(always)]
    pub fn is_mux_csx0_2(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_3`"]
    #[inline(always)]
    pub fn is_mux_csx0_3(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_4`"]
    #[inline(always)]
    pub fn is_mux_csx0_4(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_5`"]
    #[inline(always)]
    pub fn is_mux_csx0_5(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_6`"]
    #[inline(always)]
    pub fn is_mux_csx0_6(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_7`"]
    #[inline(always)]
    pub fn is_mux_csx0_7(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_7
    }
}
#[doc = "Field `MUX_CSX0` writer - SEMC_CSX0 output selection"]
pub type MUX_CSX0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR_SPEC, u8, MUX_CSX0_A, 3, O>;
impl<'a, const O: u8> MUX_CSX0_W<'a, O> {
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx0_1(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx0_2(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx0_3(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx0_4(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx0_5(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_5)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_csx0_6(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx0_7(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_7)
    }
}
#[doc = "Field `MUX_CSX1` reader - SEMC_CSX1 output selection"]
pub type MUX_CSX1_R = crate::FieldReader<u8, MUX_CSX1_A>;
#[doc = "SEMC_CSX1 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_CSX1_A {
    #[doc = "1: SDRAM CS1"]
    MUX_CSX1_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX1_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX1_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX1_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX1_5 = 5,
    #[doc = "6: SRAM CE#"]
    MUX_CSX1_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX1_7 = 7,
}
impl From<MUX_CSX1_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX1_A) -> Self {
        variant as _
    }
}
impl MUX_CSX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_CSX1_A> {
        match self.bits {
            1 => Some(MUX_CSX1_A::MUX_CSX1_1),
            2 => Some(MUX_CSX1_A::MUX_CSX1_2),
            3 => Some(MUX_CSX1_A::MUX_CSX1_3),
            4 => Some(MUX_CSX1_A::MUX_CSX1_4),
            5 => Some(MUX_CSX1_A::MUX_CSX1_5),
            6 => Some(MUX_CSX1_A::MUX_CSX1_6),
            7 => Some(MUX_CSX1_A::MUX_CSX1_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_1`"]
    #[inline(always)]
    pub fn is_mux_csx1_1(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_2`"]
    #[inline(always)]
    pub fn is_mux_csx1_2(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_3`"]
    #[inline(always)]
    pub fn is_mux_csx1_3(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_4`"]
    #[inline(always)]
    pub fn is_mux_csx1_4(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_5`"]
    #[inline(always)]
    pub fn is_mux_csx1_5(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_6`"]
    #[inline(always)]
    pub fn is_mux_csx1_6(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_7`"]
    #[inline(always)]
    pub fn is_mux_csx1_7(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_7
    }
}
#[doc = "Field `MUX_CSX1` writer - SEMC_CSX1 output selection"]
pub type MUX_CSX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR_SPEC, u8, MUX_CSX1_A, 3, O>;
impl<'a, const O: u8> MUX_CSX1_W<'a, O> {
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx1_1(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx1_2(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx1_3(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx1_4(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx1_5(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_5)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_csx1_6(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx1_7(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_7)
    }
}
#[doc = "Field `MUX_CSX2` reader - SEMC_CSX2 output selection"]
pub type MUX_CSX2_R = crate::FieldReader<u8, MUX_CSX2_A>;
#[doc = "SEMC_CSX2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_CSX2_A {
    #[doc = "1: SDRAM CS1"]
    MUX_CSX2_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX2_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX2_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX2_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX2_5 = 5,
    #[doc = "6: SRAM CE#"]
    MUX_CSX2_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX2_7 = 7,
}
impl From<MUX_CSX2_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX2_A) -> Self {
        variant as _
    }
}
impl MUX_CSX2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_CSX2_A> {
        match self.bits {
            1 => Some(MUX_CSX2_A::MUX_CSX2_1),
            2 => Some(MUX_CSX2_A::MUX_CSX2_2),
            3 => Some(MUX_CSX2_A::MUX_CSX2_3),
            4 => Some(MUX_CSX2_A::MUX_CSX2_4),
            5 => Some(MUX_CSX2_A::MUX_CSX2_5),
            6 => Some(MUX_CSX2_A::MUX_CSX2_6),
            7 => Some(MUX_CSX2_A::MUX_CSX2_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_1`"]
    #[inline(always)]
    pub fn is_mux_csx2_1(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_2`"]
    #[inline(always)]
    pub fn is_mux_csx2_2(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_3`"]
    #[inline(always)]
    pub fn is_mux_csx2_3(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_4`"]
    #[inline(always)]
    pub fn is_mux_csx2_4(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_5`"]
    #[inline(always)]
    pub fn is_mux_csx2_5(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_6`"]
    #[inline(always)]
    pub fn is_mux_csx2_6(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_7`"]
    #[inline(always)]
    pub fn is_mux_csx2_7(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_7
    }
}
#[doc = "Field `MUX_CSX2` writer - SEMC_CSX2 output selection"]
pub type MUX_CSX2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR_SPEC, u8, MUX_CSX2_A, 3, O>;
impl<'a, const O: u8> MUX_CSX2_W<'a, O> {
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx2_1(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx2_2(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx2_3(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx2_4(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx2_5(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_5)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_csx2_6(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx2_7(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_7)
    }
}
#[doc = "Field `MUX_CSX3` reader - SEMC_CSX3 output selection"]
pub type MUX_CSX3_R = crate::FieldReader<u8, MUX_CSX3_A>;
#[doc = "SEMC_CSX3 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_CSX3_A {
    #[doc = "1: SDRAM CS1"]
    MUX_CSX3_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX3_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX3_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX3_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX3_5 = 5,
    #[doc = "6: SRAM CE#"]
    MUX_CSX3_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX3_7 = 7,
}
impl From<MUX_CSX3_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX3_A) -> Self {
        variant as _
    }
}
impl MUX_CSX3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_CSX3_A> {
        match self.bits {
            1 => Some(MUX_CSX3_A::MUX_CSX3_1),
            2 => Some(MUX_CSX3_A::MUX_CSX3_2),
            3 => Some(MUX_CSX3_A::MUX_CSX3_3),
            4 => Some(MUX_CSX3_A::MUX_CSX3_4),
            5 => Some(MUX_CSX3_A::MUX_CSX3_5),
            6 => Some(MUX_CSX3_A::MUX_CSX3_6),
            7 => Some(MUX_CSX3_A::MUX_CSX3_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_1`"]
    #[inline(always)]
    pub fn is_mux_csx3_1(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_2`"]
    #[inline(always)]
    pub fn is_mux_csx3_2(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_3`"]
    #[inline(always)]
    pub fn is_mux_csx3_3(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_4`"]
    #[inline(always)]
    pub fn is_mux_csx3_4(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_5`"]
    #[inline(always)]
    pub fn is_mux_csx3_5(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_6`"]
    #[inline(always)]
    pub fn is_mux_csx3_6(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_7`"]
    #[inline(always)]
    pub fn is_mux_csx3_7(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_7
    }
}
#[doc = "Field `MUX_CSX3` writer - SEMC_CSX3 output selection"]
pub type MUX_CSX3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR_SPEC, u8, MUX_CSX3_A, 3, O>;
impl<'a, const O: u8> MUX_CSX3_W<'a, O> {
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx3_1(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx3_2(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx3_3(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx3_4(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx3_5(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_5)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_csx3_6(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx3_7(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_7)
    }
}
#[doc = "Field `MUX_RDY` reader - SEMC_RDY function selection"]
pub type MUX_RDY_R = crate::FieldReader<u8, MUX_RDY_A>;
#[doc = "SEMC_RDY function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MUX_RDY_A {
    #[doc = "0: NAND R/B# input"]
    MUX_RDY_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_RDY_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_RDY_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_RDY_3 = 3,
    #[doc = "4: NOR CE#"]
    MUX_RDY_4 = 4,
    #[doc = "5: SRAM CE#"]
    MUX_RDY_5 = 5,
    #[doc = "6: DBI CSX"]
    MUX_RDY_6 = 6,
}
impl From<MUX_RDY_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_RDY_A) -> Self {
        variant as _
    }
}
impl MUX_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUX_RDY_A> {
        match self.bits {
            0 => Some(MUX_RDY_A::MUX_RDY_0),
            1 => Some(MUX_RDY_A::MUX_RDY_1),
            2 => Some(MUX_RDY_A::MUX_RDY_2),
            3 => Some(MUX_RDY_A::MUX_RDY_3),
            4 => Some(MUX_RDY_A::MUX_RDY_4),
            5 => Some(MUX_RDY_A::MUX_RDY_5),
            6 => Some(MUX_RDY_A::MUX_RDY_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_0`"]
    #[inline(always)]
    pub fn is_mux_rdy_0(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_0
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_1`"]
    #[inline(always)]
    pub fn is_mux_rdy_1(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_1
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_2`"]
    #[inline(always)]
    pub fn is_mux_rdy_2(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_2
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_3`"]
    #[inline(always)]
    pub fn is_mux_rdy_3(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_3
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_4`"]
    #[inline(always)]
    pub fn is_mux_rdy_4(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_4
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_5`"]
    #[inline(always)]
    pub fn is_mux_rdy_5(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_5
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_6`"]
    #[inline(always)]
    pub fn is_mux_rdy_6(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_6
    }
}
#[doc = "Field `MUX_RDY` writer - SEMC_RDY function selection"]
pub type MUX_RDY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOCR_SPEC, u8, MUX_RDY_A, 3, O>;
impl<'a, const O: u8> MUX_RDY_W<'a, O> {
    #[doc = "NAND R/B# input"]
    #[inline(always)]
    pub fn mux_rdy_0(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_rdy_1(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_rdy_2(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_rdy_3(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_3)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_rdy_4(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_4)
    }
    #[doc = "SRAM CE#"]
    #[inline(always)]
    pub fn mux_rdy_5(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_5)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_rdy_6(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_6)
    }
}
impl R {
    #[doc = "Bits 0:2 - SEMC_ADDR08 output selection"]
    #[inline(always)]
    pub fn mux_a8(&self) -> MUX_A8_R {
        MUX_A8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline(always)]
    pub fn mux_csx0(&self) -> MUX_CSX0_R {
        MUX_CSX0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline(always)]
    pub fn mux_csx1(&self) -> MUX_CSX1_R {
        MUX_CSX1_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline(always)]
    pub fn mux_csx2(&self) -> MUX_CSX2_R {
        MUX_CSX2_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline(always)]
    pub fn mux_csx3(&self) -> MUX_CSX3_R {
        MUX_CSX3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline(always)]
    pub fn mux_rdy(&self) -> MUX_RDY_R {
        MUX_RDY_R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SEMC_ADDR08 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_a8(&mut self) -> MUX_A8_W<0> {
        MUX_A8_W::new(self)
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_csx0(&mut self) -> MUX_CSX0_W<3> {
        MUX_CSX0_W::new(self)
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_csx1(&mut self) -> MUX_CSX1_W<6> {
        MUX_CSX1_W::new(self)
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_csx2(&mut self) -> MUX_CSX2_W<9> {
        MUX_CSX2_W::new(self)
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_csx3(&mut self) -> MUX_CSX3_W<12> {
        MUX_CSX3_W::new(self)
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline(always)]
    #[must_use]
    pub fn mux_rdy(&mut self) -> MUX_RDY_W<15> {
        MUX_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr](index.html) module"]
pub struct IOCR_SPEC;
impl crate::RegisterSpec for IOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr::R](R) reader structure"]
impl crate::Readable for IOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr::W](W) writer structure"]
impl crate::Writable for IOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOCR to value 0"]
impl crate::Resettable for IOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
