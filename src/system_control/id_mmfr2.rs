#[doc = "Register `ID_MMFR2` reader"]
pub struct R(crate::R<ID_MMFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_MMFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_MMFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_MMFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WFI_STALL` reader - Indicates the support for Wait For Interrupt (WFI) stalling"]
pub type WFI_STALL_R = crate::FieldReader<u8, WFI_STALL_A>;
#[doc = "Indicates the support for Wait For Interrupt (WFI) stalling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WFI_STALL_A {
    #[doc = "0: Not supported"]
    WFI_STALL_0 = 0,
    #[doc = "1: Support for WFI stalling"]
    WFI_STALL_1 = 1,
}
impl From<WFI_STALL_A> for u8 {
    #[inline(always)]
    fn from(variant: WFI_STALL_A) -> Self {
        variant as _
    }
}
impl WFI_STALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WFI_STALL_A> {
        match self.bits {
            0 => Some(WFI_STALL_A::WFI_STALL_0),
            1 => Some(WFI_STALL_A::WFI_STALL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_0`"]
    #[inline(always)]
    pub fn is_wfi_stall_0(&self) -> bool {
        *self == WFI_STALL_A::WFI_STALL_0
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_1`"]
    #[inline(always)]
    pub fn is_wfi_stall_1(&self) -> bool {
        *self == WFI_STALL_A::WFI_STALL_1
    }
}
impl R {
    #[doc = "Bits 24:27 - Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub fn wfi_stall(&self) -> WFI_STALL_R {
        WFI_STALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Memory Model Feature Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr2](index.html) module"]
pub struct ID_MMFR2_SPEC;
impl crate::RegisterSpec for ID_MMFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_mmfr2::R](R) reader structure"]
impl crate::Readable for ID_MMFR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID_MMFR2 to value 0"]
impl crate::Resettable for ID_MMFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
