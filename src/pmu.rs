#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0110],
    #[doc = "0x110 - Regulator 1P1 Register"]
    pub reg_1p1: REG_1P1,
    #[doc = "0x114 - Regulator 1P1 Register"]
    pub reg_1p1_set: REG_1P1_SET,
    #[doc = "0x118 - Regulator 1P1 Register"]
    pub reg_1p1_clr: REG_1P1_CLR,
    #[doc = "0x11c - Regulator 1P1 Register"]
    pub reg_1p1_tog: REG_1P1_TOG,
    #[doc = "0x120 - Regulator 3P0 Register"]
    pub reg_3p0: REG_3P0,
    #[doc = "0x124 - Regulator 3P0 Register"]
    pub reg_3p0_set: REG_3P0_SET,
    #[doc = "0x128 - Regulator 3P0 Register"]
    pub reg_3p0_clr: REG_3P0_CLR,
    #[doc = "0x12c - Regulator 3P0 Register"]
    pub reg_3p0_tog: REG_3P0_TOG,
    #[doc = "0x130 - Regulator 2P5 Register"]
    pub reg_2p5: REG_2P5,
    #[doc = "0x134 - Regulator 2P5 Register"]
    pub reg_2p5_set: REG_2P5_SET,
    #[doc = "0x138 - Regulator 2P5 Register"]
    pub reg_2p5_clr: REG_2P5_CLR,
    #[doc = "0x13c - Regulator 2P5 Register"]
    pub reg_2p5_tog: REG_2P5_TOG,
    #[doc = "0x140 - Digital Regulator Core Register"]
    pub reg_core: REG_CORE,
    #[doc = "0x144 - Digital Regulator Core Register"]
    pub reg_core_set: REG_CORE_SET,
    #[doc = "0x148 - Digital Regulator Core Register"]
    pub reg_core_clr: REG_CORE_CLR,
    #[doc = "0x14c - Digital Regulator Core Register"]
    pub reg_core_tog: REG_CORE_TOG,
    #[doc = "0x150 - Miscellaneous Register 0"]
    pub misc0: MISC0,
    #[doc = "0x154 - Miscellaneous Register 0"]
    pub misc0_set: MISC0_SET,
    #[doc = "0x158 - Miscellaneous Register 0"]
    pub misc0_clr: MISC0_CLR,
    #[doc = "0x15c - Miscellaneous Register 0"]
    pub misc0_tog: MISC0_TOG,
    #[doc = "0x160 - Miscellaneous Register 1"]
    pub misc1: MISC1,
    #[doc = "0x164 - Miscellaneous Register 1"]
    pub misc1_set: MISC1_SET,
    #[doc = "0x168 - Miscellaneous Register 1"]
    pub misc1_clr: MISC1_CLR,
    #[doc = "0x16c - Miscellaneous Register 1"]
    pub misc1_tog: MISC1_TOG,
    #[doc = "0x170 - Miscellaneous Control Register"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Control Register"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Control Register"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Control Register"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "REG_1P1 (rw) register accessor: an alias for `Reg<REG_1P1_SPEC>`"]
pub type REG_1P1 = crate::Reg<reg_1p1::REG_1P1_SPEC>;
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1;
#[doc = "REG_1P1_SET (rw) register accessor: an alias for `Reg<REG_1P1_SET_SPEC>`"]
pub type REG_1P1_SET = crate::Reg<reg_1p1_set::REG_1P1_SET_SPEC>;
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_set;
#[doc = "REG_1P1_CLR (rw) register accessor: an alias for `Reg<REG_1P1_CLR_SPEC>`"]
pub type REG_1P1_CLR = crate::Reg<reg_1p1_clr::REG_1P1_CLR_SPEC>;
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_clr;
#[doc = "REG_1P1_TOG (rw) register accessor: an alias for `Reg<REG_1P1_TOG_SPEC>`"]
pub type REG_1P1_TOG = crate::Reg<reg_1p1_tog::REG_1P1_TOG_SPEC>;
#[doc = "Regulator 1P1 Register"]
pub mod reg_1p1_tog;
#[doc = "REG_3P0 (rw) register accessor: an alias for `Reg<REG_3P0_SPEC>`"]
pub type REG_3P0 = crate::Reg<reg_3p0::REG_3P0_SPEC>;
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0;
#[doc = "REG_3P0_SET (rw) register accessor: an alias for `Reg<REG_3P0_SET_SPEC>`"]
pub type REG_3P0_SET = crate::Reg<reg_3p0_set::REG_3P0_SET_SPEC>;
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_set;
#[doc = "REG_3P0_CLR (rw) register accessor: an alias for `Reg<REG_3P0_CLR_SPEC>`"]
pub type REG_3P0_CLR = crate::Reg<reg_3p0_clr::REG_3P0_CLR_SPEC>;
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_clr;
#[doc = "REG_3P0_TOG (rw) register accessor: an alias for `Reg<REG_3P0_TOG_SPEC>`"]
pub type REG_3P0_TOG = crate::Reg<reg_3p0_tog::REG_3P0_TOG_SPEC>;
#[doc = "Regulator 3P0 Register"]
pub mod reg_3p0_tog;
#[doc = "REG_2P5 (rw) register accessor: an alias for `Reg<REG_2P5_SPEC>`"]
pub type REG_2P5 = crate::Reg<reg_2p5::REG_2P5_SPEC>;
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5;
#[doc = "REG_2P5_SET (rw) register accessor: an alias for `Reg<REG_2P5_SET_SPEC>`"]
pub type REG_2P5_SET = crate::Reg<reg_2p5_set::REG_2P5_SET_SPEC>;
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_set;
#[doc = "REG_2P5_CLR (rw) register accessor: an alias for `Reg<REG_2P5_CLR_SPEC>`"]
pub type REG_2P5_CLR = crate::Reg<reg_2p5_clr::REG_2P5_CLR_SPEC>;
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_clr;
#[doc = "REG_2P5_TOG (rw) register accessor: an alias for `Reg<REG_2P5_TOG_SPEC>`"]
pub type REG_2P5_TOG = crate::Reg<reg_2p5_tog::REG_2P5_TOG_SPEC>;
#[doc = "Regulator 2P5 Register"]
pub mod reg_2p5_tog;
#[doc = "REG_CORE (rw) register accessor: an alias for `Reg<REG_CORE_SPEC>`"]
pub type REG_CORE = crate::Reg<reg_core::REG_CORE_SPEC>;
#[doc = "Digital Regulator Core Register"]
pub mod reg_core;
#[doc = "REG_CORE_SET (rw) register accessor: an alias for `Reg<REG_CORE_SET_SPEC>`"]
pub type REG_CORE_SET = crate::Reg<reg_core_set::REG_CORE_SET_SPEC>;
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_set;
#[doc = "REG_CORE_CLR (rw) register accessor: an alias for `Reg<REG_CORE_CLR_SPEC>`"]
pub type REG_CORE_CLR = crate::Reg<reg_core_clr::REG_CORE_CLR_SPEC>;
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_clr;
#[doc = "REG_CORE_TOG (rw) register accessor: an alias for `Reg<REG_CORE_TOG_SPEC>`"]
pub type REG_CORE_TOG = crate::Reg<reg_core_tog::REG_CORE_TOG_SPEC>;
#[doc = "Digital Regulator Core Register"]
pub mod reg_core_tog;
#[doc = "MISC0 (rw) register accessor: an alias for `Reg<MISC0_SPEC>`"]
pub type MISC0 = crate::Reg<misc0::MISC0_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0;
#[doc = "MISC0_SET (rw) register accessor: an alias for `Reg<MISC0_SET_SPEC>`"]
pub type MISC0_SET = crate::Reg<misc0_set::MISC0_SET_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_set;
#[doc = "MISC0_CLR (rw) register accessor: an alias for `Reg<MISC0_CLR_SPEC>`"]
pub type MISC0_CLR = crate::Reg<misc0_clr::MISC0_CLR_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_clr;
#[doc = "MISC0_TOG (rw) register accessor: an alias for `Reg<MISC0_TOG_SPEC>`"]
pub type MISC0_TOG = crate::Reg<misc0_tog::MISC0_TOG_SPEC>;
#[doc = "Miscellaneous Register 0"]
pub mod misc0_tog;
#[doc = "MISC1 (rw) register accessor: an alias for `Reg<MISC1_SPEC>`"]
pub type MISC1 = crate::Reg<misc1::MISC1_SPEC>;
#[doc = "Miscellaneous Register 1"]
pub mod misc1;
#[doc = "MISC1_SET (rw) register accessor: an alias for `Reg<MISC1_SET_SPEC>`"]
pub type MISC1_SET = crate::Reg<misc1_set::MISC1_SET_SPEC>;
#[doc = "Miscellaneous Register 1"]
pub mod misc1_set;
#[doc = "MISC1_CLR (rw) register accessor: an alias for `Reg<MISC1_CLR_SPEC>`"]
pub type MISC1_CLR = crate::Reg<misc1_clr::MISC1_CLR_SPEC>;
#[doc = "Miscellaneous Register 1"]
pub mod misc1_clr;
#[doc = "MISC1_TOG (rw) register accessor: an alias for `Reg<MISC1_TOG_SPEC>`"]
pub type MISC1_TOG = crate::Reg<misc1_tog::MISC1_TOG_SPEC>;
#[doc = "Miscellaneous Register 1"]
pub mod misc1_tog;
#[doc = "MISC2 (rw) register accessor: an alias for `Reg<MISC2_SPEC>`"]
pub type MISC2 = crate::Reg<misc2::MISC2_SPEC>;
#[doc = "Miscellaneous Control Register"]
pub mod misc2;
#[doc = "MISC2_SET (rw) register accessor: an alias for `Reg<MISC2_SET_SPEC>`"]
pub type MISC2_SET = crate::Reg<misc2_set::MISC2_SET_SPEC>;
#[doc = "Miscellaneous Control Register"]
pub mod misc2_set;
#[doc = "MISC2_CLR (rw) register accessor: an alias for `Reg<MISC2_CLR_SPEC>`"]
pub type MISC2_CLR = crate::Reg<misc2_clr::MISC2_CLR_SPEC>;
#[doc = "Miscellaneous Control Register"]
pub mod misc2_clr;
#[doc = "MISC2_TOG (rw) register accessor: an alias for `Reg<MISC2_TOG_SPEC>`"]
pub type MISC2_TOG = crate::Reg<misc2_tog::MISC2_TOG_SPEC>;
#[doc = "Miscellaneous Control Register"]
pub mod misc2_tog;
