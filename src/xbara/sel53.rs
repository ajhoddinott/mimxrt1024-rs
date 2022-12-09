#[doc = "Register `SEL53` reader"]
pub struct R(crate::R<SEL53_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL53_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL53_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL53_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL53` writer"]
pub struct W(crate::W<SEL53_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL53_SPEC>;
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
impl From<crate::W<SEL53_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL53_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL106` reader - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
pub type SEL106_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL106` writer - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
pub type SEL106_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL53_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL107` reader - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
pub type SEL107_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL107` writer - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
pub type SEL107_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL53_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel106(&self) -> SEL106_R {
        SEL106_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel107(&self) -> SEL107_R {
        SEL107_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT106 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel106(&mut self) -> SEL106_W<0> {
        SEL106_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT107 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel107(&mut self) -> SEL107_W<8> {
        SEL107_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 53\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel53](index.html) module"]
pub struct SEL53_SPEC;
impl crate::RegisterSpec for SEL53_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel53::R](R) reader structure"]
impl crate::Readable for SEL53_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel53::W](W) writer structure"]
impl crate::Writable for SEL53_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL53 to value 0"]
impl crate::Resettable for SEL53_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
