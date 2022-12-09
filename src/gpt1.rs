#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPT Control Register"]
    pub cr: CR,
    #[doc = "0x04 - GPT Prescaler Register"]
    pub pr: PR,
    #[doc = "0x08 - GPT Status Register"]
    pub sr: SR,
    #[doc = "0x0c - GPT Interrupt Register"]
    pub ir: IR,
    #[doc = "0x10 - GPT Output Compare Register 1"]
    pub ocr1: OCR1,
    #[doc = "0x14 - GPT Output Compare Register 2"]
    pub ocr2: OCR2,
    #[doc = "0x18 - GPT Output Compare Register 3"]
    pub ocr3: OCR3,
    #[doc = "0x1c - GPT Input Capture Register 1"]
    pub icr1: ICR1,
    #[doc = "0x20 - GPT Input Capture Register 2"]
    pub icr2: ICR2,
    #[doc = "0x24 - GPT Counter Register"]
    pub cnt: CNT,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "GPT Control Register"]
pub mod cr;
#[doc = "PR (rw) register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "GPT Prescaler Register"]
pub mod pr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "GPT Status Register"]
pub mod sr;
#[doc = "IR (rw) register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "GPT Interrupt Register"]
pub mod ir;
#[doc = "OCR1 (rw) register accessor: an alias for `Reg<OCR1_SPEC>`"]
pub type OCR1 = crate::Reg<ocr1::OCR1_SPEC>;
#[doc = "GPT Output Compare Register 1"]
pub mod ocr1;
#[doc = "OCR2 (rw) register accessor: an alias for `Reg<OCR2_SPEC>`"]
pub type OCR2 = crate::Reg<ocr2::OCR2_SPEC>;
#[doc = "GPT Output Compare Register 2"]
pub mod ocr2;
#[doc = "OCR3 (rw) register accessor: an alias for `Reg<OCR3_SPEC>`"]
pub type OCR3 = crate::Reg<ocr3::OCR3_SPEC>;
#[doc = "GPT Output Compare Register 3"]
pub mod ocr3;
#[doc = "ICR1 (r) register accessor: an alias for `Reg<ICR1_SPEC>`"]
pub type ICR1 = crate::Reg<icr1::ICR1_SPEC>;
#[doc = "GPT Input Capture Register 1"]
pub mod icr1;
#[doc = "ICR2 (r) register accessor: an alias for `Reg<ICR2_SPEC>`"]
pub type ICR2 = crate::Reg<icr2::ICR2_SPEC>;
#[doc = "GPT Input Capture Register 2"]
pub mod icr2;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "GPT Counter Register"]
pub mod cnt;
