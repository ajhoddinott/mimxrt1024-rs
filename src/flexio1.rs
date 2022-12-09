#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"]
    pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: TIMSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: TIMIEN,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: SHIFTSDEN,
    _reserved11: [u8; 0x0c],
    #[doc = "0x40 - Shifter State Register"]
    pub shiftstate: SHIFTSTATE,
    _reserved12: [u8; 0x3c],
    #[doc = "0x80..0xa0 - Shifter Control N Register"]
    pub shiftctl: [SHIFTCTL; 8],
    _reserved13: [u8; 0x60],
    #[doc = "0x100..0x120 - Shifter Configuration N Register"]
    pub shiftcfg: [SHIFTCFG; 8],
    _reserved14: [u8; 0xe0],
    #[doc = "0x200..0x220 - Shifter Buffer N Register"]
    pub shiftbuf: [SHIFTBUF; 8],
    _reserved15: [u8; 0x60],
    #[doc = "0x280..0x2a0 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis: [SHIFTBUFBIS; 8],
    _reserved16: [u8; 0x60],
    #[doc = "0x300..0x320 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys: [SHIFTBUFBYS; 8],
    _reserved17: [u8; 0x60],
    #[doc = "0x380..0x3a0 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs: [SHIFTBUFBBS; 8],
    _reserved18: [u8; 0x60],
    #[doc = "0x400..0x420 - Timer Control N Register"]
    pub timctl: [TIMCTL; 8],
    _reserved19: [u8; 0x60],
    #[doc = "0x480..0x4a0 - Timer Configuration N Register"]
    pub timcfg: [TIMCFG; 8],
    _reserved20: [u8; 0x60],
    #[doc = "0x500..0x520 - Timer Compare N Register"]
    pub timcmp: [TIMCMP; 8],
    _reserved21: [u8; 0x0160],
    #[doc = "0x680..0x6a0 - Shifter Buffer N Nibble Byte Swapped Register"]
    pub shiftbufnbs: [SHIFTBUFNBS; 8],
    _reserved22: [u8; 0x60],
    #[doc = "0x700..0x720 - Shifter Buffer N Half Word Swapped Register"]
    pub shiftbufhws: [SHIFTBUFHWS; 8],
    _reserved23: [u8; 0x60],
    #[doc = "0x780..0x7a0 - Shifter Buffer N Nibble Swapped Register"]
    pub shiftbufnis: [SHIFTBUFNIS; 8],
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "PIN (r) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "SHIFTSTAT (rw) register accessor: an alias for `Reg<SHIFTSTAT_SPEC>`"]
pub type SHIFTSTAT = crate::Reg<shiftstat::SHIFTSTAT_SPEC>;
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "SHIFTERR (rw) register accessor: an alias for `Reg<SHIFTERR_SPEC>`"]
pub type SHIFTERR = crate::Reg<shifterr::SHIFTERR_SPEC>;
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "TIMSTAT (rw) register accessor: an alias for `Reg<TIMSTAT_SPEC>`"]
pub type TIMSTAT = crate::Reg<timstat::TIMSTAT_SPEC>;
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "SHIFTSIEN (rw) register accessor: an alias for `Reg<SHIFTSIEN_SPEC>`"]
pub type SHIFTSIEN = crate::Reg<shiftsien::SHIFTSIEN_SPEC>;
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "SHIFTEIEN (rw) register accessor: an alias for `Reg<SHIFTEIEN_SPEC>`"]
pub type SHIFTEIEN = crate::Reg<shifteien::SHIFTEIEN_SPEC>;
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "TIMIEN (rw) register accessor: an alias for `Reg<TIMIEN_SPEC>`"]
pub type TIMIEN = crate::Reg<timien::TIMIEN_SPEC>;
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "SHIFTSDEN (rw) register accessor: an alias for `Reg<SHIFTSDEN_SPEC>`"]
pub type SHIFTSDEN = crate::Reg<shiftsden::SHIFTSDEN_SPEC>;
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "SHIFTSTATE (rw) register accessor: an alias for `Reg<SHIFTSTATE_SPEC>`"]
pub type SHIFTSTATE = crate::Reg<shiftstate::SHIFTSTATE_SPEC>;
#[doc = "Shifter State Register"]
pub mod shiftstate;
#[doc = "SHIFTCTL (rw) register accessor: an alias for `Reg<SHIFTCTL_SPEC>`"]
pub type SHIFTCTL = crate::Reg<shiftctl::SHIFTCTL_SPEC>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "SHIFTCFG (rw) register accessor: an alias for `Reg<SHIFTCFG_SPEC>`"]
pub type SHIFTCFG = crate::Reg<shiftcfg::SHIFTCFG_SPEC>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "SHIFTBUF (rw) register accessor: an alias for `Reg<SHIFTBUF_SPEC>`"]
pub type SHIFTBUF = crate::Reg<shiftbuf::SHIFTBUF_SPEC>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "SHIFTBUFBIS (rw) register accessor: an alias for `Reg<SHIFTBUFBIS_SPEC>`"]
pub type SHIFTBUFBIS = crate::Reg<shiftbufbis::SHIFTBUFBIS_SPEC>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "SHIFTBUFBYS (rw) register accessor: an alias for `Reg<SHIFTBUFBYS_SPEC>`"]
pub type SHIFTBUFBYS = crate::Reg<shiftbufbys::SHIFTBUFBYS_SPEC>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "SHIFTBUFBBS (rw) register accessor: an alias for `Reg<SHIFTBUFBBS_SPEC>`"]
pub type SHIFTBUFBBS = crate::Reg<shiftbufbbs::SHIFTBUFBBS_SPEC>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "TIMCTL (rw) register accessor: an alias for `Reg<TIMCTL_SPEC>`"]
pub type TIMCTL = crate::Reg<timctl::TIMCTL_SPEC>;
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "TIMCFG (rw) register accessor: an alias for `Reg<TIMCFG_SPEC>`"]
pub type TIMCFG = crate::Reg<timcfg::TIMCFG_SPEC>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "TIMCMP (rw) register accessor: an alias for `Reg<TIMCMP_SPEC>`"]
pub type TIMCMP = crate::Reg<timcmp::TIMCMP_SPEC>;
#[doc = "Timer Compare N Register"]
pub mod timcmp;
#[doc = "SHIFTBUFNBS (rw) register accessor: an alias for `Reg<SHIFTBUFNBS_SPEC>`"]
pub type SHIFTBUFNBS = crate::Reg<shiftbufnbs::SHIFTBUFNBS_SPEC>;
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub mod shiftbufnbs;
#[doc = "SHIFTBUFHWS (rw) register accessor: an alias for `Reg<SHIFTBUFHWS_SPEC>`"]
pub type SHIFTBUFHWS = crate::Reg<shiftbufhws::SHIFTBUFHWS_SPEC>;
#[doc = "Shifter Buffer N Half Word Swapped Register"]
pub mod shiftbufhws;
#[doc = "SHIFTBUFNIS (rw) register accessor: an alias for `Reg<SHIFTBUFNIS_SPEC>`"]
pub type SHIFTBUFNIS = crate::Reg<shiftbufnis::SHIFTBUFNIS_SPEC>;
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub mod shiftbufnis;
