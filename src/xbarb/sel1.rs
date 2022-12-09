#[doc = "Register `SEL1` reader"]
pub struct R(crate::R<SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL1` writer"]
pub struct W(crate::W<SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL1_SPEC>;
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
impl From<crate::W<SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL2` reader - Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
pub type SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL2` writer - Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
pub type SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `SEL3` reader - Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
pub type SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL3` writer - Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
pub type SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT2 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<0> {
        SEL2_W::new(self)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT3 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL3_W<8> {
        SEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar B Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel1](index.html) module"]
pub struct SEL1_SPEC;
impl crate::RegisterSpec for SEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel1::R](R) reader structure"]
impl crate::Readable for SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel1::W](W) writer structure"]
impl crate::Writable for SEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL1 to value 0"]
impl crate::Resettable for SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
