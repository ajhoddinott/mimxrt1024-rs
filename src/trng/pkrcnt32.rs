#[doc = "Register `PKRCNT32` reader"]
pub struct R(crate::R<PKRCNT32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNT32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNT32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNT32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_2_CT` reader - Poker 2h Count"]
pub type PKR_2_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_3_CT` reader - Poker 3h Count"]
pub type PKR_3_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker 2h Count"]
    #[inline(always)]
    pub fn pkr_2_ct(&self) -> PKR_2_CT_R {
        PKR_2_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker 3h Count"]
    #[inline(always)]
    pub fn pkr_3_ct(&self) -> PKR_3_CT_R {
        PKR_3_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count 3 and 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcnt32](index.html) module"]
pub struct PKRCNT32_SPEC;
impl crate::RegisterSpec for PKRCNT32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcnt32::R](R) reader structure"]
impl crate::Readable for PKRCNT32_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNT32 to value 0"]
impl crate::Resettable for PKRCNT32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
