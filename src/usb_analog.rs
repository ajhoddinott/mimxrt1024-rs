#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01a0],
    #[doc = "0x1a0 - USB VBUS Detect Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0x1a4 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0x1a8 - USB VBUS Detect Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0x1ac - USB VBUS Detect Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0x1b0 - USB Charger Detect Register"]
    pub usb1_chrg_detect: USB1_CHRG_DETECT,
    #[doc = "0x1b4 - USB Charger Detect Register"]
    pub usb1_chrg_detect_set: USB1_CHRG_DETECT_SET,
    #[doc = "0x1b8 - USB Charger Detect Register"]
    pub usb1_chrg_detect_clr: USB1_CHRG_DETECT_CLR,
    #[doc = "0x1bc - USB Charger Detect Register"]
    pub usb1_chrg_detect_tog: USB1_CHRG_DETECT_TOG,
    #[doc = "0x1c0 - USB VBUS Detect Status Register"]
    pub usb1_vbus_detect_stat: USB1_VBUS_DETECT_STAT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x1d0 - USB Charger Detect Status Register"]
    pub usb1_chrg_detect_stat: USB1_CHRG_DETECT_STAT,
    _reserved10: [u8; 0x0c],
    #[doc = "0x1e0 - USB Loopback Test Register"]
    pub usb1_loopback: USB1_LOOPBACK,
    #[doc = "0x1e4 - USB Loopback Test Register"]
    pub usb1_loopback_set: USB1_LOOPBACK_SET,
    #[doc = "0x1e8 - USB Loopback Test Register"]
    pub usb1_loopback_clr: USB1_LOOPBACK_CLR,
    #[doc = "0x1ec - USB Loopback Test Register"]
    pub usb1_loopback_tog: USB1_LOOPBACK_TOG,
    #[doc = "0x1f0 - USB Misc Register"]
    pub usb1_misc: USB1_MISC,
    #[doc = "0x1f4 - USB Misc Register"]
    pub usb1_misc_set: USB1_MISC_SET,
    #[doc = "0x1f8 - USB Misc Register"]
    pub usb1_misc_clr: USB1_MISC_CLR,
    #[doc = "0x1fc - USB Misc Register"]
    pub usb1_misc_tog: USB1_MISC_TOG,
    _reserved18: [u8; 0x60],
    #[doc = "0x260 - Chip Silicon Version"]
    pub digprog: DIGPROG,
}
#[doc = "USB1_VBUS_DETECT (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SPEC>`"]
pub type USB1_VBUS_DETECT = crate::Reg<usb1_vbus_detect::USB1_VBUS_DETECT_SPEC>;
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect;
#[doc = "USB1_VBUS_DETECT_SET (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SET_SPEC>`"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<usb1_vbus_detect_set::USB1_VBUS_DETECT_SET_SPEC>;
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB1_VBUS_DETECT_CLR (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_CLR_SPEC>`"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<usb1_vbus_detect_clr::USB1_VBUS_DETECT_CLR_SPEC>;
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB1_VBUS_DETECT_TOG (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_TOG_SPEC>`"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<usb1_vbus_detect_tog::USB1_VBUS_DETECT_TOG_SPEC>;
#[doc = "USB VBUS Detect Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB1_CHRG_DETECT (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_SPEC>`"]
pub type USB1_CHRG_DETECT = crate::Reg<usb1_chrg_detect::USB1_CHRG_DETECT_SPEC>;
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect;
#[doc = "USB1_CHRG_DETECT_SET (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_SET_SPEC>`"]
pub type USB1_CHRG_DETECT_SET = crate::Reg<usb1_chrg_detect_set::USB1_CHRG_DETECT_SET_SPEC>;
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_set;
#[doc = "USB1_CHRG_DETECT_CLR (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_CLR_SPEC>`"]
pub type USB1_CHRG_DETECT_CLR = crate::Reg<usb1_chrg_detect_clr::USB1_CHRG_DETECT_CLR_SPEC>;
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_clr;
#[doc = "USB1_CHRG_DETECT_TOG (rw) register accessor: an alias for `Reg<USB1_CHRG_DETECT_TOG_SPEC>`"]
pub type USB1_CHRG_DETECT_TOG = crate::Reg<usb1_chrg_detect_tog::USB1_CHRG_DETECT_TOG_SPEC>;
#[doc = "USB Charger Detect Register"]
pub mod usb1_chrg_detect_tog;
#[doc = "USB1_VBUS_DETECT_STAT (r) register accessor: an alias for `Reg<USB1_VBUS_DETECT_STAT_SPEC>`"]
pub type USB1_VBUS_DETECT_STAT = crate::Reg<usb1_vbus_detect_stat::USB1_VBUS_DETECT_STAT_SPEC>;
#[doc = "USB VBUS Detect Status Register"]
pub mod usb1_vbus_detect_stat;
#[doc = "USB1_CHRG_DETECT_STAT (r) register accessor: an alias for `Reg<USB1_CHRG_DETECT_STAT_SPEC>`"]
pub type USB1_CHRG_DETECT_STAT = crate::Reg<usb1_chrg_detect_stat::USB1_CHRG_DETECT_STAT_SPEC>;
#[doc = "USB Charger Detect Status Register"]
pub mod usb1_chrg_detect_stat;
#[doc = "USB1_LOOPBACK (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_SPEC>`"]
pub type USB1_LOOPBACK = crate::Reg<usb1_loopback::USB1_LOOPBACK_SPEC>;
#[doc = "USB Loopback Test Register"]
pub mod usb1_loopback;
#[doc = "USB1_LOOPBACK_SET (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_SET_SPEC>`"]
pub type USB1_LOOPBACK_SET = crate::Reg<usb1_loopback_set::USB1_LOOPBACK_SET_SPEC>;
#[doc = "USB Loopback Test Register"]
pub mod usb1_loopback_set;
#[doc = "USB1_LOOPBACK_CLR (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_CLR_SPEC>`"]
pub type USB1_LOOPBACK_CLR = crate::Reg<usb1_loopback_clr::USB1_LOOPBACK_CLR_SPEC>;
#[doc = "USB Loopback Test Register"]
pub mod usb1_loopback_clr;
#[doc = "USB1_LOOPBACK_TOG (rw) register accessor: an alias for `Reg<USB1_LOOPBACK_TOG_SPEC>`"]
pub type USB1_LOOPBACK_TOG = crate::Reg<usb1_loopback_tog::USB1_LOOPBACK_TOG_SPEC>;
#[doc = "USB Loopback Test Register"]
pub mod usb1_loopback_tog;
#[doc = "USB1_MISC (rw) register accessor: an alias for `Reg<USB1_MISC_SPEC>`"]
pub type USB1_MISC = crate::Reg<usb1_misc::USB1_MISC_SPEC>;
#[doc = "USB Misc Register"]
pub mod usb1_misc;
#[doc = "USB1_MISC_SET (rw) register accessor: an alias for `Reg<USB1_MISC_SET_SPEC>`"]
pub type USB1_MISC_SET = crate::Reg<usb1_misc_set::USB1_MISC_SET_SPEC>;
#[doc = "USB Misc Register"]
pub mod usb1_misc_set;
#[doc = "USB1_MISC_CLR (rw) register accessor: an alias for `Reg<USB1_MISC_CLR_SPEC>`"]
pub type USB1_MISC_CLR = crate::Reg<usb1_misc_clr::USB1_MISC_CLR_SPEC>;
#[doc = "USB Misc Register"]
pub mod usb1_misc_clr;
#[doc = "USB1_MISC_TOG (rw) register accessor: an alias for `Reg<USB1_MISC_TOG_SPEC>`"]
pub type USB1_MISC_TOG = crate::Reg<usb1_misc_tog::USB1_MISC_TOG_SPEC>;
#[doc = "USB Misc Register"]
pub mod usb1_misc_tog;
#[doc = "DIGPROG (r) register accessor: an alias for `Reg<DIGPROG_SPEC>`"]
pub type DIGPROG = crate::Reg<digprog::DIGPROG_SPEC>;
#[doc = "Chip Silicon Version"]
pub mod digprog;
