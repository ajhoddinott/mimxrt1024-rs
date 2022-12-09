#[doc = "Register `SEL42` reader"]
pub struct R(crate::R<SEL42_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL42_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL42_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL42_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL42` writer"]
pub struct W(crate::W<SEL42_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL42_SPEC>;
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
impl From<crate::W<SEL42_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL42_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL84` reader - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
pub type SEL84_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL84` writer - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
pub type SEL84_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL42_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL85` reader - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
pub type SEL85_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL85` writer - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
pub type SEL85_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL42_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel84(&self) -> SEL84_R {
        SEL84_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel85(&self) -> SEL85_R {
        SEL85_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT84 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel84(&mut self) -> SEL84_W<0> {
        SEL84_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT85 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel85(&mut self) -> SEL85_W<8> {
        SEL85_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 42\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel42](index.html) module"]
pub struct SEL42_SPEC;
impl crate::RegisterSpec for SEL42_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel42::R](R) reader structure"]
impl crate::Readable for SEL42_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel42::W](W) writer structure"]
impl crate::Writable for SEL42_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL42 to value 0"]
impl crate::Resettable for SEL42_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
