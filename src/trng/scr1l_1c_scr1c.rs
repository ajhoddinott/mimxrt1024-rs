#[doc = "Register `SCR1C` reader"]
pub struct R(crate::R<SCR1L_1C_SCR1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR1L_1C_SCR1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR1L_1C_SCR1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR1L_1C_SCR1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R1_0_CT` reader - Runs of Zero, Length 1 Count"]
pub type R1_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R1_1_CT` reader - Runs of One, Length 1 Count"]
pub type R1_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub fn r1_0_ct(&self) -> R1_0_CT_R {
        R1_0_CT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Runs of One, Length 1 Count"]
    #[inline(always)]
    pub fn r1_1_ct(&self) -> R1_1_CT_R {
        R1_1_CT_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Statistical Check Run Length 1 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr1l_1c_scr1c](index.html) module"]
pub struct SCR1L_1C_SCR1C_SPEC;
impl crate::RegisterSpec for SCR1L_1C_SCR1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr1l_1c_scr1c::R](R) reader structure"]
impl crate::Readable for SCR1L_1C_SCR1C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR1C to value 0"]
impl crate::Resettable for SCR1L_1C_SCR1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
