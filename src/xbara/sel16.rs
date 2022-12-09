#[doc = "Register `SEL16` reader"]
pub struct R(crate::R<SEL16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL16` writer"]
pub struct W(crate::W<SEL16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL16_SPEC>;
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
impl From<crate::W<SEL16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL32` reader - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
pub type SEL32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL32` writer - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
pub type SEL32_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL16_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL33` reader - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
pub type SEL33_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL33` writer - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
pub type SEL33_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL16_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel32(&self) -> SEL32_R {
        SEL32_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel33(&self) -> SEL33_R {
        SEL33_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT32 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel32(&mut self) -> SEL32_W<0> {
        SEL32_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT33 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel33(&mut self) -> SEL33_W<8> {
        SEL33_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel16](index.html) module"]
pub struct SEL16_SPEC;
impl crate::RegisterSpec for SEL16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel16::R](R) reader structure"]
impl crate::Readable for SEL16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel16::W](W) writer structure"]
impl crate::Writable for SEL16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL16 to value 0"]
impl crate::Resettable for SEL16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
