#[doc = "Register `TOVAL` reader"]
pub struct R(crate::R<TOVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVAL` writer"]
pub struct W(crate::W<TOVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOVAL_SPEC>;
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
impl From<crate::W<TOVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALLOW` reader - Low byte of the timeout value"]
pub type TOVALLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOVALLOW` writer - Low byte of the timeout value"]
pub type TOVALLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOVAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `TOVALHIGH` reader - High byte of the timeout value"]
pub type TOVALHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOVALHIGH` writer - High byte of the timeout value"]
pub type TOVALHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOVAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of the timeout value"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of the timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn tovallow(&mut self) -> TOVALLOW_W<0> {
        TOVALLOW_W::new(self)
    }
    #[doc = "Bits 8:15 - High byte of the timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W<8> {
        TOVALHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timeout Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [toval](index.html) module"]
pub struct TOVAL_SPEC;
impl crate::RegisterSpec for TOVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [toval::R](R) reader structure"]
impl crate::Readable for TOVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [toval::W](W) writer structure"]
impl crate::Writable for TOVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOVAL to value 0x0400"]
impl crate::Resettable for TOVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
