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
    #[doc = "0x10 - GPR4 General Purpose Register"]
    pub gpr4: GPR4,
    #[doc = "0x14 - GPR5 General Purpose Register"]
    pub gpr5: GPR5,
    #[doc = "0x18 - GPR6 General Purpose Register"]
    pub gpr6: GPR6,
    #[doc = "0x1c - GPR7 General Purpose Register"]
    pub gpr7: GPR7,
    #[doc = "0x20 - GPR8 General Purpose Register"]
    pub gpr8: GPR8,
    #[doc = "0x24 - GPR9 General Purpose Register"]
    pub gpr9: GPR9,
    #[doc = "0x28 - GPR10 General Purpose Register"]
    pub gpr10: GPR10,
    #[doc = "0x2c - GPR11 General Purpose Register"]
    pub gpr11: GPR11,
    #[doc = "0x30 - GPR12 General Purpose Register"]
    pub gpr12: GPR12,
    #[doc = "0x34 - GPR13 General Purpose Register"]
    pub gpr13: GPR13,
    #[doc = "0x38 - GPR14 General Purpose Register"]
    pub gpr14: GPR14,
    #[doc = "0x3c - GPR15 General Purpose Register"]
    pub gpr15: GPR15,
    #[doc = "0x40 - GPR16 General Purpose Register"]
    pub gpr16: GPR16,
    #[doc = "0x44 - GPR17 General Purpose Register"]
    pub gpr17: GPR17,
    #[doc = "0x48 - GPR18 General Purpose Register"]
    pub gpr18: GPR18,
    #[doc = "0x4c - GPR19 General Purpose Register"]
    pub gpr19: GPR19,
    #[doc = "0x50 - GPR20 General Purpose Register"]
    pub gpr20: GPR20,
    #[doc = "0x54 - GPR21 General Purpose Register"]
    pub gpr21: GPR21,
    #[doc = "0x58 - GPR22 General Purpose Register"]
    pub gpr22: GPR22,
    #[doc = "0x5c - GPR23 General Purpose Register"]
    pub gpr23: GPR23,
    #[doc = "0x60 - GPR24 General Purpose Register"]
    pub gpr24: GPR24,
    #[doc = "0x64 - GPR25 General Purpose Register"]
    pub gpr25: GPR25,
}
#[doc = "GPR0 (r) register accessor: an alias for `Reg<GPR0_SPEC>`"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "GPR0 General Purpose Register"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "GPR1 General Purpose Register"]
pub mod gpr1;
#[doc = "GPR2 (rw) register accessor: an alias for `Reg<GPR2_SPEC>`"]
pub type GPR2 = crate::Reg<gpr2::GPR2_SPEC>;
#[doc = "GPR2 General Purpose Register"]
pub mod gpr2;
#[doc = "GPR3 (rw) register accessor: an alias for `Reg<GPR3_SPEC>`"]
pub type GPR3 = crate::Reg<gpr3::GPR3_SPEC>;
#[doc = "GPR3 General Purpose Register"]
pub mod gpr3;
#[doc = "GPR4 (rw) register accessor: an alias for `Reg<GPR4_SPEC>`"]
pub type GPR4 = crate::Reg<gpr4::GPR4_SPEC>;
#[doc = "GPR4 General Purpose Register"]
pub mod gpr4;
#[doc = "GPR5 (rw) register accessor: an alias for `Reg<GPR5_SPEC>`"]
pub type GPR5 = crate::Reg<gpr5::GPR5_SPEC>;
#[doc = "GPR5 General Purpose Register"]
pub mod gpr5;
#[doc = "GPR6 (rw) register accessor: an alias for `Reg<GPR6_SPEC>`"]
pub type GPR6 = crate::Reg<gpr6::GPR6_SPEC>;
#[doc = "GPR6 General Purpose Register"]
pub mod gpr6;
#[doc = "GPR7 (rw) register accessor: an alias for `Reg<GPR7_SPEC>`"]
pub type GPR7 = crate::Reg<gpr7::GPR7_SPEC>;
#[doc = "GPR7 General Purpose Register"]
pub mod gpr7;
#[doc = "GPR8 (rw) register accessor: an alias for `Reg<GPR8_SPEC>`"]
pub type GPR8 = crate::Reg<gpr8::GPR8_SPEC>;
#[doc = "GPR8 General Purpose Register"]
pub mod gpr8;
#[doc = "GPR9 (r) register accessor: an alias for `Reg<GPR9_SPEC>`"]
pub type GPR9 = crate::Reg<gpr9::GPR9_SPEC>;
#[doc = "GPR9 General Purpose Register"]
pub mod gpr9;
#[doc = "GPR10 (rw) register accessor: an alias for `Reg<GPR10_SPEC>`"]
pub type GPR10 = crate::Reg<gpr10::GPR10_SPEC>;
#[doc = "GPR10 General Purpose Register"]
pub mod gpr10;
#[doc = "GPR11 (rw) register accessor: an alias for `Reg<GPR11_SPEC>`"]
pub type GPR11 = crate::Reg<gpr11::GPR11_SPEC>;
#[doc = "GPR11 General Purpose Register"]
pub mod gpr11;
#[doc = "GPR12 (rw) register accessor: an alias for `Reg<GPR12_SPEC>`"]
pub type GPR12 = crate::Reg<gpr12::GPR12_SPEC>;
#[doc = "GPR12 General Purpose Register"]
pub mod gpr12;
#[doc = "GPR13 (rw) register accessor: an alias for `Reg<GPR13_SPEC>`"]
pub type GPR13 = crate::Reg<gpr13::GPR13_SPEC>;
#[doc = "GPR13 General Purpose Register"]
pub mod gpr13;
#[doc = "GPR14 (rw) register accessor: an alias for `Reg<GPR14_SPEC>`"]
pub type GPR14 = crate::Reg<gpr14::GPR14_SPEC>;
#[doc = "GPR14 General Purpose Register"]
pub mod gpr14;
#[doc = "GPR15 (r) register accessor: an alias for `Reg<GPR15_SPEC>`"]
pub type GPR15 = crate::Reg<gpr15::GPR15_SPEC>;
#[doc = "GPR15 General Purpose Register"]
pub mod gpr15;
#[doc = "GPR16 (rw) register accessor: an alias for `Reg<GPR16_SPEC>`"]
pub type GPR16 = crate::Reg<gpr16::GPR16_SPEC>;
#[doc = "GPR16 General Purpose Register"]
pub mod gpr16;
#[doc = "GPR17 (rw) register accessor: an alias for `Reg<GPR17_SPEC>`"]
pub type GPR17 = crate::Reg<gpr17::GPR17_SPEC>;
#[doc = "GPR17 General Purpose Register"]
pub mod gpr17;
#[doc = "GPR18 (rw) register accessor: an alias for `Reg<GPR18_SPEC>`"]
pub type GPR18 = crate::Reg<gpr18::GPR18_SPEC>;
#[doc = "GPR18 General Purpose Register"]
pub mod gpr18;
#[doc = "GPR19 (rw) register accessor: an alias for `Reg<GPR19_SPEC>`"]
pub type GPR19 = crate::Reg<gpr19::GPR19_SPEC>;
#[doc = "GPR19 General Purpose Register"]
pub mod gpr19;
#[doc = "GPR20 (rw) register accessor: an alias for `Reg<GPR20_SPEC>`"]
pub type GPR20 = crate::Reg<gpr20::GPR20_SPEC>;
#[doc = "GPR20 General Purpose Register"]
pub mod gpr20;
#[doc = "GPR21 (rw) register accessor: an alias for `Reg<GPR21_SPEC>`"]
pub type GPR21 = crate::Reg<gpr21::GPR21_SPEC>;
#[doc = "GPR21 General Purpose Register"]
pub mod gpr21;
#[doc = "GPR22 (rw) register accessor: an alias for `Reg<GPR22_SPEC>`"]
pub type GPR22 = crate::Reg<gpr22::GPR22_SPEC>;
#[doc = "GPR22 General Purpose Register"]
pub mod gpr22;
#[doc = "GPR23 (rw) register accessor: an alias for `Reg<GPR23_SPEC>`"]
pub type GPR23 = crate::Reg<gpr23::GPR23_SPEC>;
#[doc = "GPR23 General Purpose Register"]
pub mod gpr23;
#[doc = "GPR24 (rw) register accessor: an alias for `Reg<GPR24_SPEC>`"]
pub type GPR24 = crate::Reg<gpr24::GPR24_SPEC>;
#[doc = "GPR24 General Purpose Register"]
pub mod gpr24;
#[doc = "GPR25 (rw) register accessor: an alias for `Reg<GPR25_SPEC>`"]
pub type GPR25 = crate::Reg<gpr25::GPR25_SPEC>;
#[doc = "GPR25 General Purpose Register"]
pub mod gpr25;
