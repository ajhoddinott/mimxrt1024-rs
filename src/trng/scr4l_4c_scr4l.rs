#[doc = "Register `SCR4L` reader"]
pub struct R(crate::R<SCR4L_4C_SCR4L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR4L_4C_SCR4L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR4L_4C_SCR4L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR4L_4C_SCR4L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR4L` writer"]
pub struct W(crate::W<SCR4L_4C_SCR4L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR4L_4C_SCR4L_SPEC>;
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
impl From<crate::W<SCR4L_4C_SCR4L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR4L_4C_SCR4L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN4_MAX` reader - Run Length 4 Maximum Limit"]
pub type RUN4_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN4_MAX` writer - Run Length 4 Maximum Limit"]
pub type RUN4_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR4L_4C_SCR4L_SPEC, u16, u16, 12, O>;
#[doc = "Field `RUN4_RNG` reader - Run Length 4 Range"]
pub type RUN4_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN4_RNG` writer - Run Length 4 Range"]
pub type RUN4_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR4L_4C_SCR4L_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Run Length 4 Maximum Limit"]
    #[inline(always)]
    pub fn run4_max(&self) -> RUN4_MAX_R {
        RUN4_MAX_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Run Length 4 Range"]
    #[inline(always)]
    pub fn run4_rng(&self) -> RUN4_RNG_R {
        RUN4_RNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Run Length 4 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run4_max(&mut self) -> RUN4_MAX_W<0> {
        RUN4_MAX_W::new(self)
    }
    #[doc = "Bits 16:27 - Run Length 4 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run4_rng(&mut self) -> RUN4_RNG_W<16> {
        RUN4_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 4 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr4l_4c_scr4l](index.html) module"]
pub struct SCR4L_4C_SCR4L_SPEC;
impl crate::RegisterSpec for SCR4L_4C_SCR4L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr4l_4c_scr4l::R](R) reader structure"]
impl crate::Readable for SCR4L_4C_SCR4L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr4l_4c_scr4l::W](W) writer structure"]
impl crate::Writable for SCR4L_4C_SCR4L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR4L to value 0x0040_004b"]
impl crate::Resettable for SCR4L_4C_SCR4L_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_004b;
}
