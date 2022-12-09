#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identification register"]
    pub id: ID,
    #[doc = "0x04 - Hardware General"]
    pub hwgeneral: HWGENERAL,
    #[doc = "0x08 - Host Hardware Parameters"]
    pub hwhost: HWHOST,
    #[doc = "0x0c - Device Hardware Parameters"]
    pub hwdevice: HWDEVICE,
    #[doc = "0x10 - TX Buffer Hardware Parameters"]
    pub hwtxbuf: HWTXBUF,
    #[doc = "0x14 - RX Buffer Hardware Parameters"]
    pub hwrxbuf: HWRXBUF,
    _reserved6: [u8; 0x68],
    #[doc = "0x80 - General Purpose Timer #0 Load"]
    pub gptimer0ld: GPTIMER0LD,
    #[doc = "0x84 - General Purpose Timer #0 Controller"]
    pub gptimer0ctrl: GPTIMER0CTRL,
    #[doc = "0x88 - General Purpose Timer #1 Load"]
    pub gptimer1ld: GPTIMER1LD,
    #[doc = "0x8c - General Purpose Timer #1 Controller"]
    pub gptimer1ctrl: GPTIMER1CTRL,
    #[doc = "0x90 - System Bus Config"]
    pub sbuscfg: SBUSCFG,
    _reserved11: [u8; 0x6c],
    #[doc = "0x100 - Capability Registers Length"]
    pub caplength: CAPLENGTH,
    _reserved12: [u8; 0x01],
    #[doc = "0x102 - Host Controller Interface Version"]
    pub hciversion: HCIVERSION,
    #[doc = "0x104 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host Controller Capability Parameters"]
    pub hccparams: HCCPARAMS,
    _reserved15: [u8; 0x14],
    #[doc = "0x120 - Device Controller Interface Version"]
    pub dciversion: DCIVERSION,
    _reserved16: [u8; 0x02],
    #[doc = "0x124 - Device Controller Capability Parameters"]
    pub dccparams: DCCPARAMS,
    _reserved17: [u8; 0x18],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - USB Frame Index"]
    pub frindex: FRINDEX,
    _reserved21: [u8; 0x04],
    _reserved_21_deviceaddr_periodiclistbase: [u8; 0x04],
    _reserved_22_asynclistaddr_endptlistaddr: [u8; 0x04],
    _reserved23: [u8; 0x04],
    #[doc = "0x160 - Programmable Burst Size"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - TX FIFO Fill Tuning"]
    pub txfilltuning: TXFILLTUNING,
    _reserved25: [u8; 0x10],
    #[doc = "0x178 - Endpoint NAK"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable"]
    pub endptnaken: ENDPTNAKEN,
    #[doc = "0x180 - Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x184 - Port Status & Control"]
    pub portsc1: PORTSC1,
    _reserved29: [u8; 0x1c],
    #[doc = "0x1a4 - On-The-Go Status & control"]
    pub otgsc: OTGSC,
    #[doc = "0x1a8 - USB Device Mode"]
    pub usbmode: USBMODE,
    #[doc = "0x1ac - Endpoint Setup Status"]
    pub endptsetupstat: ENDPTSETUPSTAT,
    #[doc = "0x1b0 - Endpoint Prime"]
    pub endptprime: ENDPTPRIME,
    #[doc = "0x1b4 - Endpoint Flush"]
    pub endptflush: ENDPTFLUSH,
    #[doc = "0x1b8 - Endpoint Status"]
    pub endptstat: ENDPTSTAT,
    #[doc = "0x1bc - Endpoint Complete"]
    pub endptcomplete: ENDPTCOMPLETE,
    #[doc = "0x1c0 - Endpoint Control0"]
    pub endptctrl0: ENDPTCTRL0,
    #[doc = "0x1c4 - Endpoint Control 1"]
    pub endptctrl1: ENDPTCTRL1,
    #[doc = "0x1c8 - Endpoint Control 2"]
    pub endptctrl2: ENDPTCTRL2,
    #[doc = "0x1cc - Endpoint Control 3"]
    pub endptctrl3: ENDPTCTRL3,
    #[doc = "0x1d0 - Endpoint Control 4"]
    pub endptctrl4: ENDPTCTRL4,
    #[doc = "0x1d4 - Endpoint Control 5"]
    pub endptctrl5: ENDPTCTRL5,
    #[doc = "0x1d8 - Endpoint Control 6"]
    pub endptctrl6: ENDPTCTRL6,
    #[doc = "0x1dc - Endpoint Control 7"]
    pub endptctrl7: ENDPTCTRL7,
}
impl RegisterBlock {
    #[doc = "0x154 - Frame List Base Address"]
    #[inline(always)]
    pub const fn deviceaddr_periodiclistbase_periodiclistbase(
        &self,
    ) -> &DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE {
        unsafe { &*(self as *const Self).cast::<u8>().add(340usize).cast() }
    }
    #[doc = "0x154 - Device Address"]
    #[inline(always)]
    pub const fn deviceaddr_periodiclistbase_deviceaddr(
        &self,
    ) -> &DEVICEADDR_PERIODICLISTBASE_DEVICEADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(340usize).cast() }
    }
    #[doc = "0x158 - Endpoint List Address"]
    #[inline(always)]
    pub const fn asynclistaddr_endptlistaddr_endptlistaddr(
        &self,
    ) -> &ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
    #[doc = "0x158 - Next Asynch. Address"]
    #[inline(always)]
    pub const fn asynclistaddr_endptlistaddr_asynclistaddr(
        &self,
    ) -> &ASYNCLISTADDR_ENDPTLISTADDR_ASYNCLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Identification register"]
pub mod id;
#[doc = "HWGENERAL (r) register accessor: an alias for `Reg<HWGENERAL_SPEC>`"]
pub type HWGENERAL = crate::Reg<hwgeneral::HWGENERAL_SPEC>;
#[doc = "Hardware General"]
pub mod hwgeneral;
#[doc = "HWHOST (r) register accessor: an alias for `Reg<HWHOST_SPEC>`"]
pub type HWHOST = crate::Reg<hwhost::HWHOST_SPEC>;
#[doc = "Host Hardware Parameters"]
pub mod hwhost;
#[doc = "HWDEVICE (r) register accessor: an alias for `Reg<HWDEVICE_SPEC>`"]
pub type HWDEVICE = crate::Reg<hwdevice::HWDEVICE_SPEC>;
#[doc = "Device Hardware Parameters"]
pub mod hwdevice;
#[doc = "HWTXBUF (r) register accessor: an alias for `Reg<HWTXBUF_SPEC>`"]
pub type HWTXBUF = crate::Reg<hwtxbuf::HWTXBUF_SPEC>;
#[doc = "TX Buffer Hardware Parameters"]
pub mod hwtxbuf;
#[doc = "HWRXBUF (r) register accessor: an alias for `Reg<HWRXBUF_SPEC>`"]
pub type HWRXBUF = crate::Reg<hwrxbuf::HWRXBUF_SPEC>;
#[doc = "RX Buffer Hardware Parameters"]
pub mod hwrxbuf;
#[doc = "GPTIMER0LD (rw) register accessor: an alias for `Reg<GPTIMER0LD_SPEC>`"]
pub type GPTIMER0LD = crate::Reg<gptimer0ld::GPTIMER0LD_SPEC>;
#[doc = "General Purpose Timer #0 Load"]
pub mod gptimer0ld;
#[doc = "GPTIMER0CTRL (rw) register accessor: an alias for `Reg<GPTIMER0CTRL_SPEC>`"]
pub type GPTIMER0CTRL = crate::Reg<gptimer0ctrl::GPTIMER0CTRL_SPEC>;
#[doc = "General Purpose Timer #0 Controller"]
pub mod gptimer0ctrl;
#[doc = "GPTIMER1LD (rw) register accessor: an alias for `Reg<GPTIMER1LD_SPEC>`"]
pub type GPTIMER1LD = crate::Reg<gptimer1ld::GPTIMER1LD_SPEC>;
#[doc = "General Purpose Timer #1 Load"]
pub mod gptimer1ld;
#[doc = "GPTIMER1CTRL (rw) register accessor: an alias for `Reg<GPTIMER1CTRL_SPEC>`"]
pub type GPTIMER1CTRL = crate::Reg<gptimer1ctrl::GPTIMER1CTRL_SPEC>;
#[doc = "General Purpose Timer #1 Controller"]
pub mod gptimer1ctrl;
#[doc = "SBUSCFG (rw) register accessor: an alias for `Reg<SBUSCFG_SPEC>`"]
pub type SBUSCFG = crate::Reg<sbuscfg::SBUSCFG_SPEC>;
#[doc = "System Bus Config"]
pub mod sbuscfg;
#[doc = "CAPLENGTH (r) register accessor: an alias for `Reg<CAPLENGTH_SPEC>`"]
pub type CAPLENGTH = crate::Reg<caplength::CAPLENGTH_SPEC>;
#[doc = "Capability Registers Length"]
pub mod caplength;
#[doc = "HCIVERSION (r) register accessor: an alias for `Reg<HCIVERSION_SPEC>`"]
pub type HCIVERSION = crate::Reg<hciversion::HCIVERSION_SPEC>;
#[doc = "Host Controller Interface Version"]
pub mod hciversion;
#[doc = "HCSPARAMS (r) register accessor: an alias for `Reg<HCSPARAMS_SPEC>`"]
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "HCCPARAMS (r) register accessor: an alias for `Reg<HCCPARAMS_SPEC>`"]
pub type HCCPARAMS = crate::Reg<hccparams::HCCPARAMS_SPEC>;
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "DCIVERSION (r) register accessor: an alias for `Reg<DCIVERSION_SPEC>`"]
pub type DCIVERSION = crate::Reg<dciversion::DCIVERSION_SPEC>;
#[doc = "Device Controller Interface Version"]
pub mod dciversion;
#[doc = "DCCPARAMS (r) register accessor: an alias for `Reg<DCCPARAMS_SPEC>`"]
pub type DCCPARAMS = crate::Reg<dccparams::DCCPARAMS_SPEC>;
#[doc = "Device Controller Capability Parameters"]
pub mod dccparams;
#[doc = "USBCMD (rw) register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod usbintr;
#[doc = "FRINDEX (rw) register accessor: an alias for `Reg<FRINDEX_SPEC>`"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "USB Frame Index"]
pub mod frindex;
#[doc = "DEVICEADDR_PERIODICLISTBASE_DEVICEADDR (rw) register accessor: an alias for `Reg<DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>`"]
pub type DEVICEADDR_PERIODICLISTBASE_DEVICEADDR =
    crate::Reg<deviceaddr_periodiclistbase_deviceaddr::DEVICEADDR_PERIODICLISTBASE_DEVICEADDR_SPEC>;
#[doc = "Device Address"]
pub mod deviceaddr_periodiclistbase_deviceaddr;
#[doc = "DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE (rw) register accessor: an alias for `Reg<DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC>`"]
pub type DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE = crate::Reg<
    deviceaddr_periodiclistbase_periodiclistbase::DEVICEADDR_PERIODICLISTBASE_PERIODICLISTBASE_SPEC,
>;
#[doc = "Frame List Base Address"]
pub mod deviceaddr_periodiclistbase_periodiclistbase;
#[doc = "ASYNCLISTADDR_ENDPTLISTADDR_ASYNCLISTADDR (rw) register accessor: an alias for `Reg<ASYNCLISTADDR_ENDPTLISTADDR_ASYNCLISTADDR_SPEC>`"]
pub type ASYNCLISTADDR_ENDPTLISTADDR_ASYNCLISTADDR = crate::Reg<
    asynclistaddr_endptlistaddr_asynclistaddr::ASYNCLISTADDR_ENDPTLISTADDR_ASYNCLISTADDR_SPEC,
>;
#[doc = "Next Asynch. Address"]
pub mod asynclistaddr_endptlistaddr_asynclistaddr;
#[doc = "ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR (rw) register accessor: an alias for `Reg<ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC>`"]
pub type ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR = crate::Reg<
    asynclistaddr_endptlistaddr_endptlistaddr::ASYNCLISTADDR_ENDPTLISTADDR_ENDPTLISTADDR_SPEC,
>;
#[doc = "Endpoint List Address"]
pub mod asynclistaddr_endptlistaddr_endptlistaddr;
#[doc = "BURSTSIZE (rw) register accessor: an alias for `Reg<BURSTSIZE_SPEC>`"]
pub type BURSTSIZE = crate::Reg<burstsize::BURSTSIZE_SPEC>;
#[doc = "Programmable Burst Size"]
pub mod burstsize;
#[doc = "TXFILLTUNING (rw) register accessor: an alias for `Reg<TXFILLTUNING_SPEC>`"]
pub type TXFILLTUNING = crate::Reg<txfilltuning::TXFILLTUNING_SPEC>;
#[doc = "TX FIFO Fill Tuning"]
pub mod txfilltuning;
#[doc = "ENDPTNAK (rw) register accessor: an alias for `Reg<ENDPTNAK_SPEC>`"]
pub type ENDPTNAK = crate::Reg<endptnak::ENDPTNAK_SPEC>;
#[doc = "Endpoint NAK"]
pub mod endptnak;
#[doc = "ENDPTNAKEN (rw) register accessor: an alias for `Reg<ENDPTNAKEN_SPEC>`"]
pub type ENDPTNAKEN = crate::Reg<endptnaken::ENDPTNAKEN_SPEC>;
#[doc = "Endpoint NAK Enable"]
pub mod endptnaken;
#[doc = "CONFIGFLAG (r) register accessor: an alias for `Reg<CONFIGFLAG_SPEC>`"]
pub type CONFIGFLAG = crate::Reg<configflag::CONFIGFLAG_SPEC>;
#[doc = "Configure Flag Register"]
pub mod configflag;
#[doc = "PORTSC1 (rw) register accessor: an alias for `Reg<PORTSC1_SPEC>`"]
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
#[doc = "Port Status & Control"]
pub mod portsc1;
#[doc = "OTGSC (rw) register accessor: an alias for `Reg<OTGSC_SPEC>`"]
pub type OTGSC = crate::Reg<otgsc::OTGSC_SPEC>;
#[doc = "On-The-Go Status & control"]
pub mod otgsc;
#[doc = "USBMODE (rw) register accessor: an alias for `Reg<USBMODE_SPEC>`"]
pub type USBMODE = crate::Reg<usbmode::USBMODE_SPEC>;
#[doc = "USB Device Mode"]
pub mod usbmode;
#[doc = "ENDPTSETUPSTAT (rw) register accessor: an alias for `Reg<ENDPTSETUPSTAT_SPEC>`"]
pub type ENDPTSETUPSTAT = crate::Reg<endptsetupstat::ENDPTSETUPSTAT_SPEC>;
#[doc = "Endpoint Setup Status"]
pub mod endptsetupstat;
#[doc = "ENDPTPRIME (rw) register accessor: an alias for `Reg<ENDPTPRIME_SPEC>`"]
pub type ENDPTPRIME = crate::Reg<endptprime::ENDPTPRIME_SPEC>;
#[doc = "Endpoint Prime"]
pub mod endptprime;
#[doc = "ENDPTFLUSH (rw) register accessor: an alias for `Reg<ENDPTFLUSH_SPEC>`"]
pub type ENDPTFLUSH = crate::Reg<endptflush::ENDPTFLUSH_SPEC>;
#[doc = "Endpoint Flush"]
pub mod endptflush;
#[doc = "ENDPTSTAT (r) register accessor: an alias for `Reg<ENDPTSTAT_SPEC>`"]
pub type ENDPTSTAT = crate::Reg<endptstat::ENDPTSTAT_SPEC>;
#[doc = "Endpoint Status"]
pub mod endptstat;
#[doc = "ENDPTCOMPLETE (rw) register accessor: an alias for `Reg<ENDPTCOMPLETE_SPEC>`"]
pub type ENDPTCOMPLETE = crate::Reg<endptcomplete::ENDPTCOMPLETE_SPEC>;
#[doc = "Endpoint Complete"]
pub mod endptcomplete;
#[doc = "ENDPTCTRL0 (rw) register accessor: an alias for `Reg<ENDPTCTRL0_SPEC>`"]
pub type ENDPTCTRL0 = crate::Reg<endptctrl0::ENDPTCTRL0_SPEC>;
#[doc = "Endpoint Control0"]
pub mod endptctrl0;
#[doc = "ENDPTCTRL1 (rw) register accessor: an alias for `Reg<ENDPTCTRL1_SPEC>`"]
pub type ENDPTCTRL1 = crate::Reg<endptctrl1::ENDPTCTRL1_SPEC>;
#[doc = "Endpoint Control 1"]
pub mod endptctrl1;
#[doc = "ENDPTCTRL2 (rw) register accessor: an alias for `Reg<ENDPTCTRL2_SPEC>`"]
pub type ENDPTCTRL2 = crate::Reg<endptctrl2::ENDPTCTRL2_SPEC>;
#[doc = "Endpoint Control 2"]
pub mod endptctrl2;
#[doc = "ENDPTCTRL3 (rw) register accessor: an alias for `Reg<ENDPTCTRL3_SPEC>`"]
pub type ENDPTCTRL3 = crate::Reg<endptctrl3::ENDPTCTRL3_SPEC>;
#[doc = "Endpoint Control 3"]
pub mod endptctrl3;
#[doc = "ENDPTCTRL4 (rw) register accessor: an alias for `Reg<ENDPTCTRL4_SPEC>`"]
pub type ENDPTCTRL4 = crate::Reg<endptctrl4::ENDPTCTRL4_SPEC>;
#[doc = "Endpoint Control 4"]
pub mod endptctrl4;
#[doc = "ENDPTCTRL5 (rw) register accessor: an alias for `Reg<ENDPTCTRL5_SPEC>`"]
pub type ENDPTCTRL5 = crate::Reg<endptctrl5::ENDPTCTRL5_SPEC>;
#[doc = "Endpoint Control 5"]
pub mod endptctrl5;
#[doc = "ENDPTCTRL6 (rw) register accessor: an alias for `Reg<ENDPTCTRL6_SPEC>`"]
pub type ENDPTCTRL6 = crate::Reg<endptctrl6::ENDPTCTRL6_SPEC>;
#[doc = "Endpoint Control 6"]
pub mod endptctrl6;
#[doc = "ENDPTCTRL7 (rw) register accessor: an alias for `Reg<ENDPTCTRL7_SPEC>`"]
pub type ENDPTCTRL7 = crate::Reg<endptctrl7::ENDPTCTRL7_SPEC>;
#[doc = "Endpoint Control 7"]
pub mod endptctrl7;
