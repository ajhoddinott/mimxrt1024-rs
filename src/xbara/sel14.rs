#[doc = "Register `SEL14` reader"]
pub struct R(crate::R<SEL14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL14` writer"]
pub struct W(crate::W<SEL14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL14_SPEC>;
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
impl From<crate::W<SEL14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL28` reader - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
pub type SEL28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL28` writer - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
pub type SEL28_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL14_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL29` reader - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
pub type SEL29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL29` writer - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
pub type SEL29_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL14_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel28(&self) -> SEL28_R {
        SEL28_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel29(&self) -> SEL29_R {
        SEL29_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT28 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel28(&mut self) -> SEL28_W<0> {
        SEL28_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT29 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel29(&mut self) -> SEL29_W<8> {
        SEL29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel14](index.html) module"]
pub struct SEL14_SPEC;
impl crate::RegisterSpec for SEL14_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel14::R](R) reader structure"]
impl crate::Readable for SEL14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel14::W](W) writer structure"]
impl crate::Writable for SEL14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL14 to value 0"]
impl crate::Resettable for SEL14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
