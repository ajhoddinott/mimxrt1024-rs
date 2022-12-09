#[doc = "Register `LPLR` reader"]
pub struct R(crate::R<LPLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPLR` writer"]
pub struct W(crate::W<LPLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPLR_SPEC>;
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
impl From<crate::W<LPLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZMK_WHL` reader - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
pub type ZMK_WHL_R = crate::BitReader<ZMK_WHL_A>;
#[doc = "Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_WHL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<ZMK_WHL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_WHL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_WHL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_WHL_A {
        match self.bits {
            false => ZMK_WHL_A::WRITE_ACCESS_ALLOWED,
            true => ZMK_WHL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == ZMK_WHL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == ZMK_WHL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `ZMK_WHL` writer - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
pub type ZMK_WHL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, ZMK_WHL_A, O>;
impl<'a, const O: u8> ZMK_WHL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(ZMK_WHL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(ZMK_WHL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `ZMK_RHL` reader - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
pub type ZMK_RHL_R = crate::BitReader<ZMK_RHL_A>;
#[doc = "Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_RHL_A {
    #[doc = "0: Read access is allowed (only in software programming mode)."]
    READ_ACCESS_ALLOWED = 0,
    #[doc = "1: Read access is not allowed."]
    READ_ACCESS_NOT_ALLOWED = 1,
}
impl From<ZMK_RHL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_RHL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_RHL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_RHL_A {
        match self.bits {
            false => ZMK_RHL_A::READ_ACCESS_ALLOWED,
            true => ZMK_RHL_A::READ_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_read_access_allowed(&self) -> bool {
        *self == ZMK_RHL_A::READ_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_read_access_not_allowed(&self) -> bool {
        *self == ZMK_RHL_A::READ_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `ZMK_RHL` writer - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
pub type ZMK_RHL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, ZMK_RHL_A, O>;
impl<'a, const O: u8> ZMK_RHL_W<'a, O> {
    #[doc = "Read access is allowed (only in software programming mode)."]
    #[inline(always)]
    pub fn read_access_allowed(self) -> &'a mut W {
        self.variant(ZMK_RHL_A::READ_ACCESS_ALLOWED)
    }
    #[doc = "Read access is not allowed."]
    #[inline(always)]
    pub fn read_access_not_allowed(self) -> &'a mut W {
        self.variant(ZMK_RHL_A::READ_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `SRTC_HL` reader - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
pub type SRTC_HL_R = crate::BitReader<SRTC_HL_A>;
#[doc = "Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTC_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<SRTC_HL_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTC_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_HL_A {
        match self.bits {
            false => SRTC_HL_A::WRITE_ACCESS_ALLOWED,
            true => SRTC_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == SRTC_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == SRTC_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `SRTC_HL` writer - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
pub type SRTC_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, SRTC_HL_A, O>;
impl<'a, const O: u8> SRTC_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(SRTC_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(SRTC_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `LPCALB_HL` reader - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
pub type LPCALB_HL_R = crate::BitReader<LPCALB_HL_A>;
#[doc = "LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCALB_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<LPCALB_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCALB_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_HL_A {
        match self.bits {
            false => LPCALB_HL_A::WRITE_ACCESS_ALLOWED,
            true => LPCALB_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == LPCALB_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == LPCALB_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `LPCALB_HL` writer - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
pub type LPCALB_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, LPCALB_HL_A, O>;
impl<'a, const O: u8> LPCALB_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(LPCALB_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(LPCALB_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `MC_HL` reader - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
pub type MC_HL_R = crate::BitReader<MC_HL_A>;
#[doc = "Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MC_HL_A {
    #[doc = "0: Write access (increment) is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access (increment) is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<MC_HL_A> for bool {
    #[inline(always)]
    fn from(variant: MC_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl MC_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_HL_A {
        match self.bits {
            false => MC_HL_A::WRITE_ACCESS_ALLOWED,
            true => MC_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == MC_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == MC_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `MC_HL` writer - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
pub type MC_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, MC_HL_A, O>;
impl<'a, const O: u8> MC_HL_W<'a, O> {
    #[doc = "Write access (increment) is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(MC_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access (increment) is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(MC_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `GPR_HL` reader - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
pub type GPR_HL_R = crate::BitReader<GPR_HL_A>;
#[doc = "General Purpose Register Hard Lock When set, prevents any writes to the GPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPR_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<GPR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: GPR_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl GPR_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPR_HL_A {
        match self.bits {
            false => GPR_HL_A::WRITE_ACCESS_ALLOWED,
            true => GPR_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == GPR_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == GPR_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `GPR_HL` writer - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
pub type GPR_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, GPR_HL_A, O>;
impl<'a, const O: u8> GPR_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(GPR_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(GPR_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `LPSVCR_HL` reader - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
pub type LPSVCR_HL_R = crate::BitReader<LPSVCR_HL_A>;
#[doc = "LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSVCR_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<LPSVCR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVCR_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSVCR_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVCR_HL_A {
        match self.bits {
            false => LPSVCR_HL_A::WRITE_ACCESS_ALLOWED,
            true => LPSVCR_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == LPSVCR_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == LPSVCR_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `LPSVCR_HL` writer - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
pub type LPSVCR_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, LPSVCR_HL_A, O>;
impl<'a, const O: u8> LPSVCR_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(LPSVCR_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(LPSVCR_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `LPSECR_HL` reader - LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
pub type LPSECR_HL_R = crate::BitReader<LPSECR_HL_A>;
#[doc = "LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSECR_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<LPSECR_HL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSECR_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSECR_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSECR_HL_A {
        match self.bits {
            false => LPSECR_HL_A::WRITE_ACCESS_ALLOWED,
            true => LPSECR_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == LPSECR_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == LPSECR_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `LPSECR_HL` writer - LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
pub type LPSECR_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, LPSECR_HL_A, O>;
impl<'a, const O: u8> LPSECR_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(LPSECR_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(LPSECR_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `MKS_HL` reader - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
pub type MKS_HL_R = crate::BitReader<MKS_HL_A>;
#[doc = "Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MKS_HL_A {
    #[doc = "0: Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<MKS_HL_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_HL_A) -> Self {
        variant as u8 != 0
    }
}
impl MKS_HL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_HL_A {
        match self.bits {
            false => MKS_HL_A::WRITE_ACCESS_ALLOWED,
            true => MKS_HL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == MKS_HL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == MKS_HL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `MKS_HL` writer - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
pub type MKS_HL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPLR_SPEC, MKS_HL_A, O>;
impl<'a, const O: u8> MKS_HL_W<'a, O> {
    #[doc = "Write access is allowed."]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(MKS_HL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed."]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(MKS_HL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
impl R {
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_whl(&self) -> ZMK_WHL_R {
        ZMK_WHL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rhl(&self) -> ZMK_RHL_R {
        ZMK_RHL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_hl(&self) -> SRTC_HL_R {
        SRTC_HL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_hl(&self) -> LPCALB_HL_R {
        LPCALB_HL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_hl(&self) -> MC_HL_R {
        MC_HL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_hl(&self) -> GPR_HL_R {
        GPR_HL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_hl(&self) -> LPSVCR_HL_R {
        LPSVCR_HL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub fn lpsecr_hl(&self) -> LPSECR_HL_R {
        LPSECR_HL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    pub fn mks_hl(&self) -> MKS_HL_R {
        MKS_HL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zeroizable Master Key Write Hard Lock When set, prevents any writes (software and hardware) to the ZMK registers and ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_whl(&mut self) -> ZMK_WHL_W<0> {
        ZMK_WHL_W::new(self)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Hard Lock When set, prevents any software reads to the ZMK registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_rhl(&mut self) -> ZMK_RHL_W<1> {
        ZMK_RHL_W::new(self)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Hard Lock When set, prevents any writes to the SRTC registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    #[must_use]
    pub fn srtc_hl(&mut self) -> SRTC_HL_W<2> {
        SRTC_HL_W::new(self)
    }
    #[doc = "Bit 3 - LP Calibration Hard Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    #[must_use]
    pub fn lpcalb_hl(&mut self) -> LPCALB_HL_W<3> {
        LPCALB_HL_W::new(self)
    }
    #[doc = "Bit 4 - Monotonic Counter Hard Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    #[must_use]
    pub fn mc_hl(&mut self) -> MC_HL_W<4> {
        MC_HL_W::new(self)
    }
    #[doc = "Bit 5 - General Purpose Register Hard Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    #[must_use]
    pub fn gpr_hl(&mut self) -> GPR_HL_W<5> {
        GPR_HL_W::new(self)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Hard Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    #[must_use]
    pub fn lpsvcr_hl(&mut self) -> LPSVCR_HL_W<6> {
        LPSVCR_HL_W::new(self)
    }
    #[doc = "Bit 8 - LP Security Events Configuration Register Hard Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    #[must_use]
    pub fn lpsecr_hl(&mut self) -> LPSECR_HL_W<8> {
        LPSECR_HL_W::new(self)
    }
    #[doc = "Bit 9 - Master Key Select Hard Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LP Master Key Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn mks_hl(&mut self) -> MKS_HL_W<9> {
        MKS_HL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lplr](index.html) module"]
pub struct LPLR_SPEC;
impl crate::RegisterSpec for LPLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lplr::R](R) reader structure"]
impl crate::Readable for LPLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lplr::W](W) writer structure"]
impl crate::Writable for LPLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPLR to value 0"]
impl crate::Resettable for LPLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
