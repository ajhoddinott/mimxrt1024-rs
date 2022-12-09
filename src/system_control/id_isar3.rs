#[doc = "Register `ID_ISAR3` reader"]
pub struct R(crate::R<ID_ISAR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_ISAR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_ISAR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_ISAR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SATURATE_INSTRS` reader - Indicates the supported Saturate instructions"]
pub type SATURATE_INSTRS_R = crate::FieldReader<u8, SATURATE_INSTRS_A>;
#[doc = "Indicates the supported Saturate instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SATURATE_INSTRS_A {
    #[doc = "0: None supported"]
    SATURATE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs."]
    SATURATE_INSTRS_1 = 1,
}
impl From<SATURATE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SATURATE_INSTRS_A) -> Self {
        variant as _
    }
}
impl SATURATE_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SATURATE_INSTRS_A> {
        match self.bits {
            0 => Some(SATURATE_INSTRS_A::SATURATE_INSTRS_0),
            1 => Some(SATURATE_INSTRS_A::SATURATE_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_saturate_instrs_0(&self) -> bool {
        *self == SATURATE_INSTRS_A::SATURATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_saturate_instrs_1(&self) -> bool {
        *self == SATURATE_INSTRS_A::SATURATE_INSTRS_1
    }
}
#[doc = "Field `SIMD_INSTRS` reader - Indicates the supported SIMD instructions"]
pub type SIMD_INSTRS_R = crate::FieldReader<u8, SIMD_INSTRS_A>;
#[doc = "Indicates the supported SIMD instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIMD_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    SIMD_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs."]
    SIMD_INSTRS_1 = 1,
    #[doc = "3: As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE\\[3:0\\]
bits in the PSRs."]
    SIMD_INSTRS_3 = 3,
}
impl From<SIMD_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SIMD_INSTRS_A) -> Self {
        variant as _
    }
}
impl SIMD_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIMD_INSTRS_A> {
        match self.bits {
            0 => Some(SIMD_INSTRS_A::SIMD_INSTRS_0),
            1 => Some(SIMD_INSTRS_A::SIMD_INSTRS_1),
            3 => Some(SIMD_INSTRS_A::SIMD_INSTRS_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_0`"]
    #[inline(always)]
    pub fn is_simd_instrs_0(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_1`"]
    #[inline(always)]
    pub fn is_simd_instrs_1(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_3`"]
    #[inline(always)]
    pub fn is_simd_instrs_3(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_3
    }
}
#[doc = "Field `SVC_INSTRS` reader - Indicates the supported SVC instructions"]
pub type SVC_INSTRS_R = crate::FieldReader<u8, SVC_INSTRS_A>;
#[doc = "Indicates the supported SVC instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVC_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    SVC_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SVC instruction."]
    SVC_INSTRS_1 = 1,
}
impl From<SVC_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVC_INSTRS_A) -> Self {
        variant as _
    }
}
impl SVC_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SVC_INSTRS_A> {
        match self.bits {
            0 => Some(SVC_INSTRS_A::SVC_INSTRS_0),
            1 => Some(SVC_INSTRS_A::SVC_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_0`"]
    #[inline(always)]
    pub fn is_svc_instrs_0(&self) -> bool {
        *self == SVC_INSTRS_A::SVC_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_1`"]
    #[inline(always)]
    pub fn is_svc_instrs_1(&self) -> bool {
        *self == SVC_INSTRS_A::SVC_INSTRS_1
    }
}
#[doc = "Field `SYNCHPRIM_INSTRS` reader - Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\]
indicates the supported Synchronization Primitives"]
pub type SYNCHPRIM_INSTRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TABBRANCH_INSTRS` reader - Indicates the supported Table Branch instructions"]
pub type TABBRANCH_INSTRS_R = crate::FieldReader<u8, TABBRANCH_INSTRS_A>;
#[doc = "Indicates the supported Table Branch instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TABBRANCH_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    TABBRANCH_INSTRS_0 = 0,
    #[doc = "1: Adds support for the TBB and TBH instructions."]
    TABBRANCH_INSTRS_1 = 1,
}
impl From<TABBRANCH_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: TABBRANCH_INSTRS_A) -> Self {
        variant as _
    }
}
impl TABBRANCH_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TABBRANCH_INSTRS_A> {
        match self.bits {
            0 => Some(TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_0),
            1 => Some(TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_0`"]
    #[inline(always)]
    pub fn is_tabbranch_instrs_0(&self) -> bool {
        *self == TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_1`"]
    #[inline(always)]
    pub fn is_tabbranch_instrs_1(&self) -> bool {
        *self == TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_1
    }
}
#[doc = "Field `THUMBCOPY_INSTRS` reader - Indicates the supported non flag-setting MOV instructions"]
pub type THUMBCOPY_INSTRS_R = crate::FieldReader<u8, THUMBCOPY_INSTRS_A>;
#[doc = "Indicates the supported non flag-setting MOV instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THUMBCOPY_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    THUMBCOPY_INSTRS_0 = 0,
    #[doc = "1: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    THUMBCOPY_INSTRS_1 = 1,
}
impl From<THUMBCOPY_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: THUMBCOPY_INSTRS_A) -> Self {
        variant as _
    }
}
impl THUMBCOPY_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<THUMBCOPY_INSTRS_A> {
        match self.bits {
            0 => Some(THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_0),
            1 => Some(THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_0`"]
    #[inline(always)]
    pub fn is_thumbcopy_instrs_0(&self) -> bool {
        *self == THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_1`"]
    #[inline(always)]
    pub fn is_thumbcopy_instrs_1(&self) -> bool {
        *self == THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_1
    }
}
#[doc = "Field `TRUENOP_INSTRS` reader - Indicates the supported non flag-setting MOV instructions"]
pub type TRUENOP_INSTRS_R = crate::FieldReader<u8, TRUENOP_INSTRS_A>;
#[doc = "Indicates the supported non flag-setting MOV instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRUENOP_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    TRUENOP_INSTRS_0 = 0,
    #[doc = "1: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    TRUENOP_INSTRS_1 = 1,
}
impl From<TRUENOP_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRUENOP_INSTRS_A) -> Self {
        variant as _
    }
}
impl TRUENOP_INSTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRUENOP_INSTRS_A> {
        match self.bits {
            0 => Some(TRUENOP_INSTRS_A::TRUENOP_INSTRS_0),
            1 => Some(TRUENOP_INSTRS_A::TRUENOP_INSTRS_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_0`"]
    #[inline(always)]
    pub fn is_truenop_instrs_0(&self) -> bool {
        *self == TRUENOP_INSTRS_A::TRUENOP_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_1`"]
    #[inline(always)]
    pub fn is_truenop_instrs_1(&self) -> bool {
        *self == TRUENOP_INSTRS_A::TRUENOP_INSTRS_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the supported Saturate instructions"]
    #[inline(always)]
    pub fn saturate_instrs(&self) -> SATURATE_INSTRS_R {
        SATURATE_INSTRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the supported SIMD instructions"]
    #[inline(always)]
    pub fn simd_instrs(&self) -> SIMD_INSTRS_R {
        SIMD_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the supported SVC instructions"]
    #[inline(always)]
    pub fn svc_instrs(&self) -> SVC_INSTRS_R {
        SVC_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\]
indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub fn synchprim_instrs(&self) -> SYNCHPRIM_INSTRS_R {
        SYNCHPRIM_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported Table Branch instructions"]
    #[inline(always)]
    pub fn tabbranch_instrs(&self) -> TABBRANCH_INSTRS_R {
        TABBRANCH_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub fn thumbcopy_instrs(&self) -> THUMBCOPY_INSTRS_R {
        THUMBCOPY_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub fn truenop_instrs(&self) -> TRUENOP_INSTRS_R {
        TRUENOP_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Instruction Set Attributes Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_isar3](index.html) module"]
pub struct ID_ISAR3_SPEC;
impl crate::RegisterSpec for ID_ISAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_isar3::R](R) reader structure"]
impl crate::Readable for ID_ISAR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_ISAR3 to value 0"]
impl crate::Resettable for ID_ISAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
