#[doc = "Register `GPR11` reader"]
pub struct R(crate::R<GPR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR11` writer"]
pub struct W(crate::W<GPR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR11_SPEC>;
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
impl From<crate::W<GPR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M7_APC_AC_R0_CTRL` reader - Access control of memory region-0"]
pub type M7_APC_AC_R0_CTRL_R = crate::FieldReader<u8, M7_APC_AC_R0_CTRL_A>;
#[doc = "Access control of memory region-0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M7_APC_AC_R0_CTRL_A {
    #[doc = "0: No access protection - All accesses are allowed"]
    M7_APC_AC_R0_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R0_TOP/BOT specified region (IOMUX_GPR_GPR18 - IOMUX_GPR_GPR19)"]
    M7_APC_AC_R0_CTRL_1 = 1,
}
impl From<M7_APC_AC_R0_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R0_CTRL_A) -> Self {
        variant as _
    }
}
impl M7_APC_AC_R0_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M7_APC_AC_R0_CTRL_A> {
        match self.bits {
            0 => Some(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0),
            1 => Some(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1
    }
}
#[doc = "Field `M7_APC_AC_R0_CTRL` writer - Access control of memory region-0"]
pub type M7_APC_AC_R0_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, M7_APC_AC_R0_CTRL_A, 2, O>;
impl<'a, const O: u8> M7_APC_AC_R0_CTRL_W<'a, O> {
    #[doc = "No access protection - All accesses are allowed"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0)
    }
    #[doc = "M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R0_TOP/BOT specified region (IOMUX_GPR_GPR18 - IOMUX_GPR_GPR19)"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1)
    }
}
#[doc = "Field `M7_APC_AC_R1_CTRL` reader - Access control of memory region-1"]
pub type M7_APC_AC_R1_CTRL_R = crate::FieldReader<u8, M7_APC_AC_R1_CTRL_A>;
#[doc = "Access control of memory region-1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M7_APC_AC_R1_CTRL_A {
    #[doc = "0: No access protection - All accesses are allowed"]
    M7_APC_AC_R1_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R1_TOP/BOT specified region (IOMUX_GPR_GPR20 - IOMUX_GPR_GPR21)"]
    M7_APC_AC_R1_CTRL_1 = 1,
}
impl From<M7_APC_AC_R1_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R1_CTRL_A) -> Self {
        variant as _
    }
}
impl M7_APC_AC_R1_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M7_APC_AC_R1_CTRL_A> {
        match self.bits {
            0 => Some(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0),
            1 => Some(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1
    }
}
#[doc = "Field `M7_APC_AC_R1_CTRL` writer - Access control of memory region-1"]
pub type M7_APC_AC_R1_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, M7_APC_AC_R1_CTRL_A, 2, O>;
impl<'a, const O: u8> M7_APC_AC_R1_CTRL_W<'a, O> {
    #[doc = "No access protection - All accesses are allowed"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0)
    }
    #[doc = "M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R1_TOP/BOT specified region (IOMUX_GPR_GPR20 - IOMUX_GPR_GPR21)"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1)
    }
}
#[doc = "Field `M7_APC_AC_R2_CTRL` reader - Access control of memory region-2"]
pub type M7_APC_AC_R2_CTRL_R = crate::FieldReader<u8, M7_APC_AC_R2_CTRL_A>;
#[doc = "Access control of memory region-2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M7_APC_AC_R2_CTRL_A {
    #[doc = "0: No access protection - All accesses are allowed"]
    M7_APC_AC_R2_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R2_TOP/BOT specified region (IOMUX_GPR_GPR22 - IOMUX_GPR_GPR23)"]
    M7_APC_AC_R2_CTRL_1 = 1,
}
impl From<M7_APC_AC_R2_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R2_CTRL_A) -> Self {
        variant as _
    }
}
impl M7_APC_AC_R2_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M7_APC_AC_R2_CTRL_A> {
        match self.bits {
            0 => Some(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0),
            1 => Some(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1
    }
}
#[doc = "Field `M7_APC_AC_R2_CTRL` writer - Access control of memory region-2"]
pub type M7_APC_AC_R2_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, M7_APC_AC_R2_CTRL_A, 2, O>;
impl<'a, const O: u8> M7_APC_AC_R2_CTRL_W<'a, O> {
    #[doc = "No access protection - All accesses are allowed"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0)
    }
    #[doc = "M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R2_TOP/BOT specified region (IOMUX_GPR_GPR22 - IOMUX_GPR_GPR23)"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1)
    }
}
#[doc = "Field `M7_APC_AC_R3_CTRL` reader - Access control of memory region-3"]
pub type M7_APC_AC_R3_CTRL_R = crate::FieldReader<u8, M7_APC_AC_R3_CTRL_A>;
#[doc = "Access control of memory region-3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M7_APC_AC_R3_CTRL_A {
    #[doc = "0: No access protection - All accesses are allowed"]
    M7_APC_AC_R3_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R3_TOP/BOT specified region (IOMUX_GPR_GPR24 - IOMUX_GPR_GPR25)"]
    M7_APC_AC_R3_CTRL_1 = 1,
}
impl From<M7_APC_AC_R3_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R3_CTRL_A) -> Self {
        variant as _
    }
}
impl M7_APC_AC_R3_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M7_APC_AC_R3_CTRL_A> {
        match self.bits {
            0 => Some(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0),
            1 => Some(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1
    }
}
#[doc = "Field `M7_APC_AC_R3_CTRL` writer - Access control of memory region-3"]
pub type M7_APC_AC_R3_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, M7_APC_AC_R3_CTRL_A, 2, O>;
impl<'a, const O: u8> M7_APC_AC_R3_CTRL_W<'a, O> {
    #[doc = "No access protection - All accesses are allowed"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0)
    }
    #[doc = "M7 debug protection enabled - The APC block will block CM7 breakpoints, watchpoints and trace to the GPR_M7_APC_AC_R3_TOP/BOT specified region (IOMUX_GPR_GPR24 - IOMUX_GPR_GPR25)"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1)
    }
}
#[doc = "Field `BEE_DE_RX_EN` reader - BEE data decryption of memory region-n (n = 3 to 0)."]
pub type BEE_DE_RX_EN_R = crate::FieldReader<u8, BEE_DE_RX_EN_A>;
#[doc = "BEE data decryption of memory region-n (n = 3 to 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BEE_DE_RX_EN_A {
    #[doc = "0: FlexSPI data decryption disabled"]
    BEE_DE_RX_EN_0 = 0,
    #[doc = "1: FlexSPI data decryption enabled"]
    BEE_DE_RX_EN_1 = 1,
}
impl From<BEE_DE_RX_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: BEE_DE_RX_EN_A) -> Self {
        variant as _
    }
}
impl BEE_DE_RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BEE_DE_RX_EN_A> {
        match self.bits {
            0 => Some(BEE_DE_RX_EN_A::BEE_DE_RX_EN_0),
            1 => Some(BEE_DE_RX_EN_A::BEE_DE_RX_EN_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BEE_DE_RX_EN_0`"]
    #[inline(always)]
    pub fn is_bee_de_rx_en_0(&self) -> bool {
        *self == BEE_DE_RX_EN_A::BEE_DE_RX_EN_0
    }
    #[doc = "Checks if the value of the field is `BEE_DE_RX_EN_1`"]
    #[inline(always)]
    pub fn is_bee_de_rx_en_1(&self) -> bool {
        *self == BEE_DE_RX_EN_A::BEE_DE_RX_EN_1
    }
}
#[doc = "Field `BEE_DE_RX_EN` writer - BEE data decryption of memory region-n (n = 3 to 0)."]
pub type BEE_DE_RX_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, BEE_DE_RX_EN_A, 4, O>;
impl<'a, const O: u8> BEE_DE_RX_EN_W<'a, O> {
    #[doc = "FlexSPI data decryption disabled"]
    #[inline(always)]
    pub fn bee_de_rx_en_0(self) -> &'a mut W {
        self.variant(BEE_DE_RX_EN_A::BEE_DE_RX_EN_0)
    }
    #[doc = "FlexSPI data decryption enabled"]
    #[inline(always)]
    pub fn bee_de_rx_en_1(self) -> &'a mut W {
        self.variant(BEE_DE_RX_EN_A::BEE_DE_RX_EN_1)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R0_CTRL` reader - Lock M7_APC_AC_R0_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R0_CTRL_R = crate::FieldReader<u8, LOCK_M7_APC_AC_R0_CTRL_A>;
#[doc = "Lock M7_APC_AC_R0_CTRL field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_M7_APC_AC_R0_CTRL_A {
    #[doc = "0: Field is not locked"]
    LOCK_M7_APC_AC_R0_CTRL_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_M7_APC_AC_R0_CTRL_1 = 1,
}
impl From<LOCK_M7_APC_AC_R0_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R0_CTRL_A) -> Self {
        variant as _
    }
}
impl LOCK_M7_APC_AC_R0_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_M7_APC_AC_R0_CTRL_A> {
        match self.bits {
            0 => Some(LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_0),
            1 => Some(LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R0_CTRL_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r0_ctrl_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R0_CTRL_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r0_ctrl_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R0_CTRL` writer - Lock M7_APC_AC_R0_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R0_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, LOCK_M7_APC_AC_R0_CTRL_A, 2, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R0_CTRL_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_ctrl_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_ctrl_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R0_CTRL_A::LOCK_M7_APC_AC_R0_CTRL_1)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R1_CTRL` reader - Lock M7_APC_AC_R1_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R1_CTRL_R = crate::FieldReader<u8, LOCK_M7_APC_AC_R1_CTRL_A>;
#[doc = "Lock M7_APC_AC_R1_CTRL field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_M7_APC_AC_R1_CTRL_A {
    #[doc = "0: Field is not locked"]
    LOCK_M7_APC_AC_R1_CTRL_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_M7_APC_AC_R1_CTRL_1 = 1,
}
impl From<LOCK_M7_APC_AC_R1_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R1_CTRL_A) -> Self {
        variant as _
    }
}
impl LOCK_M7_APC_AC_R1_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_M7_APC_AC_R1_CTRL_A> {
        match self.bits {
            0 => Some(LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_0),
            1 => Some(LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R1_CTRL_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r1_ctrl_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R1_CTRL_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r1_ctrl_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R1_CTRL` writer - Lock M7_APC_AC_R1_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R1_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, LOCK_M7_APC_AC_R1_CTRL_A, 2, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R1_CTRL_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_ctrl_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_ctrl_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R1_CTRL_A::LOCK_M7_APC_AC_R1_CTRL_1)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R2_CTRL` reader - Lock M7_APC_AC_R2_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R2_CTRL_R = crate::FieldReader<u8, LOCK_M7_APC_AC_R2_CTRL_A>;
#[doc = "Lock M7_APC_AC_R2_CTRL field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_M7_APC_AC_R2_CTRL_A {
    #[doc = "0: Field is not locked"]
    LOCK_M7_APC_AC_R2_CTRL_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_M7_APC_AC_R2_CTRL_1 = 1,
}
impl From<LOCK_M7_APC_AC_R2_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R2_CTRL_A) -> Self {
        variant as _
    }
}
impl LOCK_M7_APC_AC_R2_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_M7_APC_AC_R2_CTRL_A> {
        match self.bits {
            0 => Some(LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_0),
            1 => Some(LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R2_CTRL_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r2_ctrl_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R2_CTRL_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r2_ctrl_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R2_CTRL` writer - Lock M7_APC_AC_R2_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R2_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, LOCK_M7_APC_AC_R2_CTRL_A, 2, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R2_CTRL_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_ctrl_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_ctrl_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R2_CTRL_A::LOCK_M7_APC_AC_R2_CTRL_1)
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R3_CTRL` reader - Lock M7_APC_AC_R3_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R3_CTRL_R = crate::FieldReader<u8, LOCK_M7_APC_AC_R3_CTRL_A>;
#[doc = "Lock M7_APC_AC_R3_CTRL field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_M7_APC_AC_R3_CTRL_A {
    #[doc = "0: Field is not locked"]
    LOCK_M7_APC_AC_R3_CTRL_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_M7_APC_AC_R3_CTRL_1 = 1,
}
impl From<LOCK_M7_APC_AC_R3_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R3_CTRL_A) -> Self {
        variant as _
    }
}
impl LOCK_M7_APC_AC_R3_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_M7_APC_AC_R3_CTRL_A> {
        match self.bits {
            0 => Some(LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_0),
            1 => Some(LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_CTRL_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r3_ctrl_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_CTRL_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r3_ctrl_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_1
    }
}
#[doc = "Field `LOCK_M7_APC_AC_R3_CTRL` writer - Lock M7_APC_AC_R3_CTRL field for changes"]
pub type LOCK_M7_APC_AC_R3_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, LOCK_M7_APC_AC_R3_CTRL_A, 2, O>;
impl<'a, const O: u8> LOCK_M7_APC_AC_R3_CTRL_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_ctrl_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_ctrl_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_CTRL_A::LOCK_M7_APC_AC_R3_CTRL_1)
    }
}
#[doc = "Field `LOCK_BEE_DE_RX_EN` reader - Lock BEE_DE_RX_EN\\[n\\]
(n = 3 to 0) field for changes"]
pub type LOCK_BEE_DE_RX_EN_R = crate::FieldReader<u8, LOCK_BEE_DE_RX_EN_A>;
#[doc = "Lock BEE_DE_RX_EN\\[n\\]
(n = 3 to 0) field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_BEE_DE_RX_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_BEE_DE_RX_EN_0 = 0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_BEE_DE_RX_EN_1 = 1,
}
impl From<LOCK_BEE_DE_RX_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_BEE_DE_RX_EN_A) -> Self {
        variant as _
    }
}
impl LOCK_BEE_DE_RX_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_BEE_DE_RX_EN_A> {
        match self.bits {
            0 => Some(LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_0),
            1 => Some(LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_BEE_DE_RX_EN_0`"]
    #[inline(always)]
    pub fn is_lock_bee_de_rx_en_0(&self) -> bool {
        *self == LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_BEE_DE_RX_EN_1`"]
    #[inline(always)]
    pub fn is_lock_bee_de_rx_en_1(&self) -> bool {
        *self == LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_1
    }
}
#[doc = "Field `LOCK_BEE_DE_RX_EN` writer - Lock BEE_DE_RX_EN\\[n\\]
(n = 3 to 0) field for changes"]
pub type LOCK_BEE_DE_RX_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR11_SPEC, u8, LOCK_BEE_DE_RX_EN_A, 4, O>;
impl<'a, const O: u8> LOCK_BEE_DE_RX_EN_W<'a, O> {
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_bee_de_rx_en_0(self) -> &'a mut W {
        self.variant(LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_bee_de_rx_en_1(self) -> &'a mut W {
        self.variant(LOCK_BEE_DE_RX_EN_A::LOCK_BEE_DE_RX_EN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl(&self) -> M7_APC_AC_R0_CTRL_R {
        M7_APC_AC_R0_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl(&self) -> M7_APC_AC_R1_CTRL_R {
        M7_APC_AC_R1_CTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl(&self) -> M7_APC_AC_R2_CTRL_R {
        M7_APC_AC_R2_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl(&self) -> M7_APC_AC_R3_CTRL_R {
        M7_APC_AC_R3_CTRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)."]
    #[inline(always)]
    pub fn bee_de_rx_en(&self) -> BEE_DE_RX_EN_R {
        BEE_DE_RX_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r0_ctrl(&self) -> LOCK_M7_APC_AC_R0_CTRL_R {
        LOCK_M7_APC_AC_R0_CTRL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r1_ctrl(&self) -> LOCK_M7_APC_AC_R1_CTRL_R {
        LOCK_M7_APC_AC_R1_CTRL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_ctrl(&self) -> LOCK_M7_APC_AC_R2_CTRL_R {
        LOCK_M7_APC_AC_R2_CTRL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_ctrl(&self) -> LOCK_M7_APC_AC_R3_CTRL_R {
        LOCK_M7_APC_AC_R3_CTRL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Lock BEE_DE_RX_EN\\[n\\]
(n = 3 to 0) field for changes"]
    #[inline(always)]
    pub fn lock_bee_de_rx_en(&self) -> LOCK_BEE_DE_RX_EN_R {
        LOCK_BEE_DE_RX_EN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r0_ctrl(&mut self) -> M7_APC_AC_R0_CTRL_W<0> {
        M7_APC_AC_R0_CTRL_W::new(self)
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r1_ctrl(&mut self) -> M7_APC_AC_R1_CTRL_W<2> {
        M7_APC_AC_R1_CTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r2_ctrl(&mut self) -> M7_APC_AC_R2_CTRL_W<4> {
        M7_APC_AC_R2_CTRL_W::new(self)
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline(always)]
    #[must_use]
    pub fn m7_apc_ac_r3_ctrl(&mut self) -> M7_APC_AC_R3_CTRL_W<6> {
        M7_APC_AC_R3_CTRL_W::new(self)
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)."]
    #[inline(always)]
    #[must_use]
    pub fn bee_de_rx_en(&mut self) -> BEE_DE_RX_EN_W<8> {
        BEE_DE_RX_EN_W::new(self)
    }
    #[doc = "Bits 16:17 - Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r0_ctrl(&mut self) -> LOCK_M7_APC_AC_R0_CTRL_W<16> {
        LOCK_M7_APC_AC_R0_CTRL_W::new(self)
    }
    #[doc = "Bits 18:19 - Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r1_ctrl(&mut self) -> LOCK_M7_APC_AC_R1_CTRL_W<18> {
        LOCK_M7_APC_AC_R1_CTRL_W::new(self)
    }
    #[doc = "Bits 20:21 - Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r2_ctrl(&mut self) -> LOCK_M7_APC_AC_R2_CTRL_W<20> {
        LOCK_M7_APC_AC_R2_CTRL_W::new(self)
    }
    #[doc = "Bits 22:23 - Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_m7_apc_ac_r3_ctrl(&mut self) -> LOCK_M7_APC_AC_R3_CTRL_W<22> {
        LOCK_M7_APC_AC_R3_CTRL_W::new(self)
    }
    #[doc = "Bits 24:27 - Lock BEE_DE_RX_EN\\[n\\]
(n = 3 to 0) field for changes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_bee_de_rx_en(&mut self) -> LOCK_BEE_DE_RX_EN_W<24> {
        LOCK_BEE_DE_RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR11 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr11](index.html) module"]
pub struct GPR11_SPEC;
impl crate::RegisterSpec for GPR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr11::R](R) reader structure"]
impl crate::Readable for GPR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr11::W](W) writer structure"]
impl crate::Writable for GPR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR11 to value 0"]
impl crate::Resettable for GPR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
