#[doc = "Register `SEL57` reader"]
pub struct R(crate::R<SEL57_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL57_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL57_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL57_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL57` writer"]
pub struct W(crate::W<SEL57_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL57_SPEC>;
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
impl From<crate::W<SEL57_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL57_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL114` reader - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
pub type SEL114_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL114` writer - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
pub type SEL114_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL57_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL115` reader - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
pub type SEL115_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL115` writer - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
pub type SEL115_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL57_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel114(&self) -> SEL114_R {
        SEL114_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel115(&self) -> SEL115_R {
        SEL115_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT114 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel114(&mut self) -> SEL114_W<0> {
        SEL114_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT115 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel115(&mut self) -> SEL115_W<8> {
        SEL115_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 57\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel57](index.html) module"]
pub struct SEL57_SPEC;
impl crate::RegisterSpec for SEL57_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel57::R](R) reader structure"]
impl crate::Readable for SEL57_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel57::W](W) writer structure"]
impl crate::Writable for SEL57_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL57 to value 0"]
impl crate::Resettable for SEL57_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
