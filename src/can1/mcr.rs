#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXMB` reader - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
pub type MAXMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXMB` writer - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
pub type MAXMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `IDAM` reader - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
pub type IDAM_R = crate::FieldReader<u8, IDAM_A>;
#[doc = "This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Format A One full ID (standard or extended) per ID filter Table element."]
    IDAM_0 = 0,
    #[doc = "1: Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    IDAM_1 = 1,
    #[doc = "2: Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    IDAM_2 = 2,
    #[doc = "3: Format D All frames rejected."]
    IDAM_3 = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
impl IDAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::IDAM_0,
            1 => IDAM_A::IDAM_1,
            2 => IDAM_A::IDAM_2,
            3 => IDAM_A::IDAM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDAM_0`"]
    #[inline(always)]
    pub fn is_idam_0(&self) -> bool {
        *self == IDAM_A::IDAM_0
    }
    #[doc = "Checks if the value of the field is `IDAM_1`"]
    #[inline(always)]
    pub fn is_idam_1(&self) -> bool {
        *self == IDAM_A::IDAM_1
    }
    #[doc = "Checks if the value of the field is `IDAM_2`"]
    #[inline(always)]
    pub fn is_idam_2(&self) -> bool {
        *self == IDAM_A::IDAM_2
    }
    #[doc = "Checks if the value of the field is `IDAM_3`"]
    #[inline(always)]
    pub fn is_idam_3(&self) -> bool {
        *self == IDAM_A::IDAM_3
    }
}
#[doc = "Field `IDAM` writer - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
pub type IDAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, IDAM_A, 2, O>;
impl<'a, const O: u8> IDAM_W<'a, O> {
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    #[inline(always)]
    pub fn idam_0(self) -> &'a mut W {
        self.variant(IDAM_A::IDAM_0)
    }
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    #[inline(always)]
    pub fn idam_1(self) -> &'a mut W {
        self.variant(IDAM_A::IDAM_1)
    }
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    #[inline(always)]
    pub fn idam_2(self) -> &'a mut W {
        self.variant(IDAM_A::IDAM_2)
    }
    #[doc = "Format D All frames rejected."]
    #[inline(always)]
    pub fn idam_3(self) -> &'a mut W {
        self.variant(IDAM_A::IDAM_3)
    }
}
#[doc = "Field `AEN` reader - This bit is supplied for backwards compatibility reasons"]
pub type AEN_R = crate::BitReader<AEN_A>;
#[doc = "This bit is supplied for backwards compatibility reasons\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled"]
    AEN_0 = 0,
    #[doc = "1: Abort enabled"]
    AEN_1 = 1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::AEN_0,
            true => AEN_A::AEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AEN_0`"]
    #[inline(always)]
    pub fn is_aen_0(&self) -> bool {
        *self == AEN_A::AEN_0
    }
    #[doc = "Checks if the value of the field is `AEN_1`"]
    #[inline(always)]
    pub fn is_aen_1(&self) -> bool {
        *self == AEN_A::AEN_1
    }
}
#[doc = "Field `AEN` writer - This bit is supplied for backwards compatibility reasons"]
pub type AEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, AEN_A, O>;
impl<'a, const O: u8> AEN_W<'a, O> {
    #[doc = "Abort disabled"]
    #[inline(always)]
    pub fn aen_0(self) -> &'a mut W {
        self.variant(AEN_A::AEN_0)
    }
    #[doc = "Abort enabled"]
    #[inline(always)]
    pub fn aen_1(self) -> &'a mut W {
        self.variant(AEN_A::AEN_1)
    }
}
#[doc = "Field `LPRIOEN` reader - This bit is provided for backwards compatibility reasons"]
pub type LPRIOEN_R = crate::BitReader<LPRIOEN_A>;
#[doc = "This bit is provided for backwards compatibility reasons\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled"]
    LPRIOEN_0 = 0,
    #[doc = "1: Local Priority enabled"]
    LPRIOEN_1 = 1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPRIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::LPRIOEN_0,
            true => LPRIOEN_A::LPRIOEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPRIOEN_0`"]
    #[inline(always)]
    pub fn is_lprioen_0(&self) -> bool {
        *self == LPRIOEN_A::LPRIOEN_0
    }
    #[doc = "Checks if the value of the field is `LPRIOEN_1`"]
    #[inline(always)]
    pub fn is_lprioen_1(&self) -> bool {
        *self == LPRIOEN_A::LPRIOEN_1
    }
}
#[doc = "Field `LPRIOEN` writer - This bit is provided for backwards compatibility reasons"]
pub type LPRIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, LPRIOEN_A, O>;
impl<'a, const O: u8> LPRIOEN_W<'a, O> {
    #[doc = "Local Priority disabled"]
    #[inline(always)]
    pub fn lprioen_0(self) -> &'a mut W {
        self.variant(LPRIOEN_A::LPRIOEN_0)
    }
    #[doc = "Local Priority enabled"]
    #[inline(always)]
    pub fn lprioen_1(self) -> &'a mut W {
        self.variant(LPRIOEN_A::LPRIOEN_1)
    }
}
#[doc = "Field `IRMQ` reader - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
pub type IRMQ_R = crate::BitReader<IRMQ_A>;
#[doc = "This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    IRMQ_0 = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    IRMQ_1 = 1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IRMQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::IRMQ_0,
            true => IRMQ_A::IRMQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `IRMQ_0`"]
    #[inline(always)]
    pub fn is_irmq_0(&self) -> bool {
        *self == IRMQ_A::IRMQ_0
    }
    #[doc = "Checks if the value of the field is `IRMQ_1`"]
    #[inline(always)]
    pub fn is_irmq_1(&self) -> bool {
        *self == IRMQ_A::IRMQ_1
    }
}
#[doc = "Field `IRMQ` writer - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
pub type IRMQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, IRMQ_A, O>;
impl<'a, const O: u8> IRMQ_W<'a, O> {
    #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn irmq_0(self) -> &'a mut W {
        self.variant(IRMQ_A::IRMQ_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn irmq_1(self) -> &'a mut W {
        self.variant(IRMQ_A::IRMQ_1)
    }
}
#[doc = "Field `SRXDIS` reader - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
pub type SRXDIS_R = crate::BitReader<SRXDIS_A>;
#[doc = "This bit defines whether FlexCAN is allowed to receive frames transmitted by itself\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRXDIS_A {
    #[doc = "0: Self reception enabled"]
    SRXDIS_0 = 0,
    #[doc = "1: Self reception disabled"]
    SRXDIS_1 = 1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRXDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::SRXDIS_0,
            true => SRXDIS_A::SRXDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRXDIS_0`"]
    #[inline(always)]
    pub fn is_srxdis_0(&self) -> bool {
        *self == SRXDIS_A::SRXDIS_0
    }
    #[doc = "Checks if the value of the field is `SRXDIS_1`"]
    #[inline(always)]
    pub fn is_srxdis_1(&self) -> bool {
        *self == SRXDIS_A::SRXDIS_1
    }
}
#[doc = "Field `SRXDIS` writer - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
pub type SRXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SRXDIS_A, O>;
impl<'a, const O: u8> SRXDIS_W<'a, O> {
    #[doc = "Self reception enabled"]
    #[inline(always)]
    pub fn srxdis_0(self) -> &'a mut W {
        self.variant(SRXDIS_A::SRXDIS_0)
    }
    #[doc = "Self reception disabled"]
    #[inline(always)]
    pub fn srxdis_1(self) -> &'a mut W {
        self.variant(SRXDIS_A::SRXDIS_1)
    }
}
#[doc = "Field `WAKSRC` reader - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
pub type WAKSRC_R = crate::BitReader<WAKSRC_A>;
#[doc = "This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKSRC_A {
    #[doc = "0: FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    WAKSRC_0 = 0,
    #[doc = "1: FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
    WAKSRC_1 = 1,
}
impl From<WAKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: WAKSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKSRC_A {
        match self.bits {
            false => WAKSRC_A::WAKSRC_0,
            true => WAKSRC_A::WAKSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKSRC_0`"]
    #[inline(always)]
    pub fn is_waksrc_0(&self) -> bool {
        *self == WAKSRC_A::WAKSRC_0
    }
    #[doc = "Checks if the value of the field is `WAKSRC_1`"]
    #[inline(always)]
    pub fn is_waksrc_1(&self) -> bool {
        *self == WAKSRC_A::WAKSRC_1
    }
}
#[doc = "Field `WAKSRC` writer - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
pub type WAKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WAKSRC_A, O>;
impl<'a, const O: u8> WAKSRC_W<'a, O> {
    #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    #[inline(always)]
    pub fn waksrc_0(self) -> &'a mut W {
        self.variant(WAKSRC_A::WAKSRC_0)
    }
    #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus"]
    #[inline(always)]
    pub fn waksrc_1(self) -> &'a mut W {
        self.variant(WAKSRC_A::WAKSRC_1)
    }
}
#[doc = "Field `LPMACK` reader - This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
pub type LPMACK_R = crate::BitReader<LPMACK_A>;
#[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK_A {
    #[doc = "0: FLEXCAN not in any of the low power modes"]
    LPMACK_0 = 0,
    #[doc = "1: FLEXCAN is either in Disable Mode, or Stop mode"]
    LPMACK_1 = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPMACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::LPMACK_0,
            true => LPMACK_A::LPMACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPMACK_0`"]
    #[inline(always)]
    pub fn is_lpmack_0(&self) -> bool {
        *self == LPMACK_A::LPMACK_0
    }
    #[doc = "Checks if the value of the field is `LPMACK_1`"]
    #[inline(always)]
    pub fn is_lpmack_1(&self) -> bool {
        *self == LPMACK_A::LPMACK_1
    }
}
#[doc = "Field `WRNEN` reader - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
pub type WRNEN_R = crate::BitReader<WRNEN_A>;
#[doc = "When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRNEN_A {
    #[doc = "0: TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    WRNEN_0 = 0,
    #[doc = "1: TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    WRNEN_1 = 1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WRNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::WRNEN_0,
            true => WRNEN_A::WRNEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WRNEN_0`"]
    #[inline(always)]
    pub fn is_wrnen_0(&self) -> bool {
        *self == WRNEN_A::WRNEN_0
    }
    #[doc = "Checks if the value of the field is `WRNEN_1`"]
    #[inline(always)]
    pub fn is_wrnen_1(&self) -> bool {
        *self == WRNEN_A::WRNEN_1
    }
}
#[doc = "Field `WRNEN` writer - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
pub type WRNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WRNEN_A, O>;
impl<'a, const O: u8> WRNEN_W<'a, O> {
    #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn wrnen_0(self) -> &'a mut W {
        self.variant(WRNEN_A::WRNEN_0)
    }
    #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    #[inline(always)]
    pub fn wrnen_1(self) -> &'a mut W {
        self.variant(WRNEN_A::WRNEN_1)
    }
}
#[doc = "Field `SLFWAK` reader - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
pub type SLFWAK_R = crate::BitReader<SLFWAK_A>;
#[doc = "This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLFWAK_A {
    #[doc = "0: FLEXCAN Self Wake Up feature is disabled"]
    SLFWAK_0 = 0,
    #[doc = "1: FLEXCAN Self Wake Up feature is enabled"]
    SLFWAK_1 = 1,
}
impl From<SLFWAK_A> for bool {
    #[inline(always)]
    fn from(variant: SLFWAK_A) -> Self {
        variant as u8 != 0
    }
}
impl SLFWAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLFWAK_A {
        match self.bits {
            false => SLFWAK_A::SLFWAK_0,
            true => SLFWAK_A::SLFWAK_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLFWAK_0`"]
    #[inline(always)]
    pub fn is_slfwak_0(&self) -> bool {
        *self == SLFWAK_A::SLFWAK_0
    }
    #[doc = "Checks if the value of the field is `SLFWAK_1`"]
    #[inline(always)]
    pub fn is_slfwak_1(&self) -> bool {
        *self == SLFWAK_A::SLFWAK_1
    }
}
#[doc = "Field `SLFWAK` writer - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
pub type SLFWAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SLFWAK_A, O>;
impl<'a, const O: u8> SLFWAK_W<'a, O> {
    #[doc = "FLEXCAN Self Wake Up feature is disabled"]
    #[inline(always)]
    pub fn slfwak_0(self) -> &'a mut W {
        self.variant(SLFWAK_A::SLFWAK_0)
    }
    #[doc = "FLEXCAN Self Wake Up feature is enabled"]
    #[inline(always)]
    pub fn slfwak_1(self) -> &'a mut W {
        self.variant(SLFWAK_A::SLFWAK_1)
    }
}
#[doc = "Field `SUPV` reader - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
pub type SUPV_R = crate::BitReader<SUPV_A>;
#[doc = "This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUPV_A {
    #[doc = "0: FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
    SUPV_0 = 0,
    #[doc = "1: FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
    SUPV_1 = 1,
}
impl From<SUPV_A> for bool {
    #[inline(always)]
    fn from(variant: SUPV_A) -> Self {
        variant as u8 != 0
    }
}
impl SUPV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPV_A {
        match self.bits {
            false => SUPV_A::SUPV_0,
            true => SUPV_A::SUPV_1,
        }
    }
    #[doc = "Checks if the value of the field is `SUPV_0`"]
    #[inline(always)]
    pub fn is_supv_0(&self) -> bool {
        *self == SUPV_A::SUPV_0
    }
    #[doc = "Checks if the value of the field is `SUPV_1`"]
    #[inline(always)]
    pub fn is_supv_1(&self) -> bool {
        *self == SUPV_A::SUPV_1
    }
}
#[doc = "Field `SUPV` writer - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
pub type SUPV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SUPV_A, O>;
impl<'a, const O: u8> SUPV_W<'a, O> {
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses"]
    #[inline(always)]
    pub fn supv_0(self) -> &'a mut W {
        self.variant(SUPV_A::SUPV_0)
    }
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location"]
    #[inline(always)]
    pub fn supv_1(self) -> &'a mut W {
        self.variant(SUPV_A::SUPV_1)
    }
}
#[doc = "Field `FRZACK` reader - This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
pub type FRZACK_R = crate::BitReader<FRZACK_A>;
#[doc = "This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZACK_A {
    #[doc = "0: FLEXCAN not in Freeze Mode, prescaler running"]
    FRZACK_0 = 0,
    #[doc = "1: FLEXCAN in Freeze Mode, prescaler stopped"]
    FRZACK_1 = 1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::FRZACK_0,
            true => FRZACK_A::FRZACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZACK_0`"]
    #[inline(always)]
    pub fn is_frzack_0(&self) -> bool {
        *self == FRZACK_A::FRZACK_0
    }
    #[doc = "Checks if the value of the field is `FRZACK_1`"]
    #[inline(always)]
    pub fn is_frzack_1(&self) -> bool {
        *self == FRZACK_A::FRZACK_1
    }
}
#[doc = "Field `SOFTRST` reader - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
pub type SOFTRST_R = crate::BitReader<SOFTRST_A>;
#[doc = "When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request"]
    SOFTRST_0 = 0,
    #[doc = "1: Reset the registers"]
    SOFTRST_1 = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::SOFTRST_0,
            true => SOFTRST_A::SOFTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTRST_0`"]
    #[inline(always)]
    pub fn is_softrst_0(&self) -> bool {
        *self == SOFTRST_A::SOFTRST_0
    }
    #[doc = "Checks if the value of the field is `SOFTRST_1`"]
    #[inline(always)]
    pub fn is_softrst_1(&self) -> bool {
        *self == SOFTRST_A::SOFTRST_1
    }
}
#[doc = "Field `SOFTRST` writer - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
pub type SOFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SOFTRST_A, O>;
impl<'a, const O: u8> SOFTRST_W<'a, O> {
    #[doc = "No reset request"]
    #[inline(always)]
    pub fn softrst_0(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_0)
    }
    #[doc = "Reset the registers"]
    #[inline(always)]
    pub fn softrst_1(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_1)
    }
}
#[doc = "Field `WAKMSK` reader - This bit enables the Wake Up Interrupt generation."]
pub type WAKMSK_R = crate::BitReader<WAKMSK_A>;
#[doc = "This bit enables the Wake Up Interrupt generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKMSK_A {
    #[doc = "0: Wake Up Interrupt is disabled"]
    WAKMSK_0 = 0,
    #[doc = "1: Wake Up Interrupt is enabled"]
    WAKMSK_1 = 1,
}
impl From<WAKMSK_A> for bool {
    #[inline(always)]
    fn from(variant: WAKMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKMSK_A {
        match self.bits {
            false => WAKMSK_A::WAKMSK_0,
            true => WAKMSK_A::WAKMSK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAKMSK_0`"]
    #[inline(always)]
    pub fn is_wakmsk_0(&self) -> bool {
        *self == WAKMSK_A::WAKMSK_0
    }
    #[doc = "Checks if the value of the field is `WAKMSK_1`"]
    #[inline(always)]
    pub fn is_wakmsk_1(&self) -> bool {
        *self == WAKMSK_A::WAKMSK_1
    }
}
#[doc = "Field `WAKMSK` writer - This bit enables the Wake Up Interrupt generation."]
pub type WAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WAKMSK_A, O>;
impl<'a, const O: u8> WAKMSK_W<'a, O> {
    #[doc = "Wake Up Interrupt is disabled"]
    #[inline(always)]
    pub fn wakmsk_0(self) -> &'a mut W {
        self.variant(WAKMSK_A::WAKMSK_0)
    }
    #[doc = "Wake Up Interrupt is enabled"]
    #[inline(always)]
    pub fn wakmsk_1(self) -> &'a mut W {
        self.variant(WAKMSK_A::WAKMSK_1)
    }
}
#[doc = "Field `NOTRDY` reader - This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
pub type NOTRDY_R = crate::BitReader<NOTRDY_A>;
#[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTRDY_A {
    #[doc = "0: FLEXCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode"]
    NOTRDY_0 = 0,
    #[doc = "1: FLEXCAN module is either in Disable Mode, Stop Mode or Freeze Mode"]
    NOTRDY_1 = 1,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRDY_A {
        match self.bits {
            false => NOTRDY_A::NOTRDY_0,
            true => NOTRDY_A::NOTRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRDY_0`"]
    #[inline(always)]
    pub fn is_notrdy_0(&self) -> bool {
        *self == NOTRDY_A::NOTRDY_0
    }
    #[doc = "Checks if the value of the field is `NOTRDY_1`"]
    #[inline(always)]
    pub fn is_notrdy_1(&self) -> bool {
        *self == NOTRDY_A::NOTRDY_1
    }
}
#[doc = "Field `HALT` reader - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Assertion of this bit puts the FLEXCAN module into Freeze Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: No Freeze Mode request."]
    HALT_0 = 0,
    #[doc = "1: Enters Freeze Mode if the FRZ bit is asserted."]
    HALT_1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::HALT_0,
            true => HALT_A::HALT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_0`"]
    #[inline(always)]
    pub fn is_halt_0(&self) -> bool {
        *self == HALT_A::HALT_0
    }
    #[doc = "Checks if the value of the field is `HALT_1`"]
    #[inline(always)]
    pub fn is_halt_1(&self) -> bool {
        *self == HALT_A::HALT_1
    }
}
#[doc = "Field `HALT` writer - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "No Freeze Mode request."]
    #[inline(always)]
    pub fn halt_0(self) -> &'a mut W {
        self.variant(HALT_A::HALT_0)
    }
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn halt_1(self) -> &'a mut W {
        self.variant(HALT_A::HALT_1)
    }
}
#[doc = "Field `RFEN` reader - This bit controls whether the Rx FIFO feature is enabled or not"]
pub type RFEN_R = crate::BitReader<RFEN_A>;
#[doc = "This bit controls whether the Rx FIFO feature is enabled or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEN_A {
    #[doc = "0: FIFO not enabled"]
    RFEN_0 = 0,
    #[doc = "1: FIFO enabled"]
    RFEN_1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::RFEN_0,
            true => RFEN_A::RFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RFEN_0`"]
    #[inline(always)]
    pub fn is_rfen_0(&self) -> bool {
        *self == RFEN_A::RFEN_0
    }
    #[doc = "Checks if the value of the field is `RFEN_1`"]
    #[inline(always)]
    pub fn is_rfen_1(&self) -> bool {
        *self == RFEN_A::RFEN_1
    }
}
#[doc = "Field `RFEN` writer - This bit controls whether the Rx FIFO feature is enabled or not"]
pub type RFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RFEN_A, O>;
impl<'a, const O: u8> RFEN_W<'a, O> {
    #[doc = "FIFO not enabled"]
    #[inline(always)]
    pub fn rfen_0(self) -> &'a mut W {
        self.variant(RFEN_A::RFEN_0)
    }
    #[doc = "FIFO enabled"]
    #[inline(always)]
    pub fn rfen_1(self) -> &'a mut W {
        self.variant(RFEN_A::RFEN_1)
    }
}
#[doc = "Field `FRZ` reader - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
pub type FRZ_R = crate::BitReader<FRZ_A>;
#[doc = "The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze Mode"]
    FRZ_0 = 0,
    #[doc = "1: Enabled to enter Freeze Mode"]
    FRZ_1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::FRZ_0,
            true => FRZ_A::FRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZ_0`"]
    #[inline(always)]
    pub fn is_frz_0(&self) -> bool {
        *self == FRZ_A::FRZ_0
    }
    #[doc = "Checks if the value of the field is `FRZ_1`"]
    #[inline(always)]
    pub fn is_frz_1(&self) -> bool {
        *self == FRZ_A::FRZ_1
    }
}
#[doc = "Field `FRZ` writer - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
pub type FRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, FRZ_A, O>;
impl<'a, const O: u8> FRZ_W<'a, O> {
    #[doc = "Not enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn frz_0(self) -> &'a mut W {
        self.variant(FRZ_A::FRZ_0)
    }
    #[doc = "Enabled to enter Freeze Mode"]
    #[inline(always)]
    pub fn frz_1(self) -> &'a mut W {
        self.variant(FRZ_A::FRZ_1)
    }
}
#[doc = "Field `MDIS` reader - This bit controls whether FLEXCAN is enabled or not"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "This bit controls whether FLEXCAN is enabled or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FLEXCAN module"]
    MDIS_0 = 0,
    #[doc = "1: Disable the FLEXCAN module"]
    MDIS_1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::MDIS_0,
            true => MDIS_A::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline(always)]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDIS_A::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline(always)]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDIS_A::MDIS_1
    }
}
#[doc = "Field `MDIS` writer - This bit controls whether FLEXCAN is enabled or not"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "Enable the FLEXCAN module"]
    #[inline(always)]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_0)
    }
    #[doc = "Disable the FLEXCAN module"]
    #[inline(always)]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - This bit is supplied for backwards compatibility reasons"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit is provided for backwards compatibility reasons"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[inline(always)]
    pub fn waksrc(&self) -> WAKSRC_R {
        WAKSRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[inline(always)]
    pub fn slfwak(&self) -> SLFWAK_R {
        SLFWAK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit enables the Wake Up Interrupt generation."]
    #[inline(always)]
    pub fn wakmsk(&self) -> WAKMSK_R {
        WAKMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit controls whether the Rx FIFO feature is enabled or not"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit controls whether FLEXCAN is enabled or not"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[inline(always)]
    #[must_use]
    pub fn maxmb(&mut self) -> MAXMB_W<0> {
        MAXMB_W::new(self)
    }
    #[doc = "Bits 8:9 - This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[inline(always)]
    #[must_use]
    pub fn idam(&mut self) -> IDAM_W<8> {
        IDAM_W::new(self)
    }
    #[doc = "Bit 12 - This bit is supplied for backwards compatibility reasons"]
    #[inline(always)]
    #[must_use]
    pub fn aen(&mut self) -> AEN_W<12> {
        AEN_W::new(self)
    }
    #[doc = "Bit 13 - This bit is provided for backwards compatibility reasons"]
    #[inline(always)]
    #[must_use]
    pub fn lprioen(&mut self) -> LPRIOEN_W<13> {
        LPRIOEN_W::new(self)
    }
    #[doc = "Bit 16 - This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[inline(always)]
    #[must_use]
    pub fn irmq(&mut self) -> IRMQ_W<16> {
        IRMQ_W::new(self)
    }
    #[doc = "Bit 17 - This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[inline(always)]
    #[must_use]
    pub fn srxdis(&mut self) -> SRXDIS_W<17> {
        SRXDIS_W::new(self)
    }
    #[doc = "Bit 19 - This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[inline(always)]
    #[must_use]
    pub fn waksrc(&mut self) -> WAKSRC_W<19> {
        WAKSRC_W::new(self)
    }
    #[doc = "Bit 21 - When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[inline(always)]
    #[must_use]
    pub fn wrnen(&mut self) -> WRNEN_W<21> {
        WRNEN_W::new(self)
    }
    #[doc = "Bit 22 - This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn slfwak(&mut self) -> SLFWAK_W<22> {
        SLFWAK_W::new(self)
    }
    #[doc = "Bit 23 - This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[inline(always)]
    #[must_use]
    pub fn supv(&mut self) -> SUPV_W<23> {
        SUPV_W::new(self)
    }
    #[doc = "Bit 25 - When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[inline(always)]
    #[must_use]
    pub fn softrst(&mut self) -> SOFTRST_W<25> {
        SOFTRST_W::new(self)
    }
    #[doc = "Bit 26 - This bit enables the Wake Up Interrupt generation."]
    #[inline(always)]
    #[must_use]
    pub fn wakmsk(&mut self) -> WAKMSK_W<26> {
        WAKMSK_W::new(self)
    }
    #[doc = "Bit 28 - Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<28> {
        HALT_W::new(self)
    }
    #[doc = "Bit 29 - This bit controls whether the Rx FIFO feature is enabled or not"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<29> {
        RFEN_W::new(self)
    }
    #[doc = "Bit 30 - The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FRZ_W<30> {
        FRZ_W::new(self)
    }
    #[doc = "Bit 31 - This bit controls whether FLEXCAN is enabled or not"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<31> {
        MDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0x5980_000f"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x5980_000f;
}
