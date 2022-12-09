#[doc = "Register `IPTXFSTS` reader"]
pub struct R(crate::R<IPTXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FILL` reader - Fill level of IP TX FIFO."]
pub type FILL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRCNTR` reader - Total Write Data Counter: WRCNTR * 64 Bits."]
pub type WRCNTR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fill level of IP TX FIFO."]
    #[inline(always)]
    pub fn fill(&self) -> FILL_R {
        FILL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub fn wrcntr(&self) -> WRCNTR_R {
        WRCNTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IP TX FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxfsts](index.html) module"]
pub struct IPTXFSTS_SPEC;
impl crate::RegisterSpec for IPTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptxfsts::R](R) reader structure"]
impl crate::Readable for IPTXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPTXFSTS to value 0"]
impl crate::Resettable for IPTXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
