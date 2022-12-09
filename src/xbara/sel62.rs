#[doc = "Register `SEL62` reader"]
pub struct R(crate::R<SEL62_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL62_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL62_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL62_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL62` writer"]
pub struct W(crate::W<SEL62_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL62_SPEC>;
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
impl From<crate::W<SEL62_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL62_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL124` reader - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
pub type SEL124_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL124` writer - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
pub type SEL124_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL62_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL125` reader - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
pub type SEL125_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL125` writer - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
pub type SEL125_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL62_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel124(&self) -> SEL124_R {
        SEL124_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel125(&self) -> SEL125_R {
        SEL125_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT124 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel124(&mut self) -> SEL124_W<0> {
        SEL124_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT125 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel125(&mut self) -> SEL125_W<8> {
        SEL125_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 62\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel62](index.html) module"]
pub struct SEL62_SPEC;
impl crate::RegisterSpec for SEL62_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel62::R](R) reader structure"]
impl crate::Readable for SEL62_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel62::W](W) writer structure"]
impl crate::Writable for SEL62_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL62 to value 0"]
impl crate::Resettable for SEL62_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
