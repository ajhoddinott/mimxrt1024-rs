#[doc = "Register `SEL27` reader"]
pub struct R(crate::R<SEL27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL27` writer"]
pub struct W(crate::W<SEL27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL27_SPEC>;
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
impl From<crate::W<SEL27_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL27_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL54` reader - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
pub type SEL54_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL54` writer - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
pub type SEL54_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL27_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL55` reader - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
pub type SEL55_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL55` writer - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
pub type SEL55_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL27_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel54(&self) -> SEL54_R {
        SEL54_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel55(&self) -> SEL55_R {
        SEL55_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT54 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel54(&mut self) -> SEL54_W<0> {
        SEL54_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT55 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel55(&mut self) -> SEL55_W<8> {
        SEL55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel27](index.html) module"]
pub struct SEL27_SPEC;
impl crate::RegisterSpec for SEL27_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel27::R](R) reader structure"]
impl crate::Readable for SEL27_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel27::W](W) writer structure"]
impl crate::Writable for SEL27_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL27 to value 0"]
impl crate::Resettable for SEL27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
