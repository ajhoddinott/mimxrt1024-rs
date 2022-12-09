#[doc = "Register `VERSION` reader"]
pub struct R(crate::R<VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STEP` reader - Fixed read-only value reflecting the stepping of the RTL version."]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MINOR` reader - Fixed read-only value reflecting the MINOR field of the RTL version."]
pub type MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJOR` reader - Fixed read-only value reflecting the MAJOR field of the RTL version."]
pub type MAJOR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fixed read-only value reflecting the MAJOR field of the RTL version."]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "UTMI RTL Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](index.html) module"]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version::R](R) reader structure"]
impl crate::Readable for VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERSION to value 0x0403_0000"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0403_0000;
}
