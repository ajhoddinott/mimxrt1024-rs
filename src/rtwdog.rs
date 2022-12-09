#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register"]
    pub cs: CS,
    #[doc = "0x04 - Watchdog Counter Register"]
    pub cnt: CNT,
    #[doc = "0x08 - Watchdog Timeout Value Register"]
    pub toval: TOVAL,
    #[doc = "0x0c - Watchdog Window Register"]
    pub win: WIN,
}
#[doc = "CS (rw) register accessor: an alias for `Reg<CS_SPEC>`"]
pub type CS = crate::Reg<cs::CS_SPEC>;
#[doc = "Watchdog Control and Status Register"]
pub mod cs;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Watchdog Counter Register"]
pub mod cnt;
#[doc = "TOVAL (rw) register accessor: an alias for `Reg<TOVAL_SPEC>`"]
pub type TOVAL = crate::Reg<toval::TOVAL_SPEC>;
#[doc = "Watchdog Timeout Value Register"]
pub mod toval;
#[doc = "WIN (rw) register accessor: an alias for `Reg<WIN_SPEC>`"]
pub type WIN = crate::Reg<win::WIN_SPEC>;
#[doc = "Watchdog Window Register"]
pub mod win;
