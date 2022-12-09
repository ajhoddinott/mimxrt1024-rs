#[doc = "Register `SEL22` reader"]
pub struct R(crate::R<SEL22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL22` writer"]
pub struct W(crate::W<SEL22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL22_SPEC>;
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
impl From<crate::W<SEL22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL44` reader - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
pub type SEL44_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL44` writer - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
pub type SEL44_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL22_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL45` reader - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
pub type SEL45_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL45` writer - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
pub type SEL45_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL22_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel44(&self) -> SEL44_R {
        SEL44_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel45(&self) -> SEL45_R {
        SEL45_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT44 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel44(&mut self) -> SEL44_W<0> {
        SEL44_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT45 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel45(&mut self) -> SEL45_W<8> {
        SEL45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel22](index.html) module"]
pub struct SEL22_SPEC;
impl crate::RegisterSpec for SEL22_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel22::R](R) reader structure"]
impl crate::Readable for SEL22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel22::W](W) writer structure"]
impl crate::Writable for SEL22_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL22 to value 0"]
impl crate::Resettable for SEL22_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
