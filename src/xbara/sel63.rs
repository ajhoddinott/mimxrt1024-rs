#[doc = "Register `SEL63` reader"]
pub struct R(crate::R<SEL63_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL63_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL63_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL63_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL63` writer"]
pub struct W(crate::W<SEL63_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL63_SPEC>;
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
impl From<crate::W<SEL63_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL63_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL126` reader - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
pub type SEL126_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL126` writer - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
pub type SEL126_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL63_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL127` reader - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
pub type SEL127_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL127` writer - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
pub type SEL127_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL63_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel126(&self) -> SEL126_R {
        SEL126_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel127(&self) -> SEL127_R {
        SEL127_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT126 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel126(&mut self) -> SEL126_W<0> {
        SEL126_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT127 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel127(&mut self) -> SEL127_W<8> {
        SEL127_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 63\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel63](index.html) module"]
pub struct SEL63_SPEC;
impl crate::RegisterSpec for SEL63_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel63::R](R) reader structure"]
impl crate::Readable for SEL63_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel63::W](W) writer structure"]
impl crate::Writable for SEL63_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL63 to value 0"]
impl crate::Resettable for SEL63_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
