#[doc = "Register `SEL49` reader"]
pub struct R(crate::R<SEL49_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL49_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL49_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL49_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL49` writer"]
pub struct W(crate::W<SEL49_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL49_SPEC>;
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
impl From<crate::W<SEL49_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL49_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL98` reader - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
pub type SEL98_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL98` writer - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
pub type SEL98_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL49_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL99` reader - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
pub type SEL99_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL99` writer - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
pub type SEL99_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL49_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel98(&self) -> SEL98_R {
        SEL98_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel99(&self) -> SEL99_R {
        SEL99_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT98 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel98(&mut self) -> SEL98_W<0> {
        SEL98_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT99 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel99(&mut self) -> SEL99_W<8> {
        SEL99_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 49\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel49](index.html) module"]
pub struct SEL49_SPEC;
impl crate::RegisterSpec for SEL49_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel49::R](R) reader structure"]
impl crate::Readable for SEL49_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel49::W](W) writer structure"]
impl crate::Writable for SEL49_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL49 to value 0"]
impl crate::Resettable for SEL49_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
