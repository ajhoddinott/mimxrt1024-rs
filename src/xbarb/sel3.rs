#[doc = "Register `SEL3` reader"]
pub struct R(crate::R<SEL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL3` writer"]
pub struct W(crate::W<SEL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL3_SPEC>;
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
impl From<crate::W<SEL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL6` reader - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
pub type SEL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL6` writer - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
pub type SEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL3_SPEC, u8, u8, 6, O>;
#[doc = "Field `SEL7` reader - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
pub type SEL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL7` writer - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
pub type SEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL3_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT6 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL6_W<0> {
        SEL6_W::new(self)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT7 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL7_W<8> {
        SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar B Select Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel3](index.html) module"]
pub struct SEL3_SPEC;
impl crate::RegisterSpec for SEL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel3::R](R) reader structure"]
impl crate::Readable for SEL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel3::W](W) writer structure"]
impl crate::Writable for SEL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL3 to value 0"]
impl crate::Resettable for SEL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
