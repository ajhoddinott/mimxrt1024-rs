#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISFOLD` reader - Disables folding of IT instructions."]
pub type DISFOLD_R = crate::BitReader<DISFOLD_A>;
#[doc = "Disables folding of IT instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISFOLD_A {
    #[doc = "0: Normal operation."]
    DISFOLD_0 = 0,
}
impl From<DISFOLD_A> for bool {
    #[inline(always)]
    fn from(variant: DISFOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl DISFOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISFOLD_A> {
        match self.bits {
            false => Some(DISFOLD_A::DISFOLD_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISFOLD_0`"]
    #[inline(always)]
    pub fn is_disfold_0(&self) -> bool {
        *self == DISFOLD_A::DISFOLD_0
    }
}
#[doc = "Field `DISFOLD` writer - Disables folding of IT instructions."]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISFOLD_A, O>;
impl<'a, const O: u8> DISFOLD_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disfold_0(self) -> &'a mut W {
        self.variant(DISFOLD_A::DISFOLD_0)
    }
}
#[doc = "Field `FPEXCODIS` reader - Disables FPU exception outputs."]
pub type FPEXCODIS_R = crate::BitReader<FPEXCODIS_A>;
#[doc = "Disables FPU exception outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPEXCODIS_A {
    #[doc = "0: Normal operation."]
    FPEXCODIS_0 = 0,
    #[doc = "1: FPU exception outputs are disabled."]
    FPEXCODIS_1 = 1,
}
impl From<FPEXCODIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPEXCODIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FPEXCODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPEXCODIS_A {
        match self.bits {
            false => FPEXCODIS_A::FPEXCODIS_0,
            true => FPEXCODIS_A::FPEXCODIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_0`"]
    #[inline(always)]
    pub fn is_fpexcodis_0(&self) -> bool {
        *self == FPEXCODIS_A::FPEXCODIS_0
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_1`"]
    #[inline(always)]
    pub fn is_fpexcodis_1(&self) -> bool {
        *self == FPEXCODIS_A::FPEXCODIS_1
    }
}
#[doc = "Field `FPEXCODIS` writer - Disables FPU exception outputs."]
pub type FPEXCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, FPEXCODIS_A, O>;
impl<'a, const O: u8> FPEXCODIS_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn fpexcodis_0(self) -> &'a mut W {
        self.variant(FPEXCODIS_A::FPEXCODIS_0)
    }
    #[doc = "FPU exception outputs are disabled."]
    #[inline(always)]
    pub fn fpexcodis_1(self) -> &'a mut W {
        self.variant(FPEXCODIS_A::FPEXCODIS_1)
    }
}
#[doc = "Field `DISRAMODE` reader - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
pub type DISRAMODE_R = crate::BitReader<DISRAMODE_A>;
#[doc = "Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISRAMODE_A {
    #[doc = "0: Normal operation."]
    DISRAMODE_0 = 0,
    #[doc = "1: Dynamic disabled."]
    DISRAMODE_1 = 1,
}
impl From<DISRAMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DISRAMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DISRAMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISRAMODE_A {
        match self.bits {
            false => DISRAMODE_A::DISRAMODE_0,
            true => DISRAMODE_A::DISRAMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_0`"]
    #[inline(always)]
    pub fn is_disramode_0(&self) -> bool {
        *self == DISRAMODE_A::DISRAMODE_0
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_1`"]
    #[inline(always)]
    pub fn is_disramode_1(&self) -> bool {
        *self == DISRAMODE_A::DISRAMODE_1
    }
}
#[doc = "Field `DISRAMODE` writer - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
pub type DISRAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISRAMODE_A, O>;
impl<'a, const O: u8> DISRAMODE_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disramode_0(self) -> &'a mut W {
        self.variant(DISRAMODE_A::DISRAMODE_0)
    }
    #[doc = "Dynamic disabled."]
    #[inline(always)]
    pub fn disramode_1(self) -> &'a mut W {
        self.variant(DISRAMODE_A::DISRAMODE_1)
    }
}
#[doc = "Field `DISITMATBFLUSH` reader - Disables ITM and DWT ATB flush."]
pub type DISITMATBFLUSH_R = crate::BitReader<DISITMATBFLUSH_A>;
#[doc = "Disables ITM and DWT ATB flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISITMATBFLUSH_A {
    #[doc = "1: ITM and DWT ATB flush disabled, this bit is always 1."]
    DISITMATBFLUSH_1 = 1,
}
impl From<DISITMATBFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: DISITMATBFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl DISITMATBFLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISITMATBFLUSH_A> {
        match self.bits {
            true => Some(DISITMATBFLUSH_A::DISITMATBFLUSH_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISITMATBFLUSH_1`"]
    #[inline(always)]
    pub fn is_disitmatbflush_1(&self) -> bool {
        *self == DISITMATBFLUSH_A::DISITMATBFLUSH_1
    }
}
#[doc = "Field `DISITMATBFLUSH` writer - Disables ITM and DWT ATB flush."]
pub type DISITMATBFLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ACTLR_SPEC, DISITMATBFLUSH_A, O>;
impl<'a, const O: u8> DISITMATBFLUSH_W<'a, O> {
    #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
    #[inline(always)]
    pub fn disitmatbflush_1(self) -> &'a mut W {
        self.variant(DISITMATBFLUSH_A::DISITMATBFLUSH_1)
    }
}
#[doc = "Field `DISBTACREAD` reader - Disables BTAC read."]
pub type DISBTACREAD_R = crate::BitReader<DISBTACREAD_A>;
#[doc = "Disables BTAC read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISBTACREAD_A {
    #[doc = "0: Normal operation."]
    DISBTACREAD_0 = 0,
    #[doc = "1: BTAC is not used and only static branch prediction can occur."]
    DISBTACREAD_1 = 1,
}
impl From<DISBTACREAD_A> for bool {
    #[inline(always)]
    fn from(variant: DISBTACREAD_A) -> Self {
        variant as u8 != 0
    }
}
impl DISBTACREAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISBTACREAD_A {
        match self.bits {
            false => DISBTACREAD_A::DISBTACREAD_0,
            true => DISBTACREAD_A::DISBTACREAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_0`"]
    #[inline(always)]
    pub fn is_disbtacread_0(&self) -> bool {
        *self == DISBTACREAD_A::DISBTACREAD_0
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_1`"]
    #[inline(always)]
    pub fn is_disbtacread_1(&self) -> bool {
        *self == DISBTACREAD_A::DISBTACREAD_1
    }
}
#[doc = "Field `DISBTACREAD` writer - Disables BTAC read."]
pub type DISBTACREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISBTACREAD_A, O>;
impl<'a, const O: u8> DISBTACREAD_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disbtacread_0(self) -> &'a mut W {
        self.variant(DISBTACREAD_A::DISBTACREAD_0)
    }
    #[doc = "BTAC is not used and only static branch prediction can occur."]
    #[inline(always)]
    pub fn disbtacread_1(self) -> &'a mut W {
        self.variant(DISBTACREAD_A::DISBTACREAD_1)
    }
}
#[doc = "Field `DISBTACALLOC` reader - Disables BTAC allocate."]
pub type DISBTACALLOC_R = crate::BitReader<DISBTACALLOC_A>;
#[doc = "Disables BTAC allocate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISBTACALLOC_A {
    #[doc = "0: Normal operation."]
    DISBTACALLOC_0 = 0,
    #[doc = "1: No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    DISBTACALLOC_1 = 1,
}
impl From<DISBTACALLOC_A> for bool {
    #[inline(always)]
    fn from(variant: DISBTACALLOC_A) -> Self {
        variant as u8 != 0
    }
}
impl DISBTACALLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISBTACALLOC_A {
        match self.bits {
            false => DISBTACALLOC_A::DISBTACALLOC_0,
            true => DISBTACALLOC_A::DISBTACALLOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_0`"]
    #[inline(always)]
    pub fn is_disbtacalloc_0(&self) -> bool {
        *self == DISBTACALLOC_A::DISBTACALLOC_0
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_1`"]
    #[inline(always)]
    pub fn is_disbtacalloc_1(&self) -> bool {
        *self == DISBTACALLOC_A::DISBTACALLOC_1
    }
}
#[doc = "Field `DISBTACALLOC` writer - Disables BTAC allocate."]
pub type DISBTACALLOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISBTACALLOC_A, O>;
impl<'a, const O: u8> DISBTACALLOC_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disbtacalloc_0(self) -> &'a mut W {
        self.variant(DISBTACALLOC_A::DISBTACALLOC_0)
    }
    #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    #[inline(always)]
    pub fn disbtacalloc_1(self) -> &'a mut W {
        self.variant(DISBTACALLOC_A::DISBTACALLOC_1)
    }
}
#[doc = "Field `DISCRITAXIRUR` reader - Disables critical AXI Read-Under-Read."]
pub type DISCRITAXIRUR_R = crate::BitReader<DISCRITAXIRUR_A>;
#[doc = "Disables critical AXI Read-Under-Read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCRITAXIRUR_A {
    #[doc = "0: Normal operation."]
    DISCRITAXIRUR_0 = 0,
    #[doc = "1: An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    DISCRITAXIRUR_1 = 1,
}
impl From<DISCRITAXIRUR_A> for bool {
    #[inline(always)]
    fn from(variant: DISCRITAXIRUR_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCRITAXIRUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCRITAXIRUR_A {
        match self.bits {
            false => DISCRITAXIRUR_A::DISCRITAXIRUR_0,
            true => DISCRITAXIRUR_A::DISCRITAXIRUR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_0`"]
    #[inline(always)]
    pub fn is_discritaxirur_0(&self) -> bool {
        *self == DISCRITAXIRUR_A::DISCRITAXIRUR_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_1`"]
    #[inline(always)]
    pub fn is_discritaxirur_1(&self) -> bool {
        *self == DISCRITAXIRUR_A::DISCRITAXIRUR_1
    }
}
#[doc = "Field `DISCRITAXIRUR` writer - Disables critical AXI Read-Under-Read."]
pub type DISCRITAXIRUR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ACTLR_SPEC, DISCRITAXIRUR_A, O>;
impl<'a, const O: u8> DISCRITAXIRUR_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn discritaxirur_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRUR_A::DISCRITAXIRUR_0)
    }
    #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    #[inline(always)]
    pub fn discritaxirur_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRUR_A::DISCRITAXIRUR_1)
    }
}
#[doc = "Field `DISDI` reader - Disables dual-issued."]
pub type DISDI_R = crate::FieldReader<u8, DISDI_A>;
#[doc = "Disables dual-issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISDI_A {
    #[doc = "0: Normal operation."]
    DISDI_0 = 0,
    #[doc = "1: Nothing can be dual-issued when this instruction type is in channel 0."]
    DISDI_1 = 1,
}
impl From<DISDI_A> for u8 {
    #[inline(always)]
    fn from(variant: DISDI_A) -> Self {
        variant as _
    }
}
impl DISDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISDI_A> {
        match self.bits {
            0 => Some(DISDI_A::DISDI_0),
            1 => Some(DISDI_A::DISDI_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISDI_0`"]
    #[inline(always)]
    pub fn is_disdi_0(&self) -> bool {
        *self == DISDI_A::DISDI_0
    }
    #[doc = "Checks if the value of the field is `DISDI_1`"]
    #[inline(always)]
    pub fn is_disdi_1(&self) -> bool {
        *self == DISDI_A::DISDI_1
    }
}
#[doc = "Field `DISDI` writer - Disables dual-issued."]
pub type DISDI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u8, DISDI_A, 5, O>;
impl<'a, const O: u8> DISDI_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disdi_0(self) -> &'a mut W {
        self.variant(DISDI_A::DISDI_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    #[inline(always)]
    pub fn disdi_1(self) -> &'a mut W {
        self.variant(DISDI_A::DISDI_1)
    }
}
#[doc = "Field `DISISSCH1` reader - Disables dual-issued."]
pub type DISISSCH1_R = crate::FieldReader<u8, DISISSCH1_A>;
#[doc = "Disables dual-issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DISISSCH1_A {
    #[doc = "0: Normal operation."]
    DISISSCH1_0 = 0,
    #[doc = "1: Nothing can be dual-issued when this instruction type is in channel 1."]
    DISISSCH1_1 = 1,
}
impl From<DISISSCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: DISISSCH1_A) -> Self {
        variant as _
    }
}
impl DISISSCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISISSCH1_A> {
        match self.bits {
            0 => Some(DISISSCH1_A::DISISSCH1_0),
            1 => Some(DISISSCH1_A::DISISSCH1_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_0`"]
    #[inline(always)]
    pub fn is_disissch1_0(&self) -> bool {
        *self == DISISSCH1_A::DISISSCH1_0
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_1`"]
    #[inline(always)]
    pub fn is_disissch1_1(&self) -> bool {
        *self == DISISSCH1_A::DISISSCH1_1
    }
}
#[doc = "Field `DISISSCH1` writer - Disables dual-issued."]
pub type DISISSCH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACTLR_SPEC, u8, DISISSCH1_A, 5, O>;
impl<'a, const O: u8> DISISSCH1_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disissch1_0(self) -> &'a mut W {
        self.variant(DISISSCH1_A::DISISSCH1_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    #[inline(always)]
    pub fn disissch1_1(self) -> &'a mut W {
        self.variant(DISISSCH1_A::DISISSCH1_1)
    }
}
#[doc = "Field `DISDYNADD` reader - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISDYNADD_R = crate::BitReader<DISDYNADD_A>;
#[doc = "Disables dynamic allocation of ADD and SUB instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISDYNADD_A {
    #[doc = "0: Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    DISDYNADD_0 = 0,
    #[doc = "1: All ADD and SUB instructions are resolved in EX2."]
    DISDYNADD_1 = 1,
}
impl From<DISDYNADD_A> for bool {
    #[inline(always)]
    fn from(variant: DISDYNADD_A) -> Self {
        variant as u8 != 0
    }
}
impl DISDYNADD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISDYNADD_A {
        match self.bits {
            false => DISDYNADD_A::DISDYNADD_0,
            true => DISDYNADD_A::DISDYNADD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_0`"]
    #[inline(always)]
    pub fn is_disdynadd_0(&self) -> bool {
        *self == DISDYNADD_A::DISDYNADD_0
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_1`"]
    #[inline(always)]
    pub fn is_disdynadd_1(&self) -> bool {
        *self == DISDYNADD_A::DISDYNADD_1
    }
}
#[doc = "Field `DISDYNADD` writer - Disables dynamic allocation of ADD and SUB instructions"]
pub type DISDYNADD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISDYNADD_A, O>;
impl<'a, const O: u8> DISDYNADD_W<'a, O> {
    #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    #[inline(always)]
    pub fn disdynadd_0(self) -> &'a mut W {
        self.variant(DISDYNADD_A::DISDYNADD_0)
    }
    #[doc = "All ADD and SUB instructions are resolved in EX2."]
    #[inline(always)]
    pub fn disdynadd_1(self) -> &'a mut W {
        self.variant(DISDYNADD_A::DISDYNADD_1)
    }
}
#[doc = "Field `DISCRITAXIRUW` reader - Disables critical AXI read-under-write"]
pub type DISCRITAXIRUW_R = crate::BitReader<DISCRITAXIRUW_A>;
#[doc = "Disables critical AXI read-under-write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCRITAXIRUW_A {
    #[doc = "0: Normal operation. This is backwards compatible with r0."]
    DISCRITAXIRUW_0 = 0,
    #[doc = "1: AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    DISCRITAXIRUW_1 = 1,
}
impl From<DISCRITAXIRUW_A> for bool {
    #[inline(always)]
    fn from(variant: DISCRITAXIRUW_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCRITAXIRUW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCRITAXIRUW_A {
        match self.bits {
            false => DISCRITAXIRUW_A::DISCRITAXIRUW_0,
            true => DISCRITAXIRUW_A::DISCRITAXIRUW_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_0`"]
    #[inline(always)]
    pub fn is_discritaxiruw_0(&self) -> bool {
        *self == DISCRITAXIRUW_A::DISCRITAXIRUW_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_1`"]
    #[inline(always)]
    pub fn is_discritaxiruw_1(&self) -> bool {
        *self == DISCRITAXIRUW_A::DISCRITAXIRUW_1
    }
}
#[doc = "Field `DISCRITAXIRUW` writer - Disables critical AXI read-under-write"]
pub type DISCRITAXIRUW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ACTLR_SPEC, DISCRITAXIRUW_A, O>;
impl<'a, const O: u8> DISCRITAXIRUW_W<'a, O> {
    #[doc = "Normal operation. This is backwards compatible with r0."]
    #[inline(always)]
    pub fn discritaxiruw_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRUW_A::DISCRITAXIRUW_0)
    }
    #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    #[inline(always)]
    pub fn discritaxiruw_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRUW_A::DISCRITAXIRUW_1)
    }
}
#[doc = "Field `DISFPUISSOPT` reader - Disables critical AXI read-under-write"]
pub type DISFPUISSOPT_R = crate::BitReader<DISFPUISSOPT_A>;
#[doc = "Disables critical AXI read-under-write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISFPUISSOPT_A {
    #[doc = "0: Normal operation."]
    DISFPUISSOPT_0 = 0,
}
impl From<DISFPUISSOPT_A> for bool {
    #[inline(always)]
    fn from(variant: DISFPUISSOPT_A) -> Self {
        variant as u8 != 0
    }
}
impl DISFPUISSOPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISFPUISSOPT_A> {
        match self.bits {
            false => Some(DISFPUISSOPT_A::DISFPUISSOPT_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISFPUISSOPT_0`"]
    #[inline(always)]
    pub fn is_disfpuissopt_0(&self) -> bool {
        *self == DISFPUISSOPT_A::DISFPUISSOPT_0
    }
}
#[doc = "Field `DISFPUISSOPT` writer - Disables critical AXI read-under-write"]
pub type DISFPUISSOPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, DISFPUISSOPT_A, O>;
impl<'a, const O: u8> DISFPUISSOPT_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disfpuissopt_0(self) -> &'a mut W {
        self.variant(DISFPUISSOPT_A::DISFPUISSOPT_0)
    }
}
impl R {
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline(always)]
    pub fn disbtacread(&self) -> DISBTACREAD_R {
        DISBTACREAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline(always)]
    pub fn disbtacalloc(&self) -> DISBTACALLOC_R {
        DISBTACALLOC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    pub fn discritaxirur(&self) -> DISCRITAXIRUR_R {
        DISCRITAXIRUR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline(always)]
    pub fn disdi(&self) -> DISDI_R {
        DISDI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline(always)]
    pub fn disissch1(&self) -> DISISSCH1_R {
        DISISSCH1_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&self) -> DISDYNADD_R {
        DISDYNADD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&self) -> DISCRITAXIRUW_R {
        DISCRITAXIRUW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn disfpuissopt(&self) -> DISFPUISSOPT_R {
        DISFPUISSOPT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<10> {
        FPEXCODIS_W::new(self)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    #[must_use]
    pub fn disramode(&mut self) -> DISRAMODE_W<11> {
        DISRAMODE_W::new(self)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<12> {
        DISITMATBFLUSH_W::new(self)
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline(always)]
    #[must_use]
    pub fn disbtacread(&mut self) -> DISBTACREAD_W<13> {
        DISBTACREAD_W::new(self)
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline(always)]
    #[must_use]
    pub fn disbtacalloc(&mut self) -> DISBTACALLOC_W<14> {
        DISBTACALLOC_W::new(self)
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    #[must_use]
    pub fn discritaxirur(&mut self) -> DISCRITAXIRUR_W<15> {
        DISCRITAXIRUR_W::new(self)
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline(always)]
    #[must_use]
    pub fn disdi(&mut self) -> DISDI_W<16> {
        DISDI_W::new(self)
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline(always)]
    #[must_use]
    pub fn disissch1(&mut self) -> DISISSCH1_W<21> {
        DISISSCH1_W::new(self)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    #[must_use]
    pub fn disdynadd(&mut self) -> DISDYNADD_W<26> {
        DISDYNADD_W::new(self)
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline(always)]
    #[must_use]
    pub fn discritaxiruw(&mut self) -> DISCRITAXIRUW_W<27> {
        DISCRITAXIRUW_W::new(self)
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline(always)]
    #[must_use]
    pub fn disfpuissopt(&mut self) -> DISFPUISSOPT_W<28> {
        DISFPUISSOPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control Register,\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
