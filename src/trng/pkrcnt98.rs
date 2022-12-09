#[doc = "Register `PKRCNT98` reader"]
pub struct R(crate::R<PKRCNT98_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNT98_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNT98_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNT98_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_8_CT` reader - Poker 8h Count"]
pub type PKR_8_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_9_CT` reader - Poker 9h Count"]
pub type PKR_9_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 8h Count"]
    #[inline(always)]
    pub fn pkr_8_ct(&self) -> PKR_8_CT_R {
        PKR_8_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 9h Count"]
    #[inline(always)]
    pub fn pkr_9_ct(&self) -> PKR_9_CT_R {
        PKR_9_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count 9 and 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt98](index.html) module"]
pub struct PKRCNT98_SPEC;
impl crate::RegisterSpec for PKRCNT98_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcnt98::R](R) reader structure"]
impl crate::Readable for PKRCNT98_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNT98 to value 0"]
impl crate::Resettable for PKRCNT98_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
