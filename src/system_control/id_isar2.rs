#[doc = "Register `ID_ISAR2` reader"]
pub struct R(crate::R<ID_ISAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_ISAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_ISAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_ISAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOADSTORE_INSTRS` reader - Indicates the supported additional load and store instructions"]
pub type LOADSTORE_INSTRS_R = crate::FieldReader<u8, LOADSTORE_INSTRS_A>;
#[doc = "Indicates the supported additional load and store instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOADSTORE_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    LOADSTORE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the LDRD and STRD instructions"]
    LOADSTORE_INSTRS_1 = 1,
}
impl From<LOADSTORE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADSTORE_INSTRS_A) -> Self {
        variant as _
    }
}
impl LOADSTORE_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOADSTORE_INSTRS_A> {
        match self.bits {
            0 => Some(LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_0),
            1 => Some(LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_loadstore_instrs_0(&self) -> bool {
        *self == LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_loadstore_instrs_1(&self) -> bool {
        *self == LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_1
    }
}
#[doc = "Field `MEMHINT_INSTRS` reader - Indicates the supported Memory Hint instructions"]
pub type MEMHINT_INSTRS_R = crate::FieldReader<u8, MEMHINT_INSTRS_A>;
#[doc = "Indicates the supported Memory Hint instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEMHINT_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    MEMHINT_INSTRS_0 = 0,
    #[doc = "1: Adds support for the PLD instruction, ARMv7-M unused."]
    MEMHINT_INSTRS_1 = 1,
    #[doc = "2: As for 1, ARMv7-M unused."]
    MEMHINT_INSTRS_2 = 2,
    #[doc = "3: As for 1 or 2, and adds support for the PLI instruction."]
    MEMHINT_INSTRS_3 = 3,
}
impl From<MEMHINT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MEMHINT_INSTRS_A) -> Self {
        variant as _
    }
}
impl MEMHINT_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEMHINT_INSTRS_A> {
        match self.bits {
            0 => Some(MEMHINT_INSTRS_A::MEMHINT_INSTRS_0),
            1 => Some(MEMHINT_INSTRS_A::MEMHINT_INSTRS_1),
            2 => Some(MEMHINT_INSTRS_A::MEMHINT_INSTRS_2),
            3 => Some(MEMHINT_INSTRS_A::MEMHINT_INSTRS_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_memhint_instrs_0(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_memhint_instrs_1(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_memhint_instrs_2(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_3`"]
    #[inline(always)]
    pub fn is_memhint_instrs_3(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_3
    }
}
#[doc = "Field `MULTIACCESSINT_INSTRS` reader - Indicates the support for multi-access interruptible instructions"]
pub type MULTIACCESSINT_INSTRS_R = crate::FieldReader<u8, MULTIACCESSINT_INSTRS_A>;
#[doc = "Indicates the support for multi-access interruptible instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTIACCESSINT_INSTRS_A {
    #[doc = "0: None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused."]
    MULTIACCESSINT_INSTRS_0 = 0,
    #[doc = "1: LDM and STM instructions are restartable."]
    MULTIACCESSINT_INSTRS_1 = 1,
    #[doc = "2: LDM and STM instructions are continuable."]
    MULTIACCESSINT_INSTRS_2 = 2,
}
impl From<MULTIACCESSINT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTIACCESSINT_INSTRS_A) -> Self {
        variant as _
    }
}
impl MULTIACCESSINT_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULTIACCESSINT_INSTRS_A> {
        match self.bits {
            0 => Some(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_0),
            1 => Some(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_1),
            2 => Some(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_0(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_1(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_2(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_2
    }
}
#[doc = "Field `MULT_INSTRS` reader - Indicates the supported additional Multiply instructions"]
pub type MULT_INSTRS_R = crate::FieldReader<u8, MULT_INSTRS_A>;
#[doc = "Indicates the supported additional Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULT_INSTRS_A {
    #[doc = "0: None supported. This means only MUL is supported. ARMv7-M unused."]
    MULT_INSTRS_0 = 0,
    #[doc = "1: Adds support for the MLA instruction, ARMv7-M unused."]
    MULT_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the MLS instruction."]
    MULT_INSTRS_2 = 2,
}
impl From<MULT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_INSTRS_A) -> Self {
        variant as _
    }
}
impl MULT_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULT_INSTRS_A> {
        match self.bits {
            0 => Some(MULT_INSTRS_A::MULT_INSTRS_0),
            1 => Some(MULT_INSTRS_A::MULT_INSTRS_1),
            2 => Some(MULT_INSTRS_A::MULT_INSTRS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_mult_instrs_0(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_mult_instrs_1(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_mult_instrs_2(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_2
    }
}
#[doc = "Field `MULTS_INSTRS` reader - Indicates the supported advanced signed Multiply instructions"]
pub type MULTS_INSTRS_R = crate::FieldReader<u8, MULTS_INSTRS_A>;
#[doc = "Indicates the supported advanced signed Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTS_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    MULTS_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SMULL and SMLAL instructions"]
    MULTS_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions."]
    MULTS_INSTRS_2 = 2,
    #[doc = "3: As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions."]
    MULTS_INSTRS_3 = 3,
}
impl From<MULTS_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTS_INSTRS_A) -> Self {
        variant as _
    }
}
impl MULTS_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULTS_INSTRS_A> {
        match self.bits {
            0 => Some(MULTS_INSTRS_A::MULTS_INSTRS_0),
            1 => Some(MULTS_INSTRS_A::MULTS_INSTRS_1),
            2 => Some(MULTS_INSTRS_A::MULTS_INSTRS_2),
            3 => Some(MULTS_INSTRS_A::MULTS_INSTRS_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_0`"]
    #[inline(always)]
    pub fn is_mults_instrs_0(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_1`"]
    #[inline(always)]
    pub fn is_mults_instrs_1(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_2`"]
    #[inline(always)]
    pub fn is_mults_instrs_2(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_3`"]
    #[inline(always)]
    pub fn is_mults_instrs_3(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_3
    }
}
#[doc = "Field `MULTU_INSTRS` reader - Indicates the supported advanced unsigned Multiply instructions"]
pub type MULTU_INSTRS_R = crate::FieldReader<u8, MULTU_INSTRS_A>;
#[doc = "Indicates the supported advanced unsigned Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTU_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    MULTU_INSTRS_0 = 0,
    #[doc = "1: Adds support for the UMULL and UMLAL instructions."]
    MULTU_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the UMAAL instruction."]
    MULTU_INSTRS_2 = 2,
}
impl From<MULTU_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTU_INSTRS_A) -> Self {
        variant as _
    }
}
impl MULTU_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MULTU_INSTRS_A> {
        match self.bits {
            0 => Some(MULTU_INSTRS_A::MULTU_INSTRS_0),
            1 => Some(MULTU_INSTRS_A::MULTU_INSTRS_1),
            2 => Some(MULTU_INSTRS_A::MULTU_INSTRS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_0`"]
    #[inline(always)]
    pub fn is_multu_instrs_0(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_1`"]
    #[inline(always)]
    pub fn is_multu_instrs_1(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_2`"]
    #[inline(always)]
    pub fn is_multu_instrs_2(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_2
    }
}
#[doc = "Field `REVERSAL_INSTRS` reader - Indicates the supported Reversal instructions"]
pub type REVERSAL_INSTRS_R = crate::FieldReader<u8, REVERSAL_INSTRS_A>;
#[doc = "Indicates the supported Reversal instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REVERSAL_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    REVERSAL_INSTRS_0 = 0,
    #[doc = "1: Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused."]
    REVERSAL_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the RBIT instruction."]
    REVERSAL_INSTRS_2 = 2,
}
impl From<REVERSAL_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: REVERSAL_INSTRS_A) -> Self {
        variant as _
    }
}
impl REVERSAL_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REVERSAL_INSTRS_A> {
        match self.bits {
            0 => Some(REVERSAL_INSTRS_A::REVERSAL_INSTRS_0),
            1 => Some(REVERSAL_INSTRS_A::REVERSAL_INSTRS_1),
            2 => Some(REVERSAL_INSTRS_A::REVERSAL_INSTRS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_0`"]
    #[inline(always)]
    pub fn is_reversal_instrs_0(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_1`"]
    #[inline(always)]
    pub fn is_reversal_instrs_1(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_2`"]
    #[inline(always)]
    pub fn is_reversal_instrs_2(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_2
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the supported additional load and store instructions"]
    #[inline(always)]
    pub fn loadstore_instrs(&self) -> LOADSTORE_INSTRS_R {
        LOADSTORE_INSTRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the supported Memory Hint instructions"]
    #[inline(always)]
    pub fn memhint_instrs(&self) -> MEMHINT_INSTRS_R {
        MEMHINT_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the support for multi-access interruptible instructions"]
    #[inline(always)]
    pub fn multiaccessint_instrs(&self) -> MULTIACCESSINT_INSTRS_R {
        MULTIACCESSINT_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the supported additional Multiply instructions"]
    #[inline(always)]
    pub fn mult_instrs(&self) -> MULT_INSTRS_R {
        MULT_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported advanced signed Multiply instructions"]
    #[inline(always)]
    pub fn mults_instrs(&self) -> MULTS_INSTRS_R {
        MULTS_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the supported advanced unsigned Multiply instructions"]
    #[inline(always)]
    pub fn multu_instrs(&self) -> MULTU_INSTRS_R {
        MULTU_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates the supported Reversal instructions"]
    #[inline(always)]
    pub fn reversal_instrs(&self) -> REVERSAL_INSTRS_R {
        REVERSAL_INSTRS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Instruction Set Attributes Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar2](index.html) module"]
pub struct ID_ISAR2_SPEC;
impl crate::RegisterSpec for ID_ISAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_isar2::R](R) reader structure"]
impl crate::Readable for ID_ISAR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_ISAR2 to value 0"]
impl crate::Resettable for ID_ISAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
