#[doc = "Register `PLL_SYS_SS` reader"]
pub struct R(crate::R<PLL_SYS_SS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SYS_SS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SYS_SS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SYS_SS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_SYS_SS` writer"]
pub struct W(crate::W<PLL_SYS_SS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SYS_SS_SPEC>;
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
impl From<crate::W<PLL_SYS_SS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SYS_SS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STEP` writer - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SYS_SS_SPEC, u16, u16, 15, O>;
#[doc = "Field `ENABLE` reader - Enable bit"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Spread spectrum modulation disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Soread spectrum modulation enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLE_A::ENABLE_1
    }
}
#[doc = "Field `ENABLE` writer - Enable bit"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SYS_SS_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Spread spectrum modulation disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Soread spectrum modulation enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
    }
}
#[doc = "Field `STOP` reader - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
pub type STOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STOP` writer - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SYS_SS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:14 - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Frequency change step = step/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<0> {
        STEP_W::new(self)
    }
    #[doc = "Bit 15 - Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<15> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 16:31 - Frequency change = stop/CCM_ANALOG_PLL_SYS_DENOM\\[B\\]*24MHz."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<16> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "528MHz System PLL Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_sys_ss](index.html) module"]
pub struct PLL_SYS_SS_SPEC;
impl crate::RegisterSpec for PLL_SYS_SS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_sys_ss::R](R) reader structure"]
impl crate::Readable for PLL_SYS_SS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_sys_ss::W](W) writer structure"]
impl crate::Writable for PLL_SYS_SS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_SYS_SS to value 0"]
impl crate::Resettable for PLL_SYS_SS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
