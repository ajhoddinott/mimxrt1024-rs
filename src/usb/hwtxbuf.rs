#[doc = "Register `HWTXBUF` reader"]
pub struct R(crate::R<HWTXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWTXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWTXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWTXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXBURST` reader - Default burst size for memory to TX buffer transfer"]
pub type TXBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCHANADD` reader - TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
pub type TXCHANADD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default burst size for memory to TX buffer transfer"]
    #[inline(always)]
    pub fn txburst(&self) -> TXBURST_R {
        TXBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
    #[inline(always)]
    pub fn txchanadd(&self) -> TXCHANADD_R {
        TXCHANADD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "TX Buffer Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwtxbuf](index.html) module"]
pub struct HWTXBUF_SPEC;
impl crate::RegisterSpec for HWTXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwtxbuf::R](R) reader structure"]
impl crate::Readable for HWTXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWTXBUF to value 0x8008_0b08"]
impl crate::Resettable for HWTXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008_0b08;
}
