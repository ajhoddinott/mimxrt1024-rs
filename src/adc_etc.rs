#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_ETC Global Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ETC DONE0 and DONE1 IRQ State Register"]
    pub done0_1_irq: DONE0_1_IRQ,
    #[doc = "0x08 - ETC DONE_2 and DONE_ERR IRQ State Register"]
    pub done2_3_err_irq: DONE2_3_ERR_IRQ,
    #[doc = "0x0c - ETC DMA control Register"]
    pub dma_ctrl: DMA_CTRL,
    #[doc = "0x10 - ETC_TRIG Control Register"]
    pub trig0_ctrl: TRIG0_CTRL,
    #[doc = "0x14 - ETC_TRIG Counter Register"]
    pub trig0_counter: TRIG0_COUNTER,
    #[doc = "0x18 - ETC_TRIG Chain 0/1 Register"]
    pub trig0_chain_1_0: TRIG0_CHAIN_1_0,
    #[doc = "0x1c - ETC_TRIG Chain 2/3 Register"]
    pub trig0_chain_3_2: TRIG0_CHAIN_3_2,
    #[doc = "0x20 - ETC_TRIG Chain 4/5 Register"]
    pub trig0_chain_5_4: TRIG0_CHAIN_5_4,
    #[doc = "0x24 - ETC_TRIG Chain 6/7 Register"]
    pub trig0_chain_7_6: TRIG0_CHAIN_7_6,
    #[doc = "0x28 - ETC_TRIG Result Data 1/0 Register"]
    pub trig0_result_1_0: TRIG0_RESULT_1_0,
    #[doc = "0x2c - ETC_TRIG Result Data 3/2 Register"]
    pub trig0_result_3_2: TRIG0_RESULT_3_2,
    #[doc = "0x30 - ETC_TRIG Result Data 5/4 Register"]
    pub trig0_result_5_4: TRIG0_RESULT_5_4,
    #[doc = "0x34 - ETC_TRIG Result Data 7/6 Register"]
    pub trig0_result_7_6: TRIG0_RESULT_7_6,
    #[doc = "0x38 - ETC_TRIG Control Register"]
    pub trig1_ctrl: TRIG1_CTRL,
    #[doc = "0x3c - ETC_TRIG Counter Register"]
    pub trig1_counter: TRIG1_COUNTER,
    #[doc = "0x40 - ETC_TRIG Chain 0/1 Register"]
    pub trig1_chain_1_0: TRIG1_CHAIN_1_0,
    #[doc = "0x44 - ETC_TRIG Chain 2/3 Register"]
    pub trig1_chain_3_2: TRIG1_CHAIN_3_2,
    #[doc = "0x48 - ETC_TRIG Chain 4/5 Register"]
    pub trig1_chain_5_4: TRIG1_CHAIN_5_4,
    #[doc = "0x4c - ETC_TRIG Chain 6/7 Register"]
    pub trig1_chain_7_6: TRIG1_CHAIN_7_6,
    #[doc = "0x50 - ETC_TRIG Result Data 1/0 Register"]
    pub trig1_result_1_0: TRIG1_RESULT_1_0,
    #[doc = "0x54 - ETC_TRIG Result Data 3/2 Register"]
    pub trig1_result_3_2: TRIG1_RESULT_3_2,
    #[doc = "0x58 - ETC_TRIG Result Data 5/4 Register"]
    pub trig1_result_5_4: TRIG1_RESULT_5_4,
    #[doc = "0x5c - ETC_TRIG Result Data 7/6 Register"]
    pub trig1_result_7_6: TRIG1_RESULT_7_6,
    #[doc = "0x60 - ETC_TRIG Control Register"]
    pub trig2_ctrl: TRIG2_CTRL,
    #[doc = "0x64 - ETC_TRIG Counter Register"]
    pub trig2_counter: TRIG2_COUNTER,
    #[doc = "0x68 - ETC_TRIG Chain 0/1 Register"]
    pub trig2_chain_1_0: TRIG2_CHAIN_1_0,
    #[doc = "0x6c - ETC_TRIG Chain 2/3 Register"]
    pub trig2_chain_3_2: TRIG2_CHAIN_3_2,
    #[doc = "0x70 - ETC_TRIG Chain 4/5 Register"]
    pub trig2_chain_5_4: TRIG2_CHAIN_5_4,
    #[doc = "0x74 - ETC_TRIG Chain 6/7 Register"]
    pub trig2_chain_7_6: TRIG2_CHAIN_7_6,
    #[doc = "0x78 - ETC_TRIG Result Data 1/0 Register"]
    pub trig2_result_1_0: TRIG2_RESULT_1_0,
    #[doc = "0x7c - ETC_TRIG Result Data 3/2 Register"]
    pub trig2_result_3_2: TRIG2_RESULT_3_2,
    #[doc = "0x80 - ETC_TRIG Result Data 5/4 Register"]
    pub trig2_result_5_4: TRIG2_RESULT_5_4,
    #[doc = "0x84 - ETC_TRIG Result Data 7/6 Register"]
    pub trig2_result_7_6: TRIG2_RESULT_7_6,
    #[doc = "0x88 - ETC_TRIG Control Register"]
    pub trig3_ctrl: TRIG3_CTRL,
    #[doc = "0x8c - ETC_TRIG Counter Register"]
    pub trig3_counter: TRIG3_COUNTER,
    #[doc = "0x90 - ETC_TRIG Chain 0/1 Register"]
    pub trig3_chain_1_0: TRIG3_CHAIN_1_0,
    #[doc = "0x94 - ETC_TRIG Chain 2/3 Register"]
    pub trig3_chain_3_2: TRIG3_CHAIN_3_2,
    #[doc = "0x98 - ETC_TRIG Chain 4/5 Register"]
    pub trig3_chain_5_4: TRIG3_CHAIN_5_4,
    #[doc = "0x9c - ETC_TRIG Chain 6/7 Register"]
    pub trig3_chain_7_6: TRIG3_CHAIN_7_6,
    #[doc = "0xa0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig3_result_1_0: TRIG3_RESULT_1_0,
    #[doc = "0xa4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig3_result_3_2: TRIG3_RESULT_3_2,
    #[doc = "0xa8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig3_result_5_4: TRIG3_RESULT_5_4,
    #[doc = "0xac - ETC_TRIG Result Data 7/6 Register"]
    pub trig3_result_7_6: TRIG3_RESULT_7_6,
    #[doc = "0xb0 - ETC_TRIG Control Register"]
    pub trig4_ctrl: TRIG4_CTRL,
    #[doc = "0xb4 - ETC_TRIG Counter Register"]
    pub trig4_counter: TRIG4_COUNTER,
    #[doc = "0xb8 - ETC_TRIG Chain 0/1 Register"]
    pub trig4_chain_1_0: TRIG4_CHAIN_1_0,
    #[doc = "0xbc - ETC_TRIG Chain 2/3 Register"]
    pub trig4_chain_3_2: TRIG4_CHAIN_3_2,
    #[doc = "0xc0 - ETC_TRIG Chain 4/5 Register"]
    pub trig4_chain_5_4: TRIG4_CHAIN_5_4,
    #[doc = "0xc4 - ETC_TRIG Chain 6/7 Register"]
    pub trig4_chain_7_6: TRIG4_CHAIN_7_6,
    #[doc = "0xc8 - ETC_TRIG Result Data 1/0 Register"]
    pub trig4_result_1_0: TRIG4_RESULT_1_0,
    #[doc = "0xcc - ETC_TRIG Result Data 3/2 Register"]
    pub trig4_result_3_2: TRIG4_RESULT_3_2,
    #[doc = "0xd0 - ETC_TRIG Result Data 5/4 Register"]
    pub trig4_result_5_4: TRIG4_RESULT_5_4,
    #[doc = "0xd4 - ETC_TRIG Result Data 7/6 Register"]
    pub trig4_result_7_6: TRIG4_RESULT_7_6,
    #[doc = "0xd8 - ETC_TRIG Control Register"]
    pub trig5_ctrl: TRIG5_CTRL,
    #[doc = "0xdc - ETC_TRIG Counter Register"]
    pub trig5_counter: TRIG5_COUNTER,
    #[doc = "0xe0 - ETC_TRIG Chain 0/1 Register"]
    pub trig5_chain_1_0: TRIG5_CHAIN_1_0,
    #[doc = "0xe4 - ETC_TRIG Chain 2/3 Register"]
    pub trig5_chain_3_2: TRIG5_CHAIN_3_2,
    #[doc = "0xe8 - ETC_TRIG Chain 4/5 Register"]
    pub trig5_chain_5_4: TRIG5_CHAIN_5_4,
    #[doc = "0xec - ETC_TRIG Chain 6/7 Register"]
    pub trig5_chain_7_6: TRIG5_CHAIN_7_6,
    #[doc = "0xf0 - ETC_TRIG Result Data 1/0 Register"]
    pub trig5_result_1_0: TRIG5_RESULT_1_0,
    #[doc = "0xf4 - ETC_TRIG Result Data 3/2 Register"]
    pub trig5_result_3_2: TRIG5_RESULT_3_2,
    #[doc = "0xf8 - ETC_TRIG Result Data 5/4 Register"]
    pub trig5_result_5_4: TRIG5_RESULT_5_4,
    #[doc = "0xfc - ETC_TRIG Result Data 7/6 Register"]
    pub trig5_result_7_6: TRIG5_RESULT_7_6,
    #[doc = "0x100 - ETC_TRIG Control Register"]
    pub trig6_ctrl: TRIG6_CTRL,
    #[doc = "0x104 - ETC_TRIG Counter Register"]
    pub trig6_counter: TRIG6_COUNTER,
    #[doc = "0x108 - ETC_TRIG Chain 0/1 Register"]
    pub trig6_chain_1_0: TRIG6_CHAIN_1_0,
    #[doc = "0x10c - ETC_TRIG Chain 2/3 Register"]
    pub trig6_chain_3_2: TRIG6_CHAIN_3_2,
    #[doc = "0x110 - ETC_TRIG Chain 4/5 Register"]
    pub trig6_chain_5_4: TRIG6_CHAIN_5_4,
    #[doc = "0x114 - ETC_TRIG Chain 6/7 Register"]
    pub trig6_chain_7_6: TRIG6_CHAIN_7_6,
    #[doc = "0x118 - ETC_TRIG Result Data 1/0 Register"]
    pub trig6_result_1_0: TRIG6_RESULT_1_0,
    #[doc = "0x11c - ETC_TRIG Result Data 3/2 Register"]
    pub trig6_result_3_2: TRIG6_RESULT_3_2,
    #[doc = "0x120 - ETC_TRIG Result Data 5/4 Register"]
    pub trig6_result_5_4: TRIG6_RESULT_5_4,
    #[doc = "0x124 - ETC_TRIG Result Data 7/6 Register"]
    pub trig6_result_7_6: TRIG6_RESULT_7_6,
    #[doc = "0x128 - ETC_TRIG Control Register"]
    pub trig7_ctrl: TRIG7_CTRL,
    #[doc = "0x12c - ETC_TRIG Counter Register"]
    pub trig7_counter: TRIG7_COUNTER,
    #[doc = "0x130 - ETC_TRIG Chain 0/1 Register"]
    pub trig7_chain_1_0: TRIG7_CHAIN_1_0,
    #[doc = "0x134 - ETC_TRIG Chain 2/3 Register"]
    pub trig7_chain_3_2: TRIG7_CHAIN_3_2,
    #[doc = "0x138 - ETC_TRIG Chain 4/5 Register"]
    pub trig7_chain_5_4: TRIG7_CHAIN_5_4,
    #[doc = "0x13c - ETC_TRIG Chain 6/7 Register"]
    pub trig7_chain_7_6: TRIG7_CHAIN_7_6,
    #[doc = "0x140 - ETC_TRIG Result Data 1/0 Register"]
    pub trig7_result_1_0: TRIG7_RESULT_1_0,
    #[doc = "0x144 - ETC_TRIG Result Data 3/2 Register"]
    pub trig7_result_3_2: TRIG7_RESULT_3_2,
    #[doc = "0x148 - ETC_TRIG Result Data 5/4 Register"]
    pub trig7_result_5_4: TRIG7_RESULT_5_4,
    #[doc = "0x14c - ETC_TRIG Result Data 7/6 Register"]
    pub trig7_result_7_6: TRIG7_RESULT_7_6,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC_ETC Global Control Register"]
pub mod ctrl;
#[doc = "DONE0_1_IRQ (rw) register accessor: an alias for `Reg<DONE0_1_IRQ_SPEC>`"]
pub type DONE0_1_IRQ = crate::Reg<done0_1_irq::DONE0_1_IRQ_SPEC>;
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
pub mod done0_1_irq;
#[doc = "DONE2_3_ERR_IRQ (rw) register accessor: an alias for `Reg<DONE2_3_ERR_IRQ_SPEC>`"]
pub type DONE2_3_ERR_IRQ = crate::Reg<done2_3_err_irq::DONE2_3_ERR_IRQ_SPEC>;
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
pub mod done2_3_err_irq;
#[doc = "DMA_CTRL (rw) register accessor: an alias for `Reg<DMA_CTRL_SPEC>`"]
pub type DMA_CTRL = crate::Reg<dma_ctrl::DMA_CTRL_SPEC>;
#[doc = "ETC DMA control Register"]
pub mod dma_ctrl;
#[doc = "TRIG0_CTRL (rw) register accessor: an alias for `Reg<TRIG0_CTRL_SPEC>`"]
pub type TRIG0_CTRL = crate::Reg<trig0_ctrl::TRIG0_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig0_ctrl;
#[doc = "TRIG0_COUNTER (rw) register accessor: an alias for `Reg<TRIG0_COUNTER_SPEC>`"]
pub type TRIG0_COUNTER = crate::Reg<trig0_counter::TRIG0_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig0_counter;
#[doc = "TRIG0_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG0_CHAIN_1_0_SPEC>`"]
pub type TRIG0_CHAIN_1_0 = crate::Reg<trig0_chain_1_0::TRIG0_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig0_chain_1_0;
#[doc = "TRIG0_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG0_CHAIN_3_2_SPEC>`"]
pub type TRIG0_CHAIN_3_2 = crate::Reg<trig0_chain_3_2::TRIG0_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig0_chain_3_2;
#[doc = "TRIG0_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG0_CHAIN_5_4_SPEC>`"]
pub type TRIG0_CHAIN_5_4 = crate::Reg<trig0_chain_5_4::TRIG0_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig0_chain_5_4;
#[doc = "TRIG0_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG0_CHAIN_7_6_SPEC>`"]
pub type TRIG0_CHAIN_7_6 = crate::Reg<trig0_chain_7_6::TRIG0_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig0_chain_7_6;
#[doc = "TRIG0_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG0_RESULT_1_0_SPEC>`"]
pub type TRIG0_RESULT_1_0 = crate::Reg<trig0_result_1_0::TRIG0_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig0_result_1_0;
#[doc = "TRIG0_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG0_RESULT_3_2_SPEC>`"]
pub type TRIG0_RESULT_3_2 = crate::Reg<trig0_result_3_2::TRIG0_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig0_result_3_2;
#[doc = "TRIG0_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG0_RESULT_5_4_SPEC>`"]
pub type TRIG0_RESULT_5_4 = crate::Reg<trig0_result_5_4::TRIG0_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig0_result_5_4;
#[doc = "TRIG0_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG0_RESULT_7_6_SPEC>`"]
pub type TRIG0_RESULT_7_6 = crate::Reg<trig0_result_7_6::TRIG0_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig0_result_7_6;
#[doc = "TRIG1_CTRL (rw) register accessor: an alias for `Reg<TRIG1_CTRL_SPEC>`"]
pub type TRIG1_CTRL = crate::Reg<trig1_ctrl::TRIG1_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig1_ctrl;
#[doc = "TRIG1_COUNTER (rw) register accessor: an alias for `Reg<TRIG1_COUNTER_SPEC>`"]
pub type TRIG1_COUNTER = crate::Reg<trig1_counter::TRIG1_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig1_counter;
#[doc = "TRIG1_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG1_CHAIN_1_0_SPEC>`"]
pub type TRIG1_CHAIN_1_0 = crate::Reg<trig1_chain_1_0::TRIG1_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig1_chain_1_0;
#[doc = "TRIG1_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG1_CHAIN_3_2_SPEC>`"]
pub type TRIG1_CHAIN_3_2 = crate::Reg<trig1_chain_3_2::TRIG1_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig1_chain_3_2;
#[doc = "TRIG1_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG1_CHAIN_5_4_SPEC>`"]
pub type TRIG1_CHAIN_5_4 = crate::Reg<trig1_chain_5_4::TRIG1_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig1_chain_5_4;
#[doc = "TRIG1_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG1_CHAIN_7_6_SPEC>`"]
pub type TRIG1_CHAIN_7_6 = crate::Reg<trig1_chain_7_6::TRIG1_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig1_chain_7_6;
#[doc = "TRIG1_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG1_RESULT_1_0_SPEC>`"]
pub type TRIG1_RESULT_1_0 = crate::Reg<trig1_result_1_0::TRIG1_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig1_result_1_0;
#[doc = "TRIG1_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG1_RESULT_3_2_SPEC>`"]
pub type TRIG1_RESULT_3_2 = crate::Reg<trig1_result_3_2::TRIG1_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig1_result_3_2;
#[doc = "TRIG1_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG1_RESULT_5_4_SPEC>`"]
pub type TRIG1_RESULT_5_4 = crate::Reg<trig1_result_5_4::TRIG1_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig1_result_5_4;
#[doc = "TRIG1_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG1_RESULT_7_6_SPEC>`"]
pub type TRIG1_RESULT_7_6 = crate::Reg<trig1_result_7_6::TRIG1_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig1_result_7_6;
#[doc = "TRIG2_CTRL (rw) register accessor: an alias for `Reg<TRIG2_CTRL_SPEC>`"]
pub type TRIG2_CTRL = crate::Reg<trig2_ctrl::TRIG2_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig2_ctrl;
#[doc = "TRIG2_COUNTER (rw) register accessor: an alias for `Reg<TRIG2_COUNTER_SPEC>`"]
pub type TRIG2_COUNTER = crate::Reg<trig2_counter::TRIG2_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig2_counter;
#[doc = "TRIG2_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG2_CHAIN_1_0_SPEC>`"]
pub type TRIG2_CHAIN_1_0 = crate::Reg<trig2_chain_1_0::TRIG2_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig2_chain_1_0;
#[doc = "TRIG2_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG2_CHAIN_3_2_SPEC>`"]
pub type TRIG2_CHAIN_3_2 = crate::Reg<trig2_chain_3_2::TRIG2_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig2_chain_3_2;
#[doc = "TRIG2_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG2_CHAIN_5_4_SPEC>`"]
pub type TRIG2_CHAIN_5_4 = crate::Reg<trig2_chain_5_4::TRIG2_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig2_chain_5_4;
#[doc = "TRIG2_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG2_CHAIN_7_6_SPEC>`"]
pub type TRIG2_CHAIN_7_6 = crate::Reg<trig2_chain_7_6::TRIG2_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig2_chain_7_6;
#[doc = "TRIG2_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG2_RESULT_1_0_SPEC>`"]
pub type TRIG2_RESULT_1_0 = crate::Reg<trig2_result_1_0::TRIG2_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig2_result_1_0;
#[doc = "TRIG2_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG2_RESULT_3_2_SPEC>`"]
pub type TRIG2_RESULT_3_2 = crate::Reg<trig2_result_3_2::TRIG2_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig2_result_3_2;
#[doc = "TRIG2_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG2_RESULT_5_4_SPEC>`"]
pub type TRIG2_RESULT_5_4 = crate::Reg<trig2_result_5_4::TRIG2_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig2_result_5_4;
#[doc = "TRIG2_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG2_RESULT_7_6_SPEC>`"]
pub type TRIG2_RESULT_7_6 = crate::Reg<trig2_result_7_6::TRIG2_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig2_result_7_6;
#[doc = "TRIG3_CTRL (rw) register accessor: an alias for `Reg<TRIG3_CTRL_SPEC>`"]
pub type TRIG3_CTRL = crate::Reg<trig3_ctrl::TRIG3_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig3_ctrl;
#[doc = "TRIG3_COUNTER (rw) register accessor: an alias for `Reg<TRIG3_COUNTER_SPEC>`"]
pub type TRIG3_COUNTER = crate::Reg<trig3_counter::TRIG3_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig3_counter;
#[doc = "TRIG3_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG3_CHAIN_1_0_SPEC>`"]
pub type TRIG3_CHAIN_1_0 = crate::Reg<trig3_chain_1_0::TRIG3_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig3_chain_1_0;
#[doc = "TRIG3_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG3_CHAIN_3_2_SPEC>`"]
pub type TRIG3_CHAIN_3_2 = crate::Reg<trig3_chain_3_2::TRIG3_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig3_chain_3_2;
#[doc = "TRIG3_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG3_CHAIN_5_4_SPEC>`"]
pub type TRIG3_CHAIN_5_4 = crate::Reg<trig3_chain_5_4::TRIG3_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig3_chain_5_4;
#[doc = "TRIG3_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG3_CHAIN_7_6_SPEC>`"]
pub type TRIG3_CHAIN_7_6 = crate::Reg<trig3_chain_7_6::TRIG3_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig3_chain_7_6;
#[doc = "TRIG3_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG3_RESULT_1_0_SPEC>`"]
pub type TRIG3_RESULT_1_0 = crate::Reg<trig3_result_1_0::TRIG3_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig3_result_1_0;
#[doc = "TRIG3_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG3_RESULT_3_2_SPEC>`"]
pub type TRIG3_RESULT_3_2 = crate::Reg<trig3_result_3_2::TRIG3_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig3_result_3_2;
#[doc = "TRIG3_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG3_RESULT_5_4_SPEC>`"]
pub type TRIG3_RESULT_5_4 = crate::Reg<trig3_result_5_4::TRIG3_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig3_result_5_4;
#[doc = "TRIG3_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG3_RESULT_7_6_SPEC>`"]
pub type TRIG3_RESULT_7_6 = crate::Reg<trig3_result_7_6::TRIG3_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig3_result_7_6;
#[doc = "TRIG4_CTRL (rw) register accessor: an alias for `Reg<TRIG4_CTRL_SPEC>`"]
pub type TRIG4_CTRL = crate::Reg<trig4_ctrl::TRIG4_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig4_ctrl;
#[doc = "TRIG4_COUNTER (rw) register accessor: an alias for `Reg<TRIG4_COUNTER_SPEC>`"]
pub type TRIG4_COUNTER = crate::Reg<trig4_counter::TRIG4_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig4_counter;
#[doc = "TRIG4_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG4_CHAIN_1_0_SPEC>`"]
pub type TRIG4_CHAIN_1_0 = crate::Reg<trig4_chain_1_0::TRIG4_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig4_chain_1_0;
#[doc = "TRIG4_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG4_CHAIN_3_2_SPEC>`"]
pub type TRIG4_CHAIN_3_2 = crate::Reg<trig4_chain_3_2::TRIG4_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig4_chain_3_2;
#[doc = "TRIG4_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG4_CHAIN_5_4_SPEC>`"]
pub type TRIG4_CHAIN_5_4 = crate::Reg<trig4_chain_5_4::TRIG4_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig4_chain_5_4;
#[doc = "TRIG4_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG4_CHAIN_7_6_SPEC>`"]
pub type TRIG4_CHAIN_7_6 = crate::Reg<trig4_chain_7_6::TRIG4_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig4_chain_7_6;
#[doc = "TRIG4_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG4_RESULT_1_0_SPEC>`"]
pub type TRIG4_RESULT_1_0 = crate::Reg<trig4_result_1_0::TRIG4_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig4_result_1_0;
#[doc = "TRIG4_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG4_RESULT_3_2_SPEC>`"]
pub type TRIG4_RESULT_3_2 = crate::Reg<trig4_result_3_2::TRIG4_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig4_result_3_2;
#[doc = "TRIG4_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG4_RESULT_5_4_SPEC>`"]
pub type TRIG4_RESULT_5_4 = crate::Reg<trig4_result_5_4::TRIG4_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig4_result_5_4;
#[doc = "TRIG4_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG4_RESULT_7_6_SPEC>`"]
pub type TRIG4_RESULT_7_6 = crate::Reg<trig4_result_7_6::TRIG4_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig4_result_7_6;
#[doc = "TRIG5_CTRL (rw) register accessor: an alias for `Reg<TRIG5_CTRL_SPEC>`"]
pub type TRIG5_CTRL = crate::Reg<trig5_ctrl::TRIG5_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig5_ctrl;
#[doc = "TRIG5_COUNTER (rw) register accessor: an alias for `Reg<TRIG5_COUNTER_SPEC>`"]
pub type TRIG5_COUNTER = crate::Reg<trig5_counter::TRIG5_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig5_counter;
#[doc = "TRIG5_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG5_CHAIN_1_0_SPEC>`"]
pub type TRIG5_CHAIN_1_0 = crate::Reg<trig5_chain_1_0::TRIG5_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig5_chain_1_0;
#[doc = "TRIG5_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG5_CHAIN_3_2_SPEC>`"]
pub type TRIG5_CHAIN_3_2 = crate::Reg<trig5_chain_3_2::TRIG5_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig5_chain_3_2;
#[doc = "TRIG5_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG5_CHAIN_5_4_SPEC>`"]
pub type TRIG5_CHAIN_5_4 = crate::Reg<trig5_chain_5_4::TRIG5_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig5_chain_5_4;
#[doc = "TRIG5_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG5_CHAIN_7_6_SPEC>`"]
pub type TRIG5_CHAIN_7_6 = crate::Reg<trig5_chain_7_6::TRIG5_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig5_chain_7_6;
#[doc = "TRIG5_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG5_RESULT_1_0_SPEC>`"]
pub type TRIG5_RESULT_1_0 = crate::Reg<trig5_result_1_0::TRIG5_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig5_result_1_0;
#[doc = "TRIG5_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG5_RESULT_3_2_SPEC>`"]
pub type TRIG5_RESULT_3_2 = crate::Reg<trig5_result_3_2::TRIG5_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig5_result_3_2;
#[doc = "TRIG5_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG5_RESULT_5_4_SPEC>`"]
pub type TRIG5_RESULT_5_4 = crate::Reg<trig5_result_5_4::TRIG5_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig5_result_5_4;
#[doc = "TRIG5_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG5_RESULT_7_6_SPEC>`"]
pub type TRIG5_RESULT_7_6 = crate::Reg<trig5_result_7_6::TRIG5_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig5_result_7_6;
#[doc = "TRIG6_CTRL (rw) register accessor: an alias for `Reg<TRIG6_CTRL_SPEC>`"]
pub type TRIG6_CTRL = crate::Reg<trig6_ctrl::TRIG6_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig6_ctrl;
#[doc = "TRIG6_COUNTER (rw) register accessor: an alias for `Reg<TRIG6_COUNTER_SPEC>`"]
pub type TRIG6_COUNTER = crate::Reg<trig6_counter::TRIG6_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig6_counter;
#[doc = "TRIG6_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG6_CHAIN_1_0_SPEC>`"]
pub type TRIG6_CHAIN_1_0 = crate::Reg<trig6_chain_1_0::TRIG6_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig6_chain_1_0;
#[doc = "TRIG6_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG6_CHAIN_3_2_SPEC>`"]
pub type TRIG6_CHAIN_3_2 = crate::Reg<trig6_chain_3_2::TRIG6_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig6_chain_3_2;
#[doc = "TRIG6_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG6_CHAIN_5_4_SPEC>`"]
pub type TRIG6_CHAIN_5_4 = crate::Reg<trig6_chain_5_4::TRIG6_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig6_chain_5_4;
#[doc = "TRIG6_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG6_CHAIN_7_6_SPEC>`"]
pub type TRIG6_CHAIN_7_6 = crate::Reg<trig6_chain_7_6::TRIG6_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig6_chain_7_6;
#[doc = "TRIG6_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG6_RESULT_1_0_SPEC>`"]
pub type TRIG6_RESULT_1_0 = crate::Reg<trig6_result_1_0::TRIG6_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig6_result_1_0;
#[doc = "TRIG6_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG6_RESULT_3_2_SPEC>`"]
pub type TRIG6_RESULT_3_2 = crate::Reg<trig6_result_3_2::TRIG6_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig6_result_3_2;
#[doc = "TRIG6_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG6_RESULT_5_4_SPEC>`"]
pub type TRIG6_RESULT_5_4 = crate::Reg<trig6_result_5_4::TRIG6_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig6_result_5_4;
#[doc = "TRIG6_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG6_RESULT_7_6_SPEC>`"]
pub type TRIG6_RESULT_7_6 = crate::Reg<trig6_result_7_6::TRIG6_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig6_result_7_6;
#[doc = "TRIG7_CTRL (rw) register accessor: an alias for `Reg<TRIG7_CTRL_SPEC>`"]
pub type TRIG7_CTRL = crate::Reg<trig7_ctrl::TRIG7_CTRL_SPEC>;
#[doc = "ETC_TRIG Control Register"]
pub mod trig7_ctrl;
#[doc = "TRIG7_COUNTER (rw) register accessor: an alias for `Reg<TRIG7_COUNTER_SPEC>`"]
pub type TRIG7_COUNTER = crate::Reg<trig7_counter::TRIG7_COUNTER_SPEC>;
#[doc = "ETC_TRIG Counter Register"]
pub mod trig7_counter;
#[doc = "TRIG7_CHAIN_1_0 (rw) register accessor: an alias for `Reg<TRIG7_CHAIN_1_0_SPEC>`"]
pub type TRIG7_CHAIN_1_0 = crate::Reg<trig7_chain_1_0::TRIG7_CHAIN_1_0_SPEC>;
#[doc = "ETC_TRIG Chain 0/1 Register"]
pub mod trig7_chain_1_0;
#[doc = "TRIG7_CHAIN_3_2 (rw) register accessor: an alias for `Reg<TRIG7_CHAIN_3_2_SPEC>`"]
pub type TRIG7_CHAIN_3_2 = crate::Reg<trig7_chain_3_2::TRIG7_CHAIN_3_2_SPEC>;
#[doc = "ETC_TRIG Chain 2/3 Register"]
pub mod trig7_chain_3_2;
#[doc = "TRIG7_CHAIN_5_4 (rw) register accessor: an alias for `Reg<TRIG7_CHAIN_5_4_SPEC>`"]
pub type TRIG7_CHAIN_5_4 = crate::Reg<trig7_chain_5_4::TRIG7_CHAIN_5_4_SPEC>;
#[doc = "ETC_TRIG Chain 4/5 Register"]
pub mod trig7_chain_5_4;
#[doc = "TRIG7_CHAIN_7_6 (rw) register accessor: an alias for `Reg<TRIG7_CHAIN_7_6_SPEC>`"]
pub type TRIG7_CHAIN_7_6 = crate::Reg<trig7_chain_7_6::TRIG7_CHAIN_7_6_SPEC>;
#[doc = "ETC_TRIG Chain 6/7 Register"]
pub mod trig7_chain_7_6;
#[doc = "TRIG7_RESULT_1_0 (r) register accessor: an alias for `Reg<TRIG7_RESULT_1_0_SPEC>`"]
pub type TRIG7_RESULT_1_0 = crate::Reg<trig7_result_1_0::TRIG7_RESULT_1_0_SPEC>;
#[doc = "ETC_TRIG Result Data 1/0 Register"]
pub mod trig7_result_1_0;
#[doc = "TRIG7_RESULT_3_2 (r) register accessor: an alias for `Reg<TRIG7_RESULT_3_2_SPEC>`"]
pub type TRIG7_RESULT_3_2 = crate::Reg<trig7_result_3_2::TRIG7_RESULT_3_2_SPEC>;
#[doc = "ETC_TRIG Result Data 3/2 Register"]
pub mod trig7_result_3_2;
#[doc = "TRIG7_RESULT_5_4 (r) register accessor: an alias for `Reg<TRIG7_RESULT_5_4_SPEC>`"]
pub type TRIG7_RESULT_5_4 = crate::Reg<trig7_result_5_4::TRIG7_RESULT_5_4_SPEC>;
#[doc = "ETC_TRIG Result Data 5/4 Register"]
pub mod trig7_result_5_4;
#[doc = "TRIG7_RESULT_7_6 (r) register accessor: an alias for `Reg<TRIG7_RESULT_7_6_SPEC>`"]
pub type TRIG7_RESULT_7_6 = crate::Reg<trig7_result_7_6::TRIG7_RESULT_7_6_SPEC>;
#[doc = "ETC_TRIG Result Data 7/6 Register"]
pub mod trig7_result_7_6;
