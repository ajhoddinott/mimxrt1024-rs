#[doc = "Register `SEL34` reader"]
pub struct R(crate::R<SEL34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL34` writer"]
pub struct W(crate::W<SEL34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL34_SPEC>;
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
impl From<crate::W<SEL34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL68` reader - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
pub type SEL68_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL68` writer - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
pub type SEL68_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL34_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL69` reader - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
pub type SEL69_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL69` writer - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
pub type SEL69_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL34_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel68(&self) -> SEL68_R {
        SEL68_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel69(&self) -> SEL69_R {
        SEL69_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT68 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel68(&mut self) -> SEL68_W<0> {
        SEL68_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT69 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel69(&mut self) -> SEL69_W<8> {
        SEL69_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel34](index.html) module"]
pub struct SEL34_SPEC;
impl crate::RegisterSpec for SEL34_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel34::R](R) reader structure"]
impl crate::Readable for SEL34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel34::W](W) writer structure"]
impl crate::Writable for SEL34_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL34 to value 0"]
impl crate::Resettable for SEL34_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
