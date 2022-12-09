#[doc = "Register `SEL50` reader"]
pub struct R(crate::R<SEL50_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL50_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL50_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL50_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL50` writer"]
pub struct W(crate::W<SEL50_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL50_SPEC>;
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
impl From<crate::W<SEL50_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL50_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL100` reader - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
pub type SEL100_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL100` writer - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
pub type SEL100_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL50_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL101` reader - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
pub type SEL101_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL101` writer - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
pub type SEL101_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL50_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel100(&self) -> SEL100_R {
        SEL100_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel101(&self) -> SEL101_R {
        SEL101_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT100 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel100(&mut self) -> SEL100_W<0> {
        SEL100_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT101 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel101(&mut self) -> SEL101_W<8> {
        SEL101_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 50\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel50](index.html) module"]
pub struct SEL50_SPEC;
impl crate::RegisterSpec for SEL50_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel50::R](R) reader structure"]
impl crate::Readable for SEL50_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel50::W](W) writer structure"]
impl crate::Writable for SEL50_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL50 to value 0"]
impl crate::Resettable for SEL50_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
