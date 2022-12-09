#[doc = "Register `MFSR` reader"]
pub struct R(crate::R<MFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOUNT` reader - Transmit FIFO Count"]
pub type TXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCOUNT` reader - Receive FIFO Count"]
pub type RXCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Count"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Receive FIFO Count"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Master FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfsr](index.html) module"]
pub struct MFSR_SPEC;
impl crate::RegisterSpec for MFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfsr::R](R) reader structure"]
impl crate::Readable for MFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFSR to value 0"]
impl crate::Resettable for MFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
