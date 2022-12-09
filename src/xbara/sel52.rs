#[doc = "Register `SEL52` reader"]
pub struct R(crate::R<SEL52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL52` writer"]
pub struct W(crate::W<SEL52_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL52_SPEC>;
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
impl From<crate::W<SEL52_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL52_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL104` reader - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
pub type SEL104_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL104` writer - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
pub type SEL104_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL52_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL105` reader - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
pub type SEL105_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL105` writer - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
pub type SEL105_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL52_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel104(&self) -> SEL104_R {
        SEL104_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel105(&self) -> SEL105_R {
        SEL105_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT104 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel104(&mut self) -> SEL104_W<0> {
        SEL104_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT105 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel105(&mut self) -> SEL105_W<8> {
        SEL105_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 52\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel52](index.html) module"]
pub struct SEL52_SPEC;
impl crate::RegisterSpec for SEL52_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel52::R](R) reader structure"]
impl crate::Readable for SEL52_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel52::W](W) writer structure"]
impl crate::Writable for SEL52_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL52 to value 0"]
impl crate::Resettable for SEL52_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
