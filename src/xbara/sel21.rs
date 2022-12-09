#[doc = "Register `SEL21` reader"]
pub struct R(crate::R<SEL21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL21` writer"]
pub struct W(crate::W<SEL21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL21_SPEC>;
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
impl From<crate::W<SEL21_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL42` reader - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
pub type SEL42_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL42` writer - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
pub type SEL42_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL21_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL43` reader - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
pub type SEL43_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL43` writer - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
pub type SEL43_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL21_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel42(&self) -> SEL42_R {
        SEL42_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel43(&self) -> SEL43_R {
        SEL43_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT42 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel42(&mut self) -> SEL42_W<0> {
        SEL42_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT43 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel43(&mut self) -> SEL43_W<8> {
        SEL43_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel21](index.html) module"]
pub struct SEL21_SPEC;
impl crate::RegisterSpec for SEL21_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel21::R](R) reader structure"]
impl crate::Readable for SEL21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel21::W](W) writer structure"]
impl crate::Writable for SEL21_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL21 to value 0"]
impl crate::Resettable for SEL21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
