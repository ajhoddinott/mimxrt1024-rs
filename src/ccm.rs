#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CCM Control Register"]
    pub ccr: CCR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - CCM Status Register"]
    pub csr: CSR,
    #[doc = "0x0c - CCM Clock Switcher Register"]
    pub ccsr: CCSR,
    #[doc = "0x10 - CCM Arm Clock Root Register"]
    pub cacrr: CACRR,
    #[doc = "0x14 - CCM Bus Clock Divider Register"]
    pub cbcdr: CBCDR,
    #[doc = "0x18 - CCM Bus Clock Multiplexer Register"]
    pub cbcmr: CBCMR,
    #[doc = "0x1c - CCM Serial Clock Multiplexer Register 1"]
    pub cscmr1: CSCMR1,
    #[doc = "0x20 - CCM Serial Clock Multiplexer Register 2"]
    pub cscmr2: CSCMR2,
    #[doc = "0x24 - CCM Serial Clock Divider Register 1"]
    pub cscdr1: CSCDR1,
    #[doc = "0x28 - CCM Clock Divider Register"]
    pub cs1cdr: CS1CDR,
    #[doc = "0x2c - CCM Clock Divider Register"]
    pub cs2cdr: CS2CDR,
    #[doc = "0x30 - CCM D1 Clock Divider Register"]
    pub cdcdr: CDCDR,
    _reserved12: [u8; 0x04],
    #[doc = "0x38 - CCM Serial Clock Divider Register 2"]
    pub cscdr2: CSCDR2,
    #[doc = "0x3c - CCM Serial Clock Divider Register 3"]
    pub cscdr3: CSCDR3,
    _reserved14: [u8; 0x08],
    #[doc = "0x48 - CCM Divider Handshake In-Process Register"]
    pub cdhipr: CDHIPR,
    _reserved15: [u8; 0x08],
    #[doc = "0x54 - CCM Low Power Control Register"]
    pub clpcr: CLPCR,
    #[doc = "0x58 - CCM Interrupt Status Register"]
    pub cisr: CISR,
    #[doc = "0x5c - CCM Interrupt Mask Register"]
    pub cimr: CIMR,
    #[doc = "0x60 - CCM Clock Output Source Register"]
    pub ccosr: CCOSR,
    #[doc = "0x64 - CCM General Purpose Register"]
    pub cgpr: CGPR,
    #[doc = "0x68 - CCM Clock Gating Register 0"]
    pub ccgr0: CCGR0,
    #[doc = "0x6c - CCM Clock Gating Register 1"]
    pub ccgr1: CCGR1,
    #[doc = "0x70 - CCM Clock Gating Register 2"]
    pub ccgr2: CCGR2,
    #[doc = "0x74 - CCM Clock Gating Register 3"]
    pub ccgr3: CCGR3,
    #[doc = "0x78 - CCM Clock Gating Register 4"]
    pub ccgr4: CCGR4,
    #[doc = "0x7c - CCM Clock Gating Register 5"]
    pub ccgr5: CCGR5,
    #[doc = "0x80 - CCM Clock Gating Register 6"]
    pub ccgr6: CCGR6,
    _reserved27: [u8; 0x04],
    #[doc = "0x88 - CCM Module Enable Overide Register"]
    pub cmeor: CMEOR,
}
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "CCM Control Register"]
pub mod ccr;
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CCM Status Register"]
pub mod csr;
#[doc = "CCSR (rw) register accessor: an alias for `Reg<CCSR_SPEC>`"]
pub type CCSR = crate::Reg<ccsr::CCSR_SPEC>;
#[doc = "CCM Clock Switcher Register"]
pub mod ccsr;
#[doc = "CACRR (rw) register accessor: an alias for `Reg<CACRR_SPEC>`"]
pub type CACRR = crate::Reg<cacrr::CACRR_SPEC>;
#[doc = "CCM Arm Clock Root Register"]
pub mod cacrr;
#[doc = "CBCDR (rw) register accessor: an alias for `Reg<CBCDR_SPEC>`"]
pub type CBCDR = crate::Reg<cbcdr::CBCDR_SPEC>;
#[doc = "CCM Bus Clock Divider Register"]
pub mod cbcdr;
#[doc = "CBCMR (rw) register accessor: an alias for `Reg<CBCMR_SPEC>`"]
pub type CBCMR = crate::Reg<cbcmr::CBCMR_SPEC>;
#[doc = "CCM Bus Clock Multiplexer Register"]
pub mod cbcmr;
#[doc = "CSCMR1 (rw) register accessor: an alias for `Reg<CSCMR1_SPEC>`"]
pub type CSCMR1 = crate::Reg<cscmr1::CSCMR1_SPEC>;
#[doc = "CCM Serial Clock Multiplexer Register 1"]
pub mod cscmr1;
#[doc = "CSCMR2 (rw) register accessor: an alias for `Reg<CSCMR2_SPEC>`"]
pub type CSCMR2 = crate::Reg<cscmr2::CSCMR2_SPEC>;
#[doc = "CCM Serial Clock Multiplexer Register 2"]
pub mod cscmr2;
#[doc = "CSCDR1 (rw) register accessor: an alias for `Reg<CSCDR1_SPEC>`"]
pub type CSCDR1 = crate::Reg<cscdr1::CSCDR1_SPEC>;
#[doc = "CCM Serial Clock Divider Register 1"]
pub mod cscdr1;
#[doc = "CS1CDR (rw) register accessor: an alias for `Reg<CS1CDR_SPEC>`"]
pub type CS1CDR = crate::Reg<cs1cdr::CS1CDR_SPEC>;
#[doc = "CCM Clock Divider Register"]
pub mod cs1cdr;
#[doc = "CS2CDR (rw) register accessor: an alias for `Reg<CS2CDR_SPEC>`"]
pub type CS2CDR = crate::Reg<cs2cdr::CS2CDR_SPEC>;
#[doc = "CCM Clock Divider Register"]
pub mod cs2cdr;
#[doc = "CDCDR (rw) register accessor: an alias for `Reg<CDCDR_SPEC>`"]
pub type CDCDR = crate::Reg<cdcdr::CDCDR_SPEC>;
#[doc = "CCM D1 Clock Divider Register"]
pub mod cdcdr;
#[doc = "CSCDR2 (rw) register accessor: an alias for `Reg<CSCDR2_SPEC>`"]
pub type CSCDR2 = crate::Reg<cscdr2::CSCDR2_SPEC>;
#[doc = "CCM Serial Clock Divider Register 2"]
pub mod cscdr2;
#[doc = "CSCDR3 (r) register accessor: an alias for `Reg<CSCDR3_SPEC>`"]
pub type CSCDR3 = crate::Reg<cscdr3::CSCDR3_SPEC>;
#[doc = "CCM Serial Clock Divider Register 3"]
pub mod cscdr3;
#[doc = "CDHIPR (r) register accessor: an alias for `Reg<CDHIPR_SPEC>`"]
pub type CDHIPR = crate::Reg<cdhipr::CDHIPR_SPEC>;
#[doc = "CCM Divider Handshake In-Process Register"]
pub mod cdhipr;
#[doc = "CLPCR (rw) register accessor: an alias for `Reg<CLPCR_SPEC>`"]
pub type CLPCR = crate::Reg<clpcr::CLPCR_SPEC>;
#[doc = "CCM Low Power Control Register"]
pub mod clpcr;
#[doc = "CISR (rw) register accessor: an alias for `Reg<CISR_SPEC>`"]
pub type CISR = crate::Reg<cisr::CISR_SPEC>;
#[doc = "CCM Interrupt Status Register"]
pub mod cisr;
#[doc = "CIMR (rw) register accessor: an alias for `Reg<CIMR_SPEC>`"]
pub type CIMR = crate::Reg<cimr::CIMR_SPEC>;
#[doc = "CCM Interrupt Mask Register"]
pub mod cimr;
#[doc = "CCOSR (rw) register accessor: an alias for `Reg<CCOSR_SPEC>`"]
pub type CCOSR = crate::Reg<ccosr::CCOSR_SPEC>;
#[doc = "CCM Clock Output Source Register"]
pub mod ccosr;
#[doc = "CGPR (rw) register accessor: an alias for `Reg<CGPR_SPEC>`"]
pub type CGPR = crate::Reg<cgpr::CGPR_SPEC>;
#[doc = "CCM General Purpose Register"]
pub mod cgpr;
#[doc = "CCGR0 (rw) register accessor: an alias for `Reg<CCGR0_SPEC>`"]
pub type CCGR0 = crate::Reg<ccgr0::CCGR0_SPEC>;
#[doc = "CCM Clock Gating Register 0"]
pub mod ccgr0;
#[doc = "CCGR1 (rw) register accessor: an alias for `Reg<CCGR1_SPEC>`"]
pub type CCGR1 = crate::Reg<ccgr1::CCGR1_SPEC>;
#[doc = "CCM Clock Gating Register 1"]
pub mod ccgr1;
#[doc = "CCGR2 (rw) register accessor: an alias for `Reg<CCGR2_SPEC>`"]
pub type CCGR2 = crate::Reg<ccgr2::CCGR2_SPEC>;
#[doc = "CCM Clock Gating Register 2"]
pub mod ccgr2;
#[doc = "CCGR3 (rw) register accessor: an alias for `Reg<CCGR3_SPEC>`"]
pub type CCGR3 = crate::Reg<ccgr3::CCGR3_SPEC>;
#[doc = "CCM Clock Gating Register 3"]
pub mod ccgr3;
#[doc = "CCGR4 (rw) register accessor: an alias for `Reg<CCGR4_SPEC>`"]
pub type CCGR4 = crate::Reg<ccgr4::CCGR4_SPEC>;
#[doc = "CCM Clock Gating Register 4"]
pub mod ccgr4;
#[doc = "CCGR5 (rw) register accessor: an alias for `Reg<CCGR5_SPEC>`"]
pub type CCGR5 = crate::Reg<ccgr5::CCGR5_SPEC>;
#[doc = "CCM Clock Gating Register 5"]
pub mod ccgr5;
#[doc = "CCGR6 (rw) register accessor: an alias for `Reg<CCGR6_SPEC>`"]
pub type CCGR6 = crate::Reg<ccgr6::CCGR6_SPEC>;
#[doc = "CCM Clock Gating Register 6"]
pub mod ccgr6;
#[doc = "CMEOR (rw) register accessor: an alias for `Reg<CMEOR_SPEC>`"]
pub type CMEOR = crate::Reg<cmeor::CMEOR_SPEC>;
#[doc = "CCM Module Enable Overide Register"]
pub mod cmeor;
