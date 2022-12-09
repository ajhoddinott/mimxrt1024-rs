#[doc = "Register `PORTSC1` reader"]
pub struct R(crate::R<PORTSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSC1` writer"]
pub struct W(crate::W<PORTSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSC1_SPEC>;
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
impl From<crate::W<PORTSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - Current Connect Status-Read Only"]
pub type CCS_R = crate::BitReader<bool>;
#[doc = "Field `CSC` reader - Connect Status Change-R/WC"]
pub type CSC_R = crate::BitReader<bool>;
#[doc = "Field `CSC` writer - Connect Status Change-R/WC"]
pub type CSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PE` reader - Port Enabled/Disabled-Read/Write"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Port Enabled/Disabled-Read/Write"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PEC` reader - Port Enable/Disable Change-R/WC"]
pub type PEC_R = crate::BitReader<bool>;
#[doc = "Field `PEC` writer - Port Enable/Disable Change-R/WC"]
pub type PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `OCA` reader - Over-current Active-Read Only"]
pub type OCA_R = crate::BitReader<OCA_A>;
#[doc = "Over-current Active-Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCA_A {
    #[doc = "0: This port does not have an over-current condition."]
    OCA_0 = 0,
    #[doc = "1: This port currently has an over-current condition"]
    OCA_1 = 1,
}
impl From<OCA_A> for bool {
    #[inline(always)]
    fn from(variant: OCA_A) -> Self {
        variant as u8 != 0
    }
}
impl OCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCA_A {
        match self.bits {
            false => OCA_A::OCA_0,
            true => OCA_A::OCA_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCA_0`"]
    #[inline(always)]
    pub fn is_oca_0(&self) -> bool {
        *self == OCA_A::OCA_0
    }
    #[doc = "Checks if the value of the field is `OCA_1`"]
    #[inline(always)]
    pub fn is_oca_1(&self) -> bool {
        *self == OCA_A::OCA_1
    }
}
#[doc = "Field `OCC` reader - Over-current Change-R/WC"]
pub type OCC_R = crate::BitReader<bool>;
#[doc = "Field `OCC` writer - Over-current Change-R/WC"]
pub type OCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `FPR` reader - Force Port Resume -Read/Write"]
pub type FPR_R = crate::BitReader<bool>;
#[doc = "Field `FPR` writer - Force Port Resume -Read/Write"]
pub type FPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Suspend - Read/Write or Read Only"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Suspend - Read/Write or Read Only"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PR` reader - Port Reset - Read/Write or Read Only"]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - Port Reset - Read/Write or Read Only"]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `HSP` reader - High-Speed Port - Read Only"]
pub type HSP_R = crate::BitReader<bool>;
#[doc = "Field `LS` reader - Line Status-Read Only"]
pub type LS_R = crate::FieldReader<u8, LS_A>;
#[doc = "Line Status-Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LS_A {
    #[doc = "0: SE0"]
    LS_0 = 0,
    #[doc = "1: K-state"]
    LS_1 = 1,
    #[doc = "2: J-state"]
    LS_2 = 2,
    #[doc = "3: Undefined"]
    LS_3 = 3,
}
impl From<LS_A> for u8 {
    #[inline(always)]
    fn from(variant: LS_A) -> Self {
        variant as _
    }
}
impl LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LS_A {
        match self.bits {
            0 => LS_A::LS_0,
            1 => LS_A::LS_1,
            2 => LS_A::LS_2,
            3 => LS_A::LS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LS_0`"]
    #[inline(always)]
    pub fn is_ls_0(&self) -> bool {
        *self == LS_A::LS_0
    }
    #[doc = "Checks if the value of the field is `LS_1`"]
    #[inline(always)]
    pub fn is_ls_1(&self) -> bool {
        *self == LS_A::LS_1
    }
    #[doc = "Checks if the value of the field is `LS_2`"]
    #[inline(always)]
    pub fn is_ls_2(&self) -> bool {
        *self == LS_A::LS_2
    }
    #[doc = "Checks if the value of the field is `LS_3`"]
    #[inline(always)]
    pub fn is_ls_3(&self) -> bool {
        *self == LS_A::LS_3
    }
}
#[doc = "Field `LS` writer - Line Status-Read Only"]
pub type LS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PORTSC1_SPEC, u8, LS_A, 2, O>;
impl<'a, const O: u8> LS_W<'a, O> {
    #[doc = "SE0"]
    #[inline(always)]
    pub fn ls_0(self) -> &'a mut W {
        self.variant(LS_A::LS_0)
    }
    #[doc = "K-state"]
    #[inline(always)]
    pub fn ls_1(self) -> &'a mut W {
        self.variant(LS_A::LS_1)
    }
    #[doc = "J-state"]
    #[inline(always)]
    pub fn ls_2(self) -> &'a mut W {
        self.variant(LS_A::LS_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn ls_3(self) -> &'a mut W {
        self.variant(LS_A::LS_3)
    }
}
#[doc = "Field `PP` reader - Port Power (PP)-Read/Write or Read Only"]
pub type PP_R = crate::BitReader<bool>;
#[doc = "Field `PP` writer - Port Power (PP)-Read/Write or Read Only"]
pub type PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PO` reader - Port Owner-Read/Write"]
pub type PO_R = crate::BitReader<bool>;
#[doc = "Field `PO` writer - Port Owner-Read/Write"]
pub type PO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PIC` reader - Port Indicator Control - Read/Write"]
pub type PIC_R = crate::FieldReader<u8, PIC_A>;
#[doc = "Port Indicator Control - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIC_A {
    #[doc = "0: Port indicators are off"]
    PIC_0 = 0,
    #[doc = "1: Amber"]
    PIC_1 = 1,
    #[doc = "2: Green"]
    PIC_2 = 2,
    #[doc = "3: Undefined"]
    PIC_3 = 3,
}
impl From<PIC_A> for u8 {
    #[inline(always)]
    fn from(variant: PIC_A) -> Self {
        variant as _
    }
}
impl PIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIC_A {
        match self.bits {
            0 => PIC_A::PIC_0,
            1 => PIC_A::PIC_1,
            2 => PIC_A::PIC_2,
            3 => PIC_A::PIC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIC_0`"]
    #[inline(always)]
    pub fn is_pic_0(&self) -> bool {
        *self == PIC_A::PIC_0
    }
    #[doc = "Checks if the value of the field is `PIC_1`"]
    #[inline(always)]
    pub fn is_pic_1(&self) -> bool {
        *self == PIC_A::PIC_1
    }
    #[doc = "Checks if the value of the field is `PIC_2`"]
    #[inline(always)]
    pub fn is_pic_2(&self) -> bool {
        *self == PIC_A::PIC_2
    }
    #[doc = "Checks if the value of the field is `PIC_3`"]
    #[inline(always)]
    pub fn is_pic_3(&self) -> bool {
        *self == PIC_A::PIC_3
    }
}
#[doc = "Field `PIC` writer - Port Indicator Control - Read/Write"]
pub type PIC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PORTSC1_SPEC, u8, PIC_A, 2, O>;
impl<'a, const O: u8> PIC_W<'a, O> {
    #[doc = "Port indicators are off"]
    #[inline(always)]
    pub fn pic_0(self) -> &'a mut W {
        self.variant(PIC_A::PIC_0)
    }
    #[doc = "Amber"]
    #[inline(always)]
    pub fn pic_1(self) -> &'a mut W {
        self.variant(PIC_A::PIC_1)
    }
    #[doc = "Green"]
    #[inline(always)]
    pub fn pic_2(self) -> &'a mut W {
        self.variant(PIC_A::PIC_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn pic_3(self) -> &'a mut W {
        self.variant(PIC_A::PIC_3)
    }
}
#[doc = "Field `PTC` reader - Port Test Control - Read/Write"]
pub type PTC_R = crate::FieldReader<u8, PTC_A>;
#[doc = "Port Test Control - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTC_A {
    #[doc = "0: TEST_MODE_DISABLE"]
    PTC_0 = 0,
    #[doc = "1: J_STATE"]
    PTC_1 = 1,
    #[doc = "2: K_STATE"]
    PTC_2 = 2,
    #[doc = "3: SE0 (host) / NAK (device)"]
    PTC_3 = 3,
    #[doc = "4: Packet"]
    PTC_4 = 4,
    #[doc = "5: FORCE_ENABLE_HS"]
    PTC_5 = 5,
    #[doc = "6: FORCE_ENABLE_FS"]
    PTC_6 = 6,
    #[doc = "7: FORCE_ENABLE_LS"]
    PTC_7 = 7,
}
impl From<PTC_A> for u8 {
    #[inline(always)]
    fn from(variant: PTC_A) -> Self {
        variant as _
    }
}
impl PTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTC_A> {
        match self.bits {
            0 => Some(PTC_A::PTC_0),
            1 => Some(PTC_A::PTC_1),
            2 => Some(PTC_A::PTC_2),
            3 => Some(PTC_A::PTC_3),
            4 => Some(PTC_A::PTC_4),
            5 => Some(PTC_A::PTC_5),
            6 => Some(PTC_A::PTC_6),
            7 => Some(PTC_A::PTC_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PTC_0`"]
    #[inline(always)]
    pub fn is_ptc_0(&self) -> bool {
        *self == PTC_A::PTC_0
    }
    #[doc = "Checks if the value of the field is `PTC_1`"]
    #[inline(always)]
    pub fn is_ptc_1(&self) -> bool {
        *self == PTC_A::PTC_1
    }
    #[doc = "Checks if the value of the field is `PTC_2`"]
    #[inline(always)]
    pub fn is_ptc_2(&self) -> bool {
        *self == PTC_A::PTC_2
    }
    #[doc = "Checks if the value of the field is `PTC_3`"]
    #[inline(always)]
    pub fn is_ptc_3(&self) -> bool {
        *self == PTC_A::PTC_3
    }
    #[doc = "Checks if the value of the field is `PTC_4`"]
    #[inline(always)]
    pub fn is_ptc_4(&self) -> bool {
        *self == PTC_A::PTC_4
    }
    #[doc = "Checks if the value of the field is `PTC_5`"]
    #[inline(always)]
    pub fn is_ptc_5(&self) -> bool {
        *self == PTC_A::PTC_5
    }
    #[doc = "Checks if the value of the field is `PTC_6`"]
    #[inline(always)]
    pub fn is_ptc_6(&self) -> bool {
        *self == PTC_A::PTC_6
    }
    #[doc = "Checks if the value of the field is `PTC_7`"]
    #[inline(always)]
    pub fn is_ptc_7(&self) -> bool {
        *self == PTC_A::PTC_7
    }
}
#[doc = "Field `PTC` writer - Port Test Control - Read/Write"]
pub type PTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, PTC_A, 4, O>;
impl<'a, const O: u8> PTC_W<'a, O> {
    #[doc = "TEST_MODE_DISABLE"]
    #[inline(always)]
    pub fn ptc_0(self) -> &'a mut W {
        self.variant(PTC_A::PTC_0)
    }
    #[doc = "J_STATE"]
    #[inline(always)]
    pub fn ptc_1(self) -> &'a mut W {
        self.variant(PTC_A::PTC_1)
    }
    #[doc = "K_STATE"]
    #[inline(always)]
    pub fn ptc_2(self) -> &'a mut W {
        self.variant(PTC_A::PTC_2)
    }
    #[doc = "SE0 (host) / NAK (device)"]
    #[inline(always)]
    pub fn ptc_3(self) -> &'a mut W {
        self.variant(PTC_A::PTC_3)
    }
    #[doc = "Packet"]
    #[inline(always)]
    pub fn ptc_4(self) -> &'a mut W {
        self.variant(PTC_A::PTC_4)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline(always)]
    pub fn ptc_5(self) -> &'a mut W {
        self.variant(PTC_A::PTC_5)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline(always)]
    pub fn ptc_6(self) -> &'a mut W {
        self.variant(PTC_A::PTC_6)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline(always)]
    pub fn ptc_7(self) -> &'a mut W {
        self.variant(PTC_A::PTC_7)
    }
}
#[doc = "Field `WKCN` reader - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
pub type WKCN_R = crate::BitReader<bool>;
#[doc = "Field `WKCN` writer - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
pub type WKCN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `WKDC` reader - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
pub type WKDC_R = crate::BitReader<bool>;
#[doc = "Field `WKDC` writer - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
pub type WKDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `WKOC` reader - Wake on Over-current Enable (WKOC_E) - Read/Write"]
pub type WKOC_R = crate::BitReader<bool>;
#[doc = "Field `WKOC` writer - Wake on Over-current Enable (WKOC_E) - Read/Write"]
pub type WKOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PHCD` reader - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
pub type PHCD_R = crate::BitReader<PHCD_A>;
#[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHCD_A {
    #[doc = "0: Enable PHY clock"]
    PHCD_0 = 0,
    #[doc = "1: Disable PHY clock"]
    PHCD_1 = 1,
}
impl From<PHCD_A> for bool {
    #[inline(always)]
    fn from(variant: PHCD_A) -> Self {
        variant as u8 != 0
    }
}
impl PHCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHCD_A {
        match self.bits {
            false => PHCD_A::PHCD_0,
            true => PHCD_A::PHCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHCD_0`"]
    #[inline(always)]
    pub fn is_phcd_0(&self) -> bool {
        *self == PHCD_A::PHCD_0
    }
    #[doc = "Checks if the value of the field is `PHCD_1`"]
    #[inline(always)]
    pub fn is_phcd_1(&self) -> bool {
        *self == PHCD_A::PHCD_1
    }
}
#[doc = "Field `PHCD` writer - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
pub type PHCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, PHCD_A, O>;
impl<'a, const O: u8> PHCD_W<'a, O> {
    #[doc = "Enable PHY clock"]
    #[inline(always)]
    pub fn phcd_0(self) -> &'a mut W {
        self.variant(PHCD_A::PHCD_0)
    }
    #[doc = "Disable PHY clock"]
    #[inline(always)]
    pub fn phcd_1(self) -> &'a mut W {
        self.variant(PHCD_A::PHCD_1)
    }
}
#[doc = "Field `PFSC` reader - Port Force Full Speed Connect - Read/Write"]
pub type PFSC_R = crate::BitReader<PFSC_A>;
#[doc = "Port Force Full Speed Connect - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFSC_A {
    #[doc = "0: Normal operation"]
    PFSC_0 = 0,
    #[doc = "1: Forced to full speed"]
    PFSC_1 = 1,
}
impl From<PFSC_A> for bool {
    #[inline(always)]
    fn from(variant: PFSC_A) -> Self {
        variant as u8 != 0
    }
}
impl PFSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSC_A {
        match self.bits {
            false => PFSC_A::PFSC_0,
            true => PFSC_A::PFSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFSC_0`"]
    #[inline(always)]
    pub fn is_pfsc_0(&self) -> bool {
        *self == PFSC_A::PFSC_0
    }
    #[doc = "Checks if the value of the field is `PFSC_1`"]
    #[inline(always)]
    pub fn is_pfsc_1(&self) -> bool {
        *self == PFSC_A::PFSC_1
    }
}
#[doc = "Field `PFSC` writer - Port Force Full Speed Connect - Read/Write"]
pub type PFSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, PFSC_A, O>;
impl<'a, const O: u8> PFSC_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pfsc_0(self) -> &'a mut W {
        self.variant(PFSC_A::PFSC_0)
    }
    #[doc = "Forced to full speed"]
    #[inline(always)]
    pub fn pfsc_1(self) -> &'a mut W {
        self.variant(PFSC_A::PFSC_1)
    }
}
#[doc = "Field `PTS_2` reader - See description at bits 31-30"]
pub type PTS_2_R = crate::BitReader<bool>;
#[doc = "Field `PTS_2` writer - See description at bits 31-30"]
pub type PTS_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PSPD` reader - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
pub type PSPD_R = crate::FieldReader<u8, PSPD_A>;
#[doc = "Port Speed - Read Only. This register field indicates the speed at which the port is operating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSPD_A {
    #[doc = "0: Full Speed"]
    PSPD_0 = 0,
    #[doc = "1: Low Speed"]
    PSPD_1 = 1,
    #[doc = "2: High Speed"]
    PSPD_2 = 2,
    #[doc = "3: Undefined"]
    PSPD_3 = 3,
}
impl From<PSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PSPD_A) -> Self {
        variant as _
    }
}
impl PSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSPD_A {
        match self.bits {
            0 => PSPD_A::PSPD_0,
            1 => PSPD_A::PSPD_1,
            2 => PSPD_A::PSPD_2,
            3 => PSPD_A::PSPD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSPD_0`"]
    #[inline(always)]
    pub fn is_pspd_0(&self) -> bool {
        *self == PSPD_A::PSPD_0
    }
    #[doc = "Checks if the value of the field is `PSPD_1`"]
    #[inline(always)]
    pub fn is_pspd_1(&self) -> bool {
        *self == PSPD_A::PSPD_1
    }
    #[doc = "Checks if the value of the field is `PSPD_2`"]
    #[inline(always)]
    pub fn is_pspd_2(&self) -> bool {
        *self == PSPD_A::PSPD_2
    }
    #[doc = "Checks if the value of the field is `PSPD_3`"]
    #[inline(always)]
    pub fn is_pspd_3(&self) -> bool {
        *self == PSPD_A::PSPD_3
    }
}
#[doc = "Field `PSPD` writer - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
pub type PSPD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PORTSC1_SPEC, u8, PSPD_A, 2, O>;
impl<'a, const O: u8> PSPD_W<'a, O> {
    #[doc = "Full Speed"]
    #[inline(always)]
    pub fn pspd_0(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_0)
    }
    #[doc = "Low Speed"]
    #[inline(always)]
    pub fn pspd_1(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_1)
    }
    #[doc = "High Speed"]
    #[inline(always)]
    pub fn pspd_2(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn pspd_3(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_3)
    }
}
#[doc = "Field `PTW` reader - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
pub type PTW_R = crate::BitReader<PTW_A>;
#[doc = "Parallel Transceiver Width This bit has no effect if serial interface engine is used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTW_A {
    #[doc = "0: Select the 8-bit UTMI interface \\[60MHz\\]"]
    PTW_0 = 0,
    #[doc = "1: Select the 16-bit UTMI interface \\[30MHz\\]"]
    PTW_1 = 1,
}
impl From<PTW_A> for bool {
    #[inline(always)]
    fn from(variant: PTW_A) -> Self {
        variant as u8 != 0
    }
}
impl PTW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTW_A {
        match self.bits {
            false => PTW_A::PTW_0,
            true => PTW_A::PTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `PTW_0`"]
    #[inline(always)]
    pub fn is_ptw_0(&self) -> bool {
        *self == PTW_A::PTW_0
    }
    #[doc = "Checks if the value of the field is `PTW_1`"]
    #[inline(always)]
    pub fn is_ptw_1(&self) -> bool {
        *self == PTW_A::PTW_1
    }
}
#[doc = "Field `PTW` writer - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
pub type PTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, PTW_A, O>;
impl<'a, const O: u8> PTW_W<'a, O> {
    #[doc = "Select the 8-bit UTMI interface \\[60MHz\\]"]
    #[inline(always)]
    pub fn ptw_0(self) -> &'a mut W {
        self.variant(PTW_A::PTW_0)
    }
    #[doc = "Select the 16-bit UTMI interface \\[30MHz\\]"]
    #[inline(always)]
    pub fn ptw_1(self) -> &'a mut W {
        self.variant(PTW_A::PTW_1)
    }
}
#[doc = "Field `STS` reader - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
pub type STS_R = crate::BitReader<bool>;
#[doc = "Field `STS` writer - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
pub type STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PTS_1` reader - All USB port interface modes are listed in this field description, but not all are supported"]
pub type PTS_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTS_1` writer - All USB port interface modes are listed in this field description, but not all are supported"]
pub type PTS_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Current Connect Status-Read Only"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Over-current Active-Read Only"]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - High-Speed Port - Read Only"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline(always)]
    pub fn po(&self) -> PO_R {
        PO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkcn(&self) -> WKCN_R {
        WKCN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkdc(&self) -> WKDC_R {
        WKDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline(always)]
    pub fn wkoc(&self) -> WKOC_R {
        WKOC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline(always)]
    pub fn phcd(&self) -> PHCD_R {
        PHCD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline(always)]
    pub fn pfsc(&self) -> PFSC_R {
        PFSC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline(always)]
    pub fn pts_2(&self) -> PTS_2_R {
        PTS_2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline(always)]
    pub fn ptw(&self) -> PTW_R {
        PTW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline(always)]
    pub fn pts_1(&self) -> PTS_1_R {
        PTS_1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<1> {
        CSC_W::new(self)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<2> {
        PE_W::new(self)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<3> {
        PEC_W::new(self)
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OCC_W<5> {
        OCC_W::new(self)
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn fpr(&mut self) -> FPR_W<6> {
        FPR_W::new(self)
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<8> {
        PR_W::new(self)
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LS_W<10> {
        LS_W::new(self)
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<12> {
        PP_W::new(self)
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn po(&mut self) -> PO_W<13> {
        PO_W::new(self)
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn pic(&mut self) -> PIC_W<14> {
        PIC_W::new(self)
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn ptc(&mut self) -> PTC_W<16> {
        PTC_W::new(self)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn wkcn(&mut self) -> WKCN_W<20> {
        WKCN_W::new(self)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn wkdc(&mut self) -> WKDC_W<21> {
        WKDC_W::new(self)
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn wkoc(&mut self) -> WKOC_W<22> {
        WKOC_W::new(self)
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn phcd(&mut self) -> PHCD_W<23> {
        PHCD_W::new(self)
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn pfsc(&mut self) -> PFSC_W<24> {
        PFSC_W::new(self)
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline(always)]
    #[must_use]
    pub fn pts_2(&mut self) -> PTS_2_W<25> {
        PTS_2_W::new(self)
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline(always)]
    #[must_use]
    pub fn pspd(&mut self) -> PSPD_W<26> {
        PSPD_W::new(self)
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline(always)]
    #[must_use]
    pub fn ptw(&mut self) -> PTW_W<28> {
        PTW_W::new(self)
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> STS_W<29> {
        STS_W::new(self)
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline(always)]
    #[must_use]
    pub fn pts_1(&mut self) -> PTS_1_W<30> {
        PTS_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Status & Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](index.html) module"]
pub struct PORTSC1_SPEC;
impl crate::RegisterSpec for PORTSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsc1::R](R) reader structure"]
impl crate::Readable for PORTSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsc1::W](W) writer structure"]
impl crate::Writable for PORTSC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTSC1 to value 0x1000_0000"]
impl crate::Resettable for PORTSC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
