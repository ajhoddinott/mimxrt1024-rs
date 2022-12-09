#[doc = r"Register block"]
#[repr(C)]
pub struct TIMER {
    #[doc = "0x00 - Timer Load Value Register"]
    pub ldval: LDVAL,
    #[doc = "0x04 - Current Timer Value Register"]
    pub cval: CVAL,
    #[doc = "0x08 - Timer Control Register"]
    pub tctrl: TCTRL,
    #[doc = "0x0c - Timer Flag Register"]
    pub tflg: TFLG,
}
#[doc = "LDVAL (rw) register accessor: an alias for `Reg<LDVAL_SPEC>`"]
pub type LDVAL = crate::Reg<ldval::LDVAL_SPEC>;
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "CVAL (r) register accessor: an alias for `Reg<CVAL_SPEC>`"]
pub type CVAL = crate::Reg<cval::CVAL_SPEC>;
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "TCTRL (rw) register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "TFLG (rw) register accessor: an alias for `Reg<TFLG_SPEC>`"]
pub type TFLG = crate::Reg<tflg::TFLG_SPEC>;
#[doc = "Timer Flag Register"]
pub mod tflg;
