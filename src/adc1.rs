#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register for hardware triggers"]
    pub hc0: HC0,
    #[doc = "0x04..0x20 - Control register for hardware triggers"]
    pub hc: [HC; 7],
    #[doc = "0x20 - Status register for HW triggers"]
    pub hs: HS,
    #[doc = "0x24 - Data result register for HW triggers"]
    pub r0: R0,
    #[doc = "0x28..0x44 - Data result register for HW triggers"]
    pub r: [R; 7],
    #[doc = "0x44 - Configuration register"]
    pub cfg: CFG,
    #[doc = "0x48 - General control register"]
    pub gc: GC,
    #[doc = "0x4c - General status register"]
    pub gs: GS,
    #[doc = "0x50 - Compare value register"]
    pub cv: CV,
    #[doc = "0x54 - Offset correction value register"]
    pub ofs: OFS,
    #[doc = "0x58 - Calibration value register"]
    pub cal: CAL,
}
impl RegisterBlock {
    #[doc = "0x04 - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc1(&self) -> &HC {
        &self.hc[0]
    }
    #[doc = "0x08 - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc2(&self) -> &HC {
        &self.hc[1]
    }
    #[doc = "0x0c - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc3(&self) -> &HC {
        &self.hc[2]
    }
    #[doc = "0x10 - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc4(&self) -> &HC {
        &self.hc[3]
    }
    #[doc = "0x14 - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc5(&self) -> &HC {
        &self.hc[4]
    }
    #[doc = "0x18 - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc6(&self) -> &HC {
        &self.hc[5]
    }
    #[doc = "0x1c - Control register for hardware triggers"]
    #[inline(always)]
    pub fn hc7(&self) -> &HC {
        &self.hc[6]
    }
    #[doc = "0x28 - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r1(&self) -> &R {
        &self.r[0]
    }
    #[doc = "0x2c - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r2(&self) -> &R {
        &self.r[1]
    }
    #[doc = "0x30 - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r3(&self) -> &R {
        &self.r[2]
    }
    #[doc = "0x34 - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r4(&self) -> &R {
        &self.r[3]
    }
    #[doc = "0x38 - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r5(&self) -> &R {
        &self.r[4]
    }
    #[doc = "0x3c - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r6(&self) -> &R {
        &self.r[5]
    }
    #[doc = "0x40 - Data result register for HW triggers"]
    #[inline(always)]
    pub fn r7(&self) -> &R {
        &self.r[6]
    }
}
#[doc = "HC0 (rw) register accessor: an alias for `Reg<HC0_SPEC>`"]
pub type HC0 = crate::Reg<hc0::HC0_SPEC>;
#[doc = "Control register for hardware triggers"]
pub mod hc0;
#[doc = "HC (rw) register accessor: an alias for `Reg<HC_SPEC>`"]
pub type HC = crate::Reg<hc::HC_SPEC>;
#[doc = "Control register for hardware triggers"]
pub mod hc;
#[doc = "HS (r) register accessor: an alias for `Reg<HS_SPEC>`"]
pub type HS = crate::Reg<hs::HS_SPEC>;
#[doc = "Status register for HW triggers"]
pub mod hs;
#[doc = "R0 (r) register accessor: an alias for `Reg<R0_SPEC>`"]
pub type R0 = crate::Reg<r0::R0_SPEC>;
#[doc = "Data result register for HW triggers"]
pub mod r0;
#[doc = "R (r) register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "Data result register for HW triggers"]
pub mod r;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration register"]
pub mod cfg;
#[doc = "GC (rw) register accessor: an alias for `Reg<GC_SPEC>`"]
pub type GC = crate::Reg<gc::GC_SPEC>;
#[doc = "General control register"]
pub mod gc;
#[doc = "GS (rw) register accessor: an alias for `Reg<GS_SPEC>`"]
pub type GS = crate::Reg<gs::GS_SPEC>;
#[doc = "General status register"]
pub mod gs;
#[doc = "CV (rw) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Compare value register"]
pub mod cv;
#[doc = "OFS (rw) register accessor: an alias for `Reg<OFS_SPEC>`"]
pub type OFS = crate::Reg<ofs::OFS_SPEC>;
#[doc = "Offset correction value register"]
pub mod ofs;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration value register"]
pub mod cal;
