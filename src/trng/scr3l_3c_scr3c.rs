#[doc = "Register `SCR3C` reader"]
pub struct R(crate::R<SCR3L_3C_SCR3C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR3L_3C_SCR3C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR3L_3C_SCR3C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR3L_3C_SCR3C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R3_0_CT` reader - Runs of Zeroes, Length 3 Count"]
pub type R3_0_CT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R3_1_CT` reader - Runs of Ones, Length 3 Count"]
pub type R3_1_CT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Runs of Zeroes, Length 3 Count"]
    #[inline(always)]
    pub fn r3_0_ct(&self) -> R3_0_CT_R {
        R3_0_CT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Runs of Ones, Length 3 Count"]
    #[inline(always)]
    pub fn r3_1_ct(&self) -> R3_1_CT_R {
        R3_1_CT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Statistical Check Run Length 3 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr3l_3c_scr3c](index.html) module"]
pub struct SCR3L_3C_SCR3C_SPEC;
impl crate::RegisterSpec for SCR3L_3C_SCR3C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr3l_3c_scr3c::R](R) reader structure"]
impl crate::Readable for SCR3L_3C_SCR3C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCR3C to value 0"]
impl crate::Resettable for SCR3L_3C_SCR3C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
