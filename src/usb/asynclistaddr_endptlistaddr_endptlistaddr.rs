#[doc = "Register `ENDPTLISTADDR` reader"]
pub struct R(crate::R<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTLISTADDR` writer"]
pub struct W(crate::W<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>;
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
impl From<crate::W<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPBASE` reader - Endpoint List Pointer(Low)"]
pub type EPBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPBASE` writer - Endpoint List Pointer(Low)"]
pub type EPBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 11:31 - Endpoint List Pointer(Low)"]
    #[inline(always)]
    pub fn epbase(&self) -> EPBASE_R {
        EPBASE_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 11:31 - Endpoint List Pointer(Low)"]
    #[inline(always)]
    #[must_use]
    pub fn epbase(&mut self) -> EPBASE_W<11> {
        EPBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asynclistaddr_endptlistaddr_endptlistaddr](index.html) module"]
pub struct ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC;
impl crate::RegisterSpec for ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asynclistaddr_endptlistaddr_endptlistaddr::R](R) reader structure"]
impl crate::Readable for ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asynclistaddr_endptlistaddr_endptlistaddr::W](W) writer structure"]
impl crate::Writable for ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTLISTADDR to value 0"]
impl crate::Resettable for ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
