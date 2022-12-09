#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPR0 General Purpose Register"]
    pub gpr0: GPR0,
    #[doc = "0x04 - GPR1 General Purpose Register"]
    pub gpr1: GPR1,
    #[doc = "0x08 - GPR2 General Purpose Register"]
    pub gpr2: GPR2,
    #[doc = "0x0c - GPR3 General Purpose Register"]
    pub gpr3: GPR3,
}
#[doc = "GPR0 (r) register accessor: an alias for `Reg<GPR0_SPEC>`"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "GPR0 General Purpose Register"]
pub mod gpr0;
#[doc = "GPR1 (r) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "GPR1 General Purpose Register"]
pub mod gpr1;
#[doc = "GPR2 (r) register accessor: an alias for `Reg<GPR2_SPEC>`"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "GPR2 General Purpose Register"]
pub mod gpr2;
#[doc = "GPR3 (rw) register accessor: an alias for `Reg<GPR3_SPEC>`"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "GPR3 General Purpose Register"]
pub mod gpr3;
