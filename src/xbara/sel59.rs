#[doc = "Register `SEL59` reader"]
pub struct R(crate::R<SEL59_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL59_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL59_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL59_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL59` writer"]
pub struct W(crate::W<SEL59_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL59_SPEC>;
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
impl From<crate::W<SEL59_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL59_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL118` reader - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
pub type SEL118_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL118` writer - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
pub type SEL118_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL59_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL119` reader - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
pub type SEL119_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL119` writer - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
pub type SEL119_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL59_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel118(&self) -> SEL118_R {
        SEL118_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel119(&self) -> SEL119_R {
        SEL119_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT118 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel118(&mut self) -> SEL118_W<0> {
        SEL118_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT119 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel119(&mut self) -> SEL119_W<8> {
        SEL119_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 59\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel59](index.html) module"]
pub struct SEL59_SPEC;
impl crate::RegisterSpec for SEL59_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel59::R](R) reader structure"]
impl crate::Readable for SEL59_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel59::W](W) writer structure"]
impl crate::Writable for SEL59_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL59 to value 0"]
impl crate::Resettable for SEL59_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
