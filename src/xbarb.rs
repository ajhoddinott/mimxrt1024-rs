#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crossbar B Select Register 0"]
    pub sel0: SEL0,
    #[doc = "0x02 - Crossbar B Select Register 1"]
    pub sel1: SEL1,
    #[doc = "0x04 - Crossbar B Select Register 2"]
    pub sel2: SEL2,
    #[doc = "0x06 - Crossbar B Select Register 3"]
    pub sel3: SEL3,
    #[doc = "0x08 - Crossbar B Select Register 4"]
    pub sel4: SEL4,
    #[doc = "0x0a - Crossbar B Select Register 5"]
    pub sel5: SEL5,
    #[doc = "0x0c - Crossbar B Select Register 6"]
    pub sel6: SEL6,
    #[doc = "0x0e - Crossbar B Select Register 7"]
    pub sel7: SEL7,
}
#[doc = "SEL0 (rw) register accessor: an alias for `Reg<SEL0_SPEC>`"]
pub type SEL0 = crate::Reg<sel0::SEL0_SPEC>;
#[doc = "Crossbar B Select Register 0"]
pub mod sel0;
#[doc = "SEL1 (rw) register accessor: an alias for `Reg<SEL1_SPEC>`"]
pub type SEL1 = crate::Reg<sel1::SEL1_SPEC>;
#[doc = "Crossbar B Select Register 1"]
pub mod sel1;
#[doc = "SEL2 (rw) register accessor: an alias for `Reg<SEL2_SPEC>`"]
pub type SEL2 = crate::Reg<sel2::SEL2_SPEC>;
#[doc = "Crossbar B Select Register 2"]
pub mod sel2;
#[doc = "SEL3 (rw) register accessor: an alias for `Reg<SEL3_SPEC>`"]
pub type SEL3 = crate::Reg<sel3::SEL3_SPEC>;
#[doc = "Crossbar B Select Register 3"]
pub mod sel3;
#[doc = "SEL4 (rw) register accessor: an alias for `Reg<SEL4_SPEC>`"]
pub type SEL4 = crate::Reg<sel4::SEL4_SPEC>;
#[doc = "Crossbar B Select Register 4"]
pub mod sel4;
#[doc = "SEL5 (rw) register accessor: an alias for `Reg<SEL5_SPEC>`"]
pub type SEL5 = crate::Reg<sel5::SEL5_SPEC>;
#[doc = "Crossbar B Select Register 5"]
pub mod sel5;
#[doc = "SEL6 (rw) register accessor: an alias for `Reg<SEL6_SPEC>`"]
pub type SEL6 = crate::Reg<sel6::SEL6_SPEC>;
#[doc = "Crossbar B Select Register 6"]
pub mod sel6;
#[doc = "SEL7 (rw) register accessor: an alias for `Reg<SEL7_SPEC>`"]
pub type SEL7 = crate::Reg<sel7::SEL7_SPEC>;
#[doc = "Crossbar B Select Register 7"]
pub mod sel7;
