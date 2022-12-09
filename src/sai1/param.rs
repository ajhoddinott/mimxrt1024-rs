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
#[doc = "Field `DATALINE` reader - Number of Datalines"]
pub type DATALINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFO` reader - FIFO Size"]
pub type FIFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAME` reader - Frame Size"]
pub type FRAME_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of Datalines"]
    #[inline(always)]
    pub fn dataline(&self) -> DATALINE_R {
        DATALINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - FIFO Size"]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Frame Size"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 16) & 0x0f) as u8)
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
#[doc = "`reset()` method sets PARAM to value 0x0005_0504"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_0504;
}
