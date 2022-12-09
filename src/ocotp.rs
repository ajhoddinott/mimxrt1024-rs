#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTP Controller Control and Status Register"]
    pub hw_ocotp_ctrl: HW_OCOTP_CTRL,
    #[doc = "0x04 - OTP Controller Control and Status Register"]
    pub hw_ocotp_ctrl_set: HW_OCOTP_CTRL_SET,
    #[doc = "0x08 - OTP Controller Control and Status Register"]
    pub hw_ocotp_ctrl_clr: HW_OCOTP_CTRL_CLR,
    #[doc = "0x0c - OTP Controller Control and Status Register"]
    pub hw_ocotp_ctrl_tog: HW_OCOTP_CTRL_TOG,
    #[doc = "0x10 - OTP Controller Timing Register"]
    pub hw_ocotp_timing: HW_OCOTP_TIMING,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - OTP Controller Write Data Register"]
    pub hw_ocotp_data: HW_OCOTP_DATA,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - OTP Controller Write Data Register"]
    pub hw_ocotp_read_ctrl: HW_OCOTP_READ_CTRL,
    _reserved7: [u8; 0x0c],
    #[doc = "0x40 - OTP Controller Read Data Register"]
    pub hw_ocotp_read_fuse_data: HW_OCOTP_READ_FUSE_DATA,
    _reserved8: [u8; 0x0c],
    #[doc = "0x50 - Sticky bit Register"]
    pub hw_ocotp_sw_sticky: HW_OCOTP_SW_STICKY,
    _reserved9: [u8; 0x0c],
    #[doc = "0x60 - Software Controllable Signals Register"]
    pub hw_ocotp_scs: HW_OCOTP_SCS,
    #[doc = "0x64 - Software Controllable Signals Register"]
    pub hw_ocotp_scs_set: HW_OCOTP_SCS_SET,
    #[doc = "0x68 - Software Controllable Signals Register"]
    pub hw_ocotp_scs_clr: HW_OCOTP_SCS_CLR,
    #[doc = "0x6c - Software Controllable Signals Register"]
    pub hw_ocotp_scs_tog: HW_OCOTP_SCS_TOG,
    _reserved13: [u8; 0x20],
    #[doc = "0x90 - OTP Controller Version Register"]
    pub hw_ocotp_version: HW_OCOTP_VERSION,
    _reserved14: [u8; 0x6c],
    #[doc = "0x100 - OTP Controller Timing Register 2"]
    pub hw_ocotp_timing2: HW_OCOTP_TIMING2,
    _reserved15: [u8; 0x02fc],
    #[doc = "0x400 - Value of OTP Bank0 Word0 (Lock controls)"]
    pub hw_ocotp_lock: HW_OCOTP_LOCK,
    _reserved16: [u8; 0x0c],
    #[doc = "0x410 - Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg0: HW_OCOTP_CFG0,
    _reserved17: [u8; 0x0c],
    #[doc = "0x420 - Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg1: HW_OCOTP_CFG1,
    _reserved18: [u8; 0x0c],
    #[doc = "0x430 - Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg2: HW_OCOTP_CFG2,
    _reserved19: [u8; 0x0c],
    #[doc = "0x440 - Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg3: HW_OCOTP_CFG3,
    _reserved20: [u8; 0x0c],
    #[doc = "0x450 - Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg4: HW_OCOTP_CFG4,
    _reserved21: [u8; 0x0c],
    #[doc = "0x460 - Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg5: HW_OCOTP_CFG5,
    _reserved22: [u8; 0x0c],
    #[doc = "0x470 - Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    pub hw_ocotp_cfg6: HW_OCOTP_CFG6,
    _reserved23: [u8; 0x0c],
    #[doc = "0x480 - Value of OTP Bank1 Word0 (Memory Related Info.)"]
    pub hw_ocotp_mem0: HW_OCOTP_MEM0,
    _reserved24: [u8; 0x0c],
    #[doc = "0x490 - Value of OTP Bank1 Word1 (Memory Related Info.)"]
    pub hw_ocotp_mem1: HW_OCOTP_MEM1,
    _reserved25: [u8; 0x0c],
    #[doc = "0x4a0 - Value of OTP Bank1 Word2 (Memory Related Info.)"]
    pub hw_ocotp_mem2: HW_OCOTP_MEM2,
    _reserved26: [u8; 0x0c],
    #[doc = "0x4b0 - Value of OTP Bank1 Word3 (Memory Related Info.)"]
    pub hw_ocotp_mem3: HW_OCOTP_MEM3,
    _reserved27: [u8; 0x0c],
    #[doc = "0x4c0 - Value of OTP Bank 1 Word 4 (Memory Related Info.)"]
    pub hw_ocotp_mem4: HW_OCOTP_MEM4,
    _reserved28: [u8; 0x0c],
    #[doc = "0x4d0 - Value of OTP Bank 1 Word 5 (Analog Info.)"]
    pub hw_ocotp_ana0: HW_OCOTP_ANA0,
    _reserved29: [u8; 0x0c],
    #[doc = "0x4e0 - Value of OTP Bank 1 Word 6 (Analog Info.)"]
    pub hw_ocotp_ana1: HW_OCOTP_ANA1,
    _reserved30: [u8; 0x0c],
    #[doc = "0x4f0 - Value of OTP Bank 1 Word 7 (Analog Info.)"]
    pub hw_ocotp_ana2: HW_OCOTP_ANA2,
    _reserved31: [u8; 0x8c],
    #[doc = "0x580 - Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    pub hw_ocotp_srk0: HW_OCOTP_SRK0,
    _reserved32: [u8; 0x0c],
    #[doc = "0x590 - Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    pub hw_ocotp_srk1: HW_OCOTP_SRK1,
    _reserved33: [u8; 0x0c],
    #[doc = "0x5a0 - Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    pub hw_ocotp_srk2: HW_OCOTP_SRK2,
    _reserved34: [u8; 0x0c],
    #[doc = "0x5b0 - Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    pub hw_ocotp_srk3: HW_OCOTP_SRK3,
    _reserved35: [u8; 0x0c],
    #[doc = "0x5c0 - Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    pub hw_ocotp_srk4: HW_OCOTP_SRK4,
    _reserved36: [u8; 0x0c],
    #[doc = "0x5d0 - Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    pub hw_ocotp_srk5: HW_OCOTP_SRK5,
    _reserved37: [u8; 0x0c],
    #[doc = "0x5e0 - Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    pub hw_ocotp_srk6: HW_OCOTP_SRK6,
    _reserved38: [u8; 0x0c],
    #[doc = "0x5f0 - Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    pub hw_ocotp_srk7: HW_OCOTP_SRK7,
    _reserved39: [u8; 0x0c],
    #[doc = "0x600 - Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    pub hw_ocotp_sjc_resp0: HW_OCOTP_SJC_RESP0,
    _reserved40: [u8; 0x0c],
    #[doc = "0x610 - Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    pub hw_ocotp_sjc_resp1: HW_OCOTP_SJC_RESP1,
    _reserved41: [u8; 0x0c],
    #[doc = "0x620 - Value of OTP Bank4 Word2 (MAC Address)"]
    pub hw_ocotp_mac0: HW_OCOTP_MAC0,
    _reserved42: [u8; 0x0c],
    #[doc = "0x630 - Value of OTP Bank4 Word3 (MAC Address)"]
    pub hw_ocotp_mac1: HW_OCOTP_MAC1,
    _reserved43: [u8; 0x0c],
    #[doc = "0x640 - Value of OTP Bank4 Word4 (MAC Address)"]
    pub hw_ocotp_gp3: HW_OCOTP_GP3,
    _reserved44: [u8; 0x1c],
    #[doc = "0x660 - Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    pub hw_ocotp_gp1: HW_OCOTP_GP1,
    _reserved45: [u8; 0x0c],
    #[doc = "0x670 - Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    pub hw_ocotp_gp2: HW_OCOTP_GP2,
    _reserved46: [u8; 0x0c],
    #[doc = "0x680 - Value of OTP Bank5 Word0 (SW GP1)"]
    pub hw_ocotp_sw_gp1: HW_OCOTP_SW_GP1,
    _reserved47: [u8; 0x0c],
    #[doc = "0x690 - Value of OTP Bank5 Word1 (SW GP2)"]
    pub hw_ocotp_sw_gp20: HW_OCOTP_SW_GP20,
    _reserved48: [u8; 0x0c],
    #[doc = "0x6a0 - Value of OTP Bank5 Word2 (SW GP2)"]
    pub hw_ocotp_sw_gp21: HW_OCOTP_SW_GP21,
    _reserved49: [u8; 0x0c],
    #[doc = "0x6b0 - Value of OTP Bank5 Word3 (SW GP2)"]
    pub hw_ocotp_sw_gp22: HW_OCOTP_SW_GP22,
    _reserved50: [u8; 0x0c],
    #[doc = "0x6c0 - Value of OTP Bank5 Word4 (SW GP2)"]
    pub hw_ocotp_sw_gp23: HW_OCOTP_SW_GP23,
    _reserved51: [u8; 0x0c],
    #[doc = "0x6d0 - Value of OTP Bank5 Word5 (Misc Conf)"]
    pub hw_ocotp_misc_conf0: HW_OCOTP_MISC_CONF0,
    _reserved52: [u8; 0x0c],
    #[doc = "0x6e0 - Value of OTP Bank5 Word6 (Misc Conf)"]
    pub hw_ocotp_misc_conf1: HW_OCOTP_MISC_CONF1,
    _reserved53: [u8; 0x0c],
    #[doc = "0x6f0 - Value of OTP Bank5 Word7 (SRK Revoke)"]
    pub hw_ocotp_srk_revoke: HW_OCOTP_SRK_REVOKE,
}
#[doc = "HW_OCOTP_CTRL (rw) register accessor: an alias for `Reg<HW_OCOTP_CTRL_SPEC>`"]
pub type HW_OCOTP_CTRL = crate::Reg<hw_ocotp_ctrl::HW_OCOTP_CTRL_SPEC>;
#[doc = "OTP Controller Control and Status Register"]
pub mod hw_ocotp_ctrl;
#[doc = "HW_OCOTP_CTRL_SET (rw) register accessor: an alias for `Reg<HW_OCOTP_CTRL_SET_SPEC>`"]
pub type HW_OCOTP_CTRL_SET = crate::Reg<hw_ocotp_ctrl_set::HW_OCOTP_CTRL_SET_SPEC>;
#[doc = "OTP Controller Control and Status Register"]
pub mod hw_ocotp_ctrl_set;
#[doc = "HW_OCOTP_CTRL_CLR (rw) register accessor: an alias for `Reg<HW_OCOTP_CTRL_CLR_SPEC>`"]
pub type HW_OCOTP_CTRL_CLR = crate::Reg<hw_ocotp_ctrl_clr::HW_OCOTP_CTRL_CLR_SPEC>;
#[doc = "OTP Controller Control and Status Register"]
pub mod hw_ocotp_ctrl_clr;
#[doc = "HW_OCOTP_CTRL_TOG (rw) register accessor: an alias for `Reg<HW_OCOTP_CTRL_TOG_SPEC>`"]
pub type HW_OCOTP_CTRL_TOG = crate::Reg<hw_ocotp_ctrl_tog::HW_OCOTP_CTRL_TOG_SPEC>;
#[doc = "OTP Controller Control and Status Register"]
pub mod hw_ocotp_ctrl_tog;
#[doc = "HW_OCOTP_TIMING (rw) register accessor: an alias for `Reg<HW_OCOTP_TIMING_SPEC>`"]
pub type HW_OCOTP_TIMING = crate::Reg<hw_ocotp_timing::HW_OCOTP_TIMING_SPEC>;
#[doc = "OTP Controller Timing Register"]
pub mod hw_ocotp_timing;
#[doc = "HW_OCOTP_DATA (rw) register accessor: an alias for `Reg<HW_OCOTP_DATA_SPEC>`"]
pub type HW_OCOTP_DATA = crate::Reg<hw_ocotp_data::HW_OCOTP_DATA_SPEC>;
#[doc = "OTP Controller Write Data Register"]
pub mod hw_ocotp_data;
#[doc = "HW_OCOTP_READ_CTRL (rw) register accessor: an alias for `Reg<HW_OCOTP_READ_CTRL_SPEC>`"]
pub type HW_OCOTP_READ_CTRL = crate::Reg<hw_ocotp_read_ctrl::HW_OCOTP_READ_CTRL_SPEC>;
#[doc = "OTP Controller Write Data Register"]
pub mod hw_ocotp_read_ctrl;
#[doc = "HW_OCOTP_READ_FUSE_DATA (rw) register accessor: an alias for `Reg<HW_OCOTP_READ_FUSE_DATA_SPEC>`"]
pub type HW_OCOTP_READ_FUSE_DATA =
    crate::Reg<hw_ocotp_read_fuse_data::HW_OCOTP_READ_FUSE_DATA_SPEC>;
#[doc = "OTP Controller Read Data Register"]
pub mod hw_ocotp_read_fuse_data;
#[doc = "HW_OCOTP_SW_STICKY (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_STICKY_SPEC>`"]
pub type HW_OCOTP_SW_STICKY = crate::Reg<hw_ocotp_sw_sticky::HW_OCOTP_SW_STICKY_SPEC>;
#[doc = "Sticky bit Register"]
pub mod hw_ocotp_sw_sticky;
#[doc = "HW_OCOTP_SCS (rw) register accessor: an alias for `Reg<HW_OCOTP_SCS_SPEC>`"]
pub type HW_OCOTP_SCS = crate::Reg<hw_ocotp_scs::HW_OCOTP_SCS_SPEC>;
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs;
#[doc = "HW_OCOTP_SCS_SET (rw) register accessor: an alias for `Reg<HW_OCOTP_SCS_SET_SPEC>`"]
pub type HW_OCOTP_SCS_SET = crate::Reg<hw_ocotp_scs_set::HW_OCOTP_SCS_SET_SPEC>;
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_set;
#[doc = "HW_OCOTP_SCS_CLR (rw) register accessor: an alias for `Reg<HW_OCOTP_SCS_CLR_SPEC>`"]
pub type HW_OCOTP_SCS_CLR = crate::Reg<hw_ocotp_scs_clr::HW_OCOTP_SCS_CLR_SPEC>;
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_clr;
#[doc = "HW_OCOTP_SCS_TOG (rw) register accessor: an alias for `Reg<HW_OCOTP_SCS_TOG_SPEC>`"]
pub type HW_OCOTP_SCS_TOG = crate::Reg<hw_ocotp_scs_tog::HW_OCOTP_SCS_TOG_SPEC>;
#[doc = "Software Controllable Signals Register"]
pub mod hw_ocotp_scs_tog;
#[doc = "HW_OCOTP_VERSION (r) register accessor: an alias for `Reg<HW_OCOTP_VERSION_SPEC>`"]
pub type HW_OCOTP_VERSION = crate::Reg<hw_ocotp_version::HW_OCOTP_VERSION_SPEC>;
#[doc = "OTP Controller Version Register"]
pub mod hw_ocotp_version;
#[doc = "HW_OCOTP_TIMING2 (rw) register accessor: an alias for `Reg<HW_OCOTP_TIMING2_SPEC>`"]
pub type HW_OCOTP_TIMING2 = crate::Reg<hw_ocotp_timing2::HW_OCOTP_TIMING2_SPEC>;
#[doc = "OTP Controller Timing Register 2"]
pub mod hw_ocotp_timing2;
#[doc = "HW_OCOTP_LOCK (rw) register accessor: an alias for `Reg<HW_OCOTP_LOCK_SPEC>`"]
pub type HW_OCOTP_LOCK = crate::Reg<hw_ocotp_lock::HW_OCOTP_LOCK_SPEC>;
#[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
pub mod hw_ocotp_lock;
#[doc = "HW_OCOTP_CFG0 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG0_SPEC>`"]
pub type HW_OCOTP_CFG0 = crate::Reg<hw_ocotp_cfg0::HW_OCOTP_CFG0_SPEC>;
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg0;
#[doc = "HW_OCOTP_CFG1 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG1_SPEC>`"]
pub type HW_OCOTP_CFG1 = crate::Reg<hw_ocotp_cfg1::HW_OCOTP_CFG1_SPEC>;
#[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg1;
#[doc = "HW_OCOTP_CFG2 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG2_SPEC>`"]
pub type HW_OCOTP_CFG2 = crate::Reg<hw_ocotp_cfg2::HW_OCOTP_CFG2_SPEC>;
#[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg2;
#[doc = "HW_OCOTP_CFG3 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG3_SPEC>`"]
pub type HW_OCOTP_CFG3 = crate::Reg<hw_ocotp_cfg3::HW_OCOTP_CFG3_SPEC>;
#[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg3;
#[doc = "HW_OCOTP_CFG4 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG4_SPEC>`"]
pub type HW_OCOTP_CFG4 = crate::Reg<hw_ocotp_cfg4::HW_OCOTP_CFG4_SPEC>;
#[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg4;
#[doc = "HW_OCOTP_CFG5 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG5_SPEC>`"]
pub type HW_OCOTP_CFG5 = crate::Reg<hw_ocotp_cfg5::HW_OCOTP_CFG5_SPEC>;
#[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg5;
#[doc = "HW_OCOTP_CFG6 (rw) register accessor: an alias for `Reg<HW_OCOTP_CFG6_SPEC>`"]
pub type HW_OCOTP_CFG6 = crate::Reg<hw_ocotp_cfg6::HW_OCOTP_CFG6_SPEC>;
#[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
pub mod hw_ocotp_cfg6;
#[doc = "HW_OCOTP_MEM0 (rw) register accessor: an alias for `Reg<HW_OCOTP_MEM0_SPEC>`"]
pub type HW_OCOTP_MEM0 = crate::Reg<hw_ocotp_mem0::HW_OCOTP_MEM0_SPEC>;
#[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
pub mod hw_ocotp_mem0;
#[doc = "HW_OCOTP_MEM1 (rw) register accessor: an alias for `Reg<HW_OCOTP_MEM1_SPEC>`"]
pub type HW_OCOTP_MEM1 = crate::Reg<hw_ocotp_mem1::HW_OCOTP_MEM1_SPEC>;
#[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
pub mod hw_ocotp_mem1;
#[doc = "HW_OCOTP_MEM2 (rw) register accessor: an alias for `Reg<HW_OCOTP_MEM2_SPEC>`"]
pub type HW_OCOTP_MEM2 = crate::Reg<hw_ocotp_mem2::HW_OCOTP_MEM2_SPEC>;
#[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
pub mod hw_ocotp_mem2;
#[doc = "HW_OCOTP_MEM3 (rw) register accessor: an alias for `Reg<HW_OCOTP_MEM3_SPEC>`"]
pub type HW_OCOTP_MEM3 = crate::Reg<hw_ocotp_mem3::HW_OCOTP_MEM3_SPEC>;
#[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
pub mod hw_ocotp_mem3;
#[doc = "HW_OCOTP_MEM4 (rw) register accessor: an alias for `Reg<HW_OCOTP_MEM4_SPEC>`"]
pub type HW_OCOTP_MEM4 = crate::Reg<hw_ocotp_mem4::HW_OCOTP_MEM4_SPEC>;
#[doc = "Value of OTP Bank 1 Word 4 (Memory Related Info.)"]
pub mod hw_ocotp_mem4;
#[doc = "HW_OCOTP_ANA0 (rw) register accessor: an alias for `Reg<HW_OCOTP_ANA0_SPEC>`"]
pub type HW_OCOTP_ANA0 = crate::Reg<hw_ocotp_ana0::HW_OCOTP_ANA0_SPEC>;
#[doc = "Value of OTP Bank 1 Word 5 (Analog Info.)"]
pub mod hw_ocotp_ana0;
#[doc = "HW_OCOTP_ANA1 (rw) register accessor: an alias for `Reg<HW_OCOTP_ANA1_SPEC>`"]
pub type HW_OCOTP_ANA1 = crate::Reg<hw_ocotp_ana1::HW_OCOTP_ANA1_SPEC>;
#[doc = "Value of OTP Bank 1 Word 6 (Analog Info.)"]
pub mod hw_ocotp_ana1;
#[doc = "HW_OCOTP_ANA2 (rw) register accessor: an alias for `Reg<HW_OCOTP_ANA2_SPEC>`"]
pub type HW_OCOTP_ANA2 = crate::Reg<hw_ocotp_ana2::HW_OCOTP_ANA2_SPEC>;
#[doc = "Value of OTP Bank 1 Word 7 (Analog Info.)"]
pub mod hw_ocotp_ana2;
#[doc = "HW_OCOTP_SRK0 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK0_SPEC>`"]
pub type HW_OCOTP_SRK0 = crate::Reg<hw_ocotp_srk0::HW_OCOTP_SRK0_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
pub mod hw_ocotp_srk0;
#[doc = "HW_OCOTP_SRK1 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK1_SPEC>`"]
pub type HW_OCOTP_SRK1 = crate::Reg<hw_ocotp_srk1::HW_OCOTP_SRK1_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
pub mod hw_ocotp_srk1;
#[doc = "HW_OCOTP_SRK2 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK2_SPEC>`"]
pub type HW_OCOTP_SRK2 = crate::Reg<hw_ocotp_srk2::HW_OCOTP_SRK2_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
pub mod hw_ocotp_srk2;
#[doc = "HW_OCOTP_SRK3 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK3_SPEC>`"]
pub type HW_OCOTP_SRK3 = crate::Reg<hw_ocotp_srk3::HW_OCOTP_SRK3_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
pub mod hw_ocotp_srk3;
#[doc = "HW_OCOTP_SRK4 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK4_SPEC>`"]
pub type HW_OCOTP_SRK4 = crate::Reg<hw_ocotp_srk4::HW_OCOTP_SRK4_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
pub mod hw_ocotp_srk4;
#[doc = "HW_OCOTP_SRK5 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK5_SPEC>`"]
pub type HW_OCOTP_SRK5 = crate::Reg<hw_ocotp_srk5::HW_OCOTP_SRK5_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
pub mod hw_ocotp_srk5;
#[doc = "HW_OCOTP_SRK6 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK6_SPEC>`"]
pub type HW_OCOTP_SRK6 = crate::Reg<hw_ocotp_srk6::HW_OCOTP_SRK6_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
pub mod hw_ocotp_srk6;
#[doc = "HW_OCOTP_SRK7 (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK7_SPEC>`"]
pub type HW_OCOTP_SRK7 = crate::Reg<hw_ocotp_srk7::HW_OCOTP_SRK7_SPEC>;
#[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
pub mod hw_ocotp_srk7;
#[doc = "HW_OCOTP_SJC_RESP0 (rw) register accessor: an alias for `Reg<HW_OCOTP_SJC_RESP0_SPEC>`"]
pub type HW_OCOTP_SJC_RESP0 = crate::Reg<hw_ocotp_sjc_resp0::HW_OCOTP_SJC_RESP0_SPEC>;
#[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
pub mod hw_ocotp_sjc_resp0;
#[doc = "HW_OCOTP_SJC_RESP1 (rw) register accessor: an alias for `Reg<HW_OCOTP_SJC_RESP1_SPEC>`"]
pub type HW_OCOTP_SJC_RESP1 = crate::Reg<hw_ocotp_sjc_resp1::HW_OCOTP_SJC_RESP1_SPEC>;
#[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
pub mod hw_ocotp_sjc_resp1;
#[doc = "HW_OCOTP_MAC0 (rw) register accessor: an alias for `Reg<HW_OCOTP_MAC0_SPEC>`"]
pub type HW_OCOTP_MAC0 = crate::Reg<hw_ocotp_mac0::HW_OCOTP_MAC0_SPEC>;
#[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
pub mod hw_ocotp_mac0;
#[doc = "HW_OCOTP_MAC1 (rw) register accessor: an alias for `Reg<HW_OCOTP_MAC1_SPEC>`"]
pub type HW_OCOTP_MAC1 = crate::Reg<hw_ocotp_mac1::HW_OCOTP_MAC1_SPEC>;
#[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
pub mod hw_ocotp_mac1;
#[doc = "HW_OCOTP_GP3 (rw) register accessor: an alias for `Reg<HW_OCOTP_GP3_SPEC>`"]
pub type HW_OCOTP_GP3 = crate::Reg<hw_ocotp_gp3::HW_OCOTP_GP3_SPEC>;
#[doc = "Value of OTP Bank4 Word4 (MAC Address)"]
pub mod hw_ocotp_gp3;
#[doc = "HW_OCOTP_GP1 (rw) register accessor: an alias for `Reg<HW_OCOTP_GP1_SPEC>`"]
pub type HW_OCOTP_GP1 = crate::Reg<hw_ocotp_gp1::HW_OCOTP_GP1_SPEC>;
#[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
pub mod hw_ocotp_gp1;
#[doc = "HW_OCOTP_GP2 (rw) register accessor: an alias for `Reg<HW_OCOTP_GP2_SPEC>`"]
pub type HW_OCOTP_GP2 = crate::Reg<hw_ocotp_gp2::HW_OCOTP_GP2_SPEC>;
#[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
pub mod hw_ocotp_gp2;
#[doc = "HW_OCOTP_SW_GP1 (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_GP1_SPEC>`"]
pub type HW_OCOTP_SW_GP1 = crate::Reg<hw_ocotp_sw_gp1::HW_OCOTP_SW_GP1_SPEC>;
#[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
pub mod hw_ocotp_sw_gp1;
#[doc = "HW_OCOTP_SW_GP20 (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_GP20_SPEC>`"]
pub type HW_OCOTP_SW_GP20 = crate::Reg<hw_ocotp_sw_gp20::HW_OCOTP_SW_GP20_SPEC>;
#[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
pub mod hw_ocotp_sw_gp20;
#[doc = "HW_OCOTP_SW_GP21 (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_GP21_SPEC>`"]
pub type HW_OCOTP_SW_GP21 = crate::Reg<hw_ocotp_sw_gp21::HW_OCOTP_SW_GP21_SPEC>;
#[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
pub mod hw_ocotp_sw_gp21;
#[doc = "HW_OCOTP_SW_GP22 (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_GP22_SPEC>`"]
pub type HW_OCOTP_SW_GP22 = crate::Reg<hw_ocotp_sw_gp22::HW_OCOTP_SW_GP22_SPEC>;
#[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
pub mod hw_ocotp_sw_gp22;
#[doc = "HW_OCOTP_SW_GP23 (rw) register accessor: an alias for `Reg<HW_OCOTP_SW_GP23_SPEC>`"]
pub type HW_OCOTP_SW_GP23 = crate::Reg<hw_ocotp_sw_gp23::HW_OCOTP_SW_GP23_SPEC>;
#[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
pub mod hw_ocotp_sw_gp23;
#[doc = "HW_OCOTP_MISC_CONF0 (rw) register accessor: an alias for `Reg<HW_OCOTP_MISC_CONF0_SPEC>`"]
pub type HW_OCOTP_MISC_CONF0 = crate::Reg<hw_ocotp_misc_conf0::HW_OCOTP_MISC_CONF0_SPEC>;
#[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
pub mod hw_ocotp_misc_conf0;
#[doc = "HW_OCOTP_MISC_CONF1 (rw) register accessor: an alias for `Reg<HW_OCOTP_MISC_CONF1_SPEC>`"]
pub type HW_OCOTP_MISC_CONF1 = crate::Reg<hw_ocotp_misc_conf1::HW_OCOTP_MISC_CONF1_SPEC>;
#[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
pub mod hw_ocotp_misc_conf1;
#[doc = "HW_OCOTP_SRK_REVOKE (rw) register accessor: an alias for `Reg<HW_OCOTP_SRK_REVOKE_SPEC>`"]
pub type HW_OCOTP_SRK_REVOKE = crate::Reg<hw_ocotp_srk_revoke::HW_OCOTP_SRK_REVOKE_SPEC>;
#[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
pub mod hw_ocotp_srk_revoke;
