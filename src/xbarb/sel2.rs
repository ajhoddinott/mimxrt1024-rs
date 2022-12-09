#[doc = "Register `SEL2` reader"]
pub struct R(crate::R<SEL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL2` writer"]
pub struct W(crate::W<SEL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL2_SPEC>;
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
impl From<crate::W<SEL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL4` reader - Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
pub type SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL4` writer - Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
pub type SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL2_SPEC, u8, u8, 6, O>;
#[doc = "Field `SEL5` reader - Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
pub type SEL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL5` writer - Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
pub type SEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT4 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<0> {
        SEL4_W::new(self)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT5 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL5_W<8> {
        SEL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar B Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel2](index.html) module"]
pub struct SEL2_SPEC;
impl crate::RegisterSpec for SEL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel2::R](R) reader structure"]
impl crate::Readable for SEL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel2::W](W) writer structure"]
impl crate::Writable for SEL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL2 to value 0"]
impl crate::Resettable for SEL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
