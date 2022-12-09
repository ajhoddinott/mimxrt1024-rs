#[doc = "Register `CERR` reader"]
pub struct R(crate::R<CERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CERR` writer"]
pub struct W(crate::W<CERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERR_SPEC>;
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
impl From<crate::W<CERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERR` reader - Clear Error Indicator"]
pub type CERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CERR` writer - Clear Error Indicator"]
pub type CERR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CERR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CAEI` reader - Clear All Error Indicators"]
pub type CAEI_R = crate::BitReader<CAEI_A>;
#[doc = "Clear All Error Indicators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEI_A {
    #[doc = "0: Write 0 to only the ERR field specified in the CERR field"]
    CLEAR_ERR = 0,
    #[doc = "1: Write 0 to all fields in ERR"]
    CLEAR_ALL = 1,
}
impl From<CAEI_A> for bool {
    #[inline(always)]
    fn from(variant: CAEI_A) -> Self {
        variant as u8 != 0
    }
}
impl CAEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEI_A {
        match self.bits {
            false => CAEI_A::CLEAR_ERR,
            true => CAEI_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_ERR`"]
    #[inline(always)]
    pub fn is_clear_err(&self) -> bool {
        *self == CAEI_A::CLEAR_ERR
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == CAEI_A::CLEAR_ALL
    }
}
#[doc = "Field `CAEI` writer - Clear All Error Indicators"]
pub type CAEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERR_SPEC, CAEI_A, O>;
impl<'a, const O: u8> CAEI_W<'a, O> {
    #[doc = "Write 0 to only the ERR field specified in the CERR field"]
    #[inline(always)]
    pub fn clear_err(self) -> &'a mut W {
        self.variant(CAEI_A::CLEAR_ERR)
    }
    #[doc = "Write 0 to all fields in ERR"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(CAEI_A::CLEAR_ALL)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERR_SPEC, NOP_A, O>;
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
    #[doc = "Bits 0:4 - Clear Error Indicator"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    pub fn caei(&self) -> CAEI_R {
        CAEI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Error Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<0> {
        CERR_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    #[must_use]
    pub fn caei(&mut self) -> CAEI_W<6> {
        CAEI_W::new(self)
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
#[doc = "Clear Error\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](index.html) module"]
pub struct CERR_SPEC;
impl crate::RegisterSpec for CERR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cerr::R](R) reader structure"]
impl crate::Readable for CERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cerr::W](W) writer structure"]
impl crate::Writable for CERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CERR to value 0"]
impl crate::Resettable for CERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
