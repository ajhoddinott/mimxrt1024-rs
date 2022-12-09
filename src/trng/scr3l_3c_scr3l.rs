#[doc = "Register `SCR3L` reader"]
pub struct R(crate::R<SCR3L_3C_SCR3L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR3L_3C_SCR3L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR3L_3C_SCR3L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR3L_3C_SCR3L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR3L` writer"]
pub struct W(crate::W<SCR3L_3C_SCR3L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR3L_3C_SCR3L_SPEC>;
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
impl From<crate::W<SCR3L_3C_SCR3L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR3L_3C_SCR3L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN3_MAX` reader - Run Length 3 Maximum Limit"]
pub type RUN3_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN3_MAX` writer - Run Length 3 Maximum Limit"]
pub type RUN3_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR3L_3C_SCR3L_SPEC, u16, u16, 13, O>;
#[doc = "Field `RUN3_RNG` reader - Run Length 3 Range"]
pub type RUN3_RNG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RUN3_RNG` writer - Run Length 3 Range"]
pub type RUN3_RNG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCR3L_3C_SCR3L_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Run Length 3 Maximum Limit"]
    #[inline(always)]
    pub fn run3_max(&self) -> RUN3_MAX_R {
        RUN3_MAX_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Run Length 3 Range"]
    #[inline(always)]
    pub fn run3_rng(&self) -> RUN3_RNG_R {
        RUN3_RNG_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Run Length 3 Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn run3_max(&mut self) -> RUN3_MAX_W<0> {
        RUN3_MAX_W::new(self)
    }
    #[doc = "Bits 16:28 - Run Length 3 Range"]
    #[inline(always)]
    #[must_use]
    pub fn run3_rng(&mut self) -> RUN3_RNG_W<16> {
        RUN3_RNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr3l_3c_scr3l](index.html) module"]
pub struct SCR3L_3C_SCR3L_SPEC;
impl crate::RegisterSpec for SCR3L_3C_SCR3L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr3l_3c_scr3l::R](R) reader structure"]
impl crate::Readable for SCR3L_3C_SCR3L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr3l_3c_scr3l::W](W) writer structure"]
impl crate::Writable for SCR3L_3C_SCR3L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR3L to value 0x0058_007d"]
impl crate::Resettable for SCR3L_3C_SCR3L_SPEC {
    const RESET_VALUE: Self::Ux = 0x0058_007d;
}
