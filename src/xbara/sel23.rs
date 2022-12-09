#[doc = "Register `SEL23` reader"]
pub struct R(crate::R<SEL23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL23` writer"]
pub struct W(crate::W<SEL23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL23_SPEC>;
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
impl From<crate::W<SEL23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL46` reader - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
pub type SEL46_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL46` writer - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
pub type SEL46_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL23_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL47` reader - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
pub type SEL47_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL47` writer - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
pub type SEL47_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL23_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel46(&self) -> SEL46_R {
        SEL46_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel47(&self) -> SEL47_R {
        SEL47_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT46 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel46(&mut self) -> SEL46_W<0> {
        SEL46_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT47 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel47(&mut self) -> SEL47_W<8> {
        SEL47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel23](index.html) module"]
pub struct SEL23_SPEC;
impl crate::RegisterSpec for SEL23_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel23::R](R) reader structure"]
impl crate::Readable for SEL23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel23::W](W) writer structure"]
impl crate::Writable for SEL23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL23 to value 0"]
impl crate::Resettable for SEL23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
