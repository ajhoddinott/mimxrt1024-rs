#[doc = "Register `SEL47` reader"]
pub struct R(crate::R<SEL47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL47` writer"]
pub struct W(crate::W<SEL47_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL47_SPEC>;
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
impl From<crate::W<SEL47_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL47_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL94` reader - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
pub type SEL94_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL94` writer - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
pub type SEL94_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL47_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL95` reader - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
pub type SEL95_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL95` writer - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
pub type SEL95_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL47_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel94(&self) -> SEL94_R {
        SEL94_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel95(&self) -> SEL95_R {
        SEL95_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT94 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel94(&mut self) -> SEL94_W<0> {
        SEL94_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT95 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel95(&mut self) -> SEL95_W<8> {
        SEL95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel47](index.html) module"]
pub struct SEL47_SPEC;
impl crate::RegisterSpec for SEL47_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel47::R](R) reader structure"]
impl crate::Readable for SEL47_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel47::W](W) writer structure"]
impl crate::Writable for SEL47_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL47 to value 0"]
impl crate::Resettable for SEL47_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
