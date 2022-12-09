#[doc = "Register `HPSVSR` reader"]
pub struct R(crate::R<HPSVSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPSVSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPSVSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPSVSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPSVSR` writer"]
pub struct W(crate::W<HPSVSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPSVSR_SPEC>;
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
impl From<crate::W<HPSVSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPSVSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SV0` reader - Security Violation 0 security violation was detected."]
pub type SV0_R = crate::BitReader<SV0_A>;
#[doc = "Security Violation 0 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV0_A {
    #[doc = "0: No Security Violation 0 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 0 security violation was detected."]
    REPORTED = 1,
}
impl From<SV0_A> for bool {
    #[inline(always)]
    fn from(variant: SV0_A) -> Self {
        variant as u8 != 0
    }
}
impl SV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV0_A {
        match self.bits {
            false => SV0_A::NOREPORT,
            true => SV0_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV0_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV0_A::REPORTED
    }
}
#[doc = "Field `SV0` writer - Security Violation 0 security violation was detected."]
pub type SV0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV0_A, O>;
impl<'a, const O: u8> SV0_W<'a, O> {
    #[doc = "No Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV0_A::NOREPORT)
    }
    #[doc = "Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV0_A::REPORTED)
    }
}
#[doc = "Field `SV1` reader - Security Violation 1 security violation was detected."]
pub type SV1_R = crate::BitReader<SV1_A>;
#[doc = "Security Violation 1 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV1_A {
    #[doc = "0: No Security Violation 1 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 1 security violation was detected."]
    REPORTED = 1,
}
impl From<SV1_A> for bool {
    #[inline(always)]
    fn from(variant: SV1_A) -> Self {
        variant as u8 != 0
    }
}
impl SV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV1_A {
        match self.bits {
            false => SV1_A::NOREPORT,
            true => SV1_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV1_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV1_A::REPORTED
    }
}
#[doc = "Field `SV1` writer - Security Violation 1 security violation was detected."]
pub type SV1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV1_A, O>;
impl<'a, const O: u8> SV1_W<'a, O> {
    #[doc = "No Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV1_A::NOREPORT)
    }
    #[doc = "Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV1_A::REPORTED)
    }
}
#[doc = "Field `SV2` reader - Security Violation 2 security violation was detected."]
pub type SV2_R = crate::BitReader<SV2_A>;
#[doc = "Security Violation 2 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV2_A {
    #[doc = "0: No Security Violation 2 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 2 security violation was detected."]
    REPORTED = 1,
}
impl From<SV2_A> for bool {
    #[inline(always)]
    fn from(variant: SV2_A) -> Self {
        variant as u8 != 0
    }
}
impl SV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV2_A {
        match self.bits {
            false => SV2_A::NOREPORT,
            true => SV2_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV2_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV2_A::REPORTED
    }
}
#[doc = "Field `SV2` writer - Security Violation 2 security violation was detected."]
pub type SV2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV2_A, O>;
impl<'a, const O: u8> SV2_W<'a, O> {
    #[doc = "No Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV2_A::NOREPORT)
    }
    #[doc = "Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV2_A::REPORTED)
    }
}
#[doc = "Field `SV3` reader - Security Violation 3 security violation was detected."]
pub type SV3_R = crate::BitReader<SV3_A>;
#[doc = "Security Violation 3 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV3_A {
    #[doc = "0: No Security Violation 3 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 3 security violation was detected."]
    REPORTED = 1,
}
impl From<SV3_A> for bool {
    #[inline(always)]
    fn from(variant: SV3_A) -> Self {
        variant as u8 != 0
    }
}
impl SV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV3_A {
        match self.bits {
            false => SV3_A::NOREPORT,
            true => SV3_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV3_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV3_A::REPORTED
    }
}
#[doc = "Field `SV3` writer - Security Violation 3 security violation was detected."]
pub type SV3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV3_A, O>;
impl<'a, const O: u8> SV3_W<'a, O> {
    #[doc = "No Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV3_A::NOREPORT)
    }
    #[doc = "Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV3_A::REPORTED)
    }
}
#[doc = "Field `SV4` reader - Security Violation 4 security violation was detected."]
pub type SV4_R = crate::BitReader<SV4_A>;
#[doc = "Security Violation 4 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV4_A {
    #[doc = "0: No Security Violation 4 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 4 security violation was detected."]
    REPORTED = 1,
}
impl From<SV4_A> for bool {
    #[inline(always)]
    fn from(variant: SV4_A) -> Self {
        variant as u8 != 0
    }
}
impl SV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV4_A {
        match self.bits {
            false => SV4_A::NOREPORT,
            true => SV4_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV4_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV4_A::REPORTED
    }
}
#[doc = "Field `SV4` writer - Security Violation 4 security violation was detected."]
pub type SV4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV4_A, O>;
impl<'a, const O: u8> SV4_W<'a, O> {
    #[doc = "No Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV4_A::NOREPORT)
    }
    #[doc = "Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV4_A::REPORTED)
    }
}
#[doc = "Field `SV5` reader - Security Violation 5 security violation was detected."]
pub type SV5_R = crate::BitReader<SV5_A>;
#[doc = "Security Violation 5 security violation was detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SV5_A {
    #[doc = "0: No Security Violation 5 security violation was detected."]
    NOREPORT = 0,
    #[doc = "1: Security Violation 5 security violation was detected."]
    REPORTED = 1,
}
impl From<SV5_A> for bool {
    #[inline(always)]
    fn from(variant: SV5_A) -> Self {
        variant as u8 != 0
    }
}
impl SV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SV5_A {
        match self.bits {
            false => SV5_A::NOREPORT,
            true => SV5_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == SV5_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == SV5_A::REPORTED
    }
}
#[doc = "Field `SV5` writer - Security Violation 5 security violation was detected."]
pub type SV5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSVSR_SPEC, SV5_A, O>;
impl<'a, const O: u8> SV5_W<'a, O> {
    #[doc = "No Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(SV5_A::NOREPORT)
    }
    #[doc = "Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(SV5_A::REPORTED)
    }
}
#[doc = "Field `SW_SV` reader - Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
pub type SW_SV_R = crate::BitReader<bool>;
#[doc = "Field `SW_FSV` reader - Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
pub type SW_FSV_R = crate::BitReader<bool>;
#[doc = "Field `SW_LPSV` reader - LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
pub type SW_LPSV_R = crate::BitReader<bool>;
#[doc = "Field `ZMK_SYNDROME` reader - Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
pub type ZMK_SYNDROME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ZMK_ECC_FAIL` reader - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
pub type ZMK_ECC_FAIL_R = crate::BitReader<ZMK_ECC_FAIL_A>;
#[doc = "Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_ECC_FAIL_A {
    #[doc = "0: ZMK ECC Failure was not detected."]
    NOREPORT = 0,
    #[doc = "1: ZMK ECC Failure was detected."]
    REPORTED = 1,
}
impl From<ZMK_ECC_FAIL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ECC_FAIL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_ECC_FAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ECC_FAIL_A {
        match self.bits {
            false => ZMK_ECC_FAIL_A::NOREPORT,
            true => ZMK_ECC_FAIL_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == ZMK_ECC_FAIL_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == ZMK_ECC_FAIL_A::REPORTED
    }
}
#[doc = "Field `ZMK_ECC_FAIL` writer - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
pub type ZMK_ECC_FAIL_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HPSVSR_SPEC, ZMK_ECC_FAIL_A, O>;
impl<'a, const O: u8> ZMK_ECC_FAIL_W<'a, O> {
    #[doc = "ZMK ECC Failure was not detected."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAIL_A::NOREPORT)
    }
    #[doc = "ZMK ECC Failure was detected."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(ZMK_ECC_FAIL_A::REPORTED)
    }
}
#[doc = "Field `LP_SEC_VIO` reader - LP Security Violation A security volation was detected in the SNVS low power section"]
pub type LP_SEC_VIO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline(always)]
    pub fn sv0(&self) -> SV0_R {
        SV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline(always)]
    pub fn sv1(&self) -> SV1_R {
        SV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline(always)]
    pub fn sv2(&self) -> SV2_R {
        SV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline(always)]
    pub fn sv3(&self) -> SV3_R {
        SV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline(always)]
    pub fn sv4(&self) -> SV4_R {
        SV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline(always)]
    pub fn sv5(&self) -> SV5_R {
        SV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Security Violation This bit is a read-only copy of the SW_SV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_sv(&self) -> SW_SV_R {
        SW_SV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Fatal Security Violation This bit is a read-only copy of the SW_FSV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_fsv(&self) -> SW_FSV_R {
        SW_FSV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LP Software Security Violation This bit is a read-only copy of the SW_LPSV bit in the HP Command Register"]
    #[inline(always)]
    pub fn sw_lpsv(&self) -> SW_LPSV_R {
        SW_LPSV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Zeroizable Master Key Syndrome The ZMK syndrome indicates the single-bit error location and parity for the ZMK register"]
    #[inline(always)]
    pub fn zmk_syndrome(&self) -> ZMK_SYNDROME_R {
        ZMK_SYNDROME_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    pub fn zmk_ecc_fail(&self) -> ZMK_ECC_FAIL_R {
        ZMK_ECC_FAIL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - LP Security Violation A security volation was detected in the SNVS low power section"]
    #[inline(always)]
    pub fn lp_sec_vio(&self) -> LP_SEC_VIO_R {
        LP_SEC_VIO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security Violation 0 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv0(&mut self) -> SV0_W<0> {
        SV0_W::new(self)
    }
    #[doc = "Bit 1 - Security Violation 1 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv1(&mut self) -> SV1_W<1> {
        SV1_W::new(self)
    }
    #[doc = "Bit 2 - Security Violation 2 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv2(&mut self) -> SV2_W<2> {
        SV2_W::new(self)
    }
    #[doc = "Bit 3 - Security Violation 3 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv3(&mut self) -> SV3_W<3> {
        SV3_W::new(self)
    }
    #[doc = "Bit 4 - Security Violation 4 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv4(&mut self) -> SV4_W<4> {
        SV4_W::new(self)
    }
    #[doc = "Bit 5 - Security Violation 5 security violation was detected."]
    #[inline(always)]
    #[must_use]
    pub fn sv5(&mut self) -> SV5_W<5> {
        SV5_W::new(self)
    }
    #[doc = "Bit 27 - Zeroizable Master Key Error Correcting Code Check Failure When set, this bit triggers a bad key violation to the SSM and a security violation to the SNVS_LP section, which clears security sensitive data"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_ecc_fail(&mut self) -> ZMK_ECC_FAIL_W<27> {
        ZMK_ECC_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Security Violation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsvsr](index.html) module"]
pub struct HPSVSR_SPEC;
impl crate::RegisterSpec for HPSVSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpsvsr::R](R) reader structure"]
impl crate::Readable for HPSVSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpsvsr::W](W) writer structure"]
impl crate::Writable for HPSVSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0800_003f;
}
#[doc = "`reset()` method sets HPSVSR to value 0x8000_0000"]
impl crate::Resettable for HPSVSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
