#[doc = "Register `CH3SEMA` reader"]
pub struct R(crate::R<CH3SEMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3SEMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3SEMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3SEMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3SEMA` writer"]
pub struct W(crate::W<CH3SEMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3SEMA_SPEC>;
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
impl From<crate::W<CH3SEMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3SEMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCREMENT` reader - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
pub type INCREMENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INCREMENT` writer - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
pub type INCREMENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3SEMA_SPEC, u8, u8, 8, O>;
#[doc = "Field `VALUE` reader - This read-only field shows the current (instantaneous) value of the semaphore counter."]
pub type VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub fn increment(&self) -> INCREMENT_R {
        INCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    #[must_use]
    pub fn increment(&mut self) -> INCREMENT_W<0> {
        INCREMENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP channel 3 semaphore register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3sema](index.html) module"]
pub struct CH3SEMA_SPEC;
impl crate::RegisterSpec for CH3SEMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3sema::R](R) reader structure"]
impl crate::Readable for CH3SEMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3sema::W](W) writer structure"]
impl crate::Writable for CH3SEMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3SEMA to value 0"]
impl crate::Resettable for CH3SEMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
