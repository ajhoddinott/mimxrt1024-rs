#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPDIF Configuration Register"]
    pub scr: SCR,
    #[doc = "0x04 - CDText Control Register"]
    pub srcd: SRCD,
    #[doc = "0x08 - PhaseConfig Register"]
    pub srpc: SRPC,
    #[doc = "0x0c - InterruptEn Register"]
    pub sie: SIE,
    _reserved_4_sic_sis: [u8; 0x04],
    #[doc = "0x14 - SPDIFRxLeft Register"]
    pub srl: SRL,
    #[doc = "0x18 - SPDIFRxRight Register"]
    pub srr: SRR,
    #[doc = "0x1c - SPDIFRxCChannel_h Register"]
    pub srcsh: SRCSH,
    #[doc = "0x20 - SPDIFRxCChannel_l Register"]
    pub srcsl: SRCSL,
    #[doc = "0x24 - UchannelRx Register"]
    pub sru: SRU,
    #[doc = "0x28 - QchannelRx Register"]
    pub srq: SRQ,
    #[doc = "0x2c - SPDIFTxLeft Register"]
    pub stl: STL,
    #[doc = "0x30 - SPDIFTxRight Register"]
    pub str: STR,
    #[doc = "0x34 - SPDIFTxCChannelCons_h Register"]
    pub stcsch: STCSCH,
    #[doc = "0x38 - SPDIFTxCChannelCons_l Register"]
    pub stcscl: STCSCL,
    _reserved15: [u8; 0x08],
    #[doc = "0x44 - FreqMeas Register"]
    pub srfm: SRFM,
    _reserved16: [u8; 0x08],
    #[doc = "0x50 - SPDIFTxClk Register"]
    pub stc: STC,
}
impl RegisterBlock {
    #[doc = "0x10 - InterruptStat Register"]
    #[inline(always)]
    pub const fn sic_sis_sis(&self) -> &SIC_SIS_SIS {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - InterruptClear Register"]
    #[inline(always)]
    pub const fn sic_sis_sic(&self) -> &SIC_SIS_SIC {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
}
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "SPDIF Configuration Register"]
pub mod scr;
#[doc = "SRCD (rw) register accessor: an alias for `Reg<SRCD_SPEC>`"]
pub type SRCD = crate::Reg<srcd::SRCD_SPEC>;
#[doc = "CDText Control Register"]
pub mod srcd;
#[doc = "SRPC (rw) register accessor: an alias for `Reg<SRPC_SPEC>`"]
pub type SRPC = crate::Reg<srpc::SRPC_SPEC>;
#[doc = "PhaseConfig Register"]
pub mod srpc;
#[doc = "SIE (rw) register accessor: an alias for `Reg<SIE_SPEC>`"]
pub type SIE = crate::Reg<sie::SIE_SPEC>;
#[doc = "InterruptEn Register"]
pub mod sie;
#[doc = "SIC_SIS_SIC (w) register accessor: an alias for `Reg<SIC_SIS_SIC_SPEC>`"]
pub type SIC_SIS_SIC = crate::Reg<sic_sis_sic::SIC_SIS_SIC_SPEC>;
#[doc = "InterruptClear Register"]
pub mod sic_sis_sic;
#[doc = "SIC_SIS_SIS (r) register accessor: an alias for `Reg<SIC_SIS_SIS_SPEC>`"]
pub type SIC_SIS_SIS = crate::Reg<sic_sis_sis::SIC_SIS_SIS_SPEC>;
#[doc = "InterruptStat Register"]
pub mod sic_sis_sis;
#[doc = "SRL (r) register accessor: an alias for `Reg<SRL_SPEC>`"]
pub type SRL = crate::Reg<srl::SRL_SPEC>;
#[doc = "SPDIFRxLeft Register"]
pub mod srl;
#[doc = "SRR (r) register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "SPDIFRxRight Register"]
pub mod srr;
#[doc = "SRCSH (r) register accessor: an alias for `Reg<SRCSH_SPEC>`"]
pub type SRCSH = crate::Reg<srcsh::SRCSH_SPEC>;
#[doc = "SPDIFRxCChannel_h Register"]
pub mod srcsh;
#[doc = "SRCSL (r) register accessor: an alias for `Reg<SRCSL_SPEC>`"]
pub type SRCSL = crate::Reg<srcsl::SRCSL_SPEC>;
#[doc = "SPDIFRxCChannel_l Register"]
pub mod srcsl;
#[doc = "SRU (r) register accessor: an alias for `Reg<SRU_SPEC>`"]
pub type SRU = crate::Reg<sru::SRU_SPEC>;
#[doc = "UchannelRx Register"]
pub mod sru;
#[doc = "SRQ (r) register accessor: an alias for `Reg<SRQ_SPEC>`"]
pub type SRQ = crate::Reg<srq::SRQ_SPEC>;
#[doc = "QchannelRx Register"]
pub mod srq;
#[doc = "STL (w) register accessor: an alias for `Reg<STL_SPEC>`"]
pub type STL = crate::Reg<stl::STL_SPEC>;
#[doc = "SPDIFTxLeft Register"]
pub mod stl;
#[doc = "STR (w) register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "SPDIFTxRight Register"]
pub mod str;
#[doc = "STCSCH (rw) register accessor: an alias for `Reg<STCSCH_SPEC>`"]
pub type STCSCH = crate::Reg<stcsch::STCSCH_SPEC>;
#[doc = "SPDIFTxCChannelCons_h Register"]
pub mod stcsch;
#[doc = "STCSCL (rw) register accessor: an alias for `Reg<STCSCL_SPEC>`"]
pub type STCSCL = crate::Reg<stcscl::STCSCL_SPEC>;
#[doc = "SPDIFTxCChannelCons_l Register"]
pub mod stcscl;
#[doc = "SRFM (r) register accessor: an alias for `Reg<SRFM_SPEC>`"]
pub type SRFM = crate::Reg<srfm::SRFM_SPEC>;
#[doc = "FreqMeas Register"]
pub mod srfm;
#[doc = "STC (rw) register accessor: an alias for `Reg<STC_SPEC>`"]
pub type STC = crate::Reg<stc::STC_SPEC>;
#[doc = "SPDIFTxClk Register"]
pub mod stc;
