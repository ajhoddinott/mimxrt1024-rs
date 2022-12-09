#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0180],
    #[doc = "0x180 - Tempsensor Control Register 0"]
    pub tempsense0: TEMPSENSE0,
    #[doc = "0x184 - Tempsensor Control Register 0"]
    pub tempsense0_set: TEMPSENSE0_SET,
    #[doc = "0x188 - Tempsensor Control Register 0"]
    pub tempsense0_clr: TEMPSENSE0_CLR,
    #[doc = "0x18c - Tempsensor Control Register 0"]
    pub tempsense0_tog: TEMPSENSE0_TOG,
    #[doc = "0x190 - Tempsensor Control Register 1"]
    pub tempsense1: TEMPSENSE1,
    #[doc = "0x194 - Tempsensor Control Register 1"]
    pub tempsense1_set: TEMPSENSE1_SET,
    #[doc = "0x198 - Tempsensor Control Register 1"]
    pub tempsense1_clr: TEMPSENSE1_CLR,
    #[doc = "0x19c - Tempsensor Control Register 1"]
    pub tempsense1_tog: TEMPSENSE1_TOG,
    _reserved8: [u8; 0xf0],
    #[doc = "0x290 - Tempsensor Control Register 2"]
    pub tempsense2: TEMPSENSE2,
    #[doc = "0x294 - Tempsensor Control Register 2"]
    pub tempsense2_set: TEMPSENSE2_SET,
    #[doc = "0x298 - Tempsensor Control Register 2"]
    pub tempsense2_clr: TEMPSENSE2_CLR,
    #[doc = "0x29c - Tempsensor Control Register 2"]
    pub tempsense2_tog: TEMPSENSE2_TOG,
}
#[doc = "TEMPSENSE0 (rw) register accessor: an alias for `Reg<TEMPSENSE0_SPEC>`"]
pub type TEMPSENSE0 = crate::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE0_SET (rw) register accessor: an alias for `Reg<TEMPSENSE0_SET_SPEC>`"]
pub type TEMPSENSE0_SET = crate::Reg<tempsense0_set::TEMPSENSE0_SET_SPEC>;
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_set;
#[doc = "TEMPSENSE0_CLR (rw) register accessor: an alias for `Reg<TEMPSENSE0_CLR_SPEC>`"]
pub type TEMPSENSE0_CLR = crate::Reg<tempsense0_clr::TEMPSENSE0_CLR_SPEC>;
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_clr;
#[doc = "TEMPSENSE0_TOG (rw) register accessor: an alias for `Reg<TEMPSENSE0_TOG_SPEC>`"]
pub type TEMPSENSE0_TOG = crate::Reg<tempsense0_tog::TEMPSENSE0_TOG_SPEC>;
#[doc = "Tempsensor Control Register 0"]
pub mod tempsense0_tog;
#[doc = "TEMPSENSE1 (rw) register accessor: an alias for `Reg<TEMPSENSE1_SPEC>`"]
pub type TEMPSENSE1 = crate::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1;
#[doc = "TEMPSENSE1_SET (rw) register accessor: an alias for `Reg<TEMPSENSE1_SET_SPEC>`"]
pub type TEMPSENSE1_SET = crate::Reg<tempsense1_set::TEMPSENSE1_SET_SPEC>;
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_set;
#[doc = "TEMPSENSE1_CLR (rw) register accessor: an alias for `Reg<TEMPSENSE1_CLR_SPEC>`"]
pub type TEMPSENSE1_CLR = crate::Reg<tempsense1_clr::TEMPSENSE1_CLR_SPEC>;
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_clr;
#[doc = "TEMPSENSE1_TOG (rw) register accessor: an alias for `Reg<TEMPSENSE1_TOG_SPEC>`"]
pub type TEMPSENSE1_TOG = crate::Reg<tempsense1_tog::TEMPSENSE1_TOG_SPEC>;
#[doc = "Tempsensor Control Register 1"]
pub mod tempsense1_tog;
#[doc = "TEMPSENSE2 (rw) register accessor: an alias for `Reg<TEMPSENSE2_SPEC>`"]
pub type TEMPSENSE2 = crate::Reg<tempsense2::TEMPSENSE2_SPEC>;
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2;
#[doc = "TEMPSENSE2_SET (rw) register accessor: an alias for `Reg<TEMPSENSE2_SET_SPEC>`"]
pub type TEMPSENSE2_SET = crate::Reg<tempsense2_set::TEMPSENSE2_SET_SPEC>;
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_set;
#[doc = "TEMPSENSE2_CLR (rw) register accessor: an alias for `Reg<TEMPSENSE2_CLR_SPEC>`"]
pub type TEMPSENSE2_CLR = crate::Reg<tempsense2_clr::TEMPSENSE2_CLR_SPEC>;
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_clr;
#[doc = "TEMPSENSE2_TOG (rw) register accessor: an alias for `Reg<TEMPSENSE2_TOG_SPEC>`"]
pub type TEMPSENSE2_TOG = crate::Reg<tempsense2_tog::TEMPSENSE2_TOG_SPEC>;
#[doc = "Tempsensor Control Register 2"]
pub mod tempsense2_tog;
