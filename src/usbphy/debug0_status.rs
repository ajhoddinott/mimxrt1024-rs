#[doc = "Register `DEBUG0_STATUS` reader"]
pub struct R(crate::R<DEBUG0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOOP_BACK_FAIL_COUNT` reader - Running count of the failed pseudo-random generator loopback"]
pub type LOOP_BACK_FAIL_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UTMI_RXERROR_FAIL_COUNT` reader - Running count of the UTMI_RXERROR."]
pub type UTMI_RXERROR_FAIL_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SQUELCH_COUNT` reader - Running count of the squelch reset instead of normal end for HS RX."]
pub type SQUELCH_COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Running count of the failed pseudo-random generator loopback"]
    #[inline(always)]
    pub fn loop_back_fail_count(&self) -> LOOP_BACK_FAIL_COUNT_R {
        LOOP_BACK_FAIL_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Running count of the UTMI_RXERROR."]
    #[inline(always)]
    pub fn utmi_rxerror_fail_count(&self) -> UTMI_RXERROR_FAIL_COUNT_R {
        UTMI_RXERROR_FAIL_COUNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - Running count of the squelch reset instead of normal end for HS RX."]
    #[inline(always)]
    pub fn squelch_count(&self) -> SQUELCH_COUNT_R {
        SQUELCH_COUNT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[doc = "UTMI Debug Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug0_status](index.html) module"]
pub struct DEBUG0_STATUS_SPEC;
impl crate::RegisterSpec for DEBUG0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug0_status::R](R) reader structure"]
impl crate::Readable for DEBUG0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEBUG0_STATUS to value 0"]
impl crate::Resettable for DEBUG0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
