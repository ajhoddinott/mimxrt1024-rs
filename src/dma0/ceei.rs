#[doc = "Register `CEEI` reader"]
pub struct R(crate::R<CEEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEEI` writer"]
pub struct W(crate::W<CEEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEEI_SPEC>;
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
impl From<crate::W<CEEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEEI` reader - Clear Enable Error Interrupt"]
pub type CEEI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEEI` writer - Clear Enable Error Interrupt"]
pub type CEEI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CEEI_SPEC, u8, u8, 5, O>;
#[doc = "Field `CAEE` reader - Clear All Enable Error Interrupts"]
pub type CAEE_R = crate::BitReader<CAEE_A>;
#[doc = "Clear All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEE_A {
    #[doc = "0: Write 0 only to the EEI field specified in the CEEI field"]
    CLEAR_EEI = 0,
    #[doc = "1: Write 0 to all fields in EEI"]
    CLEAR_ALL = 1,
}
impl From<CAEE_A> for bool {
    #[inline(always)]
    fn from(variant: CAEE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEE_A {
        match self.bits {
            false => CAEE_A::CLEAR_EEI,
            true => CAEE_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_EEI`"]
    #[inline(always)]
    pub fn is_clear_eei(&self) -> bool {
        *self == CAEE_A::CLEAR_EEI
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == CAEE_A::CLEAR_ALL
    }
}
#[doc = "Field `CAEE` writer - Clear All Enable Error Interrupts"]
pub type CAEE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CEEI_SPEC, CAEE_A, O>;
impl<'a, const O: u8> CAEE_W<'a, O> {
    #[doc = "Write 0 only to the EEI field specified in the CEEI field"]
    #[inline(always)]
    pub fn clear_eei(self) -> &'a mut W {
        self.variant(CAEE_A::CLEAR_EEI)
    }
    #[doc = "Write 0 to all fields in EEI"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(CAEE_A::CLEAR_ALL)
    }
}
#[doc = "Field `NOP` reader - No Op Enable"]
pub type NOP_R = crate::BitReader<NOP_A>;
#[doc = "No Op Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOP_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: No operation, ignore the other fields in this register"]
    NO_OPS = 1,
}
impl From<NOP_A> for bool {
    #[inline(always)]
    fn from(variant: NOP_A) -> Self {
        variant as u8 != 0
    }
}
impl NOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOP_A {
        match self.bits {
            false => NOP_A::NORMAL_OPS,
            true => NOP_A::NO_OPS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == NOP_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `NO_OPS`"]
    #[inline(always)]
    pub fn is_no_ops(&self) -> bool {
        *self == NOP_A::NO_OPS
    }
}
#[doc = "Field `NOP` writer - No Op Enable"]
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CEEI_SPEC, NOP_A, O>;
impl<'a, const O: u8> NOP_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(NOP_A::NORMAL_OPS)
    }
    #[doc = "No operation, ignore the other fields in this register"]
    #[inline(always)]
    pub fn no_ops(self) -> &'a mut W {
        self.variant(NOP_A::NO_OPS)
    }
}
impl R {
    #[doc = "Bits 0:4 - Clear Enable Error Interrupt"]
    #[inline(always)]
    pub fn ceei(&self) -> CEEI_R {
        CEEI_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub fn caee(&self) -> CAEE_R {
        CAEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Enable Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ceei(&mut self) -> CEEI_W<0> {
        CEEI_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn caee(&mut self) -> CAEE_W<6> {
        CAEE_W::new(self)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NOP_W<7> {
        NOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Enable Error Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](index.html) module"]
pub struct CEEI_SPEC;
impl crate::RegisterSpec for CEEI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ceei::R](R) reader structure"]
impl crate::Readable for CEEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ceei::W](W) writer structure"]
impl crate::Writable for CEEI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEEI to value 0"]
impl crate::Resettable for CEEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
