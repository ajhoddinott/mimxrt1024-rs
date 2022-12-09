#[doc = "Register `PKRCNT10` reader"]
pub struct R(crate::R<PKRCNT10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNT10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNT10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNT10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_0_CT` reader - Poker 0h Count"]
pub type PKR_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_1_CT` reader - Poker 1h Count"]
pub type PKR_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 0h Count"]
    #[inline(always)]
    pub fn pkr_0_ct(&self) -> PKR_0_CT_R {
        PKR_0_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 1h Count"]
    #[inline(always)]
    pub fn pkr_1_ct(&self) -> PKR_1_CT_R {
        PKR_1_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count 1 and 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt10](index.html) module"]
pub struct PKRCNT10_SPEC;
impl crate::RegisterSpec for PKRCNT10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcnt10::R](R) reader structure"]
impl crate::Readable for PKRCNT10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNT10 to value 0"]
impl crate::Resettable for PKRCNT10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
