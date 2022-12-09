#[doc = "Register `ID_ISAR1` reader"]
pub struct R(crate::R<ID_ISAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_ISAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_ISAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_ISAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTEND_INSTRS` reader - Indicates the supported Extend instructions"]
pub type EXTEND_INSTRS_R = crate::FieldReader<u8, EXTEND_INSTRS_A>;
#[doc = "Indicates the supported Extend instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEND_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    EXTEND_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SXTB, SXTH, UXTB, and UXTH instructions"]
    EXTEND_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions"]
    EXTEND_INSTRS_2 = 2,
}
impl From<EXTEND_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEND_INSTRS_A) -> Self {
        variant as _
    }
}
impl EXTEND_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTEND_INSTRS_A> {
        match self.bits {
            0 => Some(EXTEND_INSTRS_A::EXTEND_INSTRS_0),
            1 => Some(EXTEND_INSTRS_A::EXTEND_INSTRS_1),
            2 => Some(EXTEND_INSTRS_A::EXTEND_INSTRS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_0`"]
    #[inline(always)]
    pub fn is_extend_instrs_0(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_1`"]
    #[inline(always)]
    pub fn is_extend_instrs_1(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_2`"]
    #[inline(always)]
    pub fn is_extend_instrs_2(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_2
    }
}
#[doc = "Field `IFTHEN_INSTRS` reader - Indicates the supported IfThen instructions"]
pub type IFTHEN_INSTRS_R = crate::FieldReader<u8, IFTHEN_INSTRS_A>;
#[doc = "Indicates the supported IfThen instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IFTHEN_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    IFTHEN_INSTRS_0 = 0,
    #[doc = "1: Adds support for the IT instructions, and for the IT bits in the PSRs"]
    IFTHEN_INSTRS_1 = 1,
}
impl From<IFTHEN_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: IFTHEN_INSTRS_A) -> Self {
        variant as _
    }
}
impl IFTHEN_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IFTHEN_INSTRS_A> {
        match self.bits {
            0 => Some(IFTHEN_INSTRS_A::IFTHEN_INSTRS_0),
            1 => Some(IFTHEN_INSTRS_A::IFTHEN_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_0`"]
    #[inline(always)]
    pub fn is_ifthen_instrs_0(&self) -> bool {
        *self == IFTHEN_INSTRS_A::IFTHEN_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_1`"]
    #[inline(always)]
    pub fn is_ifthen_instrs_1(&self) -> bool {
        *self == IFTHEN_INSTRS_A::IFTHEN_INSTRS_1
    }
}
#[doc = "Field `IMMEDIATE_INSTRS` reader - Indicates the support for data-processing instructions with long immediate"]
pub type IMMEDIATE_INSTRS_R = crate::FieldReader<u8, IMMEDIATE_INSTRS_A>;
#[doc = "Indicates the support for data-processing instructions with long immediate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMMEDIATE_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    IMMEDIATE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the ADDW, MOVW, MOVT, and SUBW instructions"]
    IMMEDIATE_INSTRS_1 = 1,
}
impl From<IMMEDIATE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: IMMEDIATE_INSTRS_A) -> Self {
        variant as _
    }
}
impl IMMEDIATE_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMMEDIATE_INSTRS_A> {
        match self.bits {
            0 => Some(IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_0),
            1 => Some(IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_immediate_instrs_0(&self) -> bool {
        *self == IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_immediate_instrs_1(&self) -> bool {
        *self == IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_1
    }
}
#[doc = "Field `INTERWORK_INSTRS` reader - Indicates the supported Interworking instructions"]
pub type INTERWORK_INSTRS_R = crate::FieldReader<u8, INTERWORK_INSTRS_A>;
#[doc = "Indicates the supported Interworking instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTERWORK_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    INTERWORK_INSTRS_0 = 0,
    #[doc = "1: Adds support for the BX instruction, and the T bit in the PSR"]
    INTERWORK_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior"]
    INTERWORK_INSTRS_2 = 2,
    #[doc = "3: ARMv7-M unused"]
    INTERWORK_INSTRS_3 = 3,
}
impl From<INTERWORK_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERWORK_INSTRS_A) -> Self {
        variant as _
    }
}
impl INTERWORK_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTERWORK_INSTRS_A> {
        match self.bits {
            0 => Some(INTERWORK_INSTRS_A::INTERWORK_INSTRS_0),
            1 => Some(INTERWORK_INSTRS_A::INTERWORK_INSTRS_1),
            2 => Some(INTERWORK_INSTRS_A::INTERWORK_INSTRS_2),
            3 => Some(INTERWORK_INSTRS_A::INTERWORK_INSTRS_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_0`"]
    #[inline(always)]
    pub fn is_interwork_instrs_0(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_1`"]
    #[inline(always)]
    pub fn is_interwork_instrs_1(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_2`"]
    #[inline(always)]
    pub fn is_interwork_instrs_2(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_3`"]
    #[inline(always)]
    pub fn is_interwork_instrs_3(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_3
    }
}
impl R {
    #[doc = "Bits 12:15 - Indicates the supported Extend instructions"]
    #[inline(always)]
    pub fn extend_instrs(&self) -> EXTEND_INSTRS_R {
        EXTEND_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported IfThen instructions"]
    #[inline(always)]
    pub fn ifthen_instrs(&self) -> IFTHEN_INSTRS_R {
        IFTHEN_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the support for data-processing instructions with long immediate"]
    #[inline(always)]
    pub fn immediate_instrs(&self) -> IMMEDIATE_INSTRS_R {
        IMMEDIATE_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported Interworking instructions"]
    #[inline(always)]
    pub fn interwork_instrs(&self) -> INTERWORK_INSTRS_R {
        INTERWORK_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Instruction Set Attributes Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar1](index.html) module"]
pub struct ID_ISAR1_SPEC;
impl crate::RegisterSpec for ID_ISAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_isar1::R](R) reader structure"]
impl crate::Readable for ID_ISAR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_ISAR1 to value 0"]
impl crate::Resettable for ID_ISAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
