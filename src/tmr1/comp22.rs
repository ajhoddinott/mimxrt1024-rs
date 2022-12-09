#[doc = "Register `COMP22` reader"]
pub struct R(crate::R<COMP22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP22` writer"]
pub struct W(crate::W<COMP22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP22_SPEC>;
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
impl From<crate::W<COMP22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_2` reader - Comparison Value 2"]
pub type COMPARISON_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARISON_2` writer - Comparison Value 2"]
pub type COMPARISON_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, COMP22_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Comparison Value 2"]
    #[inline(always)]
    pub fn comparison_2(&self) -> COMPARISON_2_R {
        COMPARISON_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparison Value 2"]
    #[inline(always)]
    #[must_use]
    pub fn comparison_2(&mut self) -> COMPARISON_2_W<0> {
        COMPARISON_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Compare Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp22](index.html) module"]
pub struct COMP22_SPEC;
impl crate::RegisterSpec for COMP22_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [comp22::R](R) reader structure"]
impl crate::Readable for COMP22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp22::W](W) writer structure"]
impl crate::Writable for COMP22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP22 to value 0"]
impl crate::Resettable for COMP22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
