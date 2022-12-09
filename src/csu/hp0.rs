#[doc = "Register `HP0` reader"]
pub struct R(crate::R<HP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP0` writer"]
pub struct W(crate::W<HP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP0_SPEC>;
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
impl From<crate::W<HP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_DMA` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the eDMA"]
pub type HP_DMA_R = crate::BitReader<HP_DMA_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the eDMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_DMA_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DMA_1 = 1,
}
impl From<HP_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: HP_DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_DMA_A {
        match self.bits {
            false => HP_DMA_A::HP_DMA_0,
            true => HP_DMA_A::HP_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_DMA_0`"]
    #[inline(always)]
    pub fn is_hp_dma_0(&self) -> bool {
        *self == HP_DMA_A::HP_DMA_0
    }
    #[doc = "Checks if the value of the field is `HP_DMA_1`"]
    #[inline(always)]
    pub fn is_hp_dma_1(&self) -> bool {
        *self == HP_DMA_A::HP_DMA_1
    }
}
#[doc = "Field `HP_DMA` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the eDMA"]
pub type HP_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_DMA_A, O>;
impl<'a, const O: u8> HP_DMA_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_dma_0(self) -> &'a mut W {
        self.variant(HP_DMA_A::HP_DMA_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_dma_1(self) -> &'a mut W {
        self.variant(HP_DMA_A::HP_DMA_1)
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
pub type L_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_DMA_A, O>;
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
#[doc = "Field `HP_LCDIF` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the LCDIF"]
pub type HP_LCDIF_R = crate::BitReader<HP_LCDIF_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_LCDIF_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_LCDIF_1 = 1,
}
impl From<HP_LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: HP_LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_LCDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_LCDIF_A {
        match self.bits {
            false => HP_LCDIF_A::HP_LCDIF_0,
            true => HP_LCDIF_A::HP_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_LCDIF_0`"]
    #[inline(always)]
    pub fn is_hp_lcdif_0(&self) -> bool {
        *self == HP_LCDIF_A::HP_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `HP_LCDIF_1`"]
    #[inline(always)]
    pub fn is_hp_lcdif_1(&self) -> bool {
        *self == HP_LCDIF_A::HP_LCDIF_1
    }
}
#[doc = "Field `HP_LCDIF` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the LCDIF"]
pub type HP_LCDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_LCDIF_A, O>;
impl<'a, const O: u8> HP_LCDIF_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_lcdif_0(self) -> &'a mut W {
        self.variant(HP_LCDIF_A::HP_LCDIF_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_lcdif_1(self) -> &'a mut W {
        self.variant(HP_LCDIF_A::HP_LCDIF_1)
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
pub type L_LCDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_LCDIF_A, O>;
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
#[doc = "Field `HP_CSI` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the CSI"]
pub type HP_CSI_R = crate::BitReader<HP_CSI_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the CSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_CSI_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_CSI_1 = 1,
}
impl From<HP_CSI_A> for bool {
    #[inline(always)]
    fn from(variant: HP_CSI_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_CSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_CSI_A {
        match self.bits {
            false => HP_CSI_A::HP_CSI_0,
            true => HP_CSI_A::HP_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_CSI_0`"]
    #[inline(always)]
    pub fn is_hp_csi_0(&self) -> bool {
        *self == HP_CSI_A::HP_CSI_0
    }
    #[doc = "Checks if the value of the field is `HP_CSI_1`"]
    #[inline(always)]
    pub fn is_hp_csi_1(&self) -> bool {
        *self == HP_CSI_A::HP_CSI_1
    }
}
#[doc = "Field `HP_CSI` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the CSI"]
pub type HP_CSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_CSI_A, O>;
impl<'a, const O: u8> HP_CSI_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_csi_0(self) -> &'a mut W {
        self.variant(HP_CSI_A::HP_CSI_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_csi_1(self) -> &'a mut W {
        self.variant(HP_CSI_A::HP_CSI_1)
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
pub type L_CSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_CSI_A, O>;
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
#[doc = "Field `HP_PXP` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the PXP"]
pub type HP_PXP_R = crate::BitReader<HP_PXP_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the PXP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_PXP_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_PXP_1 = 1,
}
impl From<HP_PXP_A> for bool {
    #[inline(always)]
    fn from(variant: HP_PXP_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_PXP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_PXP_A {
        match self.bits {
            false => HP_PXP_A::HP_PXP_0,
            true => HP_PXP_A::HP_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_PXP_0`"]
    #[inline(always)]
    pub fn is_hp_pxp_0(&self) -> bool {
        *self == HP_PXP_A::HP_PXP_0
    }
    #[doc = "Checks if the value of the field is `HP_PXP_1`"]
    #[inline(always)]
    pub fn is_hp_pxp_1(&self) -> bool {
        *self == HP_PXP_A::HP_PXP_1
    }
}
#[doc = "Field `HP_PXP` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the PXP"]
pub type HP_PXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_PXP_A, O>;
impl<'a, const O: u8> HP_PXP_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_pxp_0(self) -> &'a mut W {
        self.variant(HP_PXP_A::HP_PXP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_pxp_1(self) -> &'a mut W {
        self.variant(HP_PXP_A::HP_PXP_1)
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
pub type L_PXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_PXP_A, O>;
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
#[doc = "Field `HP_DCP` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the DCP"]
pub type HP_DCP_R = crate::BitReader<HP_DCP_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the DCP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_DCP_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_DCP_1 = 1,
}
impl From<HP_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: HP_DCP_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_DCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_DCP_A {
        match self.bits {
            false => HP_DCP_A::HP_DCP_0,
            true => HP_DCP_A::HP_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_DCP_0`"]
    #[inline(always)]
    pub fn is_hp_dcp_0(&self) -> bool {
        *self == HP_DCP_A::HP_DCP_0
    }
    #[doc = "Checks if the value of the field is `HP_DCP_1`"]
    #[inline(always)]
    pub fn is_hp_dcp_1(&self) -> bool {
        *self == HP_DCP_A::HP_DCP_1
    }
}
#[doc = "Field `HP_DCP` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the DCP"]
pub type HP_DCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_DCP_A, O>;
impl<'a, const O: u8> HP_DCP_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_dcp_0(self) -> &'a mut W {
        self.variant(HP_DCP_A::HP_DCP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_dcp_1(self) -> &'a mut W {
        self.variant(HP_DCP_A::HP_DCP_1)
    }
}
#[doc = "Field `L_DCP` reader - Lock bit set by the TZ software for the DCP"]
pub type L_DCP_R = crate::BitReader<L_DCP_A>;
#[doc = "Lock bit set by the TZ software for the DCP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L_DCP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit cannot be written by the software."]
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
pub type L_DCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_DCP_A, O>;
impl<'a, const O: u8> L_DCP_W<'a, O> {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_dcp_0(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit cannot be written by the software."]
    #[inline(always)]
    pub fn l_dcp_1(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_1)
    }
}
#[doc = "Field `HP_ENET` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the ENET"]
pub type HP_ENET_R = crate::BitReader<HP_ENET_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the ENET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_ENET_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET_1 = 1,
}
impl From<HP_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: HP_ENET_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_ENET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_ENET_A {
        match self.bits {
            false => HP_ENET_A::HP_ENET_0,
            true => HP_ENET_A::HP_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_ENET_0`"]
    #[inline(always)]
    pub fn is_hp_enet_0(&self) -> bool {
        *self == HP_ENET_A::HP_ENET_0
    }
    #[doc = "Checks if the value of the field is `HP_ENET_1`"]
    #[inline(always)]
    pub fn is_hp_enet_1(&self) -> bool {
        *self == HP_ENET_A::HP_ENET_1
    }
}
#[doc = "Field `HP_ENET` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the ENET"]
pub type HP_ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_ENET_A, O>;
impl<'a, const O: u8> HP_ENET_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_enet_0(self) -> &'a mut W {
        self.variant(HP_ENET_A::HP_ENET_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_enet_1(self) -> &'a mut W {
        self.variant(HP_ENET_A::HP_ENET_1)
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
pub type L_ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_ENET_A, O>;
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
#[doc = "Field `HP_USDHC1` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC1"]
pub type HP_USDHC1_R = crate::BitReader<HP_USDHC1_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_USDHC1_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC1_1 = 1,
}
impl From<HP_USDHC1_A> for bool {
    #[inline(always)]
    fn from(variant: HP_USDHC1_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_USDHC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_USDHC1_A {
        match self.bits {
            false => HP_USDHC1_A::HP_USDHC1_0,
            true => HP_USDHC1_A::HP_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USDHC1_0`"]
    #[inline(always)]
    pub fn is_hp_usdhc1_0(&self) -> bool {
        *self == HP_USDHC1_A::HP_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `HP_USDHC1_1`"]
    #[inline(always)]
    pub fn is_hp_usdhc1_1(&self) -> bool {
        *self == HP_USDHC1_A::HP_USDHC1_1
    }
}
#[doc = "Field `HP_USDHC1` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC1"]
pub type HP_USDHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_USDHC1_A, O>;
impl<'a, const O: u8> HP_USDHC1_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usdhc1_0(self) -> &'a mut W {
        self.variant(HP_USDHC1_A::HP_USDHC1_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usdhc1_1(self) -> &'a mut W {
        self.variant(HP_USDHC1_A::HP_USDHC1_1)
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
pub type L_USDHC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_USDHC1_A, O>;
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
#[doc = "Field `HP_USDHC2` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC2"]
pub type HP_USDHC2_R = crate::BitReader<HP_USDHC2_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_USDHC2_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USDHC2_1 = 1,
}
impl From<HP_USDHC2_A> for bool {
    #[inline(always)]
    fn from(variant: HP_USDHC2_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_USDHC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_USDHC2_A {
        match self.bits {
            false => HP_USDHC2_A::HP_USDHC2_0,
            true => HP_USDHC2_A::HP_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USDHC2_0`"]
    #[inline(always)]
    pub fn is_hp_usdhc2_0(&self) -> bool {
        *self == HP_USDHC2_A::HP_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `HP_USDHC2_1`"]
    #[inline(always)]
    pub fn is_hp_usdhc2_1(&self) -> bool {
        *self == HP_USDHC2_A::HP_USDHC2_1
    }
}
#[doc = "Field `HP_USDHC2` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC2"]
pub type HP_USDHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_USDHC2_A, O>;
impl<'a, const O: u8> HP_USDHC2_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usdhc2_0(self) -> &'a mut W {
        self.variant(HP_USDHC2_A::HP_USDHC2_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usdhc2_1(self) -> &'a mut W {
        self.variant(HP_USDHC2_A::HP_USDHC2_1)
    }
}
#[doc = "Field `L_USDHC2` reader - Lock bit set by the TZ software for the USDHC2"]
pub type L_USDHC2_R = crate::BitReader<L_USDHC2_A>;
#[doc = "Lock bit set by the TZ software for the USDHC2\n\nValue on reset: 0"]
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
#[doc = "Field `L_USDHC2` writer - Lock bit set by the TZ software for the USDHC2"]
pub type L_USDHC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_USDHC2_A, O>;
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
#[doc = "Field `HP_TPSMP` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the TPSMP"]
pub type HP_TPSMP_R = crate::BitReader<HP_TPSMP_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the TPSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_TPSMP_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_TPSMP_1 = 1,
}
impl From<HP_TPSMP_A> for bool {
    #[inline(always)]
    fn from(variant: HP_TPSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_TPSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_TPSMP_A {
        match self.bits {
            false => HP_TPSMP_A::HP_TPSMP_0,
            true => HP_TPSMP_A::HP_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_TPSMP_0`"]
    #[inline(always)]
    pub fn is_hp_tpsmp_0(&self) -> bool {
        *self == HP_TPSMP_A::HP_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `HP_TPSMP_1`"]
    #[inline(always)]
    pub fn is_hp_tpsmp_1(&self) -> bool {
        *self == HP_TPSMP_A::HP_TPSMP_1
    }
}
#[doc = "Field `HP_TPSMP` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the TPSMP"]
pub type HP_TPSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_TPSMP_A, O>;
impl<'a, const O: u8> HP_TPSMP_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_tpsmp_0(self) -> &'a mut W {
        self.variant(HP_TPSMP_A::HP_TPSMP_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_tpsmp_1(self) -> &'a mut W {
        self.variant(HP_TPSMP_A::HP_TPSMP_1)
    }
}
#[doc = "Field `L_TPSMP` reader - Lock bit set by the TZ software for the TPSMP"]
pub type L_TPSMP_R = crate::BitReader<L_TPSMP_A>;
#[doc = "Lock bit set by the TZ software for the TPSMP\n\nValue on reset: 0"]
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
#[doc = "Field `L_TPSMP` writer - Lock bit set by the TZ software for the TPSMP"]
pub type L_TPSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_TPSMP_A, O>;
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
#[doc = "Field `HP_USB` reader - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USB"]
pub type HP_USB_R = crate::BitReader<HP_USB_A>;
#[doc = "Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HP_USB_A {
    #[doc = "0: The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_0 = 0,
    #[doc = "1: The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_USB_1 = 1,
}
impl From<HP_USB_A> for bool {
    #[inline(always)]
    fn from(variant: HP_USB_A) -> Self {
        variant as u8 != 0
    }
}
impl HP_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_USB_A {
        match self.bits {
            false => HP_USB_A::HP_USB_0,
            true => HP_USB_A::HP_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_USB_0`"]
    #[inline(always)]
    pub fn is_hp_usb_0(&self) -> bool {
        *self == HP_USB_A::HP_USB_0
    }
    #[doc = "Checks if the value of the field is `HP_USB_1`"]
    #[inline(always)]
    pub fn is_hp_usb_1(&self) -> bool {
        *self == HP_USB_A::HP_USB_1
    }
}
#[doc = "Field `HP_USB` writer - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USB"]
pub type HP_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, HP_USB_A, O>;
impl<'a, const O: u8> HP_USB_W<'a, O> {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usb_0(self) -> &'a mut W {
        self.variant(HP_USB_A::HP_USB_0)
    }
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    #[inline(always)]
    pub fn hp_usb_1(self) -> &'a mut W {
        self.variant(HP_USB_A::HP_USB_1)
    }
}
#[doc = "Field `L_USB` reader - Lock bit set by the TZ software for the USB"]
pub type L_USB_R = crate::BitReader<L_USB_A>;
#[doc = "Lock bit set by the TZ software for the USB\n\nValue on reset: 0"]
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
#[doc = "Field `L_USB` writer - Lock bit set by the TZ software for the USB"]
pub type L_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP0_SPEC, L_USB_A, O>;
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
    #[doc = "Bit 2 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the eDMA"]
    #[inline(always)]
    pub fn hp_dma(&self) -> HP_DMA_R {
        HP_DMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub fn l_dma(&self) -> L_DMA_R {
        L_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the LCDIF"]
    #[inline(always)]
    pub fn hp_lcdif(&self) -> HP_LCDIF_R {
        HP_LCDIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub fn l_lcdif(&self) -> L_LCDIF_R {
        L_LCDIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the CSI"]
    #[inline(always)]
    pub fn hp_csi(&self) -> HP_CSI_R {
        HP_CSI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub fn l_csi(&self) -> L_CSI_R {
        L_CSI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the PXP"]
    #[inline(always)]
    pub fn hp_pxp(&self) -> HP_PXP_R {
        HP_PXP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub fn l_pxp(&self) -> L_PXP_R {
        L_PXP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the DCP"]
    #[inline(always)]
    pub fn hp_dcp(&self) -> HP_DCP_R {
        HP_DCP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub fn l_dcp(&self) -> L_DCP_R {
        L_DCP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the ENET"]
    #[inline(always)]
    pub fn hp_enet(&self) -> HP_ENET_R {
        HP_ENET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    pub fn l_enet(&self) -> L_ENET_R {
        L_ENET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC1"]
    #[inline(always)]
    pub fn hp_usdhc1(&self) -> HP_USDHC1_R {
        HP_USDHC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub fn l_usdhc1(&self) -> L_USDHC1_R {
        L_USDHC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC2"]
    #[inline(always)]
    pub fn hp_usdhc2(&self) -> HP_USDHC2_R {
        HP_USDHC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub fn l_usdhc2(&self) -> L_USDHC2_R {
        L_USDHC2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the TPSMP"]
    #[inline(always)]
    pub fn hp_tpsmp(&self) -> HP_TPSMP_R {
        HP_TPSMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub fn l_tpsmp(&self) -> L_TPSMP_R {
        L_TPSMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USB"]
    #[inline(always)]
    pub fn hp_usb(&self) -> HP_USB_R {
        HP_USB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub fn l_usb(&self) -> L_USB_R {
        L_USB_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the eDMA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_dma(&mut self) -> HP_DMA_W<2> {
        HP_DMA_W::new(self)
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    #[must_use]
    pub fn l_dma(&mut self) -> L_DMA_W<3> {
        L_DMA_W::new(self)
    }
    #[doc = "Bit 4 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn hp_lcdif(&mut self) -> HP_LCDIF_W<4> {
        HP_LCDIF_W::new(self)
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    #[must_use]
    pub fn l_lcdif(&mut self) -> L_LCDIF_W<5> {
        L_LCDIF_W::new(self)
    }
    #[doc = "Bit 6 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the CSI"]
    #[inline(always)]
    #[must_use]
    pub fn hp_csi(&mut self) -> HP_CSI_W<6> {
        HP_CSI_W::new(self)
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    #[must_use]
    pub fn l_csi(&mut self) -> L_CSI_W<7> {
        L_CSI_W::new(self)
    }
    #[doc = "Bit 8 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the PXP"]
    #[inline(always)]
    #[must_use]
    pub fn hp_pxp(&mut self) -> HP_PXP_W<8> {
        HP_PXP_W::new(self)
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    #[must_use]
    pub fn l_pxp(&mut self) -> L_PXP_W<9> {
        L_PXP_W::new(self)
    }
    #[doc = "Bit 10 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the DCP"]
    #[inline(always)]
    #[must_use]
    pub fn hp_dcp(&mut self) -> HP_DCP_W<10> {
        HP_DCP_W::new(self)
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    #[must_use]
    pub fn l_dcp(&mut self) -> L_DCP_W<11> {
        L_DCP_W::new(self)
    }
    #[doc = "Bit 14 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the ENET"]
    #[inline(always)]
    #[must_use]
    pub fn hp_enet(&mut self) -> HP_ENET_W<14> {
        HP_ENET_W::new(self)
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET"]
    #[inline(always)]
    #[must_use]
    pub fn l_enet(&mut self) -> L_ENET_W<15> {
        L_ENET_W::new(self)
    }
    #[doc = "Bit 16 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn hp_usdhc1(&mut self) -> HP_USDHC1_W<16> {
        HP_USDHC1_W::new(self)
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    #[must_use]
    pub fn l_usdhc1(&mut self) -> L_USDHC1_W<17> {
        L_USDHC1_W::new(self)
    }
    #[doc = "Bit 18 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USDHC2"]
    #[inline(always)]
    #[must_use]
    pub fn hp_usdhc2(&mut self) -> HP_USDHC2_W<18> {
        HP_USDHC2_W::new(self)
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    #[must_use]
    pub fn l_usdhc2(&mut self) -> L_USDHC2_W<19> {
        L_USDHC2_W::new(self)
    }
    #[doc = "Bit 20 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the TPSMP"]
    #[inline(always)]
    #[must_use]
    pub fn hp_tpsmp(&mut self) -> HP_TPSMP_W<20> {
        HP_TPSMP_W::new(self)
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    #[must_use]
    pub fn l_tpsmp(&mut self) -> L_TPSMP_W<21> {
        L_TPSMP_W::new(self)
    }
    #[doc = "Bit 22 - Determines whether the register value of the corresponding HP field is passed as the hprot\\[1\\]
of the USB"]
    #[inline(always)]
    #[must_use]
    pub fn hp_usb(&mut self) -> HP_USB_W<22> {
        HP_USB_W::new(self)
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
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
#[doc = "HP0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp0](index.html) module"]
pub struct HP0_SPEC;
impl crate::RegisterSpec for HP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp0::R](R) reader structure"]
impl crate::Readable for HP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp0::W](W) writer structure"]
impl crate::Writable for HP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP0 to value 0"]
impl crate::Resettable for HP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
