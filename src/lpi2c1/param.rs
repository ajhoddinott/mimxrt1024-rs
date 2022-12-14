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
#[doc = "Field `MTXFIFO` reader - Master Transmit FIFO Size"]
pub type MTXFIFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRXFIFO` reader - Master Receive FIFO Size"]
pub type MRXFIFO_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Master Transmit FIFO Size"]
    #[inline(always)]
    pub fn mtxfifo(&self) -> MTXFIFO_R {
        MTXFIFO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master Receive FIFO Size"]
    #[inline(always)]
    pub fn mrxfifo(&self) -> MRXFIFO_R {
        MRXFIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
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
#[doc = "`reset()` method sets PARAM to value 0x0202"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202;
}
