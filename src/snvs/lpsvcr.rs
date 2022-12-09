#[doc = "Register `LPSVCR` reader"]
pub struct R(crate::R<LPSVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSVCR` writer"]
pub struct W(crate::W<LPSVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSVCR_SPEC>;
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
impl From<crate::W<LPSVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SV0_EN` reader - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
pub type SV0_EN_R = crate::BitReader<SV0_EN_A>;
#[doc = "Security Violation 0 Enable This bit enables Security Violation 0 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV0_EN_A {
    #[doc = "0: Security Violation 0 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 0 is enabled in the LP domain."]
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
#[doc = "Field `SV0_EN` writer - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
pub type SV0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV0_EN_A, O>;
impl<'a, const O: u8> SV0_EN_W<'a, O> {
    #[doc = "Security Violation 0 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV0_EN_A::DISABLED)
    }
    #[doc = "Security Violation 0 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV0_EN_A::ENABLED)
    }
}
#[doc = "Field `SV1_EN` reader - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
pub type SV1_EN_R = crate::BitReader<SV1_EN_A>;
#[doc = "Security Violation 1 Enable This bit enables Security Violation 1 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV1_EN_A {
    #[doc = "0: Security Violation 1 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 1 is enabled in the LP domain."]
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
#[doc = "Field `SV1_EN` writer - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
pub type SV1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV1_EN_A, O>;
impl<'a, const O: u8> SV1_EN_W<'a, O> {
    #[doc = "Security Violation 1 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV1_EN_A::DISABLED)
    }
    #[doc = "Security Violation 1 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV1_EN_A::ENABLED)
    }
}
#[doc = "Field `SV2_EN` reader - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
pub type SV2_EN_R = crate::BitReader<SV2_EN_A>;
#[doc = "Security Violation 2 Enable This bit enables Security Violation 2 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV2_EN_A {
    #[doc = "0: Security Violation 2 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 2 is enabled in the LP domain."]
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
#[doc = "Field `SV2_EN` writer - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
pub type SV2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV2_EN_A, O>;
impl<'a, const O: u8> SV2_EN_W<'a, O> {
    #[doc = "Security Violation 2 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV2_EN_A::DISABLED)
    }
    #[doc = "Security Violation 2 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV2_EN_A::ENABLED)
    }
}
#[doc = "Field `SV3_EN` reader - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
pub type SV3_EN_R = crate::BitReader<SV3_EN_A>;
#[doc = "Security Violation 3 Enable This bit enables Security Violation 3 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV3_EN_A {
    #[doc = "0: Security Violation 3 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 3 is enabled in the LP domain."]
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
#[doc = "Field `SV3_EN` writer - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
pub type SV3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV3_EN_A, O>;
impl<'a, const O: u8> SV3_EN_W<'a, O> {
    #[doc = "Security Violation 3 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV3_EN_A::DISABLED)
    }
    #[doc = "Security Violation 3 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV3_EN_A::ENABLED)
    }
}
#[doc = "Field `SV4_EN` reader - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
pub type SV4_EN_R = crate::BitReader<SV4_EN_A>;
#[doc = "Security Violation 4 Enable This bit enables Security Violation 4 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV4_EN_A {
    #[doc = "0: Security Violation 4 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 4 is enabled in the LP domain."]
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
#[doc = "Field `SV4_EN` writer - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
pub type SV4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV4_EN_A, O>;
impl<'a, const O: u8> SV4_EN_W<'a, O> {
    #[doc = "Security Violation 4 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV4_EN_A::DISABLED)
    }
    #[doc = "Security Violation 4 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV4_EN_A::ENABLED)
    }
}
#[doc = "Field `SV5_EN` reader - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
pub type SV5_EN_R = crate::BitReader<SV5_EN_A>;
#[doc = "Security Violation 5 Enable This bit enables Security Violation 5 Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV5_EN_A {
    #[doc = "0: Security Violation 5 is disabled in the LP domain."]
    DISABLED = 0,
    #[doc = "1: Security Violation 5 is enabled in the LP domain."]
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
#[doc = "Field `SV5_EN` writer - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
pub type SV5_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPSVCR_SPEC, SV5_EN_A, O>;
impl<'a, const O: u8> SV5_EN_W<'a, O> {
    #[doc = "Security Violation 5 is disabled in the LP domain."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV5_EN_A::DISABLED)
    }
    #[doc = "Security Violation 5 is enabled in the LP domain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SV5_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline(always)]
    pub fn sv0_en(&self) -> SV0_EN_R {
        SV0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline(always)]
    pub fn sv1_en(&self) -> SV1_EN_R {
        SV1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline(always)]
    pub fn sv2_en(&self) -> SV2_EN_R {
        SV2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline(always)]
    pub fn sv3_en(&self) -> SV3_EN_R {
        SV3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline(always)]
    pub fn sv4_en(&self) -> SV4_EN_R {
        SV4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline(always)]
    pub fn sv5_en(&self) -> SV5_EN_R {
        SV5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 Enable This bit enables Security Violation 0 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv0_en(&mut self) -> SV0_EN_W<0> {
        SV0_EN_W::new(self)
    }
    #[doc = "Bit 1 - Security Violation 1 Enable This bit enables Security Violation 1 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv1_en(&mut self) -> SV1_EN_W<1> {
        SV1_EN_W::new(self)
    }
    #[doc = "Bit 2 - Security Violation 2 Enable This bit enables Security Violation 2 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv2_en(&mut self) -> SV2_EN_W<2> {
        SV2_EN_W::new(self)
    }
    #[doc = "Bit 3 - Security Violation 3 Enable This bit enables Security Violation 3 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv3_en(&mut self) -> SV3_EN_W<3> {
        SV3_EN_W::new(self)
    }
    #[doc = "Bit 4 - Security Violation 4 Enable This bit enables Security Violation 4 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv4_en(&mut self) -> SV4_EN_W<4> {
        SV4_EN_W::new(self)
    }
    #[doc = "Bit 5 - Security Violation 5 Enable This bit enables Security Violation 5 Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv5_en(&mut self) -> SV5_EN_W<5> {
        SV5_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Security Violation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsvcr](index.html) module"]
pub struct LPSVCR_SPEC;
impl crate::RegisterSpec for LPSVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsvcr::R](R) reader structure"]
impl crate::Readable for LPSVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsvcr::W](W) writer structure"]
impl crate::Writable for LPSVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSVCR to value 0"]
impl crate::Resettable for LPSVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
