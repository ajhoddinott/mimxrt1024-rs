#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter"]
    pub param: PARAM,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Control"]
    pub cr: CR,
    #[doc = "0x14 - Status"]
    pub sr: SR,
    #[doc = "0x18 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x1c - DMA Enable"]
    pub der: DER,
    #[doc = "0x20 - Configuration 0"]
    pub cfgr0: CFGR0,
    #[doc = "0x24 - Configuration 1"]
    pub cfgr1: CFGR1,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Data Match 0"]
    pub dmr0: DMR0,
    #[doc = "0x34 - Data Match 1"]
    pub dmr1: DMR1,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - Clock Configuration"]
    pub ccr: CCR,
    _reserved11: [u8; 0x14],
    #[doc = "0x58 - FIFO Control"]
    pub fcr: FCR,
    #[doc = "0x5c - FIFO Status"]
    pub fsr: FSR,
    #[doc = "0x60 - Transmit Command"]
    pub tcr: TCR,
    #[doc = "0x64 - Transmit Data"]
    pub tdr: TDR,
    _reserved15: [u8; 0x08],
    #[doc = "0x70 - Receive Status"]
    pub rsr: RSR,
    #[doc = "0x74 - Receive Data"]
    pub rdr: RDR,
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter"]
pub mod param;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control"]
pub mod cr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status"]
pub mod sr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "DER (rw) register accessor: an alias for `Reg<DER_SPEC>`"]
pub type DER = crate::Reg<der::DER_SPEC>;
#[doc = "DMA Enable"]
pub mod der;
#[doc = "CFGR0 (rw) register accessor: an alias for `Reg<CFGR0_SPEC>`"]
pub type CFGR0 = crate::Reg<cfgr0::CFGR0_SPEC>;
#[doc = "Configuration 0"]
pub mod cfgr0;
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "Configuration 1"]
pub mod cfgr1;
#[doc = "DMR0 (rw) register accessor: an alias for `Reg<DMR0_SPEC>`"]
pub type DMR0 = crate::Reg<dmr0::DMR0_SPEC>;
#[doc = "Data Match 0"]
pub mod dmr0;
#[doc = "DMR1 (rw) register accessor: an alias for `Reg<DMR1_SPEC>`"]
pub type DMR1 = crate::Reg<dmr1::DMR1_SPEC>;
#[doc = "Data Match 1"]
pub mod dmr1;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Configuration"]
pub mod ccr;
#[doc = "FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control"]
pub mod fcr;
#[doc = "FSR (r) register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "FIFO Status"]
pub mod fsr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Command"]
pub mod tcr;
#[doc = "TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data"]
pub mod tdr;
#[doc = "RSR (r) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status"]
pub mod rsr;
#[doc = "RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data"]
pub mod rdr;
