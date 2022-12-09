#[doc = "Register `ISR3` reader"]
pub struct R(crate::R<ISR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISR3` reader - IRQ\\[95:64\\]
status, read only"]
pub type ISR3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[95:64\\]
status, read only"]
    #[inline(always)]
    pub fn isr3(&self) -> ISR3_R {
        ISR3_R::new(self.bits)
    }
}
#[doc = "IRQ status resister 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr3](index.html) module"]
pub struct ISR3_SPEC;
impl crate::RegisterSpec for ISR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr3::R](R) reader structure"]
impl crate::Readable for ISR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR3 to value 0"]
impl crate::Resettable for ISR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
