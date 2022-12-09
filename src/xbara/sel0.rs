#[doc = "Register `SEL0` reader"]
pub struct R(crate::R<SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL0` writer"]
pub struct W(crate::W<SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL0_SPEC>;
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
impl From<crate::W<SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
pub type SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL0` writer - Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
pub type SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL0_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL1` reader - Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
pub type SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL1` writer - Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
pub type SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT0 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<0> {
        SEL0_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT1 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<8> {
        SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel0](index.html) module"]
pub struct SEL0_SPEC;
impl crate::RegisterSpec for SEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel0::R](R) reader structure"]
impl crate::Readable for SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel0::W](W) writer structure"]
impl crate::Writable for SEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL0 to value 0"]
impl crate::Resettable for SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
