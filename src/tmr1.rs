#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Channel Compare Register 1"]
    pub comp10: COMP10,
    #[doc = "0x02 - Timer Channel Compare Register 2"]
    pub comp20: COMP20,
    #[doc = "0x04 - Timer Channel Capture Register"]
    pub capt0: CAPT0,
    #[doc = "0x06 - Timer Channel Load Register"]
    pub load0: LOAD0,
    #[doc = "0x08 - Timer Channel Hold Register"]
    pub hold0: HOLD0,
    #[doc = "0x0a - Timer Channel Counter Register"]
    pub cntr0: CNTR0,
    #[doc = "0x0c - Timer Channel Control Register"]
    pub ctrl0: CTRL0,
    #[doc = "0x0e - Timer Channel Status and Control Register"]
    pub sctrl0: SCTRL0,
    #[doc = "0x10 - Timer Channel Comparator Load Register 1"]
    pub cmpld10: CMPLD10,
    #[doc = "0x12 - Timer Channel Comparator Load Register 2"]
    pub cmpld20: CMPLD20,
    #[doc = "0x14 - Timer Channel Comparator Status and Control Register"]
    pub csctrl0: CSCTRL0,
    #[doc = "0x16 - Timer Channel Input Filter Register"]
    pub filt0: FILT0,
    #[doc = "0x18 - Timer Channel DMA Enable Register"]
    pub dma0: DMA0,
    _reserved13: [u8; 0x04],
    #[doc = "0x1e - Timer Channel Enable Register"]
    pub enbl: ENBL,
    #[doc = "0x20 - Timer Channel Compare Register 1"]
    pub comp11: COMP11,
    #[doc = "0x22 - Timer Channel Compare Register 2"]
    pub comp21: COMP21,
    #[doc = "0x24 - Timer Channel Capture Register"]
    pub capt1: CAPT1,
    #[doc = "0x26 - Timer Channel Load Register"]
    pub load1: LOAD1,
    #[doc = "0x28 - Timer Channel Hold Register"]
    pub hold1: HOLD1,
    #[doc = "0x2a - Timer Channel Counter Register"]
    pub cntr1: CNTR1,
    #[doc = "0x2c - Timer Channel Control Register"]
    pub ctrl1: CTRL1,
    #[doc = "0x2e - Timer Channel Status and Control Register"]
    pub sctrl1: SCTRL1,
    #[doc = "0x30 - Timer Channel Comparator Load Register 1"]
    pub cmpld11: CMPLD11,
    #[doc = "0x32 - Timer Channel Comparator Load Register 2"]
    pub cmpld21: CMPLD21,
    #[doc = "0x34 - Timer Channel Comparator Status and Control Register"]
    pub csctrl1: CSCTRL1,
    #[doc = "0x36 - Timer Channel Input Filter Register"]
    pub filt1: FILT1,
    #[doc = "0x38 - Timer Channel DMA Enable Register"]
    pub dma1: DMA1,
    _reserved27: [u8; 0x06],
    #[doc = "0x40 - Timer Channel Compare Register 1"]
    pub comp12: COMP12,
    #[doc = "0x42 - Timer Channel Compare Register 2"]
    pub comp22: COMP22,
    #[doc = "0x44 - Timer Channel Capture Register"]
    pub capt2: CAPT2,
    #[doc = "0x46 - Timer Channel Load Register"]
    pub load2: LOAD2,
    #[doc = "0x48 - Timer Channel Hold Register"]
    pub hold2: HOLD2,
    #[doc = "0x4a - Timer Channel Counter Register"]
    pub cntr2: CNTR2,
    #[doc = "0x4c - Timer Channel Control Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x4e - Timer Channel Status and Control Register"]
    pub sctrl2: SCTRL2,
    #[doc = "0x50 - Timer Channel Comparator Load Register 1"]
    pub cmpld12: CMPLD12,
    #[doc = "0x52 - Timer Channel Comparator Load Register 2"]
    pub cmpld22: CMPLD22,
    #[doc = "0x54 - Timer Channel Comparator Status and Control Register"]
    pub csctrl2: CSCTRL2,
    #[doc = "0x56 - Timer Channel Input Filter Register"]
    pub filt2: FILT2,
    #[doc = "0x58 - Timer Channel DMA Enable Register"]
    pub dma2: DMA2,
    _reserved40: [u8; 0x06],
    #[doc = "0x60 - Timer Channel Compare Register 1"]
    pub comp13: COMP13,
    #[doc = "0x62 - Timer Channel Compare Register 2"]
    pub comp23: COMP23,
    #[doc = "0x64 - Timer Channel Capture Register"]
    pub capt3: CAPT3,
    #[doc = "0x66 - Timer Channel Load Register"]
    pub load3: LOAD3,
    #[doc = "0x68 - Timer Channel Hold Register"]
    pub hold3: HOLD3,
    #[doc = "0x6a - Timer Channel Counter Register"]
    pub cntr3: CNTR3,
    #[doc = "0x6c - Timer Channel Control Register"]
    pub ctrl3: CTRL3,
    #[doc = "0x6e - Timer Channel Status and Control Register"]
    pub sctrl3: SCTRL3,
    #[doc = "0x70 - Timer Channel Comparator Load Register 1"]
    pub cmpld13: CMPLD13,
    #[doc = "0x72 - Timer Channel Comparator Load Register 2"]
    pub cmpld23: CMPLD23,
    #[doc = "0x74 - Timer Channel Comparator Status and Control Register"]
    pub csctrl3: CSCTRL3,
    #[doc = "0x76 - Timer Channel Input Filter Register"]
    pub filt3: FILT3,
    #[doc = "0x78 - Timer Channel DMA Enable Register"]
    pub dma3: DMA3,
}
#[doc = "COMP10 (rw) register accessor: an alias for `Reg<COMP10_SPEC>`"]
pub type COMP10 = crate::Reg<comp10::COMP10_SPEC>;
#[doc = "Timer Channel Compare Register 1"]
pub mod comp10;
#[doc = "COMP20 (rw) register accessor: an alias for `Reg<COMP20_SPEC>`"]
pub type COMP20 = crate::Reg<comp20::COMP20_SPEC>;
#[doc = "Timer Channel Compare Register 2"]
pub mod comp20;
#[doc = "CAPT0 (rw) register accessor: an alias for `Reg<CAPT0_SPEC>`"]
pub type CAPT0 = crate::Reg<capt0::CAPT0_SPEC>;
#[doc = "Timer Channel Capture Register"]
pub mod capt0;
#[doc = "LOAD0 (rw) register accessor: an alias for `Reg<LOAD0_SPEC>`"]
pub type LOAD0 = crate::Reg<load0::LOAD0_SPEC>;
#[doc = "Timer Channel Load Register"]
pub mod load0;
#[doc = "HOLD0 (rw) register accessor: an alias for `Reg<HOLD0_SPEC>`"]
pub type HOLD0 = crate::Reg<hold0::HOLD0_SPEC>;
#[doc = "Timer Channel Hold Register"]
pub mod hold0;
#[doc = "CNTR0 (rw) register accessor: an alias for `Reg<CNTR0_SPEC>`"]
pub type CNTR0 = crate::Reg<cntr0::CNTR0_SPEC>;
#[doc = "Timer Channel Counter Register"]
pub mod cntr0;
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ctrl0;
#[doc = "SCTRL0 (rw) register accessor: an alias for `Reg<SCTRL0_SPEC>`"]
pub type SCTRL0 = crate::Reg<sctrl0::SCTRL0_SPEC>;
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl0;
#[doc = "CMPLD10 (rw) register accessor: an alias for `Reg<CMPLD10_SPEC>`"]
pub type CMPLD10 = crate::Reg<cmpld10::CMPLD10_SPEC>;
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld10;
#[doc = "CMPLD20 (rw) register accessor: an alias for `Reg<CMPLD20_SPEC>`"]
pub type CMPLD20 = crate::Reg<cmpld20::CMPLD20_SPEC>;
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld20;
#[doc = "CSCTRL0 (rw) register accessor: an alias for `Reg<CSCTRL0_SPEC>`"]
pub type CSCTRL0 = crate::Reg<csctrl0::CSCTRL0_SPEC>;
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl0;
#[doc = "FILT0 (rw) register accessor: an alias for `Reg<FILT0_SPEC>`"]
pub type FILT0 = crate::Reg<filt0::FILT0_SPEC>;
#[doc = "Timer Channel Input Filter Register"]
pub mod filt0;
#[doc = "DMA0 (rw) register accessor: an alias for `Reg<DMA0_SPEC>`"]
pub type DMA0 = crate::Reg<dma0::DMA0_SPEC>;
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma0;
#[doc = "ENBL (rw) register accessor: an alias for `Reg<ENBL_SPEC>`"]
pub type ENBL = crate::Reg<enbl::ENBL_SPEC>;
#[doc = "Timer Channel Enable Register"]
pub mod enbl;
#[doc = "COMP11 (rw) register accessor: an alias for `Reg<COMP11_SPEC>`"]
pub type COMP11 = crate::Reg<comp11::COMP11_SPEC>;
#[doc = "Timer Channel Compare Register 1"]
pub mod comp11;
#[doc = "COMP21 (rw) register accessor: an alias for `Reg<COMP21_SPEC>`"]
pub type COMP21 = crate::Reg<comp21::COMP21_SPEC>;
#[doc = "Timer Channel Compare Register 2"]
pub mod comp21;
#[doc = "CAPT1 (rw) register accessor: an alias for `Reg<CAPT1_SPEC>`"]
pub type CAPT1 = crate::Reg<capt1::CAPT1_SPEC>;
#[doc = "Timer Channel Capture Register"]
pub mod capt1;
#[doc = "LOAD1 (rw) register accessor: an alias for `Reg<LOAD1_SPEC>`"]
pub type LOAD1 = crate::Reg<load1::LOAD1_SPEC>;
#[doc = "Timer Channel Load Register"]
pub mod load1;
#[doc = "HOLD1 (rw) register accessor: an alias for `Reg<HOLD1_SPEC>`"]
pub type HOLD1 = crate::Reg<hold1::HOLD1_SPEC>;
#[doc = "Timer Channel Hold Register"]
pub mod hold1;
#[doc = "CNTR1 (rw) register accessor: an alias for `Reg<CNTR1_SPEC>`"]
pub type CNTR1 = crate::Reg<cntr1::CNTR1_SPEC>;
#[doc = "Timer Channel Counter Register"]
pub mod cntr1;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ctrl1;
#[doc = "SCTRL1 (rw) register accessor: an alias for `Reg<SCTRL1_SPEC>`"]
pub type SCTRL1 = crate::Reg<sctrl1::SCTRL1_SPEC>;
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl1;
#[doc = "CMPLD11 (rw) register accessor: an alias for `Reg<CMPLD11_SPEC>`"]
pub type CMPLD11 = crate::Reg<cmpld11::CMPLD11_SPEC>;
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld11;
#[doc = "CMPLD21 (rw) register accessor: an alias for `Reg<CMPLD21_SPEC>`"]
pub type CMPLD21 = crate::Reg<cmpld21::CMPLD21_SPEC>;
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld21;
#[doc = "CSCTRL1 (rw) register accessor: an alias for `Reg<CSCTRL1_SPEC>`"]
pub type CSCTRL1 = crate::Reg<csctrl1::CSCTRL1_SPEC>;
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl1;
#[doc = "FILT1 (rw) register accessor: an alias for `Reg<FILT1_SPEC>`"]
pub type FILT1 = crate::Reg<filt1::FILT1_SPEC>;
#[doc = "Timer Channel Input Filter Register"]
pub mod filt1;
#[doc = "DMA1 (rw) register accessor: an alias for `Reg<DMA1_SPEC>`"]
pub type DMA1 = crate::Reg<dma1::DMA1_SPEC>;
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma1;
#[doc = "COMP12 (rw) register accessor: an alias for `Reg<COMP12_SPEC>`"]
pub type COMP12 = crate::Reg<comp12::COMP12_SPEC>;
#[doc = "Timer Channel Compare Register 1"]
pub mod comp12;
#[doc = "COMP22 (rw) register accessor: an alias for `Reg<COMP22_SPEC>`"]
pub type COMP22 = crate::Reg<comp22::COMP22_SPEC>;
#[doc = "Timer Channel Compare Register 2"]
pub mod comp22;
#[doc = "CAPT2 (rw) register accessor: an alias for `Reg<CAPT2_SPEC>`"]
pub type CAPT2 = crate::Reg<capt2::CAPT2_SPEC>;
#[doc = "Timer Channel Capture Register"]
pub mod capt2;
#[doc = "LOAD2 (rw) register accessor: an alias for `Reg<LOAD2_SPEC>`"]
pub type LOAD2 = crate::Reg<load2::LOAD2_SPEC>;
#[doc = "Timer Channel Load Register"]
pub mod load2;
#[doc = "HOLD2 (rw) register accessor: an alias for `Reg<HOLD2_SPEC>`"]
pub type HOLD2 = crate::Reg<hold2::HOLD2_SPEC>;
#[doc = "Timer Channel Hold Register"]
pub mod hold2;
#[doc = "CNTR2 (rw) register accessor: an alias for `Reg<CNTR2_SPEC>`"]
pub type CNTR2 = crate::Reg<cntr2::CNTR2_SPEC>;
#[doc = "Timer Channel Counter Register"]
pub mod cntr2;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ctrl2;
#[doc = "SCTRL2 (rw) register accessor: an alias for `Reg<SCTRL2_SPEC>`"]
pub type SCTRL2 = crate::Reg<sctrl2::SCTRL2_SPEC>;
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl2;
#[doc = "CMPLD12 (rw) register accessor: an alias for `Reg<CMPLD12_SPEC>`"]
pub type CMPLD12 = crate::Reg<cmpld12::CMPLD12_SPEC>;
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld12;
#[doc = "CMPLD22 (rw) register accessor: an alias for `Reg<CMPLD22_SPEC>`"]
pub type CMPLD22 = crate::Reg<cmpld22::CMPLD22_SPEC>;
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld22;
#[doc = "CSCTRL2 (rw) register accessor: an alias for `Reg<CSCTRL2_SPEC>`"]
pub type CSCTRL2 = crate::Reg<csctrl2::CSCTRL2_SPEC>;
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl2;
#[doc = "FILT2 (rw) register accessor: an alias for `Reg<FILT2_SPEC>`"]
pub type FILT2 = crate::Reg<filt2::FILT2_SPEC>;
#[doc = "Timer Channel Input Filter Register"]
pub mod filt2;
#[doc = "DMA2 (rw) register accessor: an alias for `Reg<DMA2_SPEC>`"]
pub type DMA2 = crate::Reg<dma2::DMA2_SPEC>;
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma2;
#[doc = "COMP13 (rw) register accessor: an alias for `Reg<COMP13_SPEC>`"]
pub type COMP13 = crate::Reg<comp13::COMP13_SPEC>;
#[doc = "Timer Channel Compare Register 1"]
pub mod comp13;
#[doc = "COMP23 (rw) register accessor: an alias for `Reg<COMP23_SPEC>`"]
pub type COMP23 = crate::Reg<comp23::COMP23_SPEC>;
#[doc = "Timer Channel Compare Register 2"]
pub mod comp23;
#[doc = "CAPT3 (rw) register accessor: an alias for `Reg<CAPT3_SPEC>`"]
pub type CAPT3 = crate::Reg<capt3::CAPT3_SPEC>;
#[doc = "Timer Channel Capture Register"]
pub mod capt3;
#[doc = "LOAD3 (rw) register accessor: an alias for `Reg<LOAD3_SPEC>`"]
pub type LOAD3 = crate::Reg<load3::LOAD3_SPEC>;
#[doc = "Timer Channel Load Register"]
pub mod load3;
#[doc = "HOLD3 (rw) register accessor: an alias for `Reg<HOLD3_SPEC>`"]
pub type HOLD3 = crate::Reg<hold3::HOLD3_SPEC>;
#[doc = "Timer Channel Hold Register"]
pub mod hold3;
#[doc = "CNTR3 (rw) register accessor: an alias for `Reg<CNTR3_SPEC>`"]
pub type CNTR3 = crate::Reg<cntr3::CNTR3_SPEC>;
#[doc = "Timer Channel Counter Register"]
pub mod cntr3;
#[doc = "CTRL3 (rw) register accessor: an alias for `Reg<CTRL3_SPEC>`"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ctrl3;
#[doc = "SCTRL3 (rw) register accessor: an alias for `Reg<SCTRL3_SPEC>`"]
pub type SCTRL3 = crate::Reg<sctrl3::SCTRL3_SPEC>;
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl3;
#[doc = "CMPLD13 (rw) register accessor: an alias for `Reg<CMPLD13_SPEC>`"]
pub type CMPLD13 = crate::Reg<cmpld13::CMPLD13_SPEC>;
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld13;
#[doc = "CMPLD23 (rw) register accessor: an alias for `Reg<CMPLD23_SPEC>`"]
pub type CMPLD23 = crate::Reg<cmpld23::CMPLD23_SPEC>;
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld23;
#[doc = "CSCTRL3 (rw) register accessor: an alias for `Reg<CSCTRL3_SPEC>`"]
pub type CSCTRL3 = crate::Reg<csctrl3::CSCTRL3_SPEC>;
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl3;
#[doc = "FILT3 (rw) register accessor: an alias for `Reg<FILT3_SPEC>`"]
pub type FILT3 = crate::Reg<filt3::FILT3_SPEC>;
#[doc = "Timer Channel Input Filter Register"]
pub mod filt3;
#[doc = "DMA3 (rw) register accessor: an alias for `Reg<DMA3_SPEC>`"]
pub type DMA3 = crate::Reg<dma3::DMA3_SPEC>;
#[doc = "Timer Channel DMA Enable Register"]
pub mod dma3;
