#[doc = "Register `SEL6` reader"]
pub struct R(crate::R<SEL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL6` writer"]
pub struct W(crate::W<SEL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL6_SPEC>;
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
impl From<crate::W<SEL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL12` reader - Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
pub type SEL12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL12` writer - Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
pub type SEL12_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL6_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL13` reader - Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
pub type SEL13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL13` writer - Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
pub type SEL13_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL6_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel13(&self) -> SEL13_R {
        SEL13_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT12 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<0> {
        SEL12_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT13 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel13(&mut self) -> SEL13_W<8> {
        SEL13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel6](index.html) module"]
pub struct SEL6_SPEC;
impl crate::RegisterSpec for SEL6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel6::R](R) reader structure"]
impl crate::Readable for SEL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel6::W](W) writer structure"]
impl crate::Writable for SEL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL6 to value 0"]
impl crate::Resettable for SEL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
