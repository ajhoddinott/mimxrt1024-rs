#[doc = "Register `SEL56` reader"]
pub struct R(crate::R<SEL56_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL56_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL56_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL56_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL56` writer"]
pub struct W(crate::W<SEL56_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL56_SPEC>;
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
impl From<crate::W<SEL56_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL56_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL112` reader - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
pub type SEL112_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL112` writer - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
pub type SEL112_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL56_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL113` reader - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
pub type SEL113_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL113` writer - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
pub type SEL113_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL56_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel112(&self) -> SEL112_R {
        SEL112_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel113(&self) -> SEL113_R {
        SEL113_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT112 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel112(&mut self) -> SEL112_W<0> {
        SEL112_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT113 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel113(&mut self) -> SEL113_W<8> {
        SEL113_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 56\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel56](index.html) module"]
pub struct SEL56_SPEC;
impl crate::RegisterSpec for SEL56_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel56::R](R) reader structure"]
impl crate::Readable for SEL56_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel56::W](W) writer structure"]
impl crate::Writable for SEL56_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL56 to value 0"]
impl crate::Resettable for SEL56_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
