#[doc = "Register `SEL45` reader"]
pub struct R(crate::R<SEL45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL45` writer"]
pub struct W(crate::W<SEL45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL45_SPEC>;
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
impl From<crate::W<SEL45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL90` reader - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
pub type SEL90_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL90` writer - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
pub type SEL90_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL45_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL91` reader - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
pub type SEL91_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL91` writer - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
pub type SEL91_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL45_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel90(&self) -> SEL90_R {
        SEL90_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel91(&self) -> SEL91_R {
        SEL91_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT90 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel90(&mut self) -> SEL90_W<0> {
        SEL90_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT91 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel91(&mut self) -> SEL91_W<8> {
        SEL91_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 45\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel45](index.html) module"]
pub struct SEL45_SPEC;
impl crate::RegisterSpec for SEL45_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel45::R](R) reader structure"]
impl crate::Readable for SEL45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel45::W](W) writer structure"]
impl crate::Writable for SEL45_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL45 to value 0"]
impl crate::Resettable for SEL45_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
