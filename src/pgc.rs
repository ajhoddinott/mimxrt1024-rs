#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0220],
    #[doc = "0x220 - PGC Mega Control Register"]
    pub mega_ctrl: MEGA_CTRL,
    #[doc = "0x224 - PGC Mega Power Up Sequence Control Register"]
    pub mega_pupscr: MEGA_PUPSCR,
    #[doc = "0x228 - PGC Mega Pull Down Sequence Control Register"]
    pub mega_pdnscr: MEGA_PDNSCR,
    #[doc = "0x22c - PGC Mega Power Gating Controller Status Register"]
    pub mega_sr: MEGA_SR,
    _reserved4: [u8; 0x70],
    #[doc = "0x2a0 - PGC CPU Control Register"]
    pub cpu_ctrl: CPU_CTRL,
    #[doc = "0x2a4 - PGC CPU Power Up Sequence Control Register"]
    pub cpu_pupscr: CPU_PUPSCR,
    #[doc = "0x2a8 - PGC CPU Pull Down Sequence Control Register"]
    pub cpu_pdnscr: CPU_PDNSCR,
    #[doc = "0x2ac - PGC CPU Power Gating Controller Status Register"]
    pub cpu_sr: CPU_SR,
}
#[doc = "MEGA_CTRL (rw) register accessor: an alias for `Reg<MEGA_CTRL_SPEC>`"]
pub type MEGA_CTRL = crate::Reg<mega_ctrl::MEGA_CTRL_SPEC>;
#[doc = "PGC Mega Control Register"]
pub mod mega_ctrl;
#[doc = "MEGA_PUPSCR (rw) register accessor: an alias for `Reg<MEGA_PUPSCR_SPEC>`"]
pub type MEGA_PUPSCR = crate::Reg<mega_pupscr::MEGA_PUPSCR_SPEC>;
#[doc = "PGC Mega Power Up Sequence Control Register"]
pub mod mega_pupscr;
#[doc = "MEGA_PDNSCR (rw) register accessor: an alias for `Reg<MEGA_PDNSCR_SPEC>`"]
pub type MEGA_PDNSCR = crate::Reg<mega_pdnscr::MEGA_PDNSCR_SPEC>;
#[doc = "PGC Mega Pull Down Sequence Control Register"]
pub mod mega_pdnscr;
#[doc = "MEGA_SR (rw) register accessor: an alias for `Reg<MEGA_SR_SPEC>`"]
pub type MEGA_SR = crate::Reg<mega_sr::MEGA_SR_SPEC>;
#[doc = "PGC Mega Power Gating Controller Status Register"]
pub mod mega_sr;
#[doc = "CPU_CTRL (rw) register accessor: an alias for `Reg<CPU_CTRL_SPEC>`"]
pub type CPU_CTRL = crate::Reg<cpu_ctrl::CPU_CTRL_SPEC>;
#[doc = "PGC CPU Control Register"]
pub mod cpu_ctrl;
#[doc = "CPU_PUPSCR (rw) register accessor: an alias for `Reg<CPU_PUPSCR_SPEC>`"]
pub type CPU_PUPSCR = crate::Reg<cpu_pupscr::CPU_PUPSCR_SPEC>;
#[doc = "PGC CPU Power Up Sequence Control Register"]
pub mod cpu_pupscr;
#[doc = "CPU_PDNSCR (rw) register accessor: an alias for `Reg<CPU_PDNSCR_SPEC>`"]
pub type CPU_PDNSCR = crate::Reg<cpu_pdnscr::CPU_PDNSCR_SPEC>;
#[doc = "PGC CPU Pull Down Sequence Control Register"]
pub mod cpu_pdnscr;
#[doc = "CPU_SR (rw) register accessor: an alias for `Reg<CPU_SR_SPEC>`"]
pub type CPU_SR = crate::Reg<cpu_sr::CPU_SR_SPEC>;
#[doc = "PGC CPU Power Gating Controller Status Register"]
pub mod cpu_sr;
