#[doc = "Register `CAPLENGTH` reader"]
pub struct R(crate::R<CAPLENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPLENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPLENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPLENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPLENGTH` reader - These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
pub type CAPLENGTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new(self.bits)
    }
}
#[doc = "Capability Registers Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength](index.html) module"]
pub struct CAPLENGTH_SPEC;
impl crate::RegisterSpec for CAPLENGTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [caplength::R](R) reader structure"]
impl crate::Readable for CAPLENGTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPLENGTH to value 0x40"]
impl crate::Resettable for CAPLENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
