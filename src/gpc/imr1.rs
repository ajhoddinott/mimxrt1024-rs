#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMR1` reader - IRQ\\[31:0\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IMR1` writer - IRQ\\[31:0\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[31:0\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr1(&self) -> IMR1_R {
        IMR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[31:0\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn imr1(&mut self) -> IMR1_W<0> {
        IMR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ masking register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
