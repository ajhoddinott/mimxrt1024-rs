#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Config security level register"]
    pub csl: [CSL; 32],
    _reserved1: [u8; 0x0180],
    #[doc = "0x200 - HP0 register"]
    pub hp0: HP0,
    _reserved2: [u8; 0x14],
    #[doc = "0x218 - Secure access register"]
    pub sa: SA,
    _reserved3: [u8; 0x013c],
    #[doc = "0x358 - HPCONTROL0 register"]
    pub hpcontrol0: HPCONTROL0,
}
#[doc = "CSL (rw) register accessor: an alias for `Reg<CSL_SPEC>`"]
pub type CSL = crate::Reg<csl::CSL_SPEC>;
#[doc = "Config security level register"]
pub mod csl;
#[doc = "HP0 (rw) register accessor: an alias for `Reg<HP0_SPEC>`"]
pub type HP0 = crate::Reg<hp0::HP0_SPEC>;
#[doc = "HP0 register"]
pub mod hp0;
#[doc = "SA (rw) register accessor: an alias for `Reg<SA_SPEC>`"]
pub type SA = crate::Reg<sa::SA_SPEC>;
#[doc = "Secure access register"]
pub mod sa;
#[doc = "HPCONTROL0 (rw) register accessor: an alias for `Reg<HPCONTROL0_SPEC>`"]
pub type HPCONTROL0 = crate::Reg<hpcontrol0::HPCONTROL0_SPEC>;
#[doc = "HPCONTROL0 register"]
pub mod hpcontrol0;
