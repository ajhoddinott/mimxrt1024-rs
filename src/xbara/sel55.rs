#[doc = "Register `SEL55` reader"]
pub struct R(crate::R<SEL55_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL55_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL55_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL55_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL55` writer"]
pub struct W(crate::W<SEL55_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL55_SPEC>;
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
impl From<crate::W<SEL55_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL55_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL110` reader - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
pub type SEL110_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL110` writer - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
pub type SEL110_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL55_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL111` reader - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
pub type SEL111_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL111` writer - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
pub type SEL111_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL55_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel110(&self) -> SEL110_R {
        SEL110_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel111(&self) -> SEL111_R {
        SEL111_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT110 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel110(&mut self) -> SEL110_W<0> {
        SEL110_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT111 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel111(&mut self) -> SEL111_W<8> {
        SEL111_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 55\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel55](index.html) module"]
pub struct SEL55_SPEC;
impl crate::RegisterSpec for SEL55_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel55::R](R) reader structure"]
impl crate::Readable for SEL55_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel55::W](W) writer structure"]
impl crate::Writable for SEL55_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL55 to value 0"]
impl crate::Resettable for SEL55_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
