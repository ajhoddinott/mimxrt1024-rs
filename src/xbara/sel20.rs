#[doc = "Register `SEL20` reader"]
pub struct R(crate::R<SEL20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL20` writer"]
pub struct W(crate::W<SEL20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL20_SPEC>;
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
impl From<crate::W<SEL20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL40` reader - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
pub type SEL40_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL40` writer - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
pub type SEL40_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL20_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL41` reader - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
pub type SEL41_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL41` writer - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
pub type SEL41_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL20_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel40(&self) -> SEL40_R {
        SEL40_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel41(&self) -> SEL41_R {
        SEL41_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT40 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel40(&mut self) -> SEL40_W<0> {
        SEL40_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT41 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel41(&mut self) -> SEL41_W<8> {
        SEL41_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel20](index.html) module"]
pub struct SEL20_SPEC;
impl crate::RegisterSpec for SEL20_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel20::R](R) reader structure"]
impl crate::Readable for SEL20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel20::W](W) writer structure"]
impl crate::Writable for SEL20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL20 to value 0"]
impl crate::Resettable for SEL20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
