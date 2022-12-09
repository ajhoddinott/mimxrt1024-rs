#[doc = "Register `IMR5` reader"]
pub struct R(crate::R<IMR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR5` writer"]
pub struct W(crate::W<IMR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR5_SPEC>;
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
impl From<crate::W<IMR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMR5` reader - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IMR5` writer - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr5(&self) -> IMR5_R {
        IMR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn imr5(&mut self) -> IMR5_W<0> {
        IMR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ masking register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr5](index.html) module"]
pub struct IMR5_SPEC;
impl crate::RegisterSpec for IMR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr5::R](R) reader structure"]
impl crate::Readable for IMR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr5::W](W) writer structure"]
impl crate::Writable for IMR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR5 to value 0"]
impl crate::Resettable for IMR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
