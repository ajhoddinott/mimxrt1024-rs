#[doc = "Register `SEL19` reader"]
pub struct R(crate::R<SEL19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL19` writer"]
pub struct W(crate::W<SEL19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL19_SPEC>;
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
impl From<crate::W<SEL19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL38` reader - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
pub type SEL38_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL38` writer - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
pub type SEL38_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL19_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL39` reader - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
pub type SEL39_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL39` writer - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
pub type SEL39_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL19_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel38(&self) -> SEL38_R {
        SEL38_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel39(&self) -> SEL39_R {
        SEL39_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT38 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel38(&mut self) -> SEL38_W<0> {
        SEL38_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT39 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel39(&mut self) -> SEL39_W<8> {
        SEL39_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel19](index.html) module"]
pub struct SEL19_SPEC;
impl crate::RegisterSpec for SEL19_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel19::R](R) reader structure"]
impl crate::Readable for SEL19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel19::W](W) writer structure"]
impl crate::Writable for SEL19_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL19 to value 0"]
impl crate::Resettable for SEL19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
