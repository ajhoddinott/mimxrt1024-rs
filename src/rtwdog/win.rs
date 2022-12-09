#[doc = "Register `WIN` reader"]
pub struct R(crate::R<WIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIN` writer"]
pub struct W(crate::W<WIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIN_SPEC>;
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
impl From<crate::W<WIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINLOW` reader - Low byte of Watchdog Window"]
pub type WINLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WINLOW` writer - Low byte of Watchdog Window"]
pub type WINLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WIN_SPEC, u8, u8, 8, O>;
#[doc = "Field `WINHIGH` reader - High byte of Watchdog Window"]
pub type WINHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WINHIGH` writer - High byte of Watchdog Window"]
pub type WINHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WIN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of Watchdog Window"]
    #[inline(always)]
    #[must_use]
    pub fn winlow(&mut self) -> WINLOW_W<0> {
        WINLOW_W::new(self)
    }
    #[doc = "Bits 8:15 - High byte of Watchdog Window"]
    #[inline(always)]
    #[must_use]
    pub fn winhigh(&mut self) -> WINHIGH_W<8> {
        WINHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win](index.html) module"]
pub struct WIN_SPEC;
impl crate::RegisterSpec for WIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [win::R](R) reader structure"]
impl crate::Readable for WIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [win::W](W) writer structure"]
impl crate::Writable for WIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIN to value 0"]
impl crate::Resettable for WIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
