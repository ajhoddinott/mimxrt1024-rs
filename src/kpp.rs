#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Keypad Control Register"]
    pub kpcr: KPCR,
    #[doc = "0x02 - Keypad Status Register"]
    pub kpsr: KPSR,
    #[doc = "0x04 - Keypad Data Direction Register"]
    pub kddr: KDDR,
    #[doc = "0x06 - Keypad Data Register"]
    pub kpdr: KPDR,
}
#[doc = "KPCR (rw) register accessor: an alias for `Reg<KPCR_SPEC>`"]
pub type KPCR = crate::Reg<kpcr::KPCR_SPEC>;
#[doc = "Keypad Control Register"]
pub mod kpcr;
#[doc = "KPSR (rw) register accessor: an alias for `Reg<KPSR_SPEC>`"]
pub type KPSR = crate::Reg<kpsr::KPSR_SPEC>;
#[doc = "Keypad Status Register"]
pub mod kpsr;
#[doc = "KDDR (rw) register accessor: an alias for `Reg<KDDR_SPEC>`"]
pub type KDDR = crate::Reg<kddr::KDDR_SPEC>;
#[doc = "Keypad Data Direction Register"]
pub mod kddr;
#[doc = "KPDR (rw) register accessor: an alias for `Reg<KPDR_SPEC>`"]
pub type KPDR = crate::Reg<kpdr::KPDR_SPEC>;
#[doc = "Keypad Data Register"]
pub mod kpdr;
