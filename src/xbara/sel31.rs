#[doc = "Register `SEL31` reader"]
pub struct R(crate::R<SEL31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL31` writer"]
pub struct W(crate::W<SEL31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL31_SPEC>;
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
impl From<crate::W<SEL31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL62` reader - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
pub type SEL62_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL62` writer - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
pub type SEL62_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL31_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL63` reader - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
pub type SEL63_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL63` writer - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
pub type SEL63_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL31_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel62(&self) -> SEL62_R {
        SEL62_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel63(&self) -> SEL63_R {
        SEL63_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT62 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel62(&mut self) -> SEL62_W<0> {
        SEL62_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT63 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel63(&mut self) -> SEL63_W<8> {
        SEL63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel31](index.html) module"]
pub struct SEL31_SPEC;
impl crate::RegisterSpec for SEL31_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel31::R](R) reader structure"]
impl crate::Readable for SEL31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel31::W](W) writer structure"]
impl crate::Writable for SEL31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL31 to value 0"]
impl crate::Resettable for SEL31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
