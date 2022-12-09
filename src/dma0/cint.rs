#[doc = "Register `CINT` reader"]
pub struct R(crate::R<CINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CINT` writer"]
pub struct W(crate::W<CINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CINT_SPEC>;
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
impl From<crate::W<CINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CINT` reader - Clear Interrupt Request"]
pub type CINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CINT` writer - Clear Interrupt Request"]
pub type CINT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CINT_SPEC, u8, u8, 5, O>;
#[doc = "Field `CAIR` reader - Clear All Interrupt Requests"]
pub type CAIR_R = crate::BitReader<CAIR_A>;
#[doc = "Clear All Interrupt Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAIR_A {
    #[doc = "0: Clear only the INT field specified in the CINT field"]
    CLEAR_INT = 0,
    #[doc = "1: Clear all bits in INT"]
    CLEAR_ALL = 1,
}
impl From<CAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CAIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CAIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAIR_A {
        match self.bits {
            false => CAIR_A::CLEAR_INT,
            true => CAIR_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_INT`"]
    #[inline(always)]
    pub fn is_clear_int(&self) -> bool {
        *self == CAIR_A::CLEAR_INT
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == CAIR_A::CLEAR_ALL
    }
}
#[doc = "Field `CAIR` writer - Clear All Interrupt Requests"]
pub type CAIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, CINT_SPEC, CAIR_A, O>;
impl<'a, const O: u8> CAIR_W<'a, O> {
    #[doc = "Clear only the INT field specified in the CINT field"]
    #[inline(always)]
    pub fn clear_int(self) -> &'a mut W {
        self.variant(CAIR_A::CLEAR_INT)
    }
    #[doc = "Clear all bits in INT"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(CAIR_A::CLEAR_ALL)
    }
}
#[doc = "Field `NOP` reader - No Op Enable"]
pub type NOP_R = crate::BitReader<NOP_A>;
#[doc = "No Op Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOP_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: No operation; all other fields in this register are ignored."]
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CINT_SPEC, NOP_A, O>;
impl<'a, const O: u8> NOP_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(NOP_A::NORMAL_OPS)
    }
    #[doc = "No operation; all other fields in this register are ignored."]
    #[inline(always)]
    pub fn no_ops(self) -> &'a mut W {
        self.variant(NOP_A::NO_OPS)
    }
}
impl R {
    #[doc = "Bits 0:4 - Clear Interrupt Request"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    pub fn cair(&self) -> CAIR_R {
        CAIR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<0> {
        CINT_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    #[must_use]
    pub fn cair(&mut self) -> CAIR_W<6> {
        CAIR_W::new(self)
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
#[doc = "Clear Interrupt Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cint](index.html) module"]
pub struct CINT_SPEC;
impl crate::RegisterSpec for CINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cint::R](R) reader structure"]
impl crate::Readable for CINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cint::W](W) writer structure"]
impl crate::Writable for CINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CINT to value 0"]
impl crate::Resettable for CINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
