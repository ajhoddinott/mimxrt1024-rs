#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SNVS_HP Lock Register"]
    pub hplr: HPLR,
    #[doc = "0x04 - SNVS_HP Command Register"]
    pub hpcomr: HPCOMR,
    #[doc = "0x08 - SNVS_HP Control Register"]
    pub hpcr: HPCR,
    #[doc = "0x0c - SNVS_HP Security Interrupt Control Register"]
    pub hpsicr: HPSICR,
    #[doc = "0x10 - SNVS_HP Security Violation Control Register"]
    pub hpsvcr: HPSVCR,
    #[doc = "0x14 - SNVS_HP Status Register"]
    pub hpsr: HPSR,
    #[doc = "0x18 - SNVS_HP Security Violation Status Register"]
    pub hpsvsr: HPSVSR,
    #[doc = "0x1c - SNVS_HP High Assurance Counter IV Register"]
    pub hphacivr: HPHACIVR,
    #[doc = "0x20 - SNVS_HP High Assurance Counter Register"]
    pub hphacr: HPHACR,
    #[doc = "0x24 - SNVS_HP Real Time Counter MSB Register"]
    pub hprtcmr: HPRTCMR,
    #[doc = "0x28 - SNVS_HP Real Time Counter LSB Register"]
    pub hprtclr: HPRTCLR,
    #[doc = "0x2c - SNVS_HP Time Alarm MSB Register"]
    pub hptamr: HPTAMR,
    #[doc = "0x30 - SNVS_HP Time Alarm LSB Register"]
    pub hptalr: HPTALR,
    #[doc = "0x34 - SNVS_LP Lock Register"]
    pub lplr: LPLR,
    #[doc = "0x38 - SNVS_LP Control Register"]
    pub lpcr: LPCR,
    #[doc = "0x3c - SNVS_LP Master Key Control Register"]
    pub lpmkcr: LPMKCR,
    #[doc = "0x40 - SNVS_LP Security Violation Control Register"]
    pub lpsvcr: LPSVCR,
    _reserved17: [u8; 0x04],
    #[doc = "0x48 - SNVS_LP Security Events Configuration Register"]
    pub lpsecr: LPSECR,
    #[doc = "0x4c - SNVS_LP Status Register"]
    pub lpsr: LPSR,
    #[doc = "0x50 - SNVS_LP Secure Real Time Counter MSB Register"]
    pub lpsrtcmr: LPSRTCMR,
    #[doc = "0x54 - SNVS_LP Secure Real Time Counter LSB Register"]
    pub lpsrtclr: LPSRTCLR,
    #[doc = "0x58 - SNVS_LP Time Alarm Register"]
    pub lptar: LPTAR,
    #[doc = "0x5c - SNVS_LP Secure Monotonic Counter MSB Register"]
    pub lpsmcmr: LPSMCMR,
    #[doc = "0x60 - SNVS_LP Secure Monotonic Counter LSB Register"]
    pub lpsmclr: LPSMCLR,
    #[doc = "0x64 - SNVS_LP Digital Low-Voltage Detector Register"]
    pub lplvdr: LPLVDR,
    #[doc = "0x68 - SNVS_LP General Purpose Register 0 (legacy alias)"]
    pub lpgpr0_legacy_alias: LPGPR0_LEGACY_ALIAS,
    #[doc = "0x6c..0x8c - SNVS_LP Zeroizable Master Key Register"]
    pub lpzmkr: [LPZMKR; 8],
    _reserved27: [u8; 0x04],
    #[doc = "0x90..0xa0 - SNVS_LP General Purpose Registers 0 .. 3"]
    pub lpgpr_alias: [LPGPR_ALIAS; 4],
    _reserved28: [u8; 0x60],
    #[doc = "0x100..0x120 - SNVS_LP General Purpose Registers 0 .. 7"]
    pub lpgpr: [LPGPR; 8],
    _reserved29: [u8; 0x0ad8],
    #[doc = "0xbf8 - SNVS_HP Version ID Register 1"]
    pub hpvidr1: HPVIDR1,
    #[doc = "0xbfc - SNVS_HP Version ID Register 2"]
    pub hpvidr2: HPVIDR2,
}
#[doc = "HPLR (rw) register accessor: an alias for `Reg<HPLR_SPEC>`"]
pub type HPLR = crate::Reg<hplr::HPLR_SPEC>;
#[doc = "SNVS_HP Lock Register"]
pub mod hplr;
#[doc = "HPCOMR (rw) register accessor: an alias for `Reg<HPCOMR_SPEC>`"]
pub type HPCOMR = crate::Reg<hpcomr::HPCOMR_SPEC>;
#[doc = "SNVS_HP Command Register"]
pub mod hpcomr;
#[doc = "HPCR (rw) register accessor: an alias for `Reg<HPCR_SPEC>`"]
pub type HPCR = crate::Reg<hpcr::HPCR_SPEC>;
#[doc = "SNVS_HP Control Register"]
pub mod hpcr;
#[doc = "HPSICR (rw) register accessor: an alias for `Reg<HPSICR_SPEC>`"]
pub type HPSICR = crate::Reg<hpsicr::HPSICR_SPEC>;
#[doc = "SNVS_HP Security Interrupt Control Register"]
pub mod hpsicr;
#[doc = "HPSVCR (rw) register accessor: an alias for `Reg<HPSVCR_SPEC>`"]
pub type HPSVCR = crate::Reg<hpsvcr::HPSVCR_SPEC>;
#[doc = "SNVS_HP Security Violation Control Register"]
pub mod hpsvcr;
#[doc = "HPSR (rw) register accessor: an alias for `Reg<HPSR_SPEC>`"]
pub type HPSR = crate::Reg<hpsr::HPSR_SPEC>;
#[doc = "SNVS_HP Status Register"]
pub mod hpsr;
#[doc = "HPSVSR (rw) register accessor: an alias for `Reg<HPSVSR_SPEC>`"]
pub type HPSVSR = crate::Reg<hpsvsr::HPSVSR_SPEC>;
#[doc = "SNVS_HP Security Violation Status Register"]
pub mod hpsvsr;
#[doc = "HPHACIVR (rw) register accessor: an alias for `Reg<HPHACIVR_SPEC>`"]
pub type HPHACIVR = crate::Reg<hphacivr::HPHACIVR_SPEC>;
#[doc = "SNVS_HP High Assurance Counter IV Register"]
pub mod hphacivr;
#[doc = "HPHACR (r) register accessor: an alias for `Reg<HPHACR_SPEC>`"]
pub type HPHACR = crate::Reg<hphacr::HPHACR_SPEC>;
#[doc = "SNVS_HP High Assurance Counter Register"]
pub mod hphacr;
#[doc = "HPRTCMR (rw) register accessor: an alias for `Reg<HPRTCMR_SPEC>`"]
pub type HPRTCMR = crate::Reg<hprtcmr::HPRTCMR_SPEC>;
#[doc = "SNVS_HP Real Time Counter MSB Register"]
pub mod hprtcmr;
#[doc = "HPRTCLR (rw) register accessor: an alias for `Reg<HPRTCLR_SPEC>`"]
pub type HPRTCLR = crate::Reg<hprtclr::HPRTCLR_SPEC>;
#[doc = "SNVS_HP Real Time Counter LSB Register"]
pub mod hprtclr;
#[doc = "HPTAMR (rw) register accessor: an alias for `Reg<HPTAMR_SPEC>`"]
pub type HPTAMR = crate::Reg<hptamr::HPTAMR_SPEC>;
#[doc = "SNVS_HP Time Alarm MSB Register"]
pub mod hptamr;
#[doc = "HPTALR (rw) register accessor: an alias for `Reg<HPTALR_SPEC>`"]
pub type HPTALR = crate::Reg<hptalr::HPTALR_SPEC>;
#[doc = "SNVS_HP Time Alarm LSB Register"]
pub mod hptalr;
#[doc = "LPLR (rw) register accessor: an alias for `Reg<LPLR_SPEC>`"]
pub type LPLR = crate::Reg<lplr::LPLR_SPEC>;
#[doc = "SNVS_LP Lock Register"]
pub mod lplr;
#[doc = "LPCR (rw) register accessor: an alias for `Reg<LPCR_SPEC>`"]
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
#[doc = "SNVS_LP Control Register"]
pub mod lpcr;
#[doc = "LPMKCR (rw) register accessor: an alias for `Reg<LPMKCR_SPEC>`"]
pub type LPMKCR = crate::Reg<lpmkcr::LPMKCR_SPEC>;
#[doc = "SNVS_LP Master Key Control Register"]
pub mod lpmkcr;
#[doc = "LPSVCR (rw) register accessor: an alias for `Reg<LPSVCR_SPEC>`"]
pub type LPSVCR = crate::Reg<lpsvcr::LPSVCR_SPEC>;
#[doc = "SNVS_LP Security Violation Control Register"]
pub mod lpsvcr;
#[doc = "LPSECR (rw) register accessor: an alias for `Reg<LPSECR_SPEC>`"]
pub type LPSECR = crate::Reg<lpsecr::LPSECR_SPEC>;
#[doc = "SNVS_LP Security Events Configuration Register"]
pub mod lpsecr;
#[doc = "LPSR (rw) register accessor: an alias for `Reg<LPSR_SPEC>`"]
pub type LPSR = crate::Reg<lpsr::LPSR_SPEC>;
#[doc = "SNVS_LP Status Register"]
pub mod lpsr;
#[doc = "LPSRTCMR (rw) register accessor: an alias for `Reg<LPSRTCMR_SPEC>`"]
pub type LPSRTCMR = crate::Reg<lpsrtcmr::LPSRTCMR_SPEC>;
#[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
pub mod lpsrtcmr;
#[doc = "LPSRTCLR (rw) register accessor: an alias for `Reg<LPSRTCLR_SPEC>`"]
pub type LPSRTCLR = crate::Reg<lpsrtclr::LPSRTCLR_SPEC>;
#[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
pub mod lpsrtclr;
#[doc = "LPTAR (rw) register accessor: an alias for `Reg<LPTAR_SPEC>`"]
pub type LPTAR = crate::Reg<lptar::LPTAR_SPEC>;
#[doc = "SNVS_LP Time Alarm Register"]
pub mod lptar;
#[doc = "LPSMCMR (rw) register accessor: an alias for `Reg<LPSMCMR_SPEC>`"]
pub type LPSMCMR = crate::Reg<lpsmcmr::LPSMCMR_SPEC>;
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
pub mod lpsmcmr;
#[doc = "LPSMCLR (rw) register accessor: an alias for `Reg<LPSMCLR_SPEC>`"]
pub type LPSMCLR = crate::Reg<lpsmclr::LPSMCLR_SPEC>;
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
pub mod lpsmclr;
#[doc = "LPLVDR (rw) register accessor: an alias for `Reg<LPLVDR_SPEC>`"]
pub type LPLVDR = crate::Reg<lplvdr::LPLVDR_SPEC>;
#[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
pub mod lplvdr;
#[doc = "LPGPR0_legacy_alias (rw) register accessor: an alias for `Reg<LPGPR0_LEGACY_ALIAS_SPEC>`"]
pub type LPGPR0_LEGACY_ALIAS = crate::Reg<lpgpr0_legacy_alias::LPGPR0_LEGACY_ALIAS_SPEC>;
#[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
pub mod lpgpr0_legacy_alias;
#[doc = "LPZMKR (rw) register accessor: an alias for `Reg<LPZMKR_SPEC>`"]
pub type LPZMKR = crate::Reg<lpzmkr::LPZMKR_SPEC>;
#[doc = "SNVS_LP Zeroizable Master Key Register"]
pub mod lpzmkr;
#[doc = "LPGPR_alias (rw) register accessor: an alias for `Reg<LPGPR_ALIAS_SPEC>`"]
pub type LPGPR_ALIAS = crate::Reg<lpgpr_alias::LPGPR_ALIAS_SPEC>;
#[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
pub mod lpgpr_alias;
#[doc = "LPGPR (rw) register accessor: an alias for `Reg<LPGPR_SPEC>`"]
pub type LPGPR = crate::Reg<lpgpr::LPGPR_SPEC>;
#[doc = "SNVS_LP General Purpose Registers 0 .. 7"]
pub mod lpgpr;
#[doc = "HPVIDR1 (r) register accessor: an alias for `Reg<HPVIDR1_SPEC>`"]
pub type HPVIDR1 = crate::Reg<hpvidr1::HPVIDR1_SPEC>;
#[doc = "SNVS_HP Version ID Register 1"]
pub mod hpvidr1;
#[doc = "HPVIDR2 (r) register accessor: an alias for `Reg<HPVIDR2_SPEC>`"]
pub type HPVIDR2 = crate::Reg<hpvidr2::HPVIDR2_SPEC>;
#[doc = "SNVS_HP Version ID Register 2"]
pub mod hpvidr2;
