#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Auxiliary Control Register,"]
    pub actlr: ACTLR,
    _reserved1: [u8; 0x0cf4],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control Register"]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: CFSR,
    #[doc = "0xd2c - HardFault Status register"]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: DFSR,
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: BFAR,
    _reserved16: [u8; 0x04],
    #[doc = "0xd40 - Processor Feature Register 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0xd44 - Processor Feature Register 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0xd48 - Debug Feature Register"]
    pub id_dfr0: ID_DFR0,
    #[doc = "0xd4c - Auxiliary Feature Register"]
    pub id_afr0: ID_AFR0,
    #[doc = "0xd50 - Memory Model Feature Register 0"]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0xd54 - Memory Model Feature Register 1"]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0xd58 - Memory Model Feature Register 2"]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0xd5c - Memory Model Feature Register 3"]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0xd60 - Instruction Set Attributes Register 0"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0xd64 - Instruction Set Attributes Register 1"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0xd68 - Instruction Set Attributes Register 2"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0xd6c - Instruction Set Attributes Register 3"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0xd70 - Instruction Set Attributes Register 4"]
    pub id_isar4: ID_ISAR4,
    _reserved29: [u8; 0x04],
    #[doc = "0xd78 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0xd7c - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0xd80 - Cache Size ID Register"]
    pub ccsidr: CCSIDR,
    #[doc = "0xd84 - Cache Size Selection Register"]
    pub csselr: CSSELR,
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: CPACR,
    _reserved34: [u8; 0x0174],
    #[doc = "0xf00 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub stir: STIR,
    _reserved35: [u8; 0x4c],
    #[doc = "0xf50 - Instruction cache invalidate all to Point of Unification (PoU)"]
    pub iciallu: ICIALLU,
    _reserved36: [u8; 0x04],
    #[doc = "0xf58 - Instruction cache invalidate by address to PoU"]
    pub icimvau: ICIMVAU,
    #[doc = "0xf5c - Data cache invalidate by address to Point of Coherency (PoC)"]
    pub dcimvac: DCIMVAC,
    #[doc = "0xf60 - Data cache invalidate by set/way"]
    pub dcisw: DCISW,
    #[doc = "0xf64 - Data cache by address to PoU"]
    pub dccmvau: DCCMVAU,
    #[doc = "0xf68 - Data cache clean by address to PoC"]
    pub dccmvac: DCCMVAC,
    #[doc = "0xf6c - Data cache clean by set/way"]
    pub dccsw: DCCSW,
    #[doc = "0xf70 - Data cache clean and invalidate by address to PoC"]
    pub dccimvac: DCCIMVAC,
    #[doc = "0xf74 - Data cache clean and invalidate by set/way"]
    pub dccisw: DCCISW,
    _reserved44: [u8; 0x18],
    #[doc = "0xf90 - Instruction Tightly-Coupled Memory Control Register"]
    pub cm7_itcmcr: CM7_ITCMCR,
    #[doc = "0xf94 - Data Tightly-Coupled Memory Control Register"]
    pub cm7_dtcmcr: CM7_DTCMCR,
    #[doc = "0xf98 - AHBP Control Register"]
    pub cm7_ahbpcr: CM7_AHBPCR,
    #[doc = "0xf9c - L1 Cache Control Register"]
    pub cm7_cacr: CM7_CACR,
    #[doc = "0xfa0 - AHB Slave Control Register"]
    pub cm7_ahbscr: CM7_AHBSCR,
    _reserved49: [u8; 0x04],
    #[doc = "0xfa8 - Auxiliary Bus Fault Status Register"]
    pub cm7_abfsr: CM7_ABFSR,
}
#[doc = "ACTLR (rw) register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register,"]
pub mod actlr;
#[doc = "CPUID (r) register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR (rw) register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR (rw) register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR1 (rw) register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
#[doc = "SHPR2 (rw) register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 (rw) register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR (rw) register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "CFSR (rw) register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR (rw) register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HardFault Status register"]
pub mod hfsr;
#[doc = "DFSR (rw) register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MMFAR (rw) register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR (rw) register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "ID_PFR0 (r) register accessor: an alias for `Reg<ID_PFR0_SPEC>`"]
pub type ID_PFR0 = crate::Reg<id_pfr0::ID_PFR0_SPEC>;
#[doc = "Processor Feature Register 0"]
pub mod id_pfr0;
#[doc = "ID_PFR1 (r) register accessor: an alias for `Reg<ID_PFR1_SPEC>`"]
pub type ID_PFR1 = crate::Reg<id_pfr1::ID_PFR1_SPEC>;
#[doc = "Processor Feature Register 1"]
pub mod id_pfr1;
#[doc = "ID_DFR0 (r) register accessor: an alias for `Reg<ID_DFR0_SPEC>`"]
pub type ID_DFR0 = crate::Reg<id_dfr0::ID_DFR0_SPEC>;
#[doc = "Debug Feature Register"]
pub mod id_dfr0;
#[doc = "ID_AFR0 (r) register accessor: an alias for `Reg<ID_AFR0_SPEC>`"]
pub type ID_AFR0 = crate::Reg<id_afr0::ID_AFR0_SPEC>;
#[doc = "Auxiliary Feature Register"]
pub mod id_afr0;
#[doc = "ID_MMFR0 (r) register accessor: an alias for `Reg<ID_MMFR0_SPEC>`"]
pub type ID_MMFR0 = crate::Reg<id_mmfr0::ID_MMFR0_SPEC>;
#[doc = "Memory Model Feature Register 0"]
pub mod id_mmfr0;
#[doc = "ID_MMFR1 (r) register accessor: an alias for `Reg<ID_MMFR1_SPEC>`"]
pub type ID_MMFR1 = crate::Reg<id_mmfr1::ID_MMFR1_SPEC>;
#[doc = "Memory Model Feature Register 1"]
pub mod id_mmfr1;
#[doc = "ID_MMFR2 (r) register accessor: an alias for `Reg<ID_MMFR2_SPEC>`"]
pub type ID_MMFR2 = crate::Reg<id_mmfr2::ID_MMFR2_SPEC>;
#[doc = "Memory Model Feature Register 2"]
pub mod id_mmfr2;
#[doc = "ID_MMFR3 (r) register accessor: an alias for `Reg<ID_MMFR3_SPEC>`"]
pub type ID_MMFR3 = crate::Reg<id_mmfr3::ID_MMFR3_SPEC>;
#[doc = "Memory Model Feature Register 3"]
pub mod id_mmfr3;
#[doc = "ID_ISAR0 (r) register accessor: an alias for `Reg<ID_ISAR0_SPEC>`"]
pub type ID_ISAR0 = crate::Reg<id_isar0::ID_ISAR0_SPEC>;
#[doc = "Instruction Set Attributes Register 0"]
pub mod id_isar0;
#[doc = "ID_ISAR1 (r) register accessor: an alias for `Reg<ID_ISAR1_SPEC>`"]
pub type ID_ISAR1 = crate::Reg<id_isar1::ID_ISAR1_SPEC>;
#[doc = "Instruction Set Attributes Register 1"]
pub mod id_isar1;
#[doc = "ID_ISAR2 (r) register accessor: an alias for `Reg<ID_ISAR2_SPEC>`"]
pub type ID_ISAR2 = crate::Reg<id_isar2::ID_ISAR2_SPEC>;
#[doc = "Instruction Set Attributes Register 2"]
pub mod id_isar2;
#[doc = "ID_ISAR3 (r) register accessor: an alias for `Reg<ID_ISAR3_SPEC>`"]
pub type ID_ISAR3 = crate::Reg<id_isar3::ID_ISAR3_SPEC>;
#[doc = "Instruction Set Attributes Register 3"]
pub mod id_isar3;
#[doc = "ID_ISAR4 (r) register accessor: an alias for `Reg<ID_ISAR4_SPEC>`"]
pub type ID_ISAR4 = crate::Reg<id_isar4::ID_ISAR4_SPEC>;
#[doc = "Instruction Set Attributes Register 4"]
pub mod id_isar4;
#[doc = "CLIDR (r) register accessor: an alias for `Reg<CLIDR_SPEC>`"]
pub type CLIDR = crate::Reg<clidr::CLIDR_SPEC>;
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "CTR (r) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "CCSIDR (r) register accessor: an alias for `Reg<CCSIDR_SPEC>`"]
pub type CCSIDR = crate::Reg<ccsidr::CCSIDR_SPEC>;
#[doc = "Cache Size ID Register"]
pub mod ccsidr;
#[doc = "CSSELR (rw) register accessor: an alias for `Reg<CSSELR_SPEC>`"]
pub type CSSELR = crate::Reg<csselr::CSSELR_SPEC>;
#[doc = "Cache Size Selection Register"]
pub mod csselr;
#[doc = "CPACR (rw) register accessor: an alias for `Reg<CPACR_SPEC>`"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
#[doc = "STIR (w) register accessor: an alias for `Reg<STIR_SPEC>`"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod stir;
#[doc = "ICIALLU (w) register accessor: an alias for `Reg<ICIALLU_SPEC>`"]
pub type ICIALLU = crate::Reg<iciallu::ICIALLU_SPEC>;
#[doc = "Instruction cache invalidate all to Point of Unification (PoU)"]
pub mod iciallu;
#[doc = "ICIMVAU (w) register accessor: an alias for `Reg<ICIMVAU_SPEC>`"]
pub type ICIMVAU = crate::Reg<icimvau::ICIMVAU_SPEC>;
#[doc = "Instruction cache invalidate by address to PoU"]
pub mod icimvau;
#[doc = "DCIMVAC (w) register accessor: an alias for `Reg<DCIMVAC_SPEC>`"]
pub type DCIMVAC = crate::Reg<dcimvac::DCIMVAC_SPEC>;
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)"]
pub mod dcimvac;
#[doc = "DCISW (w) register accessor: an alias for `Reg<DCISW_SPEC>`"]
pub type DCISW = crate::Reg<dcisw::DCISW_SPEC>;
#[doc = "Data cache invalidate by set/way"]
pub mod dcisw;
#[doc = "DCCMVAU (w) register accessor: an alias for `Reg<DCCMVAU_SPEC>`"]
pub type DCCMVAU = crate::Reg<dccmvau::DCCMVAU_SPEC>;
#[doc = "Data cache by address to PoU"]
pub mod dccmvau;
#[doc = "DCCMVAC (w) register accessor: an alias for `Reg<DCCMVAC_SPEC>`"]
pub type DCCMVAC = crate::Reg<dccmvac::DCCMVAC_SPEC>;
#[doc = "Data cache clean by address to PoC"]
pub mod dccmvac;
#[doc = "DCCSW (w) register accessor: an alias for `Reg<DCCSW_SPEC>`"]
pub type DCCSW = crate::Reg<dccsw::DCCSW_SPEC>;
#[doc = "Data cache clean by set/way"]
pub mod dccsw;
#[doc = "DCCIMVAC (w) register accessor: an alias for `Reg<DCCIMVAC_SPEC>`"]
pub type DCCIMVAC = crate::Reg<dccimvac::DCCIMVAC_SPEC>;
#[doc = "Data cache clean and invalidate by address to PoC"]
pub mod dccimvac;
#[doc = "DCCISW (w) register accessor: an alias for `Reg<DCCISW_SPEC>`"]
pub type DCCISW = crate::Reg<dccisw::DCCISW_SPEC>;
#[doc = "Data cache clean and invalidate by set/way"]
pub mod dccisw;
#[doc = "CM7_ITCMCR (rw) register accessor: an alias for `Reg<CM7_ITCMCR_SPEC>`"]
pub type CM7_ITCMCR = crate::Reg<cm7_itcmcr::CM7_ITCMCR_SPEC>;
#[doc = "Instruction Tightly-Coupled Memory Control Register"]
pub mod cm7_itcmcr;
#[doc = "CM7_DTCMCR (rw) register accessor: an alias for `Reg<CM7_DTCMCR_SPEC>`"]
pub type CM7_DTCMCR = crate::Reg<cm7_dtcmcr::CM7_DTCMCR_SPEC>;
#[doc = "Data Tightly-Coupled Memory Control Register"]
pub mod cm7_dtcmcr;
#[doc = "CM7_AHBPCR (rw) register accessor: an alias for `Reg<CM7_AHBPCR_SPEC>`"]
pub type CM7_AHBPCR = crate::Reg<cm7_ahbpcr::CM7_AHBPCR_SPEC>;
#[doc = "AHBP Control Register"]
pub mod cm7_ahbpcr;
#[doc = "CM7_CACR (rw) register accessor: an alias for `Reg<CM7_CACR_SPEC>`"]
pub type CM7_CACR = crate::Reg<cm7_cacr::CM7_CACR_SPEC>;
#[doc = "L1 Cache Control Register"]
pub mod cm7_cacr;
#[doc = "CM7_AHBSCR (rw) register accessor: an alias for `Reg<CM7_AHBSCR_SPEC>`"]
pub type CM7_AHBSCR = crate::Reg<cm7_ahbscr::CM7_AHBSCR_SPEC>;
#[doc = "AHB Slave Control Register"]
pub mod cm7_ahbscr;
#[doc = "CM7_ABFSR (rw) register accessor: an alias for `Reg<CM7_ABFSR_SPEC>`"]
pub type CM7_ABFSR = crate::Reg<cm7_abfsr::CM7_ABFSR_SPEC>;
#[doc = "Auxiliary Bus Fault Status Register"]
pub mod cm7_abfsr;
