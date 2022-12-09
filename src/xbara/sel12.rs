#[doc = "Register `SEL12` reader"]
pub struct R(crate::R<SEL12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL12` writer"]
pub struct W(crate::W<SEL12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL12_SPEC>;
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
impl From<crate::W<SEL12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL24` reader - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
pub type SEL24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL24` writer - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
pub type SEL24_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL12_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL25` reader - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
pub type SEL25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL25` writer - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
pub type SEL25_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL12_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel24(&self) -> SEL24_R {
        SEL24_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel25(&self) -> SEL25_R {
        SEL25_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT24 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel24(&mut self) -> SEL24_W<0> {
        SEL24_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT25 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel25(&mut self) -> SEL25_W<8> {
        SEL25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel12](index.html) module"]
pub struct SEL12_SPEC;
impl crate::RegisterSpec for SEL12_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel12::R](R) reader structure"]
impl crate::Readable for SEL12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel12::W](W) writer structure"]
impl crate::Writable for SEL12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL12 to value 0"]
impl crate::Resettable for SEL12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
