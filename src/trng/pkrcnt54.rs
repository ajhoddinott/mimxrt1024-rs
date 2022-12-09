#[doc = "Register `PKRCNT54` reader"]
pub struct R(crate::R<PKRCNT54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNT54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNT54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNT54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_4_CT` reader - Poker 4h Count"]
pub type PKR_4_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_5_CT` reader - Poker 5h Count"]
pub type PKR_5_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 4h Count"]
    #[inline(always)]
    pub fn pkr_4_ct(&self) -> PKR_4_CT_R {
        PKR_4_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 5h Count"]
    #[inline(always)]
    pub fn pkr_5_ct(&self) -> PKR_5_CT_R {
        PKR_5_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count 5 and 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt54](index.html) module"]
pub struct PKRCNT54_SPEC;
impl crate::RegisterSpec for PKRCNT54_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcnt54::R](R) reader structure"]
impl crate::Readable for PKRCNT54_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNT54 to value 0"]
impl crate::Resettable for PKRCNT54_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
