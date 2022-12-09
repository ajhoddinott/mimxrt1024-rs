#[doc = "Register `TCD30_BITER_ELINKNO` reader"]
pub struct R(crate::R<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD30_BITER_ELINKNO` writer"]
pub struct W(crate::W<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>;
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
impl From<crate::W<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITER` reader - Starting Major Iteration Count"]
pub type BITER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BITER` writer - Starting Major Iteration Count"]
pub type BITER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC, u16, u16, 15, O>;
#[doc = "Field `ELINK` reader - Enables channel-to-channel linking on minor loop complete"]
pub type ELINK_R = crate::BitReader<ELINK_A>;
#[doc = "Enables channel-to-channel linking on minor loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELINK_A {
    #[doc = "0: Channel-to-channel linking is disabled"]
    DISABLED = 0,
    #[doc = "1: Channel-to-channel linking is enabled"]
    ENABLED = 1,
}
impl From<ELINK_A> for bool {
    #[inline(always)]
    fn from(variant: ELINK_A) -> Self {
        variant as u8 != 0
    }
}
impl ELINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELINK_A {
        match self.bits {
            false => ELINK_A::DISABLED,
            true => ELINK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ELINK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ELINK_A::ENABLED
    }
}
#[doc = "Field `ELINK` writer - Enables channel-to-channel linking on minor loop complete"]
pub type ELINK_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC, ELINK_A, O>;
impl<'a, const O: u8> ELINK_W<'a, O> {
    #[doc = "Channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ELINK_A::DISABLED)
    }
    #[doc = "Channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ELINK_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:14 - Starting Major Iteration Count"]
    #[inline(always)]
    pub fn biter(&self) -> BITER_R {
        BITER_R::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ELINK_R {
        ELINK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Starting Major Iteration Count"]
    #[inline(always)]
    #[must_use]
    pub fn biter(&mut self) -> BITER_W<0> {
        BITER_W::new(self)
    }
    #[doc = "Bit 15 - Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn elink(&mut self) -> ELINK_W<15> {
        ELINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_biter_elink_tcd30_biter_elinkno](index.html) module"]
pub struct TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC;
impl crate::RegisterSpec for TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd_biter_elink_tcd30_biter_elinkno::R](R) reader structure"]
impl crate::Readable for TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elink_tcd30_biter_elinkno::W](W) writer structure"]
impl crate::Writable for TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD30_BITER_ELINKNO to value 0"]
impl crate::Resettable for TCD_BITER_ELINK_TCD30_BITER_ELINKNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
