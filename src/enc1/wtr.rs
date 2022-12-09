#[doc = "Register `WTR` reader"]
pub struct R(crate::R<WTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTR` writer"]
pub struct W(crate::W<WTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTR_SPEC>;
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
impl From<crate::W<WTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG` reader - WDOG"]
pub type WDOG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDOG` writer - WDOG"]
pub type WDOG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - WDOG"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDOG"]
    #[inline(always)]
    #[must_use]
    pub fn wdog(&mut self) -> WDOG_W<0> {
        WDOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtr](index.html) module"]
pub struct WTR_SPEC;
impl crate::RegisterSpec for WTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wtr::R](R) reader structure"]
impl crate::Readable for WTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtr::W](W) writer structure"]
impl crate::Writable for WTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WTR to value 0"]
impl crate::Resettable for WTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
