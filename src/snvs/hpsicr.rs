#[doc = "Register `HPSICR` reader"]
pub struct R(crate::R<HPSICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPSICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPSICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPSICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPSICR` writer"]
pub struct W(crate::W<HPSICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPSICR_SPEC>;
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
impl From<crate::W<HPSICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPSICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SV0_EN` reader - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
pub type SV0_EN_R = crate::BitReader<SV0_EN_A>;
#[doc = "Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV0_EN_A {
    #[doc = "0: Security Violation 0 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 0 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV0_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_EN_A {
        match self.bits {
            false => SV0_EN_A::DISABLED,
            true => SV0_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV0_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV0_EN_A::ENABLED
    }
}
#[doc = "Field `SV0_EN` writer - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
pub type SV0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV0_EN_A, O>;
impl<'a, const O: u8> SV0_EN_W<'a, O> {
    #[doc = "Security Violation 0 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV0_EN_A::DISABLED)
    }
    #[doc = "Security Violation 0 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV0_EN_A::ENABLED)
    }
}
#[doc = "Field `SV1_EN` reader - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
pub type SV1_EN_R = crate::BitReader<SV1_EN_A>;
#[doc = "Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV1_EN_A {
    #[doc = "0: Security Violation 1 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 1 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_EN_A {
        match self.bits {
            false => SV1_EN_A::DISABLED,
            true => SV1_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV1_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV1_EN_A::ENABLED
    }
}
#[doc = "Field `SV1_EN` writer - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
pub type SV1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV1_EN_A, O>;
impl<'a, const O: u8> SV1_EN_W<'a, O> {
    #[doc = "Security Violation 1 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV1_EN_A::DISABLED)
    }
    #[doc = "Security Violation 1 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV1_EN_A::ENABLED)
    }
}
#[doc = "Field `SV2_EN` reader - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
pub type SV2_EN_R = crate::BitReader<SV2_EN_A>;
#[doc = "Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV2_EN_A {
    #[doc = "0: Security Violation 2 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 2 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_EN_A {
        match self.bits {
            false => SV2_EN_A::DISABLED,
            true => SV2_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV2_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV2_EN_A::ENABLED
    }
}
#[doc = "Field `SV2_EN` writer - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
pub type SV2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV2_EN_A, O>;
impl<'a, const O: u8> SV2_EN_W<'a, O> {
    #[doc = "Security Violation 2 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV2_EN_A::DISABLED)
    }
    #[doc = "Security Violation 2 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV2_EN_A::ENABLED)
    }
}
#[doc = "Field `SV3_EN` reader - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
pub type SV3_EN_R = crate::BitReader<SV3_EN_A>;
#[doc = "Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV3_EN_A {
    #[doc = "0: Security Violation 3 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 3 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV3_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_EN_A {
        match self.bits {
            false => SV3_EN_A::DISABLED,
            true => SV3_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV3_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV3_EN_A::ENABLED
    }
}
#[doc = "Field `SV3_EN` writer - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
pub type SV3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV3_EN_A, O>;
impl<'a, const O: u8> SV3_EN_W<'a, O> {
    #[doc = "Security Violation 3 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV3_EN_A::DISABLED)
    }
    #[doc = "Security Violation 3 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV3_EN_A::ENABLED)
    }
}
#[doc = "Field `SV4_EN` reader - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
pub type SV4_EN_R = crate::BitReader<SV4_EN_A>;
#[doc = "Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV4_EN_A {
    #[doc = "0: Security Violation 4 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 4 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_EN_A {
        match self.bits {
            false => SV4_EN_A::DISABLED,
            true => SV4_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV4_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV4_EN_A::ENABLED
    }
}
#[doc = "Field `SV4_EN` writer - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
pub type SV4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV4_EN_A, O>;
impl<'a, const O: u8> SV4_EN_W<'a, O> {
    #[doc = "Security Violation 4 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV4_EN_A::DISABLED)
    }
    #[doc = "Security Violation 4 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV4_EN_A::ENABLED)
    }
}
#[doc = "Field `SV5_EN` reader - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
pub type SV5_EN_R = crate::BitReader<SV5_EN_A>;
#[doc = "Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV5_EN_A {
    #[doc = "0: Security Violation 5 Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 5 Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<SV5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SV5_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SV5_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV5_EN_A {
        match self.bits {
            false => SV5_EN_A::DISABLED,
            true => SV5_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV5_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SV5_EN_A::ENABLED
    }
}
#[doc = "Field `SV5_EN` writer - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
pub type SV5_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, SV5_EN_A, O>;
impl<'a, const O: u8> SV5_EN_W<'a, O> {
    #[doc = "Security Violation 5 Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV5_EN_A::DISABLED)
    }
    #[doc = "Security Violation 5 Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV5_EN_A::ENABLED)
    }
}
#[doc = "Field `LPSVI_EN` reader - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
pub type LPSVI_EN_R = crate::BitReader<LPSVI_EN_A>;
#[doc = "LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSVI_EN_A {
    #[doc = "0: LP Security Violation Interrupt is Disabled"]
    DISABLED = 0,
    #[doc = "1: LP Security Violation Interrupt is Enabled"]
    ENABLED = 1,
}
impl From<LPSVI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSVI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVI_EN_A {
        match self.bits {
            false => LPSVI_EN_A::DISABLED,
            true => LPSVI_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPSVI_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPSVI_EN_A::ENABLED
    }
}
#[doc = "Field `LPSVI_EN` writer - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
pub type LPSVI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSICR_SPEC, LPSVI_EN_A, O>;
impl<'a, const O: u8> LPSVI_EN_W<'a, O> {
    #[doc = "LP Security Violation Interrupt is Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPSVI_EN_A::DISABLED)
    }
    #[doc = "LP Security Violation Interrupt is Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPSVI_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    pub fn sv0_en(&self) -> SV0_EN_R {
        SV0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    pub fn sv1_en(&self) -> SV1_EN_R {
        SV1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    pub fn sv2_en(&self) -> SV2_EN_R {
        SV2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    pub fn sv3_en(&self) -> SV3_EN_R {
        SV3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    pub fn sv4_en(&self) -> SV4_EN_R {
        SV4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    pub fn sv5_en(&self) -> SV5_EN_R {
        SV5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    pub fn lpsvi_en(&self) -> LPSVI_EN_R {
        LPSVI_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 0 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv0_en(&mut self) -> SV0_EN_W<0> {
        SV0_EN_W::new(self)
    }
    #[doc = "Bit 1 - Security Violation 1 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 1 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv1_en(&mut self) -> SV1_EN_W<1> {
        SV1_EN_W::new(self)
    }
    #[doc = "Bit 2 - Security Violation 2 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 2 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv2_en(&mut self) -> SV2_EN_W<2> {
        SV2_EN_W::new(self)
    }
    #[doc = "Bit 3 - Security Violation 3 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 3 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv3_en(&mut self) -> SV3_EN_W<3> {
        SV3_EN_W::new(self)
    }
    #[doc = "Bit 4 - Security Violation 4 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 4 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv4_en(&mut self) -> SV4_EN_W<4> {
        SV4_EN_W::new(self)
    }
    #[doc = "Bit 5 - Security Violation 5 Interrupt Enable Setting this bit to 1 enables generation of the security interrupt to the host processor upon detection of the Security Violation 5 security violation"]
    #[inline(always)]
    #[must_use]
    pub fn sv5_en(&mut self) -> SV5_EN_W<5> {
        SV5_EN_W::new(self)
    }
    #[doc = "Bit 31 - LP Security Violation Interrupt Enable This bit enables generating of the security interrupt to the host processor upon security violation signal from the LP section"]
    #[inline(always)]
    #[must_use]
    pub fn lpsvi_en(&mut self) -> LPSVI_EN_W<31> {
        LPSVI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Security Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsicr](index.html) module"]
pub struct HPSICR_SPEC;
impl crate::RegisterSpec for HPSICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpsicr::R](R) reader structure"]
impl crate::Readable for HPSICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpsicr::W](W) writer structure"]
impl crate::Writable for HPSICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPSICR to value 0"]
impl crate::Resettable for HPSICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
