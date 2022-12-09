#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCDC Register 0"]
    pub reg0: REG0,
    #[doc = "0x04 - DCDC Register 1"]
    pub reg1: REG1,
    #[doc = "0x08 - DCDC Register 2"]
    pub reg2: REG2,
    #[doc = "0x0c - DCDC Register 3"]
    pub reg3: REG3,
}
#[doc = "REG0 (rw) register accessor: an alias for `Reg<REG0_SPEC>`"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "DCDC Register 0"]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: an alias for `Reg<REG1_SPEC>`"]
pub type REG1 = crate::Reg<reg1::REG1_SPEC>;
#[doc = "DCDC Register 1"]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: an alias for `Reg<REG2_SPEC>`"]
pub type REG2 = crate::Reg<reg2::REG2_SPEC>;
#[doc = "DCDC Register 2"]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: an alias for `Reg<REG3_SPEC>`"]
pub type REG3 = crate::Reg<reg3::REG3_SPEC>;
#[doc = "DCDC Register 3"]
pub mod reg3;
