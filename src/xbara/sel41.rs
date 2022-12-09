#[doc = "Register `SEL41` reader"]
pub struct R(crate::R<SEL41_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL41_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL41_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL41_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL41` writer"]
pub struct W(crate::W<SEL41_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL41_SPEC>;
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
impl From<crate::W<SEL41_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL41_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL82` reader - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
pub type SEL82_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL82` writer - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
pub type SEL82_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL41_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL83` reader - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
pub type SEL83_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL83` writer - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
pub type SEL83_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL41_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel82(&self) -> SEL82_R {
        SEL82_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel83(&self) -> SEL83_R {
        SEL83_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT82 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel82(&mut self) -> SEL82_W<0> {
        SEL82_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT83 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel83(&mut self) -> SEL83_W<8> {
        SEL83_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel41](index.html) module"]
pub struct SEL41_SPEC;
impl crate::RegisterSpec for SEL41_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel41::R](R) reader structure"]
impl crate::Readable for SEL41_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel41::W](W) writer structure"]
impl crate::Writable for SEL41_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL41 to value 0"]
impl crate::Resettable for SEL41_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
