#[doc = "Register `GPR10` reader"]
pub struct R(crate::R<GPR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR10` writer"]
pub struct W(crate::W<GPR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR10_SPEC>;
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
impl From<crate::W<GPR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NIDEN` reader - Arm non-secure (non-invasive) debug enable"]
pub type NIDEN_R = crate::BitReader<NIDEN_A>;
#[doc = "Arm non-secure (non-invasive) debug enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NIDEN_A {
    #[doc = "0: Debug turned off"]
    NIDEN_0 = 0,
    #[doc = "1: Debug enabled (default)"]
    NIDEN_1 = 1,
}
impl From<NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_A {
        match self.bits {
            false => NIDEN_A::NIDEN_0,
            true => NIDEN_A::NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NIDEN_0`"]
    #[inline(always)]
    pub fn is_niden_0(&self) -> bool {
        *self == NIDEN_A::NIDEN_0
    }
    #[doc = "Checks if the value of the field is `NIDEN_1`"]
    #[inline(always)]
    pub fn is_niden_1(&self) -> bool {
        *self == NIDEN_A::NIDEN_1
    }
}
#[doc = "Field `NIDEN` writer - Arm non-secure (non-invasive) debug enable"]
pub type NIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, NIDEN_A, O>;
impl<'a, const O: u8> NIDEN_W<'a, O> {
    #[doc = "Debug turned off"]
    #[inline(always)]
    pub fn niden_0(self) -> &'a mut W {
        self.variant(NIDEN_A::NIDEN_0)
    }
    #[doc = "Debug enabled (default)"]
    #[inline(always)]
    pub fn niden_1(self) -> &'a mut W {
        self.variant(NIDEN_A::NIDEN_1)
    }
}
#[doc = "Field `DBG_EN` reader - Arm invasive debug enable"]
pub type DBG_EN_R = crate::BitReader<DBG_EN_A>;
#[doc = "Arm invasive debug enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_EN_A {
    #[doc = "0: Debug turned off"]
    DBG_EN_0 = 0,
    #[doc = "1: Debug enabled (default)"]
    DBG_EN_1 = 1,
}
impl From<DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            false => DBG_EN_A::DBG_EN_0,
            true => DBG_EN_A::DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline(always)]
    pub fn is_dbg_en_0(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline(always)]
    pub fn is_dbg_en_1(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_1
    }
}
#[doc = "Field `DBG_EN` writer - Arm invasive debug enable"]
pub type DBG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, DBG_EN_A, O>;
impl<'a, const O: u8> DBG_EN_W<'a, O> {
    #[doc = "Debug turned off"]
    #[inline(always)]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_0)
    }
    #[doc = "Debug enabled (default)"]
    #[inline(always)]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_1)
    }
}
#[doc = "Field `SEC_ERR_RESP` reader - Security error response enable"]
pub type SEC_ERR_RESP_R = crate::BitReader<SEC_ERR_RESP_A>;
#[doc = "Security error response enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_ERR_RESP_A {
    #[doc = "0: OKEY response"]
    SEC_ERR_RESP_0 = 0,
    #[doc = "1: SLVError (default)"]
    SEC_ERR_RESP_1 = 1,
}
impl From<SEC_ERR_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_ERR_RESP_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_ERR_RESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_ERR_RESP_A {
        match self.bits {
            false => SEC_ERR_RESP_A::SEC_ERR_RESP_0,
            true => SEC_ERR_RESP_A::SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_0`"]
    #[inline(always)]
    pub fn is_sec_err_resp_0(&self) -> bool {
        *self == SEC_ERR_RESP_A::SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_1`"]
    #[inline(always)]
    pub fn is_sec_err_resp_1(&self) -> bool {
        *self == SEC_ERR_RESP_A::SEC_ERR_RESP_1
    }
}
#[doc = "Field `SEC_ERR_RESP` writer - Security error response enable"]
pub type SEC_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, SEC_ERR_RESP_A, O>;
impl<'a, const O: u8> SEC_ERR_RESP_W<'a, O> {
    #[doc = "OKEY response"]
    #[inline(always)]
    pub fn sec_err_resp_0(self) -> &'a mut W {
        self.variant(SEC_ERR_RESP_A::SEC_ERR_RESP_0)
    }
    #[doc = "SLVError (default)"]
    #[inline(always)]
    pub fn sec_err_resp_1(self) -> &'a mut W {
        self.variant(SEC_ERR_RESP_A::SEC_ERR_RESP_1)
    }
}
#[doc = "Field `DCPKEY_OCOTP_OR_KEYMUX` reader - DCP Key selection bit."]
pub type DCPKEY_OCOTP_OR_KEYMUX_R = crate::BitReader<DCPKEY_OCOTP_OR_KEYMUX_A>;
#[doc = "DCP Key selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCPKEY_OCOTP_OR_KEYMUX_A {
    #[doc = "0: Select key from SNVS Master Key"]
    DCPKEY_OCOTP_OR_KEYMUX_0 = 0,
    #[doc = "1: Select key from OCOTP (SW_GP2)"]
    DCPKEY_OCOTP_OR_KEYMUX_1 = 1,
}
impl From<DCPKEY_OCOTP_OR_KEYMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DCPKEY_OCOTP_OR_KEYMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl DCPKEY_OCOTP_OR_KEYMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPKEY_OCOTP_OR_KEYMUX_A {
        match self.bits {
            false => DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0,
            true => DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline(always)]
    pub fn is_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline(always)]
    pub fn is_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Field `DCPKEY_OCOTP_OR_KEYMUX` writer - DCP Key selection bit."]
pub type DCPKEY_OCOTP_OR_KEYMUX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR10_SPEC, DCPKEY_OCOTP_OR_KEYMUX_A, O>;
impl<'a, const O: u8> DCPKEY_OCOTP_OR_KEYMUX_W<'a, O> {
    #[doc = "Select key from SNVS Master Key"]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Select key from OCOTP (SW_GP2)"]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1)
    }
}
#[doc = "Field `OCRAM_TZ_EN` reader - OCRAM TrustZone (TZ) enable."]
pub type OCRAM_TZ_EN_R = crate::BitReader<OCRAM_TZ_EN_A>;
#[doc = "OCRAM TrustZone (TZ) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCRAM_TZ_EN_A {
    #[doc = "0: The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)"]
    OCRAM_TZ_EN_0 = 0,
    #[doc = "1: The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\]
follows the execution mode access policy described in CSU chapter"]
    OCRAM_TZ_EN_1 = 1,
}
impl From<OCRAM_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM_TZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OCRAM_TZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM_TZ_EN_A {
        match self.bits {
            false => OCRAM_TZ_EN_A::OCRAM_TZ_EN_0,
            true => OCRAM_TZ_EN_A::OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_ocram_tz_en_0(&self) -> bool {
        *self == OCRAM_TZ_EN_A::OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_ocram_tz_en_1(&self) -> bool {
        *self == OCRAM_TZ_EN_A::OCRAM_TZ_EN_1
    }
}
#[doc = "Field `OCRAM_TZ_EN` writer - OCRAM TrustZone (TZ) enable."]
pub type OCRAM_TZ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, OCRAM_TZ_EN_A, O>;
impl<'a, const O: u8> OCRAM_TZ_EN_W<'a, O> {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)"]
    #[inline(always)]
    pub fn ocram_tz_en_0(self) -> &'a mut W {
        self.variant(OCRAM_TZ_EN_A::OCRAM_TZ_EN_0)
    }
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\]
follows the execution mode access policy described in CSU chapter"]
    #[inline(always)]
    pub fn ocram_tz_en_1(self) -> &'a mut W {
        self.variant(OCRAM_TZ_EN_A::OCRAM_TZ_EN_1)
    }
}
#[doc = "Field `OCRAM_TZ_ADDR` reader - OCRAM TrustZone (TZ) start address"]
pub type OCRAM_TZ_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCRAM_TZ_ADDR` writer - OCRAM TrustZone (TZ) start address"]
pub type OCRAM_TZ_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPR10_SPEC, u8, u8, 6, O>;
#[doc = "Field `LOCK_NIDEN` reader - Lock NIDEN field for changes"]
pub type LOCK_NIDEN_R = crate::BitReader<LOCK_NIDEN_A>;
#[doc = "Lock NIDEN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_NIDEN_A {
    #[doc = "0: Field is not locked"]
    LOCK_NIDEN_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_NIDEN_1 = 1,
}
impl From<LOCK_NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_NIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_NIDEN_A {
        match self.bits {
            false => LOCK_NIDEN_A::LOCK_NIDEN_0,
            true => LOCK_NIDEN_A::LOCK_NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_0`"]
    #[inline(always)]
    pub fn is_lock_niden_0(&self) -> bool {
        *self == LOCK_NIDEN_A::LOCK_NIDEN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_1`"]
    #[inline(always)]
    pub fn is_lock_niden_1(&self) -> bool {
        *self == LOCK_NIDEN_A::LOCK_NIDEN_1
    }
}
#[doc = "Field `LOCK_NIDEN` writer - Lock NIDEN field for changes"]
pub type LOCK_NIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, LOCK_NIDEN_A, O>;
impl<'a, const O: u8> LOCK_NIDEN_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_niden_0(self) -> &'a mut W {
        self.variant(LOCK_NIDEN_A::LOCK_NIDEN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_niden_1(self) -> &'a mut W {
        self.variant(LOCK_NIDEN_A::LOCK_NIDEN_1)
    }
}
#[doc = "Field `LOCK_DBG_EN` reader - Lock DBG_EN field for changes"]
pub type LOCK_DBG_EN_R = crate::BitReader<LOCK_DBG_EN_A>;
#[doc = "Lock DBG_EN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_DBG_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_DBG_EN_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_DBG_EN_1 = 1,
}
impl From<LOCK_DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_DBG_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_DBG_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_DBG_EN_A {
        match self.bits {
            false => LOCK_DBG_EN_A::LOCK_DBG_EN_0,
            true => LOCK_DBG_EN_A::LOCK_DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_0`"]
    #[inline(always)]
    pub fn is_lock_dbg_en_0(&self) -> bool {
        *self == LOCK_DBG_EN_A::LOCK_DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_1`"]
    #[inline(always)]
    pub fn is_lock_dbg_en_1(&self) -> bool {
        *self == LOCK_DBG_EN_A::LOCK_DBG_EN_1
    }
}
#[doc = "Field `LOCK_DBG_EN` writer - Lock DBG_EN field for changes"]
pub type LOCK_DBG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR10_SPEC, LOCK_DBG_EN_A, O>;
impl<'a, const O: u8> LOCK_DBG_EN_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_dbg_en_0(self) -> &'a mut W {
        self.variant(LOCK_DBG_EN_A::LOCK_DBG_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_dbg_en_1(self) -> &'a mut W {
        self.variant(LOCK_DBG_EN_A::LOCK_DBG_EN_1)
    }
}
#[doc = "Field `LOCK_SEC_ERR_RESP` reader - Lock SEC_ERR_RESP field for changes"]
pub type LOCK_SEC_ERR_RESP_R = crate::BitReader<LOCK_SEC_ERR_RESP_A>;
#[doc = "Lock SEC_ERR_RESP field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_SEC_ERR_RESP_A {
    #[doc = "0: Field is not locked"]
    LOCK_SEC_ERR_RESP_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_SEC_ERR_RESP_1 = 1,
}
impl From<LOCK_SEC_ERR_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_SEC_ERR_RESP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_SEC_ERR_RESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_SEC_ERR_RESP_A {
        match self.bits {
            false => LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0,
            true => LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_0`"]
    #[inline(always)]
    pub fn is_lock_sec_err_resp_0(&self) -> bool {
        *self == LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_1`"]
    #[inline(always)]
    pub fn is_lock_sec_err_resp_1(&self) -> bool {
        *self == LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1
    }
}
#[doc = "Field `LOCK_SEC_ERR_RESP` writer - Lock SEC_ERR_RESP field for changes"]
pub type LOCK_SEC_ERR_RESP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR10_SPEC, LOCK_SEC_ERR_RESP_A, O>;
impl<'a, const O: u8> LOCK_SEC_ERR_RESP_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_sec_err_resp_0(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_sec_err_resp_1(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1)
    }
}
#[doc = "Field `LOCK_DCPKEY_OCOTP_OR_KEYMUX` reader - Lock DCP Key OCOTP/Key MUX selection bit"]
pub type LOCK_DCPKEY_OCOTP_OR_KEYMUX_R = crate::BitReader<LOCK_DCPKEY_OCOTP_OR_KEYMUX_A>;
#[doc = "Lock DCP Key OCOTP/Key MUX selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_DCPKEY_OCOTP_OR_KEYMUX_A {
    #[doc = "0: Field is not locked"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_1 = 1,
}
impl From<LOCK_DCPKEY_OCOTP_OR_KEYMUX_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_DCPKEY_OCOTP_OR_KEYMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_DCPKEY_OCOTP_OR_KEYMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_A {
        match self.bits {
            false => LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
            true => LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline(always)]
    pub fn is_lock_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline(always)]
    pub fn is_lock_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Field `LOCK_DCPKEY_OCOTP_OR_KEYMUX` writer - Lock DCP Key OCOTP/Key MUX selection bit"]
pub type LOCK_DCPKEY_OCOTP_OR_KEYMUX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR10_SPEC, LOCK_DCPKEY_OCOTP_OR_KEYMUX_A, O>;
impl<'a, const O: u8> LOCK_DCPKEY_OCOTP_OR_KEYMUX_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1)
    }
}
#[doc = "Field `LOCK_OCRAM_TZ_EN` reader - Lock OCRAM_TZ_EN field for changes"]
pub type LOCK_OCRAM_TZ_EN_R = crate::BitReader<LOCK_OCRAM_TZ_EN_A>;
#[doc = "Lock OCRAM_TZ_EN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_OCRAM_TZ_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM_TZ_EN_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM_TZ_EN_1 = 1,
}
impl From<LOCK_OCRAM_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM_TZ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_OCRAM_TZ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_OCRAM_TZ_EN_A {
        match self.bits {
            false => LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0,
            true => LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_en_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_en_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1
    }
}
#[doc = "Field `LOCK_OCRAM_TZ_EN` writer - Lock OCRAM_TZ_EN field for changes"]
pub type LOCK_OCRAM_TZ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR10_SPEC, LOCK_OCRAM_TZ_EN_A, O>;
impl<'a, const O: u8> LOCK_OCRAM_TZ_EN_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram_tz_en_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram_tz_en_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1)
    }
}
#[doc = "Field `LOCK_OCRAM_TZ_ADDR` reader - Lock OCRAM_TZ_ADDR field for changes"]
pub type LOCK_OCRAM_TZ_ADDR_R = crate::FieldReader<u8, LOCK_OCRAM_TZ_ADDR_A>;
#[doc = "Lock OCRAM_TZ_ADDR field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_OCRAM_TZ_ADDR_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1 = 1,
}
impl From<LOCK_OCRAM_TZ_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM_TZ_ADDR_A) -> Self {
        variant as _
    }
}
impl LOCK_OCRAM_TZ_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_OCRAM_TZ_ADDR_A> {
        match self.bits {
            0 => Some(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0),
            1 => Some(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_0`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_addr_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_1`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_addr_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1
    }
}
#[doc = "Field `LOCK_OCRAM_TZ_ADDR` writer - Lock OCRAM_TZ_ADDR field for changes"]
pub type LOCK_OCRAM_TZ_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR10_SPEC, u8, LOCK_OCRAM_TZ_ADDR_A, 7, O>;
impl<'a, const O: u8> LOCK_OCRAM_TZ_ADDR_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1)
    }
}
impl R {
    #[doc = "Bit 0 - Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arm invasive debug enable"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security error response enable"]
    #[inline(always)]
    pub fn sec_err_resp(&self) -> SEC_ERR_RESP_R {
        SEC_ERR_RESP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux(&self) -> DCPKEY_OCOTP_OR_KEYMUX_R {
        DCPKEY_OCOTP_OR_KEYMUX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub fn ocram_tz_en(&self) -> OCRAM_TZ_EN_R {
        OCRAM_TZ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub fn ocram_tz_addr(&self) -> OCRAM_TZ_ADDR_R {
        OCRAM_TZ_ADDR_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline(always)]
    pub fn lock_niden(&self) -> LOCK_NIDEN_R {
        LOCK_NIDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline(always)]
    pub fn lock_dbg_en(&self) -> LOCK_DBG_EN_R {
        LOCK_DBG_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub fn lock_sec_err_resp(&self) -> LOCK_SEC_ERR_RESP_R {
        LOCK_SEC_ERR_RESP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux(&self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_R {
        LOCK_DCPKEY_OCOTP_OR_KEYMUX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_en(&self) -> LOCK_OCRAM_TZ_EN_R {
        LOCK_OCRAM_TZ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr(&self) -> LOCK_OCRAM_TZ_ADDR_R {
        LOCK_OCRAM_TZ_ADDR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn niden(&mut self) -> NIDEN_W<0> {
        NIDEN_W::new(self)
    }
    #[doc = "Bit 1 - Arm invasive debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_en(&mut self) -> DBG_EN_W<1> {
        DBG_EN_W::new(self)
    }
    #[doc = "Bit 2 - Security error response enable"]
    #[inline(always)]
    #[must_use]
    pub fn sec_err_resp(&mut self) -> SEC_ERR_RESP_W<2> {
        SEC_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline(always)]
    #[must_use]
    pub fn dcpkey_ocotp_or_keymux(&mut self) -> DCPKEY_OCOTP_OR_KEYMUX_W<4> {
        DCPKEY_OCOTP_OR_KEYMUX_W::new(self)
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    #[must_use]
    pub fn ocram_tz_en(&mut self) -> OCRAM_TZ_EN_W<8> {
        OCRAM_TZ_EN_W::new(self)
    }
    #[doc = "Bits 9:14 - OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    #[must_use]
    pub fn ocram_tz_addr(&mut self) -> OCRAM_TZ_ADDR_W<9> {
        OCRAM_TZ_ADDR_W::new(self)
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_niden(&mut self) -> LOCK_NIDEN_W<16> {
        LOCK_NIDEN_W::new(self)
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_dbg_en(&mut self) -> LOCK_DBG_EN_W<17> {
        LOCK_DBG_EN_W::new(self)
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_sec_err_resp(&mut self) -> LOCK_SEC_ERR_RESP_W<18> {
        LOCK_SEC_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_dcpkey_ocotp_or_keymux(&mut self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_W<20> {
        LOCK_DCPKEY_OCOTP_OR_KEYMUX_W::new(self)
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ocram_tz_en(&mut self) -> LOCK_OCRAM_TZ_EN_W<24> {
        LOCK_OCRAM_TZ_EN_W::new(self)
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ocram_tz_addr(&mut self) -> LOCK_OCRAM_TZ_ADDR_W<25> {
        LOCK_OCRAM_TZ_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR10 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr10](index.html) module"]
pub struct GPR10_SPEC;
impl crate::RegisterSpec for GPR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr10::R](R) reader structure"]
impl crate::Readable for GPR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr10::W](W) writer structure"]
impl crate::Writable for GPR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR10 to value 0x07"]
impl crate::Resettable for GPR10_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
