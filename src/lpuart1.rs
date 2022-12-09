#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - LPUART Global Register"]
    pub global: GLOBAL,
    #[doc = "0x0c - LPUART Pin Configuration Register"]
    pub pincfg: PINCFG,
    #[doc = "0x10 - LPUART Baud Rate Register"]
    pub baud: BAUD,
    #[doc = "0x14 - LPUART Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - LPUART Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x1c - LPUART Data Register"]
    pub data: DATA,
    #[doc = "0x20 - LPUART Match Address Register"]
    pub match_: MATCH,
    #[doc = "0x24 - LPUART Modem IrDA Register"]
    pub modir: MODIR,
    #[doc = "0x28 - LPUART FIFO Register"]
    pub fifo: FIFO,
    #[doc = "0x2c - LPUART Watermark Register"]
    pub water: WATER,
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "GLOBAL (rw) register accessor: an alias for `Reg<GLOBAL_SPEC>`"]
pub type GLOBAL = crate::Reg<global::GLOBAL_SPEC>;
#[doc = "LPUART Global Register"]
pub mod global;
#[doc = "PINCFG (rw) register accessor: an alias for `Reg<PINCFG_SPEC>`"]
pub type PINCFG = crate::Reg<pincfg::PINCFG_SPEC>;
#[doc = "LPUART Pin Configuration Register"]
pub mod pincfg;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "MATCH (rw) register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "LPUART Match Address Register"]
pub mod match_;
#[doc = "MODIR (rw) register accessor: an alias for `Reg<MODIR_SPEC>`"]
pub type MODIR = crate::Reg<modir::MODIR_SPEC>;
#[doc = "LPUART Modem IrDA Register"]
pub mod modir;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "LPUART FIFO Register"]
pub mod fifo;
#[doc = "WATER (rw) register accessor: an alias for `Reg<WATER_SPEC>`"]
pub type WATER = crate::Reg<water::WATER_SPEC>;
#[doc = "LPUART Watermark Register"]
pub mod water;
