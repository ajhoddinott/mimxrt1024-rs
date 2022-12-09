#[doc = "Register `MEGA_SR` reader"]
pub struct R(crate::R<MEGA_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEGA_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEGA_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEGA_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEGA_SR` writer"]
pub struct W(crate::W<MEGA_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEGA_SR_SPEC>;
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
impl From<crate::W<MEGA_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEGA_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSR` reader - Power status"]
pub type PSR_R = crate::BitReader<PSR_A>;
#[doc = "Power status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSR_A {
    #[doc = "0: The target subsystem was not powered down for the previous power-down request."]
    PSR_0 = 0,
    #[doc = "1: The target subsystem was powered down for the previous power-down request."]
    PSR_1 = 1,
}
impl From<PSR_A> for bool {
    #[inline(always)]
    fn from(variant: PSR_A) -> Self {
        variant as u8 != 0
    }
}
impl PSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSR_A {
        match self.bits {
            false => PSR_A::PSR_0,
            true => PSR_A::PSR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSR_0`"]
    #[inline(always)]
    pub fn is_psr_0(&self) -> bool {
        *self == PSR_A::PSR_0
    }
    #[doc = "Checks if the value of the field is `PSR_1`"]
    #[inline(always)]
    pub fn is_psr_1(&self) -> bool {
        *self == PSR_A::PSR_1
    }
}
#[doc = "Field `PSR` writer - Power status"]
pub type PSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEGA_SR_SPEC, PSR_A, O>;
impl<'a, const O: u8> PSR_W<'a, O> {
    #[doc = "The target subsystem was not powered down for the previous power-down request."]
    #[inline(always)]
    pub fn psr_0(self) -> &'a mut W {
        self.variant(PSR_A::PSR_0)
    }
    #[doc = "The target subsystem was powered down for the previous power-down request."]
    #[inline(always)]
    pub fn psr_1(self) -> &'a mut W {
        self.variant(PSR_A::PSR_1)
    }
}
impl R {
    #[doc = "Bit 0 - Power status"]
    #[inline(always)]
    pub fn psr(&self) -> PSR_R {
        PSR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power status"]
    #[inline(always)]
    #[must_use]
    pub fn psr(&mut self) -> PSR_W<0> {
        PSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PGC Mega Power Gating Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mega_sr](index.html) module"]
pub struct MEGA_SR_SPEC;
impl crate::RegisterSpec for MEGA_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mega_sr::R](R) reader structure"]
impl crate::Readable for MEGA_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mega_sr::W](W) writer structure"]
impl crate::Writable for MEGA_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEGA_SR to value 0"]
impl crate::Resettable for MEGA_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
