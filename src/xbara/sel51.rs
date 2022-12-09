#[doc = "Register `SEL51` reader"]
pub struct R(crate::R<SEL51_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL51_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL51_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL51_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL51` writer"]
pub struct W(crate::W<SEL51_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL51_SPEC>;
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
impl From<crate::W<SEL51_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL51_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL102` reader - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
pub type SEL102_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL102` writer - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
pub type SEL102_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL51_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL103` reader - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
pub type SEL103_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL103` writer - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
pub type SEL103_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL51_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel102(&self) -> SEL102_R {
        SEL102_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel103(&self) -> SEL103_R {
        SEL103_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT102 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel102(&mut self) -> SEL102_W<0> {
        SEL102_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT103 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel103(&mut self) -> SEL103_W<8> {
        SEL103_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 51\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel51](index.html) module"]
pub struct SEL51_SPEC;
impl crate::RegisterSpec for SEL51_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel51::R](R) reader structure"]
impl crate::Readable for SEL51_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel51::W](W) writer structure"]
impl crate::Writable for SEL51_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL51 to value 0"]
impl crate::Resettable for SEL51_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
