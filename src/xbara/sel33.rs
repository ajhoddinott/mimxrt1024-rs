#[doc = "Register `SEL33` reader"]
pub struct R(crate::R<SEL33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL33` writer"]
pub struct W(crate::W<SEL33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL33_SPEC>;
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
impl From<crate::W<SEL33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL66` reader - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
pub type SEL66_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL66` writer - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
pub type SEL66_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL33_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL67` reader - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
pub type SEL67_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL67` writer - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
pub type SEL67_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL33_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel66(&self) -> SEL66_R {
        SEL66_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel67(&self) -> SEL67_R {
        SEL67_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT66 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel66(&mut self) -> SEL66_W<0> {
        SEL66_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT67 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel67(&mut self) -> SEL67_W<8> {
        SEL67_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel33](index.html) module"]
pub struct SEL33_SPEC;
impl crate::RegisterSpec for SEL33_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel33::R](R) reader structure"]
impl crate::Readable for SEL33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel33::W](W) writer structure"]
impl crate::Writable for SEL33_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL33 to value 0"]
impl crate::Resettable for SEL33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
