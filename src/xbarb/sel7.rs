#[doc = "Register `SEL7` reader"]
pub struct R(crate::R<SEL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL7` writer"]
pub struct W(crate::W<SEL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL7_SPEC>;
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
impl From<crate::W<SEL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL14` reader - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
pub type SEL14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL14` writer - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
pub type SEL14_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL7_SPEC, u8, u8, 6, O>;
#[doc = "Field `SEL15` reader - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
pub type SEL15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL15` writer - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
pub type SEL15_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL7_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel14(&self) -> SEL14_R {
        SEL14_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel15(&self) -> SEL15_R {
        SEL15_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT14 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel14(&mut self) -> SEL14_W<0> {
        SEL14_W::new(self)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT15 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel15(&mut self) -> SEL15_W<8> {
        SEL15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar B Select Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel7](index.html) module"]
pub struct SEL7_SPEC;
impl crate::RegisterSpec for SEL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel7::R](R) reader structure"]
impl crate::Readable for SEL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel7::W](W) writer structure"]
impl crate::Writable for SEL7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL7 to value 0"]
impl crate::Resettable for SEL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
