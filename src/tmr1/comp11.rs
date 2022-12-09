#[doc = "Register `COMP11` reader"]
pub struct R(crate::R<COMP11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP11` writer"]
pub struct W(crate::W<COMP11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP11_SPEC>;
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
impl From<crate::W<COMP11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_1` reader - Comparison Value 1"]
pub type COMPARISON_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARISON_1` writer - Comparison Value 1"]
pub type COMPARISON_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, COMP11_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Comparison Value 1"]
    #[inline(always)]
    pub fn comparison_1(&self) -> COMPARISON_1_R {
        COMPARISON_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparison Value 1"]
    #[inline(always)]
    #[must_use]
    pub fn comparison_1(&mut self) -> COMPARISON_1_W<0> {
        COMPARISON_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp11](index.html) module"]
pub struct COMP11_SPEC;
impl crate::RegisterSpec for COMP11_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [comp11::R](R) reader structure"]
impl crate::Readable for COMP11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp11::W](W) writer structure"]
impl crate::Writable for COMP11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP11 to value 0"]
impl crate::Resettable for COMP11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
