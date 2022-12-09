#[doc = "Register `CH3OPTS_TOG` reader"]
pub struct R(crate::R<CH3OPTS_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3OPTS_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3OPTS_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3OPTS_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3OPTS_TOG` writer"]
pub struct W(crate::W<CH3OPTS_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3OPTS_TOG_SPEC>;
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
impl From<crate::W<CH3OPTS_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3OPTS_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECOVERY_TIMER` reader - This field indicates the recovery time for the channel"]
pub type RECOVERY_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RECOVERY_TIMER` writer - This field indicates the recovery time for the channel"]
pub type RECOVERY_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH3OPTS_TOG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub fn recovery_timer(&self) -> RECOVERY_TIMER_R {
        RECOVERY_TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field indicates the recovery time for the channel"]
    #[inline(always)]
    #[must_use]
    pub fn recovery_timer(&mut self) -> RECOVERY_TIMER_W<0> {
        RECOVERY_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP channel 3 options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3opts_tog](index.html) module"]
pub struct CH3OPTS_TOG_SPEC;
impl crate::RegisterSpec for CH3OPTS_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3opts_tog::R](R) reader structure"]
impl crate::Readable for CH3OPTS_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3opts_tog::W](W) writer structure"]
impl crate::Writable for CH3OPTS_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3OPTS_TOG to value 0"]
impl crate::Resettable for CH3OPTS_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
