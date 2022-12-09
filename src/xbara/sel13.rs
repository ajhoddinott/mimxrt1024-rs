#[doc = "Register `SEL13` reader"]
pub struct R(crate::R<SEL13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL13` writer"]
pub struct W(crate::W<SEL13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL13_SPEC>;
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
impl From<crate::W<SEL13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL26` reader - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
pub type SEL26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL26` writer - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
pub type SEL26_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL13_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL27` reader - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
pub type SEL27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL27` writer - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
pub type SEL27_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL13_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel26(&self) -> SEL26_R {
        SEL26_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel27(&self) -> SEL27_R {
        SEL27_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT26 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel26(&mut self) -> SEL26_W<0> {
        SEL26_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT27 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel27(&mut self) -> SEL27_W<8> {
        SEL27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel13](index.html) module"]
pub struct SEL13_SPEC;
impl crate::RegisterSpec for SEL13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel13::R](R) reader structure"]
impl crate::Readable for SEL13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel13::W](W) writer structure"]
impl crate::Writable for SEL13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL13 to value 0"]
impl crate::Resettable for SEL13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
