#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x04 - IO MUX Control Register"]
    pub iocr: IOCR,
    #[doc = "0x08 - Bus (AXI) Master Control Register 0"]
    pub bmcr0: BMCR0,
    #[doc = "0x0c - Bus (AXI) Master Control Register 1"]
    pub bmcr1: BMCR1,
    #[doc = "0x10..0x34 - Base Register n"]
    pub br: [BR; 9],
    _reserved5: [u8; 0x04],
    #[doc = "0x38 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x3c - Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x40 - SDRAM Control Register 0"]
    pub sdramcr0: SDRAMCR0,
    #[doc = "0x44 - SDRAM Control Register 1"]
    pub sdramcr1: SDRAMCR1,
    #[doc = "0x48 - SDRAM Control Register 2"]
    pub sdramcr2: SDRAMCR2,
    #[doc = "0x4c - SDRAM Control Register 3"]
    pub sdramcr3: SDRAMCR3,
    #[doc = "0x50 - NAND Control Register 0"]
    pub nandcr0: NANDCR0,
    #[doc = "0x54 - NAND Control Register 1"]
    pub nandcr1: NANDCR1,
    #[doc = "0x58 - NAND Control Register 2"]
    pub nandcr2: NANDCR2,
    #[doc = "0x5c - NAND Control Register 3"]
    pub nandcr3: NANDCR3,
    #[doc = "0x60 - NOR Control Register 0"]
    pub norcr0: NORCR0,
    #[doc = "0x64 - NOR Control Register 1"]
    pub norcr1: NORCR1,
    #[doc = "0x68 - NOR Control Register 2"]
    pub norcr2: NORCR2,
    #[doc = "0x6c - NOR Control Register 3"]
    pub norcr3: NORCR3,
    #[doc = "0x70 - SRAM Control Register 0"]
    pub sramcr0: SRAMCR0,
    #[doc = "0x74 - SRAM Control Register 1"]
    pub sramcr1: SRAMCR1,
    #[doc = "0x78 - SRAM Control Register 2"]
    pub sramcr2: SRAMCR2,
    #[doc = "0x7c - SRAM Control Register 3"]
    pub sramcr3: SRAMCR3,
    #[doc = "0x80 - DBI-B Control Register 0"]
    pub dbicr0: DBICR0,
    #[doc = "0x84 - DBI-B Control Register 1"]
    pub dbicr1: DBICR1,
    _reserved25: [u8; 0x08],
    #[doc = "0x90 - IP Command Control Register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0x94 - IP Command Control Register 1"]
    pub ipcr1: IPCR1,
    #[doc = "0x98 - IP Command Control Register 2"]
    pub ipcr2: IPCR2,
    #[doc = "0x9c - IP Command Register"]
    pub ipcmd: IPCMD,
    #[doc = "0xa0 - TX DATA Register"]
    pub iptxdat: IPTXDAT,
    _reserved30: [u8; 0x0c],
    #[doc = "0xb0 - RX DATA Register"]
    pub iprxdat: IPRXDAT,
    _reserved31: [u8; 0x0c],
    #[doc = "0xc0 - Status Register 0"]
    pub sts0: STS0,
    #[doc = "0xc4 - Status Register 1"]
    pub sts1: STS1,
    #[doc = "0xc8 - Status Register 2"]
    pub sts2: STS2,
    #[doc = "0xcc - Status Register 3"]
    pub sts3: STS3,
    #[doc = "0xd0 - Status Register 4"]
    pub sts4: STS4,
    #[doc = "0xd4 - Status Register 5"]
    pub sts5: STS5,
    #[doc = "0xd8 - Status Register 6"]
    pub sts6: STS6,
    #[doc = "0xdc - Status Register 7"]
    pub sts7: STS7,
    #[doc = "0xe0 - Status Register 8"]
    pub sts8: STS8,
    #[doc = "0xe4 - Status Register 9"]
    pub sts9: STS9,
    #[doc = "0xe8 - Status Register 10"]
    pub sts10: STS10,
    #[doc = "0xec - Status Register 11"]
    pub sts11: STS11,
    #[doc = "0xf0 - Status Register 12"]
    pub sts12: STS12,
    #[doc = "0xf4 - Status Register 13"]
    pub sts13: STS13,
    #[doc = "0xf8 - Status Register 14"]
    pub sts14: STS14,
    #[doc = "0xfc - Status Register 15"]
    pub sts15: STS15,
}
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "IOCR (rw) register accessor: an alias for `Reg<IOCR_SPEC>`"]
pub type IOCR = crate::Reg<iocr::IOCR_SPEC>;
#[doc = "IO MUX Control Register"]
pub mod iocr;
#[doc = "BMCR0 (rw) register accessor: an alias for `Reg<BMCR0_SPEC>`"]
pub type BMCR0 = crate::Reg<bmcr0::BMCR0_SPEC>;
#[doc = "Bus (AXI) Master Control Register 0"]
pub mod bmcr0;
#[doc = "BMCR1 (rw) register accessor: an alias for `Reg<BMCR1_SPEC>`"]
pub type BMCR1 = crate::Reg<bmcr1::BMCR1_SPEC>;
#[doc = "Bus (AXI) Master Control Register 1"]
pub mod bmcr1;
#[doc = "BR (rw) register accessor: an alias for `Reg<BR_SPEC>`"]
pub type BR = crate::Reg<br::BR_SPEC>;
#[doc = "Base Register n"]
pub mod br;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "SDRAMCR0 (rw) register accessor: an alias for `Reg<SDRAMCR0_SPEC>`"]
pub type SDRAMCR0 = crate::Reg<sdramcr0::SDRAMCR0_SPEC>;
#[doc = "SDRAM Control Register 0"]
pub mod sdramcr0;
#[doc = "SDRAMCR1 (rw) register accessor: an alias for `Reg<SDRAMCR1_SPEC>`"]
pub type SDRAMCR1 = crate::Reg<sdramcr1::SDRAMCR1_SPEC>;
#[doc = "SDRAM Control Register 1"]
pub mod sdramcr1;
#[doc = "SDRAMCR2 (rw) register accessor: an alias for `Reg<SDRAMCR2_SPEC>`"]
pub type SDRAMCR2 = crate::Reg<sdramcr2::SDRAMCR2_SPEC>;
#[doc = "SDRAM Control Register 2"]
pub mod sdramcr2;
#[doc = "SDRAMCR3 (rw) register accessor: an alias for `Reg<SDRAMCR3_SPEC>`"]
pub type SDRAMCR3 = crate::Reg<sdramcr3::SDRAMCR3_SPEC>;
#[doc = "SDRAM Control Register 3"]
pub mod sdramcr3;
#[doc = "NANDCR0 (rw) register accessor: an alias for `Reg<NANDCR0_SPEC>`"]
pub type NANDCR0 = crate::Reg<nandcr0::NANDCR0_SPEC>;
#[doc = "NAND Control Register 0"]
pub mod nandcr0;
#[doc = "NANDCR1 (rw) register accessor: an alias for `Reg<NANDCR1_SPEC>`"]
pub type NANDCR1 = crate::Reg<nandcr1::NANDCR1_SPEC>;
#[doc = "NAND Control Register 1"]
pub mod nandcr1;
#[doc = "NANDCR2 (rw) register accessor: an alias for `Reg<NANDCR2_SPEC>`"]
pub type NANDCR2 = crate::Reg<nandcr2::NANDCR2_SPEC>;
#[doc = "NAND Control Register 2"]
pub mod nandcr2;
#[doc = "NANDCR3 (rw) register accessor: an alias for `Reg<NANDCR3_SPEC>`"]
pub type NANDCR3 = crate::Reg<nandcr3::NANDCR3_SPEC>;
#[doc = "NAND Control Register 3"]
pub mod nandcr3;
#[doc = "NORCR0 (rw) register accessor: an alias for `Reg<NORCR0_SPEC>`"]
pub type NORCR0 = crate::Reg<norcr0::NORCR0_SPEC>;
#[doc = "NOR Control Register 0"]
pub mod norcr0;
#[doc = "NORCR1 (rw) register accessor: an alias for `Reg<NORCR1_SPEC>`"]
pub type NORCR1 = crate::Reg<norcr1::NORCR1_SPEC>;
#[doc = "NOR Control Register 1"]
pub mod norcr1;
#[doc = "NORCR2 (rw) register accessor: an alias for `Reg<NORCR2_SPEC>`"]
pub type NORCR2 = crate::Reg<norcr2::NORCR2_SPEC>;
#[doc = "NOR Control Register 2"]
pub mod norcr2;
#[doc = "NORCR3 (rw) register accessor: an alias for `Reg<NORCR3_SPEC>`"]
pub type NORCR3 = crate::Reg<norcr3::NORCR3_SPEC>;
#[doc = "NOR Control Register 3"]
pub mod norcr3;
#[doc = "SRAMCR0 (rw) register accessor: an alias for `Reg<SRAMCR0_SPEC>`"]
pub type SRAMCR0 = crate::Reg<sramcr0::SRAMCR0_SPEC>;
#[doc = "SRAM Control Register 0"]
pub mod sramcr0;
#[doc = "SRAMCR1 (rw) register accessor: an alias for `Reg<SRAMCR1_SPEC>`"]
pub type SRAMCR1 = crate::Reg<sramcr1::SRAMCR1_SPEC>;
#[doc = "SRAM Control Register 1"]
pub mod sramcr1;
#[doc = "SRAMCR2 (rw) register accessor: an alias for `Reg<SRAMCR2_SPEC>`"]
pub type SRAMCR2 = crate::Reg<sramcr2::SRAMCR2_SPEC>;
#[doc = "SRAM Control Register 2"]
pub mod sramcr2;
#[doc = "SRAMCR3 (rw) register accessor: an alias for `Reg<SRAMCR3_SPEC>`"]
pub type SRAMCR3 = crate::Reg<sramcr3::SRAMCR3_SPEC>;
#[doc = "SRAM Control Register 3"]
pub mod sramcr3;
#[doc = "DBICR0 (rw) register accessor: an alias for `Reg<DBICR0_SPEC>`"]
pub type DBICR0 = crate::Reg<dbicr0::DBICR0_SPEC>;
#[doc = "DBI-B Control Register 0"]
pub mod dbicr0;
#[doc = "DBICR1 (rw) register accessor: an alias for `Reg<DBICR1_SPEC>`"]
pub type DBICR1 = crate::Reg<dbicr1::DBICR1_SPEC>;
#[doc = "DBI-B Control Register 1"]
pub mod dbicr1;
#[doc = "IPCR0 (rw) register accessor: an alias for `Reg<IPCR0_SPEC>`"]
pub type IPCR0 = crate::Reg<ipcr0::IPCR0_SPEC>;
#[doc = "IP Command Control Register 0"]
pub mod ipcr0;
#[doc = "IPCR1 (rw) register accessor: an alias for `Reg<IPCR1_SPEC>`"]
pub type IPCR1 = crate::Reg<ipcr1::IPCR1_SPEC>;
#[doc = "IP Command Control Register 1"]
pub mod ipcr1;
#[doc = "IPCR2 (rw) register accessor: an alias for `Reg<IPCR2_SPEC>`"]
pub type IPCR2 = crate::Reg<ipcr2::IPCR2_SPEC>;
#[doc = "IP Command Control Register 2"]
pub mod ipcr2;
#[doc = "IPCMD (rw) register accessor: an alias for `Reg<IPCMD_SPEC>`"]
pub type IPCMD = crate::Reg<ipcmd::IPCMD_SPEC>;
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IPTXDAT (rw) register accessor: an alias for `Reg<IPTXDAT_SPEC>`"]
pub type IPTXDAT = crate::Reg<iptxdat::IPTXDAT_SPEC>;
#[doc = "TX DATA Register"]
pub mod iptxdat;
#[doc = "IPRXDAT (r) register accessor: an alias for `Reg<IPRXDAT_SPEC>`"]
pub type IPRXDAT = crate::Reg<iprxdat::IPRXDAT_SPEC>;
#[doc = "RX DATA Register"]
pub mod iprxdat;
#[doc = "STS0 (r) register accessor: an alias for `Reg<STS0_SPEC>`"]
pub type STS0 = crate::Reg<sts0::STS0_SPEC>;
#[doc = "Status Register 0"]
pub mod sts0;
#[doc = "STS1 (r) register accessor: an alias for `Reg<STS1_SPEC>`"]
pub type STS1 = crate::Reg<sts1::STS1_SPEC>;
#[doc = "Status Register 1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: an alias for `Reg<STS2_SPEC>`"]
pub type STS2 = crate::Reg<sts2::STS2_SPEC>;
#[doc = "Status Register 2"]
pub mod sts2;
#[doc = "STS3 (r) register accessor: an alias for `Reg<STS3_SPEC>`"]
pub type STS3 = crate::Reg<sts3::STS3_SPEC>;
#[doc = "Status Register 3"]
pub mod sts3;
#[doc = "STS4 (r) register accessor: an alias for `Reg<STS4_SPEC>`"]
pub type STS4 = crate::Reg<sts4::STS4_SPEC>;
#[doc = "Status Register 4"]
pub mod sts4;
#[doc = "STS5 (r) register accessor: an alias for `Reg<STS5_SPEC>`"]
pub type STS5 = crate::Reg<sts5::STS5_SPEC>;
#[doc = "Status Register 5"]
pub mod sts5;
#[doc = "STS6 (r) register accessor: an alias for `Reg<STS6_SPEC>`"]
pub type STS6 = crate::Reg<sts6::STS6_SPEC>;
#[doc = "Status Register 6"]
pub mod sts6;
#[doc = "STS7 (r) register accessor: an alias for `Reg<STS7_SPEC>`"]
pub type STS7 = crate::Reg<sts7::STS7_SPEC>;
#[doc = "Status Register 7"]
pub mod sts7;
#[doc = "STS8 (r) register accessor: an alias for `Reg<STS8_SPEC>`"]
pub type STS8 = crate::Reg<sts8::STS8_SPEC>;
#[doc = "Status Register 8"]
pub mod sts8;
#[doc = "STS9 (r) register accessor: an alias for `Reg<STS9_SPEC>`"]
pub type STS9 = crate::Reg<sts9::STS9_SPEC>;
#[doc = "Status Register 9"]
pub mod sts9;
#[doc = "STS10 (r) register accessor: an alias for `Reg<STS10_SPEC>`"]
pub type STS10 = crate::Reg<sts10::STS10_SPEC>;
#[doc = "Status Register 10"]
pub mod sts10;
#[doc = "STS11 (r) register accessor: an alias for `Reg<STS11_SPEC>`"]
pub type STS11 = crate::Reg<sts11::STS11_SPEC>;
#[doc = "Status Register 11"]
pub mod sts11;
#[doc = "STS12 (r) register accessor: an alias for `Reg<STS12_SPEC>`"]
pub type STS12 = crate::Reg<sts12::STS12_SPEC>;
#[doc = "Status Register 12"]
pub mod sts12;
#[doc = "STS13 (r) register accessor: an alias for `Reg<STS13_SPEC>`"]
pub type STS13 = crate::Reg<sts13::STS13_SPEC>;
#[doc = "Status Register 13"]
pub mod sts13;
#[doc = "STS14 (r) register accessor: an alias for `Reg<STS14_SPEC>`"]
pub type STS14 = crate::Reg<sts14::STS14_SPEC>;
#[doc = "Status Register 14"]
pub mod sts14;
#[doc = "STS15 (r) register accessor: an alias for `Reg<STS15_SPEC>`"]
pub type STS15 = crate::Reg<sts15::STS15_SPEC>;
#[doc = "Status Register 15"]
pub mod sts15;
