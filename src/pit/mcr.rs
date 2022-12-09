#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRZ` reader - Freeze"]
pub type FRZ_R = crate::BitReader<FRZ_A>;
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZ_A {
    #[doc = "0: Timers continue to run in Debug mode."]
    T000001 = 0,
    #[doc = "1: Timers are stopped in Debug mode."]
    T0000011 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::T000001,
            true => FRZ_A::T0000011,
        }
    }
    #[doc = "Checks if the value of the field is `T000001`"]
    #[inline(always)]
    pub fn is_t000001(&self) -> bool {
        *self == FRZ_A::T000001
    }
    #[doc = "Checks if the value of the field is `T0000011`"]
    #[inline(always)]
    pub fn is_t0000011(&self) -> bool {
        *self == FRZ_A::T0000011
    }
}
#[doc = "Field `FRZ` writer - Freeze"]
pub type FRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, FRZ_A, O>;
impl<'a, const O: u8> FRZ_W<'a, O> {
    #[doc = "Timers continue to run in Debug mode."]
    #[inline(always)]
    pub fn t000001(self) -> &'a mut W {
        self.variant(FRZ_A::T000001)
    }
    #[doc = "Timers are stopped in Debug mode."]
    #[inline(always)]
    pub fn t0000011(self) -> &'a mut W {
        self.variant(FRZ_A::T0000011)
    }
}
#[doc = "Field `MDIS` reader - Module Disable for PIT"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Module Disable for PIT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: Clock for standard PIT timers is enabled."]
    T0301 = 0,
    #[doc = "1: Clock for standard PIT timers is disabled."]
    T00000111 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::T0301,
            true => MDIS_A::T00000111,
        }
    }
    #[doc = "Checks if the value of the field is `T0301`"]
    #[inline(always)]
    pub fn is_t0301(&self) -> bool {
        *self == MDIS_A::T0301
    }
    #[doc = "Checks if the value of the field is `T00000111`"]
    #[inline(always)]
    pub fn is_t00000111(&self) -> bool {
        *self == MDIS_A::T00000111
    }
}
#[doc = "Field `MDIS` writer - Module Disable for PIT"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "Clock for standard PIT timers is enabled."]
    #[inline(always)]
    pub fn t0301(self) -> &'a mut W {
        self.variant(MDIS_A::T0301)
    }
    #[doc = "Clock for standard PIT timers is disabled."]
    #[inline(always)]
    pub fn t00000111(self) -> &'a mut W {
        self.variant(MDIS_A::T00000111)
    }
}
impl R {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable for PIT"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FRZ_W<0> {
        FRZ_W::new(self)
    }
    #[doc = "Bit 1 - Module Disable for PIT"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<1> {
        MDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0x02"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
