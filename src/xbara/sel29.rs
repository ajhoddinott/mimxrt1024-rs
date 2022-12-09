#[doc = "Register `SEL29` reader"]
pub struct R(crate::R<SEL29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL29` writer"]
pub struct W(crate::W<SEL29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL29_SPEC>;
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
impl From<crate::W<SEL29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL58` reader - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
pub type SEL58_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL58` writer - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
pub type SEL58_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL29_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL59` reader - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
pub type SEL59_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL59` writer - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
pub type SEL59_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL29_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel58(&self) -> SEL58_R {
        SEL58_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel59(&self) -> SEL59_R {
        SEL59_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT58 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel58(&mut self) -> SEL58_W<0> {
        SEL58_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT59 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel59(&mut self) -> SEL59_W<8> {
        SEL59_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel29](index.html) module"]
pub struct SEL29_SPEC;
impl crate::RegisterSpec for SEL29_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel29::R](R) reader structure"]
impl crate::Readable for SEL29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel29::W](W) writer structure"]
impl crate::Writable for SEL29_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL29 to value 0"]
impl crate::Resettable for SEL29_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
