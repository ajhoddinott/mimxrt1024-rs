#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter"]
    pub param: PARAM,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Master Control"]
    pub mcr: MCR,
    #[doc = "0x14 - Master Status"]
    pub msr: MSR,
    #[doc = "0x18 - Master Interrupt Enable"]
    pub mier: MIER,
    #[doc = "0x1c - Master DMA Enable"]
    pub mder: MDER,
    #[doc = "0x20 - Master Configuration 0"]
    pub mcfgr0: MCFGR0,
    #[doc = "0x24 - Master Configuration 1"]
    pub mcfgr1: MCFGR1,
    #[doc = "0x28 - Master Configuration 2"]
    pub mcfgr2: MCFGR2,
    #[doc = "0x2c - Master Configuration 3"]
    pub mcfgr3: MCFGR3,
    _reserved10: [u8; 0x10],
    #[doc = "0x40 - Master Data Match"]
    pub mdmr: MDMR,
    _reserved11: [u8; 0x04],
    #[doc = "0x48 - Master Clock Configuration 0"]
    pub mccr0: MCCR0,
    _reserved12: [u8; 0x04],
    #[doc = "0x50 - Master Clock Configuration 1"]
    pub mccr1: MCCR1,
    _reserved13: [u8; 0x04],
    #[doc = "0x58 - Master FIFO Control"]
    pub mfcr: MFCR,
    #[doc = "0x5c - Master FIFO Status"]
    pub mfsr: MFSR,
    #[doc = "0x60 - Master Transmit Data"]
    pub mtdr: MTDR,
    _reserved16: [u8; 0x0c],
    #[doc = "0x70 - Master Receive Data"]
    pub mrdr: MRDR,
    _reserved17: [u8; 0x9c],
    #[doc = "0x110 - Slave Control"]
    pub scr: SCR,
    #[doc = "0x114 - Slave Status"]
    pub ssr: SSR,
    #[doc = "0x118 - Slave Interrupt Enable"]
    pub sier: SIER,
    #[doc = "0x11c - Slave DMA Enable"]
    pub sder: SDER,
    _reserved21: [u8; 0x04],
    #[doc = "0x124 - Slave Configuration 1"]
    pub scfgr1: SCFGR1,
    #[doc = "0x128 - Slave Configuration 2"]
    pub scfgr2: SCFGR2,
    _reserved23: [u8; 0x14],
    #[doc = "0x140 - Slave Address Match"]
    pub samr: SAMR,
    _reserved24: [u8; 0x0c],
    #[doc = "0x150 - Slave Address Status"]
    pub sasr: SASR,
    #[doc = "0x154 - Slave Transmit ACK"]
    pub star: STAR,
    _reserved26: [u8; 0x08],
    #[doc = "0x160 - Slave Transmit Data"]
    pub stdr: STDR,
    _reserved27: [u8; 0x0c],
    #[doc = "0x170 - Slave Receive Data"]
    pub srdr: SRDR,
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter"]
pub mod param;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Master Control"]
pub mod mcr;
#[doc = "MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Master Status"]
pub mod msr;
#[doc = "MIER (rw) register accessor: an alias for `Reg<MIER_SPEC>`"]
pub type MIER = crate::Reg<mier::MIER_SPEC>;
#[doc = "Master Interrupt Enable"]
pub mod mier;
#[doc = "MDER (rw) register accessor: an alias for `Reg<MDER_SPEC>`"]
pub type MDER = crate::Reg<mder::MDER_SPEC>;
#[doc = "Master DMA Enable"]
pub mod mder;
#[doc = "MCFGR0 (rw) register accessor: an alias for `Reg<MCFGR0_SPEC>`"]
pub type MCFGR0 = crate::Reg<mcfgr0::MCFGR0_SPEC>;
#[doc = "Master Configuration 0"]
pub mod mcfgr0;
#[doc = "MCFGR1 (rw) register accessor: an alias for `Reg<MCFGR1_SPEC>`"]
pub type MCFGR1 = crate::Reg<mcfgr1::MCFGR1_SPEC>;
#[doc = "Master Configuration 1"]
pub mod mcfgr1;
#[doc = "MCFGR2 (rw) register accessor: an alias for `Reg<MCFGR2_SPEC>`"]
pub type MCFGR2 = crate::Reg<mcfgr2::MCFGR2_SPEC>;
#[doc = "Master Configuration 2"]
pub mod mcfgr2;
#[doc = "MCFGR3 (rw) register accessor: an alias for `Reg<MCFGR3_SPEC>`"]
pub type MCFGR3 = crate::Reg<mcfgr3::MCFGR3_SPEC>;
#[doc = "Master Configuration 3"]
pub mod mcfgr3;
#[doc = "MDMR (rw) register accessor: an alias for `Reg<MDMR_SPEC>`"]
pub type MDMR = crate::Reg<mdmr::MDMR_SPEC>;
#[doc = "Master Data Match"]
pub mod mdmr;
#[doc = "MCCR0 (rw) register accessor: an alias for `Reg<MCCR0_SPEC>`"]
pub type MCCR0 = crate::Reg<mccr0::MCCR0_SPEC>;
#[doc = "Master Clock Configuration 0"]
pub mod mccr0;
#[doc = "MCCR1 (rw) register accessor: an alias for `Reg<MCCR1_SPEC>`"]
pub type MCCR1 = crate::Reg<mccr1::MCCR1_SPEC>;
#[doc = "Master Clock Configuration 1"]
pub mod mccr1;
#[doc = "MFCR (rw) register accessor: an alias for `Reg<MFCR_SPEC>`"]
pub type MFCR = crate::Reg<mfcr::MFCR_SPEC>;
#[doc = "Master FIFO Control"]
pub mod mfcr;
#[doc = "MFSR (r) register accessor: an alias for `Reg<MFSR_SPEC>`"]
pub type MFSR = crate::Reg<mfsr::MFSR_SPEC>;
#[doc = "Master FIFO Status"]
pub mod mfsr;
#[doc = "MTDR (w) register accessor: an alias for `Reg<MTDR_SPEC>`"]
pub type MTDR = crate::Reg<mtdr::MTDR_SPEC>;
#[doc = "Master Transmit Data"]
pub mod mtdr;
#[doc = "MRDR (r) register accessor: an alias for `Reg<MRDR_SPEC>`"]
pub type MRDR = crate::Reg<mrdr::MRDR_SPEC>;
#[doc = "Master Receive Data"]
pub mod mrdr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Slave Control"]
pub mod scr;
#[doc = "SSR (rw) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Slave Status"]
pub mod ssr;
#[doc = "SIER (rw) register accessor: an alias for `Reg<SIER_SPEC>`"]
pub type SIER = crate::Reg<sier::SIER_SPEC>;
#[doc = "Slave Interrupt Enable"]
pub mod sier;
#[doc = "SDER (rw) register accessor: an alias for `Reg<SDER_SPEC>`"]
pub type SDER = crate::Reg<sder::SDER_SPEC>;
#[doc = "Slave DMA Enable"]
pub mod sder;
#[doc = "SCFGR1 (rw) register accessor: an alias for `Reg<SCFGR1_SPEC>`"]
pub type SCFGR1 = crate::Reg<scfgr1::SCFGR1_SPEC>;
#[doc = "Slave Configuration 1"]
pub mod scfgr1;
#[doc = "SCFGR2 (rw) register accessor: an alias for `Reg<SCFGR2_SPEC>`"]
pub type SCFGR2 = crate::Reg<scfgr2::SCFGR2_SPEC>;
#[doc = "Slave Configuration 2"]
pub mod scfgr2;
#[doc = "SAMR (rw) register accessor: an alias for `Reg<SAMR_SPEC>`"]
pub type SAMR = crate::Reg<samr::SAMR_SPEC>;
#[doc = "Slave Address Match"]
pub mod samr;
#[doc = "SASR (r) register accessor: an alias for `Reg<SASR_SPEC>`"]
pub type SASR = crate::Reg<sasr::SASR_SPEC>;
#[doc = "Slave Address Status"]
pub mod sasr;
#[doc = "STAR (rw) register accessor: an alias for `Reg<STAR_SPEC>`"]
pub type STAR = crate::Reg<star::STAR_SPEC>;
#[doc = "Slave Transmit ACK"]
pub mod star;
#[doc = "STDR (w) register accessor: an alias for `Reg<STDR_SPEC>`"]
pub type STDR = crate::Reg<stdr::STDR_SPEC>;
#[doc = "Slave Transmit Data"]
pub mod stdr;
#[doc = "SRDR (r) register accessor: an alias for `Reg<SRDR_SPEC>`"]
pub type SRDR = crate::Reg<srdr::SRDR_SPEC>;
#[doc = "Slave Receive Data"]
pub mod srdr;
