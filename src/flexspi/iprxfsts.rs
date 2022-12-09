#[doc = "Register `IPRXFSTS` reader"]
pub struct R(crate::R<IPRXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FILL` reader - Fill level of IP RX FIFO."]
pub type FILL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDCNTR` reader - Total Read Data Counter: RDCNTR * 64 Bits."]
pub type RDCNTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP RX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub fn rdcntr(&self) -> RDCNTR_R {
        RDCNTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IP RX FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprxfsts](index.html) module"]
pub struct IPRXFSTS_SPEC;
impl crate::RegisterSpec for IPRXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprxfsts::R](R) reader structure"]
impl crate::Readable for IPRXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPRXFSTS to value 0"]
impl crate::Resettable for IPRXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
