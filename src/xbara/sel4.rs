#[doc = "Register `SEL4` reader"]
pub struct R(crate::R<SEL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL4` writer"]
pub struct W(crate::W<SEL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL4_SPEC>;
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
impl From<crate::W<SEL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL8` reader - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
pub type SEL8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL8` writer - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
pub type SEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL4_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL9` reader - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
pub type SEL9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL9` writer - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
pub type SEL9_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL4_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel9(&self) -> SEL9_R {
        SEL9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT8 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL8_W<0> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT9 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel9(&mut self) -> SEL9_W<8> {
        SEL9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel4](index.html) module"]
pub struct SEL4_SPEC;
impl crate::RegisterSpec for SEL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel4::R](R) reader structure"]
impl crate::Readable for SEL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel4::W](W) writer structure"]
impl crate::Writable for SEL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL4 to value 0"]
impl crate::Resettable for SEL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
