#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STATUS` writer"]
pub struct W(crate::W<INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STATUS_SPEC>;
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
impl From<crate::W<INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - Command complete"]
pub type CC_R = crate::BitReader<CC_A>;
#[doc = "Command complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC_A {
    #[doc = "0: Command not complete"]
    CC_0 = 0,
    #[doc = "1: Command complete"]
    CC_1 = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
impl CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::CC_0,
            true => CC_A::CC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CC_0`"]
    #[inline(always)]
    pub fn is_cc_0(&self) -> bool {
        *self == CC_A::CC_0
    }
    #[doc = "Checks if the value of the field is `CC_1`"]
    #[inline(always)]
    pub fn is_cc_1(&self) -> bool {
        *self == CC_A::CC_1
    }
}
#[doc = "Field `CC` writer - Command complete"]
pub type CC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CC_A, O>;
impl<'a, const O: u8> CC_W<'a, O> {
    #[doc = "Command not complete"]
    #[inline(always)]
    pub fn cc_0(self) -> &'a mut W {
        self.variant(CC_A::CC_0)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub fn cc_1(self) -> &'a mut W {
        self.variant(CC_A::CC_1)
    }
}
#[doc = "Field `TC` reader - Transfer complete"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transfer complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transfer does not complete"]
    TC_0 = 0,
    #[doc = "1: Transfer complete"]
    TC_1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::TC_0,
            true => TC_A::TC_1,
        }
    }
    #[doc = "Checks if the value of the field is `TC_0`"]
    #[inline(always)]
    pub fn is_tc_0(&self) -> bool {
        *self == TC_A::TC_0
    }
    #[doc = "Checks if the value of the field is `TC_1`"]
    #[inline(always)]
    pub fn is_tc_1(&self) -> bool {
        *self == TC_A::TC_1
    }
}
#[doc = "Field `TC` writer - Transfer complete"]
pub type TC_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, TC_A, O>;
impl<'a, const O: u8> TC_W<'a, O> {
    #[doc = "Transfer does not complete"]
    #[inline(always)]
    pub fn tc_0(self) -> &'a mut W {
        self.variant(TC_A::TC_0)
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn tc_1(self) -> &'a mut W {
        self.variant(TC_A::TC_1)
    }
}
#[doc = "Field `BGE` reader - Block gap event"]
pub type BGE_R = crate::BitReader<BGE_A>;
#[doc = "Block gap event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGE_A {
    #[doc = "0: No block gap event"]
    BGE_0 = 0,
    #[doc = "1: Transaction stopped at block gap"]
    BGE_1 = 1,
}
impl From<BGE_A> for bool {
    #[inline(always)]
    fn from(variant: BGE_A) -> Self {
        variant as u8 != 0
    }
}
impl BGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGE_A {
        match self.bits {
            false => BGE_A::BGE_0,
            true => BGE_A::BGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGE_0`"]
    #[inline(always)]
    pub fn is_bge_0(&self) -> bool {
        *self == BGE_A::BGE_0
    }
    #[doc = "Checks if the value of the field is `BGE_1`"]
    #[inline(always)]
    pub fn is_bge_1(&self) -> bool {
        *self == BGE_A::BGE_1
    }
}
#[doc = "Field `BGE` writer - Block gap event"]
pub type BGE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, BGE_A, O>;
impl<'a, const O: u8> BGE_W<'a, O> {
    #[doc = "No block gap event"]
    #[inline(always)]
    pub fn bge_0(self) -> &'a mut W {
        self.variant(BGE_A::BGE_0)
    }
    #[doc = "Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bge_1(self) -> &'a mut W {
        self.variant(BGE_A::BGE_1)
    }
}
#[doc = "Field `DINT` reader - DMA interrupt"]
pub type DINT_R = crate::BitReader<DINT_A>;
#[doc = "DMA interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINT_A {
    #[doc = "0: No DMA interrupt"]
    DINT_0 = 0,
    #[doc = "1: DMA interrupt is generated."]
    DINT_1 = 1,
}
impl From<DINT_A> for bool {
    #[inline(always)]
    fn from(variant: DINT_A) -> Self {
        variant as u8 != 0
    }
}
impl DINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINT_A {
        match self.bits {
            false => DINT_A::DINT_0,
            true => DINT_A::DINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DINT_0`"]
    #[inline(always)]
    pub fn is_dint_0(&self) -> bool {
        *self == DINT_A::DINT_0
    }
    #[doc = "Checks if the value of the field is `DINT_1`"]
    #[inline(always)]
    pub fn is_dint_1(&self) -> bool {
        *self == DINT_A::DINT_1
    }
}
#[doc = "Field `DINT` writer - DMA interrupt"]
pub type DINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DINT_A, O>;
impl<'a, const O: u8> DINT_W<'a, O> {
    #[doc = "No DMA interrupt"]
    #[inline(always)]
    pub fn dint_0(self) -> &'a mut W {
        self.variant(DINT_A::DINT_0)
    }
    #[doc = "DMA interrupt is generated."]
    #[inline(always)]
    pub fn dint_1(self) -> &'a mut W {
        self.variant(DINT_A::DINT_1)
    }
}
#[doc = "Field `BWR` reader - Buffer write ready"]
pub type BWR_R = crate::BitReader<BWR_A>;
#[doc = "Buffer write ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWR_A {
    #[doc = "0: Not ready to write buffer"]
    BWR_0 = 0,
    #[doc = "1: Ready to write buffer"]
    BWR_1 = 1,
}
impl From<BWR_A> for bool {
    #[inline(always)]
    fn from(variant: BWR_A) -> Self {
        variant as u8 != 0
    }
}
impl BWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWR_A {
        match self.bits {
            false => BWR_A::BWR_0,
            true => BWR_A::BWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWR_0`"]
    #[inline(always)]
    pub fn is_bwr_0(&self) -> bool {
        *self == BWR_A::BWR_0
    }
    #[doc = "Checks if the value of the field is `BWR_1`"]
    #[inline(always)]
    pub fn is_bwr_1(&self) -> bool {
        *self == BWR_A::BWR_1
    }
}
#[doc = "Field `BWR` writer - Buffer write ready"]
pub type BWR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, BWR_A, O>;
impl<'a, const O: u8> BWR_W<'a, O> {
    #[doc = "Not ready to write buffer"]
    #[inline(always)]
    pub fn bwr_0(self) -> &'a mut W {
        self.variant(BWR_A::BWR_0)
    }
    #[doc = "Ready to write buffer"]
    #[inline(always)]
    pub fn bwr_1(self) -> &'a mut W {
        self.variant(BWR_A::BWR_1)
    }
}
#[doc = "Field `BRR` reader - Buffer read ready"]
pub type BRR_R = crate::BitReader<BRR_A>;
#[doc = "Buffer read ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRR_A {
    #[doc = "0: Not ready to read buffer"]
    BRR_0 = 0,
    #[doc = "1: Ready to read buffer"]
    BRR_1 = 1,
}
impl From<BRR_A> for bool {
    #[inline(always)]
    fn from(variant: BRR_A) -> Self {
        variant as u8 != 0
    }
}
impl BRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRR_A {
        match self.bits {
            false => BRR_A::BRR_0,
            true => BRR_A::BRR_1,
        }
    }
    #[doc = "Checks if the value of the field is `BRR_0`"]
    #[inline(always)]
    pub fn is_brr_0(&self) -> bool {
        *self == BRR_A::BRR_0
    }
    #[doc = "Checks if the value of the field is `BRR_1`"]
    #[inline(always)]
    pub fn is_brr_1(&self) -> bool {
        *self == BRR_A::BRR_1
    }
}
#[doc = "Field `BRR` writer - Buffer read ready"]
pub type BRR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, BRR_A, O>;
impl<'a, const O: u8> BRR_W<'a, O> {
    #[doc = "Not ready to read buffer"]
    #[inline(always)]
    pub fn brr_0(self) -> &'a mut W {
        self.variant(BRR_A::BRR_0)
    }
    #[doc = "Ready to read buffer"]
    #[inline(always)]
    pub fn brr_1(self) -> &'a mut W {
        self.variant(BRR_A::BRR_1)
    }
}
#[doc = "Field `CINS` reader - Card insertion"]
pub type CINS_R = crate::BitReader<CINS_A>;
#[doc = "Card insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINS_A {
    #[doc = "0: Card state unstable or removed"]
    CINS_0 = 0,
    #[doc = "1: Card inserted"]
    CINS_1 = 1,
}
impl From<CINS_A> for bool {
    #[inline(always)]
    fn from(variant: CINS_A) -> Self {
        variant as u8 != 0
    }
}
impl CINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINS_A {
        match self.bits {
            false => CINS_A::CINS_0,
            true => CINS_A::CINS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINS_0`"]
    #[inline(always)]
    pub fn is_cins_0(&self) -> bool {
        *self == CINS_A::CINS_0
    }
    #[doc = "Checks if the value of the field is `CINS_1`"]
    #[inline(always)]
    pub fn is_cins_1(&self) -> bool {
        *self == CINS_A::CINS_1
    }
}
#[doc = "Field `CINS` writer - Card insertion"]
pub type CINS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CINS_A, O>;
impl<'a, const O: u8> CINS_W<'a, O> {
    #[doc = "Card state unstable or removed"]
    #[inline(always)]
    pub fn cins_0(self) -> &'a mut W {
        self.variant(CINS_A::CINS_0)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub fn cins_1(self) -> &'a mut W {
        self.variant(CINS_A::CINS_1)
    }
}
#[doc = "Field `CRM` reader - Card removal"]
pub type CRM_R = crate::BitReader<CRM_A>;
#[doc = "Card removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRM_A {
    #[doc = "0: Card state unstable or inserted"]
    CRM_0 = 0,
    #[doc = "1: Card removed"]
    CRM_1 = 1,
}
impl From<CRM_A> for bool {
    #[inline(always)]
    fn from(variant: CRM_A) -> Self {
        variant as u8 != 0
    }
}
impl CRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRM_A {
        match self.bits {
            false => CRM_A::CRM_0,
            true => CRM_A::CRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRM_0`"]
    #[inline(always)]
    pub fn is_crm_0(&self) -> bool {
        *self == CRM_A::CRM_0
    }
    #[doc = "Checks if the value of the field is `CRM_1`"]
    #[inline(always)]
    pub fn is_crm_1(&self) -> bool {
        *self == CRM_A::CRM_1
    }
}
#[doc = "Field `CRM` writer - Card removal"]
pub type CRM_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CRM_A, O>;
impl<'a, const O: u8> CRM_W<'a, O> {
    #[doc = "Card state unstable or inserted"]
    #[inline(always)]
    pub fn crm_0(self) -> &'a mut W {
        self.variant(CRM_A::CRM_0)
    }
    #[doc = "Card removed"]
    #[inline(always)]
    pub fn crm_1(self) -> &'a mut W {
        self.variant(CRM_A::CRM_1)
    }
}
#[doc = "Field `CINT` reader - Card interrupt"]
pub type CINT_R = crate::BitReader<CINT_A>;
#[doc = "Card interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINT_A {
    #[doc = "0: No card interrupt"]
    CINT_0 = 0,
    #[doc = "1: Generate card interrupt"]
    CINT_1 = 1,
}
impl From<CINT_A> for bool {
    #[inline(always)]
    fn from(variant: CINT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINT_A {
        match self.bits {
            false => CINT_A::CINT_0,
            true => CINT_A::CINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINT_0`"]
    #[inline(always)]
    pub fn is_cint_0(&self) -> bool {
        *self == CINT_A::CINT_0
    }
    #[doc = "Checks if the value of the field is `CINT_1`"]
    #[inline(always)]
    pub fn is_cint_1(&self) -> bool {
        *self == CINT_A::CINT_1
    }
}
#[doc = "Field `CINT` writer - Card interrupt"]
pub type CINT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CINT_A, O>;
impl<'a, const O: u8> CINT_W<'a, O> {
    #[doc = "No card interrupt"]
    #[inline(always)]
    pub fn cint_0(self) -> &'a mut W {
        self.variant(CINT_A::CINT_0)
    }
    #[doc = "Generate card interrupt"]
    #[inline(always)]
    pub fn cint_1(self) -> &'a mut W {
        self.variant(CINT_A::CINT_1)
    }
}
#[doc = "Field `RTE` reader - Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type RTE_R = crate::BitReader<RTE_A>;
#[doc = "Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTE_A {
    #[doc = "0: Re-tuning is not required."]
    RTE_0 = 0,
    #[doc = "1: Re-tuning should be performed."]
    RTE_1 = 1,
}
impl From<RTE_A> for bool {
    #[inline(always)]
    fn from(variant: RTE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTE_A {
        match self.bits {
            false => RTE_A::RTE_0,
            true => RTE_A::RTE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTE_0`"]
    #[inline(always)]
    pub fn is_rte_0(&self) -> bool {
        *self == RTE_A::RTE_0
    }
    #[doc = "Checks if the value of the field is `RTE_1`"]
    #[inline(always)]
    pub fn is_rte_1(&self) -> bool {
        *self == RTE_A::RTE_1
    }
}
#[doc = "Field `RTE` writer - Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type RTE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, RTE_A, O>;
impl<'a, const O: u8> RTE_W<'a, O> {
    #[doc = "Re-tuning is not required."]
    #[inline(always)]
    pub fn rte_0(self) -> &'a mut W {
        self.variant(RTE_A::RTE_0)
    }
    #[doc = "Re-tuning should be performed."]
    #[inline(always)]
    pub fn rte_1(self) -> &'a mut W {
        self.variant(RTE_A::RTE_1)
    }
}
#[doc = "Field `TP` reader - Tuning pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TP_R = crate::BitReader<bool>;
#[doc = "Field `TP` writer - Tuning pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `ERR_INT_STATUS` reader - Error Interrupt Status"]
pub type ERR_INT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `CTOE` reader - Command timeout error"]
pub type CTOE_R = crate::BitReader<CTOE_A>;
#[doc = "Command timeout error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOE_A {
    #[doc = "0: No error"]
    CTOE_0 = 0,
    #[doc = "1: Time out"]
    CTOE_1 = 1,
}
impl From<CTOE_A> for bool {
    #[inline(always)]
    fn from(variant: CTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOE_A {
        match self.bits {
            false => CTOE_A::CTOE_0,
            true => CTOE_A::CTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTOE_0`"]
    #[inline(always)]
    pub fn is_ctoe_0(&self) -> bool {
        *self == CTOE_A::CTOE_0
    }
    #[doc = "Checks if the value of the field is `CTOE_1`"]
    #[inline(always)]
    pub fn is_ctoe_1(&self) -> bool {
        *self == CTOE_A::CTOE_1
    }
}
#[doc = "Field `CTOE` writer - Command timeout error"]
pub type CTOE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CTOE_A, O>;
impl<'a, const O: u8> CTOE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ctoe_0(self) -> &'a mut W {
        self.variant(CTOE_A::CTOE_0)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn ctoe_1(self) -> &'a mut W {
        self.variant(CTOE_A::CTOE_1)
    }
}
#[doc = "Field `CCE` reader - Command CRC error"]
pub type CCE_R = crate::BitReader<CCE_A>;
#[doc = "Command CRC error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: No error"]
    CCE_0 = 0,
    #[doc = "1: CRC error generated"]
    CCE_1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::CCE_0,
            true => CCE_A::CCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCE_0`"]
    #[inline(always)]
    pub fn is_cce_0(&self) -> bool {
        *self == CCE_A::CCE_0
    }
    #[doc = "Checks if the value of the field is `CCE_1`"]
    #[inline(always)]
    pub fn is_cce_1(&self) -> bool {
        *self == CCE_A::CCE_1
    }
}
#[doc = "Field `CCE` writer - Command CRC error"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CCE_A, O>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn cce_0(self) -> &'a mut W {
        self.variant(CCE_A::CCE_0)
    }
    #[doc = "CRC error generated"]
    #[inline(always)]
    pub fn cce_1(self) -> &'a mut W {
        self.variant(CCE_A::CCE_1)
    }
}
#[doc = "Field `CEBE` reader - Command end bit error"]
pub type CEBE_R = crate::BitReader<CEBE_A>;
#[doc = "Command end bit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBE_A {
    #[doc = "0: No error"]
    CEBE_0 = 0,
    #[doc = "1: End bit error generated"]
    CEBE_1 = 1,
}
impl From<CEBE_A> for bool {
    #[inline(always)]
    fn from(variant: CEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBE_A {
        match self.bits {
            false => CEBE_A::CEBE_0,
            true => CEBE_A::CEBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEBE_0`"]
    #[inline(always)]
    pub fn is_cebe_0(&self) -> bool {
        *self == CEBE_A::CEBE_0
    }
    #[doc = "Checks if the value of the field is `CEBE_1`"]
    #[inline(always)]
    pub fn is_cebe_1(&self) -> bool {
        *self == CEBE_A::CEBE_1
    }
}
#[doc = "Field `CEBE` writer - Command end bit error"]
pub type CEBE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CEBE_A, O>;
impl<'a, const O: u8> CEBE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn cebe_0(self) -> &'a mut W {
        self.variant(CEBE_A::CEBE_0)
    }
    #[doc = "End bit error generated"]
    #[inline(always)]
    pub fn cebe_1(self) -> &'a mut W {
        self.variant(CEBE_A::CEBE_1)
    }
}
#[doc = "Field `CIE` reader - Command index error"]
pub type CIE_R = crate::BitReader<CIE_A>;
#[doc = "Command index error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIE_A {
    #[doc = "0: No error"]
    CIE_0 = 0,
    #[doc = "1: Error"]
    CIE_1 = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::CIE_0,
            true => CIE_A::CIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIE_0`"]
    #[inline(always)]
    pub fn is_cie_0(&self) -> bool {
        *self == CIE_A::CIE_0
    }
    #[doc = "Checks if the value of the field is `CIE_1`"]
    #[inline(always)]
    pub fn is_cie_1(&self) -> bool {
        *self == CIE_A::CIE_1
    }
}
#[doc = "Field `CIE` writer - Command index error"]
pub type CIE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, CIE_A, O>;
impl<'a, const O: u8> CIE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn cie_0(self) -> &'a mut W {
        self.variant(CIE_A::CIE_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn cie_1(self) -> &'a mut W {
        self.variant(CIE_A::CIE_1)
    }
}
#[doc = "Field `DTOE` reader - Data timeout error"]
pub type DTOE_R = crate::BitReader<DTOE_A>;
#[doc = "Data timeout error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOE_A {
    #[doc = "0: No error"]
    DTOE_0 = 0,
    #[doc = "1: Time out"]
    DTOE_1 = 1,
}
impl From<DTOE_A> for bool {
    #[inline(always)]
    fn from(variant: DTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOE_A {
        match self.bits {
            false => DTOE_A::DTOE_0,
            true => DTOE_A::DTOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTOE_0`"]
    #[inline(always)]
    pub fn is_dtoe_0(&self) -> bool {
        *self == DTOE_A::DTOE_0
    }
    #[doc = "Checks if the value of the field is `DTOE_1`"]
    #[inline(always)]
    pub fn is_dtoe_1(&self) -> bool {
        *self == DTOE_A::DTOE_1
    }
}
#[doc = "Field `DTOE` writer - Data timeout error"]
pub type DTOE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DTOE_A, O>;
impl<'a, const O: u8> DTOE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn dtoe_0(self) -> &'a mut W {
        self.variant(DTOE_A::DTOE_0)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn dtoe_1(self) -> &'a mut W {
        self.variant(DTOE_A::DTOE_1)
    }
}
#[doc = "Field `DCE` reader - Data CRC error"]
pub type DCE_R = crate::BitReader<DCE_A>;
#[doc = "Data CRC error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCE_A {
    #[doc = "0: No error"]
    DCE_0 = 0,
    #[doc = "1: Error"]
    DCE_1 = 1,
}
impl From<DCE_A> for bool {
    #[inline(always)]
    fn from(variant: DCE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCE_A {
        match self.bits {
            false => DCE_A::DCE_0,
            true => DCE_A::DCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCE_0`"]
    #[inline(always)]
    pub fn is_dce_0(&self) -> bool {
        *self == DCE_A::DCE_0
    }
    #[doc = "Checks if the value of the field is `DCE_1`"]
    #[inline(always)]
    pub fn is_dce_1(&self) -> bool {
        *self == DCE_A::DCE_1
    }
}
#[doc = "Field `DCE` writer - Data CRC error"]
pub type DCE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DCE_A, O>;
impl<'a, const O: u8> DCE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn dce_0(self) -> &'a mut W {
        self.variant(DCE_A::DCE_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn dce_1(self) -> &'a mut W {
        self.variant(DCE_A::DCE_1)
    }
}
#[doc = "Field `DEBE` reader - Data end bit error"]
pub type DEBE_R = crate::BitReader<DEBE_A>;
#[doc = "Data end bit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBE_A {
    #[doc = "0: No error"]
    DEBE_0 = 0,
    #[doc = "1: Error"]
    DEBE_1 = 1,
}
impl From<DEBE_A> for bool {
    #[inline(always)]
    fn from(variant: DEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBE_A {
        match self.bits {
            false => DEBE_A::DEBE_0,
            true => DEBE_A::DEBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEBE_0`"]
    #[inline(always)]
    pub fn is_debe_0(&self) -> bool {
        *self == DEBE_A::DEBE_0
    }
    #[doc = "Checks if the value of the field is `DEBE_1`"]
    #[inline(always)]
    pub fn is_debe_1(&self) -> bool {
        *self == DEBE_A::DEBE_1
    }
}
#[doc = "Field `DEBE` writer - Data end bit error"]
pub type DEBE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DEBE_A, O>;
impl<'a, const O: u8> DEBE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn debe_0(self) -> &'a mut W {
        self.variant(DEBE_A::DEBE_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn debe_1(self) -> &'a mut W {
        self.variant(DEBE_A::DEBE_1)
    }
}
#[doc = "Field `AC12E` reader - Auto CMD12 error"]
pub type AC12E_R = crate::BitReader<AC12E_A>;
#[doc = "Auto CMD12 error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12E_A {
    #[doc = "0: No error"]
    AC12E_0 = 0,
    #[doc = "1: Error"]
    AC12E_1 = 1,
}
impl From<AC12E_A> for bool {
    #[inline(always)]
    fn from(variant: AC12E_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12E_A {
        match self.bits {
            false => AC12E_A::AC12E_0,
            true => AC12E_A::AC12E_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12E_0`"]
    #[inline(always)]
    pub fn is_ac12e_0(&self) -> bool {
        *self == AC12E_A::AC12E_0
    }
    #[doc = "Checks if the value of the field is `AC12E_1`"]
    #[inline(always)]
    pub fn is_ac12e_1(&self) -> bool {
        *self == AC12E_A::AC12E_1
    }
}
#[doc = "Field `AC12E` writer - Auto CMD12 error"]
pub type AC12E_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, AC12E_A, O>;
impl<'a, const O: u8> AC12E_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ac12e_0(self) -> &'a mut W {
        self.variant(AC12E_A::AC12E_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn ac12e_1(self) -> &'a mut W {
        self.variant(AC12E_A::AC12E_1)
    }
}
#[doc = "Field `TNE` reader - Tuning error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TNE_R = crate::BitReader<bool>;
#[doc = "Field `TNE` writer - Tuning error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
pub type TNE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, bool, O>;
#[doc = "Field `DMAE` reader - DMA error"]
pub type DMAE_R = crate::BitReader<DMAE_A>;
#[doc = "DMA error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAE_A {
    #[doc = "0: No error"]
    DMAE_0 = 0,
    #[doc = "1: Error"]
    DMAE_1 = 1,
}
impl From<DMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAE_A {
        match self.bits {
            false => DMAE_A::DMAE_0,
            true => DMAE_A::DMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAE_0`"]
    #[inline(always)]
    pub fn is_dmae_0(&self) -> bool {
        *self == DMAE_A::DMAE_0
    }
    #[doc = "Checks if the value of the field is `DMAE_1`"]
    #[inline(always)]
    pub fn is_dmae_1(&self) -> bool {
        *self == DMAE_A::DMAE_1
    }
}
#[doc = "Field `DMAE` writer - DMA error"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INT_STATUS_SPEC, DMAE_A, O>;
impl<'a, const O: u8> DMAE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn dmae_0(self) -> &'a mut W {
        self.variant(DMAE_A::DMAE_0)
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn dmae_1(self) -> &'a mut W {
        self.variant(DMAE_A::DMAE_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block gap event"]
    #[inline(always)]
    pub fn bge(&self) -> BGE_R {
        BGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA interrupt"]
    #[inline(always)]
    pub fn dint(&self) -> DINT_R {
        DINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer write ready"]
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer read ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card removal"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rte(&self) -> RTE_R {
        RTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Tuning pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt Status"]
    #[inline(always)]
    pub fn err_int_status(&self) -> ERR_INT_STATUS_R {
        ERR_INT_STATUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Command timeout error"]
    #[inline(always)]
    pub fn ctoe(&self) -> CTOE_R {
        CTOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command end bit error"]
    #[inline(always)]
    pub fn cebe(&self) -> CEBE_R {
        CEBE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command index error"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data timeout error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data end bit error"]
    #[inline(always)]
    pub fn debe(&self) -> DEBE_R {
        DEBE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 error"]
    #[inline(always)]
    pub fn ac12e(&self) -> AC12E_R {
        AC12E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn tne(&self) -> TNE_R {
        TNE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA error"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command complete"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<0> {
        CC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<1> {
        TC_W::new(self)
    }
    #[doc = "Bit 2 - Block gap event"]
    #[inline(always)]
    #[must_use]
    pub fn bge(&mut self) -> BGE_W<2> {
        BGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dint(&mut self) -> DINT_W<3> {
        DINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer write ready"]
    #[inline(always)]
    #[must_use]
    pub fn bwr(&mut self) -> BWR_W<4> {
        BWR_W::new(self)
    }
    #[doc = "Bit 5 - Buffer read ready"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<5> {
        BRR_W::new(self)
    }
    #[doc = "Bit 6 - Card insertion"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CINS_W<6> {
        CINS_W::new(self)
    }
    #[doc = "Bit 7 - Card removal"]
    #[inline(always)]
    #[must_use]
    pub fn crm(&mut self) -> CRM_W<7> {
        CRM_W::new(self)
    }
    #[doc = "Bit 8 - Card interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<8> {
        CINT_W::new(self)
    }
    #[doc = "Bit 12 - Re-tuning event: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rte(&mut self) -> RTE_W<12> {
        RTE_W::new(self)
    }
    #[doc = "Bit 14 - Tuning pass:(only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn tp(&mut self) -> TP_W<14> {
        TP_W::new(self)
    }
    #[doc = "Bit 16 - Command timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn ctoe(&mut self) -> CTOE_W<16> {
        CTOE_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<17> {
        CCE_W::new(self)
    }
    #[doc = "Bit 18 - Command end bit error"]
    #[inline(always)]
    #[must_use]
    pub fn cebe(&mut self) -> CEBE_W<18> {
        CEBE_W::new(self)
    }
    #[doc = "Bit 19 - Command index error"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CIE_W<19> {
        CIE_W::new(self)
    }
    #[doc = "Bit 20 - Data timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<20> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC error"]
    #[inline(always)]
    #[must_use]
    pub fn dce(&mut self) -> DCE_W<21> {
        DCE_W::new(self)
    }
    #[doc = "Bit 22 - Data end bit error"]
    #[inline(always)]
    #[must_use]
    pub fn debe(&mut self) -> DEBE_W<22> {
        DEBE_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12e(&mut self) -> AC12E_W<24> {
        AC12E_W::new(self)
    }
    #[doc = "Bit 26 - Tuning error: (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn tne(&mut self) -> TNE_W<26> {
        TNE_W::new(self)
    }
    #[doc = "Bit 28 - DMA error"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<28> {
        DMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_status::W](W) writer structure"]
impl crate::Writable for INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x157f_51ff;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
