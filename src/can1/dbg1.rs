#[doc = "Register `DBG1` reader"]
pub struct R(crate::R<DBG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFSM` reader - CAN Finite State Machine"]
pub type CFSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CBN` reader - CAN Bit Number"]
pub type CBN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - CAN Finite State Machine"]
    #[inline(always)]
    pub fn cfsm(&self) -> CFSM_R {
        CFSM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - CAN Bit Number"]
    #[inline(always)]
    pub fn cbn(&self) -> CBN_R {
        CBN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "Debug 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg1](index.html) module"]
pub struct DBG1_SPEC;
impl crate::RegisterSpec for DBG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg1::R](R) reader structure"]
impl crate::Readable for DBG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBG1 to value 0x0001_0000"]
impl crate::Resettable for DBG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
