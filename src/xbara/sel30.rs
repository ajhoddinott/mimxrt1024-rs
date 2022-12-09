#[doc = "Register `SEL30` reader"]
pub struct R(crate::R<SEL30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL30` writer"]
pub struct W(crate::W<SEL30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL30_SPEC>;
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
impl From<crate::W<SEL30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL60` reader - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
pub type SEL60_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL60` writer - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
pub type SEL60_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL30_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL61` reader - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
pub type SEL61_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL61` writer - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
pub type SEL61_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL30_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel60(&self) -> SEL60_R {
        SEL60_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel61(&self) -> SEL61_R {
        SEL61_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT60 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel60(&mut self) -> SEL60_W<0> {
        SEL60_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT61 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel61(&mut self) -> SEL61_W<8> {
        SEL61_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel30](index.html) module"]
pub struct SEL30_SPEC;
impl crate::RegisterSpec for SEL30_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel30::R](R) reader structure"]
impl crate::Readable for SEL30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel30::W](W) writer structure"]
impl crate::Writable for SEL30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL30 to value 0"]
impl crate::Resettable for SEL30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
