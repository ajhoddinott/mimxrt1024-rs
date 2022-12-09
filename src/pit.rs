#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    pub mcr: MCR,
    _reserved1: [u8; 0xdc],
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    pub ltmr64h: LTMR64H,
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    pub ltmr64l: LTMR64L,
    _reserved3: [u8; 0x18],
    #[doc = "0x100..0x140 - no description available"]
    pub timer: [TIMER; 4],
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "LTMR64H (r) register accessor: an alias for `Reg<LTMR64H_SPEC>`"]
pub type LTMR64H = crate::Reg<ltmr64h::LTMR64H_SPEC>;
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "LTMR64L (r) register accessor: an alias for `Reg<LTMR64L_SPEC>`"]
pub type LTMR64L = crate::Reg<ltmr64l::LTMR64L_SPEC>;
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
#[doc = "no description available"]
pub use self::timer::TIMER;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod timer;
