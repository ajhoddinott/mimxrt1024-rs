#[doc = "Register `SEL15` reader"]
pub struct R(crate::R<SEL15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL15` writer"]
pub struct W(crate::W<SEL15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL15_SPEC>;
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
impl From<crate::W<SEL15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL30` reader - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
pub type SEL30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL30` writer - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
pub type SEL30_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL15_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL31` reader - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
pub type SEL31_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL31` writer - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
pub type SEL31_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL15_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel30(&self) -> SEL30_R {
        SEL30_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel31(&self) -> SEL31_R {
        SEL31_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT30 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel30(&mut self) -> SEL30_W<0> {
        SEL30_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT31 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel31(&mut self) -> SEL31_W<8> {
        SEL31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel15](index.html) module"]
pub struct SEL15_SPEC;
impl crate::RegisterSpec for SEL15_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel15::R](R) reader structure"]
impl crate::Readable for SEL15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel15::W](W) writer structure"]
impl crate::Writable for SEL15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL15 to value 0"]
impl crate::Resettable for SEL15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
