#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1: PLL_USB1,
    #[doc = "0x14 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_set: PLL_USB1_SET,
    #[doc = "0x18 - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_clr: PLL_USB1_CLR,
    #[doc = "0x1c - Analog USB1 480MHz PLL Control Register"]
    pub pll_usb1_tog: PLL_USB1_TOG,
    _reserved4: [u8; 0x10],
    #[doc = "0x30 - Analog System PLL Control Register"]
    pub pll_sys: PLL_SYS,
    #[doc = "0x34 - Analog System PLL Control Register"]
    pub pll_sys_set: PLL_SYS_SET,
    #[doc = "0x38 - Analog System PLL Control Register"]
    pub pll_sys_clr: PLL_SYS_CLR,
    #[doc = "0x3c - Analog System PLL Control Register"]
    pub pll_sys_tog: PLL_SYS_TOG,
    #[doc = "0x40 - 528MHz System PLL Spread Spectrum Register"]
    pub pll_sys_ss: PLL_SYS_SS,
    _reserved9: [u8; 0x0c],
    #[doc = "0x50 - Numerator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_num: PLL_SYS_NUM,
    _reserved10: [u8; 0x0c],
    #[doc = "0x60 - Denominator of 528MHz System PLL Fractional Loop Divider Register"]
    pub pll_sys_denom: PLL_SYS_DENOM,
    _reserved11: [u8; 0x0c],
    #[doc = "0x70 - Analog Audio PLL control Register"]
    pub pll_audio: PLL_AUDIO,
    #[doc = "0x74 - Analog Audio PLL control Register"]
    pub pll_audio_set: PLL_AUDIO_SET,
    #[doc = "0x78 - Analog Audio PLL control Register"]
    pub pll_audio_clr: PLL_AUDIO_CLR,
    #[doc = "0x7c - Analog Audio PLL control Register"]
    pub pll_audio_tog: PLL_AUDIO_TOG,
    #[doc = "0x80 - Numerator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_num: PLL_AUDIO_NUM,
    _reserved16: [u8; 0x0c],
    #[doc = "0x90 - Denominator of Audio PLL Fractional Loop Divider Register"]
    pub pll_audio_denom: PLL_AUDIO_DENOM,
    _reserved17: [u8; 0x4c],
    #[doc = "0xe0 - Analog ENET PLL Control Register"]
    pub pll_enet: PLL_ENET,
    #[doc = "0xe4 - Analog ENET PLL Control Register"]
    pub pll_enet_set: PLL_ENET_SET,
    #[doc = "0xe8 - Analog ENET PLL Control Register"]
    pub pll_enet_clr: PLL_ENET_CLR,
    #[doc = "0xec - Analog ENET PLL Control Register"]
    pub pll_enet_tog: PLL_ENET_TOG,
    #[doc = "0xf0 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480: PFD_480,
    #[doc = "0xf4 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_set: PFD_480_SET,
    #[doc = "0xf8 - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_clr: PFD_480_CLR,
    #[doc = "0xfc - 480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
    pub pfd_480_tog: PFD_480_TOG,
    #[doc = "0x100 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528: PFD_528,
    #[doc = "0x104 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_set: PFD_528_SET,
    #[doc = "0x108 - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_clr: PFD_528_CLR,
    #[doc = "0x10c - 528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
    pub pfd_528_tog: PFD_528_TOG,
    _reserved29: [u8; 0x40],
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
    #[doc = "0x170 - Miscellaneous Register 2"]
    pub misc2: MISC2,
    #[doc = "0x174 - Miscellaneous Register 2"]
    pub misc2_set: MISC2_SET,
    #[doc = "0x178 - Miscellaneous Register 2"]
    pub misc2_clr: MISC2_CLR,
    #[doc = "0x17c - Miscellaneous Register 2"]
    pub misc2_tog: MISC2_TOG,
}
#[doc = "PLL_USB1 (rw) register accessor: an alias for `Reg<PLL_USB1_SPEC>`"]
pub type PLL_USB1 = crate::Reg<pll_usb1::PLL_USB1_SPEC>;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1;
#[doc = "PLL_USB1_SET (rw) register accessor: an alias for `Reg<PLL_USB1_SET_SPEC>`"]
pub type PLL_USB1_SET = crate::Reg<pll_usb1_set::PLL_USB1_SET_SPEC>;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_set;
#[doc = "PLL_USB1_CLR (rw) register accessor: an alias for `Reg<PLL_USB1_CLR_SPEC>`"]
pub type PLL_USB1_CLR = crate::Reg<pll_usb1_clr::PLL_USB1_CLR_SPEC>;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_clr;
#[doc = "PLL_USB1_TOG (rw) register accessor: an alias for `Reg<PLL_USB1_TOG_SPEC>`"]
pub type PLL_USB1_TOG = crate::Reg<pll_usb1_tog::PLL_USB1_TOG_SPEC>;
#[doc = "Analog USB1 480MHz PLL Control Register"]
pub mod pll_usb1_tog;
#[doc = "PLL_SYS (rw) register accessor: an alias for `Reg<PLL_SYS_SPEC>`"]
pub type PLL_SYS = crate::Reg<pll_sys::PLL_SYS_SPEC>;
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys;
#[doc = "PLL_SYS_SET (rw) register accessor: an alias for `Reg<PLL_SYS_SET_SPEC>`"]
pub type PLL_SYS_SET = crate::Reg<pll_sys_set::PLL_SYS_SET_SPEC>;
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_set;
#[doc = "PLL_SYS_CLR (rw) register accessor: an alias for `Reg<PLL_SYS_CLR_SPEC>`"]
pub type PLL_SYS_CLR = crate::Reg<pll_sys_clr::PLL_SYS_CLR_SPEC>;
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_clr;
#[doc = "PLL_SYS_TOG (rw) register accessor: an alias for `Reg<PLL_SYS_TOG_SPEC>`"]
pub type PLL_SYS_TOG = crate::Reg<pll_sys_tog::PLL_SYS_TOG_SPEC>;
#[doc = "Analog System PLL Control Register"]
pub mod pll_sys_tog;
#[doc = "PLL_SYS_SS (rw) register accessor: an alias for `Reg<PLL_SYS_SS_SPEC>`"]
pub type PLL_SYS_SS = crate::Reg<pll_sys_ss::PLL_SYS_SS_SPEC>;
#[doc = "528MHz System PLL Spread Spectrum Register"]
pub mod pll_sys_ss;
#[doc = "PLL_SYS_NUM (rw) register accessor: an alias for `Reg<PLL_SYS_NUM_SPEC>`"]
pub type PLL_SYS_NUM = crate::Reg<pll_sys_num::PLL_SYS_NUM_SPEC>;
#[doc = "Numerator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_num;
#[doc = "PLL_SYS_DENOM (rw) register accessor: an alias for `Reg<PLL_SYS_DENOM_SPEC>`"]
pub type PLL_SYS_DENOM = crate::Reg<pll_sys_denom::PLL_SYS_DENOM_SPEC>;
#[doc = "Denominator of 528MHz System PLL Fractional Loop Divider Register"]
pub mod pll_sys_denom;
#[doc = "PLL_AUDIO (rw) register accessor: an alias for `Reg<PLL_AUDIO_SPEC>`"]
pub type PLL_AUDIO = crate::Reg<pll_audio::PLL_AUDIO_SPEC>;
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio;
#[doc = "PLL_AUDIO_SET (rw) register accessor: an alias for `Reg<PLL_AUDIO_SET_SPEC>`"]
pub type PLL_AUDIO_SET = crate::Reg<pll_audio_set::PLL_AUDIO_SET_SPEC>;
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_set;
#[doc = "PLL_AUDIO_CLR (rw) register accessor: an alias for `Reg<PLL_AUDIO_CLR_SPEC>`"]
pub type PLL_AUDIO_CLR = crate::Reg<pll_audio_clr::PLL_AUDIO_CLR_SPEC>;
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_clr;
#[doc = "PLL_AUDIO_TOG (rw) register accessor: an alias for `Reg<PLL_AUDIO_TOG_SPEC>`"]
pub type PLL_AUDIO_TOG = crate::Reg<pll_audio_tog::PLL_AUDIO_TOG_SPEC>;
#[doc = "Analog Audio PLL control Register"]
pub mod pll_audio_tog;
#[doc = "PLL_AUDIO_NUM (rw) register accessor: an alias for `Reg<PLL_AUDIO_NUM_SPEC>`"]
pub type PLL_AUDIO_NUM = crate::Reg<pll_audio_num::PLL_AUDIO_NUM_SPEC>;
#[doc = "Numerator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_num;
#[doc = "PLL_AUDIO_DENOM (rw) register accessor: an alias for `Reg<PLL_AUDIO_DENOM_SPEC>`"]
pub type PLL_AUDIO_DENOM = crate::Reg<pll_audio_denom::PLL_AUDIO_DENOM_SPEC>;
#[doc = "Denominator of Audio PLL Fractional Loop Divider Register"]
pub mod pll_audio_denom;
#[doc = "PLL_ENET (rw) register accessor: an alias for `Reg<PLL_ENET_SPEC>`"]
pub type PLL_ENET = crate::Reg<pll_enet::PLL_ENET_SPEC>;
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet;
#[doc = "PLL_ENET_SET (rw) register accessor: an alias for `Reg<PLL_ENET_SET_SPEC>`"]
pub type PLL_ENET_SET = crate::Reg<pll_enet_set::PLL_ENET_SET_SPEC>;
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_set;
#[doc = "PLL_ENET_CLR (rw) register accessor: an alias for `Reg<PLL_ENET_CLR_SPEC>`"]
pub type PLL_ENET_CLR = crate::Reg<pll_enet_clr::PLL_ENET_CLR_SPEC>;
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_clr;
#[doc = "PLL_ENET_TOG (rw) register accessor: an alias for `Reg<PLL_ENET_TOG_SPEC>`"]
pub type PLL_ENET_TOG = crate::Reg<pll_enet_tog::PLL_ENET_TOG_SPEC>;
#[doc = "Analog ENET PLL Control Register"]
pub mod pll_enet_tog;
#[doc = "PFD_480 (rw) register accessor: an alias for `Reg<PFD_480_SPEC>`"]
pub type PFD_480 = crate::Reg<pfd_480::PFD_480_SPEC>;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480;
#[doc = "PFD_480_SET (rw) register accessor: an alias for `Reg<PFD_480_SET_SPEC>`"]
pub type PFD_480_SET = crate::Reg<pfd_480_set::PFD_480_SET_SPEC>;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_set;
#[doc = "PFD_480_CLR (rw) register accessor: an alias for `Reg<PFD_480_CLR_SPEC>`"]
pub type PFD_480_CLR = crate::Reg<pfd_480_clr::PFD_480_CLR_SPEC>;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_clr;
#[doc = "PFD_480_TOG (rw) register accessor: an alias for `Reg<PFD_480_TOG_SPEC>`"]
pub type PFD_480_TOG = crate::Reg<pfd_480_tog::PFD_480_TOG_SPEC>;
#[doc = "480MHz Clock (PLL3) Phase Fractional Divider Control Register"]
pub mod pfd_480_tog;
#[doc = "PFD_528 (rw) register accessor: an alias for `Reg<PFD_528_SPEC>`"]
pub type PFD_528 = crate::Reg<pfd_528::PFD_528_SPEC>;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528;
#[doc = "PFD_528_SET (rw) register accessor: an alias for `Reg<PFD_528_SET_SPEC>`"]
pub type PFD_528_SET = crate::Reg<pfd_528_set::PFD_528_SET_SPEC>;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_set;
#[doc = "PFD_528_CLR (rw) register accessor: an alias for `Reg<PFD_528_CLR_SPEC>`"]
pub type PFD_528_CLR = crate::Reg<pfd_528_clr::PFD_528_CLR_SPEC>;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_clr;
#[doc = "PFD_528_TOG (rw) register accessor: an alias for `Reg<PFD_528_TOG_SPEC>`"]
pub type PFD_528_TOG = crate::Reg<pfd_528_tog::PFD_528_TOG_SPEC>;
#[doc = "528MHz Clock (PLL2) Phase Fractional Divider Control Register"]
pub mod pfd_528_tog;
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
#[doc = "Miscellaneous Register 2"]
pub mod misc2;
#[doc = "MISC2_SET (rw) register accessor: an alias for `Reg<MISC2_SET_SPEC>`"]
pub type MISC2_SET = crate::Reg<misc2_set::MISC2_SET_SPEC>;
#[doc = "Miscellaneous Register 2"]
pub mod misc2_set;
#[doc = "MISC2_CLR (rw) register accessor: an alias for `Reg<MISC2_CLR_SPEC>`"]
pub type MISC2_CLR = crate::Reg<misc2_clr::MISC2_CLR_SPEC>;
#[doc = "Miscellaneous Register 2"]
pub mod misc2_clr;
#[doc = "MISC2_TOG (rw) register accessor: an alias for `Reg<MISC2_TOG_SPEC>`"]
pub type MISC2_TOG = crate::Reg<misc2_tog::MISC2_TOG_SPEC>;
#[doc = "Miscellaneous Register 2"]
pub mod misc2_tog;
