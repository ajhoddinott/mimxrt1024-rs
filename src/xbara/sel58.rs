#[doc = "Register `SEL58` reader"]
pub struct R(crate::R<SEL58_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL58_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL58_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL58_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL58` writer"]
pub struct W(crate::W<SEL58_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL58_SPEC>;
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
impl From<crate::W<SEL58_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL58_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL116` reader - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
pub type SEL116_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL116` writer - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
pub type SEL116_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL58_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL117` reader - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
pub type SEL117_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL117` writer - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
pub type SEL117_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL58_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel116(&self) -> SEL116_R {
        SEL116_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel117(&self) -> SEL117_R {
        SEL117_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT116 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel116(&mut self) -> SEL116_W<0> {
        SEL116_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT117 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel117(&mut self) -> SEL117_W<8> {
        SEL117_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 58\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel58](index.html) module"]
pub struct SEL58_SPEC;
impl crate::RegisterSpec for SEL58_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel58::R](R) reader structure"]
impl crate::Readable for SEL58_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel58::W](W) writer structure"]
impl crate::Writable for SEL58_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL58 to value 0"]
impl crate::Resettable for SEL58_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
