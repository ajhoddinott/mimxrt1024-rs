#[doc = "Register `SCR6PL` reader"]
pub struct R(crate::R<SCR6PL_PC_SCR6PL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR6PL_PC_SCR6PL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR6PL_PC_SCR6PL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR6PL_PC_SCR6PL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR6PL` writer"]
pub struct W(crate::W<SCR6PL_PC_SCR6PL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR6PL_PC_SCR6PL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCR6PL_PC_SCR6PL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR6PL_PC_SCR6PL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN6P_MAX` reader - Run Length 6+ Maximum Limit"]
pub type RUN6P_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN6P_MAX` writer - Run Length 6+ Maximum Limit"]
pub type RUN6P_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR6PL_PC_SCR6PL_SPEC, u16, u16, 11, O>;
#[doc = "Field `RUN6P_RNG` reader - Run Length 6+ Range"]
pub type RUN6P_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN6P_RNG` writer - Run Length 6+ Range"]
pub type RUN6P_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR6PL_PC_SCR6PL_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub fn run6p_max(&self) -> RUN6P_MAX_R {
        RUN6P_MAX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    pub fn run6p_rng(&self) -> RUN6P_RNG_R {
        RUN6P_RNG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run6p_max(&mut self) -> RUN6P_MAX_W<0> {
        RUN6P_MAX_W::new(self)
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    #[must_use]
    pub fn run6p_rng(&mut self) -> RUN6P_RNG_W<16> {
        RUN6P_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 6+ Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr6pl_pc_scr6pl](index.html) module"]
pub struct SCR6PL_PC_SCR6PL_SPEC;
impl crate::RegisterSpec for SCR6PL_PC_SCR6PL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr6pl_pc_scr6pl::R](R) reader structure"]
impl crate::Readable for SCR6PL_PC_SCR6PL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr6pl_pc_scr6pl::W](W) writer structure"]
impl crate::Writable for SCR6PL_PC_SCR6PL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR6PL to value 0x002e_002f"]
impl crate::Resettable for SCR6PL_PC_SCR6PL_SPEC {
    const RESET_VALUE: Self::Ux = 0x002e_002f;
}
