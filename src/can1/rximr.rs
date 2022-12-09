#[doc = "Register `RXIMR%s` reader"]
pub struct R(crate::R<RXIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIMR%s` writer"]
pub struct W(crate::W<RXIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIMR_SPEC>;
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
impl From<crate::W<RXIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MI` reader - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
pub type MI_R = crate::FieldReader<u32, MI_A>;
#[doc = "These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum MI_A {
    #[doc = "0: the corresponding bit in the filter is \"don't care\""]
    MI_0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    MI_1 = 1,
}
impl From<MI_A> for u32 {
    #[inline(always)]
    fn from(variant: MI_A) -> Self {
        variant as _
    }
}
impl MI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MI_A> {
        match self.bits {
            0 => Some(MI_A::MI_0),
            1 => Some(MI_A::MI_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MI_0`"]
    #[inline(always)]
    pub fn is_mi_0(&self) -> bool {
        *self == MI_A::MI_0
    }
    #[doc = "Checks if the value of the field is `MI_1`"]
    #[inline(always)]
    pub fn is_mi_1(&self) -> bool {
        *self == MI_A::MI_1
    }
}
#[doc = "Field `MI` writer - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
pub type MI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXIMR_SPEC, u32, MI_A, 32, O>;
impl<'a, const O: u8> MI_W<'a, O> {
    #[doc = "the corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn mi_0(self) -> &'a mut W {
        self.variant(MI_A::MI_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn mi_1(self) -> &'a mut W {
        self.variant(MI_A::MI_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[inline(always)]
    #[must_use]
    pub fn mi(&mut self) -> MI_W<0> {
        MI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rximr](index.html) module"]
pub struct RXIMR_SPEC;
impl crate::RegisterSpec for RXIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rximr::R](R) reader structure"]
impl crate::Readable for RXIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rximr::W](W) writer structure"]
impl crate::Writable for RXIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXIMR%s to value 0"]
impl crate::Resettable for RXIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
