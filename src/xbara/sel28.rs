#[doc = "Register `SEL28` reader"]
pub struct R(crate::R<SEL28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL28` writer"]
pub struct W(crate::W<SEL28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL28_SPEC>;
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
impl From<crate::W<SEL28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL56` reader - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
pub type SEL56_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL56` writer - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
pub type SEL56_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL28_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL57` reader - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
pub type SEL57_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL57` writer - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
pub type SEL57_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL28_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel56(&self) -> SEL56_R {
        SEL56_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel57(&self) -> SEL57_R {
        SEL57_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT56 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel56(&mut self) -> SEL56_W<0> {
        SEL56_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT57 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel57(&mut self) -> SEL57_W<8> {
        SEL57_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel28](index.html) module"]
pub struct SEL28_SPEC;
impl crate::RegisterSpec for SEL28_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel28::R](R) reader structure"]
impl crate::Readable for SEL28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel28::W](W) writer structure"]
impl crate::Writable for SEL28_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL28 to value 0"]
impl crate::Resettable for SEL28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
