#[doc = "Register `SEL46` reader"]
pub struct R(crate::R<SEL46_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL46_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL46_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL46_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL46` writer"]
pub struct W(crate::W<SEL46_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL46_SPEC>;
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
impl From<crate::W<SEL46_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL46_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL92` reader - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
pub type SEL92_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL92` writer - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
pub type SEL92_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL46_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL93` reader - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
pub type SEL93_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL93` writer - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
pub type SEL93_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL46_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel92(&self) -> SEL92_R {
        SEL92_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel93(&self) -> SEL93_R {
        SEL93_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT92 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel92(&mut self) -> SEL92_W<0> {
        SEL92_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT93 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel93(&mut self) -> SEL93_W<8> {
        SEL93_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 46\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel46](index.html) module"]
pub struct SEL46_SPEC;
impl crate::RegisterSpec for SEL46_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel46::R](R) reader structure"]
impl crate::Readable for SEL46_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel46::W](W) writer structure"]
impl crate::Writable for SEL46_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL46 to value 0"]
impl crate::Resettable for SEL46_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
