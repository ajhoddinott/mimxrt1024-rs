#[doc = "Register `CSL%s` reader"]
pub struct R(crate::R<CSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSL%s` writer"]
pub struct W(crate::W<CSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSL_SPEC>;
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
impl From<crate::W<CSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUR_S2` reader - Secure user read access control for the second slave"]
pub type SUR_S2_R = crate::BitReader<SUR_S2_A>;
#[doc = "Secure user read access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUR_S2_A {
    #[doc = "0: The secure user read access is disabled for the second slave."]
    SUR_S2_0 = 0,
    #[doc = "1: The secure user read access is enabled for the second slave."]
    SUR_S2_1 = 1,
}
impl From<SUR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SUR_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl SUR_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUR_S2_A {
        match self.bits {
            false => SUR_S2_A::SUR_S2_0,
            true => SUR_S2_A::SUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S2_0`"]
    #[inline(always)]
    pub fn is_sur_s2_0(&self) -> bool {
        *self == SUR_S2_A::SUR_S2_0
    }
    #[doc = "Checks if the value of the field is `SUR_S2_1`"]
    #[inline(always)]
    pub fn is_sur_s2_1(&self) -> bool {
        *self == SUR_S2_A::SUR_S2_1
    }
}
#[doc = "Field `SUR_S2` writer - Secure user read access control for the second slave"]
pub type SUR_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SUR_S2_A, O>;
impl<'a, const O: u8> SUR_S2_W<'a, O> {
    #[doc = "The secure user read access is disabled for the second slave."]
    #[inline(always)]
    pub fn sur_s2_0(self) -> &'a mut W {
        self.variant(SUR_S2_A::SUR_S2_0)
    }
    #[doc = "The secure user read access is enabled for the second slave."]
    #[inline(always)]
    pub fn sur_s2_1(self) -> &'a mut W {
        self.variant(SUR_S2_A::SUR_S2_1)
    }
}
#[doc = "Field `SSR_S2` reader - Secure supervisor read access control for the second slave"]
pub type SSR_S2_R = crate::BitReader<SSR_S2_A>;
#[doc = "Secure supervisor read access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSR_S2_A {
    #[doc = "0: The secure supervisor read access is disabled for the second slave."]
    SSR_S2_0 = 0,
    #[doc = "1: The secure supervisor read access is enabled for the second slave."]
    SSR_S2_1 = 1,
}
impl From<SSR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SSR_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl SSR_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSR_S2_A {
        match self.bits {
            false => SSR_S2_A::SSR_S2_0,
            true => SSR_S2_A::SSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S2_0`"]
    #[inline(always)]
    pub fn is_ssr_s2_0(&self) -> bool {
        *self == SSR_S2_A::SSR_S2_0
    }
    #[doc = "Checks if the value of the field is `SSR_S2_1`"]
    #[inline(always)]
    pub fn is_ssr_s2_1(&self) -> bool {
        *self == SSR_S2_A::SSR_S2_1
    }
}
#[doc = "Field `SSR_S2` writer - Secure supervisor read access control for the second slave"]
pub type SSR_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SSR_S2_A, O>;
impl<'a, const O: u8> SSR_S2_W<'a, O> {
    #[doc = "The secure supervisor read access is disabled for the second slave."]
    #[inline(always)]
    pub fn ssr_s2_0(self) -> &'a mut W {
        self.variant(SSR_S2_A::SSR_S2_0)
    }
    #[doc = "The secure supervisor read access is enabled for the second slave."]
    #[inline(always)]
    pub fn ssr_s2_1(self) -> &'a mut W {
        self.variant(SSR_S2_A::SSR_S2_1)
    }
}
#[doc = "Field `NUR_S2` reader - Non-secure user read access control for the second slave"]
pub type NUR_S2_R = crate::BitReader<NUR_S2_A>;
#[doc = "Non-secure user read access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUR_S2_A {
    #[doc = "0: The non-secure user read access is disabled for the second slave."]
    NUR_S2_0 = 0,
    #[doc = "1: The non-secure user read access is enabled for the second slave."]
    NUR_S2_1 = 1,
}
impl From<NUR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NUR_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl NUR_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUR_S2_A {
        match self.bits {
            false => NUR_S2_A::NUR_S2_0,
            true => NUR_S2_A::NUR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S2_0`"]
    #[inline(always)]
    pub fn is_nur_s2_0(&self) -> bool {
        *self == NUR_S2_A::NUR_S2_0
    }
    #[doc = "Checks if the value of the field is `NUR_S2_1`"]
    #[inline(always)]
    pub fn is_nur_s2_1(&self) -> bool {
        *self == NUR_S2_A::NUR_S2_1
    }
}
#[doc = "Field `NUR_S2` writer - Non-secure user read access control for the second slave"]
pub type NUR_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NUR_S2_A, O>;
impl<'a, const O: u8> NUR_S2_W<'a, O> {
    #[doc = "The non-secure user read access is disabled for the second slave."]
    #[inline(always)]
    pub fn nur_s2_0(self) -> &'a mut W {
        self.variant(NUR_S2_A::NUR_S2_0)
    }
    #[doc = "The non-secure user read access is enabled for the second slave."]
    #[inline(always)]
    pub fn nur_s2_1(self) -> &'a mut W {
        self.variant(NUR_S2_A::NUR_S2_1)
    }
}
#[doc = "Field `NSR_S2` reader - Non-secure supervisor read access control for the second slave"]
pub type NSR_S2_R = crate::BitReader<NSR_S2_A>;
#[doc = "Non-secure supervisor read access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSR_S2_A {
    #[doc = "0: The non-secure supervisor read access is disabled for the second slave."]
    NSR_S2_0 = 0,
    #[doc = "1: The non-secure supervisor read access is enabled for the second slave."]
    NSR_S2_1 = 1,
}
impl From<NSR_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NSR_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl NSR_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSR_S2_A {
        match self.bits {
            false => NSR_S2_A::NSR_S2_0,
            true => NSR_S2_A::NSR_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S2_0`"]
    #[inline(always)]
    pub fn is_nsr_s2_0(&self) -> bool {
        *self == NSR_S2_A::NSR_S2_0
    }
    #[doc = "Checks if the value of the field is `NSR_S2_1`"]
    #[inline(always)]
    pub fn is_nsr_s2_1(&self) -> bool {
        *self == NSR_S2_A::NSR_S2_1
    }
}
#[doc = "Field `NSR_S2` writer - Non-secure supervisor read access control for the second slave"]
pub type NSR_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NSR_S2_A, O>;
impl<'a, const O: u8> NSR_S2_W<'a, O> {
    #[doc = "The non-secure supervisor read access is disabled for the second slave."]
    #[inline(always)]
    pub fn nsr_s2_0(self) -> &'a mut W {
        self.variant(NSR_S2_A::NSR_S2_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the second slave."]
    #[inline(always)]
    pub fn nsr_s2_1(self) -> &'a mut W {
        self.variant(NSR_S2_A::NSR_S2_1)
    }
}
#[doc = "Field `SUW_S2` reader - Secure user write access control for the second slave"]
pub type SUW_S2_R = crate::BitReader<SUW_S2_A>;
#[doc = "Secure user write access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUW_S2_A {
    #[doc = "0: The secure user write access is disabled for the second slave."]
    SUW_S2_0 = 0,
    #[doc = "1: The secure user write access is enabled for the second slave."]
    SUW_S2_1 = 1,
}
impl From<SUW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SUW_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl SUW_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUW_S2_A {
        match self.bits {
            false => SUW_S2_A::SUW_S2_0,
            true => SUW_S2_A::SUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S2_0`"]
    #[inline(always)]
    pub fn is_suw_s2_0(&self) -> bool {
        *self == SUW_S2_A::SUW_S2_0
    }
    #[doc = "Checks if the value of the field is `SUW_S2_1`"]
    #[inline(always)]
    pub fn is_suw_s2_1(&self) -> bool {
        *self == SUW_S2_A::SUW_S2_1
    }
}
#[doc = "Field `SUW_S2` writer - Secure user write access control for the second slave"]
pub type SUW_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SUW_S2_A, O>;
impl<'a, const O: u8> SUW_S2_W<'a, O> {
    #[doc = "The secure user write access is disabled for the second slave."]
    #[inline(always)]
    pub fn suw_s2_0(self) -> &'a mut W {
        self.variant(SUW_S2_A::SUW_S2_0)
    }
    #[doc = "The secure user write access is enabled for the second slave."]
    #[inline(always)]
    pub fn suw_s2_1(self) -> &'a mut W {
        self.variant(SUW_S2_A::SUW_S2_1)
    }
}
#[doc = "Field `SSW_S2` reader - Secure supervisor write access control for the second slave"]
pub type SSW_S2_R = crate::BitReader<SSW_S2_A>;
#[doc = "Secure supervisor write access control for the second slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSW_S2_A {
    #[doc = "0: The secure supervisor write access is disabled for the second slave."]
    SSW_S2_0 = 0,
    #[doc = "1: The secure supervisor write access is enabled for the second slave."]
    SSW_S2_1 = 1,
}
impl From<SSW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl SSW_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_S2_A {
        match self.bits {
            false => SSW_S2_A::SSW_S2_0,
            true => SSW_S2_A::SSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S2_0`"]
    #[inline(always)]
    pub fn is_ssw_s2_0(&self) -> bool {
        *self == SSW_S2_A::SSW_S2_0
    }
    #[doc = "Checks if the value of the field is `SSW_S2_1`"]
    #[inline(always)]
    pub fn is_ssw_s2_1(&self) -> bool {
        *self == SSW_S2_A::SSW_S2_1
    }
}
#[doc = "Field `SSW_S2` writer - Secure supervisor write access control for the second slave"]
pub type SSW_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SSW_S2_A, O>;
impl<'a, const O: u8> SSW_S2_W<'a, O> {
    #[doc = "The secure supervisor write access is disabled for the second slave."]
    #[inline(always)]
    pub fn ssw_s2_0(self) -> &'a mut W {
        self.variant(SSW_S2_A::SSW_S2_0)
    }
    #[doc = "The secure supervisor write access is enabled for the second slave."]
    #[inline(always)]
    pub fn ssw_s2_1(self) -> &'a mut W {
        self.variant(SSW_S2_A::SSW_S2_1)
    }
}
#[doc = "Field `NUW_S2` reader - Non-secure user write access control for the second slave"]
pub type NUW_S2_R = crate::BitReader<NUW_S2_A>;
#[doc = "Non-secure user write access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUW_S2_A {
    #[doc = "0: The non-secure user write access is disabled for the second slave."]
    NUW_S2_0 = 0,
    #[doc = "1: The non-secure user write access is enabled for the second slave."]
    NUW_S2_1 = 1,
}
impl From<NUW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NUW_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl NUW_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUW_S2_A {
        match self.bits {
            false => NUW_S2_A::NUW_S2_0,
            true => NUW_S2_A::NUW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S2_0`"]
    #[inline(always)]
    pub fn is_nuw_s2_0(&self) -> bool {
        *self == NUW_S2_A::NUW_S2_0
    }
    #[doc = "Checks if the value of the field is `NUW_S2_1`"]
    #[inline(always)]
    pub fn is_nuw_s2_1(&self) -> bool {
        *self == NUW_S2_A::NUW_S2_1
    }
}
#[doc = "Field `NUW_S2` writer - Non-secure user write access control for the second slave"]
pub type NUW_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NUW_S2_A, O>;
impl<'a, const O: u8> NUW_S2_W<'a, O> {
    #[doc = "The non-secure user write access is disabled for the second slave."]
    #[inline(always)]
    pub fn nuw_s2_0(self) -> &'a mut W {
        self.variant(NUW_S2_A::NUW_S2_0)
    }
    #[doc = "The non-secure user write access is enabled for the second slave."]
    #[inline(always)]
    pub fn nuw_s2_1(self) -> &'a mut W {
        self.variant(NUW_S2_A::NUW_S2_1)
    }
}
#[doc = "Field `NSW_S2` reader - Non-secure supervisor write access control for the second slave"]
pub type NSW_S2_R = crate::BitReader<NSW_S2_A>;
#[doc = "Non-secure supervisor write access control for the second slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSW_S2_A {
    #[doc = "0: The non-secure supervisor write access is disabled for the second slave."]
    NSW_S2_0 = 0,
    #[doc = "1: The non-secure supervisor write access is enabled for the second slave."]
    NSW_S2_1 = 1,
}
impl From<NSW_S2_A> for bool {
    #[inline(always)]
    fn from(variant: NSW_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl NSW_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSW_S2_A {
        match self.bits {
            false => NSW_S2_A::NSW_S2_0,
            true => NSW_S2_A::NSW_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S2_0`"]
    #[inline(always)]
    pub fn is_nsw_s2_0(&self) -> bool {
        *self == NSW_S2_A::NSW_S2_0
    }
    #[doc = "Checks if the value of the field is `NSW_S2_1`"]
    #[inline(always)]
    pub fn is_nsw_s2_1(&self) -> bool {
        *self == NSW_S2_A::NSW_S2_1
    }
}
#[doc = "Field `NSW_S2` writer - Non-secure supervisor write access control for the second slave"]
pub type NSW_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NSW_S2_A, O>;
impl<'a, const O: u8> NSW_S2_W<'a, O> {
    #[doc = "The non-secure supervisor write access is disabled for the second slave."]
    #[inline(always)]
    pub fn nsw_s2_0(self) -> &'a mut W {
        self.variant(NSW_S2_A::NSW_S2_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the second slave."]
    #[inline(always)]
    pub fn nsw_s2_1(self) -> &'a mut W {
        self.variant(NSW_S2_A::NSW_S2_1)
    }
}
#[doc = "Field `LOCK_S2` reader - The lock bit corresponding to the second slave. It is written by the secure software."]
pub type LOCK_S2_R = crate::BitReader<LOCK_S2_A>;
#[doc = "The lock bit corresponding to the second slave. It is written by the secure software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_S2_A {
    #[doc = "0: Not locked. Bits 7-0 can be written by the software."]
    LOCK_S2_0 = 0,
    #[doc = "1: Bits 7-0 are locked and cannot be written by the software"]
    LOCK_S2_1 = 1,
}
impl From<LOCK_S2_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_S2_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_S2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_S2_A {
        match self.bits {
            false => LOCK_S2_A::LOCK_S2_0,
            true => LOCK_S2_A::LOCK_S2_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_0`"]
    #[inline(always)]
    pub fn is_lock_s2_0(&self) -> bool {
        *self == LOCK_S2_A::LOCK_S2_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S2_1`"]
    #[inline(always)]
    pub fn is_lock_s2_1(&self) -> bool {
        *self == LOCK_S2_A::LOCK_S2_1
    }
}
#[doc = "Field `LOCK_S2` writer - The lock bit corresponding to the second slave. It is written by the secure software."]
pub type LOCK_S2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, LOCK_S2_A, O>;
impl<'a, const O: u8> LOCK_S2_W<'a, O> {
    #[doc = "Not locked. Bits 7-0 can be written by the software."]
    #[inline(always)]
    pub fn lock_s2_0(self) -> &'a mut W {
        self.variant(LOCK_S2_A::LOCK_S2_0)
    }
    #[doc = "Bits 7-0 are locked and cannot be written by the software"]
    #[inline(always)]
    pub fn lock_s2_1(self) -> &'a mut W {
        self.variant(LOCK_S2_A::LOCK_S2_1)
    }
}
#[doc = "Field `SUR_S1` reader - Secure user read access control for the first slave"]
pub type SUR_S1_R = crate::BitReader<SUR_S1_A>;
#[doc = "Secure user read access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUR_S1_A {
    #[doc = "0: The secure user read access is disabled for the first slave."]
    SUR_S1_0 = 0,
    #[doc = "1: The secure user read access is enabled for the first slave."]
    SUR_S1_1 = 1,
}
impl From<SUR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SUR_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl SUR_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUR_S1_A {
        match self.bits {
            false => SUR_S1_A::SUR_S1_0,
            true => SUR_S1_A::SUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUR_S1_0`"]
    #[inline(always)]
    pub fn is_sur_s1_0(&self) -> bool {
        *self == SUR_S1_A::SUR_S1_0
    }
    #[doc = "Checks if the value of the field is `SUR_S1_1`"]
    #[inline(always)]
    pub fn is_sur_s1_1(&self) -> bool {
        *self == SUR_S1_A::SUR_S1_1
    }
}
#[doc = "Field `SUR_S1` writer - Secure user read access control for the first slave"]
pub type SUR_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SUR_S1_A, O>;
impl<'a, const O: u8> SUR_S1_W<'a, O> {
    #[doc = "The secure user read access is disabled for the first slave."]
    #[inline(always)]
    pub fn sur_s1_0(self) -> &'a mut W {
        self.variant(SUR_S1_A::SUR_S1_0)
    }
    #[doc = "The secure user read access is enabled for the first slave."]
    #[inline(always)]
    pub fn sur_s1_1(self) -> &'a mut W {
        self.variant(SUR_S1_A::SUR_S1_1)
    }
}
#[doc = "Field `SSR_S1` reader - Secure supervisor read access control for the first slave"]
pub type SSR_S1_R = crate::BitReader<SSR_S1_A>;
#[doc = "Secure supervisor read access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSR_S1_A {
    #[doc = "0: The secure supervisor read access is disabled for the first slave."]
    SSR_S1_0 = 0,
    #[doc = "1: The secure supervisor read access is enabled for the first slave."]
    SSR_S1_1 = 1,
}
impl From<SSR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SSR_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl SSR_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSR_S1_A {
        match self.bits {
            false => SSR_S1_A::SSR_S1_0,
            true => SSR_S1_A::SSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSR_S1_0`"]
    #[inline(always)]
    pub fn is_ssr_s1_0(&self) -> bool {
        *self == SSR_S1_A::SSR_S1_0
    }
    #[doc = "Checks if the value of the field is `SSR_S1_1`"]
    #[inline(always)]
    pub fn is_ssr_s1_1(&self) -> bool {
        *self == SSR_S1_A::SSR_S1_1
    }
}
#[doc = "Field `SSR_S1` writer - Secure supervisor read access control for the first slave"]
pub type SSR_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SSR_S1_A, O>;
impl<'a, const O: u8> SSR_S1_W<'a, O> {
    #[doc = "The secure supervisor read access is disabled for the first slave."]
    #[inline(always)]
    pub fn ssr_s1_0(self) -> &'a mut W {
        self.variant(SSR_S1_A::SSR_S1_0)
    }
    #[doc = "The secure supervisor read access is enabled for the first slave."]
    #[inline(always)]
    pub fn ssr_s1_1(self) -> &'a mut W {
        self.variant(SSR_S1_A::SSR_S1_1)
    }
}
#[doc = "Field `NUR_S1` reader - Non-secure user read access control for the first slave"]
pub type NUR_S1_R = crate::BitReader<NUR_S1_A>;
#[doc = "Non-secure user read access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUR_S1_A {
    #[doc = "0: The non-secure user read access is disabled for the first slave."]
    NUR_S1_0 = 0,
    #[doc = "1: The non-secure user read access is enabled for the first slave."]
    NUR_S1_1 = 1,
}
impl From<NUR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NUR_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl NUR_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUR_S1_A {
        match self.bits {
            false => NUR_S1_A::NUR_S1_0,
            true => NUR_S1_A::NUR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUR_S1_0`"]
    #[inline(always)]
    pub fn is_nur_s1_0(&self) -> bool {
        *self == NUR_S1_A::NUR_S1_0
    }
    #[doc = "Checks if the value of the field is `NUR_S1_1`"]
    #[inline(always)]
    pub fn is_nur_s1_1(&self) -> bool {
        *self == NUR_S1_A::NUR_S1_1
    }
}
#[doc = "Field `NUR_S1` writer - Non-secure user read access control for the first slave"]
pub type NUR_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NUR_S1_A, O>;
impl<'a, const O: u8> NUR_S1_W<'a, O> {
    #[doc = "The non-secure user read access is disabled for the first slave."]
    #[inline(always)]
    pub fn nur_s1_0(self) -> &'a mut W {
        self.variant(NUR_S1_A::NUR_S1_0)
    }
    #[doc = "The non-secure user read access is enabled for the first slave."]
    #[inline(always)]
    pub fn nur_s1_1(self) -> &'a mut W {
        self.variant(NUR_S1_A::NUR_S1_1)
    }
}
#[doc = "Field `NSR_S1` reader - Non-secure supervisor read access control for the first slave"]
pub type NSR_S1_R = crate::BitReader<NSR_S1_A>;
#[doc = "Non-secure supervisor read access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSR_S1_A {
    #[doc = "0: The non-secure supervisor read access is disabled for the first slave."]
    NSR_S1_0 = 0,
    #[doc = "1: The non-secure supervisor read access is enabled for the first slave."]
    NSR_S1_1 = 1,
}
impl From<NSR_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NSR_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl NSR_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSR_S1_A {
        match self.bits {
            false => NSR_S1_A::NSR_S1_0,
            true => NSR_S1_A::NSR_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSR_S1_0`"]
    #[inline(always)]
    pub fn is_nsr_s1_0(&self) -> bool {
        *self == NSR_S1_A::NSR_S1_0
    }
    #[doc = "Checks if the value of the field is `NSR_S1_1`"]
    #[inline(always)]
    pub fn is_nsr_s1_1(&self) -> bool {
        *self == NSR_S1_A::NSR_S1_1
    }
}
#[doc = "Field `NSR_S1` writer - Non-secure supervisor read access control for the first slave"]
pub type NSR_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NSR_S1_A, O>;
impl<'a, const O: u8> NSR_S1_W<'a, O> {
    #[doc = "The non-secure supervisor read access is disabled for the first slave."]
    #[inline(always)]
    pub fn nsr_s1_0(self) -> &'a mut W {
        self.variant(NSR_S1_A::NSR_S1_0)
    }
    #[doc = "The non-secure supervisor read access is enabled for the first slave."]
    #[inline(always)]
    pub fn nsr_s1_1(self) -> &'a mut W {
        self.variant(NSR_S1_A::NSR_S1_1)
    }
}
#[doc = "Field `SUW_S1` reader - Secure user write access control for the first slave"]
pub type SUW_S1_R = crate::BitReader<SUW_S1_A>;
#[doc = "Secure user write access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUW_S1_A {
    #[doc = "0: The secure user write access is disabled for the first slave."]
    SUW_S1_0 = 0,
    #[doc = "1: The secure user write access is enabled for the first slave."]
    SUW_S1_1 = 1,
}
impl From<SUW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SUW_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl SUW_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUW_S1_A {
        match self.bits {
            false => SUW_S1_A::SUW_S1_0,
            true => SUW_S1_A::SUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUW_S1_0`"]
    #[inline(always)]
    pub fn is_suw_s1_0(&self) -> bool {
        *self == SUW_S1_A::SUW_S1_0
    }
    #[doc = "Checks if the value of the field is `SUW_S1_1`"]
    #[inline(always)]
    pub fn is_suw_s1_1(&self) -> bool {
        *self == SUW_S1_A::SUW_S1_1
    }
}
#[doc = "Field `SUW_S1` writer - Secure user write access control for the first slave"]
pub type SUW_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SUW_S1_A, O>;
impl<'a, const O: u8> SUW_S1_W<'a, O> {
    #[doc = "The secure user write access is disabled for the first slave."]
    #[inline(always)]
    pub fn suw_s1_0(self) -> &'a mut W {
        self.variant(SUW_S1_A::SUW_S1_0)
    }
    #[doc = "The secure user write access is enabled for the first slave."]
    #[inline(always)]
    pub fn suw_s1_1(self) -> &'a mut W {
        self.variant(SUW_S1_A::SUW_S1_1)
    }
}
#[doc = "Field `SSW_S1` reader - Secure supervisor write access control for the first slave"]
pub type SSW_S1_R = crate::BitReader<SSW_S1_A>;
#[doc = "Secure supervisor write access control for the first slave\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSW_S1_A {
    #[doc = "0: The secure supervisor write access is disabled for the first slave."]
    SSW_S1_0 = 0,
    #[doc = "1: The secure supervisor write access is enabled for the first slave."]
    SSW_S1_1 = 1,
}
impl From<SSW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl SSW_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_S1_A {
        match self.bits {
            false => SSW_S1_A::SSW_S1_0,
            true => SSW_S1_A::SSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_S1_0`"]
    #[inline(always)]
    pub fn is_ssw_s1_0(&self) -> bool {
        *self == SSW_S1_A::SSW_S1_0
    }
    #[doc = "Checks if the value of the field is `SSW_S1_1`"]
    #[inline(always)]
    pub fn is_ssw_s1_1(&self) -> bool {
        *self == SSW_S1_A::SSW_S1_1
    }
}
#[doc = "Field `SSW_S1` writer - Secure supervisor write access control for the first slave"]
pub type SSW_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, SSW_S1_A, O>;
impl<'a, const O: u8> SSW_S1_W<'a, O> {
    #[doc = "The secure supervisor write access is disabled for the first slave."]
    #[inline(always)]
    pub fn ssw_s1_0(self) -> &'a mut W {
        self.variant(SSW_S1_A::SSW_S1_0)
    }
    #[doc = "The secure supervisor write access is enabled for the first slave."]
    #[inline(always)]
    pub fn ssw_s1_1(self) -> &'a mut W {
        self.variant(SSW_S1_A::SSW_S1_1)
    }
}
#[doc = "Field `NUW_S1` reader - Non-secure user write access control for the first slave"]
pub type NUW_S1_R = crate::BitReader<NUW_S1_A>;
#[doc = "Non-secure user write access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUW_S1_A {
    #[doc = "0: The non-secure user write access is disabled for the first slave."]
    NUW_S1_0 = 0,
    #[doc = "1: The non-secure user write access is enabled for the first slave."]
    NUW_S1_1 = 1,
}
impl From<NUW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NUW_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl NUW_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUW_S1_A {
        match self.bits {
            false => NUW_S1_A::NUW_S1_0,
            true => NUW_S1_A::NUW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUW_S1_0`"]
    #[inline(always)]
    pub fn is_nuw_s1_0(&self) -> bool {
        *self == NUW_S1_A::NUW_S1_0
    }
    #[doc = "Checks if the value of the field is `NUW_S1_1`"]
    #[inline(always)]
    pub fn is_nuw_s1_1(&self) -> bool {
        *self == NUW_S1_A::NUW_S1_1
    }
}
#[doc = "Field `NUW_S1` writer - Non-secure user write access control for the first slave"]
pub type NUW_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NUW_S1_A, O>;
impl<'a, const O: u8> NUW_S1_W<'a, O> {
    #[doc = "The non-secure user write access is disabled for the first slave."]
    #[inline(always)]
    pub fn nuw_s1_0(self) -> &'a mut W {
        self.variant(NUW_S1_A::NUW_S1_0)
    }
    #[doc = "The non-secure user write access is enabled for the first slave."]
    #[inline(always)]
    pub fn nuw_s1_1(self) -> &'a mut W {
        self.variant(NUW_S1_A::NUW_S1_1)
    }
}
#[doc = "Field `NSW_S1` reader - Non-secure supervisor write access control for the first slave"]
pub type NSW_S1_R = crate::BitReader<NSW_S1_A>;
#[doc = "Non-secure supervisor write access control for the first slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSW_S1_A {
    #[doc = "0: The non-secure supervisor write access is disabled for the first slave."]
    NSW_S1_0 = 0,
    #[doc = "1: The non-secure supervisor write access is enabled for the first slave"]
    NSW_S1_1 = 1,
}
impl From<NSW_S1_A> for bool {
    #[inline(always)]
    fn from(variant: NSW_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl NSW_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSW_S1_A {
        match self.bits {
            false => NSW_S1_A::NSW_S1_0,
            true => NSW_S1_A::NSW_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSW_S1_0`"]
    #[inline(always)]
    pub fn is_nsw_s1_0(&self) -> bool {
        *self == NSW_S1_A::NSW_S1_0
    }
    #[doc = "Checks if the value of the field is `NSW_S1_1`"]
    #[inline(always)]
    pub fn is_nsw_s1_1(&self) -> bool {
        *self == NSW_S1_A::NSW_S1_1
    }
}
#[doc = "Field `NSW_S1` writer - Non-secure supervisor write access control for the first slave"]
pub type NSW_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, NSW_S1_A, O>;
impl<'a, const O: u8> NSW_S1_W<'a, O> {
    #[doc = "The non-secure supervisor write access is disabled for the first slave."]
    #[inline(always)]
    pub fn nsw_s1_0(self) -> &'a mut W {
        self.variant(NSW_S1_A::NSW_S1_0)
    }
    #[doc = "The non-secure supervisor write access is enabled for the first slave"]
    #[inline(always)]
    pub fn nsw_s1_1(self) -> &'a mut W {
        self.variant(NSW_S1_A::NSW_S1_1)
    }
}
#[doc = "Field `LOCK_S1` reader - The lock bit corresponding to the first slave. It is written by the secure software."]
pub type LOCK_S1_R = crate::BitReader<LOCK_S1_A>;
#[doc = "The lock bit corresponding to the first slave. It is written by the secure software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_S1_A {
    #[doc = "0: Not locked. The bits 16-23 can be written by the software."]
    LOCK_S1_0 = 0,
    #[doc = "1: The bits 16-23 are locked and can't be written by the software."]
    LOCK_S1_1 = 1,
}
impl From<LOCK_S1_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_S1_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_S1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_S1_A {
        match self.bits {
            false => LOCK_S1_A::LOCK_S1_0,
            true => LOCK_S1_A::LOCK_S1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_0`"]
    #[inline(always)]
    pub fn is_lock_s1_0(&self) -> bool {
        *self == LOCK_S1_A::LOCK_S1_0
    }
    #[doc = "Checks if the value of the field is `LOCK_S1_1`"]
    #[inline(always)]
    pub fn is_lock_s1_1(&self) -> bool {
        *self == LOCK_S1_A::LOCK_S1_1
    }
}
#[doc = "Field `LOCK_S1` writer - The lock bit corresponding to the first slave. It is written by the secure software."]
pub type LOCK_S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSL_SPEC, LOCK_S1_A, O>;
impl<'a, const O: u8> LOCK_S1_W<'a, O> {
    #[doc = "Not locked. The bits 16-23 can be written by the software."]
    #[inline(always)]
    pub fn lock_s1_0(self) -> &'a mut W {
        self.variant(LOCK_S1_A::LOCK_S1_0)
    }
    #[doc = "The bits 16-23 are locked and can't be written by the software."]
    #[inline(always)]
    pub fn lock_s1_1(self) -> &'a mut W {
        self.variant(LOCK_S1_A::LOCK_S1_1)
    }
}
impl R {
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline(always)]
    pub fn sur_s2(&self) -> SUR_S2_R {
        SUR_S2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn ssr_s2(&self) -> SSR_S2_R {
        SSR_S2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline(always)]
    pub fn nur_s2(&self) -> NUR_S2_R {
        NUR_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    pub fn nsr_s2(&self) -> NSR_S2_R {
        NSR_S2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline(always)]
    pub fn suw_s2(&self) -> SUW_S2_R {
        SUW_S2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn ssw_s2(&self) -> SSW_S2_R {
        SSW_S2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline(always)]
    pub fn nuw_s2(&self) -> NUW_S2_R {
        NUW_S2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    pub fn nsw_s2(&self) -> NSW_S2_R {
        NSW_S2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s2(&self) -> LOCK_S2_R {
        LOCK_S2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline(always)]
    pub fn sur_s1(&self) -> SUR_S1_R {
        SUR_S1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn ssr_s1(&self) -> SSR_S1_R {
        SSR_S1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline(always)]
    pub fn nur_s1(&self) -> NUR_S1_R {
        NUR_S1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    pub fn nsr_s1(&self) -> NSR_S1_R {
        NSR_S1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline(always)]
    pub fn suw_s1(&self) -> SUW_S1_R {
        SUW_S1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn ssw_s1(&self) -> SSW_S1_R {
        SSW_S1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline(always)]
    pub fn nuw_s1(&self) -> NUW_S1_R {
        NUW_S1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    pub fn nsw_s1(&self) -> NSW_S1_R {
        NSW_S1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    pub fn lock_s1(&self) -> LOCK_S1_R {
        LOCK_S1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure user read access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn sur_s2(&mut self) -> SUR_S2_W<0> {
        SUR_S2_W::new(self)
    }
    #[doc = "Bit 1 - Secure supervisor read access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn ssr_s2(&mut self) -> SSR_S2_W<1> {
        SSR_S2_W::new(self)
    }
    #[doc = "Bit 2 - Non-secure user read access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn nur_s2(&mut self) -> NUR_S2_W<2> {
        NUR_S2_W::new(self)
    }
    #[doc = "Bit 3 - Non-secure supervisor read access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn nsr_s2(&mut self) -> NSR_S2_W<3> {
        NSR_S2_W::new(self)
    }
    #[doc = "Bit 4 - Secure user write access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn suw_s2(&mut self) -> SUW_S2_W<4> {
        SUW_S2_W::new(self)
    }
    #[doc = "Bit 5 - Secure supervisor write access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn ssw_s2(&mut self) -> SSW_S2_W<5> {
        SSW_S2_W::new(self)
    }
    #[doc = "Bit 6 - Non-secure user write access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn nuw_s2(&mut self) -> NUW_S2_W<6> {
        NUW_S2_W::new(self)
    }
    #[doc = "Bit 7 - Non-secure supervisor write access control for the second slave"]
    #[inline(always)]
    #[must_use]
    pub fn nsw_s2(&mut self) -> NSW_S2_W<7> {
        NSW_S2_W::new(self)
    }
    #[doc = "Bit 8 - The lock bit corresponding to the second slave. It is written by the secure software."]
    #[inline(always)]
    #[must_use]
    pub fn lock_s2(&mut self) -> LOCK_S2_W<8> {
        LOCK_S2_W::new(self)
    }
    #[doc = "Bit 16 - Secure user read access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn sur_s1(&mut self) -> SUR_S1_W<16> {
        SUR_S1_W::new(self)
    }
    #[doc = "Bit 17 - Secure supervisor read access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn ssr_s1(&mut self) -> SSR_S1_W<17> {
        SSR_S1_W::new(self)
    }
    #[doc = "Bit 18 - Non-secure user read access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn nur_s1(&mut self) -> NUR_S1_W<18> {
        NUR_S1_W::new(self)
    }
    #[doc = "Bit 19 - Non-secure supervisor read access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn nsr_s1(&mut self) -> NSR_S1_W<19> {
        NSR_S1_W::new(self)
    }
    #[doc = "Bit 20 - Secure user write access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn suw_s1(&mut self) -> SUW_S1_W<20> {
        SUW_S1_W::new(self)
    }
    #[doc = "Bit 21 - Secure supervisor write access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn ssw_s1(&mut self) -> SSW_S1_W<21> {
        SSW_S1_W::new(self)
    }
    #[doc = "Bit 22 - Non-secure user write access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn nuw_s1(&mut self) -> NUW_S1_W<22> {
        NUW_S1_W::new(self)
    }
    #[doc = "Bit 23 - Non-secure supervisor write access control for the first slave"]
    #[inline(always)]
    #[must_use]
    pub fn nsw_s1(&mut self) -> NSW_S1_W<23> {
        NSW_S1_W::new(self)
    }
    #[doc = "Bit 24 - The lock bit corresponding to the first slave. It is written by the secure software."]
    #[inline(always)]
    #[must_use]
    pub fn lock_s1(&mut self) -> LOCK_S1_W<24> {
        LOCK_S1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Config security level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csl](index.html) module"]
pub struct CSL_SPEC;
impl crate::RegisterSpec for CSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csl::R](R) reader structure"]
impl crate::Readable for CSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csl::W](W) writer structure"]
impl crate::Writable for CSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSL%s to value 0x0033_0033"]
impl crate::Resettable for CSL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0033_0033;
}
