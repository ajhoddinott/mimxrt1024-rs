#[doc = "Register `ISR5` reader"]
pub struct R(crate::R<ISR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISR5` reader - IRQ\\[159:128\\]
status, read only"]
pub type ISR5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[159:128\\]
status, read only"]
    #[inline(always)]
    pub fn isr5(&self) -> ISR5_R {
        ISR5_R::new(self.bits)
    }
}
#[doc = "IRQ status resister 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr5](index.html) module"]
pub struct ISR5_SPEC;
impl crate::RegisterSpec for ISR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr5::R](R) reader structure"]
impl crate::Readable for ISR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR5 to value 0"]
impl crate::Resettable for ISR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
