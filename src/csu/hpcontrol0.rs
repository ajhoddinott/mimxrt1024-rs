#[doc = "Register `HPCONTROL0` reader"]
pub struct R(crate::R<HPCONTROL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPCONTROL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPCONTROL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPCONTROL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPCONTROL0` writer"]
pub struct W(crate::W<HPCONTROL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPCONTROL0_SPEC>;
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
impl From<crate::W<HPCONTROL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPCONTROL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPC_DMA` reader - Indicates the privilege/user mode for the eDMA"]
pub type HPC_DMA_R = crate::BitReader<HPC_DMA_A>;
#[doc = "Indicates the privilege/user mode for the eDMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_DMA_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_DMA_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_DMA_1 = 1,
}
impl From<HPC_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_DMA_A {
        match self.bits {
            false => HPC_DMA_A::HPC_DMA_0,
            true => HPC_DMA_A::HPC_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_DMA_0`"]
    #[inline(always)]
    pub fn is_hpc_dma_0(&self) -> bool {
        *self == HPC_DMA_A::HPC_DMA_0
    }
    #[doc = "Checks if the value of the field is `HPC_DMA_1`"]
    #[inline(always)]
    pub fn is_hpc_dma_1(&self) -> bool {
        *self == HPC_DMA_A::HPC_DMA_1
    }
}
#[doc = "Field `HPC_DMA` writer - Indicates the privilege/user mode for the eDMA"]
pub type HPC_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_DMA_A, O>;
impl<'a, const O: u8> HPC_DMA_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_dma_0(self) -> &'a mut W {
        self.variant(HPC_DMA_A::HPC_DMA_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_dma_1(self) -> &'a mut W {
        self.variant(HPC_DMA_A::HPC_DMA_1)
    }
}
#[doc = "Field `L_DMA` reader - Lock bit set by the TZ software for the eDMA"]
pub type L_DMA_R = crate::BitReader<L_DMA_A>;
#[doc = "Lock bit set by the TZ software for the eDMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_DMA_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1 = 1,
}
impl From<L_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: L_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl L_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_DMA_A {
        match self.bits {
            false => L_DMA_A::L_DMA_0,
            true => L_DMA_A::L_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DMA_0`"]
    #[inline(always)]
    pub fn is_l_dma_0(&self) -> bool {
        *self == L_DMA_A::L_DMA_0
    }
    #[doc = "Checks if the value of the field is `L_DMA_1`"]
    #[inline(always)]
    pub fn is_l_dma_1(&self) -> bool {
        *self == L_DMA_A::L_DMA_1
    }
}
#[doc = "Field `L_DMA` writer - Lock bit set by the TZ software for the eDMA"]
pub type L_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_DMA_A, O>;
impl<'a, const O: u8> L_DMA_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_dma_0(self) -> &'a mut W {
        self.variant(L_DMA_A::L_DMA_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_dma_1(self) -> &'a mut W {
        self.variant(L_DMA_A::L_DMA_1)
    }
}
#[doc = "Field `HPC_LCDIF` reader - Indicates the privilege/user mode for the LCDIF"]
pub type HPC_LCDIF_R = crate::BitReader<HPC_LCDIF_A>;
#[doc = "Indicates the privilege/user mode for the LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_LCDIF_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_LCDIF_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_LCDIF_1 = 1,
}
impl From<HPC_LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_LCDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_LCDIF_A {
        match self.bits {
            false => HPC_LCDIF_A::HPC_LCDIF_0,
            true => HPC_LCDIF_A::HPC_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_LCDIF_0`"]
    #[inline(always)]
    pub fn is_hpc_lcdif_0(&self) -> bool {
        *self == HPC_LCDIF_A::HPC_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `HPC_LCDIF_1`"]
    #[inline(always)]
    pub fn is_hpc_lcdif_1(&self) -> bool {
        *self == HPC_LCDIF_A::HPC_LCDIF_1
    }
}
#[doc = "Field `HPC_LCDIF` writer - Indicates the privilege/user mode for the LCDIF"]
pub type HPC_LCDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_LCDIF_A, O>;
impl<'a, const O: u8> HPC_LCDIF_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_lcdif_0(self) -> &'a mut W {
        self.variant(HPC_LCDIF_A::HPC_LCDIF_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_lcdif_1(self) -> &'a mut W {
        self.variant(HPC_LCDIF_A::HPC_LCDIF_1)
    }
}
#[doc = "Field `L_LCDIF` reader - Lock bit set by the TZ software for the LCDIF"]
pub type L_LCDIF_R = crate::BitReader<L_LCDIF_A>;
#[doc = "Lock bit set by the TZ software for the LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_LCDIF_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1 = 1,
}
impl From<L_LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: L_LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl L_LCDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_LCDIF_A {
        match self.bits {
            false => L_LCDIF_A::L_LCDIF_0,
            true => L_LCDIF_A::L_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_0`"]
    #[inline(always)]
    pub fn is_l_lcdif_0(&self) -> bool {
        *self == L_LCDIF_A::L_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_1`"]
    #[inline(always)]
    pub fn is_l_lcdif_1(&self) -> bool {
        *self == L_LCDIF_A::L_LCDIF_1
    }
}
#[doc = "Field `L_LCDIF` writer - Lock bit set by the TZ software for the LCDIF"]
pub type L_LCDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_LCDIF_A, O>;
impl<'a, const O: u8> L_LCDIF_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_lcdif_0(self) -> &'a mut W {
        self.variant(L_LCDIF_A::L_LCDIF_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_lcdif_1(self) -> &'a mut W {
        self.variant(L_LCDIF_A::L_LCDIF_1)
    }
}
#[doc = "Field `HPC_CSI` reader - Indicates the privilege/user mode for the CSI"]
pub type HPC_CSI_R = crate::BitReader<HPC_CSI_A>;
#[doc = "Indicates the privilege/user mode for the CSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_CSI_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_CSI_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_CSI_1 = 1,
}
impl From<HPC_CSI_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_CSI_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_CSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_CSI_A {
        match self.bits {
            false => HPC_CSI_A::HPC_CSI_0,
            true => HPC_CSI_A::HPC_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_CSI_0`"]
    #[inline(always)]
    pub fn is_hpc_csi_0(&self) -> bool {
        *self == HPC_CSI_A::HPC_CSI_0
    }
    #[doc = "Checks if the value of the field is `HPC_CSI_1`"]
    #[inline(always)]
    pub fn is_hpc_csi_1(&self) -> bool {
        *self == HPC_CSI_A::HPC_CSI_1
    }
}
#[doc = "Field `HPC_CSI` writer - Indicates the privilege/user mode for the CSI"]
pub type HPC_CSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_CSI_A, O>;
impl<'a, const O: u8> HPC_CSI_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_csi_0(self) -> &'a mut W {
        self.variant(HPC_CSI_A::HPC_CSI_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_csi_1(self) -> &'a mut W {
        self.variant(HPC_CSI_A::HPC_CSI_1)
    }
}
#[doc = "Field `L_CSI` reader - Lock bit set by the TZ software for the CSI"]
pub type L_CSI_R = crate::BitReader<L_CSI_A>;
#[doc = "Lock bit set by the TZ software for the CSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_CSI_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1 = 1,
}
impl From<L_CSI_A> for bool {
    #[inline(always)]
    fn from(variant: L_CSI_A) -> Self {
        variant as u8 != 0
    }
}
impl L_CSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_CSI_A {
        match self.bits {
            false => L_CSI_A::L_CSI_0,
            true => L_CSI_A::L_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_CSI_0`"]
    #[inline(always)]
    pub fn is_l_csi_0(&self) -> bool {
        *self == L_CSI_A::L_CSI_0
    }
    #[doc = "Checks if the value of the field is `L_CSI_1`"]
    #[inline(always)]
    pub fn is_l_csi_1(&self) -> bool {
        *self == L_CSI_A::L_CSI_1
    }
}
#[doc = "Field `L_CSI` writer - Lock bit set by the TZ software for the CSI"]
pub type L_CSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_CSI_A, O>;
impl<'a, const O: u8> L_CSI_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_csi_0(self) -> &'a mut W {
        self.variant(L_CSI_A::L_CSI_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_csi_1(self) -> &'a mut W {
        self.variant(L_CSI_A::L_CSI_1)
    }
}
#[doc = "Field `HPC_PXP` reader - Indicates the privilege/user mode for the PXP"]
pub type HPC_PXP_R = crate::BitReader<HPC_PXP_A>;
#[doc = "Indicates the privilege/user mode for the PXP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_PXP_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_PXP_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_PXP_1 = 1,
}
impl From<HPC_PXP_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_PXP_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_PXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_PXP_A {
        match self.bits {
            false => HPC_PXP_A::HPC_PXP_0,
            true => HPC_PXP_A::HPC_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_PXP_0`"]
    #[inline(always)]
    pub fn is_hpc_pxp_0(&self) -> bool {
        *self == HPC_PXP_A::HPC_PXP_0
    }
    #[doc = "Checks if the value of the field is `HPC_PXP_1`"]
    #[inline(always)]
    pub fn is_hpc_pxp_1(&self) -> bool {
        *self == HPC_PXP_A::HPC_PXP_1
    }
}
#[doc = "Field `HPC_PXP` writer - Indicates the privilege/user mode for the PXP"]
pub type HPC_PXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_PXP_A, O>;
impl<'a, const O: u8> HPC_PXP_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_pxp_0(self) -> &'a mut W {
        self.variant(HPC_PXP_A::HPC_PXP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_pxp_1(self) -> &'a mut W {
        self.variant(HPC_PXP_A::HPC_PXP_1)
    }
}
#[doc = "Field `L_PXP` reader - Lock bit set by the TZ software for the PXP"]
pub type L_PXP_R = crate::BitReader<L_PXP_A>;
#[doc = "Lock bit set by the TZ software for the PXP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_PXP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1 = 1,
}
impl From<L_PXP_A> for bool {
    #[inline(always)]
    fn from(variant: L_PXP_A) -> Self {
        variant as u8 != 0
    }
}
impl L_PXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_PXP_A {
        match self.bits {
            false => L_PXP_A::L_PXP_0,
            true => L_PXP_A::L_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_PXP_0`"]
    #[inline(always)]
    pub fn is_l_pxp_0(&self) -> bool {
        *self == L_PXP_A::L_PXP_0
    }
    #[doc = "Checks if the value of the field is `L_PXP_1`"]
    #[inline(always)]
    pub fn is_l_pxp_1(&self) -> bool {
        *self == L_PXP_A::L_PXP_1
    }
}
#[doc = "Field `L_PXP` writer - Lock bit set by the TZ software for the PXP"]
pub type L_PXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_PXP_A, O>;
impl<'a, const O: u8> L_PXP_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_pxp_0(self) -> &'a mut W {
        self.variant(L_PXP_A::L_PXP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_pxp_1(self) -> &'a mut W {
        self.variant(L_PXP_A::L_PXP_1)
    }
}
#[doc = "Field `HPC_DCP` reader - Indicates the privilege/user mode for the DCP"]
pub type HPC_DCP_R = crate::BitReader<HPC_DCP_A>;
#[doc = "Indicates the privilege/user mode for the DCP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_DCP_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_DCP_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_DCP_1 = 1,
}
impl From<HPC_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_DCP_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_DCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_DCP_A {
        match self.bits {
            false => HPC_DCP_A::HPC_DCP_0,
            true => HPC_DCP_A::HPC_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_DCP_0`"]
    #[inline(always)]
    pub fn is_hpc_dcp_0(&self) -> bool {
        *self == HPC_DCP_A::HPC_DCP_0
    }
    #[doc = "Checks if the value of the field is `HPC_DCP_1`"]
    #[inline(always)]
    pub fn is_hpc_dcp_1(&self) -> bool {
        *self == HPC_DCP_A::HPC_DCP_1
    }
}
#[doc = "Field `HPC_DCP` writer - Indicates the privilege/user mode for the DCP"]
pub type HPC_DCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_DCP_A, O>;
impl<'a, const O: u8> HPC_DCP_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_dcp_0(self) -> &'a mut W {
        self.variant(HPC_DCP_A::HPC_DCP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_dcp_1(self) -> &'a mut W {
        self.variant(HPC_DCP_A::HPC_DCP_1)
    }
}
#[doc = "Field `L_DCP` reader - Lock bit set by the TZ software for the DCP"]
pub type L_DCP_R = crate::BitReader<L_DCP_A>;
#[doc = "Lock bit set by the TZ software for the DCP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_DCP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1 = 1,
}
impl From<L_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: L_DCP_A) -> Self {
        variant as u8 != 0
    }
}
impl L_DCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_DCP_A {
        match self.bits {
            false => L_DCP_A::L_DCP_0,
            true => L_DCP_A::L_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DCP_0`"]
    #[inline(always)]
    pub fn is_l_dcp_0(&self) -> bool {
        *self == L_DCP_A::L_DCP_0
    }
    #[doc = "Checks if the value of the field is `L_DCP_1`"]
    #[inline(always)]
    pub fn is_l_dcp_1(&self) -> bool {
        *self == L_DCP_A::L_DCP_1
    }
}
#[doc = "Field `L_DCP` writer - Lock bit set by the TZ software for the DCP"]
pub type L_DCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_DCP_A, O>;
impl<'a, const O: u8> L_DCP_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_dcp_0(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_dcp_1(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_1)
    }
}
#[doc = "Field `HPC_ENET` reader - Indicates the privilege/user mode for the ENET"]
pub type HPC_ENET_R = crate::BitReader<HPC_ENET_A>;
#[doc = "Indicates the privilege/user mode for the ENET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_ENET_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_ENET_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_ENET_1 = 1,
}
impl From<HPC_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_ENET_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_ENET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_ENET_A {
        match self.bits {
            false => HPC_ENET_A::HPC_ENET_0,
            true => HPC_ENET_A::HPC_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_ENET_0`"]
    #[inline(always)]
    pub fn is_hpc_enet_0(&self) -> bool {
        *self == HPC_ENET_A::HPC_ENET_0
    }
    #[doc = "Checks if the value of the field is `HPC_ENET_1`"]
    #[inline(always)]
    pub fn is_hpc_enet_1(&self) -> bool {
        *self == HPC_ENET_A::HPC_ENET_1
    }
}
#[doc = "Field `HPC_ENET` writer - Indicates the privilege/user mode for the ENET"]
pub type HPC_ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_ENET_A, O>;
impl<'a, const O: u8> HPC_ENET_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_enet_0(self) -> &'a mut W {
        self.variant(HPC_ENET_A::HPC_ENET_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_enet_1(self) -> &'a mut W {
        self.variant(HPC_ENET_A::HPC_ENET_1)
    }
}
#[doc = "Field `L_ENET` reader - Lock bit set by the TZ software for the ENET"]
pub type L_ENET_R = crate::BitReader<L_ENET_A>;
#[doc = "Lock bit set by the TZ software for the ENET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_ENET_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1 = 1,
}
impl From<L_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: L_ENET_A) -> Self {
        variant as u8 != 0
    }
}
impl L_ENET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_ENET_A {
        match self.bits {
            false => L_ENET_A::L_ENET_0,
            true => L_ENET_A::L_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_ENET_0`"]
    #[inline(always)]
    pub fn is_l_enet_0(&self) -> bool {
        *self == L_ENET_A::L_ENET_0
    }
    #[doc = "Checks if the value of the field is `L_ENET_1`"]
    #[inline(always)]
    pub fn is_l_enet_1(&self) -> bool {
        *self == L_ENET_A::L_ENET_1
    }
}
#[doc = "Field `L_ENET` writer - Lock bit set by the TZ software for the ENET"]
pub type L_ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_ENET_A, O>;
impl<'a, const O: u8> L_ENET_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_enet_0(self) -> &'a mut W {
        self.variant(L_ENET_A::L_ENET_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_enet_1(self) -> &'a mut W {
        self.variant(L_ENET_A::L_ENET_1)
    }
}
#[doc = "Field `HPC_USDHC1` reader - Indicates the privilege/user mode for the USDHC1"]
pub type HPC_USDHC1_R = crate::BitReader<HPC_USDHC1_A>;
#[doc = "Indicates the privilege/user mode for the USDHC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_USDHC1_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_USDHC1_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_USDHC1_1 = 1,
}
impl From<HPC_USDHC1_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_USDHC1_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_USDHC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_USDHC1_A {
        match self.bits {
            false => HPC_USDHC1_A::HPC_USDHC1_0,
            true => HPC_USDHC1_A::HPC_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC1_0`"]
    #[inline(always)]
    pub fn is_hpc_usdhc1_0(&self) -> bool {
        *self == HPC_USDHC1_A::HPC_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC1_1`"]
    #[inline(always)]
    pub fn is_hpc_usdhc1_1(&self) -> bool {
        *self == HPC_USDHC1_A::HPC_USDHC1_1
    }
}
#[doc = "Field `HPC_USDHC1` writer - Indicates the privilege/user mode for the USDHC1"]
pub type HPC_USDHC1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_USDHC1_A, O>;
impl<'a, const O: u8> HPC_USDHC1_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usdhc1_0(self) -> &'a mut W {
        self.variant(HPC_USDHC1_A::HPC_USDHC1_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usdhc1_1(self) -> &'a mut W {
        self.variant(HPC_USDHC1_A::HPC_USDHC1_1)
    }
}
#[doc = "Field `L_USDHC1` reader - Lock bit set by the TZ software for the USDHC1"]
pub type L_USDHC1_R = crate::BitReader<L_USDHC1_A>;
#[doc = "Lock bit set by the TZ software for the USDHC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_USDHC1_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1 = 1,
}
impl From<L_USDHC1_A> for bool {
    #[inline(always)]
    fn from(variant: L_USDHC1_A) -> Self {
        variant as u8 != 0
    }
}
impl L_USDHC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USDHC1_A {
        match self.bits {
            false => L_USDHC1_A::L_USDHC1_0,
            true => L_USDHC1_A::L_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_0`"]
    #[inline(always)]
    pub fn is_l_usdhc1_0(&self) -> bool {
        *self == L_USDHC1_A::L_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_1`"]
    #[inline(always)]
    pub fn is_l_usdhc1_1(&self) -> bool {
        *self == L_USDHC1_A::L_USDHC1_1
    }
}
#[doc = "Field `L_USDHC1` writer - Lock bit set by the TZ software for the USDHC1"]
pub type L_USDHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_USDHC1_A, O>;
impl<'a, const O: u8> L_USDHC1_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usdhc1_0(self) -> &'a mut W {
        self.variant(L_USDHC1_A::L_USDHC1_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usdhc1_1(self) -> &'a mut W {
        self.variant(L_USDHC1_A::L_USDHC1_1)
    }
}
#[doc = "Field `HPC_USDHC2` reader - Indicates the privilege/user mode for the USDHC2"]
pub type HPC_USDHC2_R = crate::BitReader<HPC_USDHC2_A>;
#[doc = "Indicates the privilege/user mode for the USDHC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_USDHC2_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_USDHC2_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_USDHC2_1 = 1,
}
impl From<HPC_USDHC2_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_USDHC2_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_USDHC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_USDHC2_A {
        match self.bits {
            false => HPC_USDHC2_A::HPC_USDHC2_0,
            true => HPC_USDHC2_A::HPC_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC2_0`"]
    #[inline(always)]
    pub fn is_hpc_usdhc2_0(&self) -> bool {
        *self == HPC_USDHC2_A::HPC_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `HPC_USDHC2_1`"]
    #[inline(always)]
    pub fn is_hpc_usdhc2_1(&self) -> bool {
        *self == HPC_USDHC2_A::HPC_USDHC2_1
    }
}
#[doc = "Field `HPC_USDHC2` writer - Indicates the privilege/user mode for the USDHC2"]
pub type HPC_USDHC2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_USDHC2_A, O>;
impl<'a, const O: u8> HPC_USDHC2_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usdhc2_0(self) -> &'a mut W {
        self.variant(HPC_USDHC2_A::HPC_USDHC2_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usdhc2_1(self) -> &'a mut W {
        self.variant(HPC_USDHC2_A::HPC_USDHC2_1)
    }
}
#[doc = "Field `L_USDHC2` reader - Lock bit set by the TZ software for the USDHC2."]
pub type L_USDHC2_R = crate::BitReader<L_USDHC2_A>;
#[doc = "Lock bit set by the TZ software for the USDHC2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_USDHC2_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1 = 1,
}
impl From<L_USDHC2_A> for bool {
    #[inline(always)]
    fn from(variant: L_USDHC2_A) -> Self {
        variant as u8 != 0
    }
}
impl L_USDHC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USDHC2_A {
        match self.bits {
            false => L_USDHC2_A::L_USDHC2_0,
            true => L_USDHC2_A::L_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_0`"]
    #[inline(always)]
    pub fn is_l_usdhc2_0(&self) -> bool {
        *self == L_USDHC2_A::L_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_1`"]
    #[inline(always)]
    pub fn is_l_usdhc2_1(&self) -> bool {
        *self == L_USDHC2_A::L_USDHC2_1
    }
}
#[doc = "Field `L_USDHC2` writer - Lock bit set by the TZ software for the USDHC2."]
pub type L_USDHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_USDHC2_A, O>;
impl<'a, const O: u8> L_USDHC2_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usdhc2_0(self) -> &'a mut W {
        self.variant(L_USDHC2_A::L_USDHC2_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usdhc2_1(self) -> &'a mut W {
        self.variant(L_USDHC2_A::L_USDHC2_1)
    }
}
#[doc = "Field `HPC_TPSMP` reader - Indicates the privilege/user mode for the TPSMP"]
pub type HPC_TPSMP_R = crate::BitReader<HPC_TPSMP_A>;
#[doc = "Indicates the privilege/user mode for the TPSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_TPSMP_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_TPSMP_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_TPSMP_1 = 1,
}
impl From<HPC_TPSMP_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_TPSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_TPSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_TPSMP_A {
        match self.bits {
            false => HPC_TPSMP_A::HPC_TPSMP_0,
            true => HPC_TPSMP_A::HPC_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_TPSMP_0`"]
    #[inline(always)]
    pub fn is_hpc_tpsmp_0(&self) -> bool {
        *self == HPC_TPSMP_A::HPC_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `HPC_TPSMP_1`"]
    #[inline(always)]
    pub fn is_hpc_tpsmp_1(&self) -> bool {
        *self == HPC_TPSMP_A::HPC_TPSMP_1
    }
}
#[doc = "Field `HPC_TPSMP` writer - Indicates the privilege/user mode for the TPSMP"]
pub type HPC_TPSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_TPSMP_A, O>;
impl<'a, const O: u8> HPC_TPSMP_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_tpsmp_0(self) -> &'a mut W {
        self.variant(HPC_TPSMP_A::HPC_TPSMP_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_tpsmp_1(self) -> &'a mut W {
        self.variant(HPC_TPSMP_A::HPC_TPSMP_1)
    }
}
#[doc = "Field `L_TPSMP` reader - Lock bit set by the TZ software for the TPSMP."]
pub type L_TPSMP_R = crate::BitReader<L_TPSMP_A>;
#[doc = "Lock bit set by the TZ software for the TPSMP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_TPSMP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1 = 1,
}
impl From<L_TPSMP_A> for bool {
    #[inline(always)]
    fn from(variant: L_TPSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl L_TPSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_TPSMP_A {
        match self.bits {
            false => L_TPSMP_A::L_TPSMP_0,
            true => L_TPSMP_A::L_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_0`"]
    #[inline(always)]
    pub fn is_l_tpsmp_0(&self) -> bool {
        *self == L_TPSMP_A::L_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_1`"]
    #[inline(always)]
    pub fn is_l_tpsmp_1(&self) -> bool {
        *self == L_TPSMP_A::L_TPSMP_1
    }
}
#[doc = "Field `L_TPSMP` writer - Lock bit set by the TZ software for the TPSMP."]
pub type L_TPSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_TPSMP_A, O>;
impl<'a, const O: u8> L_TPSMP_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_tpsmp_0(self) -> &'a mut W {
        self.variant(L_TPSMP_A::L_TPSMP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_tpsmp_1(self) -> &'a mut W {
        self.variant(L_TPSMP_A::L_TPSMP_1)
    }
}
#[doc = "Field `HPC_USB` reader - Indicates the privilege/user mode for the USB"]
pub type HPC_USB_R = crate::BitReader<HPC_USB_A>;
#[doc = "Indicates the privilege/user mode for the USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPC_USB_A {
    #[doc = "0: User mode for the corresponding master"]
    HPC_USB_0 = 0,
    #[doc = "1: Supervisor mode for the corresponding master"]
    HPC_USB_1 = 1,
}
impl From<HPC_USB_A> for bool {
    #[inline(always)]
    fn from(variant: HPC_USB_A) -> Self {
        variant as u8 != 0
    }
}
impl HPC_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPC_USB_A {
        match self.bits {
            false => HPC_USB_A::HPC_USB_0,
            true => HPC_USB_A::HPC_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPC_USB_0`"]
    #[inline(always)]
    pub fn is_hpc_usb_0(&self) -> bool {
        *self == HPC_USB_A::HPC_USB_0
    }
    #[doc = "Checks if the value of the field is `HPC_USB_1`"]
    #[inline(always)]
    pub fn is_hpc_usb_1(&self) -> bool {
        *self == HPC_USB_A::HPC_USB_1
    }
}
#[doc = "Field `HPC_USB` writer - Indicates the privilege/user mode for the USB"]
pub type HPC_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, HPC_USB_A, O>;
impl<'a, const O: u8> HPC_USB_W<'a, O> {
    #[doc = "User mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usb_0(self) -> &'a mut W {
        self.variant(HPC_USB_A::HPC_USB_0)
    }
    #[doc = "Supervisor mode for the corresponding master"]
    #[inline(always)]
    pub fn hpc_usb_1(self) -> &'a mut W {
        self.variant(HPC_USB_A::HPC_USB_1)
    }
}
#[doc = "Field `L_USB` reader - Lock bit set by the TZ software for the USB."]
pub type L_USB_R = crate::BitReader<L_USB_A>;
#[doc = "Lock bit set by the TZ software for the USB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_USB_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1 = 1,
}
impl From<L_USB_A> for bool {
    #[inline(always)]
    fn from(variant: L_USB_A) -> Self {
        variant as u8 != 0
    }
}
impl L_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USB_A {
        match self.bits {
            false => L_USB_A::L_USB_0,
            true => L_USB_A::L_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USB_0`"]
    #[inline(always)]
    pub fn is_l_usb_0(&self) -> bool {
        *self == L_USB_A::L_USB_0
    }
    #[doc = "Checks if the value of the field is `L_USB_1`"]
    #[inline(always)]
    pub fn is_l_usb_1(&self) -> bool {
        *self == L_USB_A::L_USB_1
    }
}
#[doc = "Field `L_USB` writer - Lock bit set by the TZ software for the USB."]
pub type L_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPCONTROL0_SPEC, L_USB_A, O>;
impl<'a, const O: u8> L_USB_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usb_0(self) -> &'a mut W {
        self.variant(L_USB_A::L_USB_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usb_1(self) -> &'a mut W {
        self.variant(L_USB_A::L_USB_1)
    }
}
impl R {
    #[doc = "Bit 2 - Indicates the privilege/user mode for the eDMA"]
    #[inline(always)]
    pub fn hpc_dma(&self) -> HPC_DMA_R {
        HPC_DMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub fn l_dma(&self) -> L_DMA_R {
        L_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the privilege/user mode for the LCDIF"]
    #[inline(always)]
    pub fn hpc_lcdif(&self) -> HPC_LCDIF_R {
        HPC_LCDIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub fn l_lcdif(&self) -> L_LCDIF_R {
        L_LCDIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the privilege/user mode for the CSI"]
    #[inline(always)]
    pub fn hpc_csi(&self) -> HPC_CSI_R {
        HPC_CSI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub fn l_csi(&self) -> L_CSI_R {
        L_CSI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the privilege/user mode for the PXP"]
    #[inline(always)]
    pub fn hpc_pxp(&self) -> HPC_PXP_R {
        HPC_PXP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub fn l_pxp(&self) -> L_PXP_R {
        L_PXP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates the privilege/user mode for the DCP"]
    #[inline(always)]
    pub fn hpc_dcp(&self) -> HPC_DCP_R {
        HPC_DCP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub fn l_dcp(&self) -> L_DCP_R {
        L_DCP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates the privilege/user mode for the ENET"]
    #[inline(always)]
    pub fn hpc_enet(&self) -> HPC_ENET_R {
        HPC_ENET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub fn l_enet(&self) -> L_ENET_R {
        L_ENET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates the privilege/user mode for the USDHC1"]
    #[inline(always)]
    pub fn hpc_usdhc1(&self) -> HPC_USDHC1_R {
        HPC_USDHC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub fn l_usdhc1(&self) -> L_USDHC1_R {
        L_USDHC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Indicates the privilege/user mode for the USDHC2"]
    #[inline(always)]
    pub fn hpc_usdhc2(&self) -> HPC_USDHC2_R {
        HPC_USDHC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2."]
    #[inline(always)]
    pub fn l_usdhc2(&self) -> L_USDHC2_R {
        L_USDHC2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates the privilege/user mode for the TPSMP"]
    #[inline(always)]
    pub fn hpc_tpsmp(&self) -> HPC_TPSMP_R {
        HPC_TPSMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP."]
    #[inline(always)]
    pub fn l_tpsmp(&self) -> L_TPSMP_R {
        L_TPSMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Indicates the privilege/user mode for the USB"]
    #[inline(always)]
    pub fn hpc_usb(&self) -> HPC_USB_R {
        HPC_USB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB."]
    #[inline(always)]
    pub fn l_usb(&self) -> L_USB_R {
        L_USB_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Indicates the privilege/user mode for the eDMA"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_dma(&mut self) -> HPC_DMA_W<2> {
        HPC_DMA_W::new(self)
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    #[must_use]
    pub fn l_dma(&mut self) -> L_DMA_W<3> {
        L_DMA_W::new(self)
    }
    #[doc = "Bit 4 - Indicates the privilege/user mode for the LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_lcdif(&mut self) -> HPC_LCDIF_W<4> {
        HPC_LCDIF_W::new(self)
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn l_lcdif(&mut self) -> L_LCDIF_W<5> {
        L_LCDIF_W::new(self)
    }
    #[doc = "Bit 6 - Indicates the privilege/user mode for the CSI"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_csi(&mut self) -> HPC_CSI_W<6> {
        HPC_CSI_W::new(self)
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    #[must_use]
    pub fn l_csi(&mut self) -> L_CSI_W<7> {
        L_CSI_W::new(self)
    }
    #[doc = "Bit 8 - Indicates the privilege/user mode for the PXP"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_pxp(&mut self) -> HPC_PXP_W<8> {
        HPC_PXP_W::new(self)
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    #[must_use]
    pub fn l_pxp(&mut self) -> L_PXP_W<9> {
        L_PXP_W::new(self)
    }
    #[doc = "Bit 10 - Indicates the privilege/user mode for the DCP"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_dcp(&mut self) -> HPC_DCP_W<10> {
        HPC_DCP_W::new(self)
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    #[must_use]
    pub fn l_dcp(&mut self) -> L_DCP_W<11> {
        L_DCP_W::new(self)
    }
    #[doc = "Bit 14 - Indicates the privilege/user mode for the ENET"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_enet(&mut self) -> HPC_ENET_W<14> {
        HPC_ENET_W::new(self)
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    #[must_use]
    pub fn l_enet(&mut self) -> L_ENET_W<15> {
        L_ENET_W::new(self)
    }
    #[doc = "Bit 16 - Indicates the privilege/user mode for the USDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_usdhc1(&mut self) -> HPC_USDHC1_W<16> {
        HPC_USDHC1_W::new(self)
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn l_usdhc1(&mut self) -> L_USDHC1_W<17> {
        L_USDHC1_W::new(self)
    }
    #[doc = "Bit 18 - Indicates the privilege/user mode for the USDHC2"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_usdhc2(&mut self) -> HPC_USDHC2_W<18> {
        HPC_USDHC2_W::new(self)
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2."]
    #[inline(always)]
    #[must_use]
    pub fn l_usdhc2(&mut self) -> L_USDHC2_W<19> {
        L_USDHC2_W::new(self)
    }
    #[doc = "Bit 20 - Indicates the privilege/user mode for the TPSMP"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_tpsmp(&mut self) -> HPC_TPSMP_W<20> {
        HPC_TPSMP_W::new(self)
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP."]
    #[inline(always)]
    #[must_use]
    pub fn l_tpsmp(&mut self) -> L_TPSMP_W<21> {
        L_TPSMP_W::new(self)
    }
    #[doc = "Bit 22 - Indicates the privilege/user mode for the USB"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_usb(&mut self) -> HPC_USB_W<22> {
        HPC_USB_W::new(self)
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB."]
    #[inline(always)]
    #[must_use]
    pub fn l_usb(&mut self) -> L_USB_W<23> {
        L_USB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HPCONTROL0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcontrol0](index.html) module"]
pub struct HPCONTROL0_SPEC;
impl crate::RegisterSpec for HPCONTROL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpcontrol0::R](R) reader structure"]
impl crate::Readable for HPCONTROL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpcontrol0::W](W) writer structure"]
impl crate::Writable for HPCONTROL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPCONTROL0 to value 0"]
impl crate::Resettable for HPCONTROL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
