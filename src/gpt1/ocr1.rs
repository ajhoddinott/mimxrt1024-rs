#[doc = "Register `OCR1` reader"]
pub struct R(crate::R<OCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR1` writer"]
pub struct W(crate::W<OCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR1_SPEC>;
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
impl From<crate::W<OCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Compare Value"]
pub type COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Output Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr1](index.html) module"]
pub struct OCR1_SPEC;
impl crate::RegisterSpec for OCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocr1::R](R) reader structure"]
impl crate::Readable for OCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr1::W](W) writer structure"]
impl crate::Writable for OCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR1 to value 0xffff_ffff"]
impl crate::Resettable for OCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
