#[doc = "Register `ISR2` reader"]
pub struct R(crate::R<ISR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISR2` reader - IRQ\\[63:32\\]
status, read only"]
pub type ISR2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[63:32\\]
status, read only"]
    #[inline(always)]
    pub fn isr2(&self) -> ISR2_R {
        ISR2_R::new(self.bits)
    }
}
#[doc = "IRQ status resister 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr2](index.html) module"]
pub struct ISR2_SPEC;
impl crate::RegisterSpec for ISR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr2::R](R) reader structure"]
impl crate::Readable for ISR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR2 to value 0"]
impl crate::Resettable for ISR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
