#[doc = "Register `SCR5C` reader"]
pub struct R(crate::R<SCR5L_5C_SCR5C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR5L_5C_SCR5C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR5L_5C_SCR5C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR5L_5C_SCR5C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R5_0_CT` reader - Runs of Zero, Length 5 Count"]
pub type R5_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R5_1_CT` reader - Runs of One, Length 5 Count"]
pub type R5_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 5 Count"]
    #[inline(always)]
    pub fn r5_0_ct(&self) -> R5_0_CT_R {
        R5_0_CT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 5 Count"]
    #[inline(always)]
    pub fn r5_1_ct(&self) -> R5_1_CT_R {
        R5_1_CT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Statistical Check Run Length 5 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr5l_5c_scr5c](index.html) module"]
pub struct SCR5L_5C_SCR5C_SPEC;
impl crate::RegisterSpec for SCR5L_5C_SCR5C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr5l_5c_scr5c::R](R) reader structure"]
impl crate::Readable for SCR5L_5C_SCR5C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR5C to value 0"]
impl crate::Resettable for SCR5L_5C_SCR5C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
