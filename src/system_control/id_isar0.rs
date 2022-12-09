#[doc = "Register `ID_ISAR0` reader"]
pub struct R(crate::R<ID_ISAR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_ISAR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_ISAR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_ISAR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BITCOUNT_INSTRS` reader - Indicates the supported Bit Counting instructions"]
pub type BITCOUNT_INSTRS_R = crate::FieldReader<u8, BITCOUNT_INSTRS_A>;
#[doc = "Indicates the supported Bit Counting instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITCOUNT_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    BITCOUNT_INSTRS_0 = 0,
    #[doc = "1: Adds support for the CLZ instruction"]
    BITCOUNT_INSTRS_1 = 1,
}
impl From<BITCOUNT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITCOUNT_INSTRS_A) -> Self {
        variant as _
    }
}
impl BITCOUNT_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITCOUNT_INSTRS_A> {
        match self.bits {
            0 => Some(BITCOUNT_INSTRS_A::BITCOUNT_INSTRS_0),
            1 => Some(BITCOUNT_INSTRS_A::BITCOUNT_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITCOUNT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_bitcount_instrs_0(&self) -> bool {
        *self == BITCOUNT_INSTRS_A::BITCOUNT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BITCOUNT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_bitcount_instrs_1(&self) -> bool {
        *self == BITCOUNT_INSTRS_A::BITCOUNT_INSTRS_1
    }
}
#[doc = "Field `BITFIELD_INSTRS` reader - Indicates the supported BitField instructions"]
pub type BITFIELD_INSTRS_R = crate::FieldReader<u8, BITFIELD_INSTRS_A>;
#[doc = "Indicates the supported BitField instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITFIELD_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    BITFIELD_INSTRS_0 = 0,
    #[doc = "1: Adds support for the BFC, BFI, SBFX, and UBFX instructions"]
    BITFIELD_INSTRS_1 = 1,
}
impl From<BITFIELD_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITFIELD_INSTRS_A) -> Self {
        variant as _
    }
}
impl BITFIELD_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITFIELD_INSTRS_A> {
        match self.bits {
            0 => Some(BITFIELD_INSTRS_A::BITFIELD_INSTRS_0),
            1 => Some(BITFIELD_INSTRS_A::BITFIELD_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITFIELD_INSTRS_0`"]
    #[inline(always)]
    pub fn is_bitfield_instrs_0(&self) -> bool {
        *self == BITFIELD_INSTRS_A::BITFIELD_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BITFIELD_INSTRS_1`"]
    #[inline(always)]
    pub fn is_bitfield_instrs_1(&self) -> bool {
        *self == BITFIELD_INSTRS_A::BITFIELD_INSTRS_1
    }
}
#[doc = "Field `CMPBRANCH_INSTRS` reader - Indicates the supported combined Compare and Branch instructions"]
pub type CMPBRANCH_INSTRS_R = crate::FieldReader<u8, CMPBRANCH_INSTRS_A>;
#[doc = "Indicates the supported combined Compare and Branch instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPBRANCH_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    CMPBRANCH_INSTRS_0 = 0,
    #[doc = "1: Adds support for the CBNZ and CBZ instructions"]
    CMPBRANCH_INSTRS_1 = 1,
}
impl From<CMPBRANCH_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPBRANCH_INSTRS_A) -> Self {
        variant as _
    }
}
impl CMPBRANCH_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPBRANCH_INSTRS_A> {
        match self.bits {
            0 => Some(CMPBRANCH_INSTRS_A::CMPBRANCH_INSTRS_0),
            1 => Some(CMPBRANCH_INSTRS_A::CMPBRANCH_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMPBRANCH_INSTRS_0`"]
    #[inline(always)]
    pub fn is_cmpbranch_instrs_0(&self) -> bool {
        *self == CMPBRANCH_INSTRS_A::CMPBRANCH_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `CMPBRANCH_INSTRS_1`"]
    #[inline(always)]
    pub fn is_cmpbranch_instrs_1(&self) -> bool {
        *self == CMPBRANCH_INSTRS_A::CMPBRANCH_INSTRS_1
    }
}
#[doc = "Field `COPROC_INSTRS` reader - Indicates the supported Coprocessor instructions"]
pub type COPROC_INSTRS_R = crate::FieldReader<u8, COPROC_INSTRS_A>;
#[doc = "Indicates the supported Coprocessor instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COPROC_INSTRS_A {
    #[doc = "0: None supported, except for separately attributed architectures, for example the Floating-point extension"]
    COPROC_INSTRS_0 = 0,
    #[doc = "1: Adds support for generic CDP, LDC, MCR, MRC, and STC instructions"]
    COPROC_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for generic CDP2, LDC2, MCR2, MRC2, and STC2 instructions"]
    COPROC_INSTRS_2 = 2,
    #[doc = "3: As for 2, and adds support for generic MCRR and MRRC instructions"]
    COPROC_INSTRS_3 = 3,
    #[doc = "4: As for 3, and adds support for generic MCRR2 and MRRC2 instructions"]
    COPROC_INSTRS_4 = 4,
}
impl From<COPROC_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: COPROC_INSTRS_A) -> Self {
        variant as _
    }
}
impl COPROC_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COPROC_INSTRS_A> {
        match self.bits {
            0 => Some(COPROC_INSTRS_A::COPROC_INSTRS_0),
            1 => Some(COPROC_INSTRS_A::COPROC_INSTRS_1),
            2 => Some(COPROC_INSTRS_A::COPROC_INSTRS_2),
            3 => Some(COPROC_INSTRS_A::COPROC_INSTRS_3),
            4 => Some(COPROC_INSTRS_A::COPROC_INSTRS_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_0`"]
    #[inline(always)]
    pub fn is_coproc_instrs_0(&self) -> bool {
        *self == COPROC_INSTRS_A::COPROC_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_1`"]
    #[inline(always)]
    pub fn is_coproc_instrs_1(&self) -> bool {
        *self == COPROC_INSTRS_A::COPROC_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_2`"]
    #[inline(always)]
    pub fn is_coproc_instrs_2(&self) -> bool {
        *self == COPROC_INSTRS_A::COPROC_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_3`"]
    #[inline(always)]
    pub fn is_coproc_instrs_3(&self) -> bool {
        *self == COPROC_INSTRS_A::COPROC_INSTRS_3
    }
    #[doc = "Checks if the value of the field is `COPROC_INSTRS_4`"]
    #[inline(always)]
    pub fn is_coproc_instrs_4(&self) -> bool {
        *self == COPROC_INSTRS_A::COPROC_INSTRS_4
    }
}
#[doc = "Field `DEBUG_INSTRS` reader - Indicates the supported Debug instructions"]
pub type DEBUG_INSTRS_R = crate::FieldReader<u8, DEBUG_INSTRS_A>;
#[doc = "Indicates the supported Debug instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBUG_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    DEBUG_INSTRS_0 = 0,
    #[doc = "1: Adds support for the BKPT instruction"]
    DEBUG_INSTRS_1 = 1,
}
impl From<DEBUG_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUG_INSTRS_A) -> Self {
        variant as _
    }
}
impl DEBUG_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEBUG_INSTRS_A> {
        match self.bits {
            0 => Some(DEBUG_INSTRS_A::DEBUG_INSTRS_0),
            1 => Some(DEBUG_INSTRS_A::DEBUG_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUG_INSTRS_0`"]
    #[inline(always)]
    pub fn is_debug_instrs_0(&self) -> bool {
        *self == DEBUG_INSTRS_A::DEBUG_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `DEBUG_INSTRS_1`"]
    #[inline(always)]
    pub fn is_debug_instrs_1(&self) -> bool {
        *self == DEBUG_INSTRS_A::DEBUG_INSTRS_1
    }
}
#[doc = "Field `DIVIDE_INSTRS` reader - Indicates the supported Divide instructions"]
pub type DIVIDE_INSTRS_R = crate::FieldReader<u8, DIVIDE_INSTRS_A>;
#[doc = "Indicates the supported Divide instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVIDE_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    DIVIDE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SDIV and UDIV instructions"]
    DIVIDE_INSTRS_1 = 1,
}
impl From<DIVIDE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVIDE_INSTRS_A) -> Self {
        variant as _
    }
}
impl DIVIDE_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVIDE_INSTRS_A> {
        match self.bits {
            0 => Some(DIVIDE_INSTRS_A::DIVIDE_INSTRS_0),
            1 => Some(DIVIDE_INSTRS_A::DIVIDE_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_divide_instrs_0(&self) -> bool {
        *self == DIVIDE_INSTRS_A::DIVIDE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `DIVIDE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_divide_instrs_1(&self) -> bool {
        *self == DIVIDE_INSTRS_A::DIVIDE_INSTRS_1
    }
}
impl R {
    #[doc = "Bits 4:7 - Indicates the supported Bit Counting instructions"]
    #[inline(always)]
    pub fn bitcount_instrs(&self) -> BITCOUNT_INSTRS_R {
        BITCOUNT_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the supported BitField instructions"]
    #[inline(always)]
    pub fn bitfield_instrs(&self) -> BITFIELD_INSTRS_R {
        BITFIELD_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the supported combined Compare and Branch instructions"]
    #[inline(always)]
    pub fn cmpbranch_instrs(&self) -> CMPBRANCH_INSTRS_R {
        CMPBRANCH_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported Coprocessor instructions"]
    #[inline(always)]
    pub fn coproc_instrs(&self) -> COPROC_INSTRS_R {
        COPROC_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the supported Debug instructions"]
    #[inline(always)]
    pub fn debug_instrs(&self) -> DEBUG_INSTRS_R {
        DEBUG_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported Divide instructions"]
    #[inline(always)]
    pub fn divide_instrs(&self) -> DIVIDE_INSTRS_R {
        DIVIDE_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Instruction Set Attributes Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar0](index.html) module"]
pub struct ID_ISAR0_SPEC;
impl crate::RegisterSpec for ID_ISAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_isar0::R](R) reader structure"]
impl crate::Readable for ID_ISAR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_ISAR0 to value 0"]
impl crate::Resettable for ID_ISAR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
