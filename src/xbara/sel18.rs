#[doc = "Register `SEL18` reader"]
pub struct R(crate::R<SEL18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL18` writer"]
pub struct W(crate::W<SEL18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL18_SPEC>;
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
impl From<crate::W<SEL18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL36` reader - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
pub type SEL36_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL36` writer - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
pub type SEL36_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL18_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL37` reader - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
pub type SEL37_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL37` writer - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
pub type SEL37_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL18_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel36(&self) -> SEL36_R {
        SEL36_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel37(&self) -> SEL37_R {
        SEL37_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT36 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel36(&mut self) -> SEL36_W<0> {
        SEL36_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT37 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel37(&mut self) -> SEL37_W<8> {
        SEL37_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel18](index.html) module"]
pub struct SEL18_SPEC;
impl crate::RegisterSpec for SEL18_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel18::R](R) reader structure"]
impl crate::Readable for SEL18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel18::W](W) writer structure"]
impl crate::Writable for SEL18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL18 to value 0"]
impl crate::Resettable for SEL18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
