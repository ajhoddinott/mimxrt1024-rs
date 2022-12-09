#[doc = "Register `SEL24` reader"]
pub struct R(crate::R<SEL24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL24` writer"]
pub struct W(crate::W<SEL24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL24_SPEC>;
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
impl From<crate::W<SEL24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL48` reader - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
pub type SEL48_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL48` writer - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
pub type SEL48_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL24_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL49` reader - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
pub type SEL49_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL49` writer - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
pub type SEL49_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL24_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel48(&self) -> SEL48_R {
        SEL48_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel49(&self) -> SEL49_R {
        SEL49_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT48 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel48(&mut self) -> SEL48_W<0> {
        SEL48_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT49 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel49(&mut self) -> SEL49_W<8> {
        SEL49_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel24](index.html) module"]
pub struct SEL24_SPEC;
impl crate::RegisterSpec for SEL24_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel24::R](R) reader structure"]
impl crate::Readable for SEL24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel24::W](W) writer structure"]
impl crate::Writable for SEL24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL24 to value 0"]
impl crate::Resettable for SEL24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
