#[doc = "Register `RFDR[%s]` reader"]
pub struct R(crate::R<RFDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RXDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits)
    }
}
#[doc = "IP RX FIFO Data Register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfdr](index.html) module"]
pub struct RFDR_SPEC;
impl crate::RegisterSpec for RFDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfdr::R](R) reader structure"]
impl crate::Readable for RFDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFDR[%s]
to value 0"]
impl crate::Resettable for RFDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
