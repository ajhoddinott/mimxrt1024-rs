#[doc = "Register `CDNE` reader"]
pub struct R(crate::R<CDNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDNE` writer"]
pub struct W(crate::W<CDNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDNE_SPEC>;
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
impl From<crate::W<CDNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDNE` reader - Clear DONE field"]
pub type CDNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CDNE` writer - Clear DONE field"]
pub type CDNE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CDNE_SPEC, u8, u8, 5, O>;
#[doc = "Field `CADN` reader - Clears All DONE fields"]
pub type CADN_R = crate::BitReader<CADN_A>;
#[doc = "Clears All DONE fields\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CADN_A {
    #[doc = "0: Writes 0 to only the TCDn_CSR\\[DONE\\]
field specified in the CDNE field"]
    CLEAR_DONE = 0,
    #[doc = "1: Writes 0 to all bits in TCDn_CSR\\[DONE\\]"]
    CLEAR_ALL = 1,
}
impl From<CADN_A> for bool {
    #[inline(always)]
    fn from(variant: CADN_A) -> Self {
        variant as u8 != 0
    }
}
impl CADN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CADN_A {
        match self.bits {
            false => CADN_A::CLEAR_DONE,
            true => CADN_A::CLEAR_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR_DONE`"]
    #[inline(always)]
    pub fn is_clear_done(&self) -> bool {
        *self == CADN_A::CLEAR_DONE
    }
    #[doc = "Checks if the value of the field is `CLEAR_ALL`"]
    #[inline(always)]
    pub fn is_clear_all(&self) -> bool {
        *self == CADN_A::CLEAR_ALL
    }
}
#[doc = "Field `CADN` writer - Clears All DONE fields"]
pub type CADN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CDNE_SPEC, CADN_A, O>;
impl<'a, const O: u8> CADN_W<'a, O> {
    #[doc = "Writes 0 to only the TCDn_CSR\\[DONE\\]
field specified in the CDNE field"]
    #[inline(always)]
    pub fn clear_done(self) -> &'a mut W {
        self.variant(CADN_A::CLEAR_DONE)
    }
    #[doc = "Writes 0 to all bits in TCDn_CSR\\[DONE\\]"]
    #[inline(always)]
    pub fn clear_all(self) -> &'a mut W {
        self.variant(CADN_A::CLEAR_ALL)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CDNE_SPEC, NOP_A, O>;
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
    #[doc = "Bits 0:4 - Clear DONE field"]
    #[inline(always)]
    pub fn cdne(&self) -> CDNE_R {
        CDNE_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Clears All DONE fields"]
    #[inline(always)]
    pub fn cadn(&self) -> CADN_R {
        CADN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear DONE field"]
    #[inline(always)]
    #[must_use]
    pub fn cdne(&mut self) -> CDNE_W<0> {
        CDNE_W::new(self)
    }
    #[doc = "Bit 6 - Clears All DONE fields"]
    #[inline(always)]
    #[must_use]
    pub fn cadn(&mut self) -> CADN_W<6> {
        CADN_W::new(self)
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
#[doc = "Clear DONE Status Bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](index.html) module"]
pub struct CDNE_SPEC;
impl crate::RegisterSpec for CDNE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cdne::R](R) reader structure"]
impl crate::Readable for CDNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdne::W](W) writer structure"]
impl crate::Writable for CDNE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDNE to value 0"]
impl crate::Resettable for CDNE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
