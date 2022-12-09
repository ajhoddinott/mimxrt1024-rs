#[doc = "Register `SEL64` reader"]
pub struct R(crate::R<SEL64_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL64_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL64_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL64_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL64` writer"]
pub struct W(crate::W<SEL64_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL64_SPEC>;
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
impl From<crate::W<SEL64_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL64_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL128` reader - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
pub type SEL128_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL128` writer - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
pub type SEL128_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL64_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL129` reader - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
pub type SEL129_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL129` writer - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
pub type SEL129_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL64_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel128(&self) -> SEL128_R {
        SEL128_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel129(&self) -> SEL129_R {
        SEL129_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT128 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel128(&mut self) -> SEL128_W<0> {
        SEL128_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT129 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel129(&mut self) -> SEL129_W<8> {
        SEL129_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 64\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel64](index.html) module"]
pub struct SEL64_SPEC;
impl crate::RegisterSpec for SEL64_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel64::R](R) reader structure"]
impl crate::Readable for SEL64_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel64::W](W) writer structure"]
impl crate::Writable for SEL64_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL64 to value 0"]
impl crate::Resettable for SEL64_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
