#[doc = "Register `GDIR` reader"]
pub struct R(crate::R<GDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GDIR` writer"]
pub struct W(crate::W<GDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GDIR_SPEC>;
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
impl From<crate::W<GDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GDIR` reader - GPIO direction bits"]
pub type GDIR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GDIR` writer - GPIO direction bits"]
pub type GDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GDIR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO direction bits"]
    #[inline(always)]
    pub fn gdir(&self) -> GDIR_R {
        GDIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO direction bits"]
    #[inline(always)]
    #[must_use]
    pub fn gdir(&mut self) -> GDIR_W<0> {
        GDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdir](index.html) module"]
pub struct GDIR_SPEC;
impl crate::RegisterSpec for GDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gdir::R](R) reader structure"]
impl crate::Readable for GDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gdir::W](W) writer structure"]
impl crate::Writable for GDIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GDIR to value 0"]
impl crate::Resettable for GDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
