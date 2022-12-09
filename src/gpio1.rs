#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO data register"]
    pub dr: DR,
    #[doc = "0x04 - GPIO direction register"]
    pub gdir: GDIR,
    #[doc = "0x08 - GPIO pad status register"]
    pub psr: PSR,
    #[doc = "0x0c - GPIO interrupt configuration register1"]
    pub icr1: ICR1,
    #[doc = "0x10 - GPIO interrupt configuration register2"]
    pub icr2: ICR2,
    #[doc = "0x14 - GPIO interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x18 - GPIO interrupt status register"]
    pub isr: ISR,
    #[doc = "0x1c - GPIO edge select register"]
    pub edge_sel: EDGE_SEL,
    _reserved8: [u8; 0x64],
    #[doc = "0x84 - GPIO data register SET"]
    pub dr_set: DR_SET,
    #[doc = "0x88 - GPIO data register CLEAR"]
    pub dr_clear: DR_CLEAR,
    #[doc = "0x8c - GPIO data register TOGGLE"]
    pub dr_toggle: DR_TOGGLE,
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "GPIO data register"]
pub mod dr;
#[doc = "GDIR (rw) register accessor: an alias for `Reg<GDIR_SPEC>`"]
pub type GDIR = crate::Reg<gdir::GDIR_SPEC>;
#[doc = "GPIO direction register"]
pub mod gdir;
#[doc = "PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "GPIO pad status register"]
pub mod psr;
#[doc = "ICR1 (rw) register accessor: an alias for `Reg<ICR1_SPEC>`"]
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
#[doc = "GPIO interrupt configuration register1"]
pub mod icr1;
#[doc = "ICR2 (rw) register accessor: an alias for `Reg<ICR2_SPEC>`"]
pub type ICR2 = crate::Reg<icr2::ICR2_SPEC>;
#[doc = "GPIO interrupt configuration register2"]
pub mod icr2;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "GPIO interrupt mask register"]
pub mod imr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod isr;
#[doc = "EDGE_SEL (rw) register accessor: an alias for `Reg<EDGE_SEL_SPEC>`"]
pub type EDGE_SEL = crate::Reg<edge_sel::EDGE_SEL_SPEC>;
#[doc = "GPIO edge select register"]
pub mod edge_sel;
#[doc = "DR_SET (w) register accessor: an alias for `Reg<DR_SET_SPEC>`"]
pub type DR_SET = crate::Reg<dr_set::DR_SET_SPEC>;
#[doc = "GPIO data register SET"]
pub mod dr_set;
#[doc = "DR_CLEAR (w) register accessor: an alias for `Reg<DR_CLEAR_SPEC>`"]
pub type DR_CLEAR = crate::Reg<dr_clear::DR_CLEAR_SPEC>;
#[doc = "GPIO data register CLEAR"]
pub mod dr_clear;
#[doc = "DR_TOGGLE (w) register accessor: an alias for `Reg<DR_TOGGLE_SPEC>`"]
pub type DR_TOGGLE = crate::Reg<dr_toggle::DR_TOGGLE_SPEC>;
#[doc = "GPIO data register TOGGLE"]
pub mod dr_toggle;
