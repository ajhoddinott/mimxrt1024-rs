#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control Register"]
    pub wcr: WCR,
    #[doc = "0x02 - Watchdog Service Register"]
    pub wsr: WSR,
    #[doc = "0x04 - Watchdog Reset Status Register"]
    pub wrsr: WRSR,
    #[doc = "0x06 - Watchdog Interrupt Control Register"]
    pub wicr: WICR,
    #[doc = "0x08 - Watchdog Miscellaneous Control Register"]
    pub wmcr: WMCR,
}
#[doc = "WCR (rw) register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "Watchdog Control Register"]
pub mod wcr;
#[doc = "WSR (rw) register accessor: an alias for `Reg<WSR_SPEC>`"]
pub type WSR = crate::Reg<wsr::WSR_SPEC>;
#[doc = "Watchdog Service Register"]
pub mod wsr;
#[doc = "WRSR (r) register accessor: an alias for `Reg<WRSR_SPEC>`"]
pub type WRSR = crate::Reg<wrsr::WRSR_SPEC>;
#[doc = "Watchdog Reset Status Register"]
pub mod wrsr;
#[doc = "WICR (rw) register accessor: an alias for `Reg<WICR_SPEC>`"]
pub type WICR = crate::Reg<wicr::WICR_SPEC>;
#[doc = "Watchdog Interrupt Control Register"]
pub mod wicr;
#[doc = "WMCR (rw) register accessor: an alias for `Reg<WMCR_SPEC>`"]
pub type WMCR = crate::Reg<wmcr::WMCR_SPEC>;
#[doc = "Watchdog Miscellaneous Control Register"]
pub mod wmcr;
