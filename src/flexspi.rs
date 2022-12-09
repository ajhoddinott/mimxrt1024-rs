#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Control Register 0"]
    pub mcr0: MCR0,
    #[doc = "0x04 - Module Control Register 1"]
    pub mcr1: MCR1,
    #[doc = "0x08 - Module Control Register 2"]
    pub mcr2: MCR2,
    #[doc = "0x0c - AHB Bus Control Register"]
    pub ahbcr: AHBCR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x14 - Interrupt Register"]
    pub intr: INTR,
    #[doc = "0x18 - LUT Key Register"]
    pub lutkey: LUTKEY,
    #[doc = "0x1c - LUT Control Register"]
    pub lutcr: LUTCR,
    #[doc = "0x20 - AHB RX Buffer 0 Control Register 0"]
    pub ahbrxbuf0cr0: AHBRXBUF0CR0,
    #[doc = "0x24 - AHB RX Buffer 1 Control Register 0"]
    pub ahbrxbuf1cr0: AHBRXBUF1CR0,
    #[doc = "0x28 - AHB RX Buffer 2 Control Register 0"]
    pub ahbrxbuf2cr0: AHBRXBUF2CR0,
    #[doc = "0x2c - AHB RX Buffer 3 Control Register 0"]
    pub ahbrxbuf3cr0: AHBRXBUF3CR0,
    _reserved12: [u8; 0x30],
    #[doc = "0x60 - Flash Control Register 0"]
    pub flsha1cr0: FLSHA1CR0,
    #[doc = "0x64 - Flash Control Register 0"]
    pub flsha2cr0: FLSHA2CR0,
    #[doc = "0x68 - Flash Control Register 0"]
    pub flshb1cr0: FLSHB1CR0,
    #[doc = "0x6c - Flash Control Register 0"]
    pub flshb2cr0: FLSHB2CR0,
    #[doc = "0x70..0x80 - Flash Control Register 1"]
    pub flshcr1: [FLSHCR1; 4],
    #[doc = "0x80..0x90 - Flash Control Register 2"]
    pub flshcr2: [FLSHCR2; 4],
    _reserved18: [u8; 0x04],
    #[doc = "0x94 - Flash Control Register 4"]
    pub flshcr4: FLSHCR4,
    _reserved19: [u8; 0x08],
    #[doc = "0xa0 - IP Control Register 0"]
    pub ipcr0: IPCR0,
    #[doc = "0xa4 - IP Control Register 1"]
    pub ipcr1: IPCR1,
    _reserved21: [u8; 0x08],
    #[doc = "0xb0 - IP Command Register"]
    pub ipcmd: IPCMD,
    _reserved22: [u8; 0x04],
    #[doc = "0xb8 - IP RX FIFO Control Register"]
    pub iprxfcr: IPRXFCR,
    #[doc = "0xbc - IP TX FIFO Control Register"]
    pub iptxfcr: IPTXFCR,
    #[doc = "0xc0..0xc8 - DLL Control Register 0"]
    pub dllcr: [DLLCR; 2],
    _reserved25: [u8; 0x18],
    #[doc = "0xe0 - Status Register 0"]
    pub sts0: STS0,
    #[doc = "0xe4 - Status Register 1"]
    pub sts1: STS1,
    #[doc = "0xe8 - Status Register 2"]
    pub sts2: STS2,
    #[doc = "0xec - AHB Suspend Status Register"]
    pub ahbspndsts: AHBSPNDSTS,
    #[doc = "0xf0 - IP RX FIFO Status Register"]
    pub iprxfsts: IPRXFSTS,
    #[doc = "0xf4 - IP TX FIFO Status Register"]
    pub iptxfsts: IPTXFSTS,
    _reserved31: [u8; 0x08],
    #[doc = "0x100..0x180 - IP RX FIFO Data Register x"]
    pub rfdr: [RFDR; 32],
    #[doc = "0x180..0x200 - IP TX FIFO Data Register x"]
    pub tfdr: [TFDR; 32],
    #[doc = "0x200..0x300 - LUT x"]
    pub lut: [LUT; 64],
}
impl RegisterBlock {
    #[doc = "0x70 - Flash Control Register 1"]
    #[inline(always)]
    pub fn flshcr1a1(&self) -> &FLSHCR1 {
        &self.flshcr1[0]
    }
    #[doc = "0x74 - Flash Control Register 1"]
    #[inline(always)]
    pub fn flshcr1a2(&self) -> &FLSHCR1 {
        &self.flshcr1[1]
    }
    #[doc = "0x78 - Flash Control Register 1"]
    #[inline(always)]
    pub fn flshcr1b1(&self) -> &FLSHCR1 {
        &self.flshcr1[2]
    }
    #[doc = "0x7c - Flash Control Register 1"]
    #[inline(always)]
    pub fn flshcr1b2(&self) -> &FLSHCR1 {
        &self.flshcr1[3]
    }
    #[doc = "0x80 - Flash Control Register 2"]
    #[inline(always)]
    pub fn flshcr2a1(&self) -> &FLSHCR2 {
        &self.flshcr2[0]
    }
    #[doc = "0x84 - Flash Control Register 2"]
    #[inline(always)]
    pub fn flshcr2a2(&self) -> &FLSHCR2 {
        &self.flshcr2[1]
    }
    #[doc = "0x88 - Flash Control Register 2"]
    #[inline(always)]
    pub fn flshcr2b1(&self) -> &FLSHCR2 {
        &self.flshcr2[2]
    }
    #[doc = "0x8c - Flash Control Register 2"]
    #[inline(always)]
    pub fn flshcr2b2(&self) -> &FLSHCR2 {
        &self.flshcr2[3]
    }
    #[doc = "0xc0 - DLL Control Register 0"]
    #[inline(always)]
    pub fn dllcra(&self) -> &DLLCR {
        &self.dllcr[0]
    }
    #[doc = "0xc4 - DLL Control Register 0"]
    #[inline(always)]
    pub fn dllcrb(&self) -> &DLLCR {
        &self.dllcr[1]
    }
}
#[doc = "MCR0 (rw) register accessor: an alias for `Reg<MCR0_SPEC>`"]
pub type MCR0 = crate::Reg<mcr0::MCR0_SPEC>;
#[doc = "Module Control Register 0"]
pub mod mcr0;
#[doc = "MCR1 (rw) register accessor: an alias for `Reg<MCR1_SPEC>`"]
pub type MCR1 = crate::Reg<mcr1::MCR1_SPEC>;
#[doc = "Module Control Register 1"]
pub mod mcr1;
#[doc = "MCR2 (rw) register accessor: an alias for `Reg<MCR2_SPEC>`"]
pub type MCR2 = crate::Reg<mcr2::MCR2_SPEC>;
#[doc = "Module Control Register 2"]
pub mod mcr2;
#[doc = "AHBCR (rw) register accessor: an alias for `Reg<AHBCR_SPEC>`"]
pub type AHBCR = crate::Reg<ahbcr::AHBCR_SPEC>;
#[doc = "AHB Bus Control Register"]
pub mod ahbcr;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Register"]
pub mod intr;
#[doc = "LUTKEY (rw) register accessor: an alias for `Reg<LUTKEY_SPEC>`"]
pub type LUTKEY = crate::Reg<lutkey::LUTKEY_SPEC>;
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LUTCR (rw) register accessor: an alias for `Reg<LUTCR_SPEC>`"]
pub type LUTCR = crate::Reg<lutcr::LUTCR_SPEC>;
#[doc = "LUT Control Register"]
pub mod lutcr;
#[doc = "AHBRXBUF0CR0 (rw) register accessor: an alias for `Reg<AHBRXBUF0CR0_SPEC>`"]
pub type AHBRXBUF0CR0 = crate::Reg<ahbrxbuf0cr0::AHBRXBUF0CR0_SPEC>;
#[doc = "AHB RX Buffer 0 Control Register 0"]
pub mod ahbrxbuf0cr0;
#[doc = "AHBRXBUF1CR0 (rw) register accessor: an alias for `Reg<AHBRXBUF1CR0_SPEC>`"]
pub type AHBRXBUF1CR0 = crate::Reg<ahbrxbuf1cr0::AHBRXBUF1CR0_SPEC>;
#[doc = "AHB RX Buffer 1 Control Register 0"]
pub mod ahbrxbuf1cr0;
#[doc = "AHBRXBUF2CR0 (rw) register accessor: an alias for `Reg<AHBRXBUF2CR0_SPEC>`"]
pub type AHBRXBUF2CR0 = crate::Reg<ahbrxbuf2cr0::AHBRXBUF2CR0_SPEC>;
#[doc = "AHB RX Buffer 2 Control Register 0"]
pub mod ahbrxbuf2cr0;
#[doc = "AHBRXBUF3CR0 (rw) register accessor: an alias for `Reg<AHBRXBUF3CR0_SPEC>`"]
pub type AHBRXBUF3CR0 = crate::Reg<ahbrxbuf3cr0::AHBRXBUF3CR0_SPEC>;
#[doc = "AHB RX Buffer 3 Control Register 0"]
pub mod ahbrxbuf3cr0;
#[doc = "FLSHA1CR0 (rw) register accessor: an alias for `Reg<FLSHA1CR0_SPEC>`"]
pub type FLSHA1CR0 = crate::Reg<flsha1cr0::FLSHA1CR0_SPEC>;
#[doc = "Flash Control Register 0"]
pub mod flsha1cr0;
#[doc = "FLSHA2CR0 (rw) register accessor: an alias for `Reg<FLSHA2CR0_SPEC>`"]
pub type FLSHA2CR0 = crate::Reg<flsha2cr0::FLSHA2CR0_SPEC>;
#[doc = "Flash Control Register 0"]
pub mod flsha2cr0;
#[doc = "FLSHB1CR0 (rw) register accessor: an alias for `Reg<FLSHB1CR0_SPEC>`"]
pub type FLSHB1CR0 = crate::Reg<flshb1cr0::FLSHB1CR0_SPEC>;
#[doc = "Flash Control Register 0"]
pub mod flshb1cr0;
#[doc = "FLSHB2CR0 (rw) register accessor: an alias for `Reg<FLSHB2CR0_SPEC>`"]
pub type FLSHB2CR0 = crate::Reg<flshb2cr0::FLSHB2CR0_SPEC>;
#[doc = "Flash Control Register 0"]
pub mod flshb2cr0;
#[doc = "FLSHCR1 (rw) register accessor: an alias for `Reg<FLSHCR1_SPEC>`"]
pub type FLSHCR1 = crate::Reg<flshcr1::FLSHCR1_SPEC>;
#[doc = "Flash Control Register 1"]
pub mod flshcr1;
#[doc = "FLSHCR2 (rw) register accessor: an alias for `Reg<FLSHCR2_SPEC>`"]
pub type FLSHCR2 = crate::Reg<flshcr2::FLSHCR2_SPEC>;
#[doc = "Flash Control Register 2"]
pub mod flshcr2;
#[doc = "FLSHCR4 (rw) register accessor: an alias for `Reg<FLSHCR4_SPEC>`"]
pub type FLSHCR4 = crate::Reg<flshcr4::FLSHCR4_SPEC>;
#[doc = "Flash Control Register 4"]
pub mod flshcr4;
#[doc = "IPCR0 (rw) register accessor: an alias for `Reg<IPCR0_SPEC>`"]
pub type IPCR0 = crate::Reg<ipcr0::IPCR0_SPEC>;
#[doc = "IP Control Register 0"]
pub mod ipcr0;
#[doc = "IPCR1 (rw) register accessor: an alias for `Reg<IPCR1_SPEC>`"]
pub type IPCR1 = crate::Reg<ipcr1::IPCR1_SPEC>;
#[doc = "IP Control Register 1"]
pub mod ipcr1;
#[doc = "IPCMD (rw) register accessor: an alias for `Reg<IPCMD_SPEC>`"]
pub type IPCMD = crate::Reg<ipcmd::IPCMD_SPEC>;
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IPRXFCR (rw) register accessor: an alias for `Reg<IPRXFCR_SPEC>`"]
pub type IPRXFCR = crate::Reg<iprxfcr::IPRXFCR_SPEC>;
#[doc = "IP RX FIFO Control Register"]
pub mod iprxfcr;
#[doc = "IPTXFCR (rw) register accessor: an alias for `Reg<IPTXFCR_SPEC>`"]
pub type IPTXFCR = crate::Reg<iptxfcr::IPTXFCR_SPEC>;
#[doc = "IP TX FIFO Control Register"]
pub mod iptxfcr;
#[doc = "DLLCR (rw) register accessor: an alias for `Reg<DLLCR_SPEC>`"]
pub type DLLCR = crate::Reg<dllcr::DLLCR_SPEC>;
#[doc = "DLL Control Register 0"]
pub mod dllcr;
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
#[doc = "AHBSPNDSTS (r) register accessor: an alias for `Reg<AHBSPNDSTS_SPEC>`"]
pub type AHBSPNDSTS = crate::Reg<ahbspndsts::AHBSPNDSTS_SPEC>;
#[doc = "AHB Suspend Status Register"]
pub mod ahbspndsts;
#[doc = "IPRXFSTS (r) register accessor: an alias for `Reg<IPRXFSTS_SPEC>`"]
pub type IPRXFSTS = crate::Reg<iprxfsts::IPRXFSTS_SPEC>;
#[doc = "IP RX FIFO Status Register"]
pub mod iprxfsts;
#[doc = "IPTXFSTS (r) register accessor: an alias for `Reg<IPTXFSTS_SPEC>`"]
pub type IPTXFSTS = crate::Reg<iptxfsts::IPTXFSTS_SPEC>;
#[doc = "IP TX FIFO Status Register"]
pub mod iptxfsts;
#[doc = "RFDR (r) register accessor: an alias for `Reg<RFDR_SPEC>`"]
pub type RFDR = crate::Reg<rfdr::RFDR_SPEC>;
#[doc = "IP RX FIFO Data Register x"]
pub mod rfdr;
#[doc = "TFDR (w) register accessor: an alias for `Reg<TFDR_SPEC>`"]
pub type TFDR = crate::Reg<tfdr::TFDR_SPEC>;
#[doc = "IP TX FIFO Data Register x"]
pub mod tfdr;
#[doc = "LUT (rw) register accessor: an alias for `Reg<LUT_SPEC>`"]
pub type LUT = crate::Reg<lut::LUT_SPEC>;
#[doc = "LUT x"]
pub mod lut;
