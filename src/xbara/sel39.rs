#[doc = "Register `SEL39` reader"]
pub struct R(crate::R<SEL39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL39` writer"]
pub struct W(crate::W<SEL39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL39_SPEC>;
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
impl From<crate::W<SEL39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL78` reader - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
pub type SEL78_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL78` writer - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
pub type SEL78_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL39_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL79` reader - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
pub type SEL79_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL79` writer - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
pub type SEL79_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL39_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel78(&self) -> SEL78_R {
        SEL78_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel79(&self) -> SEL79_R {
        SEL79_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT78 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel78(&mut self) -> SEL78_W<0> {
        SEL78_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT79 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel79(&mut self) -> SEL79_W<8> {
        SEL79_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel39](index.html) module"]
pub struct SEL39_SPEC;
impl crate::RegisterSpec for SEL39_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel39::R](R) reader structure"]
impl crate::Readable for SEL39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel39::W](W) writer structure"]
impl crate::Writable for SEL39_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL39 to value 0"]
impl crate::Resettable for SEL39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
