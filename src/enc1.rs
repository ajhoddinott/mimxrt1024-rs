#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x02 - Input Filter Register"]
    pub filt: FILT,
    #[doc = "0x04 - Watchdog Timeout Register"]
    pub wtr: WTR,
    #[doc = "0x06 - Position Difference Counter Register"]
    pub posd: POSD,
    #[doc = "0x08 - Position Difference Hold Register"]
    pub posdh: POSDH,
    #[doc = "0x0a - Revolution Counter Register"]
    pub rev: REV,
    #[doc = "0x0c - Revolution Hold Register"]
    pub revh: REVH,
    #[doc = "0x0e - Upper Position Counter Register"]
    pub upos: UPOS,
    #[doc = "0x10 - Lower Position Counter Register"]
    pub lpos: LPOS,
    #[doc = "0x12 - Upper Position Hold Register"]
    pub uposh: UPOSH,
    #[doc = "0x14 - Lower Position Hold Register"]
    pub lposh: LPOSH,
    #[doc = "0x16 - Upper Initialization Register"]
    pub uinit: UINIT,
    #[doc = "0x18 - Lower Initialization Register"]
    pub linit: LINIT,
    #[doc = "0x1a - Input Monitor Register"]
    pub imr: IMR,
    #[doc = "0x1c - Test Register"]
    pub tst: TST,
    #[doc = "0x1e - Control 2 Register"]
    pub ctrl2: CTRL2,
    #[doc = "0x20 - Upper Modulus Register"]
    pub umod: UMOD,
    #[doc = "0x22 - Lower Modulus Register"]
    pub lmod: LMOD,
    #[doc = "0x24 - Upper Position Compare Register"]
    pub ucomp: UCOMP,
    #[doc = "0x26 - Lower Position Compare Register"]
    pub lcomp: LCOMP,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FILT (rw) register accessor: an alias for `Reg<FILT_SPEC>`"]
pub type FILT = crate::Reg<filt::FILT_SPEC>;
#[doc = "Input Filter Register"]
pub mod filt;
#[doc = "WTR (rw) register accessor: an alias for `Reg<WTR_SPEC>`"]
pub type WTR = crate::Reg<wtr::WTR_SPEC>;
#[doc = "Watchdog Timeout Register"]
pub mod wtr;
#[doc = "POSD (rw) register accessor: an alias for `Reg<POSD_SPEC>`"]
pub type POSD = crate::Reg<posd::POSD_SPEC>;
#[doc = "Position Difference Counter Register"]
pub mod posd;
#[doc = "POSDH (r) register accessor: an alias for `Reg<POSDH_SPEC>`"]
pub type POSDH = crate::Reg<posdh::POSDH_SPEC>;
#[doc = "Position Difference Hold Register"]
pub mod posdh;
#[doc = "REV (rw) register accessor: an alias for `Reg<REV_SPEC>`"]
pub type REV = crate::Reg<rev::REV_SPEC>;
#[doc = "Revolution Counter Register"]
pub mod rev;
#[doc = "REVH (r) register accessor: an alias for `Reg<REVH_SPEC>`"]
pub type REVH = crate::Reg<revh::REVH_SPEC>;
#[doc = "Revolution Hold Register"]
pub mod revh;
#[doc = "UPOS (rw) register accessor: an alias for `Reg<UPOS_SPEC>`"]
pub type UPOS = crate::Reg<upos::UPOS_SPEC>;
#[doc = "Upper Position Counter Register"]
pub mod upos;
#[doc = "LPOS (rw) register accessor: an alias for `Reg<LPOS_SPEC>`"]
pub type LPOS = crate::Reg<lpos::LPOS_SPEC>;
#[doc = "Lower Position Counter Register"]
pub mod lpos;
#[doc = "UPOSH (r) register accessor: an alias for `Reg<UPOSH_SPEC>`"]
pub type UPOSH = crate::Reg<uposh::UPOSH_SPEC>;
#[doc = "Upper Position Hold Register"]
pub mod uposh;
#[doc = "LPOSH (r) register accessor: an alias for `Reg<LPOSH_SPEC>`"]
pub type LPOSH = crate::Reg<lposh::LPOSH_SPEC>;
#[doc = "Lower Position Hold Register"]
pub mod lposh;
#[doc = "UINIT (rw) register accessor: an alias for `Reg<UINIT_SPEC>`"]
pub type UINIT = crate::Reg<uinit::UINIT_SPEC>;
#[doc = "Upper Initialization Register"]
pub mod uinit;
#[doc = "LINIT (rw) register accessor: an alias for `Reg<LINIT_SPEC>`"]
pub type LINIT = crate::Reg<linit::LINIT_SPEC>;
#[doc = "Lower Initialization Register"]
pub mod linit;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Input Monitor Register"]
pub mod imr;
#[doc = "TST (rw) register accessor: an alias for `Reg<TST_SPEC>`"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "Test Register"]
pub mod tst;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "UMOD (rw) register accessor: an alias for `Reg<UMOD_SPEC>`"]
pub type UMOD = crate::Reg<umod::UMOD_SPEC>;
#[doc = "Upper Modulus Register"]
pub mod umod;
#[doc = "LMOD (rw) register accessor: an alias for `Reg<LMOD_SPEC>`"]
pub type LMOD = crate::Reg<lmod::LMOD_SPEC>;
#[doc = "Lower Modulus Register"]
pub mod lmod;
#[doc = "UCOMP (rw) register accessor: an alias for `Reg<UCOMP_SPEC>`"]
pub type UCOMP = crate::Reg<ucomp::UCOMP_SPEC>;
#[doc = "Upper Position Compare Register"]
pub mod ucomp;
#[doc = "LCOMP (rw) register accessor: an alias for `Reg<LCOMP_SPEC>`"]
pub type LCOMP = crate::Reg<lcomp::LCOMP_SPEC>;
#[doc = "Lower Position Compare Register"]
pub mod lcomp;
