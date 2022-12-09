#[doc = "Register `HPLR` reader"]
pub struct R(crate::R<HPLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPLR` writer"]
pub struct W(crate::W<HPLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPLR_SPEC>;
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
impl From<crate::W<HPLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZMK_WSL` reader - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
pub type ZMK_WSL_R = crate::BitReader<ZMK_WSL_A>;
#[doc = "Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_WSL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_NOT_ALLOWED = 1,
}
impl From<ZMK_WSL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_WSL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_WSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_WSL_A {
        match self.bits {
            false => ZMK_WSL_A::WRITE_ALLOWED,
            true => ZMK_WSL_A::WRITE_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == ZMK_WSL_A::WRITE_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == ZMK_WSL_A::WRITE_NOT_ALLOWED
    }
}
#[doc = "Field `ZMK_WSL` writer - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
pub type ZMK_WSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, ZMK_WSL_A, O>;
impl<'a, const O: u8> ZMK_WSL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(ZMK_WSL_A::WRITE_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(ZMK_WSL_A::WRITE_NOT_ALLOWED)
    }
}
#[doc = "Field `ZMK_RSL` reader - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
pub type ZMK_RSL_R = crate::BitReader<ZMK_RSL_A>;
#[doc = "Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZMK_RSL_A {
    #[doc = "0: Read access is allowed (only in software Programming mode)"]
    READ_ALLOWED = 0,
    #[doc = "1: Read access is not allowed"]
    READ_NOT_ALLOWED = 1,
}
impl From<ZMK_RSL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_RSL_A) -> Self {
        variant as u8 != 0
    }
}
impl ZMK_RSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_RSL_A {
        match self.bits {
            false => ZMK_RSL_A::READ_ALLOWED,
            true => ZMK_RSL_A::READ_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `READ_ALLOWED`"]
    #[inline(always)]
    pub fn is_read_allowed(&self) -> bool {
        *self == ZMK_RSL_A::READ_ALLOWED
    }
    #[doc = "Checks if the value of the field is `READ_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_read_not_allowed(&self) -> bool {
        *self == ZMK_RSL_A::READ_NOT_ALLOWED
    }
}
#[doc = "Field `ZMK_RSL` writer - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
pub type ZMK_RSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, ZMK_RSL_A, O>;
impl<'a, const O: u8> ZMK_RSL_W<'a, O> {
    #[doc = "Read access is allowed (only in software Programming mode)"]
    #[inline(always)]
    pub fn read_allowed(self) -> &'a mut W {
        self.variant(ZMK_RSL_A::READ_ALLOWED)
    }
    #[doc = "Read access is not allowed"]
    #[inline(always)]
    pub fn read_not_allowed(self) -> &'a mut W {
        self.variant(ZMK_RSL_A::READ_NOT_ALLOWED)
    }
}
#[doc = "Field `SRTC_SL` reader - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
pub type SRTC_SL_R = crate::BitReader<SRTC_SL_A>;
#[doc = "Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRTC_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_NOT_ALLOWED = 1,
}
impl From<SRTC_SL_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl SRTC_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_SL_A {
        match self.bits {
            false => SRTC_SL_A::WRITE_ALLOWED,
            true => SRTC_SL_A::WRITE_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == SRTC_SL_A::WRITE_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == SRTC_SL_A::WRITE_NOT_ALLOWED
    }
}
#[doc = "Field `SRTC_SL` writer - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
pub type SRTC_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, SRTC_SL_A, O>;
impl<'a, const O: u8> SRTC_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(SRTC_SL_A::WRITE_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(SRTC_SL_A::WRITE_NOT_ALLOWED)
    }
}
#[doc = "Field `LPCALB_SL` reader - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
pub type LPCALB_SL_R = crate::BitReader<LPCALB_SL_A>;
#[doc = "LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCALB_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_NOT_ALLOWED = 1,
}
impl From<LPCALB_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCALB_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_SL_A {
        match self.bits {
            false => LPCALB_SL_A::WRITE_ALLOWED,
            true => LPCALB_SL_A::WRITE_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == LPCALB_SL_A::WRITE_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == LPCALB_SL_A::WRITE_NOT_ALLOWED
    }
}
#[doc = "Field `LPCALB_SL` writer - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
pub type LPCALB_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, LPCALB_SL_A, O>;
impl<'a, const O: u8> LPCALB_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(LPCALB_SL_A::WRITE_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(LPCALB_SL_A::WRITE_NOT_ALLOWED)
    }
}
#[doc = "Field `MC_SL` reader - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
pub type MC_SL_R = crate::BitReader<MC_SL_A>;
#[doc = "Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MC_SL_A {
    #[doc = "0: Write access (increment) is allowed"]
    WRITE_ALLOWED = 0,
    #[doc = "1: Write access (increment) is not allowed"]
    WRITE_NOT_ALLOWED = 1,
}
impl From<MC_SL_A> for bool {
    #[inline(always)]
    fn from(variant: MC_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl MC_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_SL_A {
        match self.bits {
            false => MC_SL_A::WRITE_ALLOWED,
            true => MC_SL_A::WRITE_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == MC_SL_A::WRITE_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == MC_SL_A::WRITE_NOT_ALLOWED
    }
}
#[doc = "Field `MC_SL` writer - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
pub type MC_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, MC_SL_A, O>;
impl<'a, const O: u8> MC_SL_W<'a, O> {
    #[doc = "Write access (increment) is allowed"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(MC_SL_A::WRITE_ALLOWED)
    }
    #[doc = "Write access (increment) is not allowed"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(MC_SL_A::WRITE_NOT_ALLOWED)
    }
}
#[doc = "Field `GPR_SL` reader - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
pub type GPR_SL_R = crate::BitReader<GPR_SL_A>;
#[doc = "General Purpose Register Soft Lock When set, prevents any writes to the GPR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPR_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_NOT_ALLOWED = 1,
}
impl From<GPR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: GPR_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl GPR_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPR_SL_A {
        match self.bits {
            false => GPR_SL_A::WRITE_ALLOWED,
            true => GPR_SL_A::WRITE_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_allowed(&self) -> bool {
        *self == GPR_SL_A::WRITE_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_not_allowed(&self) -> bool {
        *self == GPR_SL_A::WRITE_NOT_ALLOWED
    }
}
#[doc = "Field `GPR_SL` writer - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
pub type GPR_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, GPR_SL_A, O>;
impl<'a, const O: u8> GPR_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_allowed(self) -> &'a mut W {
        self.variant(GPR_SL_A::WRITE_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_not_allowed(self) -> &'a mut W {
        self.variant(GPR_SL_A::WRITE_NOT_ALLOWED)
    }
}
#[doc = "Field `LPSVCR_SL` reader - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
pub type LPSVCR_SL_R = crate::BitReader<LPSVCR_SL_A>;
#[doc = "LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSVCR_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<LPSVCR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSVCR_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSVCR_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSVCR_SL_A {
        match self.bits {
            false => LPSVCR_SL_A::WRITE_ACCESS_ALLOWED,
            true => LPSVCR_SL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == LPSVCR_SL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == LPSVCR_SL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `LPSVCR_SL` writer - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
pub type LPSVCR_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, LPSVCR_SL_A, O>;
impl<'a, const O: u8> LPSVCR_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(LPSVCR_SL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(LPSVCR_SL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `LPSECR_SL` reader - LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
pub type LPSECR_SL_R = crate::BitReader<LPSECR_SL_A>;
#[doc = "LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSECR_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<LPSECR_SL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSECR_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSECR_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSECR_SL_A {
        match self.bits {
            false => LPSECR_SL_A::WRITE_ACCESS_ALLOWED,
            true => LPSECR_SL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == LPSECR_SL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == LPSECR_SL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `LPSECR_SL` writer - LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
pub type LPSECR_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, LPSECR_SL_A, O>;
impl<'a, const O: u8> LPSECR_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(LPSECR_SL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(LPSECR_SL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `MKS_SL` reader - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
pub type MKS_SL_R = crate::BitReader<MKS_SL_A>;
#[doc = "Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MKS_SL_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<MKS_SL_A> for bool {
    #[inline(always)]
    fn from(variant: MKS_SL_A) -> Self {
        variant as u8 != 0
    }
}
impl MKS_SL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MKS_SL_A {
        match self.bits {
            false => MKS_SL_A::WRITE_ACCESS_ALLOWED,
            true => MKS_SL_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == MKS_SL_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == MKS_SL_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `MKS_SL` writer - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
pub type MKS_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, MKS_SL_A, O>;
impl<'a, const O: u8> MKS_SL_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(MKS_SL_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(MKS_SL_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `HPSVCR_L` reader - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
pub type HPSVCR_L_R = crate::BitReader<HPSVCR_L_A>;
#[doc = "HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPSVCR_L_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<HPSVCR_L_A> for bool {
    #[inline(always)]
    fn from(variant: HPSVCR_L_A) -> Self {
        variant as u8 != 0
    }
}
impl HPSVCR_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPSVCR_L_A {
        match self.bits {
            false => HPSVCR_L_A::WRITE_ACCESS_ALLOWED,
            true => HPSVCR_L_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == HPSVCR_L_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == HPSVCR_L_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `HPSVCR_L` writer - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
pub type HPSVCR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, HPSVCR_L_A, O>;
impl<'a, const O: u8> HPSVCR_L_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(HPSVCR_L_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(HPSVCR_L_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `HPSICR_L` reader - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
pub type HPSICR_L_R = crate::BitReader<HPSICR_L_A>;
#[doc = "HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPSICR_L_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<HPSICR_L_A> for bool {
    #[inline(always)]
    fn from(variant: HPSICR_L_A) -> Self {
        variant as u8 != 0
    }
}
impl HPSICR_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPSICR_L_A {
        match self.bits {
            false => HPSICR_L_A::WRITE_ACCESS_ALLOWED,
            true => HPSICR_L_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == HPSICR_L_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == HPSICR_L_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `HPSICR_L` writer - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
pub type HPSICR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, HPSICR_L_A, O>;
impl<'a, const O: u8> HPSICR_L_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(HPSICR_L_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(HPSICR_L_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
#[doc = "Field `HAC_L` reader - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
pub type HAC_L_R = crate::BitReader<HAC_L_A>;
#[doc = "High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAC_L_A {
    #[doc = "0: Write access is allowed"]
    WRITE_ACCESS_ALLOWED = 0,
    #[doc = "1: Write access is not allowed"]
    WRITE_ACCESS_NOT_ALLOWED = 1,
}
impl From<HAC_L_A> for bool {
    #[inline(always)]
    fn from(variant: HAC_L_A) -> Self {
        variant as u8 != 0
    }
}
impl HAC_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAC_L_A {
        match self.bits {
            false => HAC_L_A::WRITE_ACCESS_ALLOWED,
            true => HAC_L_A::WRITE_ACCESS_NOT_ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_allowed(&self) -> bool {
        *self == HAC_L_A::WRITE_ACCESS_ALLOWED
    }
    #[doc = "Checks if the value of the field is `WRITE_ACCESS_NOT_ALLOWED`"]
    #[inline(always)]
    pub fn is_write_access_not_allowed(&self) -> bool {
        *self == HAC_L_A::WRITE_ACCESS_NOT_ALLOWED
    }
}
#[doc = "Field `HAC_L` writer - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
pub type HAC_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPLR_SPEC, HAC_L_A, O>;
impl<'a, const O: u8> HAC_L_W<'a, O> {
    #[doc = "Write access is allowed"]
    #[inline(always)]
    pub fn write_access_allowed(self) -> &'a mut W {
        self.variant(HAC_L_A::WRITE_ACCESS_ALLOWED)
    }
    #[doc = "Write access is not allowed"]
    #[inline(always)]
    pub fn write_access_not_allowed(self) -> &'a mut W {
        self.variant(HAC_L_A::WRITE_ACCESS_NOT_ALLOWED)
    }
}
impl R {
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_wsl(&self) -> ZMK_WSL_R {
        ZMK_WSL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    pub fn zmk_rsl(&self) -> ZMK_RSL_R {
        ZMK_RSL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    pub fn srtc_sl(&self) -> SRTC_SL_R {
        SRTC_SL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    pub fn lpcalb_sl(&self) -> LPCALB_SL_R {
        LPCALB_SL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    pub fn mc_sl(&self) -> MC_SL_R {
        MC_SL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    pub fn gpr_sl(&self) -> GPR_SL_R {
        GPR_SL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    pub fn lpsvcr_sl(&self) -> LPSVCR_SL_R {
        LPSVCR_SL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    pub fn lpsecr_sl(&self) -> LPSECR_SL_R {
        LPSECR_SL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    pub fn mks_sl(&self) -> MKS_SL_R {
        MKS_SL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    pub fn hpsvcr_l(&self) -> HPSVCR_L_R {
        HPSVCR_L_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    pub fn hpsicr_l(&self) -> HPSICR_L_R {
        HPSICR_L_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    pub fn hac_l(&self) -> HAC_L_R {
        HAC_L_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Zeroizable Master Key Write Soft Lock When set, prevents any writes (software and hardware) to the ZMK registers and the ZMK_HWP, ZMK_VAL, and ZMK_ECC_EN fields of the LPMKCR"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_wsl(&mut self) -> ZMK_WSL_W<0> {
        ZMK_WSL_W::new(self)
    }
    #[doc = "Bit 1 - Zeroizable Master Key Read Soft Lock When set, prevents any software reads to the ZMK Registers and ZMK_ECC_VALUE field of the LPMKCR"]
    #[inline(always)]
    #[must_use]
    pub fn zmk_rsl(&mut self) -> ZMK_RSL_W<1> {
        ZMK_RSL_W::new(self)
    }
    #[doc = "Bit 2 - Secure Real Time Counter Soft Lock When set, prevents any writes to the SRTC Registers, SRTC_ENV, and SRTC_INV_EN bits"]
    #[inline(always)]
    #[must_use]
    pub fn srtc_sl(&mut self) -> SRTC_SL_W<2> {
        SRTC_SL_W::new(self)
    }
    #[doc = "Bit 3 - LP Calibration Soft Lock When set, prevents any writes to the LP Calibration Value (LPCALB_VAL) and LP Calibration Enable (LPCALB_EN)"]
    #[inline(always)]
    #[must_use]
    pub fn lpcalb_sl(&mut self) -> LPCALB_SL_W<3> {
        LPCALB_SL_W::new(self)
    }
    #[doc = "Bit 4 - Monotonic Counter Soft Lock When set, prevents any writes (increments) to the MC Registers and MC_ENV bit"]
    #[inline(always)]
    #[must_use]
    pub fn mc_sl(&mut self) -> MC_SL_W<4> {
        MC_SL_W::new(self)
    }
    #[doc = "Bit 5 - General Purpose Register Soft Lock When set, prevents any writes to the GPR"]
    #[inline(always)]
    #[must_use]
    pub fn gpr_sl(&mut self) -> GPR_SL_W<5> {
        GPR_SL_W::new(self)
    }
    #[doc = "Bit 6 - LP Security Violation Control Register Soft Lock When set, prevents any writes to the LPSVCR"]
    #[inline(always)]
    #[must_use]
    pub fn lpsvcr_sl(&mut self) -> LPSVCR_SL_W<6> {
        LPSVCR_SL_W::new(self)
    }
    #[doc = "Bit 8 - LP Security Events Configuration Register Soft Lock When set, prevents any writes to the LPSECR"]
    #[inline(always)]
    #[must_use]
    pub fn lpsecr_sl(&mut self) -> LPSECR_SL_W<8> {
        LPSECR_SL_W::new(self)
    }
    #[doc = "Bit 9 - Master Key Select Soft Lock When set, prevents any writes to the MASTER_KEY_SEL field of the LPMKCR"]
    #[inline(always)]
    #[must_use]
    pub fn mks_sl(&mut self) -> MKS_SL_W<9> {
        MKS_SL_W::new(self)
    }
    #[doc = "Bit 16 - HP Security Violation Control Register Lock When set, prevents any writes to the HPSVCR"]
    #[inline(always)]
    #[must_use]
    pub fn hpsvcr_l(&mut self) -> HPSVCR_L_W<16> {
        HPSVCR_L_W::new(self)
    }
    #[doc = "Bit 17 - HP Security Interrupt Control Register Lock When set, prevents any writes to the HPSICR"]
    #[inline(always)]
    #[must_use]
    pub fn hpsicr_l(&mut self) -> HPSICR_L_W<17> {
        HPSICR_L_W::new(self)
    }
    #[doc = "Bit 18 - High Assurance Counter Lock When set, prevents any writes to HPHACIVR, HPHACR, and HAC_EN bit of HPCOMR"]
    #[inline(always)]
    #[must_use]
    pub fn hac_l(&mut self) -> HAC_L_W<18> {
        HAC_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hplr](index.html) module"]
pub struct HPLR_SPEC;
impl crate::RegisterSpec for HPLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hplr::R](R) reader structure"]
impl crate::Readable for HPLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hplr::W](W) writer structure"]
impl crate::Writable for HPLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPLR to value 0"]
impl crate::Resettable for HPLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
