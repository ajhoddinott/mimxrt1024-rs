#[doc = "Register `CH1CMDPTR` reader"]
pub struct R(crate::R<CH1CMDPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CMDPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CMDPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CMDPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CMDPTR` writer"]
pub struct W(crate::W<CH1CMDPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CMDPTR_SPEC>;
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
impl From<crate::W<CH1CMDPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CMDPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Pointer to the descriptor structure to be processed for channel 1."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Pointer to the descriptor structure to be processed for channel 1."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CMDPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the descriptor structure to be processed for channel 1."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the descriptor structure to be processed for channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP channel 1 command pointer address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cmdptr](index.html) module"]
pub struct CH1CMDPTR_SPEC;
impl crate::RegisterSpec for CH1CMDPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1cmdptr::R](R) reader structure"]
impl crate::Readable for CH1CMDPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1cmdptr::W](W) writer structure"]
impl crate::Writable for CH1CMDPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CMDPTR to value 0"]
impl crate::Resettable for CH1CMDPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
