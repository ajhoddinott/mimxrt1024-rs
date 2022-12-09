#[doc = r"Register block"]
#[repr(C)]
pub struct SM {
    #[doc = "0x00 - Counter Register"]
    pub smcnt: SMCNT,
    #[doc = "0x02 - Initial Count Register"]
    pub sminit: SMINIT,
    #[doc = "0x04 - Control 2 Register"]
    pub smctrl2: SMCTRL2,
    #[doc = "0x06 - Control Register"]
    pub smctrl: SMCTRL,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - Value Register 0"]
    pub smval0: SMVAL0,
    #[doc = "0x0c - Fractional Value Register 1"]
    pub smfracval1: SMFRACVAL1,
    #[doc = "0x0e - Value Register 1"]
    pub smval1: SMVAL1,
    #[doc = "0x10 - Fractional Value Register 2"]
    pub smfracval2: SMFRACVAL2,
    #[doc = "0x12 - Value Register 2"]
    pub smval2: SMVAL2,
    #[doc = "0x14 - Fractional Value Register 3"]
    pub smfracval3: SMFRACVAL3,
    #[doc = "0x16 - Value Register 3"]
    pub smval3: SMVAL3,
    #[doc = "0x18 - Fractional Value Register 4"]
    pub smfracval4: SMFRACVAL4,
    #[doc = "0x1a - Value Register 4"]
    pub smval4: SMVAL4,
    #[doc = "0x1c - Fractional Value Register 5"]
    pub smfracval5: SMFRACVAL5,
    #[doc = "0x1e - Value Register 5"]
    pub smval5: SMVAL5,
    #[doc = "0x20 - Fractional Control Register"]
    pub smfrctrl: SMFRCTRL,
    #[doc = "0x22 - Output Control Register"]
    pub smoctrl: SMOCTRL,
    #[doc = "0x24 - Status Register"]
    pub smsts: SMSTS,
    #[doc = "0x26 - Interrupt Enable Register"]
    pub sminten: SMINTEN,
    #[doc = "0x28 - DMA Enable Register"]
    pub smdmaen: SMDMAEN,
    #[doc = "0x2a - Output Trigger Control Register"]
    pub smtctrl: SMTCTRL,
    #[doc = "0x2c - Fault Disable Mapping Register 0"]
    pub smdismap0: SMDISMAP0,
    _reserved22: [u8; 0x02],
    #[doc = "0x30 - Deadtime Count Register 0"]
    pub smdtcnt0: SMDTCNT0,
    #[doc = "0x32 - Deadtime Count Register 1"]
    pub smdtcnt1: SMDTCNT1,
    #[doc = "0x34 - Capture Control A Register"]
    pub smcaptctrla: SMCAPTCTRLA,
    #[doc = "0x36 - Capture Compare A Register"]
    pub smcaptcompa: SMCAPTCOMPA,
    #[doc = "0x38 - Capture Control B Register"]
    pub smcaptctrlb: SMCAPTCTRLB,
    #[doc = "0x3a - Capture Compare B Register"]
    pub smcaptcompb: SMCAPTCOMPB,
    #[doc = "0x3c - Capture Control X Register"]
    pub smcaptctrlx: SMCAPTCTRLX,
    #[doc = "0x3e - Capture Compare X Register"]
    pub smcaptcompx: SMCAPTCOMPX,
    #[doc = "0x40 - Capture Value 0 Register"]
    pub smcval0: SMCVAL0,
    #[doc = "0x42 - Capture Value 0 Cycle Register"]
    pub smcval0cyc: SMCVAL0CYC,
    #[doc = "0x44 - Capture Value 1 Register"]
    pub smcval1: SMCVAL1,
    #[doc = "0x46 - Capture Value 1 Cycle Register"]
    pub smcval1cyc: SMCVAL1CYC,
    #[doc = "0x48 - Capture Value 2 Register"]
    pub smcval2: SMCVAL2,
    #[doc = "0x4a - Capture Value 2 Cycle Register"]
    pub smcval2cyc: SMCVAL2CYC,
    #[doc = "0x4c - Capture Value 3 Register"]
    pub smcval3: SMCVAL3,
    #[doc = "0x4e - Capture Value 3 Cycle Register"]
    pub smcval3cyc: SMCVAL3CYC,
    #[doc = "0x50 - Capture Value 4 Register"]
    pub smcval4: SMCVAL4,
    #[doc = "0x52 - Capture Value 4 Cycle Register"]
    pub smcval4cyc: SMCVAL4CYC,
    #[doc = "0x54 - Capture Value 5 Register"]
    pub smcval5: SMCVAL5,
    #[doc = "0x56 - Capture Value 5 Cycle Register"]
    pub smcval5cyc: SMCVAL5CYC,
}
#[doc = "SMCNT (r) register accessor: an alias for `Reg<SMCNT_SPEC>`"]
pub type SMCNT = crate::Reg<smcnt::SMCNT_SPEC>;
#[doc = "Counter Register"]
pub mod smcnt;
#[doc = "SMINIT (rw) register accessor: an alias for `Reg<SMINIT_SPEC>`"]
pub type SMINIT = crate::Reg<sminit::SMINIT_SPEC>;
#[doc = "Initial Count Register"]
pub mod sminit;
#[doc = "SMCTRL2 (rw) register accessor: an alias for `Reg<SMCTRL2_SPEC>`"]
pub type SMCTRL2 = crate::Reg<smctrl2::SMCTRL2_SPEC>;
#[doc = "Control 2 Register"]
pub mod smctrl2;
#[doc = "SMCTRL (rw) register accessor: an alias for `Reg<SMCTRL_SPEC>`"]
pub type SMCTRL = crate::Reg<smctrl::SMCTRL_SPEC>;
#[doc = "Control Register"]
pub mod smctrl;
#[doc = "SMVAL0 (rw) register accessor: an alias for `Reg<SMVAL0_SPEC>`"]
pub type SMVAL0 = crate::Reg<smval0::SMVAL0_SPEC>;
#[doc = "Value Register 0"]
pub mod smval0;
#[doc = "SMFRACVAL1 (rw) register accessor: an alias for `Reg<SMFRACVAL1_SPEC>`"]
pub type SMFRACVAL1 = crate::Reg<smfracval1::SMFRACVAL1_SPEC>;
#[doc = "Fractional Value Register 1"]
pub mod smfracval1;
#[doc = "SMVAL1 (rw) register accessor: an alias for `Reg<SMVAL1_SPEC>`"]
pub type SMVAL1 = crate::Reg<smval1::SMVAL1_SPEC>;
#[doc = "Value Register 1"]
pub mod smval1;
#[doc = "SMFRACVAL2 (rw) register accessor: an alias for `Reg<SMFRACVAL2_SPEC>`"]
pub type SMFRACVAL2 = crate::Reg<smfracval2::SMFRACVAL2_SPEC>;
#[doc = "Fractional Value Register 2"]
pub mod smfracval2;
#[doc = "SMVAL2 (rw) register accessor: an alias for `Reg<SMVAL2_SPEC>`"]
pub type SMVAL2 = crate::Reg<smval2::SMVAL2_SPEC>;
#[doc = "Value Register 2"]
pub mod smval2;
#[doc = "SMFRACVAL3 (rw) register accessor: an alias for `Reg<SMFRACVAL3_SPEC>`"]
pub type SMFRACVAL3 = crate::Reg<smfracval3::SMFRACVAL3_SPEC>;
#[doc = "Fractional Value Register 3"]
pub mod smfracval3;
#[doc = "SMVAL3 (rw) register accessor: an alias for `Reg<SMVAL3_SPEC>`"]
pub type SMVAL3 = crate::Reg<smval3::SMVAL3_SPEC>;
#[doc = "Value Register 3"]
pub mod smval3;
#[doc = "SMFRACVAL4 (rw) register accessor: an alias for `Reg<SMFRACVAL4_SPEC>`"]
pub type SMFRACVAL4 = crate::Reg<smfracval4::SMFRACVAL4_SPEC>;
#[doc = "Fractional Value Register 4"]
pub mod smfracval4;
#[doc = "SMVAL4 (rw) register accessor: an alias for `Reg<SMVAL4_SPEC>`"]
pub type SMVAL4 = crate::Reg<smval4::SMVAL4_SPEC>;
#[doc = "Value Register 4"]
pub mod smval4;
#[doc = "SMFRACVAL5 (rw) register accessor: an alias for `Reg<SMFRACVAL5_SPEC>`"]
pub type SMFRACVAL5 = crate::Reg<smfracval5::SMFRACVAL5_SPEC>;
#[doc = "Fractional Value Register 5"]
pub mod smfracval5;
#[doc = "SMVAL5 (rw) register accessor: an alias for `Reg<SMVAL5_SPEC>`"]
pub type SMVAL5 = crate::Reg<smval5::SMVAL5_SPEC>;
#[doc = "Value Register 5"]
pub mod smval5;
#[doc = "SMFRCTRL (rw) register accessor: an alias for `Reg<SMFRCTRL_SPEC>`"]
pub type SMFRCTRL = crate::Reg<smfrctrl::SMFRCTRL_SPEC>;
#[doc = "Fractional Control Register"]
pub mod smfrctrl;
#[doc = "SMOCTRL (rw) register accessor: an alias for `Reg<SMOCTRL_SPEC>`"]
pub type SMOCTRL = crate::Reg<smoctrl::SMOCTRL_SPEC>;
#[doc = "Output Control Register"]
pub mod smoctrl;
#[doc = "SMSTS (rw) register accessor: an alias for `Reg<SMSTS_SPEC>`"]
pub type SMSTS = crate::Reg<smsts::SMSTS_SPEC>;
#[doc = "Status Register"]
pub mod smsts;
#[doc = "SMINTEN (rw) register accessor: an alias for `Reg<SMINTEN_SPEC>`"]
pub type SMINTEN = crate::Reg<sminten::SMINTEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod sminten;
#[doc = "SMDMAEN (rw) register accessor: an alias for `Reg<SMDMAEN_SPEC>`"]
pub type SMDMAEN = crate::Reg<smdmaen::SMDMAEN_SPEC>;
#[doc = "DMA Enable Register"]
pub mod smdmaen;
#[doc = "SMTCTRL (rw) register accessor: an alias for `Reg<SMTCTRL_SPEC>`"]
pub type SMTCTRL = crate::Reg<smtctrl::SMTCTRL_SPEC>;
#[doc = "Output Trigger Control Register"]
pub mod smtctrl;
#[doc = "SMDISMAP0 (rw) register accessor: an alias for `Reg<SMDISMAP0_SPEC>`"]
pub type SMDISMAP0 = crate::Reg<smdismap0::SMDISMAP0_SPEC>;
#[doc = "Fault Disable Mapping Register 0"]
pub mod smdismap0;
#[doc = "SMDTCNT0 (rw) register accessor: an alias for `Reg<SMDTCNT0_SPEC>`"]
pub type SMDTCNT0 = crate::Reg<smdtcnt0::SMDTCNT0_SPEC>;
#[doc = "Deadtime Count Register 0"]
pub mod smdtcnt0;
#[doc = "SMDTCNT1 (rw) register accessor: an alias for `Reg<SMDTCNT1_SPEC>`"]
pub type SMDTCNT1 = crate::Reg<smdtcnt1::SMDTCNT1_SPEC>;
#[doc = "Deadtime Count Register 1"]
pub mod smdtcnt1;
#[doc = "SMCAPTCTRLA (rw) register accessor: an alias for `Reg<SMCAPTCTRLA_SPEC>`"]
pub type SMCAPTCTRLA = crate::Reg<smcaptctrla::SMCAPTCTRLA_SPEC>;
#[doc = "Capture Control A Register"]
pub mod smcaptctrla;
#[doc = "SMCAPTCOMPA (rw) register accessor: an alias for `Reg<SMCAPTCOMPA_SPEC>`"]
pub type SMCAPTCOMPA = crate::Reg<smcaptcompa::SMCAPTCOMPA_SPEC>;
#[doc = "Capture Compare A Register"]
pub mod smcaptcompa;
#[doc = "SMCAPTCTRLB (rw) register accessor: an alias for `Reg<SMCAPTCTRLB_SPEC>`"]
pub type SMCAPTCTRLB = crate::Reg<smcaptctrlb::SMCAPTCTRLB_SPEC>;
#[doc = "Capture Control B Register"]
pub mod smcaptctrlb;
#[doc = "SMCAPTCOMPB (rw) register accessor: an alias for `Reg<SMCAPTCOMPB_SPEC>`"]
pub type SMCAPTCOMPB = crate::Reg<smcaptcompb::SMCAPTCOMPB_SPEC>;
#[doc = "Capture Compare B Register"]
pub mod smcaptcompb;
#[doc = "SMCAPTCTRLX (rw) register accessor: an alias for `Reg<SMCAPTCTRLX_SPEC>`"]
pub type SMCAPTCTRLX = crate::Reg<smcaptctrlx::SMCAPTCTRLX_SPEC>;
#[doc = "Capture Control X Register"]
pub mod smcaptctrlx;
#[doc = "SMCAPTCOMPX (rw) register accessor: an alias for `Reg<SMCAPTCOMPX_SPEC>`"]
pub type SMCAPTCOMPX = crate::Reg<smcaptcompx::SMCAPTCOMPX_SPEC>;
#[doc = "Capture Compare X Register"]
pub mod smcaptcompx;
#[doc = "SMCVAL0 (r) register accessor: an alias for `Reg<SMCVAL0_SPEC>`"]
pub type SMCVAL0 = crate::Reg<smcval0::SMCVAL0_SPEC>;
#[doc = "Capture Value 0 Register"]
pub mod smcval0;
#[doc = "SMCVAL0CYC (r) register accessor: an alias for `Reg<SMCVAL0CYC_SPEC>`"]
pub type SMCVAL0CYC = crate::Reg<smcval0cyc::SMCVAL0CYC_SPEC>;
#[doc = "Capture Value 0 Cycle Register"]
pub mod smcval0cyc;
#[doc = "SMCVAL1 (r) register accessor: an alias for `Reg<SMCVAL1_SPEC>`"]
pub type SMCVAL1 = crate::Reg<smcval1::SMCVAL1_SPEC>;
#[doc = "Capture Value 1 Register"]
pub mod smcval1;
#[doc = "SMCVAL1CYC (r) register accessor: an alias for `Reg<SMCVAL1CYC_SPEC>`"]
pub type SMCVAL1CYC = crate::Reg<smcval1cyc::SMCVAL1CYC_SPEC>;
#[doc = "Capture Value 1 Cycle Register"]
pub mod smcval1cyc;
#[doc = "SMCVAL2 (r) register accessor: an alias for `Reg<SMCVAL2_SPEC>`"]
pub type SMCVAL2 = crate::Reg<smcval2::SMCVAL2_SPEC>;
#[doc = "Capture Value 2 Register"]
pub mod smcval2;
#[doc = "SMCVAL2CYC (r) register accessor: an alias for `Reg<SMCVAL2CYC_SPEC>`"]
pub type SMCVAL2CYC = crate::Reg<smcval2cyc::SMCVAL2CYC_SPEC>;
#[doc = "Capture Value 2 Cycle Register"]
pub mod smcval2cyc;
#[doc = "SMCVAL3 (r) register accessor: an alias for `Reg<SMCVAL3_SPEC>`"]
pub type SMCVAL3 = crate::Reg<smcval3::SMCVAL3_SPEC>;
#[doc = "Capture Value 3 Register"]
pub mod smcval3;
#[doc = "SMCVAL3CYC (r) register accessor: an alias for `Reg<SMCVAL3CYC_SPEC>`"]
pub type SMCVAL3CYC = crate::Reg<smcval3cyc::SMCVAL3CYC_SPEC>;
#[doc = "Capture Value 3 Cycle Register"]
pub mod smcval3cyc;
#[doc = "SMCVAL4 (r) register accessor: an alias for `Reg<SMCVAL4_SPEC>`"]
pub type SMCVAL4 = crate::Reg<smcval4::SMCVAL4_SPEC>;
#[doc = "Capture Value 4 Register"]
pub mod smcval4;
#[doc = "SMCVAL4CYC (r) register accessor: an alias for `Reg<SMCVAL4CYC_SPEC>`"]
pub type SMCVAL4CYC = crate::Reg<smcval4cyc::SMCVAL4CYC_SPEC>;
#[doc = "Capture Value 4 Cycle Register"]
pub mod smcval4cyc;
#[doc = "SMCVAL5 (r) register accessor: an alias for `Reg<SMCVAL5_SPEC>`"]
pub type SMCVAL5 = crate::Reg<smcval5::SMCVAL5_SPEC>;
#[doc = "Capture Value 5 Register"]
pub mod smcval5;
#[doc = "SMCVAL5CYC (r) register accessor: an alias for `Reg<SMCVAL5CYC_SPEC>`"]
pub type SMCVAL5CYC = crate::Reg<smcval5cyc::SMCVAL5CYC_SPEC>;
#[doc = "Capture Value 5 Cycle Register"]
pub mod smcval5cyc;
