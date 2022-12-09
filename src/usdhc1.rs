#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address"]
    pub ds_addr: DS_ADDR,
    #[doc = "0x04 - Block Attributes"]
    pub blk_att: BLK_ATT,
    #[doc = "0x08 - Command Argument"]
    pub cmd_arg: CMD_ARG,
    #[doc = "0x0c - Command Transfer Type"]
    pub cmd_xfr_typ: CMD_XFR_TYP,
    #[doc = "0x10 - Command Response0"]
    pub cmd_rsp0: CMD_RSP0,
    #[doc = "0x14 - Command Response1"]
    pub cmd_rsp1: CMD_RSP1,
    #[doc = "0x18 - Command Response2"]
    pub cmd_rsp2: CMD_RSP2,
    #[doc = "0x1c - Command Response3"]
    pub cmd_rsp3: CMD_RSP3,
    #[doc = "0x20 - Data Buffer Access Port"]
    pub data_buff_acc_port: DATA_BUFF_ACC_PORT,
    #[doc = "0x24 - Present State"]
    pub pres_state: PRES_STATE,
    #[doc = "0x28 - Protocol Control"]
    pub prot_ctrl: PROT_CTRL,
    #[doc = "0x2c - System Control"]
    pub sys_ctrl: SYS_CTRL,
    #[doc = "0x30 - Interrupt Status"]
    pub int_status: INT_STATUS,
    #[doc = "0x34 - Interrupt Status Enable"]
    pub int_status_en: INT_STATUS_EN,
    #[doc = "0x38 - Interrupt Signal Enable"]
    pub int_signal_en: INT_SIGNAL_EN,
    #[doc = "0x3c - Auto CMD12 Error Status"]
    pub autocmd12_err_status: AUTOCMD12_ERR_STATUS,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub host_ctrl_cap: HOST_CTRL_CAP,
    #[doc = "0x44 - Watermark Level"]
    pub wtmk_lvl: WTMK_LVL,
    #[doc = "0x48 - Mixer Control"]
    pub mix_ctrl: MIX_CTRL,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - Force Event"]
    pub force_event: FORCE_EVENT,
    #[doc = "0x54 - ADMA Error Status"]
    pub adma_err_status: ADMA_ERR_STATUS,
    #[doc = "0x58 - ADMA System Address"]
    pub adma_sys_addr: ADMA_SYS_ADDR,
    _reserved22: [u8; 0x04],
    #[doc = "0x60 - DLL (Delay Line) Control"]
    pub dll_ctrl: DLL_CTRL,
    #[doc = "0x64 - DLL Status"]
    pub dll_status: DLL_STATUS,
    #[doc = "0x68 - CLK Tuning Control and Status"]
    pub clk_tune_ctrl_status: CLK_TUNE_CTRL_STATUS,
    _reserved25: [u8; 0x54],
    #[doc = "0xc0 - Vendor Specific Register"]
    pub vend_spec: VEND_SPEC,
    #[doc = "0xc4 - MMC Boot"]
    pub mmc_boot: MMC_BOOT,
    #[doc = "0xc8 - Vendor Specific 2 Register"]
    pub vend_spec2: VEND_SPEC2,
    #[doc = "0xcc - Tuning Control"]
    pub tuning_ctrl: TUNING_CTRL,
}
#[doc = "DS_ADDR (rw) register accessor: an alias for `Reg<DS_ADDR_SPEC>`"]
pub type DS_ADDR = crate::Reg<ds_addr::DS_ADDR_SPEC>;
#[doc = "DMA System Address"]
pub mod ds_addr;
#[doc = "BLK_ATT (rw) register accessor: an alias for `Reg<BLK_ATT_SPEC>`"]
pub type BLK_ATT = crate::Reg<blk_att::BLK_ATT_SPEC>;
#[doc = "Block Attributes"]
pub mod blk_att;
#[doc = "CMD_ARG (rw) register accessor: an alias for `Reg<CMD_ARG_SPEC>`"]
pub type CMD_ARG = crate::Reg<cmd_arg::CMD_ARG_SPEC>;
#[doc = "Command Argument"]
pub mod cmd_arg;
#[doc = "CMD_XFR_TYP (rw) register accessor: an alias for `Reg<CMD_XFR_TYP_SPEC>`"]
pub type CMD_XFR_TYP = crate::Reg<cmd_xfr_typ::CMD_XFR_TYP_SPEC>;
#[doc = "Command Transfer Type"]
pub mod cmd_xfr_typ;
#[doc = "CMD_RSP0 (r) register accessor: an alias for `Reg<CMD_RSP0_SPEC>`"]
pub type CMD_RSP0 = crate::Reg<cmd_rsp0::CMD_RSP0_SPEC>;
#[doc = "Command Response0"]
pub mod cmd_rsp0;
#[doc = "CMD_RSP1 (r) register accessor: an alias for `Reg<CMD_RSP1_SPEC>`"]
pub type CMD_RSP1 = crate::Reg<cmd_rsp1::CMD_RSP1_SPEC>;
#[doc = "Command Response1"]
pub mod cmd_rsp1;
#[doc = "CMD_RSP2 (r) register accessor: an alias for `Reg<CMD_RSP2_SPEC>`"]
pub type CMD_RSP2 = crate::Reg<cmd_rsp2::CMD_RSP2_SPEC>;
#[doc = "Command Response2"]
pub mod cmd_rsp2;
#[doc = "CMD_RSP3 (r) register accessor: an alias for `Reg<CMD_RSP3_SPEC>`"]
pub type CMD_RSP3 = crate::Reg<cmd_rsp3::CMD_RSP3_SPEC>;
#[doc = "Command Response3"]
pub mod cmd_rsp3;
#[doc = "DATA_BUFF_ACC_PORT (rw) register accessor: an alias for `Reg<DATA_BUFF_ACC_PORT_SPEC>`"]
pub type DATA_BUFF_ACC_PORT = crate::Reg<data_buff_acc_port::DATA_BUFF_ACC_PORT_SPEC>;
#[doc = "Data Buffer Access Port"]
pub mod data_buff_acc_port;
#[doc = "PRES_STATE (r) register accessor: an alias for `Reg<PRES_STATE_SPEC>`"]
pub type PRES_STATE = crate::Reg<pres_state::PRES_STATE_SPEC>;
#[doc = "Present State"]
pub mod pres_state;
#[doc = "PROT_CTRL (rw) register accessor: an alias for `Reg<PROT_CTRL_SPEC>`"]
pub type PROT_CTRL = crate::Reg<prot_ctrl::PROT_CTRL_SPEC>;
#[doc = "Protocol Control"]
pub mod prot_ctrl;
#[doc = "SYS_CTRL (rw) register accessor: an alias for `Reg<SYS_CTRL_SPEC>`"]
pub type SYS_CTRL = crate::Reg<sys_ctrl::SYS_CTRL_SPEC>;
#[doc = "System Control"]
pub mod sys_ctrl;
#[doc = "INT_STATUS (rw) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "INT_STATUS_EN (rw) register accessor: an alias for `Reg<INT_STATUS_EN_SPEC>`"]
pub type INT_STATUS_EN = crate::Reg<int_status_en::INT_STATUS_EN_SPEC>;
#[doc = "Interrupt Status Enable"]
pub mod int_status_en;
#[doc = "INT_SIGNAL_EN (rw) register accessor: an alias for `Reg<INT_SIGNAL_EN_SPEC>`"]
pub type INT_SIGNAL_EN = crate::Reg<int_signal_en::INT_SIGNAL_EN_SPEC>;
#[doc = "Interrupt Signal Enable"]
pub mod int_signal_en;
#[doc = "AUTOCMD12_ERR_STATUS (rw) register accessor: an alias for `Reg<AUTOCMD12_ERR_STATUS_SPEC>`"]
pub type AUTOCMD12_ERR_STATUS = crate::Reg<autocmd12_err_status::AUTOCMD12_ERR_STATUS_SPEC>;
#[doc = "Auto CMD12 Error Status"]
pub mod autocmd12_err_status;
#[doc = "HOST_CTRL_CAP (rw) register accessor: an alias for `Reg<HOST_CTRL_CAP_SPEC>`"]
pub type HOST_CTRL_CAP = crate::Reg<host_ctrl_cap::HOST_CTRL_CAP_SPEC>;
#[doc = "Host Controller Capabilities"]
pub mod host_ctrl_cap;
#[doc = "WTMK_LVL (rw) register accessor: an alias for `Reg<WTMK_LVL_SPEC>`"]
pub type WTMK_LVL = crate::Reg<wtmk_lvl::WTMK_LVL_SPEC>;
#[doc = "Watermark Level"]
pub mod wtmk_lvl;
#[doc = "MIX_CTRL (rw) register accessor: an alias for `Reg<MIX_CTRL_SPEC>`"]
pub type MIX_CTRL = crate::Reg<mix_ctrl::MIX_CTRL_SPEC>;
#[doc = "Mixer Control"]
pub mod mix_ctrl;
#[doc = "FORCE_EVENT (rw) register accessor: an alias for `Reg<FORCE_EVENT_SPEC>`"]
pub type FORCE_EVENT = crate::Reg<force_event::FORCE_EVENT_SPEC>;
#[doc = "Force Event"]
pub mod force_event;
#[doc = "ADMA_ERR_STATUS (r) register accessor: an alias for `Reg<ADMA_ERR_STATUS_SPEC>`"]
pub type ADMA_ERR_STATUS = crate::Reg<adma_err_status::ADMA_ERR_STATUS_SPEC>;
#[doc = "ADMA Error Status"]
pub mod adma_err_status;
#[doc = "ADMA_SYS_ADDR (rw) register accessor: an alias for `Reg<ADMA_SYS_ADDR_SPEC>`"]
pub type ADMA_SYS_ADDR = crate::Reg<adma_sys_addr::ADMA_SYS_ADDR_SPEC>;
#[doc = "ADMA System Address"]
pub mod adma_sys_addr;
#[doc = "DLL_CTRL (rw) register accessor: an alias for `Reg<DLL_CTRL_SPEC>`"]
pub type DLL_CTRL = crate::Reg<dll_ctrl::DLL_CTRL_SPEC>;
#[doc = "DLL (Delay Line) Control"]
pub mod dll_ctrl;
#[doc = "DLL_STATUS (r) register accessor: an alias for `Reg<DLL_STATUS_SPEC>`"]
pub type DLL_STATUS = crate::Reg<dll_status::DLL_STATUS_SPEC>;
#[doc = "DLL Status"]
pub mod dll_status;
#[doc = "CLK_TUNE_CTRL_STATUS (rw) register accessor: an alias for `Reg<CLK_TUNE_CTRL_STATUS_SPEC>`"]
pub type CLK_TUNE_CTRL_STATUS = crate::Reg<clk_tune_ctrl_status::CLK_TUNE_CTRL_STATUS_SPEC>;
#[doc = "CLK Tuning Control and Status"]
pub mod clk_tune_ctrl_status;
#[doc = "VEND_SPEC (rw) register accessor: an alias for `Reg<VEND_SPEC_SPEC>`"]
pub type VEND_SPEC = crate::Reg<vend_spec::VEND_SPEC_SPEC>;
#[doc = "Vendor Specific Register"]
pub mod vend_spec;
#[doc = "MMC_BOOT (rw) register accessor: an alias for `Reg<MMC_BOOT_SPEC>`"]
pub type MMC_BOOT = crate::Reg<mmc_boot::MMC_BOOT_SPEC>;
#[doc = "MMC Boot"]
pub mod mmc_boot;
#[doc = "VEND_SPEC2 (rw) register accessor: an alias for `Reg<VEND_SPEC2_SPEC>`"]
pub type VEND_SPEC2 = crate::Reg<vend_spec2::VEND_SPEC2_SPEC>;
#[doc = "Vendor Specific 2 Register"]
pub mod vend_spec2;
#[doc = "TUNING_CTRL (rw) register accessor: an alias for `Reg<TUNING_CTRL_SPEC>`"]
pub type TUNING_CTRL = crate::Reg<tuning_ctrl::TUNING_CTRL_SPEC>;
#[doc = "Tuning Control"]
pub mod tuning_ctrl;
