#[doc = "Register `LMOD` reader"]
pub struct R(crate::R<LMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMOD` writer"]
pub struct W(crate::W<LMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMOD_SPEC>;
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
impl From<crate::W<LMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD` reader - MOD"]
pub type MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MOD` writer - MOD"]
pub type MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, LMOD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - MOD"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<0> {
        MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower Modulus Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmod](index.html) module"]
pub struct LMOD_SPEC;
impl crate::RegisterSpec for LMOD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lmod::R](R) reader structure"]
impl crate::Readable for LMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmod::W](W) writer structure"]
impl crate::Writable for LMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LMOD to value 0"]
impl crate::Resettable for LMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
