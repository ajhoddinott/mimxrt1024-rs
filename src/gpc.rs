#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPC Interface control register"]
    pub cntr: CNTR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - IRQ masking register 1"]
    pub imr1: IMR1,
    #[doc = "0x0c - IRQ masking register 2"]
    pub imr2: IMR2,
    #[doc = "0x10 - IRQ masking register 3"]
    pub imr3: IMR3,
    #[doc = "0x14 - IRQ masking register 4"]
    pub imr4: IMR4,
    #[doc = "0x18 - IRQ status resister 1"]
    pub isr1: ISR1,
    #[doc = "0x1c - IRQ status resister 2"]
    pub isr2: ISR2,
    #[doc = "0x20 - IRQ status resister 3"]
    pub isr3: ISR3,
    #[doc = "0x24 - IRQ status resister 4"]
    pub isr4: ISR4,
    _reserved9: [u8; 0x0c],
    #[doc = "0x34 - IRQ masking register 5"]
    pub imr5: IMR5,
    #[doc = "0x38 - IRQ status resister 5"]
    pub isr5: ISR5,
}
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "GPC Interface control register"]
pub mod cntr;
#[doc = "IMR1 (rw) register accessor: an alias for `Reg<IMR1_SPEC>`"]
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
#[doc = "IRQ masking register 1"]
pub mod imr1;
#[doc = "IMR2 (rw) register accessor: an alias for `Reg<IMR2_SPEC>`"]
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
#[doc = "IRQ masking register 2"]
pub mod imr2;
#[doc = "IMR3 (rw) register accessor: an alias for `Reg<IMR3_SPEC>`"]
pub type IMR3 = crate::Reg<imr3::IMR3_SPEC>;
#[doc = "IRQ masking register 3"]
pub mod imr3;
#[doc = "IMR4 (rw) register accessor: an alias for `Reg<IMR4_SPEC>`"]
pub type IMR4 = crate::Reg<imr4::IMR4_SPEC>;
#[doc = "IRQ masking register 4"]
pub mod imr4;
#[doc = "ISR1 (r) register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "IRQ status resister 1"]
pub mod isr1;
#[doc = "ISR2 (r) register accessor: an alias for `Reg<ISR2_SPEC>`"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "IRQ status resister 2"]
pub mod isr2;
#[doc = "ISR3 (r) register accessor: an alias for `Reg<ISR3_SPEC>`"]
pub type ISR3 = crate::Reg<isr3::ISR3_SPEC>;
#[doc = "IRQ status resister 3"]
pub mod isr3;
#[doc = "ISR4 (r) register accessor: an alias for `Reg<ISR4_SPEC>`"]
pub type ISR4 = crate::Reg<isr4::ISR4_SPEC>;
#[doc = "IRQ status resister 4"]
pub mod isr4;
#[doc = "IMR5 (rw) register accessor: an alias for `Reg<IMR5_SPEC>`"]
pub type IMR5 = crate::Reg<imr5::IMR5_SPEC>;
#[doc = "IRQ masking register 5"]
pub mod imr5;
#[doc = "ISR5 (r) register accessor: an alias for `Reg<ISR5_SPEC>`"]
pub type ISR5 = crate::Reg<isr5::ISR5_SPEC>;
#[doc = "IRQ status resister 5"]
pub mod isr5;
