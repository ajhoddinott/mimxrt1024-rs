#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: PWD,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: TX,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: RX,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: STATUS,
    _reserved17: [u8; 0x0c],
    #[doc = "0x50 - USB PHY Debug Register"]
    pub debug: DEBUG,
    #[doc = "0x54 - USB PHY Debug Register"]
    pub debug_set: DEBUG_SET,
    #[doc = "0x58 - USB PHY Debug Register"]
    pub debug_clr: DEBUG_CLR,
    #[doc = "0x5c - USB PHY Debug Register"]
    pub debug_tog: DEBUG_TOG,
    #[doc = "0x60 - UTMI Debug Status Register 0"]
    pub debug0_status: DEBUG0_STATUS,
    _reserved22: [u8; 0x0c],
    #[doc = "0x70 - UTMI Debug Status Register 1"]
    pub debug1: DEBUG1,
    #[doc = "0x74 - UTMI Debug Status Register 1"]
    pub debug1_set: DEBUG1_SET,
    #[doc = "0x78 - UTMI Debug Status Register 1"]
    pub debug1_clr: DEBUG1_CLR,
    #[doc = "0x7c - UTMI Debug Status Register 1"]
    pub debug1_tog: DEBUG1_TOG,
    #[doc = "0x80 - UTMI RTL Version"]
    pub version: VERSION,
}
#[doc = "PWD (rw) register accessor: an alias for `Reg<PWD_SPEC>`"]
pub type PWD = crate::Reg<pwd::PWD_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "PWD_SET (rw) register accessor: an alias for `Reg<PWD_SET_SPEC>`"]
pub type PWD_SET = crate::Reg<pwd_set::PWD_SET_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "PWD_CLR (rw) register accessor: an alias for `Reg<PWD_CLR_SPEC>`"]
pub type PWD_CLR = crate::Reg<pwd_clr::PWD_CLR_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "PWD_TOG (rw) register accessor: an alias for `Reg<PWD_TOG_SPEC>`"]
pub type PWD_TOG = crate::Reg<pwd_tog::PWD_TOG_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "TX (rw) register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "TX_SET (rw) register accessor: an alias for `Reg<TX_SET_SPEC>`"]
pub type TX_SET = crate::Reg<tx_set::TX_SET_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "TX_CLR (rw) register accessor: an alias for `Reg<TX_CLR_SPEC>`"]
pub type TX_CLR = crate::Reg<tx_clr::TX_CLR_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "TX_TOG (rw) register accessor: an alias for `Reg<TX_TOG_SPEC>`"]
pub type TX_TOG = crate::Reg<tx_tog::TX_TOG_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "RX (rw) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "RX_SET (rw) register accessor: an alias for `Reg<RX_SET_SPEC>`"]
pub type RX_SET = crate::Reg<rx_set::RX_SET_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "RX_CLR (rw) register accessor: an alias for `Reg<RX_CLR_SPEC>`"]
pub type RX_CLR = crate::Reg<rx_clr::RX_CLR_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "RX_TOG (rw) register accessor: an alias for `Reg<RX_TOG_SPEC>`"]
pub type RX_TOG = crate::Reg<rx_tog::RX_TOG_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "CTRL_SET (rw) register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "CTRL_CLR (rw) register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG (rw) register accessor: an alias for `Reg<CTRL_TOG_SPEC>`"]
pub type CTRL_TOG = crate::Reg<ctrl_tog::CTRL_TOG_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "DEBUG (rw) register accessor: an alias for `Reg<DEBUG_SPEC>`"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "USB PHY Debug Register"]
pub mod debug;
#[doc = "DEBUG_SET (rw) register accessor: an alias for `Reg<DEBUG_SET_SPEC>`"]
pub type DEBUG_SET = crate::Reg<debug_set::DEBUG_SET_SPEC>;
#[doc = "USB PHY Debug Register"]
pub mod debug_set;
#[doc = "DEBUG_CLR (rw) register accessor: an alias for `Reg<DEBUG_CLR_SPEC>`"]
pub type DEBUG_CLR = crate::Reg<debug_clr::DEBUG_CLR_SPEC>;
#[doc = "USB PHY Debug Register"]
pub mod debug_clr;
#[doc = "DEBUG_TOG (rw) register accessor: an alias for `Reg<DEBUG_TOG_SPEC>`"]
pub type DEBUG_TOG = crate::Reg<debug_tog::DEBUG_TOG_SPEC>;
#[doc = "USB PHY Debug Register"]
pub mod debug_tog;
#[doc = "DEBUG0_STATUS (r) register accessor: an alias for `Reg<DEBUG0_STATUS_SPEC>`"]
pub type DEBUG0_STATUS = crate::Reg<debug0_status::DEBUG0_STATUS_SPEC>;
#[doc = "UTMI Debug Status Register 0"]
pub mod debug0_status;
#[doc = "DEBUG1 (rw) register accessor: an alias for `Reg<DEBUG1_SPEC>`"]
pub type DEBUG1 = crate::Reg<debug1::DEBUG1_SPEC>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1;
#[doc = "DEBUG1_SET (rw) register accessor: an alias for `Reg<DEBUG1_SET_SPEC>`"]
pub type DEBUG1_SET = crate::Reg<debug1_set::DEBUG1_SET_SPEC>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_set;
#[doc = "DEBUG1_CLR (rw) register accessor: an alias for `Reg<DEBUG1_CLR_SPEC>`"]
pub type DEBUG1_CLR = crate::Reg<debug1_clr::DEBUG1_CLR_SPEC>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_clr;
#[doc = "DEBUG1_TOG (rw) register accessor: an alias for `Reg<DEBUG1_TOG_SPEC>`"]
pub type DEBUG1_TOG = crate::Reg<debug1_tog::DEBUG1_TOG_SPEC>;
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_tog;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "UTMI RTL Version"]
pub mod version;
