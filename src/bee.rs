#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Offset region 0 Register"]
    pub addr_offset0: ADDR_OFFSET0,
    #[doc = "0x08 - Offset region 1 Register"]
    pub addr_offset1: ADDR_OFFSET1,
    #[doc = "0x0c - AES Key 0 Register"]
    pub aes_key0_w0: AES_KEY0_W0,
    #[doc = "0x10 - AES Key 1 Register"]
    pub aes_key0_w1: AES_KEY0_W1,
    #[doc = "0x14 - AES Key 2 Register"]
    pub aes_key0_w2: AES_KEY0_W2,
    #[doc = "0x18 - AES Key 3 Register"]
    pub aes_key0_w3: AES_KEY0_W3,
    #[doc = "0x1c - Status Register"]
    pub status: STATUS,
    #[doc = "0x20 - NONCE00 Register"]
    pub ctr_nonce0_w0: CTR_NONCE0_W0,
    #[doc = "0x24 - NONCE01 Register"]
    pub ctr_nonce0_w1: CTR_NONCE0_W1,
    #[doc = "0x28 - NONCE02 Register"]
    pub ctr_nonce0_w2: CTR_NONCE0_W2,
    #[doc = "0x2c - NONCE03 Register"]
    pub ctr_nonce0_w3: CTR_NONCE0_W3,
    #[doc = "0x30 - NONCE10 Register"]
    pub ctr_nonce1_w0: CTR_NONCE1_W0,
    #[doc = "0x34 - NONCE11 Register"]
    pub ctr_nonce1_w1: CTR_NONCE1_W1,
    #[doc = "0x38 - NONCE12 Register"]
    pub ctr_nonce1_w2: CTR_NONCE1_W2,
    #[doc = "0x3c - NONCE13 Register"]
    pub ctr_nonce1_w3: CTR_NONCE1_W3,
    #[doc = "0x40 - Region1 Top Address Register"]
    pub region1_top: REGION1_TOP,
    #[doc = "0x44 - Region1 Bottom Address Register"]
    pub region1_bot: REGION1_BOT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDR_OFFSET0 (rw) register accessor: an alias for `Reg<ADDR_OFFSET0_SPEC>`"]
pub type ADDR_OFFSET0 = crate::Reg<addr_offset0::ADDR_OFFSET0_SPEC>;
#[doc = "Offset region 0 Register"]
pub mod addr_offset0;
#[doc = "ADDR_OFFSET1 (rw) register accessor: an alias for `Reg<ADDR_OFFSET1_SPEC>`"]
pub type ADDR_OFFSET1 = crate::Reg<addr_offset1::ADDR_OFFSET1_SPEC>;
#[doc = "Offset region 1 Register"]
pub mod addr_offset1;
#[doc = "AES_KEY0_W0 (w) register accessor: an alias for `Reg<AES_KEY0_W0_SPEC>`"]
pub type AES_KEY0_W0 = crate::Reg<aes_key0_w0::AES_KEY0_W0_SPEC>;
#[doc = "AES Key 0 Register"]
pub mod aes_key0_w0;
#[doc = "AES_KEY0_W1 (w) register accessor: an alias for `Reg<AES_KEY0_W1_SPEC>`"]
pub type AES_KEY0_W1 = crate::Reg<aes_key0_w1::AES_KEY0_W1_SPEC>;
#[doc = "AES Key 1 Register"]
pub mod aes_key0_w1;
#[doc = "AES_KEY0_W2 (w) register accessor: an alias for `Reg<AES_KEY0_W2_SPEC>`"]
pub type AES_KEY0_W2 = crate::Reg<aes_key0_w2::AES_KEY0_W2_SPEC>;
#[doc = "AES Key 2 Register"]
pub mod aes_key0_w2;
#[doc = "AES_KEY0_W3 (w) register accessor: an alias for `Reg<AES_KEY0_W3_SPEC>`"]
pub type AES_KEY0_W3 = crate::Reg<aes_key0_w3::AES_KEY0_W3_SPEC>;
#[doc = "AES Key 3 Register"]
pub mod aes_key0_w3;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CTR_NONCE0_W0 (w) register accessor: an alias for `Reg<CTR_NONCE0_W0_SPEC>`"]
pub type CTR_NONCE0_W0 = crate::Reg<ctr_nonce0_w0::CTR_NONCE0_W0_SPEC>;
#[doc = "NONCE00 Register"]
pub mod ctr_nonce0_w0;
#[doc = "CTR_NONCE0_W1 (w) register accessor: an alias for `Reg<CTR_NONCE0_W1_SPEC>`"]
pub type CTR_NONCE0_W1 = crate::Reg<ctr_nonce0_w1::CTR_NONCE0_W1_SPEC>;
#[doc = "NONCE01 Register"]
pub mod ctr_nonce0_w1;
#[doc = "CTR_NONCE0_W2 (w) register accessor: an alias for `Reg<CTR_NONCE0_W2_SPEC>`"]
pub type CTR_NONCE0_W2 = crate::Reg<ctr_nonce0_w2::CTR_NONCE0_W2_SPEC>;
#[doc = "NONCE02 Register"]
pub mod ctr_nonce0_w2;
#[doc = "CTR_NONCE0_W3 (w) register accessor: an alias for `Reg<CTR_NONCE0_W3_SPEC>`"]
pub type CTR_NONCE0_W3 = crate::Reg<ctr_nonce0_w3::CTR_NONCE0_W3_SPEC>;
#[doc = "NONCE03 Register"]
pub mod ctr_nonce0_w3;
#[doc = "CTR_NONCE1_W0 (w) register accessor: an alias for `Reg<CTR_NONCE1_W0_SPEC>`"]
pub type CTR_NONCE1_W0 = crate::Reg<ctr_nonce1_w0::CTR_NONCE1_W0_SPEC>;
#[doc = "NONCE10 Register"]
pub mod ctr_nonce1_w0;
#[doc = "CTR_NONCE1_W1 (w) register accessor: an alias for `Reg<CTR_NONCE1_W1_SPEC>`"]
pub type CTR_NONCE1_W1 = crate::Reg<ctr_nonce1_w1::CTR_NONCE1_W1_SPEC>;
#[doc = "NONCE11 Register"]
pub mod ctr_nonce1_w1;
#[doc = "CTR_NONCE1_W2 (w) register accessor: an alias for `Reg<CTR_NONCE1_W2_SPEC>`"]
pub type CTR_NONCE1_W2 = crate::Reg<ctr_nonce1_w2::CTR_NONCE1_W2_SPEC>;
#[doc = "NONCE12 Register"]
pub mod ctr_nonce1_w2;
#[doc = "CTR_NONCE1_W3 (w) register accessor: an alias for `Reg<CTR_NONCE1_W3_SPEC>`"]
pub type CTR_NONCE1_W3 = crate::Reg<ctr_nonce1_w3::CTR_NONCE1_W3_SPEC>;
#[doc = "NONCE13 Register"]
pub mod ctr_nonce1_w3;
#[doc = "REGION1_TOP (rw) register accessor: an alias for `Reg<REGION1_TOP_SPEC>`"]
pub type REGION1_TOP = crate::Reg<region1_top::REGION1_TOP_SPEC>;
#[doc = "Region1 Top Address Register"]
pub mod region1_top;
#[doc = "REGION1_BOT (rw) register accessor: an alias for `Reg<REGION1_BOT_SPEC>`"]
pub type REGION1_BOT = crate::Reg<region1_bot::REGION1_BOT_SPEC>;
#[doc = "Region1 Bottom Address Register"]
pub mod region1_bot;
