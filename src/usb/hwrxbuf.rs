#[doc = "Register `HWRXBUF` reader"]
pub struct R(crate::R<HWRXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWRXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWRXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWRXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXBURST` reader - Default burst size for memory to RX buffer transfer"]
pub type RXBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXADD` reader - Buffer total size for all receive endpoints is (2^RXADD)"]
pub type RXADD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default burst size for memory to RX buffer transfer"]
    #[inline(always)]
    pub fn rxburst(&self) -> RXBURST_R {
        RXBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Buffer total size for all receive endpoints is (2^RXADD)"]
    #[inline(always)]
    pub fn rxadd(&self) -> RXADD_R {
        RXADD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RX Buffer Hardware Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrxbuf](index.html) module"]
pub struct HWRXBUF_SPEC;
impl crate::RegisterSpec for HWRXBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwrxbuf::R](R) reader structure"]
impl crate::Readable for HWRXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWRXBUF to value 0x0808"]
impl crate::Resettable for HWRXBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808;
}
