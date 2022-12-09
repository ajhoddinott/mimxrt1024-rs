#[doc = "Register `PKRCNTFE` reader"]
pub struct R(crate::R<PKRCNTFE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKRCNTFE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKRCNTFE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKRCNTFE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PKR_E_CT` reader - Poker Eh Count"]
pub type PKR_E_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKR_F_CT` reader - Poker Fh Count"]
pub type PKR_F_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Poker Eh Count"]
    #[inline(always)]
    pub fn pkr_e_ct(&self) -> PKR_E_CT_R {
        PKR_E_CT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Poker Fh Count"]
    #[inline(always)]
    pub fn pkr_f_ct(&self) -> PKR_F_CT_R {
        PKR_F_CT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Poker Count F and E Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkrcntfe](index.html) module"]
pub struct PKRCNTFE_SPEC;
impl crate::RegisterSpec for PKRCNTFE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkrcntfe::R](R) reader structure"]
impl crate::Readable for PKRCNTFE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKRCNTFE to value 0"]
impl crate::Resettable for PKRCNTFE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
