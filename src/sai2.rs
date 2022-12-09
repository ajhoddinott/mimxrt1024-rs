#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter"]
    pub param: PARAM,
    #[doc = "0x08 - Transmit Control"]
    pub tcsr: TCSR,
    #[doc = "0x0c - Transmit Configuration 1"]
    pub tcr1: TCR1,
    #[doc = "0x10 - Transmit Configuration 2"]
    pub tcr2: TCR2,
    #[doc = "0x14 - Transmit Configuration 3"]
    pub tcr3: TCR3,
    #[doc = "0x18 - Transmit Configuration 4"]
    pub tcr4: TCR4,
    #[doc = "0x1c - Transmit Configuration 5"]
    pub tcr5: TCR5,
    #[doc = "0x20 - Transmit Data"]
    pub tdr0: TDR0,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - Transmit FIFO"]
    pub tfr0: TFR0,
    _reserved10: [u8; 0x1c],
    #[doc = "0x60 - Transmit Mask"]
    pub tmr: TMR,
    _reserved11: [u8; 0x24],
    #[doc = "0x88 - Receive Control"]
    pub rcsr: RCSR,
    #[doc = "0x8c - Receive Configuration 1"]
    pub rcr1: RCR1,
    #[doc = "0x90 - Receive Configuration 2"]
    pub rcr2: RCR2,
    #[doc = "0x94 - Receive Configuration 3"]
    pub rcr3: RCR3,
    #[doc = "0x98 - Receive Configuration 4"]
    pub rcr4: RCR4,
    #[doc = "0x9c - Receive Configuration 5"]
    pub rcr5: RCR5,
    #[doc = "0xa0 - Receive Data"]
    pub rdr0: RDR0,
    _reserved18: [u8; 0x1c],
    #[doc = "0xc0 - Receive FIFO"]
    pub rfr0: RFR0,
    _reserved19: [u8; 0x1c],
    #[doc = "0xe0 - Receive Mask"]
    pub rmr: RMR,
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter"]
pub mod param;
#[doc = "TCSR (rw) register accessor: an alias for `Reg<TCSR_SPEC>`"]
pub type TCSR = crate::Reg<tcsr::TCSR_SPEC>;
#[doc = "Transmit Control"]
pub mod tcsr;
#[doc = "TCR1 (rw) register accessor: an alias for `Reg<TCR1_SPEC>`"]
pub type TCR1 = crate::Reg<tcr1::TCR1_SPEC>;
#[doc = "Transmit Configuration 1"]
pub mod tcr1;
#[doc = "TCR2 (rw) register accessor: an alias for `Reg<TCR2_SPEC>`"]
pub type TCR2 = crate::Reg<tcr2::TCR2_SPEC>;
#[doc = "Transmit Configuration 2"]
pub mod tcr2;
#[doc = "TCR3 (rw) register accessor: an alias for `Reg<TCR3_SPEC>`"]
pub type TCR3 = crate::Reg<tcr3::TCR3_SPEC>;
#[doc = "Transmit Configuration 3"]
pub mod tcr3;
#[doc = "TCR4 (rw) register accessor: an alias for `Reg<TCR4_SPEC>`"]
pub type TCR4 = crate::Reg<tcr4::TCR4_SPEC>;
#[doc = "Transmit Configuration 4"]
pub mod tcr4;
#[doc = "TCR5 (rw) register accessor: an alias for `Reg<TCR5_SPEC>`"]
pub type TCR5 = crate::Reg<tcr5::TCR5_SPEC>;
#[doc = "Transmit Configuration 5"]
pub mod tcr5;
#[doc = "TDR0 (rw) register accessor: an alias for `Reg<TDR0_SPEC>`"]
pub type TDR0 = crate::Reg<tdr0::TDR0_SPEC>;
#[doc = "Transmit Data"]
pub mod tdr0;
#[doc = "TFR0 (r) register accessor: an alias for `Reg<TFR0_SPEC>`"]
pub type TFR0 = crate::Reg<tfr0::TFR0_SPEC>;
#[doc = "Transmit FIFO"]
pub mod tfr0;
#[doc = "TMR (rw) register accessor: an alias for `Reg<TMR_SPEC>`"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Transmit Mask"]
pub mod tmr;
#[doc = "RCSR (rw) register accessor: an alias for `Reg<RCSR_SPEC>`"]
pub type RCSR = crate::Reg<rcsr::RCSR_SPEC>;
#[doc = "Receive Control"]
pub mod rcsr;
#[doc = "RCR1 (rw) register accessor: an alias for `Reg<RCR1_SPEC>`"]
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
#[doc = "Receive Configuration 1"]
pub mod rcr1;
#[doc = "RCR2 (rw) register accessor: an alias for `Reg<RCR2_SPEC>`"]
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
#[doc = "Receive Configuration 2"]
pub mod rcr2;
#[doc = "RCR3 (rw) register accessor: an alias for `Reg<RCR3_SPEC>`"]
pub type RCR3 = crate::Reg<rcr3::RCR3_SPEC>;
#[doc = "Receive Configuration 3"]
pub mod rcr3;
#[doc = "RCR4 (rw) register accessor: an alias for `Reg<RCR4_SPEC>`"]
pub type RCR4 = crate::Reg<rcr4::RCR4_SPEC>;
#[doc = "Receive Configuration 4"]
pub mod rcr4;
#[doc = "RCR5 (rw) register accessor: an alias for `Reg<RCR5_SPEC>`"]
pub type RCR5 = crate::Reg<rcr5::RCR5_SPEC>;
#[doc = "Receive Configuration 5"]
pub mod rcr5;
#[doc = "RDR0 (r) register accessor: an alias for `Reg<RDR0_SPEC>`"]
pub type RDR0 = crate::Reg<rdr0::RDR0_SPEC>;
#[doc = "Receive Data"]
pub mod rdr0;
#[doc = "RFR0 (r) register accessor: an alias for `Reg<RFR0_SPEC>`"]
pub type RFR0 = crate::Reg<rfr0::RFR0_SPEC>;
#[doc = "Receive FIFO"]
pub mod rfr0;
#[doc = "RMR (rw) register accessor: an alias for `Reg<RMR_SPEC>`"]
pub type RMR = crate::Reg<rmr::RMR_SPEC>;
#[doc = "Receive Mask"]
pub mod rmr;
