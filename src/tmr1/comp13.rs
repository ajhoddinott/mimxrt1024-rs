#[doc = "Register `COMP13` reader"]
pub struct R(crate::R<COMP13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP13` writer"]
pub struct W(crate::W<COMP13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP13_SPEC>;
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
impl From<crate::W<COMP13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_1` reader - Comparison Value 1"]
pub type COMPARISON_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARISON_1` writer - Comparison Value 1"]
pub type COMPARISON_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, COMP13_SPEC, u16, u16, 16, O>;
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
#[doc = "Timer Channel Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp13](index.html) module"]
pub struct COMP13_SPEC;
impl crate::RegisterSpec for COMP13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [comp13::R](R) reader structure"]
impl crate::Readable for COMP13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp13::W](W) writer structure"]
impl crate::Writable for COMP13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP13 to value 0"]
impl crate::Resettable for COMP13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
