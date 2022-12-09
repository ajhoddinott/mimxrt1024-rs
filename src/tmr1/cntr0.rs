#[doc = "Register `CNTR0` reader"]
pub struct R(crate::R<CNTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR0` writer"]
pub struct W(crate::W<CNTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR0_SPEC>;
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
impl From<crate::W<CNTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - COUNTER"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNTER` writer - COUNTER"]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CNTR0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - COUNTER"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - COUNTER"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr0](index.html) module"]
pub struct CNTR0_SPEC;
impl crate::RegisterSpec for CNTR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cntr0::R](R) reader structure"]
impl crate::Readable for CNTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr0::W](W) writer structure"]
impl crate::Writable for CNTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTR0 to value 0"]
impl crate::Resettable for CNTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
