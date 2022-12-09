#[doc = "Register `SEL9` reader"]
pub struct R(crate::R<SEL9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL9` writer"]
pub struct W(crate::W<SEL9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL9_SPEC>;
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
impl From<crate::W<SEL9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL18` reader - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
pub type SEL18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL18` writer - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
pub type SEL18_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL9_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL19` reader - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
pub type SEL19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL19` writer - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
pub type SEL19_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL9_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel18(&self) -> SEL18_R {
        SEL18_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel19(&self) -> SEL19_R {
        SEL19_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT18 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel18(&mut self) -> SEL18_W<0> {
        SEL18_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT19 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel19(&mut self) -> SEL19_W<8> {
        SEL19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel9](index.html) module"]
pub struct SEL9_SPEC;
impl crate::RegisterSpec for SEL9_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel9::R](R) reader structure"]
impl crate::Readable for SEL9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel9::W](W) writer structure"]
impl crate::Writable for SEL9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL9 to value 0"]
impl crate::Resettable for SEL9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
