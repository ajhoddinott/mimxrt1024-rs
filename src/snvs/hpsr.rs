#[doc = "Register `HPSR` reader"]
pub struct R(crate::R<HPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPSR` writer"]
pub struct W(crate::W<HPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPSR_SPEC>;
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
impl From<crate::W<HPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPTA` reader - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
pub type HPTA_R = crate::BitReader<HPTA_A>;
#[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPTA_A {
    #[doc = "0: No time alarm interrupt occurred."]
    NOREPORT = 0,
    #[doc = "1: A time alarm interrupt occurred."]
    REPORTED = 1,
}
impl From<HPTA_A> for bool {
    #[inline(always)]
    fn from(variant: HPTA_A) -> Self {
        variant as u8 != 0
    }
}
impl HPTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPTA_A {
        match self.bits {
            false => HPTA_A::NOREPORT,
            true => HPTA_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == HPTA_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == HPTA_A::REPORTED
    }
}
#[doc = "Field `HPTA` writer - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
pub type HPTA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSR_SPEC, HPTA_A, O>;
impl<'a, const O: u8> HPTA_W<'a, O> {
    #[doc = "No time alarm interrupt occurred."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(HPTA_A::NOREPORT)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(HPTA_A::REPORTED)
    }
}
#[doc = "Field `PI` reader - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
pub type PI_R = crate::BitReader<PI_A>;
#[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PI_A {
    #[doc = "0: No periodic interrupt occurred."]
    NOREPORT = 0,
    #[doc = "1: A periodic interrupt occurred."]
    REPORTED = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
impl PI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::NOREPORT,
            true => PI_A::REPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOREPORT`"]
    #[inline(always)]
    pub fn is_noreport(&self) -> bool {
        *self == PI_A::NOREPORT
    }
    #[doc = "Checks if the value of the field is `REPORTED`"]
    #[inline(always)]
    pub fn is_reported(&self) -> bool {
        *self == PI_A::REPORTED
    }
}
#[doc = "Field `PI` writer - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
pub type PI_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSR_SPEC, PI_A, O>;
impl<'a, const O: u8> PI_W<'a, O> {
    #[doc = "No periodic interrupt occurred."]
    #[inline(always)]
    pub fn noreport(self) -> &'a mut W {
        self.variant(PI_A::NOREPORT)
    }
    #[doc = "A periodic interrupt occurred."]
    #[inline(always)]
    pub fn reported(self) -> &'a mut W {
        self.variant(PI_A::REPORTED)
    }
}
#[doc = "Field `LPDIS` reader - Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
pub type LPDIS_R = crate::BitReader<bool>;
#[doc = "Field `BTN` reader - Button Value of the BTN input"]
pub type BTN_R = crate::BitReader<bool>;
#[doc = "Field `BI` reader - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
pub type BI_R = crate::BitReader<bool>;
#[doc = "Field `BI` writer - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
pub type BI_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, HPSR_SPEC, bool, O>;
#[doc = "Field `SSM_STATE` reader - System Security Monitor State This field contains the encoded state of the SSM's state machine"]
pub type SSM_STATE_R = crate::FieldReader<u8, SSM_STATE_A>;
#[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSM_STATE_A {
    #[doc = "0: Init"]
    INIT = 0,
    #[doc = "1: Hard Fail"]
    HARD_FAIL = 1,
    #[doc = "3: Soft Fail"]
    SOFT_FAIL = 3,
    #[doc = "8: Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
    INTERMEDIATE = 8,
    #[doc = "9: Check"]
    CHECK = 9,
    #[doc = "11: Non-Secure"]
    NON_SECURE = 11,
    #[doc = "13: Trusted"]
    TRUSTED = 13,
    #[doc = "15: Secure"]
    SECURE = 15,
}
impl From<SSM_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSM_STATE_A) -> Self {
        variant as _
    }
}
impl SSM_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSM_STATE_A> {
        match self.bits {
            0 => Some(SSM_STATE_A::INIT),
            1 => Some(SSM_STATE_A::HARD_FAIL),
            3 => Some(SSM_STATE_A::SOFT_FAIL),
            8 => Some(SSM_STATE_A::INTERMEDIATE),
            9 => Some(SSM_STATE_A::CHECK),
            11 => Some(SSM_STATE_A::NON_SECURE),
            13 => Some(SSM_STATE_A::TRUSTED),
            15 => Some(SSM_STATE_A::SECURE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INIT`"]
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == SSM_STATE_A::INIT
    }
    #[doc = "Checks if the value of the field is `HARD_FAIL`"]
    #[inline(always)]
    pub fn is_hard_fail(&self) -> bool {
        *self == SSM_STATE_A::HARD_FAIL
    }
    #[doc = "Checks if the value of the field is `SOFT_FAIL`"]
    #[inline(always)]
    pub fn is_soft_fail(&self) -> bool {
        *self == SSM_STATE_A::SOFT_FAIL
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE`"]
    #[inline(always)]
    pub fn is_intermediate(&self) -> bool {
        *self == SSM_STATE_A::INTERMEDIATE
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == SSM_STATE_A::CHECK
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SSM_STATE_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `TRUSTED`"]
    #[inline(always)]
    pub fn is_trusted(&self) -> bool {
        *self == SSM_STATE_A::TRUSTED
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SSM_STATE_A::SECURE
    }
}
#[doc = "Field `SYS_SECURITY_CFG` reader - System Security Configuration This field reflects the three security configuration inputs to SNVS"]
pub type SYS_SECURITY_CFG_R = crate::FieldReader<u8, SYS_SECURITY_CFG_A>;
#[doc = "System Security Configuration This field reflects the three security configuration inputs to SNVS\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYS_SECURITY_CFG_A {
    #[doc = "0: Fab Configuration - the default configuration of newly fabricated chips"]
    FAB_CONFIG = 0,
    #[doc = "1: Open Configuration - the configuration after NXP-programmable fuses have been blown"]
    OPEN_CONFIG = 1,
    #[doc = "3: Closed Configuration - the configuration after OEM-programmable fuses have been blown"]
    CLOSED_CONFIG = 3,
    #[doc = "7: Field Return Configuration - the configuration of chips that are returned to NXP for analysis"]
    FIELD_RETURN_CONFIG = 7,
}
impl From<SYS_SECURITY_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SYS_SECURITY_CFG_A) -> Self {
        variant as _
    }
}
impl SYS_SECURITY_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYS_SECURITY_CFG_A> {
        match self.bits {
            0 => Some(SYS_SECURITY_CFG_A::FAB_CONFIG),
            1 => Some(SYS_SECURITY_CFG_A::OPEN_CONFIG),
            3 => Some(SYS_SECURITY_CFG_A::CLOSED_CONFIG),
            7 => Some(SYS_SECURITY_CFG_A::FIELD_RETURN_CONFIG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FAB_CONFIG`"]
    #[inline(always)]
    pub fn is_fab_config(&self) -> bool {
        *self == SYS_SECURITY_CFG_A::FAB_CONFIG
    }
    #[doc = "Checks if the value of the field is `OPEN_CONFIG`"]
    #[inline(always)]
    pub fn is_open_config(&self) -> bool {
        *self == SYS_SECURITY_CFG_A::OPEN_CONFIG
    }
    #[doc = "Checks if the value of the field is `CLOSED_CONFIG`"]
    #[inline(always)]
    pub fn is_closed_config(&self) -> bool {
        *self == SYS_SECURITY_CFG_A::CLOSED_CONFIG
    }
    #[doc = "Checks if the value of the field is `FIELD_RETURN_CONFIG`"]
    #[inline(always)]
    pub fn is_field_return_config(&self) -> bool {
        *self == SYS_SECURITY_CFG_A::FIELD_RETURN_CONFIG
    }
}
#[doc = "Field `SYS_SECURE_BOOT` reader - System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
pub type SYS_SECURE_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `OTPMK_SYNDROME` reader - One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
pub type OTPMK_SYNDROME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OTPMK_ZERO` reader - One Time Programmable Master Key is Equal to Zero"]
pub type OTPMK_ZERO_R = crate::BitReader<OTPMK_ZERO_A>;
#[doc = "One Time Programmable Master Key is Equal to Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTPMK_ZERO_A {
    #[doc = "0: The OTPMK is not zero."]
    OTPMK_NOT_ZERO = 0,
    #[doc = "1: The OTPMK is zero."]
    OTPMK_IS_ZERO = 1,
}
impl From<OTPMK_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: OTPMK_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl OTPMK_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTPMK_ZERO_A {
        match self.bits {
            false => OTPMK_ZERO_A::OTPMK_NOT_ZERO,
            true => OTPMK_ZERO_A::OTPMK_IS_ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `OTPMK_NOT_ZERO`"]
    #[inline(always)]
    pub fn is_otpmk_not_zero(&self) -> bool {
        *self == OTPMK_ZERO_A::OTPMK_NOT_ZERO
    }
    #[doc = "Checks if the value of the field is `OTPMK_IS_ZERO`"]
    #[inline(always)]
    pub fn is_otpmk_is_zero(&self) -> bool {
        *self == OTPMK_ZERO_A::OTPMK_IS_ZERO
    }
}
#[doc = "Field `ZMK_ZERO` reader - Zeroizable Master Key is Equal to Zero"]
pub type ZMK_ZERO_R = crate::BitReader<ZMK_ZERO_A>;
#[doc = "Zeroizable Master Key is Equal to Zero\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_ZERO_A {
    #[doc = "0: The ZMK is not zero."]
    ZMK_NOT_ZERO = 0,
    #[doc = "1: The ZMK is zero."]
    ZMK_IS_ZERO = 1,
}
impl From<ZMK_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_ZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ZERO_A {
        match self.bits {
            false => ZMK_ZERO_A::ZMK_NOT_ZERO,
            true => ZMK_ZERO_A::ZMK_IS_ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_NOT_ZERO`"]
    #[inline(always)]
    pub fn is_zmk_not_zero(&self) -> bool {
        *self == ZMK_ZERO_A::ZMK_NOT_ZERO
    }
    #[doc = "Checks if the value of the field is `ZMK_IS_ZERO`"]
    #[inline(always)]
    pub fn is_zmk_is_zero(&self) -> bool {
        *self == ZMK_ZERO_A::ZMK_IS_ZERO
    }
}
impl R {
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn hpta(&self) -> HPTA_R {
        HPTA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline(always)]
    pub fn lpdis(&self) -> LPDIS_R {
        LPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Button Value of the BTN input"]
    #[inline(always)]
    pub fn btn(&self) -> BTN_R {
        BTN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline(always)]
    pub fn ssm_state(&self) -> SSM_STATE_R {
        SSM_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - System Security Configuration This field reflects the three security configuration inputs to SNVS"]
    #[inline(always)]
    pub fn sys_security_cfg(&self) -> SYS_SECURITY_CFG_R {
        SYS_SECURITY_CFG_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - System Secure Boot If SYS_SECURE_BOOT is 1, the chip boots from internal ROM"]
    #[inline(always)]
    pub fn sys_secure_boot(&self) -> SYS_SECURE_BOOT_R {
        SYS_SECURE_BOOT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline(always)]
    pub fn otpmk_syndrome(&self) -> OTPMK_SYNDROME_R {
        OTPMK_SYNDROME_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - One Time Programmable Master Key is Equal to Zero"]
    #[inline(always)]
    pub fn otpmk_zero(&self) -> OTPMK_ZERO_R {
        OTPMK_ZERO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Zeroizable Master Key is Equal to Zero"]
    #[inline(always)]
    pub fn zmk_zero(&self) -> ZMK_ZERO_R {
        ZMK_ZERO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    #[must_use]
    pub fn hpta(&mut self) -> HPTA_W<0> {
        HPTA_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PI_W<1> {
        PI_W::new(self)
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    #[must_use]
    pub fn bi(&mut self) -> BI_W<7> {
        BI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpsr](index.html) module"]
pub struct HPSR_SPEC;
impl crate::RegisterSpec for HPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpsr::R](R) reader structure"]
impl crate::Readable for HPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpsr::W](W) writer structure"]
impl crate::Writable for HPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x83;
}
#[doc = "`reset()` method sets HPSR to value 0x8000_b000"]
impl crate::Resettable for HPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_b000;
}
