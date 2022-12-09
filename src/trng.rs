#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Miscellaneous Control Register"]
    pub mctl: MCTL,
    #[doc = "0x04 - Statistical Check Miscellaneous Register"]
    pub scmisc: SCMISC,
    #[doc = "0x08 - Poker Range Register"]
    pub pkrrng: PKRRNG,
    _reserved_3_max_sq: [u8; 0x04],
    #[doc = "0x10 - Seed Control Register"]
    pub sdctl: SDCTL,
    _reserved_5_sblim_totsam: [u8; 0x04],
    #[doc = "0x18 - Frequency Count Minimum Limit Register"]
    pub frqmin: FRQMIN,
    _reserved_7_max_cnt: [u8; 0x04],
    _reserved_8_scml_mc: [u8; 0x04],
    _reserved_9_scr1l_1c_scr: [u8; 0x04],
    _reserved_10_scr2l_2c_scr: [u8; 0x04],
    _reserved_11_scr3l_3c_scr: [u8; 0x04],
    _reserved_12_scr4l_4c_scr: [u8; 0x04],
    _reserved_13_scr5l_5c_scr: [u8; 0x04],
    _reserved_14_scr6pl_pc_scr: [u8; 0x04],
    #[doc = "0x3c - Status Register"]
    pub status: STATUS,
    #[doc = "0x40..0x80 - Entropy Read Register"]
    pub ent: [ENT; 16],
    #[doc = "0x80 - Statistical Check Poker Count 1 and 0 Register"]
    pub pkrcnt10: PKRCNT10,
    #[doc = "0x84 - Statistical Check Poker Count 3 and 2 Register"]
    pub pkrcnt32: PKRCNT32,
    #[doc = "0x88 - Statistical Check Poker Count 5 and 4 Register"]
    pub pkrcnt54: PKRCNT54,
    #[doc = "0x8c - Statistical Check Poker Count 7 and 6 Register"]
    pub pkrcnt76: PKRCNT76,
    #[doc = "0x90 - Statistical Check Poker Count 9 and 8 Register"]
    pub pkrcnt98: PKRCNT98,
    #[doc = "0x94 - Statistical Check Poker Count B and A Register"]
    pub pkrcntba: PKRCNTBA,
    #[doc = "0x98 - Statistical Check Poker Count D and C Register"]
    pub pkrcntdc: PKRCNTDC,
    #[doc = "0x9c - Statistical Check Poker Count F and E Register"]
    pub pkrcntfe: PKRCNTFE,
    #[doc = "0xa0 - Security Configuration Register"]
    pub sec_cfg: SEC_CFG,
    #[doc = "0xa4 - Interrupt Control Register"]
    pub int_ctrl: INT_CTRL,
    #[doc = "0xa8 - Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0xac - Interrupt Status Register"]
    pub int_status: INT_STATUS,
    _reserved29: [u8; 0x40],
    #[doc = "0xf0 - Version ID Register (MS)"]
    pub vid1: VID1,
    #[doc = "0xf4 - Version ID Register (LS)"]
    pub vid2: VID2,
}
impl RegisterBlock {
    #[doc = "0x0c - Poker Square Calculation Result Register"]
    #[inline(always)]
    pub const fn max_sq_pkrsq(&self) -> &MAX_SQ_PKRSQ {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x0c - Poker Maximum Limit Register"]
    #[inline(always)]
    pub const fn max_sq_pkrmax(&self) -> &MAX_SQ_PKRMAX {
        unsafe { &*(self as *const Self).cast::<u8>().add(12usize).cast() }
    }
    #[doc = "0x14 - Total Samples Register"]
    #[inline(always)]
    pub const fn sblim_totsam_totsam(&self) -> &SBLIM_TOTSAM_TOTSAM {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - Sparse Bit Limit Register"]
    #[inline(always)]
    pub const fn sblim_totsam_sblim(&self) -> &SBLIM_TOTSAM_SBLIM {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x1c - Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub const fn max_cnt_frqmax(&self) -> &MAX_CNT_FRQMAX {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Frequency Count Register"]
    #[inline(always)]
    pub const fn max_cnt_frqcnt(&self) -> &MAX_CNT_FRQCNT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub const fn scml_mc_scml(&self) -> &SCML_MC_SCML {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub const fn scml_mc_scmc(&self) -> &SCML_MC_SCMC {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub const fn scr1l_1c_scr1l(&self) -> &SCR1L_1C_SCR1L {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x24 - Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub const fn scr1l_1c_scr1c(&self) -> &SCR1L_1C_SCR1C {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub const fn scr2l_2c_scr2l(&self) -> &SCR2L_2C_SCR2L {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub const fn scr2l_2c_scr2c(&self) -> &SCR2L_2C_SCR2C {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub const fn scr3l_3c_scr3l(&self) -> &SCR3L_3C_SCR3L {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x2c - Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub const fn scr3l_3c_scr3c(&self) -> &SCR3L_3C_SCR3C {
        unsafe { &*(self as *const Self).cast::<u8>().add(44usize).cast() }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub const fn scr4l_4c_scr4l(&self) -> &SCR4L_4C_SCR4L {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub const fn scr4l_4c_scr4c(&self) -> &SCR4L_4C_SCR4C {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub const fn scr5l_5c_scr5l(&self) -> &SCR5L_5C_SCR5L {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub const fn scr5l_5c_scr5c(&self) -> &SCR5L_5C_SCR5C {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub const fn scr6pl_pc_scr6pl(&self) -> &SCR6PL_PC_SCR6PL {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x38 - Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub const fn scr6pl_pc_scr6pc(&self) -> &SCR6PL_PC_SCR6PC {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
}
#[doc = "MCTL (rw) register accessor: an alias for `Reg<MCTL_SPEC>`"]
pub type MCTL = crate::Reg<mctl::MCTL_SPEC>;
#[doc = "Miscellaneous Control Register"]
pub mod mctl;
#[doc = "SCMISC (rw) register accessor: an alias for `Reg<SCMISC_SPEC>`"]
pub type SCMISC = crate::Reg<scmisc::SCMISC_SPEC>;
#[doc = "Statistical Check Miscellaneous Register"]
pub mod scmisc;
#[doc = "PKRRNG (rw) register accessor: an alias for `Reg<PKRRNG_SPEC>`"]
pub type PKRRNG = crate::Reg<pkrrng::PKRRNG_SPEC>;
#[doc = "Poker Range Register"]
pub mod pkrrng;
#[doc = "MAX_SQ_PKRMAX (rw) register accessor: an alias for `Reg<MAX_SQ_PKRMAX_SPEC>`"]
pub type MAX_SQ_PKRMAX = crate::Reg<max_sq_pkrmax::MAX_SQ_PKRMAX_SPEC>;
#[doc = "Poker Maximum Limit Register"]
pub mod max_sq_pkrmax;
#[doc = "MAX_SQ_PKRSQ (r) register accessor: an alias for `Reg<MAX_SQ_PKRSQ_SPEC>`"]
pub type MAX_SQ_PKRSQ = crate::Reg<max_sq_pkrsq::MAX_SQ_PKRSQ_SPEC>;
#[doc = "Poker Square Calculation Result Register"]
pub mod max_sq_pkrsq;
#[doc = "SDCTL (rw) register accessor: an alias for `Reg<SDCTL_SPEC>`"]
pub type SDCTL = crate::Reg<sdctl::SDCTL_SPEC>;
#[doc = "Seed Control Register"]
pub mod sdctl;
#[doc = "SBLIM_TOTSAM_SBLIM (rw) register accessor: an alias for `Reg<SBLIM_TOTSAM_SBLIM_SPEC>`"]
pub type SBLIM_TOTSAM_SBLIM = crate::Reg<sblim_totsam_sblim::SBLIM_TOTSAM_SBLIM_SPEC>;
#[doc = "Sparse Bit Limit Register"]
pub mod sblim_totsam_sblim;
#[doc = "SBLIM_TOTSAM_TOTSAM (r) register accessor: an alias for `Reg<SBLIM_TOTSAM_TOTSAM_SPEC>`"]
pub type SBLIM_TOTSAM_TOTSAM = crate::Reg<sblim_totsam_totsam::SBLIM_TOTSAM_TOTSAM_SPEC>;
#[doc = "Total Samples Register"]
pub mod sblim_totsam_totsam;
#[doc = "FRQMIN (rw) register accessor: an alias for `Reg<FRQMIN_SPEC>`"]
pub type FRQMIN = crate::Reg<frqmin::FRQMIN_SPEC>;
#[doc = "Frequency Count Minimum Limit Register"]
pub mod frqmin;
#[doc = "MAX_CNT_FRQCNT (r) register accessor: an alias for `Reg<MAX_CNT_FRQCNT_SPEC>`"]
pub type MAX_CNT_FRQCNT = crate::Reg<max_cnt_frqcnt::MAX_CNT_FRQCNT_SPEC>;
#[doc = "Frequency Count Register"]
pub mod max_cnt_frqcnt;
#[doc = "MAX_CNT_FRQMAX (rw) register accessor: an alias for `Reg<MAX_CNT_FRQMAX_SPEC>`"]
pub type MAX_CNT_FRQMAX = crate::Reg<max_cnt_frqmax::MAX_CNT_FRQMAX_SPEC>;
#[doc = "Frequency Count Maximum Limit Register"]
pub mod max_cnt_frqmax;
#[doc = "SCML_MC_SCMC (r) register accessor: an alias for `Reg<SCML_MC_SCMC_SPEC>`"]
pub type SCML_MC_SCMC = crate::Reg<scml_mc_scmc::SCML_MC_SCMC_SPEC>;
#[doc = "Statistical Check Monobit Count Register"]
pub mod scml_mc_scmc;
#[doc = "SCML_MC_SCML (rw) register accessor: an alias for `Reg<SCML_MC_SCML_SPEC>`"]
pub type SCML_MC_SCML = crate::Reg<scml_mc_scml::SCML_MC_SCML_SPEC>;
#[doc = "Statistical Check Monobit Limit Register"]
pub mod scml_mc_scml;
#[doc = "SCR1L_1C_SCR1C (r) register accessor: an alias for `Reg<SCR1L_1C_SCR1C_SPEC>`"]
pub type SCR1L_1C_SCR1C = crate::Reg<scr1l_1c_scr1c::SCR1L_1C_SCR1C_SPEC>;
#[doc = "Statistical Check Run Length 1 Count Register"]
pub mod scr1l_1c_scr1c;
#[doc = "SCR1L_1C_SCR1L (rw) register accessor: an alias for `Reg<SCR1L_1C_SCR1L_SPEC>`"]
pub type SCR1L_1C_SCR1L = crate::Reg<scr1l_1c_scr1l::SCR1L_1C_SCR1L_SPEC>;
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod scr1l_1c_scr1l;
#[doc = "SCR2L_2C_SCR2C (r) register accessor: an alias for `Reg<SCR2L_2C_SCR2C_SPEC>`"]
pub type SCR2L_2C_SCR2C = crate::Reg<scr2l_2c_scr2c::SCR2L_2C_SCR2C_SPEC>;
#[doc = "Statistical Check Run Length 2 Count Register"]
pub mod scr2l_2c_scr2c;
#[doc = "SCR2L_2C_SCR2L (rw) register accessor: an alias for `Reg<SCR2L_2C_SCR2L_SPEC>`"]
pub type SCR2L_2C_SCR2L = crate::Reg<scr2l_2c_scr2l::SCR2L_2C_SCR2L_SPEC>;
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod scr2l_2c_scr2l;
#[doc = "SCR3L_3C_SCR3C (r) register accessor: an alias for `Reg<SCR3L_3C_SCR3C_SPEC>`"]
pub type SCR3L_3C_SCR3C = crate::Reg<scr3l_3c_scr3c::SCR3L_3C_SCR3C_SPEC>;
#[doc = "Statistical Check Run Length 3 Count Register"]
pub mod scr3l_3c_scr3c;
#[doc = "SCR3L_3C_SCR3L (rw) register accessor: an alias for `Reg<SCR3L_3C_SCR3L_SPEC>`"]
pub type SCR3L_3C_SCR3L = crate::Reg<scr3l_3c_scr3l::SCR3L_3C_SCR3L_SPEC>;
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod scr3l_3c_scr3l;
#[doc = "SCR4L_4C_SCR4C (r) register accessor: an alias for `Reg<SCR4L_4C_SCR4C_SPEC>`"]
pub type SCR4L_4C_SCR4C = crate::Reg<scr4l_4c_scr4c::SCR4L_4C_SCR4C_SPEC>;
#[doc = "Statistical Check Run Length 4 Count Register"]
pub mod scr4l_4c_scr4c;
#[doc = "SCR4L_4C_SCR4L (rw) register accessor: an alias for `Reg<SCR4L_4C_SCR4L_SPEC>`"]
pub type SCR4L_4C_SCR4L = crate::Reg<scr4l_4c_scr4l::SCR4L_4C_SCR4L_SPEC>;
#[doc = "Statistical Check Run Length 4 Limit Register"]
pub mod scr4l_4c_scr4l;
#[doc = "SCR5L_5C_SCR5C (r) register accessor: an alias for `Reg<SCR5L_5C_SCR5C_SPEC>`"]
pub type SCR5L_5C_SCR5C = crate::Reg<scr5l_5c_scr5c::SCR5L_5C_SCR5C_SPEC>;
#[doc = "Statistical Check Run Length 5 Count Register"]
pub mod scr5l_5c_scr5c;
#[doc = "SCR5L_5C_SCR5L (rw) register accessor: an alias for `Reg<SCR5L_5C_SCR5L_SPEC>`"]
pub type SCR5L_5C_SCR5L = crate::Reg<scr5l_5c_scr5l::SCR5L_5C_SCR5L_SPEC>;
#[doc = "Statistical Check Run Length 5 Limit Register"]
pub mod scr5l_5c_scr5l;
#[doc = "SCR6PL_PC_SCR6PC (r) register accessor: an alias for `Reg<SCR6PL_PC_SCR6PC_SPEC>`"]
pub type SCR6PL_PC_SCR6PC = crate::Reg<scr6pl_pc_scr6pc::SCR6PL_PC_SCR6PC_SPEC>;
#[doc = "Statistical Check Run Length 6+ Count Register"]
pub mod scr6pl_pc_scr6pc;
#[doc = "SCR6PL_PC_SCR6PL (rw) register accessor: an alias for `Reg<SCR6PL_PC_SCR6PL_SPEC>`"]
pub type SCR6PL_PC_SCR6PL = crate::Reg<scr6pl_pc_scr6pl::SCR6PL_PC_SCR6PL_SPEC>;
#[doc = "Statistical Check Run Length 6+ Limit Register"]
pub mod scr6pl_pc_scr6pl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ENT (r) register accessor: an alias for `Reg<ENT_SPEC>`"]
pub type ENT = crate::Reg<ent::ENT_SPEC>;
#[doc = "Entropy Read Register"]
pub mod ent;
#[doc = "PKRCNT10 (r) register accessor: an alias for `Reg<PKRCNT10_SPEC>`"]
pub type PKRCNT10 = crate::Reg<pkrcnt10::PKRCNT10_SPEC>;
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
pub mod pkrcnt10;
#[doc = "PKRCNT32 (r) register accessor: an alias for `Reg<PKRCNT32_SPEC>`"]
pub type PKRCNT32 = crate::Reg<pkrcnt32::PKRCNT32_SPEC>;
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
pub mod pkrcnt32;
#[doc = "PKRCNT54 (r) register accessor: an alias for `Reg<PKRCNT54_SPEC>`"]
pub type PKRCNT54 = crate::Reg<pkrcnt54::PKRCNT54_SPEC>;
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
pub mod pkrcnt54;
#[doc = "PKRCNT76 (r) register accessor: an alias for `Reg<PKRCNT76_SPEC>`"]
pub type PKRCNT76 = crate::Reg<pkrcnt76::PKRCNT76_SPEC>;
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
pub mod pkrcnt76;
#[doc = "PKRCNT98 (r) register accessor: an alias for `Reg<PKRCNT98_SPEC>`"]
pub type PKRCNT98 = crate::Reg<pkrcnt98::PKRCNT98_SPEC>;
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
pub mod pkrcnt98;
#[doc = "PKRCNTBA (r) register accessor: an alias for `Reg<PKRCNTBA_SPEC>`"]
pub type PKRCNTBA = crate::Reg<pkrcntba::PKRCNTBA_SPEC>;
#[doc = "Statistical Check Poker Count B and A Register"]
pub mod pkrcntba;
#[doc = "PKRCNTDC (r) register accessor: an alias for `Reg<PKRCNTDC_SPEC>`"]
pub type PKRCNTDC = crate::Reg<pkrcntdc::PKRCNTDC_SPEC>;
#[doc = "Statistical Check Poker Count D and C Register"]
pub mod pkrcntdc;
#[doc = "PKRCNTFE (r) register accessor: an alias for `Reg<PKRCNTFE_SPEC>`"]
pub type PKRCNTFE = crate::Reg<pkrcntfe::PKRCNTFE_SPEC>;
#[doc = "Statistical Check Poker Count F and E Register"]
pub mod pkrcntfe;
#[doc = "SEC_CFG (rw) register accessor: an alias for `Reg<SEC_CFG_SPEC>`"]
pub type SEC_CFG = crate::Reg<sec_cfg::SEC_CFG_SPEC>;
#[doc = "Security Configuration Register"]
pub mod sec_cfg;
#[doc = "INT_CTRL (rw) register accessor: an alias for `Reg<INT_CTRL_SPEC>`"]
pub type INT_CTRL = crate::Reg<int_ctrl::INT_CTRL_SPEC>;
#[doc = "Interrupt Control Register"]
pub mod int_ctrl;
#[doc = "INT_MASK (rw) register accessor: an alias for `Reg<INT_MASK_SPEC>`"]
pub type INT_MASK = crate::Reg<int_mask::INT_MASK_SPEC>;
#[doc = "Mask Register"]
pub mod int_mask;
#[doc = "INT_STATUS (r) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "VID1 (r) register accessor: an alias for `Reg<VID1_SPEC>`"]
pub type VID1 = crate::Reg<vid1::VID1_SPEC>;
#[doc = "Version ID Register (MS)"]
pub mod vid1;
#[doc = "VID2 (r) register accessor: an alias for `Reg<VID2_SPEC>`"]
pub type VID2 = crate::Reg<vid2::VID2_SPEC>;
#[doc = "Version ID Register (LS)"]
pub mod vid2;
