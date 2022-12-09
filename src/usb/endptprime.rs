#[doc = "Register `ENDPTPRIME` reader"]
pub struct R(crate::R<ENDPTPRIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTPRIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTPRIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTPRIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTPRIME` writer"]
pub struct W(crate::W<ENDPTPRIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTPRIME_SPEC>;
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
impl From<crate::W<ENDPTPRIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTPRIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERB` reader - Prime Endpoint Receive Buffer - R/WS"]
pub type PERB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERB` writer - Prime Endpoint Receive Buffer - R/WS"]
pub type PERB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTPRIME_SPEC, u8, u8, 8, O>;
#[doc = "Field `PETB` reader - Prime Endpoint Transmit Buffer - R/WS"]
pub type PETB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PETB` writer - Prime Endpoint Transmit Buffer - R/WS"]
pub type PETB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTPRIME_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Prime Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Prime Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    pub fn petb(&self) -> PETB_R {
        PETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prime Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PERB_W<0> {
        PERB_W::new(self)
    }
    #[doc = "Bits 16:23 - Prime Endpoint Transmit Buffer - R/WS"]
    #[inline(always)]
    #[must_use]
    pub fn petb(&mut self) -> PETB_W<16> {
        PETB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Prime\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptprime](index.html) module"]
pub struct ENDPTPRIME_SPEC;
impl crate::RegisterSpec for ENDPTPRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptprime::R](R) reader structure"]
impl crate::Readable for ENDPTPRIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptprime::W](W) writer structure"]
impl crate::Writable for ENDPTPRIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPTPRIME to value 0"]
impl crate::Resettable for ENDPTPRIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
