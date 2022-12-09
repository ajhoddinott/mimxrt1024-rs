#[doc = "Register `ID_PFR0` reader"]
pub struct R(crate::R<ID_PFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE0` reader - ARM instruction set support"]
pub type STATE0_R = crate::FieldReader<u8, STATE0_A>;
#[doc = "ARM instruction set support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE0_A {
    #[doc = "0: ARMv7-M unused"]
    STATE0_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    STATE0_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    STATE0_2 = 2,
    #[doc = "3: Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions."]
    STATE0_3 = 3,
}
impl From<STATE0_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE0_A) -> Self {
        variant as _
    }
}
impl STATE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE0_A> {
        match self.bits {
            0 => Some(STATE0_A::STATE0_0),
            1 => Some(STATE0_A::STATE0_1),
            2 => Some(STATE0_A::STATE0_2),
            3 => Some(STATE0_A::STATE0_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATE0_0`"]
    #[inline(always)]
    pub fn is_state0_0(&self) -> bool {
        *self == STATE0_A::STATE0_0
    }
    #[doc = "Checks if the value of the field is `STATE0_1`"]
    #[inline(always)]
    pub fn is_state0_1(&self) -> bool {
        *self == STATE0_A::STATE0_1
    }
    #[doc = "Checks if the value of the field is `STATE0_2`"]
    #[inline(always)]
    pub fn is_state0_2(&self) -> bool {
        *self == STATE0_A::STATE0_2
    }
    #[doc = "Checks if the value of the field is `STATE0_3`"]
    #[inline(always)]
    pub fn is_state0_3(&self) -> bool {
        *self == STATE0_A::STATE0_3
    }
}
#[doc = "Field `STATE1` reader - Thumb instruction set support"]
pub type STATE1_R = crate::FieldReader<u8, STATE1_A>;
#[doc = "Thumb instruction set support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE1_A {
    #[doc = "0: The processor does not support the ARM instruction set."]
    STATE1_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    STATE1_1 = 1,
}
impl From<STATE1_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE1_A) -> Self {
        variant as _
    }
}
impl STATE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE1_A> {
        match self.bits {
            0 => Some(STATE1_A::STATE1_0),
            1 => Some(STATE1_A::STATE1_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATE1_0`"]
    #[inline(always)]
    pub fn is_state1_0(&self) -> bool {
        *self == STATE1_A::STATE1_0
    }
    #[doc = "Checks if the value of the field is `STATE1_1`"]
    #[inline(always)]
    pub fn is_state1_1(&self) -> bool {
        *self == STATE1_A::STATE1_1
    }
}
#[doc = "Field `STATE2` reader - ARMv7-M unused"]
pub type STATE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE3` reader - ARMv7-M unused"]
pub type STATE3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - ARM instruction set support"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Thumb instruction set support"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ARMv7-M unused"]
    #[inline(always)]
    pub fn state2(&self) -> STATE2_R {
        STATE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ARMv7-M unused"]
    #[inline(always)]
    pub fn state3(&self) -> STATE3_R {
        STATE3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Processor Feature Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr0](index.html) module"]
pub struct ID_PFR0_SPEC;
impl crate::RegisterSpec for ID_PFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr0::R](R) reader structure"]
impl crate::Readable for ID_PFR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_PFR0 to value 0"]
impl crate::Resettable for ID_PFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
