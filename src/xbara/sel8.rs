#[doc = "Register `SEL8` reader"]
pub struct R(crate::R<SEL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL8` writer"]
pub struct W(crate::W<SEL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL8_SPEC>;
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
impl From<crate::W<SEL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL16` reader - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
pub type SEL16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL16` writer - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
pub type SEL16_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL8_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL17` reader - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
pub type SEL17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL17` writer - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
pub type SEL17_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL8_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel16(&self) -> SEL16_R {
        SEL16_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel17(&self) -> SEL17_R {
        SEL17_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT16 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel16(&mut self) -> SEL16_W<0> {
        SEL16_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT17 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel17(&mut self) -> SEL17_W<8> {
        SEL17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel8](index.html) module"]
pub struct SEL8_SPEC;
impl crate::RegisterSpec for SEL8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel8::R](R) reader structure"]
impl crate::Readable for SEL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel8::W](W) writer structure"]
impl crate::Writable for SEL8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL8 to value 0"]
impl crate::Resettable for SEL8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
