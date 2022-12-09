#[doc = "Register `IMASK2` reader"]
pub struct R(crate::R<IMASK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMASK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMASK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMASK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMASK2` writer"]
pub struct W(crate::W<IMASK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMASK2_SPEC>;
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
impl From<crate::W<IMASK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMASK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFHM` reader - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
pub type BUFHM_R = crate::FieldReader<u32, BUFHM_A>;
#[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BUFHM_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled"]
    BUFHM_0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled"]
    BUFHM_1 = 1,
}
impl From<BUFHM_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFHM_A) -> Self {
        variant as _
    }
}
impl BUFHM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUFHM_A> {
        match self.bits {
            0 => Some(BUFHM_A::BUFHM_0),
            1 => Some(BUFHM_A::BUFHM_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUFHM_0`"]
    #[inline(always)]
    pub fn is_bufhm_0(&self) -> bool {
        *self == BUFHM_A::BUFHM_0
    }
    #[doc = "Checks if the value of the field is `BUFHM_1`"]
    #[inline(always)]
    pub fn is_bufhm_1(&self) -> bool {
        *self == BUFHM_A::BUFHM_1
    }
}
#[doc = "Field `BUFHM` writer - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
pub type BUFHM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMASK2_SPEC, u32, BUFHM_A, 32, O>;
impl<'a, const O: u8> BUFHM_W<'a, O> {
    #[doc = "The corresponding buffer Interrupt is disabled"]
    #[inline(always)]
    pub fn bufhm_0(self) -> &'a mut W {
        self.variant(BUFHM_A::BUFHM_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled"]
    #[inline(always)]
    pub fn bufhm_1(self) -> &'a mut W {
        self.variant(BUFHM_A::BUFHM_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline(always)]
    pub fn bufhm(&self) -> BUFHM_R {
        BUFHM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bufhm(&mut self) -> BUFHM_W<0> {
        BUFHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Masks 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask2](index.html) module"]
pub struct IMASK2_SPEC;
impl crate::RegisterSpec for IMASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imask2::R](R) reader structure"]
impl crate::Readable for IMASK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imask2::W](W) writer structure"]
impl crate::Writable for IMASK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMASK2 to value 0"]
impl crate::Resettable for IMASK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
