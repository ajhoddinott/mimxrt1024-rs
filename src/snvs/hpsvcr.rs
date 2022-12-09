#[doc = "Register `HPSVCR` reader"]
pub struct R(crate::R<HPSVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPSVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPSVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPSVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPSVCR` writer"]
pub struct W(crate::W<HPSVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPSVCR_SPEC>;
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
impl From<crate::W<HPSVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPSVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SV0_CFG` reader - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
pub type SV0_CFG_R = crate::BitReader<SV0_CFG_A>;
#[doc = "Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV0_CFG_A {
    #[doc = "0: Security Violation 0 is a non-fatal violation"]
    NON_FATAL = 0,
    #[doc = "1: Security Violation 0 is a fatal violation"]
    FATAL = 1,
}
impl From<SV0_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SV0_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_CFG_A {
        match self.bits {
            false => SV0_CFG_A::NON_FATAL,
            true => SV0_CFG_A::FATAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV0_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV0_CFG_A::FATAL
    }
}
#[doc = "Field `SV0_CFG` writer - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
pub type SV0_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSVCR_SPEC, SV0_CFG_A, O>;
impl<'a, const O: u8> SV0_CFG_W<'a, O> {
    #[doc = "Security Violation 0 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV0_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 0 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV0_CFG_A::FATAL)
    }
}
#[doc = "Field `SV1_CFG` reader - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
pub type SV1_CFG_R = crate::BitReader<SV1_CFG_A>;
#[doc = "Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV1_CFG_A {
    #[doc = "0: Security Violation 1 is a non-fatal violation"]
    NON_FATAL = 0,
    #[doc = "1: Security Violation 1 is a fatal violation"]
    FATAL = 1,
}
impl From<SV1_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SV1_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_CFG_A {
        match self.bits {
            false => SV1_CFG_A::NON_FATAL,
            true => SV1_CFG_A::FATAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV1_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV1_CFG_A::FATAL
    }
}
#[doc = "Field `SV1_CFG` writer - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
pub type SV1_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSVCR_SPEC, SV1_CFG_A, O>;
impl<'a, const O: u8> SV1_CFG_W<'a, O> {
    #[doc = "Security Violation 1 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV1_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 1 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV1_CFG_A::FATAL)
    }
}
#[doc = "Field `SV2_CFG` reader - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
pub type SV2_CFG_R = crate::BitReader<SV2_CFG_A>;
#[doc = "Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV2_CFG_A {
    #[doc = "0: Security Violation 2 is a non-fatal violation"]
    NON_FATAL = 0,
    #[doc = "1: Security Violation 2 is a fatal violation"]
    FATAL = 1,
}
impl From<SV2_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SV2_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_CFG_A {
        match self.bits {
            false => SV2_CFG_A::NON_FATAL,
            true => SV2_CFG_A::FATAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV2_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV2_CFG_A::FATAL
    }
}
#[doc = "Field `SV2_CFG` writer - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
pub type SV2_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSVCR_SPEC, SV2_CFG_A, O>;
impl<'a, const O: u8> SV2_CFG_W<'a, O> {
    #[doc = "Security Violation 2 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV2_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 2 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV2_CFG_A::FATAL)
    }
}
#[doc = "Field `SV3_CFG` reader - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
pub type SV3_CFG_R = crate::BitReader<SV3_CFG_A>;
#[doc = "Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV3_CFG_A {
    #[doc = "0: Security Violation 3 is a non-fatal violation"]
    NON_FATAL = 0,
    #[doc = "1: Security Violation 3 is a fatal violation"]
    FATAL = 1,
}
impl From<SV3_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SV3_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_CFG_A {
        match self.bits {
            false => SV3_CFG_A::NON_FATAL,
            true => SV3_CFG_A::FATAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV3_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV3_CFG_A::FATAL
    }
}
#[doc = "Field `SV3_CFG` writer - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
pub type SV3_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSVCR_SPEC, SV3_CFG_A, O>;
impl<'a, const O: u8> SV3_CFG_W<'a, O> {
    #[doc = "Security Violation 3 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV3_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 3 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV3_CFG_A::FATAL)
    }
}
#[doc = "Field `SV4_CFG` reader - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
pub type SV4_CFG_R = crate::BitReader<SV4_CFG_A>;
#[doc = "Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV4_CFG_A {
    #[doc = "0: Security Violation 4 is a non-fatal violation"]
    NON_FATAL = 0,
    #[doc = "1: Security Violation 4 is a fatal violation"]
    FATAL = 1,
}
impl From<SV4_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl SV4_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_CFG_A {
        match self.bits {
            false => SV4_CFG_A::NON_FATAL,
            true => SV4_CFG_A::FATAL,
        }
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV4_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV4_CFG_A::FATAL
    }
}
#[doc = "Field `SV4_CFG` writer - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
pub type SV4_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPSVCR_SPEC, SV4_CFG_A, O>;
impl<'a, const O: u8> SV4_CFG_W<'a, O> {
    #[doc = "Security Violation 4 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV4_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 4 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV4_CFG_A::FATAL)
    }
}
#[doc = "Field `SV5_CFG` reader - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
pub type SV5_CFG_R = crate::FieldReader<u8, SV5_CFG_A>;
#[doc = "Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SV5_CFG_A {
    #[doc = "0: Security Violation 5 is disabled"]
    DISABLED = 0,
    #[doc = "1: Security Violation 5 is a non-fatal violation"]
    NON_FATAL = 1,
    #[doc = "2: Security Violation 5 is a fatal violation"]
    FATAL = 2,
}
impl From<SV5_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SV5_CFG_A) -> Self {
        variant as _
    }
}
impl SV5_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SV5_CFG_A> {
        match self.bits {
            0 => Some(SV5_CFG_A::DISABLED),
            1 => Some(SV5_CFG_A::NON_FATAL),
            2 => Some(SV5_CFG_A::FATAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SV5_CFG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == SV5_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == SV5_CFG_A::FATAL
    }
}
#[doc = "Field `SV5_CFG` writer - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
pub type SV5_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPSVCR_SPEC, u8, SV5_CFG_A, 2, O>;
impl<'a, const O: u8> SV5_CFG_W<'a, O> {
    #[doc = "Security Violation 5 is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SV5_CFG_A::DISABLED)
    }
    #[doc = "Security Violation 5 is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(SV5_CFG_A::NON_FATAL)
    }
    #[doc = "Security Violation 5 is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(SV5_CFG_A::FATAL)
    }
}
#[doc = "Field `LPSV_CFG` reader - LP Security Violation Configuration This field configures the LP security violation source."]
pub type LPSV_CFG_R = crate::FieldReader<u8, LPSV_CFG_A>;
#[doc = "LP Security Violation Configuration This field configures the LP security violation source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPSV_CFG_A {
    #[doc = "0: LP security violation is disabled"]
    DISABLED = 0,
    #[doc = "1: LP security violation is a non-fatal violation"]
    NON_FATAL = 1,
    #[doc = "2: LP security violation is a fatal violation"]
    FATAL = 2,
}
impl From<LPSV_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSV_CFG_A) -> Self {
        variant as _
    }
}
impl LPSV_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPSV_CFG_A> {
        match self.bits {
            0 => Some(LPSV_CFG_A::DISABLED),
            1 => Some(LPSV_CFG_A::NON_FATAL),
            2 => Some(LPSV_CFG_A::FATAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPSV_CFG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NON_FATAL`"]
    #[inline(always)]
    pub fn is_non_fatal(&self) -> bool {
        *self == LPSV_CFG_A::NON_FATAL
    }
    #[doc = "Checks if the value of the field is `FATAL`"]
    #[inline(always)]
    pub fn is_fatal(&self) -> bool {
        *self == LPSV_CFG_A::FATAL
    }
}
#[doc = "Field `LPSV_CFG` writer - LP Security Violation Configuration This field configures the LP security violation source."]
pub type LPSV_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPSVCR_SPEC, u8, LPSV_CFG_A, 2, O>;
impl<'a, const O: u8> LPSV_CFG_W<'a, O> {
    #[doc = "LP security violation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::DISABLED)
    }
    #[doc = "LP security violation is a non-fatal violation"]
    #[inline(always)]
    pub fn non_fatal(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::NON_FATAL)
    }
    #[doc = "LP security violation is a fatal violation"]
    #[inline(always)]
    pub fn fatal(self) -> &'a mut W {
        self.variant(LPSV_CFG_A::FATAL)
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    pub fn sv0_cfg(&self) -> SV0_CFG_R {
        SV0_CFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    pub fn sv1_cfg(&self) -> SV1_CFG_R {
        SV1_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    pub fn sv2_cfg(&self) -> SV2_CFG_R {
        SV2_CFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    pub fn sv3_cfg(&self) -> SV3_CFG_R {
        SV3_CFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    pub fn sv4_cfg(&self) -> SV4_CFG_R {
        SV4_CFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    pub fn sv5_cfg(&self) -> SV5_CFG_R {
        SV5_CFG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    pub fn lpsv_cfg(&self) -> LPSV_CFG_R {
        LPSV_CFG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 Security Violation Configuration This field configures the Security Violation 0 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv0_cfg(&mut self) -> SV0_CFG_W<0> {
        SV0_CFG_W::new(self)
    }
    #[doc = "Bit 1 - Security Violation 1 Security Violation Configuration This field configures the Security Violation 1 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv1_cfg(&mut self) -> SV1_CFG_W<1> {
        SV1_CFG_W::new(self)
    }
    #[doc = "Bit 2 - Security Violation 2 Security Violation Configuration This field configures the Security Violation 2 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv2_cfg(&mut self) -> SV2_CFG_W<2> {
        SV2_CFG_W::new(self)
    }
    #[doc = "Bit 3 - Security Violation 3 Security Violation Configuration This field configures the Security Violation 3 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv3_cfg(&mut self) -> SV3_CFG_W<3> {
        SV3_CFG_W::new(self)
    }
    #[doc = "Bit 4 - Security Violation 4 Security Violation Configuration This field configures the Security Violation 4 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv4_cfg(&mut self) -> SV4_CFG_W<4> {
        SV4_CFG_W::new(self)
    }
    #[doc = "Bits 5:6 - Security Violation 5 Security Violation Configuration This field configures the Security Violation 5 Security Violation Input"]
    #[inline(always)]
    #[must_use]
    pub fn sv5_cfg(&mut self) -> SV5_CFG_W<5> {
        SV5_CFG_W::new(self)
    }
    #[doc = "Bits 30:31 - LP Security Violation Configuration This field configures the LP security violation source."]
    #[inline(always)]
    #[must_use]
    pub fn lpsv_cfg(&mut self) -> LPSV_CFG_W<30> {
        LPSV_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Security Violation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsvcr](index.html) module"]
pub struct HPSVCR_SPEC;
impl crate::RegisterSpec for HPSVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpsvcr::R](R) reader structure"]
impl crate::Readable for HPSVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpsvcr::W](W) writer structure"]
impl crate::Writable for HPSVCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPSVCR to value 0"]
impl crate::Resettable for HPSVCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
