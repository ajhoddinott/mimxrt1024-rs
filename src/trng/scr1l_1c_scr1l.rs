#[doc = "Register `SCR1L` reader"]
pub struct R(crate::R<SCR1L_1C_SCR1L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR1L_1C_SCR1L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR1L_1C_SCR1L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR1L_1C_SCR1L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR1L` writer"]
pub struct W(crate::W<SCR1L_1C_SCR1L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR1L_1C_SCR1L_SPEC>;
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
impl From<crate::W<SCR1L_1C_SCR1L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR1L_1C_SCR1L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN1_MAX` reader - Run Length 1 Maximum Limit"]
pub type RUN1_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN1_MAX` writer - Run Length 1 Maximum Limit"]
pub type RUN1_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR1L_1C_SCR1L_SPEC, u16, u16, 15, O>;
#[doc = "Field `RUN1_RNG` reader - Run Length 1 Range"]
pub type RUN1_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN1_RNG` writer - Run Length 1 Range"]
pub type RUN1_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR1L_1C_SCR1L_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Run Length 1 Maximum Limit"]
    #[inline(always)]
    pub fn run1_max(&self) -> RUN1_MAX_R {
        RUN1_MAX_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Run Length 1 Range"]
    #[inline(always)]
    pub fn run1_rng(&self) -> RUN1_RNG_R {
        RUN1_RNG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Run Length 1 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run1_max(&mut self) -> RUN1_MAX_W<0> {
        RUN1_MAX_W::new(self)
    }
    #[doc = "Bits 16:30 - Run Length 1 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run1_rng(&mut self) -> RUN1_RNG_W<16> {
        RUN1_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr1l_1c_scr1l](index.html) module"]
pub struct SCR1L_1C_SCR1L_SPEC;
impl crate::RegisterSpec for SCR1L_1C_SCR1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr1l_1c_scr1l::R](R) reader structure"]
impl crate::Readable for SCR1L_1C_SCR1L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr1l_1c_scr1l::W](W) writer structure"]
impl crate::Writable for SCR1L_1C_SCR1L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR1L to value 0x00b2_0195"]
impl crate::Resettable for SCR1L_1C_SCR1L_SPEC {
    const RESET_VALUE: Self::Ux = 0x00b2_0195;
}
