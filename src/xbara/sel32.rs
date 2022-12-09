#[doc = "Register `SEL32` reader"]
pub struct R(crate::R<SEL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL32` writer"]
pub struct W(crate::W<SEL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL32_SPEC>;
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
impl From<crate::W<SEL32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL64` reader - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
pub type SEL64_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL64` writer - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
pub type SEL64_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL32_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL65` reader - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
pub type SEL65_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL65` writer - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
pub type SEL65_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL32_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel64(&self) -> SEL64_R {
        SEL64_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel65(&self) -> SEL65_R {
        SEL65_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT64 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel64(&mut self) -> SEL64_W<0> {
        SEL64_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT65 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel65(&mut self) -> SEL65_W<8> {
        SEL65_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel32](index.html) module"]
pub struct SEL32_SPEC;
impl crate::RegisterSpec for SEL32_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel32::R](R) reader structure"]
impl crate::Readable for SEL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel32::W](W) writer structure"]
impl crate::Writable for SEL32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL32 to value 0"]
impl crate::Resettable for SEL32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
