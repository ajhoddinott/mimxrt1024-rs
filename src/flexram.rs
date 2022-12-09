#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCM CRTL Register"]
    pub tcm_ctrl: TCM_CTRL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    #[doc = "0x14 - Interrupt Status Enable Register"]
    pub int_stat_en: INT_STAT_EN,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub int_sig_en: INT_SIG_EN,
}
#[doc = "TCM_CTRL (rw) register accessor: an alias for `Reg<TCM_CTRL_SPEC>`"]
pub type TCM_CTRL = crate::Reg<tcm_ctrl::TCM_CTRL_SPEC>;
#[doc = "TCM CRTL Register"]
pub mod tcm_ctrl;
#[doc = "INT_STATUS (rw) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "INT_STAT_EN (rw) register accessor: an alias for `Reg<INT_STAT_EN_SPEC>`"]
pub type INT_STAT_EN = crate::Reg<int_stat_en::INT_STAT_EN_SPEC>;
#[doc = "Interrupt Status Enable Register"]
pub mod int_stat_en;
#[doc = "INT_SIG_EN (rw) register accessor: an alias for `Reg<INT_SIG_EN_SPEC>`"]
pub type INT_SIG_EN = crate::Reg<int_sig_en::INT_SIG_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_sig_en;
