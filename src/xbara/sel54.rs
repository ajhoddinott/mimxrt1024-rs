#[doc = "Register `SEL54` reader"]
pub struct R(crate::R<SEL54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL54` writer"]
pub struct W(crate::W<SEL54_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL54_SPEC>;
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
impl From<crate::W<SEL54_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL54_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL108` reader - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
pub type SEL108_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL108` writer - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
pub type SEL108_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL54_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL109` reader - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
pub type SEL109_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL109` writer - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
pub type SEL109_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL54_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel108(&self) -> SEL108_R {
        SEL108_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel109(&self) -> SEL109_R {
        SEL109_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT108 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel108(&mut self) -> SEL108_W<0> {
        SEL108_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT109 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel109(&mut self) -> SEL109_W<8> {
        SEL109_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 54\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel54](index.html) module"]
pub struct SEL54_SPEC;
impl crate::RegisterSpec for SEL54_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel54::R](R) reader structure"]
impl crate::Readable for SEL54_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel54::W](W) writer structure"]
impl crate::Writable for SEL54_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL54 to value 0"]
impl crate::Resettable for SEL54_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
