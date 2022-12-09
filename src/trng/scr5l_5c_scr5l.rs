#[doc = "Register `SCR5L` reader"]
pub struct R(crate::R<SCR5L_5C_SCR5L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR5L_5C_SCR5L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR5L_5C_SCR5L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR5L_5C_SCR5L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR5L` writer"]
pub struct W(crate::W<SCR5L_5C_SCR5L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR5L_5C_SCR5L_SPEC>;
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
impl From<crate::W<SCR5L_5C_SCR5L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR5L_5C_SCR5L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN5_MAX` reader - Run Length 5 Maximum Limit"]
pub type RUN5_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN5_MAX` writer - Run Length 5 Maximum Limit"]
pub type RUN5_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR5L_5C_SCR5L_SPEC, u16, u16, 11, O>;
#[doc = "Field `RUN5_RNG` reader - Run Length 5 Range"]
pub type RUN5_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN5_RNG` writer - Run Length 5 Range"]
pub type RUN5_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR5L_5C_SCR5L_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub fn run5_max(&self) -> RUN5_MAX_R {
        RUN5_MAX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    pub fn run5_rng(&self) -> RUN5_RNG_R {
        RUN5_RNG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run5_max(&mut self) -> RUN5_MAX_W<0> {
        RUN5_MAX_W::new(self)
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run5_rng(&mut self) -> RUN5_RNG_W<16> {
        RUN5_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 5 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr5l_5c_scr5l](index.html) module"]
pub struct SCR5L_5C_SCR5L_SPEC;
impl crate::RegisterSpec for SCR5L_5C_SCR5L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr5l_5c_scr5l::R](R) reader structure"]
impl crate::Readable for SCR5L_5C_SCR5L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr5l_5c_scr5l::W](W) writer structure"]
impl crate::Writable for SCR5L_5C_SCR5L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR5L to value 0x002e_002f"]
impl crate::Resettable for SCR5L_5C_SCR5L_SPEC {
    const RESET_VALUE: Self::Ux = 0x002e_002f;
}
