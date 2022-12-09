#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFIFO` reader - Transmit FIFO Size"]
pub type TXFIFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO` reader - Receive FIFO Size"]
pub type RXFIFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCSNUM` reader - PCS Number"]
pub type PCSNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Size"]
    #[inline(always)]
    pub fn txfifo(&self) -> TXFIFO_R {
        TXFIFO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO Size"]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PCS Number"]
    #[inline(always)]
    pub fn pcsnum(&self) -> PCSNUM_R {
        PCSNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Parameter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x0004_0404"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0404;
}
