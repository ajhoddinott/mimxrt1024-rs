#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Channel index Configuration Register"]
    pub chcfg: [CHCFG; 32],
}
#[doc = "CHCFG (rw) register accessor: an alias for `Reg<CHCFG_SPEC>`"]
pub type CHCFG = crate::Reg<chcfg::CHCFG_SPEC>;
#[doc = "Channel index Configuration Register"]
pub mod chcfg;
