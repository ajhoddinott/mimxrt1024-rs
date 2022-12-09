#[doc = "Register `SEL65` reader"]
pub struct R(crate::R<SEL65_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL65_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL65_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL65_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL65` writer"]
pub struct W(crate::W<SEL65_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL65_SPEC>;
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
impl From<crate::W<SEL65_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL65_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL130` reader - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
pub type SEL130_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL130` writer - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
pub type SEL130_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL65_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL131` reader - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
pub type SEL131_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL131` writer - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
pub type SEL131_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL65_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel130(&self) -> SEL130_R {
        SEL130_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel131(&self) -> SEL131_R {
        SEL131_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT130 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel130(&mut self) -> SEL130_W<0> {
        SEL130_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT131 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel131(&mut self) -> SEL131_W<8> {
        SEL131_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 65\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel65](index.html) module"]
pub struct SEL65_SPEC;
impl crate::RegisterSpec for SEL65_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel65::R](R) reader structure"]
impl crate::Readable for SEL65_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel65::W](W) writer structure"]
impl crate::Writable for SEL65_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL65 to value 0"]
impl crate::Resettable for SEL65_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
