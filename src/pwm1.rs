#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x58 - no description available"]
    pub sm0: SM,
    _reserved1: [u8; 0x08],
    #[doc = "0x60..0xb8 - no description available"]
    pub sm1: SM,
    _reserved2: [u8; 0x08],
    #[doc = "0xc0..0x118 - no description available"]
    pub sm2: SM,
    _reserved3: [u8; 0x08],
    #[doc = "0x120..0x178 - no description available"]
    pub sm3: SM,
    _reserved4: [u8; 0x08],
    #[doc = "0x180 - Output Enable Register"]
    pub outen: OUTEN,
    #[doc = "0x182 - Mask Register"]
    pub mask: MASK,
    #[doc = "0x184 - Software Controlled Output Register"]
    pub swcout: SWCOUT,
    #[doc = "0x186 - PWM Source Select Register"]
    pub dtsrcsel: DTSRCSEL,
    #[doc = "0x188 - Master Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x18a - Master Control 2 Register"]
    pub mctrl2: MCTRL2,
    #[doc = "0x18c - Fault Control Register"]
    pub fctrl0: FCTRL0,
    #[doc = "0x18e - Fault Status Register"]
    pub fsts0: FSTS0,
    #[doc = "0x190 - Fault Filter Register"]
    pub ffilt0: FFILT0,
    #[doc = "0x192 - Fault Test Register"]
    pub ftst0: FTST0,
    #[doc = "0x194 - Fault Control 2 Register"]
    pub fctrl20: FCTRL20,
}
#[doc = "no description available"]
pub use self::sm::SM;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod sm;
#[doc = "OUTEN (rw) register accessor: an alias for `Reg<OUTEN_SPEC>`"]
pub type OUTEN = crate::Reg<outen::OUTEN_SPEC>;
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask Register"]
pub mod mask;
#[doc = "SWCOUT (rw) register accessor: an alias for `Reg<SWCOUT_SPEC>`"]
pub type SWCOUT = crate::Reg<swcout::SWCOUT_SPEC>;
#[doc = "Software Controlled Output Register"]
pub mod swcout;
#[doc = "DTSRCSEL (rw) register accessor: an alias for `Reg<DTSRCSEL_SPEC>`"]
pub type DTSRCSEL = crate::Reg<dtsrcsel::DTSRCSEL_SPEC>;
#[doc = "PWM Source Select Register"]
pub mod dtsrcsel;
#[doc = "MCTRL (rw) register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Master Control Register"]
pub mod mctrl;
#[doc = "MCTRL2 (rw) register accessor: an alias for `Reg<MCTRL2_SPEC>`"]
pub type MCTRL2 = crate::Reg<mctrl2::MCTRL2_SPEC>;
#[doc = "Master Control 2 Register"]
pub mod mctrl2;
#[doc = "FCTRL0 (rw) register accessor: an alias for `Reg<FCTRL0_SPEC>`"]
pub type FCTRL0 = crate::Reg<fctrl0::FCTRL0_SPEC>;
#[doc = "Fault Control Register"]
pub mod fctrl0;
#[doc = "FSTS0 (rw) register accessor: an alias for `Reg<FSTS0_SPEC>`"]
pub type FSTS0 = crate::Reg<fsts0::FSTS0_SPEC>;
#[doc = "Fault Status Register"]
pub mod fsts0;
#[doc = "FFILT0 (rw) register accessor: an alias for `Reg<FFILT0_SPEC>`"]
pub type FFILT0 = crate::Reg<ffilt0::FFILT0_SPEC>;
#[doc = "Fault Filter Register"]
pub mod ffilt0;
#[doc = "FTST0 (rw) register accessor: an alias for `Reg<FTST0_SPEC>`"]
pub type FTST0 = crate::Reg<ftst0::FTST0_SPEC>;
#[doc = "Fault Test Register"]
pub mod ftst0;
#[doc = "FCTRL20 (rw) register accessor: an alias for `Reg<FCTRL20_SPEC>`"]
pub type FCTRL20 = crate::Reg<fctrl20::FCTRL20_SPEC>;
#[doc = "Fault Control 2 Register"]
pub mod fctrl20;
