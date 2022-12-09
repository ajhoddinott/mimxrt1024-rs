#[doc = "Register `SEL17` reader"]
pub struct R(crate::R<SEL17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL17` writer"]
pub struct W(crate::W<SEL17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL17_SPEC>;
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
impl From<crate::W<SEL17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL34` reader - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
pub type SEL34_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL34` writer - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
pub type SEL34_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL17_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL35` reader - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
pub type SEL35_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL35` writer - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
pub type SEL35_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL17_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel34(&self) -> SEL34_R {
        SEL34_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel35(&self) -> SEL35_R {
        SEL35_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT34 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel34(&mut self) -> SEL34_W<0> {
        SEL34_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT35 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel35(&mut self) -> SEL35_W<8> {
        SEL35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel17](index.html) module"]
pub struct SEL17_SPEC;
impl crate::RegisterSpec for SEL17_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel17::R](R) reader structure"]
impl crate::Readable for SEL17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel17::W](W) writer structure"]
impl crate::Writable for SEL17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL17 to value 0"]
impl crate::Resettable for SEL17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
