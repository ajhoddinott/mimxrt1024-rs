#[doc = "Register `DIGPROG` reader"]
pub struct R(crate::R<DIGPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIGPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIGPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIGPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SILICON_REVISION` reader - Chip silicon revision"]
pub type SILICON_REVISION_R = crate::FieldReader<u32, SILICON_REVISION_A>;
#[doc = "Chip silicon revision\n\nValue on reset: 7012352"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SILICON_REVISION_A {
    #[doc = "7012352: Silicon revision 1.0"]
    SILICON_REVISION_7012352 = 7012352,
}
impl From<SILICON_REVISION_A> for u32 {
    #[inline(always)]
    fn from(variant: SILICON_REVISION_A) -> Self {
        variant as _
    }
}
impl SILICON_REVISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SILICON_REVISION_A> {
        match self.bits {
            7012352 => Some(SILICON_REVISION_A::SILICON_REVISION_7012352),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SILICON_REVISION_7012352`"]
    #[inline(always)]
    pub fn is_silicon_revision_7012352(&self) -> bool {
        *self == SILICON_REVISION_A::SILICON_REVISION_7012352
    }
}
impl R {
    #[doc = "Bits 0:31 - Chip silicon revision"]
    #[inline(always)]
    pub fn silicon_revision(&self) -> SILICON_REVISION_R {
        SILICON_REVISION_R::new(self.bits)
    }
}
#[doc = "Chip Silicon Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digprog](index.html) module"]
pub struct DIGPROG_SPEC;
impl crate::RegisterSpec for DIGPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [digprog::R](R) reader structure"]
impl crate::Readable for DIGPROG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIGPROG to value 0x006b_0000"]
impl crate::Resettable for DIGPROG_SPEC {
    const RESET_VALUE: Self::Ux = 0x006b_0000;
}
