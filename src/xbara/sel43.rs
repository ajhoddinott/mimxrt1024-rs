#[doc = "Register `SEL43` reader"]
pub struct R(crate::R<SEL43_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL43_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL43_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL43_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL43` writer"]
pub struct W(crate::W<SEL43_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL43_SPEC>;
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
impl From<crate::W<SEL43_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL43_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL86` reader - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
pub type SEL86_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL86` writer - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
pub type SEL86_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL43_SPEC, u8, u8, 7, O>;
#[doc = "Field `SEL87` reader - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
pub type SEL87_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL87` writer - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
pub type SEL87_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SEL43_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel86(&self) -> SEL86_R {
        SEL86_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    pub fn sel87(&self) -> SEL87_R {
        SEL87_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input (XBARA_INn) to be muxed to XBARA_OUT86 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel86(&mut self) -> SEL86_W<0> {
        SEL86_W::new(self)
    }
    #[doc = "Bits 8:14 - Input (XBARA_INn) to be muxed to XBARA_OUT87 (refer to Functional Description section for input/output assignment)"]
    #[inline(always)]
    #[must_use]
    pub fn sel87(&mut self) -> SEL87_W<8> {
        SEL87_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Select Register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel43](index.html) module"]
pub struct SEL43_SPEC;
impl crate::RegisterSpec for SEL43_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sel43::R](R) reader structure"]
impl crate::Readable for SEL43_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel43::W](W) writer structure"]
impl crate::Writable for SEL43_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL43 to value 0"]
impl crate::Resettable for SEL43_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
