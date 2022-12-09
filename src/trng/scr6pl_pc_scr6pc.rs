#[doc = "Register `SCR6PC` reader"]
pub struct R(crate::R<SCR6PL_PC_SCR6PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR6PL_PC_SCR6PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR6PL_PC_SCR6PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR6PL_PC_SCR6PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R6P_0_CT` reader - Runs of Zero, Length 6+ Count"]
pub type R6P_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R6P_1_CT` reader - Runs of One, Length 6+ Count"]
pub type R6P_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Runs of Zero, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_0_ct(&self) -> R6P_0_CT_R {
        R6P_0_CT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Runs of One, Length 6+ Count"]
    #[inline(always)]
    pub fn r6p_1_ct(&self) -> R6P_1_CT_R {
        R6P_1_CT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Statistical Check Run Length 6+ Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr6pl_pc_scr6pc](index.html) module"]
pub struct SCR6PL_PC_SCR6PC_SPEC;
impl crate::RegisterSpec for SCR6PL_PC_SCR6PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr6pl_pc_scr6pc::R](R) reader structure"]
impl crate::Readable for SCR6PL_PC_SCR6PC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR6PC to value 0"]
impl crate::Resettable for SCR6PL_PC_SCR6PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
