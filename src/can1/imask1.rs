#[doc = "Register `IMASK1` reader"]
pub struct R(crate::R<IMASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK1` writer"]
pub struct W(crate::W<IMASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK1_SPEC>;
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
impl From<crate::W<IMASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFLM` reader - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
pub type BUFLM_R = crate::FieldReader<u32, BUFLM_A>;
#[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BUFLM_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled"]
    BUFLM_0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled"]
    BUFLM_1 = 1,
}
impl From<BUFLM_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFLM_A) -> Self {
        variant as _
    }
}
impl BUFLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUFLM_A> {
        match self.bits {
            0 => Some(BUFLM_A::BUFLM_0),
            1 => Some(BUFLM_A::BUFLM_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUFLM_0`"]
    #[inline(always)]
    pub fn is_buflm_0(&self) -> bool {
        *self == BUFLM_A::BUFLM_0
    }
    #[doc = "Checks if the value of the field is `BUFLM_1`"]
    #[inline(always)]
    pub fn is_buflm_1(&self) -> bool {
        *self == BUFLM_A::BUFLM_1
    }
}
#[doc = "Field `BUFLM` writer - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
pub type BUFLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMASK1_SPEC, u32, BUFLM_A, 32, O>;
impl<'a, const O: u8> BUFLM_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    #[inline(always)]
    pub fn buflm_0(self) -> &'a mut W {
        self.variant(BUFLM_A::BUFLM_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled"]
    #[inline(always)]
    pub fn buflm_1(self) -> &'a mut W {
        self.variant(BUFLM_A::BUFLM_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[inline(always)]
    pub fn buflm(&self) -> BUFLM_R {
        BUFLM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buflm(&mut self) -> BUFLM_W<0> {
        BUFLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Masks 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask1](index.html) module"]
pub struct IMASK1_SPEC;
impl crate::RegisterSpec for IMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask1::R](R) reader structure"]
impl crate::Readable for IMASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask1::W](W) writer structure"]
impl crate::Writable for IMASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMASK1 to value 0"]
impl crate::Resettable for IMASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
