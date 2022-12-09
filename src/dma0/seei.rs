#[doc = "Register `SEEI` reader"]
pub struct R(crate::R<SEEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEEI` writer"]
pub struct W(crate::W<SEEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEEI_SPEC>;
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
impl From<crate::W<SEEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEEI` reader - Set Enable Error Interrupt"]
pub type SEEI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEEI` writer - Set Enable Error Interrupt"]
pub type SEEI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SEEI_SPEC, u8, u8, 5, O>;
#[doc = "Field `SAEE` reader - Set All Enable Error Interrupts"]
pub type SAEE_R = crate::BitReader<SAEE_A>;
#[doc = "Set All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAEE_A {
    #[doc = "0: Write 1 only to the EEI field specified in the SEEI field"]
    SET_EEI = 0,
    #[doc = "1: Writes 1 to all fields in EEI"]
    SET_ALL = 1,
}
impl From<SAEE_A> for bool {
    #[inline(always)]
    fn from(variant: SAEE_A) -> Self {
        variant as u8 != 0
    }
}
impl SAEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEE_A {
        match self.bits {
            false => SAEE_A::SET_EEI,
            true => SAEE_A::SET_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `SET_EEI`"]
    #[inline(always)]
    pub fn is_set_eei(&self) -> bool {
        *self == SAEE_A::SET_EEI
    }
    #[doc = "Checks if the value of the field is `SET_ALL`"]
    #[inline(always)]
    pub fn is_set_all(&self) -> bool {
        *self == SAEE_A::SET_ALL
    }
}
#[doc = "Field `SAEE` writer - Set All Enable Error Interrupts"]
pub type SAEE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEEI_SPEC, SAEE_A, O>;
impl<'a, const O: u8> SAEE_W<'a, O> {
    #[doc = "Write 1 only to the EEI field specified in the SEEI field"]
    #[inline(always)]
    pub fn set_eei(self) -> &'a mut W {
        self.variant(SAEE_A::SET_EEI)
    }
    #[doc = "Writes 1 to all fields in EEI"]
    #[inline(always)]
    pub fn set_all(self) -> &'a mut W {
        self.variant(SAEE_A::SET_ALL)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SEEI_SPEC, NOP_A, O>;
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
    #[doc = "Bits 0:4 - Set Enable Error Interrupt"]
    #[inline(always)]
    pub fn seei(&self) -> SEEI_R {
        SEEI_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Set All Enable Error Interrupts"]
    #[inline(always)]
    pub fn saee(&self) -> SAEE_R {
        SAEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set Enable Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn seei(&mut self) -> SEEI_W<0> {
        SEEI_W::new(self)
    }
    #[doc = "Bit 6 - Set All Enable Error Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn saee(&mut self) -> SAEE_W<6> {
        SAEE_W::new(self)
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
#[doc = "Set Enable Error Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seei](index.html) module"]
pub struct SEEI_SPEC;
impl crate::RegisterSpec for SEEI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seei::R](R) reader structure"]
impl crate::Readable for SEEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seei::W](W) writer structure"]
impl crate::Writable for SEEI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEEI to value 0"]
impl crate::Resettable for SEEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
