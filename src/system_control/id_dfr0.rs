#[doc = "Register `ID_DFR0` reader"]
pub struct R(crate::R<ID_DFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_DFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_DFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_DFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEBUGMODEL` reader - Support for memory-mapped debug model for M profile processors"]
pub type DEBUGMODEL_R = crate::FieldReader<u8, DEBUGMODEL_A>;
#[doc = "Support for memory-mapped debug model for M profile processors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBUGMODEL_A {
    #[doc = "0: Not supported"]
    DEBUGMODEL_0 = 0,
    #[doc = "1: Support for M profile Debug architecture, with memory-mapped access."]
    DEBUGMODEL_1 = 1,
}
impl From<DEBUGMODEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUGMODEL_A) -> Self {
        variant as _
    }
}
impl DEBUGMODEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEBUGMODEL_A> {
        match self.bits {
            0 => Some(DEBUGMODEL_A::DEBUGMODEL_0),
            1 => Some(DEBUGMODEL_A::DEBUGMODEL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_0`"]
    #[inline(always)]
    pub fn is_debugmodel_0(&self) -> bool {
        *self == DEBUGMODEL_A::DEBUGMODEL_0
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_1`"]
    #[inline(always)]
    pub fn is_debugmodel_1(&self) -> bool {
        *self == DEBUGMODEL_A::DEBUGMODEL_1
    }
}
impl R {
    #[doc = "Bits 20:23 - Support for memory-mapped debug model for M profile processors"]
    #[inline(always)]
    pub fn debugmodel(&self) -> DEBUGMODEL_R {
        DEBUGMODEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Debug Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_dfr0](index.html) module"]
pub struct ID_DFR0_SPEC;
impl crate::RegisterSpec for ID_DFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_dfr0::R](R) reader structure"]
impl crate::Readable for ID_DFR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_DFR0 to value 0"]
impl crate::Resettable for ID_DFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
