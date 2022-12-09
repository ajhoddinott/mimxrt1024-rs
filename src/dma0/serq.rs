#[doc = "Register `SERQ` reader"]
pub struct R(crate::R<SERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERQ` writer"]
pub struct W(crate::W<SERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERQ_SPEC>;
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
impl From<crate::W<SERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERQ` reader - Set Enable Request"]
pub type SERQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SERQ` writer - Set Enable Request"]
pub type SERQ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SERQ_SPEC, u8, u8, 5, O>;
#[doc = "Field `SAER` reader - Set All Enable Requests"]
pub type SAER_R = crate::BitReader<SAER_A>;
#[doc = "Set All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAER_A {
    #[doc = "0: Write 1 to only the ERQ field specified in the SERQ field"]
    SET_ERQ = 0,
    #[doc = "1: Write 1 to all fields in ERQ"]
    SET_ALL = 1,
}
impl From<SAER_A> for bool {
    #[inline(always)]
    fn from(variant: SAER_A) -> Self {
        variant as u8 != 0
    }
}
impl SAER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAER_A {
        match self.bits {
            false => SAER_A::SET_ERQ,
            true => SAER_A::SET_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `SET_ERQ`"]
    #[inline(always)]
    pub fn is_set_erq(&self) -> bool {
        *self == SAER_A::SET_ERQ
    }
    #[doc = "Checks if the value of the field is `SET_ALL`"]
    #[inline(always)]
    pub fn is_set_all(&self) -> bool {
        *self == SAER_A::SET_ALL
    }
}
#[doc = "Field `SAER` writer - Set All Enable Requests"]
pub type SAER_W<'a, const O: u8> = crate::BitWriter<'a, u8, SERQ_SPEC, SAER_A, O>;
impl<'a, const O: u8> SAER_W<'a, O> {
    #[doc = "Write 1 to only the ERQ field specified in the SERQ field"]
    #[inline(always)]
    pub fn set_erq(self) -> &'a mut W {
        self.variant(SAER_A::SET_ERQ)
    }
    #[doc = "Write 1 to all fields in ERQ"]
    #[inline(always)]
    pub fn set_all(self) -> &'a mut W {
        self.variant(SAER_A::SET_ALL)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SERQ_SPEC, NOP_A, O>;
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
    #[doc = "Bits 0:4 - Set Enable Request"]
    #[inline(always)]
    pub fn serq(&self) -> SERQ_R {
        SERQ_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    pub fn saer(&self) -> SAER_R {
        SAER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn serq(&mut self) -> SERQ_W<0> {
        SERQ_W::new(self)
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn saer(&mut self) -> SAER_W<6> {
        SAER_W::new(self)
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
#[doc = "Set Enable Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serq](index.html) module"]
pub struct SERQ_SPEC;
impl crate::RegisterSpec for SERQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [serq::R](R) reader structure"]
impl crate::Readable for SERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [serq::W](W) writer structure"]
impl crate::Writable for SERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SERQ to value 0"]
impl crate::Resettable for SERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
