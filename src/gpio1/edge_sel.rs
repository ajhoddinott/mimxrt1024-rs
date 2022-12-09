#[doc = "Register `EDGE_SEL` reader"]
pub struct R(crate::R<EDGE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDGE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDGE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDGE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDGE_SEL` writer"]
pub struct W(crate::W<EDGE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDGE_SEL_SPEC>;
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
impl From<crate::W<EDGE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDGE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_EDGE_SEL` reader - Edge select"]
pub type GPIO_EDGE_SEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_EDGE_SEL` writer - Edge select"]
pub type GPIO_EDGE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EDGE_SEL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Edge select"]
    #[inline(always)]
    pub fn gpio_edge_sel(&self) -> GPIO_EDGE_SEL_R {
        GPIO_EDGE_SEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Edge select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_edge_sel(&mut self) -> GPIO_EDGE_SEL_W<0> {
        GPIO_EDGE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO edge select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edge_sel](index.html) module"]
pub struct EDGE_SEL_SPEC;
impl crate::RegisterSpec for EDGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edge_sel::R](R) reader structure"]
impl crate::Readable for EDGE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edge_sel::W](W) writer structure"]
impl crate::Writable for EDGE_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDGE_SEL to value 0"]
impl crate::Resettable for EDGE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
