#[doc = "Register `SCR4C` reader"]
pub struct R(crate::R<SCR4L_4C_SCR4C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR4L_4C_SCR4C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR4L_4C_SCR4C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR4L_4C_SCR4C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R4_0_CT` reader - Runs of Zero, Length 4 Count"]
pub type R4_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R4_1_CT` reader - Runs of One, Length 4 Count"]
pub type R4_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Runs of Zero, Length 4 Count"]
    #[inline(always)]
    pub fn r4_0_ct(&self) -> R4_0_CT_R {
        R4_0_CT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Runs of One, Length 4 Count"]
    #[inline(always)]
    pub fn r4_1_ct(&self) -> R4_1_CT_R {
        R4_1_CT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "Statistical Check Run Length 4 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr4l_4c_scr4c](index.html) module"]
pub struct SCR4L_4C_SCR4C_SPEC;
impl crate::RegisterSpec for SCR4L_4C_SCR4C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr4l_4c_scr4c::R](R) reader structure"]
impl crate::Readable for SCR4L_4C_SCR4C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR4C to value 0"]
impl crate::Resettable for SCR4L_4C_SCR4C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
