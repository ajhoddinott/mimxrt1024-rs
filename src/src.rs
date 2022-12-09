#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRC Control Register"]
    pub scr: SCR,
    #[doc = "0x04 - SRC Boot Mode Register 1"]
    pub sbmr1: SBMR1,
    #[doc = "0x08 - SRC Reset Status Register"]
    pub srsr: SRSR,
    _reserved3: [u8; 0x10],
    #[doc = "0x1c - SRC Boot Mode Register 2"]
    pub sbmr2: SBMR2,
    #[doc = "0x20 - SRC General Purpose Register 1"]
    pub gpr1: GPR1,
    #[doc = "0x24 - SRC General Purpose Register 2"]
    pub gpr2: GPR2,
    #[doc = "0x28 - SRC General Purpose Register 3"]
    pub gpr3: GPR3,
    #[doc = "0x2c - SRC General Purpose Register 4"]
    pub gpr4: GPR4,
    #[doc = "0x30 - SRC General Purpose Register 5"]
    pub gpr5: GPR5,
    #[doc = "0x34 - SRC General Purpose Register 6"]
    pub gpr6: GPR6,
    #[doc = "0x38 - SRC General Purpose Register 7"]
    pub gpr7: GPR7,
    #[doc = "0x3c - SRC General Purpose Register 8"]
    pub gpr8: GPR8,
    #[doc = "0x40 - SRC General Purpose Register 9"]
    pub gpr9: GPR9,
    #[doc = "0x44 - SRC General Purpose Register 10"]
    pub gpr10: GPR10,
}
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "SRC Control Register"]
pub mod scr;
#[doc = "SBMR1 (r) register accessor: an alias for `Reg<SBMR1_SPEC>`"]
pub type SBMR1 = crate::Reg<sbmr1::SBMR1_SPEC>;
#[doc = "SRC Boot Mode Register 1"]
pub mod sbmr1;
#[doc = "SRSR (rw) register accessor: an alias for `Reg<SRSR_SPEC>`"]
pub type SRSR = crate::Reg<srsr::SRSR_SPEC>;
#[doc = "SRC Reset Status Register"]
pub mod srsr;
#[doc = "SBMR2 (r) register accessor: an alias for `Reg<SBMR2_SPEC>`"]
pub type SBMR2 = crate::Reg<sbmr2::SBMR2_SPEC>;
#[doc = "SRC Boot Mode Register 2"]
pub mod sbmr2;
#[doc = "GPR1 (rw) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "SRC General Purpose Register 1"]
pub mod gpr1;
#[doc = "GPR2 (rw) register accessor: an alias for `Reg<GPR2_SPEC>`"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "SRC General Purpose Register 2"]
pub mod gpr2;
#[doc = "GPR3 (rw) register accessor: an alias for `Reg<GPR3_SPEC>`"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "SRC General Purpose Register 3"]
pub mod gpr3;
#[doc = "GPR4 (rw) register accessor: an alias for `Reg<GPR4_SPEC>`"]
pub type GPR4 = crate::Reg<gpr4::GPR4_SPEC>;
#[doc = "SRC General Purpose Register 4"]
pub mod gpr4;
#[doc = "GPR5 (rw) register accessor: an alias for `Reg<GPR5_SPEC>`"]
pub type GPR5 = crate::Reg<gpr5::GPR5_SPEC>;
#[doc = "SRC General Purpose Register 5"]
pub mod gpr5;
#[doc = "GPR6 (rw) register accessor: an alias for `Reg<GPR6_SPEC>`"]
pub type GPR6 = crate::Reg<gpr6::GPR6_SPEC>;
#[doc = "SRC General Purpose Register 6"]
pub mod gpr6;
#[doc = "GPR7 (rw) register accessor: an alias for `Reg<GPR7_SPEC>`"]
pub type GPR7 = crate::Reg<gpr7::GPR7_SPEC>;
#[doc = "SRC General Purpose Register 7"]
pub mod gpr7;
#[doc = "GPR8 (rw) register accessor: an alias for `Reg<GPR8_SPEC>`"]
pub type GPR8 = crate::Reg<gpr8::GPR8_SPEC>;
#[doc = "SRC General Purpose Register 8"]
pub mod gpr8;
#[doc = "GPR9 (r) register accessor: an alias for `Reg<GPR9_SPEC>`"]
pub type GPR9 = crate::Reg<gpr9::GPR9_SPEC>;
#[doc = "SRC General Purpose Register 9"]
pub mod gpr9;
#[doc = "GPR10 (rw) register accessor: an alias for `Reg<GPR10_SPEC>`"]
pub type GPR10 = crate::Reg<gpr10::GPR10_SPEC>;
#[doc = "SRC General Purpose Register 10"]
pub mod gpr10;
