#[doc = "Register `SEL36` reader"]
pub struct R(crate::R<SEL36_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL36_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL36_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL36_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL36` writer"]
pub struct W(crate::W<SEL36_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL36_SPEC>;
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
impl From<crate::W<SEL36_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL36_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL72` reader - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
pub type SEL72_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL72` writer - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
pub type SEL72_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL36_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL73` reader - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
pub type SEL73_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL73` writer - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
pub type SEL73_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL36_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel72(&self) -> SEL72_R {
        SEL72_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel73(&self) -> SEL73_R {
        SEL73_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT72 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel72(&mut self) -> SEL72_W<0> {
        SEL72_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT73 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel73(&mut self) -> SEL73_W<8> {
        SEL73_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel36](index.html) module"]
pub struct SEL36_SPEC;
impl crate::RegisterSpec for SEL36_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel36::R](R) reader structure"]
impl crate::Readable for SEL36_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel36::W](W) writer structure"]
impl crate::Writable for SEL36_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL36 to value 0"]
impl crate::Resettable for SEL36_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
