#[doc = "Register `PKRCNTDC` reader"]
pub struct R(crate::R<PKRCNTDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNTDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNTDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNTDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_C_CT` reader - Poker Ch Count"]
pub type PKR_C_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_D_CT` reader - Poker Dh Count"]
pub type PKR_D_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Ch Count"]
    #[inline(always)]
    pub fn pkr_c_ct(&self) -> PKR_C_CT_R {
        PKR_C_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Dh Count"]
    #[inline(always)]
    pub fn pkr_d_ct(&self) -> PKR_D_CT_R {
        PKR_D_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count D and C Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntdc](index.html) module"]
pub struct PKRCNTDC_SPEC;
impl crate::RegisterSpec for PKRCNTDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcntdc::R](R) reader structure"]
impl crate::Readable for PKRCNTDC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNTDC to value 0"]
impl crate::Resettable for PKRCNTDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
