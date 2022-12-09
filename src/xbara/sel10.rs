#[doc = "Register `SEL10` reader"]
pub struct R(crate::R<SEL10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL10` writer"]
pub struct W(crate::W<SEL10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL10_SPEC>;
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
impl From<crate::W<SEL10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL20` reader - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
pub type SEL20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL20` writer - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
pub type SEL20_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL10_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL21` reader - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
pub type SEL21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL21` writer - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
pub type SEL21_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL10_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel20(&self) -> SEL20_R {
        SEL20_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel21(&self) -> SEL21_R {
        SEL21_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT20 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel20(&mut self) -> SEL20_W<0> {
        SEL20_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT21 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel21(&mut self) -> SEL21_W<8> {
        SEL21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel10](index.html) module"]
pub struct SEL10_SPEC;
impl crate::RegisterSpec for SEL10_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel10::R](R) reader structure"]
impl crate::Readable for SEL10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel10::W](W) writer structure"]
impl crate::Writable for SEL10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL10 to value 0"]
impl crate::Resettable for SEL10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
