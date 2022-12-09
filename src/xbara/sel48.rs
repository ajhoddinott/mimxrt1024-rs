#[doc = "Register `SEL48` reader"]
pub struct R(crate::R<SEL48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL48` writer"]
pub struct W(crate::W<SEL48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL48_SPEC>;
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
impl From<crate::W<SEL48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL96` reader - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
pub type SEL96_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL96` writer - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
pub type SEL96_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL48_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL97` reader - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
pub type SEL97_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL97` writer - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
pub type SEL97_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL48_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel96(&self) -> SEL96_R {
        SEL96_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel97(&self) -> SEL97_R {
        SEL97_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT96 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel96(&mut self) -> SEL96_W<0> {
        SEL96_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT97 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel97(&mut self) -> SEL97_W<8> {
        SEL97_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 48\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel48](index.html) module"]
pub struct SEL48_SPEC;
impl crate::RegisterSpec for SEL48_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel48::R](R) reader structure"]
impl crate::Readable for SEL48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel48::W](W) writer structure"]
impl crate::Writable for SEL48_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL48 to value 0"]
impl crate::Resettable for SEL48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
