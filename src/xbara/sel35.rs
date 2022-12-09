#[doc = "Register `SEL35` reader"]
pub struct R(crate::R<SEL35_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL35_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL35_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL35_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL35` writer"]
pub struct W(crate::W<SEL35_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL35_SPEC>;
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
impl From<crate::W<SEL35_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL35_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL70` reader - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
pub type SEL70_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL70` writer - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
pub type SEL70_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL35_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL71` reader - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
pub type SEL71_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL71` writer - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
pub type SEL71_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL35_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel70(&self) -> SEL70_R {
        SEL70_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel71(&self) -> SEL71_R {
        SEL71_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT70 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel70(&mut self) -> SEL70_W<0> {
        SEL70_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT71 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel71(&mut self) -> SEL71_W<8> {
        SEL71_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel35](index.html) module"]
pub struct SEL35_SPEC;
impl crate::RegisterSpec for SEL35_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel35::R](R) reader structure"]
impl crate::Readable for SEL35_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel35::W](W) writer structure"]
impl crate::Writable for SEL35_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL35 to value 0"]
impl crate::Resettable for SEL35_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
