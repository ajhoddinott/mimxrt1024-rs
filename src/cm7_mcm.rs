#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Interrupt Status and Control Register"]
    pub iscr: ISCR,
}
#[doc = "ISCR (rw) register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
