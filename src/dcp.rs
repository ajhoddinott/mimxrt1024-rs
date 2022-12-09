#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCP control register 0"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DCP control register 0"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x08 - DCP control register 0"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x0c - DCP control register 0"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x10 - DCP status register"]
    pub stat: STAT,
    #[doc = "0x14 - DCP status register"]
    pub stat_set: STAT_SET,
    #[doc = "0x18 - DCP status register"]
    pub stat_clr: STAT_CLR,
    #[doc = "0x1c - DCP status register"]
    pub stat_tog: STAT_TOG,
    #[doc = "0x20 - DCP channel control register"]
    pub channelctrl: CHANNELCTRL,
    #[doc = "0x24 - DCP channel control register"]
    pub channelctrl_set: CHANNELCTRL_SET,
    #[doc = "0x28 - DCP channel control register"]
    pub channelctrl_clr: CHANNELCTRL_CLR,
    #[doc = "0x2c - DCP channel control register"]
    pub channelctrl_tog: CHANNELCTRL_TOG,
    #[doc = "0x30 - DCP capability 0 register"]
    pub capability0: CAPABILITY0,
    _reserved13: [u8; 0x0c],
    #[doc = "0x40 - DCP capability 1 register"]
    pub capability1: CAPABILITY1,
    _reserved14: [u8; 0x0c],
    #[doc = "0x50 - DCP context buffer pointer"]
    pub context: CONTEXT,
    _reserved15: [u8; 0x0c],
    #[doc = "0x60 - DCP key index"]
    pub key: KEY,
    _reserved16: [u8; 0x0c],
    #[doc = "0x70 - DCP key data"]
    pub keydata: KEYDATA,
    _reserved17: [u8; 0x0c],
    #[doc = "0x80 - DCP work packet 0 status register"]
    pub packet0: PACKET0,
    _reserved18: [u8; 0x0c],
    #[doc = "0x90 - DCP work packet 1 status register"]
    pub packet1: PACKET1,
    _reserved19: [u8; 0x0c],
    #[doc = "0xa0 - DCP work packet 2 status register"]
    pub packet2: PACKET2,
    _reserved20: [u8; 0x0c],
    #[doc = "0xb0 - DCP work packet 3 status register"]
    pub packet3: PACKET3,
    _reserved21: [u8; 0x0c],
    #[doc = "0xc0 - DCP work packet 4 status register"]
    pub packet4: PACKET4,
    _reserved22: [u8; 0x0c],
    #[doc = "0xd0 - DCP work packet 5 status register"]
    pub packet5: PACKET5,
    _reserved23: [u8; 0x0c],
    #[doc = "0xe0 - DCP work packet 6 status register"]
    pub packet6: PACKET6,
    _reserved24: [u8; 0x1c],
    #[doc = "0x100 - DCP channel 0 command pointer address register"]
    pub ch0cmdptr: CH0CMDPTR,
    _reserved25: [u8; 0x0c],
    #[doc = "0x110 - DCP channel 0 semaphore register"]
    pub ch0sema: CH0SEMA,
    _reserved26: [u8; 0x0c],
    #[doc = "0x120 - DCP channel 0 status register"]
    pub ch0stat: CH0STAT,
    #[doc = "0x124 - DCP channel 0 status register"]
    pub ch0stat_set: CH0STAT_SET,
    #[doc = "0x128 - DCP channel 0 status register"]
    pub ch0stat_clr: CH0STAT_CLR,
    #[doc = "0x12c - DCP channel 0 status register"]
    pub ch0stat_tog: CH0STAT_TOG,
    #[doc = "0x130 - DCP channel 0 options register"]
    pub ch0opts: CH0OPTS,
    #[doc = "0x134 - DCP channel 0 options register"]
    pub ch0opts_set: CH0OPTS_SET,
    #[doc = "0x138 - DCP channel 0 options register"]
    pub ch0opts_clr: CH0OPTS_CLR,
    #[doc = "0x13c - DCP channel 0 options register"]
    pub ch0opts_tog: CH0OPTS_TOG,
    #[doc = "0x140 - DCP channel 1 command pointer address register"]
    pub ch1cmdptr: CH1CMDPTR,
    _reserved35: [u8; 0x0c],
    #[doc = "0x150 - DCP channel 1 semaphore register"]
    pub ch1sema: CH1SEMA,
    _reserved36: [u8; 0x0c],
    #[doc = "0x160 - DCP channel 1 status register"]
    pub ch1stat: CH1STAT,
    #[doc = "0x164 - DCP channel 1 status register"]
    pub ch1stat_set: CH1STAT_SET,
    #[doc = "0x168 - DCP channel 1 status register"]
    pub ch1stat_clr: CH1STAT_CLR,
    #[doc = "0x16c - DCP channel 1 status register"]
    pub ch1stat_tog: CH1STAT_TOG,
    #[doc = "0x170 - DCP channel 1 options register"]
    pub ch1opts: CH1OPTS,
    #[doc = "0x174 - DCP channel 1 options register"]
    pub ch1opts_set: CH1OPTS_SET,
    #[doc = "0x178 - DCP channel 1 options register"]
    pub ch1opts_clr: CH1OPTS_CLR,
    #[doc = "0x17c - DCP channel 1 options register"]
    pub ch1opts_tog: CH1OPTS_TOG,
    #[doc = "0x180 - DCP channel 2 command pointer address register"]
    pub ch2cmdptr: CH2CMDPTR,
    _reserved45: [u8; 0x0c],
    #[doc = "0x190 - DCP channel 2 semaphore register"]
    pub ch2sema: CH2SEMA,
    _reserved46: [u8; 0x0c],
    #[doc = "0x1a0 - DCP channel 2 status register"]
    pub ch2stat: CH2STAT,
    #[doc = "0x1a4 - DCP channel 2 status register"]
    pub ch2stat_set: CH2STAT_SET,
    #[doc = "0x1a8 - DCP channel 2 status register"]
    pub ch2stat_clr: CH2STAT_CLR,
    #[doc = "0x1ac - DCP channel 2 status register"]
    pub ch2stat_tog: CH2STAT_TOG,
    #[doc = "0x1b0 - DCP channel 2 options register"]
    pub ch2opts: CH2OPTS,
    #[doc = "0x1b4 - DCP channel 2 options register"]
    pub ch2opts_set: CH2OPTS_SET,
    #[doc = "0x1b8 - DCP channel 2 options register"]
    pub ch2opts_clr: CH2OPTS_CLR,
    #[doc = "0x1bc - DCP channel 2 options register"]
    pub ch2opts_tog: CH2OPTS_TOG,
    #[doc = "0x1c0 - DCP channel 3 command pointer address register"]
    pub ch3cmdptr: CH3CMDPTR,
    _reserved55: [u8; 0x0c],
    #[doc = "0x1d0 - DCP channel 3 semaphore register"]
    pub ch3sema: CH3SEMA,
    _reserved56: [u8; 0x0c],
    #[doc = "0x1e0 - DCP channel 3 status register"]
    pub ch3stat: CH3STAT,
    #[doc = "0x1e4 - DCP channel 3 status register"]
    pub ch3stat_set: CH3STAT_SET,
    #[doc = "0x1e8 - DCP channel 3 status register"]
    pub ch3stat_clr: CH3STAT_CLR,
    #[doc = "0x1ec - DCP channel 3 status register"]
    pub ch3stat_tog: CH3STAT_TOG,
    #[doc = "0x1f0 - DCP channel 3 options register"]
    pub ch3opts: CH3OPTS,
    #[doc = "0x1f4 - DCP channel 3 options register"]
    pub ch3opts_set: CH3OPTS_SET,
    #[doc = "0x1f8 - DCP channel 3 options register"]
    pub ch3opts_clr: CH3OPTS_CLR,
    #[doc = "0x1fc - DCP channel 3 options register"]
    pub ch3opts_tog: CH3OPTS_TOG,
    _reserved64: [u8; 0x0200],
    #[doc = "0x400 - DCP debug select register"]
    pub dbgselect: DBGSELECT,
    _reserved65: [u8; 0x0c],
    #[doc = "0x410 - DCP debug data register"]
    pub dbgdata: DBGDATA,
    _reserved66: [u8; 0x0c],
    #[doc = "0x420 - DCP page table register"]
    pub pagetable: PAGETABLE,
    _reserved67: [u8; 0x0c],
    #[doc = "0x430 - DCP version register"]
    pub version: VERSION,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DCP control register 0"]
pub mod ctrl;
#[doc = "CTRL_SET (rw) register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "DCP control register 0"]
pub mod ctrl_set;
#[doc = "CTRL_CLR (rw) register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "DCP control register 0"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG (rw) register accessor: an alias for `Reg<CTRL_TOG_SPEC>`"]
pub type CTRL_TOG = crate::Reg<ctrl_tog::CTRL_TOG_SPEC>;
#[doc = "DCP control register 0"]
pub mod ctrl_tog;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "DCP status register"]
pub mod stat;
#[doc = "STAT_SET (rw) register accessor: an alias for `Reg<STAT_SET_SPEC>`"]
pub type STAT_SET = crate::Reg<stat_set::STAT_SET_SPEC>;
#[doc = "DCP status register"]
pub mod stat_set;
#[doc = "STAT_CLR (rw) register accessor: an alias for `Reg<STAT_CLR_SPEC>`"]
pub type STAT_CLR = crate::Reg<stat_clr::STAT_CLR_SPEC>;
#[doc = "DCP status register"]
pub mod stat_clr;
#[doc = "STAT_TOG (rw) register accessor: an alias for `Reg<STAT_TOG_SPEC>`"]
pub type STAT_TOG = crate::Reg<stat_tog::STAT_TOG_SPEC>;
#[doc = "DCP status register"]
pub mod stat_tog;
#[doc = "CHANNELCTRL (rw) register accessor: an alias for `Reg<CHANNELCTRL_SPEC>`"]
pub type CHANNELCTRL = crate::Reg<channelctrl::CHANNELCTRL_SPEC>;
#[doc = "DCP channel control register"]
pub mod channelctrl;
#[doc = "CHANNELCTRL_SET (rw) register accessor: an alias for `Reg<CHANNELCTRL_SET_SPEC>`"]
pub type CHANNELCTRL_SET = crate::Reg<channelctrl_set::CHANNELCTRL_SET_SPEC>;
#[doc = "DCP channel control register"]
pub mod channelctrl_set;
#[doc = "CHANNELCTRL_CLR (rw) register accessor: an alias for `Reg<CHANNELCTRL_CLR_SPEC>`"]
pub type CHANNELCTRL_CLR = crate::Reg<channelctrl_clr::CHANNELCTRL_CLR_SPEC>;
#[doc = "DCP channel control register"]
pub mod channelctrl_clr;
#[doc = "CHANNELCTRL_TOG (rw) register accessor: an alias for `Reg<CHANNELCTRL_TOG_SPEC>`"]
pub type CHANNELCTRL_TOG = crate::Reg<channelctrl_tog::CHANNELCTRL_TOG_SPEC>;
#[doc = "DCP channel control register"]
pub mod channelctrl_tog;
#[doc = "CAPABILITY0 (rw) register accessor: an alias for `Reg<CAPABILITY0_SPEC>`"]
pub type CAPABILITY0 = crate::Reg<capability0::CAPABILITY0_SPEC>;
#[doc = "DCP capability 0 register"]
pub mod capability0;
#[doc = "CAPABILITY1 (r) register accessor: an alias for `Reg<CAPABILITY1_SPEC>`"]
pub type CAPABILITY1 = crate::Reg<capability1::CAPABILITY1_SPEC>;
#[doc = "DCP capability 1 register"]
pub mod capability1;
#[doc = "CONTEXT (rw) register accessor: an alias for `Reg<CONTEXT_SPEC>`"]
pub type CONTEXT = crate::Reg<context::CONTEXT_SPEC>;
#[doc = "DCP context buffer pointer"]
pub mod context;
#[doc = "KEY (rw) register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "DCP key index"]
pub mod key;
#[doc = "KEYDATA (rw) register accessor: an alias for `Reg<KEYDATA_SPEC>`"]
pub type KEYDATA = crate::Reg<keydata::KEYDATA_SPEC>;
#[doc = "DCP key data"]
pub mod keydata;
#[doc = "PACKET0 (r) register accessor: an alias for `Reg<PACKET0_SPEC>`"]
pub type PACKET0 = crate::Reg<packet0::PACKET0_SPEC>;
#[doc = "DCP work packet 0 status register"]
pub mod packet0;
#[doc = "PACKET1 (r) register accessor: an alias for `Reg<PACKET1_SPEC>`"]
pub type PACKET1 = crate::Reg<packet1::PACKET1_SPEC>;
#[doc = "DCP work packet 1 status register"]
pub mod packet1;
#[doc = "PACKET2 (r) register accessor: an alias for `Reg<PACKET2_SPEC>`"]
pub type PACKET2 = crate::Reg<packet2::PACKET2_SPEC>;
#[doc = "DCP work packet 2 status register"]
pub mod packet2;
#[doc = "PACKET3 (r) register accessor: an alias for `Reg<PACKET3_SPEC>`"]
pub type PACKET3 = crate::Reg<packet3::PACKET3_SPEC>;
#[doc = "DCP work packet 3 status register"]
pub mod packet3;
#[doc = "PACKET4 (r) register accessor: an alias for `Reg<PACKET4_SPEC>`"]
pub type PACKET4 = crate::Reg<packet4::PACKET4_SPEC>;
#[doc = "DCP work packet 4 status register"]
pub mod packet4;
#[doc = "PACKET5 (r) register accessor: an alias for `Reg<PACKET5_SPEC>`"]
pub type PACKET5 = crate::Reg<packet5::PACKET5_SPEC>;
#[doc = "DCP work packet 5 status register"]
pub mod packet5;
#[doc = "PACKET6 (r) register accessor: an alias for `Reg<PACKET6_SPEC>`"]
pub type PACKET6 = crate::Reg<packet6::PACKET6_SPEC>;
#[doc = "DCP work packet 6 status register"]
pub mod packet6;
#[doc = "CH0CMDPTR (rw) register accessor: an alias for `Reg<CH0CMDPTR_SPEC>`"]
pub type CH0CMDPTR = crate::Reg<ch0cmdptr::CH0CMDPTR_SPEC>;
#[doc = "DCP channel 0 command pointer address register"]
pub mod ch0cmdptr;
#[doc = "CH0SEMA (rw) register accessor: an alias for `Reg<CH0SEMA_SPEC>`"]
pub type CH0SEMA = crate::Reg<ch0sema::CH0SEMA_SPEC>;
#[doc = "DCP channel 0 semaphore register"]
pub mod ch0sema;
#[doc = "CH0STAT (rw) register accessor: an alias for `Reg<CH0STAT_SPEC>`"]
pub type CH0STAT = crate::Reg<ch0stat::CH0STAT_SPEC>;
#[doc = "DCP channel 0 status register"]
pub mod ch0stat;
#[doc = "CH0STAT_SET (rw) register accessor: an alias for `Reg<CH0STAT_SET_SPEC>`"]
pub type CH0STAT_SET = crate::Reg<ch0stat_set::CH0STAT_SET_SPEC>;
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_set;
#[doc = "CH0STAT_CLR (rw) register accessor: an alias for `Reg<CH0STAT_CLR_SPEC>`"]
pub type CH0STAT_CLR = crate::Reg<ch0stat_clr::CH0STAT_CLR_SPEC>;
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_clr;
#[doc = "CH0STAT_TOG (rw) register accessor: an alias for `Reg<CH0STAT_TOG_SPEC>`"]
pub type CH0STAT_TOG = crate::Reg<ch0stat_tog::CH0STAT_TOG_SPEC>;
#[doc = "DCP channel 0 status register"]
pub mod ch0stat_tog;
#[doc = "CH0OPTS (rw) register accessor: an alias for `Reg<CH0OPTS_SPEC>`"]
pub type CH0OPTS = crate::Reg<ch0opts::CH0OPTS_SPEC>;
#[doc = "DCP channel 0 options register"]
pub mod ch0opts;
#[doc = "CH0OPTS_SET (rw) register accessor: an alias for `Reg<CH0OPTS_SET_SPEC>`"]
pub type CH0OPTS_SET = crate::Reg<ch0opts_set::CH0OPTS_SET_SPEC>;
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_set;
#[doc = "CH0OPTS_CLR (rw) register accessor: an alias for `Reg<CH0OPTS_CLR_SPEC>`"]
pub type CH0OPTS_CLR = crate::Reg<ch0opts_clr::CH0OPTS_CLR_SPEC>;
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_clr;
#[doc = "CH0OPTS_TOG (rw) register accessor: an alias for `Reg<CH0OPTS_TOG_SPEC>`"]
pub type CH0OPTS_TOG = crate::Reg<ch0opts_tog::CH0OPTS_TOG_SPEC>;
#[doc = "DCP channel 0 options register"]
pub mod ch0opts_tog;
#[doc = "CH1CMDPTR (rw) register accessor: an alias for `Reg<CH1CMDPTR_SPEC>`"]
pub type CH1CMDPTR = crate::Reg<ch1cmdptr::CH1CMDPTR_SPEC>;
#[doc = "DCP channel 1 command pointer address register"]
pub mod ch1cmdptr;
#[doc = "CH1SEMA (rw) register accessor: an alias for `Reg<CH1SEMA_SPEC>`"]
pub type CH1SEMA = crate::Reg<ch1sema::CH1SEMA_SPEC>;
#[doc = "DCP channel 1 semaphore register"]
pub mod ch1sema;
#[doc = "CH1STAT (rw) register accessor: an alias for `Reg<CH1STAT_SPEC>`"]
pub type CH1STAT = crate::Reg<ch1stat::CH1STAT_SPEC>;
#[doc = "DCP channel 1 status register"]
pub mod ch1stat;
#[doc = "CH1STAT_SET (rw) register accessor: an alias for `Reg<CH1STAT_SET_SPEC>`"]
pub type CH1STAT_SET = crate::Reg<ch1stat_set::CH1STAT_SET_SPEC>;
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_set;
#[doc = "CH1STAT_CLR (rw) register accessor: an alias for `Reg<CH1STAT_CLR_SPEC>`"]
pub type CH1STAT_CLR = crate::Reg<ch1stat_clr::CH1STAT_CLR_SPEC>;
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_clr;
#[doc = "CH1STAT_TOG (rw) register accessor: an alias for `Reg<CH1STAT_TOG_SPEC>`"]
pub type CH1STAT_TOG = crate::Reg<ch1stat_tog::CH1STAT_TOG_SPEC>;
#[doc = "DCP channel 1 status register"]
pub mod ch1stat_tog;
#[doc = "CH1OPTS (rw) register accessor: an alias for `Reg<CH1OPTS_SPEC>`"]
pub type CH1OPTS = crate::Reg<ch1opts::CH1OPTS_SPEC>;
#[doc = "DCP channel 1 options register"]
pub mod ch1opts;
#[doc = "CH1OPTS_SET (rw) register accessor: an alias for `Reg<CH1OPTS_SET_SPEC>`"]
pub type CH1OPTS_SET = crate::Reg<ch1opts_set::CH1OPTS_SET_SPEC>;
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_set;
#[doc = "CH1OPTS_CLR (rw) register accessor: an alias for `Reg<CH1OPTS_CLR_SPEC>`"]
pub type CH1OPTS_CLR = crate::Reg<ch1opts_clr::CH1OPTS_CLR_SPEC>;
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_clr;
#[doc = "CH1OPTS_TOG (rw) register accessor: an alias for `Reg<CH1OPTS_TOG_SPEC>`"]
pub type CH1OPTS_TOG = crate::Reg<ch1opts_tog::CH1OPTS_TOG_SPEC>;
#[doc = "DCP channel 1 options register"]
pub mod ch1opts_tog;
#[doc = "CH2CMDPTR (rw) register accessor: an alias for `Reg<CH2CMDPTR_SPEC>`"]
pub type CH2CMDPTR = crate::Reg<ch2cmdptr::CH2CMDPTR_SPEC>;
#[doc = "DCP channel 2 command pointer address register"]
pub mod ch2cmdptr;
#[doc = "CH2SEMA (rw) register accessor: an alias for `Reg<CH2SEMA_SPEC>`"]
pub type CH2SEMA = crate::Reg<ch2sema::CH2SEMA_SPEC>;
#[doc = "DCP channel 2 semaphore register"]
pub mod ch2sema;
#[doc = "CH2STAT (rw) register accessor: an alias for `Reg<CH2STAT_SPEC>`"]
pub type CH2STAT = crate::Reg<ch2stat::CH2STAT_SPEC>;
#[doc = "DCP channel 2 status register"]
pub mod ch2stat;
#[doc = "CH2STAT_SET (rw) register accessor: an alias for `Reg<CH2STAT_SET_SPEC>`"]
pub type CH2STAT_SET = crate::Reg<ch2stat_set::CH2STAT_SET_SPEC>;
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_set;
#[doc = "CH2STAT_CLR (rw) register accessor: an alias for `Reg<CH2STAT_CLR_SPEC>`"]
pub type CH2STAT_CLR = crate::Reg<ch2stat_clr::CH2STAT_CLR_SPEC>;
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_clr;
#[doc = "CH2STAT_TOG (rw) register accessor: an alias for `Reg<CH2STAT_TOG_SPEC>`"]
pub type CH2STAT_TOG = crate::Reg<ch2stat_tog::CH2STAT_TOG_SPEC>;
#[doc = "DCP channel 2 status register"]
pub mod ch2stat_tog;
#[doc = "CH2OPTS (rw) register accessor: an alias for `Reg<CH2OPTS_SPEC>`"]
pub type CH2OPTS = crate::Reg<ch2opts::CH2OPTS_SPEC>;
#[doc = "DCP channel 2 options register"]
pub mod ch2opts;
#[doc = "CH2OPTS_SET (rw) register accessor: an alias for `Reg<CH2OPTS_SET_SPEC>`"]
pub type CH2OPTS_SET = crate::Reg<ch2opts_set::CH2OPTS_SET_SPEC>;
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_set;
#[doc = "CH2OPTS_CLR (rw) register accessor: an alias for `Reg<CH2OPTS_CLR_SPEC>`"]
pub type CH2OPTS_CLR = crate::Reg<ch2opts_clr::CH2OPTS_CLR_SPEC>;
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_clr;
#[doc = "CH2OPTS_TOG (rw) register accessor: an alias for `Reg<CH2OPTS_TOG_SPEC>`"]
pub type CH2OPTS_TOG = crate::Reg<ch2opts_tog::CH2OPTS_TOG_SPEC>;
#[doc = "DCP channel 2 options register"]
pub mod ch2opts_tog;
#[doc = "CH3CMDPTR (rw) register accessor: an alias for `Reg<CH3CMDPTR_SPEC>`"]
pub type CH3CMDPTR = crate::Reg<ch3cmdptr::CH3CMDPTR_SPEC>;
#[doc = "DCP channel 3 command pointer address register"]
pub mod ch3cmdptr;
#[doc = "CH3SEMA (rw) register accessor: an alias for `Reg<CH3SEMA_SPEC>`"]
pub type CH3SEMA = crate::Reg<ch3sema::CH3SEMA_SPEC>;
#[doc = "DCP channel 3 semaphore register"]
pub mod ch3sema;
#[doc = "CH3STAT (rw) register accessor: an alias for `Reg<CH3STAT_SPEC>`"]
pub type CH3STAT = crate::Reg<ch3stat::CH3STAT_SPEC>;
#[doc = "DCP channel 3 status register"]
pub mod ch3stat;
#[doc = "CH3STAT_SET (rw) register accessor: an alias for `Reg<CH3STAT_SET_SPEC>`"]
pub type CH3STAT_SET = crate::Reg<ch3stat_set::CH3STAT_SET_SPEC>;
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_set;
#[doc = "CH3STAT_CLR (rw) register accessor: an alias for `Reg<CH3STAT_CLR_SPEC>`"]
pub type CH3STAT_CLR = crate::Reg<ch3stat_clr::CH3STAT_CLR_SPEC>;
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_clr;
#[doc = "CH3STAT_TOG (rw) register accessor: an alias for `Reg<CH3STAT_TOG_SPEC>`"]
pub type CH3STAT_TOG = crate::Reg<ch3stat_tog::CH3STAT_TOG_SPEC>;
#[doc = "DCP channel 3 status register"]
pub mod ch3stat_tog;
#[doc = "CH3OPTS (rw) register accessor: an alias for `Reg<CH3OPTS_SPEC>`"]
pub type CH3OPTS = crate::Reg<ch3opts::CH3OPTS_SPEC>;
#[doc = "DCP channel 3 options register"]
pub mod ch3opts;
#[doc = "CH3OPTS_SET (rw) register accessor: an alias for `Reg<CH3OPTS_SET_SPEC>`"]
pub type CH3OPTS_SET = crate::Reg<ch3opts_set::CH3OPTS_SET_SPEC>;
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_set;
#[doc = "CH3OPTS_CLR (rw) register accessor: an alias for `Reg<CH3OPTS_CLR_SPEC>`"]
pub type CH3OPTS_CLR = crate::Reg<ch3opts_clr::CH3OPTS_CLR_SPEC>;
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_clr;
#[doc = "CH3OPTS_TOG (rw) register accessor: an alias for `Reg<CH3OPTS_TOG_SPEC>`"]
pub type CH3OPTS_TOG = crate::Reg<ch3opts_tog::CH3OPTS_TOG_SPEC>;
#[doc = "DCP channel 3 options register"]
pub mod ch3opts_tog;
#[doc = "DBGSELECT (rw) register accessor: an alias for `Reg<DBGSELECT_SPEC>`"]
pub type DBGSELECT = crate::Reg<dbgselect::DBGSELECT_SPEC>;
#[doc = "DCP debug select register"]
pub mod dbgselect;
#[doc = "DBGDATA (r) register accessor: an alias for `Reg<DBGDATA_SPEC>`"]
pub type DBGDATA = crate::Reg<dbgdata::DBGDATA_SPEC>;
#[doc = "DCP debug data register"]
pub mod dbgdata;
#[doc = "PAGETABLE (rw) register accessor: an alias for `Reg<PAGETABLE_SPEC>`"]
pub type PAGETABLE = crate::Reg<pagetable::PAGETABLE_SPEC>;
#[doc = "DCP page table register"]
pub mod pagetable;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "DCP version register"]
pub mod version;
