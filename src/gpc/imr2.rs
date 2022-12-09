#[doc = "Register `IMR2` reader"]
pub struct R(crate::R<IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR2` writer"]
pub struct W(crate::W<IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR2_SPEC>;
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
impl From<crate::W<IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMR2` reader - IRQ\\[63:32\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IMR2` writer - IRQ\\[63:32\\]
masking bits: 1-irq masked, 0-irq is not masked"]
pub type IMR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[63:32\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    pub fn imr2(&self) -> IMR2_R {
        IMR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ\\[63:32\\]
masking bits: 1-irq masked, 0-irq is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn imr2(&mut self) -> IMR2_W<0> {
        IMR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ masking register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](index.html) module"]
pub struct IMR2_SPEC;
impl crate::RegisterSpec for IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr2::R](R) reader structure"]
impl crate::Readable for IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr2::W](W) writer structure"]
impl crate::Writable for IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0"]
impl crate::Resettable for IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
