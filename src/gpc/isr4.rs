#[doc = "Register `ISR4` reader"]
pub struct R(crate::R<ISR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISR4` reader - IRQ\\[127:96\\]
status, read only"]
pub type ISR4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ\\[127:96\\]
status, read only"]
    #[inline(always)]
    pub fn isr4(&self) -> ISR4_R {
        ISR4_R::new(self.bits)
    }
}
#[doc = "IRQ status resister 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr4](index.html) module"]
pub struct ISR4_SPEC;
impl crate::RegisterSpec for ISR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr4::R](R) reader structure"]
impl crate::Readable for ISR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR4 to value 0"]
impl crate::Resettable for ISR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
