#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register"]
    pub sw_mux_ctl_pad_wakeup: SW_MUX_CTL_PAD_WAKEUP,
    #[doc = "0x04 - SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
    pub sw_mux_ctl_pad_pmic_on_req: SW_MUX_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x08 - SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register"]
    pub sw_mux_ctl_pad_pmic_stby_req: SW_MUX_CTL_PAD_PMIC_STBY_REQ,
    #[doc = "0x0c - SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
    pub sw_pad_ctl_pad_test_mode: SW_PAD_CTL_PAD_TEST_MODE,
    #[doc = "0x10 - SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
    pub sw_pad_ctl_pad_por_b: SW_PAD_CTL_PAD_POR_B,
    #[doc = "0x14 - SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
    pub sw_pad_ctl_pad_onoff: SW_PAD_CTL_PAD_ONOFF,
    #[doc = "0x18 - SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
    pub sw_pad_ctl_pad_wakeup: SW_PAD_CTL_PAD_WAKEUP,
    #[doc = "0x1c - SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
    pub sw_pad_ctl_pad_pmic_on_req: SW_PAD_CTL_PAD_PMIC_ON_REQ,
    #[doc = "0x20 - SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
    pub sw_pad_ctl_pad_pmic_stby_req: SW_PAD_CTL_PAD_PMIC_STBY_REQ,
}
#[doc = "SW_MUX_CTL_PAD_WAKEUP (rw) register accessor: an alias for `Reg<SW_MUX_CTL_PAD_WAKEUP_SPEC>`"]
pub type SW_MUX_CTL_PAD_WAKEUP = crate::Reg<sw_mux_ctl_pad_wakeup::SW_MUX_CTL_PAD_WAKEUP_SPEC>;
#[doc = "SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register"]
pub mod sw_mux_ctl_pad_wakeup;
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ (rw) register accessor: an alias for `Reg<SW_MUX_CTL_PAD_PMIC_ON_REQ_SPEC>`"]
pub type SW_MUX_CTL_PAD_PMIC_ON_REQ =
    crate::Reg<sw_mux_ctl_pad_pmic_on_req::SW_MUX_CTL_PAD_PMIC_ON_REQ_SPEC>;
#[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
pub mod sw_mux_ctl_pad_pmic_on_req;
#[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ (rw) register accessor: an alias for `Reg<SW_MUX_CTL_PAD_PMIC_STBY_REQ_SPEC>`"]
pub type SW_MUX_CTL_PAD_PMIC_STBY_REQ =
    crate::Reg<sw_mux_ctl_pad_pmic_stby_req::SW_MUX_CTL_PAD_PMIC_STBY_REQ_SPEC>;
#[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register"]
pub mod sw_mux_ctl_pad_pmic_stby_req;
#[doc = "SW_PAD_CTL_PAD_TEST_MODE (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_TEST_MODE_SPEC>`"]
pub type SW_PAD_CTL_PAD_TEST_MODE =
    crate::Reg<sw_pad_ctl_pad_test_mode::SW_PAD_CTL_PAD_TEST_MODE_SPEC>;
#[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
pub mod sw_pad_ctl_pad_test_mode;
#[doc = "SW_PAD_CTL_PAD_POR_B (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_POR_B_SPEC>`"]
pub type SW_PAD_CTL_PAD_POR_B = crate::Reg<sw_pad_ctl_pad_por_b::SW_PAD_CTL_PAD_POR_B_SPEC>;
#[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
pub mod sw_pad_ctl_pad_por_b;
#[doc = "SW_PAD_CTL_PAD_ONOFF (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_ONOFF_SPEC>`"]
pub type SW_PAD_CTL_PAD_ONOFF = crate::Reg<sw_pad_ctl_pad_onoff::SW_PAD_CTL_PAD_ONOFF_SPEC>;
#[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
pub mod sw_pad_ctl_pad_onoff;
#[doc = "SW_PAD_CTL_PAD_WAKEUP (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_WAKEUP_SPEC>`"]
pub type SW_PAD_CTL_PAD_WAKEUP = crate::Reg<sw_pad_ctl_pad_wakeup::SW_PAD_CTL_PAD_WAKEUP_SPEC>;
#[doc = "SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
pub mod sw_pad_ctl_pad_wakeup;
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_PMIC_ON_REQ_SPEC>`"]
pub type SW_PAD_CTL_PAD_PMIC_ON_REQ =
    crate::Reg<sw_pad_ctl_pad_pmic_on_req::SW_PAD_CTL_PAD_PMIC_ON_REQ_SPEC>;
#[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
pub mod sw_pad_ctl_pad_pmic_on_req;
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ (rw) register accessor: an alias for `Reg<SW_PAD_CTL_PAD_PMIC_STBY_REQ_SPEC>`"]
pub type SW_PAD_CTL_PAD_PMIC_STBY_REQ =
    crate::Reg<sw_pad_ctl_pad_pmic_stby_req::SW_PAD_CTL_PAD_PMIC_STBY_REQ_SPEC>;
#[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
pub mod sw_pad_ctl_pad_pmic_stby_req;
