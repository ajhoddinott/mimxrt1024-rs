#[doc = "Register `ID_PFR1` reader"]
pub struct R(crate::R<ID_PFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROGMODEL` reader - M profile programmers' model"]
pub type PROGMODEL_R = crate::FieldReader<u8, PROGMODEL_A>;
#[doc = "M profile programmers' model\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROGMODEL_A {
    #[doc = "0: ARMv7-M unused"]
    PROGMODEL_0 = 0,
    #[doc = "2: Two-stack programmers' model supported"]
    PROGMODEL_2 = 2,
}
impl From<PROGMODEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PROGMODEL_A) -> Self {
        variant as _
    }
}
impl PROGMODEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROGMODEL_A> {
        match self.bits {
            0 => Some(PROGMODEL_A::PROGMODEL_0),
            2 => Some(PROGMODEL_A::PROGMODEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_0`"]
    #[inline(always)]
    pub fn is_progmodel_0(&self) -> bool {
        *self == PROGMODEL_A::PROGMODEL_0
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_2`"]
    #[inline(always)]
    pub fn is_progmodel_2(&self) -> bool {
        *self == PROGMODEL_A::PROGMODEL_2
    }
}
impl R {
    #[doc = "Bits 8:11 - M profile programmers' model"]
    #[inline(always)]
    pub fn progmodel(&self) -> PROGMODEL_R {
        PROGMODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Processor Feature Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr1](index.html) module"]
pub struct ID_PFR1_SPEC;
impl crate::RegisterSpec for ID_PFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr1::R](R) reader structure"]
impl crate::Readable for ID_PFR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_PFR1 to value 0"]
impl crate::Resettable for ID_PFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
