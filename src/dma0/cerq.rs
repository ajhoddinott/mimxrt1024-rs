#[doc = "Register `CERQ` reader"]
pub struct R(crate::R<CERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CERQ` writer"]
pub struct W(crate::W<CERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERQ_SPEC>;
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
impl From<crate::W<CERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERQ` reader - Clear Enable Request"]
pub type CERQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CERQ` writer - Clear Enable Request"]
pub type CERQ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CERQ_SPEC, u8, u8, 5, O>;
#[doc = "Field `CAER` reader - Clear All Enable Requests"]
pub type CAER_R = crate::BitReader<CAER_A>;
#[doc = "Clear All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAER_A {
    #[doc = "0: Write 0 to only the ERQ field specified in the CERQ field"]
    CLEAR_ERQ = 0,
    #[doc = "1: Write 0 to all fields in ERQ"]
    CLEAR_ALL = 1,
}
impl From<CAER_A> for bool {
    #[inline(always)]
    fn from(variant: CAER_A) -> Self {
        variant as u8 != 0
    }
}
impl CAER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAER_A {
        match self.bits {
            false => CAER_A::CLEAR_ERQ,
            true => CAER_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_ERQ`"]
    #[inline(always)]
    pub fn is_clear_erq(&self) -> bool {
        *self == CAER_A::CLEAR_ERQ
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == CAER_A::CLEAR_ALL
    }
}
#[doc = "Field `CAER` writer - Clear All Enable Requests"]
pub type CAER_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERQ_SPEC, CAER_A, O>;
impl<'a, const O: u8> CAER_W<'a, O> {
    #[doc = "Write 0 to only the ERQ field specified in the CERQ field"]
    #[inline(always)]
    pub fn clear_erq(self) -> &'a mut W {
        self.variant(CAER_A::CLEAR_ERQ)
    }
    #[doc = "Write 0 to all fields in ERQ"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(CAER_A::CLEAR_ALL)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERQ_SPEC, NOP_A, O>;
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
    #[doc = "Bits 0:4 - Clear Enable Request"]
    #[inline(always)]
    pub fn cerq(&self) -> CERQ_R {
        CERQ_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    pub fn caer(&self) -> CAER_R {
        CAER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn cerq(&mut self) -> CERQ_W<0> {
        CERQ_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn caer(&mut self) -> CAER_W<6> {
        CAER_W::new(self)
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
#[doc = "Clear Enable Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](index.html) module"]
pub struct CERQ_SPEC;
impl crate::RegisterSpec for CERQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cerq::R](R) reader structure"]
impl crate::Readable for CERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cerq::W](W) writer structure"]
impl crate::Writable for CERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CERQ to value 0"]
impl crate::Resettable for CERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
