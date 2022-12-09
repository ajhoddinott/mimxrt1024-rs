#[doc = "Register `PKRCNTBA` reader"]
pub struct R(crate::R<PKRCNTBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNTBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNTBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNTBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_A_CT` reader - Poker Ah Count"]
pub type PKR_A_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_B_CT` reader - Poker Bh Count"]
pub type PKR_B_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Ah Count"]
    #[inline(always)]
    pub fn pkr_a_ct(&self) -> PKR_A_CT_R {
        PKR_A_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Bh Count"]
    #[inline(always)]
    pub fn pkr_b_ct(&self) -> PKR_B_CT_R {
        PKR_B_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count B and A Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntba](index.html) module"]
pub struct PKRCNTBA_SPEC;
impl crate::RegisterSpec for PKRCNTBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcntba::R](R) reader structure"]
impl crate::Readable for PKRCNTBA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNTBA to value 0"]
impl crate::Resettable for PKRCNTBA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
