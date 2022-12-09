#[doc = "Register `STS2` reader"]
pub struct R(crate::R<STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ASLVLOCK` reader - Flash A sample clock slave delay line locked."]
pub type ASLVLOCK_R = crate::BitReader<bool>;
#[doc = "Field `AREFLOCK` reader - Flash A sample clock reference delay line locked."]
pub type AREFLOCK_R = crate::BitReader<bool>;
#[doc = "Field `ASLVSEL` reader - Flash A sample clock slave delay line delay cell number selection ."]
pub type ASLVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREFSEL` reader - Flash A sample clock reference delay line delay cell number selection."]
pub type AREFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSLVLOCK` reader - Flash B sample clock slave delay line locked."]
pub type BSLVLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BREFLOCK` reader - Flash B sample clock reference delay line locked."]
pub type BREFLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BSLVSEL` reader - Flash B sample clock slave delay line delay cell number selection."]
pub type BSLVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREFSEL` reader - Flash B sample clock reference delay line delay cell number selection."]
pub type BREFSEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub fn aslvlock(&self) -> ASLVLOCK_R {
        ASLVLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub fn areflock(&self) -> AREFLOCK_R {
        AREFLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn aslvsel(&self) -> ASLVSEL_R {
        ASLVSEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn arefsel(&self) -> AREFSEL_R {
        AREFSEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub fn bslvlock(&self) -> BSLVLOCK_R {
        BSLVLOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub fn breflock(&self) -> BREFLOCK_R {
        BREFLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub fn bslvsel(&self) -> BSLVSEL_R {
        BSLVSEL_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn brefsel(&self) -> BREFSEL_R {
        BREFSEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts2](index.html) module"]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts2::R](R) reader structure"]
impl crate::Readable for STS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS2 to value 0x0100_0100"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
