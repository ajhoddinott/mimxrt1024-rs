#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0150],
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    _reserved4: [u8; 0x0110],
    #[doc = "0x270 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl: LOWPWR_CTRL,
    #[doc = "0x274 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_set: LOWPWR_CTRL_SET,
    #[doc = "0x278 - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_clr: LOWPWR_CTRL_CLR,
    #[doc = "0x27c - XTAL OSC (LP) Control Register"]
    pub lowpwr_ctrl_tog: LOWPWR_CTRL_TOG,
    _reserved8: [u8; 0x20],
    #[doc = "0x2a0 - XTAL OSC Configuration 0 Register"]
    pub osc_config0: OSC_CONFIG0,
    #[doc = "0x2a4 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_set: OSC_CONFIG0_SET,
    #[doc = "0x2a8 - XTAL OSC Configuration 0 Register"]
    pub osc_config0_clr: OSC_CONFIG0_CLR,
    #[doc = "0x2ac - XTAL OSC Configuration 0 Register"]
    pub osc_config0_tog: OSC_CONFIG0_TOG,
    #[doc = "0x2b0 - XTAL OSC Configuration 1 Register"]
    pub osc_config1: OSC_CONFIG1,
    #[doc = "0x2b4 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_set: OSC_CONFIG1_SET,
    #[doc = "0x2b8 - XTAL OSC Configuration 1 Register"]
    pub osc_config1_clr: OSC_CONFIG1_CLR,
    #[doc = "0x2bc - XTAL OSC Configuration 1 Register"]
    pub osc_config1_tog: OSC_CONFIG1_TOG,
    #[doc = "0x2c0 - XTAL OSC Configuration 2 Register"]
    pub osc_config2: OSC_CONFIG2,
    #[doc = "0x2c4 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_set: OSC_CONFIG2_SET,
    #[doc = "0x2c8 - XTAL OSC Configuration 2 Register"]
    pub osc_config2_clr: OSC_CONFIG2_CLR,
    #[doc = "0x2cc - XTAL OSC Configuration 2 Register"]
    pub osc_config2_tog: OSC_CONFIG2_TOG,
}
#[doc = "MISC0 (rw) register accessor: an alias for `Reg<MISC0_SPEC>`"]
pub type MISC0 = crate::Reg<misc0::MISC0_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "MISC0_SET (rw) register accessor: an alias for `Reg<MISC0_SET_SPEC>`"]
pub type MISC0_SET = crate::Reg<misc0_set::MISC0_SET_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "MISC0_CLR (rw) register accessor: an alias for `Reg<MISC0_CLR_SPEC>`"]
pub type MISC0_CLR = crate::Reg<misc0_clr::MISC0_CLR_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "MISC0_TOG (rw) register accessor: an alias for `Reg<MISC0_TOG_SPEC>`"]
pub type MISC0_TOG = crate::Reg<misc0_tog::MISC0_TOG_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "LOWPWR_CTRL (rw) register accessor: an alias for `Reg<LOWPWR_CTRL_SPEC>`"]
pub type LOWPWR_CTRL = crate::Reg<lowpwr_ctrl::LOWPWR_CTRL_SPEC>;
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl;
#[doc = "LOWPWR_CTRL_SET (rw) register accessor: an alias for `Reg<LOWPWR_CTRL_SET_SPEC>`"]
pub type LOWPWR_CTRL_SET = crate::Reg<lowpwr_ctrl_set::LOWPWR_CTRL_SET_SPEC>;
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_set;
#[doc = "LOWPWR_CTRL_CLR (rw) register accessor: an alias for `Reg<LOWPWR_CTRL_CLR_SPEC>`"]
pub type LOWPWR_CTRL_CLR = crate::Reg<lowpwr_ctrl_clr::LOWPWR_CTRL_CLR_SPEC>;
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_clr;
#[doc = "LOWPWR_CTRL_TOG (rw) register accessor: an alias for `Reg<LOWPWR_CTRL_TOG_SPEC>`"]
pub type LOWPWR_CTRL_TOG = crate::Reg<lowpwr_ctrl_tog::LOWPWR_CTRL_TOG_SPEC>;
#[doc = "XTAL OSC (LP) Control Register"]
pub mod lowpwr_ctrl_tog;
#[doc = "OSC_CONFIG0 (rw) register accessor: an alias for `Reg<OSC_CONFIG0_SPEC>`"]
pub type OSC_CONFIG0 = crate::Reg<osc_config0::OSC_CONFIG0_SPEC>;
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0;
#[doc = "OSC_CONFIG0_SET (rw) register accessor: an alias for `Reg<OSC_CONFIG0_SET_SPEC>`"]
pub type OSC_CONFIG0_SET = crate::Reg<osc_config0_set::OSC_CONFIG0_SET_SPEC>;
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_set;
#[doc = "OSC_CONFIG0_CLR (rw) register accessor: an alias for `Reg<OSC_CONFIG0_CLR_SPEC>`"]
pub type OSC_CONFIG0_CLR = crate::Reg<osc_config0_clr::OSC_CONFIG0_CLR_SPEC>;
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_clr;
#[doc = "OSC_CONFIG0_TOG (rw) register accessor: an alias for `Reg<OSC_CONFIG0_TOG_SPEC>`"]
pub type OSC_CONFIG0_TOG = crate::Reg<osc_config0_tog::OSC_CONFIG0_TOG_SPEC>;
#[doc = "XTAL OSC Configuration 0 Register"]
pub mod osc_config0_tog;
#[doc = "OSC_CONFIG1 (rw) register accessor: an alias for `Reg<OSC_CONFIG1_SPEC>`"]
pub type OSC_CONFIG1 = crate::Reg<osc_config1::OSC_CONFIG1_SPEC>;
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1;
#[doc = "OSC_CONFIG1_SET (rw) register accessor: an alias for `Reg<OSC_CONFIG1_SET_SPEC>`"]
pub type OSC_CONFIG1_SET = crate::Reg<osc_config1_set::OSC_CONFIG1_SET_SPEC>;
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_set;
#[doc = "OSC_CONFIG1_CLR (rw) register accessor: an alias for `Reg<OSC_CONFIG1_CLR_SPEC>`"]
pub type OSC_CONFIG1_CLR = crate::Reg<osc_config1_clr::OSC_CONFIG1_CLR_SPEC>;
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_clr;
#[doc = "OSC_CONFIG1_TOG (rw) register accessor: an alias for `Reg<OSC_CONFIG1_TOG_SPEC>`"]
pub type OSC_CONFIG1_TOG = crate::Reg<osc_config1_tog::OSC_CONFIG1_TOG_SPEC>;
#[doc = "XTAL OSC Configuration 1 Register"]
pub mod osc_config1_tog;
#[doc = "OSC_CONFIG2 (rw) register accessor: an alias for `Reg<OSC_CONFIG2_SPEC>`"]
pub type OSC_CONFIG2 = crate::Reg<osc_config2::OSC_CONFIG2_SPEC>;
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2;
#[doc = "OSC_CONFIG2_SET (rw) register accessor: an alias for `Reg<OSC_CONFIG2_SET_SPEC>`"]
pub type OSC_CONFIG2_SET = crate::Reg<osc_config2_set::OSC_CONFIG2_SET_SPEC>;
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_set;
#[doc = "OSC_CONFIG2_CLR (rw) register accessor: an alias for `Reg<OSC_CONFIG2_CLR_SPEC>`"]
pub type OSC_CONFIG2_CLR = crate::Reg<osc_config2_clr::OSC_CONFIG2_CLR_SPEC>;
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_clr;
#[doc = "OSC_CONFIG2_TOG (rw) register accessor: an alias for `Reg<OSC_CONFIG2_TOG_SPEC>`"]
pub type OSC_CONFIG2_TOG = crate::Reg<osc_config2_tog::OSC_CONFIG2_TOG_SPEC>;
#[doc = "XTAL OSC Configuration 2 Register"]
pub mod osc_config2_tog;
