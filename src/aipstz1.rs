#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Priviledge Registers"]
    pub mpr: MPR,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Registers"]
    pub opacr: OPACR,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Registers"]
    pub opacr1: OPACR1,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Registers"]
    pub opacr2: OPACR2,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Registers"]
    pub opacr3: OPACR3,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Registers"]
    pub opacr4: OPACR4,
}
#[doc = "MPR (rw) register accessor: an alias for `Reg<MPR_SPEC>`"]
pub type MPR = crate::Reg<mpr::MPR_SPEC>;
#[doc = "Master Priviledge Registers"]
pub mod mpr;
#[doc = "OPACR (rw) register accessor: an alias for `Reg<OPACR_SPEC>`"]
pub type OPACR = crate::Reg<opacr::OPACR_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr;
#[doc = "OPACR1 (rw) register accessor: an alias for `Reg<OPACR1_SPEC>`"]
pub type OPACR1 = crate::Reg<opacr1::OPACR1_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr1;
#[doc = "OPACR2 (rw) register accessor: an alias for `Reg<OPACR2_SPEC>`"]
pub type OPACR2 = crate::Reg<opacr2::OPACR2_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr2;
#[doc = "OPACR3 (rw) register accessor: an alias for `Reg<OPACR3_SPEC>`"]
pub type OPACR3 = crate::Reg<opacr3::OPACR3_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr3;
#[doc = "OPACR4 (rw) register accessor: an alias for `Reg<OPACR4_SPEC>`"]
pub type OPACR4 = crate::Reg<opacr4::OPACR4_SPEC>;
#[doc = "Off-Platform Peripheral Access Control Registers"]
pub mod opacr4;
