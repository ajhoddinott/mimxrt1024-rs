#[doc = "Register `SEL5` reader"]
pub struct R(crate::R<SEL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL5` writer"]
pub struct W(crate::W<SEL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL5_SPEC>;
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
impl From<crate::W<SEL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL10` reader - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
pub type SEL10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL10` writer - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
pub type SEL10_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL5_SPEC, u8, u8, 6, O>;
#[doc = "Field `SEL11` reader - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
pub type SEL11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL11` writer - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
pub type SEL11_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL5_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBARB_INn) to be muxed to XBARB_OUT10 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL10_W<0> {
        SEL10_W::new(self)
    }
    #[doc = "Bits 8:13 - Input (XBARB_INn) to be muxed to XBARB_OUT11 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL11_W<8> {
        SEL11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar B Select Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel5](index.html) module"]
pub struct SEL5_SPEC;
impl crate::RegisterSpec for SEL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel5::R](R) reader structure"]
impl crate::Readable for SEL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel5::W](W) writer structure"]
impl crate::Writable for SEL5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL5 to value 0"]
impl crate::Resettable for SEL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
