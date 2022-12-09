#[doc = "Register `SCR2L` reader"]
pub struct R(crate::R<SCR2L_2C_SCR2L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR2L_2C_SCR2L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR2L_2C_SCR2L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR2L_2C_SCR2L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR2L` writer"]
pub struct W(crate::W<SCR2L_2C_SCR2L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR2L_2C_SCR2L_SPEC>;
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
impl From<crate::W<SCR2L_2C_SCR2L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR2L_2C_SCR2L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN2_MAX` reader - Run Length 2 Maximum Limit"]
pub type RUN2_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN2_MAX` writer - Run Length 2 Maximum Limit"]
pub type RUN2_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR2L_2C_SCR2L_SPEC, u16, u16, 14, O>;
#[doc = "Field `RUN2_RNG` reader - Run Length 2 Range"]
pub type RUN2_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN2_RNG` writer - Run Length 2 Range"]
pub type RUN2_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR2L_2C_SCR2L_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub fn run2_max(&self) -> RUN2_MAX_R {
        RUN2_MAX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    pub fn run2_rng(&self) -> RUN2_RNG_R {
        RUN2_RNG_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run2_max(&mut self) -> RUN2_MAX_W<0> {
        RUN2_MAX_W::new(self)
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run2_rng(&mut self) -> RUN2_RNG_W<16> {
        RUN2_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr2l_2c_scr2l](index.html) module"]
pub struct SCR2L_2C_SCR2L_SPEC;
impl crate::RegisterSpec for SCR2L_2C_SCR2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr2l_2c_scr2l::R](R) reader structure"]
impl crate::Readable for SCR2L_2C_SCR2L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr2l_2c_scr2l::W](W) writer structure"]
impl crate::Writable for SCR2L_2C_SCR2L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR2L to value 0x007a_00dc"]
impl crate::Resettable for SCR2L_2C_SCR2L_SPEC {
    const RESET_VALUE: Self::Ux = 0x007a_00dc;
}
