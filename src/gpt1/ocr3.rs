#[doc = "Register `OCR3` reader"]
pub struct R(crate::R<OCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCR3` writer"]
pub struct W(crate::W<OCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCR3_SPEC>;
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
impl From<crate::W<OCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Compare Value"]
pub type COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCR3_SPEC, u32, u32, 32, O>;
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
#[doc = "GPT Output Compare Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocr3](index.html) module"]
pub struct OCR3_SPEC;
impl crate::RegisterSpec for OCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocr3::R](R) reader structure"]
impl crate::Readable for OCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocr3::W](W) writer structure"]
impl crate::Writable for OCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCR3 to value 0xffff_ffff"]
impl crate::Resettable for OCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
