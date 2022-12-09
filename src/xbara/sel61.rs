#[doc = "Register `SEL61` reader"]
pub struct R(crate::R<SEL61_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL61_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL61_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL61_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL61` writer"]
pub struct W(crate::W<SEL61_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL61_SPEC>;
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
impl From<crate::W<SEL61_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL61_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL122` reader - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
pub type SEL122_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL122` writer - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
pub type SEL122_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL61_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL123` reader - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
pub type SEL123_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL123` writer - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
pub type SEL123_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL61_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel122(&self) -> SEL122_R {
        SEL122_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel123(&self) -> SEL123_R {
        SEL123_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT122 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel122(&mut self) -> SEL122_W<0> {
        SEL122_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT123 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel123(&mut self) -> SEL123_W<8> {
        SEL123_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 61\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel61](index.html) module"]
pub struct SEL61_SPEC;
impl crate::RegisterSpec for SEL61_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel61::R](R) reader structure"]
impl crate::Readable for SEL61_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel61::W](W) writer structure"]
impl crate::Writable for SEL61_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL61 to value 0"]
impl crate::Resettable for SEL61_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
