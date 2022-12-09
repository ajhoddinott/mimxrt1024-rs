#[doc = "Register `SEL44` reader"]
pub struct R(crate::R<SEL44_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL44_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL44_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL44_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL44` writer"]
pub struct W(crate::W<SEL44_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL44_SPEC>;
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
impl From<crate::W<SEL44_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL44_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL88` reader - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
pub type SEL88_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL88` writer - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
pub type SEL88_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL44_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL89` reader - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
pub type SEL89_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL89` writer - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
pub type SEL89_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL44_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel88(&self) -> SEL88_R {
        SEL88_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel89(&self) -> SEL89_R {
        SEL89_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT88 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel88(&mut self) -> SEL88_W<0> {
        SEL88_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT89 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel89(&mut self) -> SEL89_W<8> {
        SEL89_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 44\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel44](index.html) module"]
pub struct SEL44_SPEC;
impl crate::RegisterSpec for SEL44_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel44::R](R) reader structure"]
impl crate::Readable for SEL44_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel44::W](W) writer structure"]
impl crate::Writable for SEL44_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL44 to value 0"]
impl crate::Resettable for SEL44_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
